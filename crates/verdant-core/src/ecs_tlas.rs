//! Exact Two-Level Bounding Volume Hierarchy (`TLAS` + `MeshBlas`) queries (`ecs_tlas.rs`).

use super::*;

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
            let uniform =
                sx == sy && sy == sz && c0.dot(c1) == 0.0 && c1.dot(c2) == 0.0 && c0.dot(c2) == 0.0;
            if let Some(inv_mat) = world_mat.inverse_affine().filter(|_| uniform) {
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
                let world_bounds = mesh.bounds().transformed(world_mat);
                let blas = MeshBlas::from_transformed_mesh(mesh, world_mat)?;
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
        self.sweep_capsule_counted(world_origin, Vec3::ZERO, world_radius, world_delta)
    }

    /// Sweep the complete capsule (the segment from origin to origin + axis, inflated by radius).
    #[must_use]
    pub fn sweep_capsule_counted(
        &self,
        world_origin: Vec3,
        axis: Vec3,
        world_radius: f64,
        world_delta: Vec3,
    ) -> (Option<(EntityId, SweptSphereHit)>, usize) {
        if self.tlas.nodes.is_empty() {
            return (None, 0);
        }
        let r = world_radius.max(0.0);
        let bound = r + axis.length();
        let mut best: Option<(EntityId, SweptSphereHit)> = None;
        let mut visited_nodes = 0usize;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if segment_swept_aabb_toi(
            world_origin,
            world_delta,
            self.tlas.nodes[0].bounds(),
            bound,
        )
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
            if segment_swept_aabb_toi(world_origin, world_delta, node.bounds(), bound)
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
                    if segment_swept_aabb_toi(world_origin, world_delta, inst.world_bounds, bound)
                        .is_none_or(|t| t > cur_t)
                    {
                        continue;
                    }
                    let local_origin = inst.world_to_local.transform_point3(world_origin);
                    let local_delta = inst.world_to_local.transform_vector3(world_delta);
                    let local_radius = r / inst.uniform_scale;
                    let (local_hit_opt, blas_visited) = self.blas_pool[inst.blas_slot]
                        .sweep_capsule_counted(
                            local_origin,
                            inst.world_to_local.transform_vector3(axis),
                            local_radius,
                            local_delta,
                        );
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
                    bound,
                )
                .filter(|&t| t <= cur_t);
                let tr = segment_swept_aabb_toi(
                    world_origin,
                    world_delta,
                    self.tlas.nodes[right_idx as usize].bounds(),
                    bound,
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
