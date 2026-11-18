//! 3D Gilbert-Johnson-Keerthi (`GJK`) convex collision detection and Coulomb contact solver.
//!
//! Iteration caps, simplex bounds, Baumgarte stabilization, and Coulomb friction cone projections
//! execute directly through the Lean-verified `crate::prism_kernel` emitted by Prism.

use serde::{Deserialize, Serialize};

use crate::error::CollisionError;
use crate::geometry::ProceduralMesh;
use crate::math::{geometric_epsilon, half_f64, two_f64, Aabb, Mat4, Ray, Vec3, DIM_SPATIAL};

/// Convex polyhedral collider defined by a finite cloud of local-space vertices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvexHullCollider {
    /// Local-space hull vertices in meters.
    pub vertices: Vec<Vec3>,
}

impl ConvexHullCollider {
    /// Validates a non-empty, finite vertex set for convex support queries.
    pub fn new(vertices: Vec<Vec3>) -> Result<Self, CollisionError> {
        if vertices.is_empty()
            || vertices
                .iter()
                .any(|v| !v.x.is_finite() || !v.y.is_finite() || !v.z.is_finite())
        {
            return Err(CollisionError::EmptyOrDegenerateHull {
                vertex_count: vertices.len(),
            });
        }
        Ok(Self { vertices })
    }

    /// Builds an axis-aligned box collider from positive `half_extents`.
    pub fn from_box_half_extents(half_extents: Vec3) -> Result<Self, CollisionError> {
        if !half_extents.x.is_finite()
            || !half_extents.y.is_finite()
            || !half_extents.z.is_finite()
            || half_extents.x <= 0.0
            || half_extents.y <= 0.0
            || half_extents.z <= 0.0
        {
            return Err(CollisionError::EmptyOrDegenerateHull { vertex_count: 0 });
        }
        let xs = [-half_extents.x, half_extents.x];
        let ys = [-half_extents.y, half_extents.y];
        let zs = [-half_extents.z, half_extents.z];
        let mut verts = Vec::new();
        for &x in &xs {
            for &y in &ys {
                for &z in &zs {
                    verts.push(Vec3::new(x, y, z));
                }
            }
        }
        Self::new(verts)
    }

    /// World-space AABB at `world_offset`.
    #[must_use]
    pub fn world_aabb(&self, world_offset: Vec3) -> Aabb {
        let first = self.vertices[0] + world_offset;
        let mut min = first;
        let mut max = first;
        for &v in &self.vertices {
            let p = v + world_offset;
            min = Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
            max = Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
        }
        Aabb { min, max }
    }

    /// Furthest world-space vertex along `direction` (`0` heap allocations).
    #[must_use]
    pub fn support(&self, world_offset: Vec3, direction: Vec3) -> Vec3 {
        let mut best = self.vertices[0] + world_offset;
        let mut best_dot = best.dot(direction);
        for &v in self.vertices.iter().skip(1) {
            let p = v + world_offset;
            let d = p.dot(direction);
            if d > best_dot {
                best_dot = d;
                best = p;
            }
        }
        best
    }

    /// Evaluates batch GJK directional support vertices across `directions` via tropical `(max, +)`
    /// contractions in `verdant_tensor::tropical_gjk_support_batch`.
    pub fn support_batch_tropical(
        &self,
        world_offset: Vec3,
        directions: &[Vec3],
    ) -> Result<Vec<Vec3>, CollisionError> {
        if self.vertices.is_empty() || directions.is_empty() {
            return Err(CollisionError::EmptyOrDegenerateHull {
                vertex_count: self.vertices.len(),
            });
        }
        let scale = verdant_tensor::kernel_consts().skin_weight_scale_f64;
        let dirs_q: Vec<[i32; 1 + 1 + 1]> = directions
            .iter()
            .map(|d| {
                let u = d.normalized();
                [
                    (u.x * scale).round() as i32,
                    (u.y * scale).round() as i32,
                    (u.z * scale).round() as i32,
                ]
            })
            .collect();
        let verts_q: Vec<[i32; 1 + 1 + 1]> = self
            .vertices
            .iter()
            .map(|&v| {
                let p = v + world_offset;
                [
                    (p.x * scale).round() as i32,
                    (p.y * scale).round() as i32,
                    (p.z * scale).round() as i32,
                ]
            })
            .collect();
        let winners =
            verdant_tensor::tropical_gjk_support_batch(&dirs_q, &verts_q).map_err(|_| {
                CollisionError::EmptyOrDegenerateHull {
                    vertex_count: self.vertices.len(),
                }
            })?;
        Ok(winners
            .into_iter()
            .map(|(idx, _)| self.vertices[idx] + world_offset)
            .collect())
    }
}

/// Narrowphase GJK collision result and minimum separation manifold.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactManifold {
    /// Whether the Minkowski difference contains the origin.
    pub intersecting: bool,
    /// Unit contact normal pointing from `hull_a` toward `hull_b`.
    pub normal: Vec3,
    /// Positive penetration depth in meters when `intersecting` is true (`0.0` otherwise).
    pub penetration_meters: f64,
    /// Number of bounded GJK support iterations executed.
    pub iterations: u32,
}

fn triple_cross(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    b.scale(a.dot(c)) - a.scale(b.dot(c))
}

fn update_simplex(simplex: &mut Vec<Vec3>, dir: &mut Vec3) -> bool {
    let eps = geometric_epsilon();
    let two = 1usize + 1usize;
    let three = two + 1usize;
    let four = three + 1usize;

    if simplex.len() == two {
        let a = simplex[1];
        let b = simplex[0];
        let ab = b - a;
        let ao = Vec3::ZERO - a;
        if ab.dot(ao) > 0.0 {
            let perp = triple_cross(ab, ao, ab);
            *dir = if perp.length() > eps {
                perp
            } else {
                Vec3::new(-ab.y, ab.x, 0.0)
            };
        } else {
            *simplex = vec![a];
            *dir = ao;
        }
        return false;
    }

    if simplex.len() == three {
        let a = simplex[two];
        let b = simplex[1];
        let c = simplex[0];
        let ab = b - a;
        let ac = c - a;
        let ao = Vec3::ZERO - a;
        let abc = ab.cross(ac);

        if abc.cross(ac).dot(ao) > 0.0 {
            if ac.dot(ao) > 0.0 {
                *simplex = vec![c, a];
                *dir = triple_cross(ac, ao, ac);
            } else {
                *simplex = vec![b, a];
                return update_simplex(simplex, dir);
            }
        } else if ab.cross(abc).dot(ao) > 0.0 {
            *simplex = vec![b, a];
            return update_simplex(simplex, dir);
        } else if abc.dot(ao).abs() <= eps {
            return true;
        } else if abc.dot(ao) > 0.0 {
            *dir = abc;
        } else {
            *simplex = vec![b, c, a];
            *dir = Vec3::ZERO - abc;
        }
        return false;
    }

    if simplex.len() == four {
        let a = simplex[three];
        let b = simplex[two];
        let c = simplex[1];
        let d = simplex[0];
        let ao = Vec3::ZERO - a;
        let ab = b - a;
        let ac = c - a;
        let ad = d - a;

        let abc = ab.cross(ac);
        let acd = ac.cross(ad);
        let adb = ad.cross(ab);

        if abc.dot(ao) > eps {
            *simplex = vec![c, b, a];
            return update_simplex(simplex, dir);
        }
        if acd.dot(ao) > eps {
            *simplex = vec![d, c, a];
            return update_simplex(simplex, dir);
        }
        if adb.dot(ao) > eps {
            *simplex = vec![b, d, a];
            return update_simplex(simplex, dir);
        }
        return true;
    }

    false
}

fn minkowski_depth_along(
    hull_a: &ConvexHullCollider,
    pos_a: Vec3,
    hull_b: &ConvexHullCollider,
    pos_b: Vec3,
    axis: Vec3,
) -> Option<(Vec3, f64)> {
    let len = axis.length();
    if len <= geometric_epsilon() {
        return None;
    }
    let unit = axis.scale(1.0 / len);
    let max_a = hull_a.support(pos_a, unit).dot(unit);
    let min_b = hull_b.support(pos_b, Vec3::ZERO - unit).dot(unit);
    let overlap = max_a - min_b;
    if overlap >= 0.0 {
        Some((unit, overlap))
    } else {
        None
    }
}

/// Runs bounded 3D GJK Minkowski intersection and extracts a deterministic contact manifold.
pub fn gjk_intersect_and_manifold(
    hull_a: &ConvexHullCollider,
    pos_a: Vec3,
    hull_b: &ConvexHullCollider,
    pos_b: Vec3,
) -> Result<ContactManifold, CollisionError> {
    if hull_a.vertices.is_empty() || hull_b.vertices.is_empty() {
        return Err(CollisionError::EmptyOrDegenerateHull { vertex_count: 0 });
    }

    let max_iters =
        crate::prism_kernel::gjkMaxIterations().expect("verified Prism gjkMaxIterations") as u32;
    let max_simplex = crate::prism_kernel::gjkSimplexMaxVertices()
        .expect("verified Prism gjkSimplexMaxVertices") as usize;

    let delta = pos_b - pos_a;
    let mut dir = if delta.length() > geometric_epsilon() {
        delta
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    let first = hull_a.support(pos_a, dir) - hull_b.support(pos_b, Vec3::ZERO - dir);
    let mut simplex = Vec::with_capacity(max_simplex);
    simplex.push(first);
    dir = Vec3::ZERO - first;

    let mut intersecting = false;
    let mut iterations = 0_u32;
    while iterations < max_iters {
        iterations += 1;
        if dir.length() <= geometric_epsilon() {
            intersecting = true;
            break;
        }
        let p = hull_a.support(pos_a, dir) - hull_b.support(pos_b, Vec3::ZERO - dir);
        if p.dot(dir) < 0.0 {
            intersecting = false;
            break;
        }
        simplex.push(p);
        if simplex.len() > max_simplex {
            simplex.remove(0);
        }
        if update_simplex(&mut simplex, &mut dir) {
            intersecting = true;
            break;
        }
    }

    if !intersecting {
        let fallback_n = if delta.length() > geometric_epsilon() {
            delta.normalized()
        } else {
            Vec3::UP
        };
        return Ok(ContactManifold {
            intersecting: false,
            normal: fallback_n,
            penetration_meters: 0.0,
            iterations,
        });
    }

    let candidates = [
        delta,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
    ];

    let mut best_normal = Vec3::UP;
    let mut best_depth = f64::INFINITY;
    for axis in candidates {
        if let Some((n, depth)) = minkowski_depth_along(hull_a, pos_a, hull_b, pos_b, axis) {
            if depth < best_depth && (delta.length() <= geometric_epsilon() || n.dot(delta) >= 0.0)
            {
                best_depth = depth;
                best_normal = n;
            }
        }
    }
    if !best_depth.is_finite() {
        best_depth = half_f64() * geometric_epsilon();
    }

    Ok(ContactManifold {
        intersecting: true,
        normal: best_normal,
        penetration_meters: best_depth,
        iterations,
    })
}

/// Normal and Coulomb-clamped tangential impulses computed by the Prism contact solver.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolvedContactImpulse {
    /// Non-negative normal impulse in `N*s`.
    pub normal_impulse_ns: f64,
    /// Coulomb-cone-clamped tangential friction impulse magnitude in `N*s`.
    pub tangent_impulse_ns: f64,
    /// Proof witness from `prism_kernel::coulombFrictionConeValid`.
    pub friction_cone_valid: bool,
}

/// Solves normal + Baumgarte stabilization and Coulomb friction impulses via `crate::prism_kernel`.
#[allow(clippy::too_many_arguments)]
pub fn solve_contact_impulse(
    closing_velocity_mps: f64,
    tangent_velocity_mps: f64,
    effective_mass_kg: f64,
    restitution: f64,
    friction_mu: f64,
    penetration_meters: f64,
    slop_meters: f64,
    dt_seconds: f64,
) -> Result<SolvedContactImpulse, CollisionError> {
    if !effective_mass_kg.is_finite()
        || !dt_seconds.is_finite()
        || effective_mass_kg <= 0.0
        || dt_seconds <= 0.0
    {
        return Err(CollisionError::InvalidContactSolverInputs {
            effective_mass_kg,
            dt_seconds,
        });
    }

    let scale =
        crate::prism_kernel::skinWeightScale().expect("verified Prism skinWeightScale") as f64;
    let closing_milli = (closing_velocity_mps.max(0.0) * scale).round() as u64;
    let rest_permille = (restitution.clamp(0.0, 1.0) * scale).round() as u64;
    let pen_milli = (penetration_meters.max(0.0) * scale).round() as u64;
    let slop_milli = (slop_meters.max(0.0) * scale).round() as u64;
    let dt_millis = (dt_seconds * scale).round().max(1.0) as u64;

    let delta_vn_milli = crate::prism_kernel::solveContactImpulseMilli(
        closing_milli,
        rest_permille,
        pen_milli,
        slop_milli,
        dt_millis,
    )
    .expect("verified Prism solveContactImpulseMilli");

    let normal_impulse_milli = ((delta_vn_milli as f64) * effective_mass_kg).round() as u64;
    let desired_tangent_milli =
        (tangent_velocity_mps.abs() * effective_mass_kg * scale).round() as u64;
    let mu_milli = (friction_mu.max(0.0) * scale).round() as u64;

    let clamped_tangent_milli = crate::prism_kernel::clampCoulombFrictionMilli(
        desired_tangent_milli,
        normal_impulse_milli,
        mu_milli,
    )
    .expect("verified Prism clampCoulombFrictionMilli");

    let cone_ok = crate::prism_kernel::coulombFrictionConeValid(
        clamped_tangent_milli,
        normal_impulse_milli,
        mu_milli,
    )
    .expect("verified Prism coulombFrictionConeValid");

    Ok(SolvedContactImpulse {
        normal_impulse_ns: (normal_impulse_milli as f64) / scale,
        tangent_impulse_ns: (clamped_tangent_milli as f64) / scale,
        friction_cone_valid: cone_ok,
    })
}

/// Exact triangle surface intersection returned by [`MeshBlas::raycast_exact`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlasTriangleHit {
    /// Index of the intersected triangle (`0..triangle_count`).
    pub triangle_index: usize,
    /// Ray distance `t >= 0` in space units.
    pub distance_meters: f64,
    /// Hit point `ray.origin + t * ray.direction`.
    pub point: Vec3,
    /// Outward-oriented geometric face normal opposing `ray.direction`.
    pub geometric_normal: Vec3,
    /// Barycentrically interpolated unit shading normal.
    pub shading_normal: Vec3,
    /// Barycentric weights `[w0, w1, w2]` summing to `1.0`.
    pub barycentrics: [f64; DIM_SPATIAL],
}

/// Exact continuous time-of-impact (`TOI`) hit returned by [`MeshBlas::sweep_sphere`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SweptSphereHit {
    /// Normalized time-of-impact `t` in `[0.0, 1.0]`.
    pub toi: f64,
    /// Surface contact point touched at `origin + toi * delta`.
    pub contact_point: Vec3,
    /// Unit outward contact normal pointing from the triangle surface toward the sphere center.
    pub contact_normal: Vec3,
    /// Index of the intersected triangle (`0..triangle_count`).
    pub triangle_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct BlasTriangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
    face_normal: Vec3,
    bounds: Aabb,
    tri_index: usize,
}

/// Cache-line-packed 32-byte linear BVH node (`2` nodes per `64`-byte cache line, zero `Box` pointers).
///
/// Pre-order layout invariant (`prism_kernel::flatBvhNodeBoundsValid`):
/// - When `prim_count > 0` (leaf): primitives live in `prim_indices[right_or_first .. right_or_first + prim_count]`.
/// - When `prim_count == 0` (internal): `left_child = node_index + 1` and `right_child = right_or_first`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlatBvhNode {
    /// Conservative lower `f32` AABB corner in local/world space.
    pub aabb_min: [f32; DIM_SPATIAL],
    /// First primitive offset (when `prim_count > 0`) or right child node index (when `prim_count == 0`).
    pub right_or_first: u32,
    /// Conservative upper `f32` AABB corner in local/world space.
    pub aabb_max: [f32; DIM_SPATIAL],
    /// Primitive count (`0` for internal nodes, `>= 1` for leaf nodes).
    pub prim_count: u32,
}

impl FlatBvhNode {
    /// Converts `f64` metric `Aabb` into conservative `f32` bounds guaranteed to enclose `bounds`.
    #[inline]
    #[must_use]
    pub fn pack_bounds(bounds: Aabb) -> ([f32; DIM_SPATIAL], [f32; DIM_SPATIAL]) {
        (
            bounds.min.to_f32_array().map(f32::next_down),
            bounds.max.to_f32_array().map(f32::next_up),
        )
    }

    /// Unpacks conservative `Aabb` for exact ray/sweep slab tests.
    #[inline]
    #[must_use]
    pub fn bounds(&self) -> Aabb {
        let i2 = 1usize + 1usize;
        Aabb {
            min: Vec3::new(
                f64::from(self.aabb_min[0]),
                f64::from(self.aabb_min[1]),
                f64::from(self.aabb_min[i2]),
            ),
            max: Vec3::new(
                f64::from(self.aabb_max[0]),
                f64::from(self.aabb_max[1]),
                f64::from(self.aabb_max[i2]),
            ),
        }
    }
}

/// Bottom-Level Acceleration Structure (`BLAS`) indexing a mesh's exact triangles in a contiguous
/// 32-byte binned-SAH `FlatBvhNode` array for `O(log N)` zero-allocation raycasts, sweeps, and clearance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeshBlas {
    triangles: Vec<BlasTriangle>,
    nodes: Vec<FlatBvhNode>,
    prim_indices: Vec<u32>,
}

fn closest_point_on_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }

    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let denom = (d1 - d3).max(geometric_epsilon());
        return a + ab.scale(d1 / denom);
    }

    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let denom = (d2 - d6).max(geometric_epsilon());
        return a + ac.scale(d2 / denom);
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let denom = ((d4 - d3) + (d5 - d6)).max(geometric_epsilon());
        return b + (c - b).scale((d4 - d3) / denom);
    }

    let denom = (va + vb + vc).max(geometric_epsilon());
    let v = vb / denom;
    let w = vc / denom;
    a + ab.scale(v) + ac.scale(w)
}

fn point_in_triangle_barycentric(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> bool {
    let eps = geometric_epsilon();
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let denom = d00 * d11 - d01 * d01;
    if denom.abs() <= eps {
        return false;
    }
    let inv = 1.0 / denom;
    let v = (d11 * d20 - d01 * d21) * inv;
    let w = (d00 * d21 - d01 * d20) * inv;
    let u = 1.0 - v - w;
    u >= -eps && v >= -eps && w >= -eps
}

fn smallest_unit_quadratic_root(a_coef: f64, b_coef: f64, c_coef: f64) -> Option<f64> {
    let eps = geometric_epsilon();
    let two = two_f64();
    let four = two * two;
    if a_coef <= eps {
        return None;
    }
    let disc = b_coef * b_coef - four * a_coef * c_coef;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let inv_2a = 1.0 / (two * a_coef);
    let r0 = (-b_coef - sqrt_disc) * inv_2a;
    if (0.0..=1.0).contains(&r0) {
        return Some(r0);
    }
    let r1 = (-b_coef + sqrt_disc) * inv_2a;
    if (0.0..=1.0).contains(&r1) {
        return Some(r1);
    }
    None
}

fn sweep_sphere_against_triangle(
    origin: Vec3,
    radius: f64,
    delta: Vec3,
    tri: &BlasTriangle,
) -> Option<SweptSphereHit> {
    let eps = geometric_epsilon();
    let two = two_f64();

    // 1. Check initial contact or penetration if moving toward the triangle:
    let cp0 = closest_point_on_triangle(origin, tri.v0, tri.v1, tri.v2);
    let diff0 = origin - cp0;
    let dist0_sq = diff0.length_squared();
    if dist0_sq <= radius * radius {
        let normal0 = if dist0_sq > eps * eps {
            diff0.scale(1.0 / dist0_sq.sqrt())
        } else if (origin - tri.v0).dot(tri.face_normal) >= 0.0 {
            tri.face_normal
        } else {
            Vec3::ZERO - tri.face_normal
        };
        if delta.dot(normal0) < -eps {
            return Some(SweptSphereHit {
                toi: 0.0,
                contact_point: cp0,
                contact_normal: normal0,
                triangle_index: tri.tri_index,
            });
        }
    }

    let mut best: Option<SweptSphereHit> = None;
    let mut consider = |toi: f64, contact_point: Vec3, contact_normal: Vec3| {
        if !(0.0..=1.0).contains(&toi) || delta.dot(contact_normal) >= -eps {
            return;
        }
        if best.is_none_or(|prev| toi < prev.toi) {
            best = Some(SweptSphereHit {
                toi,
                contact_point,
                contact_normal,
                triangle_index: tri.tri_index,
            });
        }
    };

    // 2. Interior plane sweep:
    let n_side = if (origin - tri.v0).dot(tri.face_normal) >= 0.0 {
        tri.face_normal
    } else {
        Vec3::ZERO - tri.face_normal
    };
    let d0 = (origin - tri.v0).dot(n_side);
    let dn = delta.dot(n_side);
    if dn < -eps && d0 >= radius {
        let t_plane = (radius - d0) / dn;
        if (0.0..=1.0).contains(&t_plane) {
            let p_plane = origin + delta.scale(t_plane) - n_side.scale(radius);
            if point_in_triangle_barycentric(p_plane, tri.v0, tri.v1, tri.v2) {
                consider(t_plane, p_plane, n_side);
            }
        }
    }

    // 3. Edge cylinder sweeps:
    let edges = [(tri.v0, tri.v1), (tri.v1, tri.v2), (tri.v2, tri.v0)];
    for (p, q) in edges {
        let e = q - p;
        let ee = e.dot(e);
        if ee <= eps {
            continue;
        }
        let m = origin - p;
        let m_perp = m - e.scale(m.dot(e) / ee);
        let d_perp = delta - e.scale(delta.dot(e) / ee);
        let a_c = d_perp.length_squared();
        let b_c = two * m_perp.dot(d_perp);
        let c_c = m_perp.length_squared() - radius * radius;
        if b_c < 0.0 {
            if let Some(t_edge) = smallest_unit_quadratic_root(a_c, b_c, c_c) {
                let s = (m + delta.scale(t_edge)).dot(e) / ee;
                if (0.0..=1.0).contains(&s) {
                    let cp = p + e.scale(s);
                    let center_at_t = origin + delta.scale(t_edge);
                    let n_edge = (center_at_t - cp).normalized();
                    consider(t_edge, cp, n_edge);
                }
            }
        }
    }

    // 4. Vertex sphere sweeps:
    let a_v = delta.length_squared();
    for v in [tri.v0, tri.v1, tri.v2] {
        let m = origin - v;
        let b_v = two * m.dot(delta);
        let c_v = m.length_squared() - radius * radius;
        if b_v < 0.0 {
            if let Some(t_vert) = smallest_unit_quadratic_root(a_v, b_v, c_v) {
                let center_at_t = origin + delta.scale(t_vert);
                let n_vert = (center_at_t - v).normalized();
                consider(t_vert, v, n_vert);
            }
        }
    }

    best
}

// Sweeping a segment plus a ball against a face is equivalent to sweeping
// its lower ball against the triangle extruded by the negative segment.
fn sweep_capsule_against_triangle(
    origin: Vec3,
    axis: Vec3,
    radius: f64,
    delta: Vec3,
    tri: &BlasTriangle,
) -> Option<SweptSphereHit> {
    let mut best = sweep_sphere_against_triangle(origin, radius, delta, tri);
    if axis == Vec3::ZERO {
        return best;
    }
    let mut retain = |hit: SweptSphereHit| {
        if best.is_none_or(|prev| hit.toi < prev.toi) {
            best = Some(hit);
        }
    };
    if let Some(hit) = sweep_sphere_against_triangle(origin + axis, radius, delta, tri) {
        retain(hit);
    }
    for (a, b) in [(tri.v0, tri.v1), (tri.v1, tri.v2), (tri.v2, tri.v0)] {
        let edge = b - a;
        let cross = edge.cross(axis);
        if cross.length_squared() <= geometric_epsilon().powi(1 + 1) {
            continue;
        }
        let normal = (Vec3::ZERO - cross).normalized();
        let across = axis.cross(cross);
        for (v0, v1, v2) in [(a, b, b - axis), (a, b - axis, a - axis)] {
            let side = BlasTriangle {
                v0,
                v1,
                v2,
                face_normal: normal,
                ..*tri
            };
            if let Some(mut hit) = sweep_sphere_against_triangle(origin, radius, delta, &side) {
                // Lift the witness on the extruded quad back to its source edge.
                let along =
                    ((hit.contact_point - a).dot(across) / edge.dot(across)).clamp(0.0, 1.0);
                hit.contact_point = a + edge.scale(along);
                retain(hit);
            }
        }
    }
    best
}

pub(crate) fn segment_swept_aabb_toi(
    origin: Vec3,
    delta: Vec3,
    bounds: Aabb,
    radius: f64,
) -> Option<f64> {
    let eps = geometric_epsilon();
    let orig = [origin.x, origin.y, origin.z];
    let dir = [delta.x, delta.y, delta.z];
    let bmin = [
        bounds.min.x - radius,
        bounds.min.y - radius,
        bounds.min.z - radius,
    ];
    let bmax = [
        bounds.max.x + radius,
        bounds.max.y + radius,
        bounds.max.z + radius,
    ];

    let mut tmin = 0.0_f64;
    let mut tmax = 1.0_f64;
    for axis in 0..DIM_SPATIAL {
        if dir[axis].abs() <= eps {
            if orig[axis] < bmin[axis] || orig[axis] > bmax[axis] {
                return None;
            }
        } else {
            let inv = 1.0 / dir[axis];
            let mut t1 = (bmin[axis] - orig[axis]) * inv;
            let mut t2 = (bmax[axis] - orig[axis]) * inv;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);
            if tmin > tmax {
                return None;
            }
        }
    }
    Some(tmin)
}

fn aabb_half_area(b: Aabb) -> f64 {
    let d = b.max - b.min;
    let dx = d.x.max(0.0);
    let dy = d.y.max(0.0);
    let dz = d.z.max(0.0);
    dx * dy + dy * dz + dz * dx
}

pub(crate) fn aabb_point_dist_sq(bounds: Aabb, point: Vec3) -> f64 {
    let cx = point.x.max(bounds.min.x).min(bounds.max.x) - point.x;
    let cy = point.y.max(bounds.min.y).min(bounds.max.y) - point.y;
    let cz = point.z.max(bounds.min.z).min(bounds.max.z) - point.z;
    cx * cx + cy * cy + cz * cz
}

/// Builds a contiguous pre-order binned-SAH `FlatBvhNode` array over `prim_bounds`.
pub(crate) fn build_flat_sah_bvh(
    prim_bounds: &[Aabb],
    leaf_max_prims: usize,
) -> (Vec<FlatBvhNode>, Vec<u32>) {
    if prim_bounds.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut prim_indices: Vec<u32> = (0..prim_bounds.len() as u32).collect();
    let mut nodes = Vec::with_capacity(prim_bounds.len() * (1 + 1));

    fn emit_node(
        prim_bounds: &[Aabb],
        prim_indices: &mut [u32],
        base_offset: u32,
        leaf_cap: usize,
        nodes: &mut Vec<FlatBvhNode>,
    ) -> u32 {
        let node_idx = nodes.len() as u32;
        let mut total = prim_bounds[prim_indices[0] as usize];
        let first_c = total.center();
        let mut c_min = first_c;
        let mut c_max = first_c;

        for &pi in &prim_indices[1..] {
            let b = prim_bounds[pi as usize];
            total = total.union(b);
            let c = b.center();
            c_min = Vec3::new(c_min.x.min(c.x), c_min.y.min(c.y), c_min.z.min(c.z));
            c_max = Vec3::new(c_max.x.max(c.x), c_max.y.max(c.y), c_max.z.max(c.z));
        }

        let (aabb_min, aabb_max) = FlatBvhNode::pack_bounds(total);
        if prim_indices.len() <= leaf_cap {
            nodes.push(FlatBvhNode {
                aabb_min,
                right_or_first: base_offset,
                aabb_max,
                prim_count: prim_indices.len() as u32,
            });
            return node_idx;
        }

        let c_span = c_max - c_min;
        let axis = if c_span.x >= c_span.y && c_span.x >= c_span.z {
            0usize
        } else if c_span.y >= c_span.z {
            1usize
        } else {
            1usize + 1usize
        };
        let axis_min = [c_min.x, c_min.y, c_min.z][axis];
        let axis_span = [c_span.x, c_span.y, c_span.z][axis];
        let eps = geometric_epsilon();

        // 8-bin Surface Area Heuristic (`SAH_BIN_COUNT`) partition with deterministic median fallback:
        let mut split_mid = prim_indices.len() / (1 + 1);
        if axis_span > eps && prim_indices.len() > verdant_tensor::SAH_BIN_COUNT {
            const BINS: usize = verdant_tensor::SAH_BIN_COUNT;
            let mut bin_counts = [0usize; BINS];
            let mut bin_bounds: [Option<Aabb>; BINS] = [None; BINS];
            let scale = (BINS as f64) / axis_span;

            for &pi in prim_indices.iter() {
                let pb = prim_bounds[pi as usize];
                let pc = pb.center();
                let coord = [pc.x, pc.y, pc.z][axis];
                let b_idx = (((coord - axis_min) * scale).floor().max(0.0) as usize).min(BINS - 1);
                bin_counts[b_idx] += 1;
                bin_bounds[b_idx] = Some(match bin_bounds[b_idx] {
                    Some(acc) => acc.union(pb),
                    None => pb,
                });
            }

            let mut left_area = [0.0_f64; BINS - 1];
            let mut left_count = [0usize; BINS - 1];
            let mut run_box: Option<Aabb> = None;
            let mut run_cnt = 0usize;
            for i in 0..(BINS - 1) {
                run_cnt += bin_counts[i];
                if let Some(bb) = bin_bounds[i] {
                    run_box = Some(match run_box {
                        Some(acc) => acc.union(bb),
                        None => bb,
                    });
                }
                left_count[i] = run_cnt;
                left_area[i] = run_box.map_or(0.0, aabb_half_area);
            }

            let mut right_box: Option<Aabb> = None;
            let mut right_cnt = 0usize;
            let mut best_cost = f64::INFINITY;
            let mut best_bin_split: Option<usize> = None;
            for i in (0..(BINS - 1)).rev() {
                right_cnt += bin_counts[i + 1];
                if let Some(bb) = bin_bounds[i + 1] {
                    right_box = Some(match right_box {
                        Some(acc) => acc.union(bb),
                        None => bb,
                    });
                }
                if left_count[i] > 0 && right_cnt > 0 {
                    let r_area = right_box.map_or(0.0, aabb_half_area);
                    let cost = (left_count[i] as f64) * left_area[i] + (right_cnt as f64) * r_area;
                    if cost < best_cost {
                        best_cost = cost;
                        best_bin_split = Some(i);
                    }
                }
            }

            if let Some(split_bin) = best_bin_split {
                let threshold = axis_min + ((split_bin + 1) as f64) / scale;
                let mut write_left = 0usize;
                for cursor in 0..prim_indices.len() {
                    let pi = prim_indices[cursor];
                    let pc = prim_bounds[pi as usize].center();
                    let coord = [pc.x, pc.y, pc.z][axis];
                    if coord <= threshold {
                        prim_indices.swap(write_left, cursor);
                        write_left += 1;
                    }
                }
                if write_left > 0 && write_left < prim_indices.len() {
                    split_mid = write_left;
                }
            }
        }

        nodes.push(FlatBvhNode {
            aabb_min,
            right_or_first: 0,
            aabb_max,
            prim_count: 0,
        });

        let (left_slice, right_slice) = prim_indices.split_at_mut(split_mid);
        let left_idx = emit_node(prim_bounds, left_slice, base_offset, leaf_cap, nodes);
        debug_assert_eq!(left_idx, node_idx + 1);
        let right_idx = emit_node(
            prim_bounds,
            right_slice,
            base_offset + (split_mid as u32),
            leaf_cap,
            nodes,
        );
        nodes[node_idx as usize].right_or_first = right_idx;
        debug_assert!(crate::prism_kernel::flatBvhNodeBoundsValid(
            u64::from(node_idx),
            u64::from(right_idx),
            (nodes.len() as u64) + 1
        ));
        node_idx
    }

    emit_node(
        prim_bounds,
        &mut prim_indices,
        0,
        leaf_max_prims.max(1),
        &mut nodes,
    );
    (nodes, prim_indices)
}

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
                        // Push farther child first so LIFO stack visits closer child first!
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
