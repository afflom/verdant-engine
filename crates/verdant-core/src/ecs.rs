//! Deterministic Entity-Component-System (`ECS`), hierarchical scene graph, and BVH acceleration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use verdant_tensor::{ColumnBitPlaneTable, WordBitset, ARCHETYPE_BIT_PLANES};

use crate::collision::{
    aabb_point_dist_sq, build_flat_sah_bvh, segment_swept_aabb_toi, BlasTriangleHit, FlatBvhNode,
    MeshBlas, SweptSphereHit,
};
use crate::error::{CollisionError, TensorComputeError};
use crate::geometry::ProceduralMesh;
use crate::math::{
    geometric_epsilon, pow10_u64, three_u32, Aabb, Frustum, Mat4, Quat, Ray, Vec3, DIM_HOMOGENEOUS,
    DIM_SPATIAL,
};
use crate::prism_kernel;

/// Stable, deterministic identifier of a scene entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

/// Universal 3D scene role classification of an entity within `SceneWorld`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EntityRole {
    /// Directional sun, sky, or analytical light source.
    DirectionalLight,
    /// Heightfield terrain patch or rock outcrop.
    TerrainPatch,
    /// Buoyant fluid volume or water surface plane.
    FluidVolume,
    /// Instanced surface foliage or deformable prop.
    SurfaceFoliage,
    /// Autonomous NavMesh or steering agent.
    AutonomousAgent,
    /// Static architectural or structural mesh.
    StaticArchitecture,
    /// Articulated skeletal or spring-joint rig.
    ArticulatedRig,
    /// Kinematic spline mover or moving platform.
    KinematicMover,
    /// Interactive player avatar or camera rig.
    PlayerRig,
}

/// Local-to-parent spatial transformation (`Translation`, `Rotation`, `Scale`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// Local position in meters relative to parent (or world root).
    pub translation: Vec3,
    /// Local unit quaternion orientation.
    pub rotation: Quat,
    /// Non-uniform local scale factors.
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    /// Constructs a transform at `translation` with identity rotation and unit scale.
    #[must_use]
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self {
            translation: Vec3::new(x, y, z),
            ..Self::default()
        }
    }

    /// Converts local transform into a homogeneous affine matrix.
    #[must_use]
    pub fn local_matrix(self) -> Mat4 {
        Mat4::from_trs(self.translation, self.rotation, self.scale)
    }
}

/// Scene entity record holding hierarchy, transform, bounds, mesh/material references, and metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntityRecord {
    /// Stable entity handle.
    pub id: EntityId,
    /// Human-readable scene node name.
    pub name: String,
    /// Domain classification in the scene.
    pub role: EntityRole,
    /// Optional parent entity in the scene graph.
    pub parent: Option<EntityId>,
    /// Ordered child entities.
    pub children: Vec<EntityId>,
    /// Local TRS transform.
    pub local_transform: Transform,
    /// Local-space bounding box before world transform.
    pub local_bounds: Aabb,
    /// Optional mesh asset index in the scene's mesh library.
    pub mesh_index: Option<usize>,
    /// Optional PBR material index in the scene's material library.
    pub material_index: Option<usize>,
    /// Progressive streaming LOD resolution scale in `[0, 1]` at which this entity activates.
    pub emerged_at_lod_scale: f64,
    /// Asset or authoring provenance tag.
    pub provenance_tag: String,
}

/// Raycast hit result returned by spatial scene queries.
#[derive(Clone, Debug, PartialEq)]
pub struct RaycastHit {
    /// Intersected entity handle.
    pub entity: EntityId,
    /// Ray parameter `t` in meters from ray origin.
    pub distance_meters: f64,
    /// World-space hit point.
    pub point_world: Vec3,
}

/// Bounding Volume Hierarchy (`BVH`) over world-space entity bounds backed by a contiguous
/// 32-byte binned-SAH `FlatBvhNode` array (`O(log N)` zero-allocation stack traversal).
#[derive(Clone, Debug, Default)]
pub struct BoundingVolumeHierarchy {
    pub(crate) items: Vec<(EntityId, Aabb)>,
    pub(crate) nodes: Vec<FlatBvhNode>,
    pub(crate) prim_indices: Vec<u32>,
}

impl BoundingVolumeHierarchy {
    /// Builds a top-down binned-SAH flat BVH from `(EntityId, world_aabb)` pairs.
    #[must_use]
    pub fn build(items: Vec<(EntityId, Aabb)>) -> Self {
        if items.is_empty() {
            return Self::default();
        }
        let prim_bounds: Vec<Aabb> = items.iter().map(|(_, b)| *b).collect();
        let leaf_cap = verdant_tensor::kernel_consts().flat_bvh_leaf_max_prims;
        let (nodes, prim_indices) = build_flat_sah_bvh(&prim_bounds, leaf_cap);
        Self {
            items,
            nodes,
            prim_indices,
        }
    }

    /// Number of indexed scene entities in this BVH.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the BVH contains zero entities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Number of contiguous 32-byte `FlatBvhNode` entries in this TLAS.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the closest entity intersected by `ray` within `max_distance_meters` (`0` heap allocations).
    #[must_use]
    pub fn raycast(&self, ray: Ray, max_distance_meters: f64) -> Option<RaycastHit> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut best: Option<(EntityId, f64)> = None;
        let mut limit = max_distance_meters;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if ray
            .intersect_aabb(self.nodes[0].bounds())
            .is_some_and(|t| t <= limit)
        {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.nodes[idx];
            if ray.intersect_aabb(node.bounds()).is_none_or(|t| t > limit) {
                continue;
            }
            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.prim_indices[start..end] {
                    let (entity, bounds) = self.items[pi as usize];
                    if let Some(t) = ray.intersect_aabb(bounds) {
                        if t <= limit {
                            limit = t;
                            best = Some((entity, t));
                        }
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let tl = ray
                    .intersect_aabb(self.nodes[left_idx as usize].bounds())
                    .filter(|&d| d <= limit);
                let tr = ray
                    .intersect_aabb(self.nodes[right_idx as usize].bounds())
                    .filter(|&d| d <= limit);
                match (tl, tr) {
                    (Some(dl), Some(dr)) => {
                        if dl <= dr {
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                        } else {
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                        }
                    }
                    (Some(_), None) => {
                        if sp < stack.len() {
                            stack[sp] = left_idx;
                            sp += 1;
                        }
                    }
                    (None, Some(_)) => {
                        if sp < stack.len() {
                            stack[sp] = right_idx;
                            sp += 1;
                        }
                    }
                    (None, None) => {}
                }
            }
        }

        best.map(|(entity, dist)| RaycastHit {
            entity,
            distance_meters: dist,
            point_world: ray.origin + ray.direction.scale(dist),
        })
    }

    /// Marks intersecting primitive indices (`0..self.len()`) directly into a 64-bit `WordBitset`
    /// with zero heap allocations.
    pub fn query_frustum_into_mask(&self, frustum: &Frustum, out_mask: &mut WordBitset) {
        if out_mask.len() != self.items.len() {
            out_mask.resize(self.items.len());
        }
        out_mask.clear_all();
        if self.nodes.is_empty() {
            return;
        }
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;
        if frustum.intersects_aabb(self.nodes[0].bounds()) {
            stack[0] = 0;
            sp = 1;
        }
        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.nodes[idx];
            if !frustum.intersects_aabb(node.bounds()) {
                continue;
            }
            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.prim_indices[start..end] {
                    let (_, bounds) = self.items[pi as usize];
                    if frustum.intersects_aabb(bounds) {
                        out_mask.set(pi as usize);
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                if sp < stack.len() {
                    stack[sp] = right_idx;
                    sp += 1;
                }
                if sp < stack.len() {
                    stack[sp] = left_idx;
                    sp += 1;
                }
            }
        }
    }

    /// Counts entities whose world-space AABB intersects `frustum` with zero `Vec` allocations.
    #[must_use]
    pub fn count_frustum_visible(&self, frustum: &Frustum) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        let mut count = 0usize;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;
        if frustum.intersects_aabb(self.nodes[0].bounds()) {
            stack[0] = 0;
            sp = 1;
        }
        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.nodes[idx];
            if !frustum.intersects_aabb(node.bounds()) {
                continue;
            }
            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.prim_indices[start..end] {
                    if frustum.intersects_aabb(self.items[pi as usize].1) {
                        count += 1;
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                if sp < stack.len() {
                    stack[sp] = right_idx;
                    sp += 1;
                }
                if sp < stack.len() {
                    stack[sp] = left_idx;
                    sp += 1;
                }
            }
        }
        count
    }

    /// Collects all entities whose world-space AABB intersects `frustum`, sorted deterministically.
    #[must_use]
    pub fn query_frustum(&self, frustum: &Frustum) -> Vec<EntityId> {
        let mut mask = WordBitset::with_len(self.items.len());
        self.query_frustum_into_mask(frustum, &mut mask);
        let mut visible = Vec::with_capacity(mask.count_ones());
        mask.for_each_set_bit(|slot| {
            visible.push(self.items[slot].0);
        });
        visible.sort_unstable();
        visible
    }

    /// Evaluates frustum visibility across all indexed entities via tropical `(max, +)` directional
    /// support contractions in `verdant_tensor::tropical_gjk_support_batch`.
    pub fn query_frustum_tropical(
        &self,
        frustum: &Frustum,
    ) -> Result<Vec<EntityId>, TensorComputeError> {
        if self.items.is_empty() {
            return Ok(Vec::new());
        }

        let scale = pow10_u64(three_u32()) as f64;
        let scale_sq = scale * scale;
        let plane_dirs: Vec<[i32; DIM_SPATIAL]> = frustum
            .planes
            .iter()
            .map(|p| {
                [
                    (p.normal.x * scale).round() as i32,
                    (p.normal.y * scale).round() as i32,
                    (p.normal.z * scale).round() as i32,
                ]
            })
            .collect();

        let mut visible = Vec::new();
        for &(entity, aabb) in &self.items {
            let xs = [aabb.min.x, aabb.max.x];
            let ys = [aabb.min.y, aabb.max.y];
            let zs = [aabb.min.z, aabb.max.z];
            let mut corners = [[0_i32; DIM_SPATIAL]; 1 << DIM_SPATIAL];
            let mut c_idx = 0usize;
            for &x in &xs {
                for &y in &ys {
                    for &z in &zs {
                        corners[c_idx] = [
                            (x * scale).round() as i32,
                            (y * scale).round() as i32,
                            (z * scale).round() as i32,
                        ];
                        c_idx += 1;
                    }
                }
            }
            let supports = verdant_tensor::tropical_gjk_support_batch(&plane_dirs, &corners)?;
            let inside =
                supports
                    .iter()
                    .zip(frustum.planes.iter())
                    .all(|(&(_, max_dot_q), plane)| {
                        (max_dot_q as f64) + plane.distance * scale_sq >= 0.0
                    });
            if inside {
                visible.push(entity);
            }
        }
        visible.sort_unstable();
        Ok(visible)
    }
}

/// Cache-oblivious Structure-of-Arrays (`SoA`) archetype table paired with 64-bit word-packed
/// bitsets (`WordBitset`, adapted from `verdant-aqm`), scaling ECS transform propagation, frustum
/// visibility, and GPU hardware instancing compaction to `100,000+` entities in `O(dirty + N / 64)`.
#[derive(Clone, Debug, Default)]
pub struct SoaSceneTable {
    /// Dense slot -> `EntityId`.
    pub entity_ids: Vec<EntityId>,
    /// Dense slot -> `EntityRole`.
    pub roles: Vec<EntityRole>,
    /// Topologically ordered parent slot (`parent_slot < slot`).
    pub parent_slots: Vec<Option<u32>>,
    /// Dense local transforms.
    pub local_transforms: Vec<Transform>,
    /// Dense local AABBs.
    pub local_bounds: Vec<Aabb>,
    /// Cached world matrices.
    pub world_matrices: Vec<Mat4>,
    /// Cached world AABBs.
    pub world_bounds: Vec<Aabb>,
    /// Optional mesh index per slot.
    pub mesh_indices: Vec<Option<u32>>,
    /// Packed linear RGBA tint per slot for GPU instancing.
    pub color_tints: Vec<[f32; DIM_HOMOGENEOUS]>,
    /// 64-bit word-packed mask of dirty transforms awaiting propagation.
    pub dirty_mask: WordBitset,
    /// 64-bit word-packed mask of static world entities.
    pub static_mask: WordBitset,
    /// 64-bit word-packed mask of renderable entities (`mesh_index.is_some()`).
    pub renderable_mask: WordBitset,
    /// 64-bit word-packed mask of frustum-visible entities.
    pub visible_mask: WordBitset,
    /// Column-transposed `8 x ceil(N / 64)` capability bit-plane matrix (`0` = Visible, `1` = SolidCollider,
    /// `2` = ShadowCaster, `3` = DynamicActor, `4` = InstancedMember, `5` = AudioEmitter, `6` = PortalTrigger,
    /// `7` = InteractiveStory), extrapolated from `verdant-aqm` `ColumnBitPlaneTableau`.
    pub capability_planes: ColumnBitPlaneTable,
    /// Monotonic epoch incremented whenever static geometry changes.
    pub static_epoch: u64,
}

impl SoaSceneTable {
    /// Preallocates contiguous `SoA` columns for `capacity` entities.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entity_ids: Vec::with_capacity(capacity),
            roles: Vec::with_capacity(capacity),
            parent_slots: Vec::with_capacity(capacity),
            local_transforms: Vec::with_capacity(capacity),
            local_bounds: Vec::with_capacity(capacity),
            world_matrices: Vec::with_capacity(capacity),
            world_bounds: Vec::with_capacity(capacity),
            mesh_indices: Vec::with_capacity(capacity),
            color_tints: Vec::with_capacity(capacity),
            dirty_mask: WordBitset::with_len(0),
            static_mask: WordBitset::with_len(0),
            renderable_mask: WordBitset::with_len(0),
            visible_mask: WordBitset::with_len(0),
            capability_planes: ColumnBitPlaneTable::new(ARCHETYPE_BIT_PLANES, 0),
            static_epoch: 1,
        }
    }

    fn sync_slot_capability_planes(
        planes: &mut ColumnBitPlaneTable,
        slot: usize,
        role: EntityRole,
        has_mesh: bool,
    ) {
        let c_vis = 0usize;
        let c_solid = 1usize;
        let c_shadow = 1 + 1;
        let c_dyn = c_shadow + 1;
        let c_inst = c_dyn + 1;
        planes.set(c_vis, slot, has_mesh);
        planes.set(
            c_solid,
            slot,
            matches!(
                role,
                EntityRole::TerrainPatch
                    | EntityRole::StaticArchitecture
                    | EntityRole::KinematicMover
            ),
        );
        planes.set(
            c_shadow,
            slot,
            has_mesh && !matches!(role, EntityRole::FluidVolume | EntityRole::DirectionalLight),
        );
        planes.set(
            c_dyn,
            slot,
            matches!(
                role,
                EntityRole::AutonomousAgent
                    | EntityRole::ArticulatedRig
                    | EntityRole::KinematicMover
                    | EntityRole::PlayerRig
            ),
        );
        planes.set(c_inst, slot, matches!(role, EntityRole::SurfaceFoliage));
    }

    /// Compiles a `SceneWorld` into topologically ordered `SoA` columns + `WordBitset` masks in `O(N)`.
    pub fn from_scene_world(scene: &SceneWorld) -> Result<Self, TensorComputeError> {
        let mats = scene.propagate_all_world_matrices()?;
        let n = mats.len();
        let mut table = Self::with_capacity(n);
        table.dirty_mask = WordBitset::with_len(n);
        table.static_mask = WordBitset::with_len(n);
        table.renderable_mask = WordBitset::with_len(n);
        table.visible_mask = WordBitset::all_ones(n);
        table.capability_planes = ColumnBitPlaneTable::new(ARCHETYPE_BIT_PLANES, n);

        let mut id_to_slot: BTreeMap<EntityId, u32> = BTreeMap::new();
        for (slot, &(id, world_mat)) in mats.iter().enumerate() {
            id_to_slot.insert(id, slot as u32);
            let Some(rec) = scene.get(id) else {
                continue;
            };
            let wb = rec.local_bounds.transformed(world_mat);
            table.entity_ids.push(id);
            table.roles.push(rec.role);
            table
                .parent_slots
                .push(rec.parent.and_then(|pid| id_to_slot.get(&pid).copied()));
            table.local_transforms.push(rec.local_transform);
            table.local_bounds.push(rec.local_bounds);
            table.world_matrices.push(world_mat);
            table.world_bounds.push(wb);
            table.mesh_indices.push(rec.mesh_index.map(|m| m as u32));
            table.color_tints.push([1.0, 1.0, 1.0, 1.0]);
            if matches!(
                rec.role,
                EntityRole::TerrainPatch
                    | EntityRole::StaticArchitecture
                    | EntityRole::FluidVolume
                    | EntityRole::SurfaceFoliage
            ) {
                table.static_mask.set(slot);
            }
            if rec.mesh_index.is_some() {
                table.renderable_mask.set(slot);
            }
            Self::sync_slot_capability_planes(
                &mut table.capability_planes,
                slot,
                rec.role,
                rec.mesh_index.is_some(),
            );
        }
        Ok(table)
    }

    /// Appends a dense entity directly into the `SoA` table in `O(1)` amortized time.
    pub fn spawn_dense(
        &mut self,
        id: EntityId,
        role: EntityRole,
        parent_slot: Option<u32>,
        local_transform: Transform,
        local_bounds: Aabb,
        visual: (Option<u32>, [f32; DIM_HOMOGENEOUS]),
    ) -> usize {
        let (mesh_index, color_tint) = visual;
        let slot = self.entity_ids.len();
        let local_mat = local_transform.local_matrix();
        let world_mat = match parent_slot {
            Some(ps) if (ps as usize) < self.world_matrices.len() => {
                self.world_matrices[ps as usize] * local_mat
            }
            _ => local_mat,
        };
        let wb = local_bounds.transformed(world_mat);
        self.entity_ids.push(id);
        self.roles.push(role);
        self.parent_slots.push(parent_slot);
        self.local_transforms.push(local_transform);
        self.local_bounds.push(local_bounds);
        self.world_matrices.push(world_mat);
        self.world_bounds.push(wb);
        self.mesh_indices.push(mesh_index);
        self.color_tints.push(color_tint);

        let new_len = slot + 1;
        self.dirty_mask.resize(new_len);
        self.static_mask.resize(new_len);
        self.renderable_mask.resize(new_len);
        self.visible_mask.resize(new_len);
        self.visible_mask.set(slot);
        if matches!(
            role,
            EntityRole::TerrainPatch
                | EntityRole::StaticArchitecture
                | EntityRole::FluidVolume
                | EntityRole::SurfaceFoliage
        ) {
            self.static_mask.set(slot);
            self.static_epoch = self.static_epoch.wrapping_add(1);
        }
        if mesh_index.is_some() {
            self.renderable_mask.set(slot);
        }
        self.capability_planes.resize_rows(new_len);
        Self::sync_slot_capability_planes(
            &mut self.capability_planes,
            slot,
            role,
            mesh_index.is_some(),
        );
        slot
    }

    /// Evaluates an `O(N / 64)` multi-capability intersection & exclusion query over `capability_planes`.
    pub fn query_capability_planes_into(
        &self,
        required_cols: &[usize],
        excluded_cols: &[usize],
        out_mask: &mut WordBitset,
    ) -> usize {
        self.capability_planes
            .query_archetype_into(required_cols, excluded_cols, out_mask)
    }

    /// Total dense entity count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entity_ids.len()
    }

    /// Returns true if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }

    /// Updates `local_transforms[slot]` and sets bit `slot` in `dirty_mask` (`O(1)`).
    #[inline]
    pub fn set_local_transform(&mut self, slot: usize, transform: Transform) {
        if slot < self.local_transforms.len() {
            self.local_transforms[slot] = transform;
            self.dirty_mask.set(slot);
            if self.static_mask.test(slot) {
                self.static_epoch = self.static_epoch.wrapping_add(1);
            }
        }
    }

    /// Sweeps `dirty_mask` via 64-bit hardware `trailing_zeros` (`O(dirty + N / 64)`), updating
    /// strictly dirty world matrices & AABBs and returning the exact count of recomputed transforms.
    pub fn propagate_dirty_transforms(&mut self) -> usize {
        if self.dirty_mask.count_ones() == 0 {
            return 0;
        }
        let mut updated = 0usize;
        // Propagate dirty bits to descendants whose parent slot was marked dirty:
        for slot in 0..self.parent_slots.len() {
            if let Some(ps) = self.parent_slots[slot] {
                if self.dirty_mask.test(ps as usize) {
                    self.dirty_mask.set(slot);
                }
            }
        }
        let world_mats = &mut self.world_matrices;
        let world_bnds = &mut self.world_bounds;
        let local_xfs = &self.local_transforms;
        let local_bnds = &self.local_bounds;
        let parents = &self.parent_slots;

        self.dirty_mask.for_each_set_bit(|slot| {
            let local_mat = local_xfs[slot].local_matrix();
            let w_mat = match parents[slot] {
                Some(ps) => world_mats[ps as usize] * local_mat,
                None => local_mat,
            };
            world_mats[slot] = w_mat;
            world_bnds[slot] = local_bnds[slot].transformed(w_mat);
            updated += 1;
        });
        self.dirty_mask.clear_all();
        updated
    }

    /// Sweeps `renderable_mask` in `O(renderable + N / 64)` to populate `visible_mask` against
    /// `frustum` and returns the visible count via hardware `popcnt`.
    pub fn cull_frustum_word_packed(&mut self, frustum: &Frustum) -> usize {
        self.visible_mask.clear_all();
        let bounds = &self.world_bounds;
        let vis = &mut self.visible_mask;
        self.renderable_mask.for_each_set_bit(|slot| {
            if frustum.intersects_aabb(bounds[slot]) {
                vis.set(slot);
            }
        });
        self.visible_mask.count_ones()
    }

    /// Compacts all frustum-visible instances matching `target_mesh_index` into `16`-float GPU
    /// instance rows (`col0.xyz, tint.r, col1.xyz, tint.g, col2.xyz, tint.b, trans.xyz, tint.a`)
    /// ready for a single `gl.drawElementsInstanced` call.
    pub fn compact_visible_instances_for_mesh(
        &self,
        target_mesh_index: u32,
        out_f32_x16: &mut Vec<f32>,
    ) -> usize {
        out_f32_x16.clear();
        let i2 = 1usize + 1usize;
        let i3 = i2 + 1usize;
        let mut count = 0usize;
        self.visible_mask.for_each_set_bit(|slot| {
            if self.mesh_indices[slot] == Some(target_mesh_index) {
                let m = &self.world_matrices[slot].cols;
                let c = self.color_tints[slot];
                out_f32_x16.extend_from_slice(&[
                    m[0][0] as f32,
                    m[0][1] as f32,
                    m[0][i2] as f32,
                    c[0],
                    m[1][0] as f32,
                    m[1][1] as f32,
                    m[1][i2] as f32,
                    c[1],
                    m[i2][0] as f32,
                    m[i2][1] as f32,
                    m[i2][i2] as f32,
                    c[i2],
                    m[i3][0] as f32,
                    m[i3][1] as f32,
                    m[i3][i2] as f32,
                    c[i3],
                ]);
                count += 1;
            }
        });
        count
    }
}

/// Deterministic hierarchical scene world container (`BTreeMap`-backed for bit-exact iteration).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SceneWorld {
    pub(crate) next_id: u64,
    pub(crate) entities: BTreeMap<EntityId, EntityRecord>,
}

impl SceneWorld {
    /// Creates an empty scene world.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entities: BTreeMap::new(),
        }
    }

    /// Spawns a new entity into the scene graph and attaches it to `parent` when specified.
    pub fn spawn<S: Into<String>>(
        &mut self,
        name: impl Into<String>,
        role: EntityRole,
        parent: Option<EntityId>,
        local_transform: Transform,
        local_bounds: Aabb,
        emergence: (f64, S),
    ) -> EntityId {
        let (emerged_at_lod_scale, provenance_tag) = emergence;
        let id = EntityId(self.next_id);
        self.next_id += 1;
        if let Some(parent_id) = parent {
            if let Some(p) = self.entities.get_mut(&parent_id) {
                p.children.push(id);
            }
        }
        self.entities.insert(
            id,
            EntityRecord {
                id,
                name: name.into(),
                role,
                parent,
                children: Vec::new(),
                local_transform,
                local_bounds,
                mesh_index: None,
                material_index: None,
                emerged_at_lod_scale,
                provenance_tag: provenance_tag.into(),
            },
        );
        id
    }

    /// Assigns mesh and PBR material library indices to an existing entity.
    pub fn attach_renderable(
        &mut self,
        entity: EntityId,
        mesh_index: usize,
        material_index: usize,
    ) {
        if let Some(rec) = self.entities.get_mut(&entity) {
            rec.mesh_index = Some(mesh_index);
            rec.material_index = Some(material_index);
        }
    }

    /// Total entity count in the scene.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Looks up an entity record by handle.
    #[must_use]
    pub fn get(&self, id: EntityId) -> Option<&EntityRecord> {
        self.entities.get(&id)
    }

    /// Mutably looks up an entity record by handle.
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut EntityRecord> {
        self.entities.get_mut(&id)
    }

    /// Iterates over all entities in deterministic `EntityId` order.
    pub fn entities(&self) -> impl Iterator<Item = &EntityRecord> {
        self.entities.values()
    }

    /// Computes the world-space transformation matrix of `id` by composing parent transforms.
    #[must_use]
    pub fn world_matrix(&self, id: EntityId) -> Option<Mat4> {
        let rec = self.entities.get(&id)?;
        let local = rec.local_transform.local_matrix();
        match rec.parent {
            Some(parent_id) => Some(self.world_matrix(parent_id)? * local),
            None => Some(local),
        }
    }

    /// Computes world-space AABB for `id`.
    #[must_use]
    pub fn world_bounds(&self, id: EntityId) -> Option<Aabb> {
        let rec = self.entities.get(&id)?;
        let mat = self.world_matrix(id)?;
        Some(rec.local_bounds.transformed(mat))
    }

    /// Propagates world-space transformation matrices across the entire scene hierarchy in an
    /// `O(N)` topological Kahn pass via `verdant_tensor::propagate_hierarchy_matrices_exact` (`uor-matmul`).
    pub fn propagate_all_world_matrices(
        &self,
    ) -> Result<Vec<(EntityId, Mat4)>, TensorComputeError> {
        if self.entities.is_empty() {
            return Ok(Vec::new());
        }
        let n = self.entities.len();
        let mut ordered_ids: Vec<EntityId> = Vec::with_capacity(n);
        let mut id_to_order_slot: BTreeMap<EntityId, usize> = BTreeMap::new();
        let mut remaining: Vec<EntityId> = self.entities.keys().copied().collect();

        while !remaining.is_empty() {
            let prev_len = remaining.len();
            let mut deferred = Vec::new();
            for id in remaining {
                let Some(rec) = self.entities.get(&id) else {
                    continue;
                };
                match rec.parent {
                    None => {
                        id_to_order_slot.insert(id, ordered_ids.len());
                        ordered_ids.push(id);
                    }
                    Some(pid)
                        if id_to_order_slot.contains_key(&pid)
                            || !self.entities.contains_key(&pid) =>
                    {
                        id_to_order_slot.insert(id, ordered_ids.len());
                        ordered_ids.push(id);
                    }
                    Some(_) => deferred.push(id),
                }
            }
            if deferred.len() == prev_len {
                return Err(TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "scene hierarchy contains a parent cycle".to_string(),
                });
            }
            remaining = deferred;
        }

        let mut parent_slots = Vec::with_capacity(ordered_ids.len());
        let mut local_cols = Vec::with_capacity(ordered_ids.len());
        for &id in &ordered_ids {
            let Some(rec) = self.entities.get(&id) else {
                continue;
            };
            parent_slots.push(
                rec.parent
                    .and_then(|pid| id_to_order_slot.get(&pid).copied()),
            );
            local_cols.push(rec.local_transform.local_matrix().cols);
        }

        let world_cols =
            verdant_tensor::propagate_hierarchy_matrices_exact(&parent_slots, &local_cols)?;
        let mut pairs = Vec::with_capacity(ordered_ids.len());
        for (id, cols) in ordered_ids.into_iter().zip(world_cols) {
            pairs.push((id, Mat4 { cols }));
        }
        Ok(pairs)
    }

    /// Compiles this `SceneWorld` into a cache-oblivious `SoaSceneTable` backed by 64-bit `WordBitset` masks.
    pub fn build_soa_table(&self) -> Result<SoaSceneTable, TensorComputeError> {
        SoaSceneTable::from_scene_world(self)
    }

    /// Builds a fresh world-space `BoundingVolumeHierarchy` over all active entities.
    #[must_use]
    pub fn build_bvh(&self) -> BoundingVolumeHierarchy {
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(mut world_mats) = self.propagate_all_world_matrices() {
            world_mats.sort_unstable_by_key(|(id, _)| *id);
            let items: Vec<(EntityId, Aabb)> = world_mats
                .into_iter()
                .filter_map(|(id, mat)| {
                    let rec = self.entities.get(&id)?;
                    Some((id, rec.local_bounds.transformed(mat)))
                })
                .collect();
            return BoundingVolumeHierarchy::build(items);
        }
        let items: Vec<(EntityId, Aabb)> = self
            .entities
            .keys()
            .filter_map(|&id| Some((id, self.world_bounds(id)?)))
            .collect();
        BoundingVolumeHierarchy::build(items)
    }

    /// Builds an exact two-level BVH (`Entity TLAS` + triangle `MeshBlas`) over solid scene entities.
    pub fn build_two_level_bvh(
        &self,
        meshes: &[ProceduralMesh],
    ) -> Result<TwoLevelBvh, CollisionError> {
        TwoLevelBvh::build_solid(self, meshes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct TwoLevelInstance {
    entity: EntityId,
    role: EntityRole,
    world_bounds: Aabb,
    local_to_world: Mat4,
    world_to_local: Mat4,
    uniform_scale: f64,
    blas_slot: usize,
}

/// Exact Two-Level Bounding Volume Hierarchy (`TLAS` over scene entities + `MeshBlas` over surface
/// triangles) enabling hollow doorways, vaulted interiors, arch bridges, and zero-penetration sweeps.
#[derive(Clone, Debug)]
pub struct TwoLevelBvh {
    instances: Vec<TwoLevelInstance>,
    blas_pool: Vec<MeshBlas>,
    tlas: BoundingVolumeHierarchy,
}

impl TwoLevelBvh {
    /// Returns whether `role` represents solid physical geometry for movement and camera occlusion.
    #[must_use]
    pub const fn is_solid_role(role: EntityRole) -> bool {
        matches!(
            role,
            EntityRole::TerrainPatch
                | EntityRole::StaticArchitecture
                | EntityRole::ArticulatedRig
                | EntityRole::KinematicMover
                | EntityRole::AutonomousAgent
        )
    }

    /// Builds a `TwoLevelBvh` indexing all solid renderable entities in `scene`.
    pub fn build_solid(
        scene: &SceneWorld,
        meshes: &[ProceduralMesh],
    ) -> Result<Self, CollisionError> {
        Self::build_filtered(scene, meshes, Self::is_solid_role)
    }

    /// Builds a `TwoLevelBvh` indexing entities whose `EntityRole` satisfies `include_role`.
    pub fn build_filtered<F>(
        scene: &SceneWorld,
        meshes: &[ProceduralMesh],
        include_role: F,
    ) -> Result<Self, CollisionError>
    where
        F: Fn(EntityRole) -> bool,
    {
        let mut shared_mesh_slot: Vec<Option<usize>> = vec![None; meshes.len()];
        let mut blas_pool: Vec<MeshBlas> = Vec::new();
        let mut instances: Vec<TwoLevelInstance> = Vec::new();
        let mut tlas_items: Vec<(EntityId, Aabb)> = Vec::new();

        let eps = geometric_epsilon();
        let scale_tol = 1.0 / (pow10_u64(three_u32()) as f64);
        let i2 = 1usize + 1usize;

        for entity in scene.entities() {
            if !include_role(entity.role) {
                continue;
            }
            let Some(mesh_idx) = entity.mesh_index else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_idx) else {
                continue;
            };
            let Some(world_mat) = scene.world_matrix(entity.id) else {
                continue;
            };

            let c0 = Vec3::new(
                world_mat.cols[0][0],
                world_mat.cols[0][1],
                world_mat.cols[0][i2],
            );
            let c1 = Vec3::new(
                world_mat.cols[1][0],
                world_mat.cols[1][1],
                world_mat.cols[1][i2],
            );
            let c2 = Vec3::new(
                world_mat.cols[i2][0],
                world_mat.cols[i2][1],
                world_mat.cols[i2][i2],
            );
            let sx = c0.length();
            let sy = c1.length();
            let sz = c2.length();
            if sx <= eps || sy <= eps || sz <= eps {
                continue;
            }
            let uniform = (sx - sy).abs() <= scale_tol * sx.max(1.0)
                && (sy - sz).abs() <= scale_tol * sy.max(1.0)
                && c0.normalized().dot(c1.normalized()).abs() <= scale_tol
                && c1.normalized().dot(c2.normalized()).abs() <= scale_tol
                && c0.normalized().dot(c2.normalized()).abs() <= scale_tol;

            if uniform {
                let Some(inv_mat) = world_mat.inverse_affine() else {
                    continue;
                };
                let slot = if let Some(existing) = shared_mesh_slot[mesh_idx] {
                    existing
                } else {
                    let blas = MeshBlas::from_mesh(mesh)?;
                    let idx = blas_pool.len();
                    blas_pool.push(blas);
                    shared_mesh_slot[mesh_idx] = Some(idx);
                    idx
                };
                let world_bounds = mesh.bounds().transformed(world_mat);
                tlas_items.push((entity.id, world_bounds));
                instances.push(TwoLevelInstance {
                    entity: entity.id,
                    role: entity.role,
                    world_bounds,
                    local_to_world: world_mat,
                    world_to_local: inv_mat,
                    uniform_scale: sx,
                    blas_slot: slot,
                });
            } else {
                // Non-uniformly scaled or sheared entity: bake world-space positions into an exact metric BLAS:
                let mut world_positions = Vec::with_capacity(mesh.positions.len());
                let mut world_normals = Vec::with_capacity(mesh.normals.len());
                let origin_w = world_mat.transform_point3(Vec3::ZERO);
                for (&pos, &nrm) in mesh.positions.iter().zip(mesh.normals.iter()) {
                    let pw = world_mat.transform_point3(Vec3::new(
                        f64::from(pos[0]),
                        f64::from(pos[1]),
                        f64::from(pos[i2]),
                    ));
                    world_positions.push(pw.to_f32_array());
                    let nw = (world_mat.transform_point3(Vec3::new(
                        f64::from(nrm[0]),
                        f64::from(nrm[1]),
                        f64::from(nrm[i2]),
                    )) - origin_w)
                        .normalized();
                    world_normals.push(nw.to_f32_array());
                }
                let baked_mesh = ProceduralMesh {
                    name: "world_blas".to_string(),
                    positions: world_positions,
                    normals: world_normals,
                    uvs: mesh.uvs.clone(),
                    colors: mesh.colors.clone(),
                    indices: mesh.indices.clone(),
                    morph_targets: Vec::new(),
                };
                let world_bounds = baked_mesh.bounds();
                let Ok(blas) = MeshBlas::from_mesh(&baked_mesh) else {
                    continue;
                };
                let slot = blas_pool.len();
                blas_pool.push(blas);
                tlas_items.push((entity.id, world_bounds));
                instances.push(TwoLevelInstance {
                    entity: entity.id,
                    role: entity.role,
                    world_bounds,
                    local_to_world: Mat4::IDENTITY,
                    world_to_local: Mat4::IDENTITY,
                    uniform_scale: 1.0,
                    blas_slot: slot,
                });
            }
        }

        let tlas = BoundingVolumeHierarchy::build(tlas_items);
        Ok(Self {
            instances,
            blas_pool,
            tlas,
        })
    }

    /// Number of solid entity instances indexed in the TLAS.
    #[must_use]
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    /// Total surface triangles across all indexed instances.
    #[must_use]
    pub fn total_surface_triangles(&self) -> usize {
        self.instances
            .iter()
            .map(|inst| self.blas_pool[inst.blas_slot].triangle_count())
            .sum()
    }

    /// Underlying entity-level TLAS.
    #[must_use]
    pub const fn tlas(&self) -> &BoundingVolumeHierarchy {
        &self.tlas
    }

    /// Total contiguous 32-byte `FlatBvhNode` count across the TLAS and all unique `MeshBlas` entries.
    #[must_use]
    pub fn total_bvh_nodes(&self) -> usize {
        self.tlas.node_count()
            + self
                .blas_pool
                .iter()
                .map(MeshBlas::node_count)
                .sum::<usize>()
    }

    /// Exact `O(log N)` two-level world-space raycast returning `(hit, total_visited_bvh_nodes)`
    /// with zero heap allocations and near-child-first TLAS + BLAS traversal.
    #[must_use]
    pub fn raycast_exact_counted(
        &self,
        world_ray: Ray,
        max_distance: f64,
    ) -> (Option<(EntityId, BlasTriangleHit)>, usize) {
        if self.tlas.nodes.is_empty() {
            return (None, 0);
        }
        let eps = geometric_epsilon();
        let mut best: Option<(EntityId, BlasTriangleHit)> = None;
        let mut limit = max_distance;
        let mut visited_nodes = 0usize;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if world_ray
            .intersect_aabb(self.tlas.nodes[0].bounds())
            .is_some_and(|t| t <= limit)
        {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.tlas.nodes[idx];
            visited_nodes += 1;
            if world_ray
                .intersect_aabb(node.bounds())
                .is_none_or(|t| t > limit)
            {
                continue;
            }

            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.tlas.prim_indices[start..end] {
                    let inst = &self.instances[pi as usize];
                    if world_ray
                        .intersect_aabb(inst.world_bounds)
                        .is_none_or(|t_box| t_box > limit)
                    {
                        continue;
                    }
                    let local_origin = inst.world_to_local.transform_point3(world_ray.origin);
                    let local_dir_raw = inst.world_to_local.transform_vector3(world_ray.direction);
                    let dir_scale = local_dir_raw.length();
                    if dir_scale <= eps {
                        continue;
                    }
                    let local_ray = Ray {
                        origin: local_origin,
                        direction: local_dir_raw.scale(1.0 / dir_scale),
                    };
                    let local_max = limit * dir_scale;
                    let (local_hit_opt, blas_visited) =
                        self.blas_pool[inst.blas_slot].raycast_exact_counted(local_ray, local_max);
                    visited_nodes += blas_visited;
                    if let Some(local_hit) = local_hit_opt {
                        let world_point = inst.local_to_world.transform_point3(local_hit.point);
                        let world_dist = world_point.distance(world_ray.origin);
                        if world_dist <= limit {
                            limit = world_dist;
                            let geom_n = inst
                                .local_to_world
                                .transform_vector3(local_hit.geometric_normal)
                                .normalized();
                            let shade_n = inst
                                .local_to_world
                                .transform_vector3(local_hit.shading_normal)
                                .normalized();
                            best = Some((
                                inst.entity,
                                BlasTriangleHit {
                                    triangle_index: local_hit.triangle_index,
                                    distance_meters: world_dist,
                                    point: world_point,
                                    geometric_normal: geom_n,
                                    shading_normal: shade_n,
                                    barycentrics: local_hit.barycentrics,
                                },
                            ));
                        }
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let tl = world_ray
                    .intersect_aabb(self.tlas.nodes[left_idx as usize].bounds())
                    .filter(|&d| d <= limit);
                let tr = world_ray
                    .intersect_aabb(self.tlas.nodes[right_idx as usize].bounds())
                    .filter(|&d| d <= limit);
                match (tl, tr) {
                    (Some(dl), Some(dr)) => {
                        if dl <= dr {
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                        } else {
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                        }
                    }
                    (Some(_), None) => {
                        if sp < stack.len() {
                            stack[sp] = left_idx;
                            sp += 1;
                        }
                    }
                    (None, Some(_)) => {
                        if sp < stack.len() {
                            stack[sp] = right_idx;
                            sp += 1;
                        }
                    }
                    (None, None) => {}
                }
            }
        }
        (best, visited_nodes)
    }

    /// Exact two-level world-space raycast against surface triangles (`TLAS -> MeshBlas`).
    #[must_use]
    pub fn raycast_exact(
        &self,
        world_ray: Ray,
        max_distance: f64,
    ) -> Option<(EntityId, BlasTriangleHit)> {
        self.raycast_exact_counted(world_ray, max_distance).0
    }

    /// Exact continuous world-space swept-sphere query (`t in [0.0, 1.0]`) traversing the contiguous
    /// `FlatBvhNode` TLAS and `MeshBlas` in `O(log N)` with near-child-first TOI ordering and zero heap allocations.
    #[must_use]
    pub fn sweep_sphere_counted(
        &self,
        world_origin: Vec3,
        world_radius: f64,
        world_delta: Vec3,
    ) -> (Option<(EntityId, SweptSphereHit)>, usize) {
        if self.tlas.nodes.is_empty() {
            return (None, 0);
        }
        let r = world_radius.max(0.0);
        let mut best: Option<(EntityId, SweptSphereHit)> = None;
        let mut visited_nodes = 0usize;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if segment_swept_aabb_toi(world_origin, world_delta, self.tlas.nodes[0].bounds(), r)
            .is_some()
        {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.tlas.nodes[idx];
            visited_nodes += 1;
            let max_t = best.map_or(1.0, |(_, h)| h.toi);
            if segment_swept_aabb_toi(world_origin, world_delta, node.bounds(), r)
                .is_none_or(|t| t > max_t)
            {
                continue;
            }

            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.tlas.prim_indices[start..end] {
                    let inst = &self.instances[pi as usize];
                    let cur_t = best.map_or(1.0, |(_, h)| h.toi);
                    if segment_swept_aabb_toi(world_origin, world_delta, inst.world_bounds, r)
                        .is_none_or(|t| t > cur_t)
                    {
                        continue;
                    }
                    let local_origin = inst.world_to_local.transform_point3(world_origin);
                    let local_delta = inst.world_to_local.transform_vector3(world_delta);
                    let local_radius = r / inst.uniform_scale;
                    let (local_hit_opt, blas_visited) = self.blas_pool[inst.blas_slot]
                        .sweep_sphere_counted(local_origin, local_radius, local_delta);
                    visited_nodes += blas_visited;

                    if let Some(local_hit) = local_hit_opt {
                        if best.is_none_or(|(_, prev)| local_hit.toi < prev.toi) {
                            let world_contact = inst
                                .local_to_world
                                .transform_point3(local_hit.contact_point);
                            let world_normal = inst
                                .local_to_world
                                .transform_vector3(local_hit.contact_normal)
                                .normalized();
                            best = Some((
                                inst.entity,
                                SweptSphereHit {
                                    toi: local_hit.toi,
                                    contact_point: world_contact,
                                    contact_normal: world_normal,
                                    triangle_index: local_hit.triangle_index,
                                },
                            ));
                        }
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let cur_t = best.map_or(1.0, |(_, h)| h.toi);
                let tl = segment_swept_aabb_toi(
                    world_origin,
                    world_delta,
                    self.tlas.nodes[left_idx as usize].bounds(),
                    r,
                )
                .filter(|&t| t <= cur_t);
                let tr = segment_swept_aabb_toi(
                    world_origin,
                    world_delta,
                    self.tlas.nodes[right_idx as usize].bounds(),
                    r,
                )
                .filter(|&t| t <= cur_t);
                match (tl, tr) {
                    (Some(dl), Some(dr)) => {
                        if dl <= dr {
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                        } else {
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                        }
                    }
                    (Some(_), None) => {
                        if sp < stack.len() {
                            stack[sp] = left_idx;
                            sp += 1;
                        }
                    }
                    (None, Some(_)) => {
                        if sp < stack.len() {
                            stack[sp] = right_idx;
                            sp += 1;
                        }
                    }
                    (None, None) => {}
                }
            }
        }
        (best, visited_nodes)
    }

    /// Exact continuous world-space swept-sphere query (`t in [0.0, 1.0]`).
    #[must_use]
    pub fn sweep_sphere(
        &self,
        world_origin: Vec3,
        world_radius: f64,
        world_delta: Vec3,
    ) -> Option<(EntityId, SweptSphereHit)> {
        self.sweep_sphere_counted(world_origin, world_radius, world_delta)
            .0
    }

    /// Evaluates the minimum Euclidean surface clearance around `world_point` within `search_radius`
    /// in `O(log N)` across `self.tlas` + `MeshBlas` with zero heap allocations.
    #[must_use]
    pub fn min_surface_clearance(
        &self,
        world_point: Vec3,
        search_radius: f64,
    ) -> Option<(EntityId, Vec3, Vec3, f64)> {
        if self.tlas.nodes.is_empty() {
            return None;
        }
        let mut best_dist = search_radius.max(0.0);
        let mut best_dist_sq = best_dist * best_dist;
        let mut best: Option<(EntityId, Vec3, Vec3, f64)> = None;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if aabb_point_dist_sq(self.tlas.nodes[0].bounds(), world_point) <= best_dist_sq {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.tlas.nodes[idx];
            if aabb_point_dist_sq(node.bounds(), world_point) > best_dist_sq {
                continue;
            }
            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &pi in &self.tlas.prim_indices[start..end] {
                    let inst = &self.instances[pi as usize];
                    if aabb_point_dist_sq(inst.world_bounds, world_point) > best_dist_sq {
                        continue;
                    }
                    let local_pt = inst.world_to_local.transform_point3(world_point);
                    let local_r = best_dist / inst.uniform_scale;
                    if let Some((cp_local, n_local, d_local)) =
                        self.blas_pool[inst.blas_slot].closest_point(local_pt, local_r)
                    {
                        let d_world = d_local * inst.uniform_scale;
                        if d_world <= best_dist {
                            best_dist = d_world;
                            best_dist_sq = d_world * d_world;
                            let cp_world = inst.local_to_world.transform_point3(cp_local);
                            let n_world =
                                inst.local_to_world.transform_vector3(n_local).normalized();
                            best = Some((inst.entity, cp_world, n_world, d_world));
                        }
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let dl =
                    aabb_point_dist_sq(self.tlas.nodes[left_idx as usize].bounds(), world_point);
                let dr =
                    aabb_point_dist_sq(self.tlas.nodes[right_idx as usize].bounds(), world_point);
                let l_ok = dl <= best_dist_sq;
                let r_ok = dr <= best_dist_sq;
                match (l_ok, r_ok) {
                    (true, true) => {
                        if dl <= dr {
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                        } else {
                            if sp < stack.len() {
                                stack[sp] = left_idx;
                                sp += 1;
                            }
                            if sp < stack.len() {
                                stack[sp] = right_idx;
                                sp += 1;
                            }
                        }
                    }
                    (true, false) => {
                        if sp < stack.len() {
                            stack[sp] = left_idx;
                            sp += 1;
                        }
                    }
                    (false, true) => {
                        if sp < stack.len() {
                            stack[sp] = right_idx;
                            sp += 1;
                        }
                    }
                    (false, false) => {}
                }
            }
        }
        best
    }

    /// Verifies that `TwoLevelBvh` instance counts and surface clearance predicates align with
    /// `Subsystems::canonicalDimensions` and `Subsystems::sweptClearanceValid` in `prism_kernel`.
    #[must_use]
    pub fn prism_bvh_invariants_valid(&self) -> bool {
        let Ok(dims) = prism_kernel::canonicalDimensions() else {
            return false;
        };
        let margin = prism_kernel::ccdSkinMarginMilli();
        let radius = prism_kernel::sFortyEight().unwrap_or(1);
        dims.dimSpatial == u64::from(three_u32())
            && self.instances.len() <= self.blas_pool.len()
            && prism_kernel::sweptClearanceValid(radius + margin, radius, margin) == Ok(true)
    }
}
