//! Contiguous flat SAH triangle BLAS implementation (`collision_blas.rs`).

use super::*;

impl MeshBlas {
    /// Builds a surface-area-ordered binary triangle BVH from `mesh`.
    pub fn from_mesh(mesh: &ProceduralMesh) -> Result<Self, CollisionError> {
        Self::from_transformed_mesh(mesh, Mat4::IDENTITY)
    }

    pub(crate) fn from_transformed_mesh(
        mesh: &ProceduralMesh,
        transform: Mat4,
    ) -> Result<Self, CollisionError> {
        let inverse = transform.inverse_affine();
        if mesh.positions.is_empty() || mesh.indices.len() < DIM_SPATIAL {
            return Err(CollisionError::EmptyOrDegenerateHull {
                vertex_count: mesh.positions.len(),
            });
        }
        let i2 = 1usize + 1usize;
        let eps = geometric_epsilon();
        let mut triangles = Vec::with_capacity(mesh.indices.len() / DIM_SPATIAL);

        for (tri_idx, tri) in mesh.indices.chunks_exact(DIM_SPATIAL).enumerate() {
            let ia = tri[0] as usize;
            let ib = tri[1] as usize;
            let ic = tri[i2] as usize;
            if ia >= mesh.positions.len()
                || ib >= mesh.positions.len()
                || ic >= mesh.positions.len()
            {
                return Err(CollisionError::EmptyOrDegenerateHull {
                    vertex_count: mesh.positions.len(),
                });
            }
            let to_v = |p: [f32; DIM_SPATIAL]| {
                Vec3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[i2]))
            };
            let v0 = transform.transform_point3(to_v(mesh.positions[ia]));
            let v1 = transform.transform_point3(to_v(mesh.positions[ib]));
            let v2 = transform.transform_point3(to_v(mesh.positions[ic]));
            let raw_n = (v1 - v0).cross(v2 - v0);
            if raw_n.length_squared() <= eps * eps {
                continue;
            }
            let face_normal = raw_n.normalized();
            let shading_normal = |index: usize| {
                let Some((normal, inv)) = mesh.normals.get(index).zip(inverse) else {
                    return face_normal;
                };
                let n = to_v(*normal);
                let cols = inv.cols;
                let component =
                    |c: [f64; crate::math::DIM_HOMOGENEOUS]| n.dot(Vec3::new(c[0], c[1], c[i2]));
                Vec3::new(component(cols[0]), component(cols[1]), component(cols[i2])).normalized()
            };
            let (n0, n1, n2) = (shading_normal(ia), shading_normal(ib), shading_normal(ic));
            let min = Vec3::new(
                v0.x.min(v1.x).min(v2.x),
                v0.y.min(v1.y).min(v2.y),
                v0.z.min(v1.z).min(v2.z),
            );
            let max = Vec3::new(
                v0.x.max(v1.x).max(v2.x),
                v0.y.max(v1.y).max(v2.y),
                v0.z.max(v1.z).max(v2.z),
            );
            triangles.push(BlasTriangle {
                v0,
                v1,
                v2,
                n0,
                n1,
                n2,
                face_normal,
                bounds: Aabb::new(min, max),
                tri_index: tri_idx,
            });
        }

        if triangles.is_empty() {
            return Err(CollisionError::EmptyOrDegenerateHull { vertex_count: 0 });
        }

        let prim_bounds: Vec<Aabb> = triangles.iter().map(|t| t.bounds).collect();
        let leaf_cap = verdant_tensor::kernel_consts().flat_bvh_leaf_max_prims;
        let (nodes, prim_indices) = build_flat_sah_bvh(&prim_bounds, leaf_cap);
        Ok(Self {
            triangles,
            nodes,
            prim_indices,
        })
    }

    /// Number of non-degenerate triangles indexed in this BLAS.
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Number of contiguous 32-byte `FlatBvhNode` entries in this BLAS.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Underlying contiguous 32-byte `FlatBvhNode` slice.
    #[must_use]
    pub fn flat_nodes(&self) -> &[FlatBvhNode] {
        &self.nodes
    }

    /// Exact Möller–Trumbore raycast returning `(hit, visited_nodes)` with zero heap allocations.
    #[must_use]
    pub fn raycast_exact_counted(
        &self,
        ray: Ray,
        max_distance: f64,
    ) -> (Option<BlasTriangleHit>, usize) {
        if self.nodes.is_empty() {
            return (None, 0);
        }
        let eps = geometric_epsilon();
        let mut best: Option<BlasTriangleHit> = None;
        let mut limit = max_distance;
        let mut visited = 0usize;
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
            visited += 1;
            if ray.intersect_aabb(node.bounds()).is_none_or(|t| t > limit) {
                continue;
            }

            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &ti_u32 in &self.prim_indices[start..end] {
                    let tri = &self.triangles[ti_u32 as usize];
                    let e1 = tri.v1 - tri.v0;
                    let e2 = tri.v2 - tri.v0;
                    let pvec = ray.direction.cross(e2);
                    let det = e1.dot(pvec);
                    if det.abs() <= eps {
                        continue;
                    }
                    let inv_det = 1.0 / det;
                    let tvec = ray.origin - tri.v0;
                    let u = tvec.dot(pvec) * inv_det;
                    if !(0.0..=1.0).contains(&u) {
                        continue;
                    }
                    let qvec = tvec.cross(e1);
                    let v = ray.direction.dot(qvec) * inv_det;
                    if v < 0.0 || u + v > 1.0 {
                        continue;
                    }
                    let t = e2.dot(qvec) * inv_det;
                    if t >= 0.0 && t <= limit {
                        limit = t;
                        let w0 = 1.0 - u - v;
                        let geom_n = if ray.direction.dot(tri.face_normal) <= 0.0 {
                            tri.face_normal
                        } else {
                            Vec3::ZERO - tri.face_normal
                        };
                        let raw_shade = tri.n0.scale(w0) + tri.n1.scale(u) + tri.n2.scale(v);
                        let shade_n = if raw_shade.dot(geom_n) >= 0.0 {
                            raw_shade.normalized()
                        } else {
                            (Vec3::ZERO - raw_shade).normalized()
                        };
                        best = Some(BlasTriangleHit {
                            triangle_index: tri.tri_index,
                            distance_meters: t,
                            point: ray.origin + ray.direction.scale(t),
                            geometric_normal: geom_n,
                            shading_normal: shade_n,
                            barycentrics: [w0, u, v],
                        });
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
        (best, visited)
    }

    /// Exact Möller–Trumbore raycast returning the nearest triangle surface hit within `max_distance`.
    #[must_use]
    pub fn raycast_exact(&self, ray: Ray, max_distance: f64) -> Option<BlasTriangleHit> {
        self.raycast_exact_counted(ray, max_distance).0
    }

    /// Continuous swept-sphere query returning `(hit, visited_nodes)` with near-child-first TOI pruning.
    #[must_use]
    pub fn sweep_sphere_counted(
        &self,
        origin: Vec3,
        radius: f64,
        delta: Vec3,
    ) -> (Option<SweptSphereHit>, usize) {
        self.sweep_capsule_counted(origin, Vec3::ZERO, radius, delta)
    }

    pub(crate) fn sweep_capsule_counted(
        &self,
        origin: Vec3,
        axis: Vec3,
        radius: f64,
        delta: Vec3,
    ) -> (Option<SweptSphereHit>, usize) {
        if self.nodes.is_empty() {
            return (None, 0);
        }
        let r = radius.max(0.0);
        let bound = r + axis.length();
        let mut best: Option<SweptSphereHit> = None;
        let mut visited = 0usize;
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if segment_swept_aabb_toi(origin, delta, self.nodes[0].bounds(), bound).is_some() {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.nodes[idx];
            visited += 1;
            let max_t = best.map_or(1.0, |h| h.toi);
            if segment_swept_aabb_toi(origin, delta, node.bounds(), bound).is_none_or(|t| t > max_t)
            {
                continue;
            }

            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &ti_u32 in &self.prim_indices[start..end] {
                    let tri = &self.triangles[ti_u32 as usize];
                    let cur_t = best.map_or(1.0, |h| h.toi);
                    if segment_swept_aabb_toi(origin, delta, tri.bounds, bound)
                        .is_none_or(|t| t > cur_t)
                    {
                        continue;
                    }
                    if let Some(hit) = sweep_capsule_against_triangle(origin, axis, r, delta, tri) {
                        if best.is_none_or(|prev| hit.toi < prev.toi) {
                            best = Some(hit);
                        }
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let cur_t = best.map_or(1.0, |h| h.toi);
                let tl = segment_swept_aabb_toi(
                    origin,
                    delta,
                    self.nodes[left_idx as usize].bounds(),
                    bound,
                )
                .filter(|&t| t <= cur_t);
                let tr = segment_swept_aabb_toi(
                    origin,
                    delta,
                    self.nodes[right_idx as usize].bounds(),
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
        (best, visited)
    }

    /// Continuous swept-sphere query returning the earliest surface `TOI` in `[0.0, 1.0]`.
    #[must_use]
    pub fn sweep_sphere(&self, origin: Vec3, radius: f64, delta: Vec3) -> Option<SweptSphereHit> {
        self.sweep_sphere_counted(origin, radius, delta).0
    }

    /// Queries the closest surface point within `max_radius`, returning `(closest_point, outward_normal, distance)`.
    #[must_use]
    pub fn closest_point(&self, point: Vec3, max_radius: f64) -> Option<(Vec3, Vec3, f64)> {
        if self.nodes.is_empty() {
            return None;
        }
        let r_clamped = max_radius.max(0.0);
        let mut best_dist_sq = r_clamped * r_clamped;
        let mut best: Option<(Vec3, Vec3, f64)> = None;
        let eps = geometric_epsilon();
        let mut stack = [0_u32; verdant_tensor::FLAT_BVH_STACK_CAP];
        let mut sp = 0usize;

        if aabb_point_dist_sq(self.nodes[0].bounds(), point) <= best_dist_sq {
            stack[0] = 0;
            sp = 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp] as usize;
            let node = &self.nodes[idx];
            if aabb_point_dist_sq(node.bounds(), point) > best_dist_sq {
                continue;
            }
            if node.prim_count > 0 {
                let start = node.right_or_first as usize;
                let end = start + (node.prim_count as usize);
                for &ti_u32 in &self.prim_indices[start..end] {
                    let tri = &self.triangles[ti_u32 as usize];
                    if aabb_point_dist_sq(tri.bounds, point) > best_dist_sq {
                        continue;
                    }
                    let cp = closest_point_on_triangle(point, tri.v0, tri.v1, tri.v2);
                    let diff = point - cp;
                    let d_sq = diff.length_squared();
                    if d_sq <= best_dist_sq {
                        best_dist_sq = d_sq;
                        let dist = d_sq.sqrt();
                        let normal = if dist > eps {
                            diff.scale(1.0 / dist)
                        } else if (point - tri.v0).dot(tri.face_normal) >= 0.0 {
                            tri.face_normal
                        } else {
                            Vec3::ZERO - tri.face_normal
                        };
                        best = Some((cp, normal, dist));
                    }
                }
            } else {
                let left_idx = (idx + 1) as u32;
                let right_idx = node.right_or_first;
                let dl = aabb_point_dist_sq(self.nodes[left_idx as usize].bounds(), point);
                let dr = aabb_point_dist_sq(self.nodes[right_idx as usize].bounds(), point);
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
}
