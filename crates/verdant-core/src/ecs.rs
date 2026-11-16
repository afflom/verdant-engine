//! Deterministic Entity-Component-System (`ECS`), hierarchical scene graph, and BVH acceleration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::collision::{BlasTriangleHit, MeshBlas, SweptSphereHit};
use crate::error::{CollisionError, TensorComputeError};
use crate::geometry::ProceduralMesh;
use crate::math::{
    geometric_epsilon, pow10_u64, three_u32, Aabb, Frustum, Mat4, Quat, Ray, Vec3, DIM_PLANAR,
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

#[derive(Clone, Debug)]
enum BvhNode {
    Leaf {
        bounds: Aabb,
        entity: EntityId,
    },
    Internal {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

/// Bounding Volume Hierarchy (`BVH`) over world-space entity bounds for culling and picking.
#[derive(Clone, Debug, Default)]
pub struct BoundingVolumeHierarchy {
    root: Option<BvhNode>,
    leaf_count: usize,
}

impl BoundingVolumeHierarchy {
    /// Builds a top-down median-split BVH from `(EntityId, world_aabb)` pairs.
    #[must_use]
    pub fn build(mut items: Vec<(EntityId, Aabb)>) -> Self {
        let leaf_count = items.len();
        let root = Self::build_recursive(&mut items);
        Self { root, leaf_count }
    }

    /// Number of indexed scene entities in this BVH.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.leaf_count
    }

    /// Returns whether the BVH contains zero entities.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.leaf_count == 0
    }

    fn build_recursive(items: &mut [(EntityId, Aabb)]) -> Option<BvhNode> {
        match items {
            [] => None,
            [(entity, bounds)] => Some(BvhNode::Leaf {
                bounds: *bounds,
                entity: *entity,
            }),
            many => {
                let mut total = many[0].1;
                for (_, b) in &many[1..] {
                    total = total.union(*b);
                }
                let span = total.max - total.min;
                let axis = if span.x >= span.y && span.x >= span.z {
                    0
                } else if span.y >= span.z {
                    1
                } else {
                    DIM_PLANAR
                };
                many.sort_unstable_by(|(id_a, a), (id_b, b)| {
                    let ca = a.center();
                    let cb = b.center();
                    let va = [ca.x, ca.y, ca.z][axis];
                    let vb = [cb.x, cb.y, cb.z][axis];
                    va.total_cmp(&vb).then_with(|| id_a.cmp(id_b))
                });
                let mid = many.len() / DIM_PLANAR;
                let (left_slice, right_slice) = many.split_at_mut(mid);
                let left = Box::new(Self::build_recursive(left_slice)?);
                let right = Box::new(Self::build_recursive(right_slice)?);
                Some(BvhNode::Internal {
                    bounds: total,
                    left,
                    right,
                })
            }
        }
    }

    /// Returns the closest entity intersected by `ray` within `max_distance_meters`.
    #[must_use]
    pub fn raycast(&self, ray: Ray, max_distance_meters: f64) -> Option<RaycastHit> {
        let root = self.root.as_ref()?;
        let mut best: Option<(EntityId, f64)> = None;
        Self::raycast_node(root, ray, max_distance_meters, &mut best);
        best.map(|(entity, dist)| RaycastHit {
            entity,
            distance_meters: dist,
            point_world: ray.origin + ray.direction.scale(dist),
        })
    }

    fn raycast_node(node: &BvhNode, ray: Ray, max_dist: f64, best: &mut Option<(EntityId, f64)>) {
        let limit = best.map_or(max_dist, |(_, d)| d);
        match node {
            BvhNode::Leaf { bounds, entity } => {
                if let Some(t) = ray.intersect_aabb(*bounds) {
                    if t <= limit {
                        *best = Some((*entity, t));
                    }
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if let Some(t) = ray.intersect_aabb(*bounds) {
                    if t <= limit {
                        Self::raycast_node(left, ray, max_dist, best);
                        Self::raycast_node(right, ray, max_dist, best);
                    }
                }
            }
        }
    }

    /// Collects all entities whose world-space AABB intersects `frustum`, sorted deterministically.
    #[must_use]
    pub fn query_frustum(&self, frustum: &Frustum) -> Vec<EntityId> {
        let mut visible = Vec::new();
        if let Some(root) = &self.root {
            Self::frustum_node(root, frustum, &mut visible);
        }
        visible.sort();
        visible
    }

    fn frustum_node(node: &BvhNode, frustum: &Frustum, out: &mut Vec<EntityId>) {
        match node {
            BvhNode::Leaf { bounds, entity } => {
                if frustum.intersects_aabb(*bounds) {
                    out.push(*entity);
                }
            }
            BvhNode::Internal {
                bounds,
                left,
                right,
            } => {
                if frustum.intersects_aabb(*bounds) {
                    Self::frustum_node(left, frustum, out);
                    Self::frustum_node(right, frustum, out);
                }
            }
        }
    }

    fn collect_leaves(node: &BvhNode, out: &mut Vec<(EntityId, Aabb)>) {
        match node {
            BvhNode::Leaf { bounds, entity } => out.push((*entity, *bounds)),
            BvhNode::Internal { left, right, .. } => {
                Self::collect_leaves(left, out);
                Self::collect_leaves(right, out);
            }
        }
    }

    /// Evaluates frustum visibility across all indexed entities via tropical `(max, +)` directional
    /// support contractions in `verdant_tensor::tropical_gjk_support_batch`.
    pub fn query_frustum_tropical(
        &self,
        frustum: &Frustum,
    ) -> Result<Vec<EntityId>, TensorComputeError> {
        let Some(root) = &self.root else {
            return Ok(Vec::new());
        };
        let mut leaves = Vec::with_capacity(self.leaf_count);
        Self::collect_leaves(root, &mut leaves);
        if leaves.is_empty() {
            return Ok(Vec::new());
        }

        let scale = pow10_u64(three_u32()) as f64;
        let scale_sq = scale * scale;
        let plane_dirs: Vec<[i32; 1 + 1 + 1]> = frustum
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
        for (entity, aabb) in leaves {
            let xs = [aabb.min.x, aabb.max.x];
            let ys = [aabb.min.y, aabb.max.y];
            let zs = [aabb.min.z, aabb.max.z];
            let mut corners = Vec::new();
            for &x in &xs {
                for &y in &ys {
                    for &z in &zs {
                        corners.push([
                            (x * scale).round() as i32,
                            (y * scale).round() as i32,
                            (z * scale).round() as i32,
                        ]);
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
        visible.sort();
        Ok(visible)
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

    /// Propagates world-space transformation matrices across the entire scene hierarchy in a
    /// topological pass via `verdant_tensor::propagate_hierarchy_matrices_exact` (`uor-matmul`).
    pub fn propagate_all_world_matrices(
        &self,
    ) -> Result<Vec<(EntityId, Mat4)>, TensorComputeError> {
        if self.entities.is_empty() {
            return Ok(Vec::new());
        }
        let mut ordered_ids: Vec<EntityId> = Vec::with_capacity(self.entities.len());
        let mut remaining: Vec<EntityId> = self.entities.keys().copied().collect();

        while !remaining.is_empty() {
            let prev_len = remaining.len();
            let mut deferred = Vec::new();
            for id in remaining {
                let Some(rec) = self.entities.get(&id) else {
                    continue;
                };
                match rec.parent {
                    None => ordered_ids.push(id),
                    Some(pid)
                        if ordered_ids.contains(&pid) || !self.entities.contains_key(&pid) =>
                    {
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
                    .and_then(|pid| ordered_ids.iter().position(|&oid| oid == pid)),
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

    /// Exact two-level world-space raycast against surface triangles (`TLAS -> MeshBlas`).
    #[must_use]
    pub fn raycast_exact(
        &self,
        world_ray: Ray,
        max_distance: f64,
    ) -> Option<(EntityId, BlasTriangleHit)> {
        let eps = geometric_epsilon();
        let mut best: Option<(EntityId, BlasTriangleHit)> = None;
        let mut limit = max_distance;

        for inst in &self.instances {
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
            if let Some(local_hit) =
                self.blas_pool[inst.blas_slot].raycast_exact(local_ray, local_max)
            {
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
        best
    }

    /// Exact continuous world-space swept-sphere query (`t in [0.0, 1.0]`) pruned via tropical
    /// `kernel12TropicalBlasSweep` over the entity TLAS and solved against triangle `MeshBlas` leaves.
    #[must_use]
    pub fn sweep_sphere(
        &self,
        world_origin: Vec3,
        world_radius: f64,
        world_delta: Vec3,
    ) -> Option<(EntityId, SweptSphereHit)> {
        if self.instances.is_empty() {
            return None;
        }
        let r = world_radius.max(0.0);
        let scale = pow10_u64(three_u32()) as f64;
        let pad_milli = 1_i32 + 1_i32;
        let origin_milli = [
            (world_origin.x * scale).round() as i32,
            (world_origin.y * scale).round() as i32,
            (world_origin.z * scale).round() as i32,
        ];
        let radius_milli = ((r * scale).ceil() as i32).saturating_add(pad_milli);
        let delta_milli = [
            (world_delta.x * scale).round() as i32,
            (world_delta.y * scale).round() as i32,
            (world_delta.z * scale).round() as i32,
        ];

        let mut mins_milli = Vec::with_capacity(self.instances.len());
        let mut maxs_milli = Vec::with_capacity(self.instances.len());
        for inst in &self.instances {
            mins_milli.push([
                (inst.world_bounds.min.x * scale).floor() as i32,
                (inst.world_bounds.min.y * scale).floor() as i32,
                (inst.world_bounds.min.z * scale).floor() as i32,
            ]);
            maxs_milli.push([
                (inst.world_bounds.max.x * scale).ceil() as i32,
                (inst.world_bounds.max.y * scale).ceil() as i32,
                (inst.world_bounds.max.z * scale).ceil() as i32,
            ]);
        }

        let pruned = verdant_tensor::tropical_swept_aabb_toi_bounds(
            origin_milli,
            radius_milli,
            delta_milli,
            &mins_milli,
            &maxs_milli,
        )
        .ok();

        let mut best: Option<(EntityId, SweptSphereHit)> = None;
        for (idx, inst) in self.instances.iter().enumerate() {
            if let Some(ref intervals) = pruned {
                if intervals[idx].is_none() {
                    continue;
                }
            }
            let local_origin = inst.world_to_local.transform_point3(world_origin);
            let local_delta = inst.world_to_local.transform_vector3(world_delta);
            let local_radius = r / inst.uniform_scale;

            if let Some(local_hit) =
                self.blas_pool[inst.blas_slot].sweep_sphere(local_origin, local_radius, local_delta)
            {
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
        best
    }

    /// Evaluates the minimum Euclidean surface clearance around `world_point` within `search_radius`,
    /// returning `(entity, closest_world_point, outward_world_normal, world_distance)`.
    #[must_use]
    pub fn min_surface_clearance(
        &self,
        world_point: Vec3,
        search_radius: f64,
    ) -> Option<(EntityId, Vec3, Vec3, f64)> {
        let mut best_dist = search_radius.max(0.0);
        let mut best: Option<(EntityId, Vec3, Vec3, f64)> = None;
        let query_box =
            Aabb::from_center_half_extents(world_point, Vec3::new(best_dist, best_dist, best_dist));

        for inst in &self.instances {
            if !inst.world_bounds.intersects(query_box) {
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
                    let cp_world = inst.local_to_world.transform_point3(cp_local);
                    let n_world = inst.local_to_world.transform_vector3(n_local).normalized();
                    best = Some((inst.entity, cp_world, n_world, d_world));
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
