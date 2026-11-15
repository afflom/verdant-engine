//! Deterministic Entity-Component-System (`ECS`), hierarchical scene graph, and BVH acceleration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::math::{Aabb, Frustum, Mat4, Quat, Ray, Vec3, DIM_PLANAR};

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
                many.sort_by(|(id_a, a), (id_b, b)| {
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

    /// Builds a fresh world-space `BoundingVolumeHierarchy` over all active entities.
    #[must_use]
    pub fn build_bvh(&self) -> BoundingVolumeHierarchy {
        let items: Vec<(EntityId, Aabb)> = self
            .entities
            .keys()
            .filter_map(|&id| Some((id, self.world_bounds(id)?)))
            .collect();
        BoundingVolumeHierarchy::build(items)
    }
}
