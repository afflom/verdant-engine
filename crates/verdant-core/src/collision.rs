//! 3D Gilbert-Johnson-Keerthi (`GJK`) convex collision detection and Coulomb contact solver.
//!
//! Iteration caps, simplex bounds, Baumgarte stabilization, and Coulomb friction cone projections
//! execute directly through the Lean-verified `crate::prism_kernel` emitted by Prism.

use serde::{Deserialize, Serialize};

use crate::error::CollisionError;
use crate::math::{geometric_epsilon, half_f64, Aabb, Vec3};

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

    /// Furthest world-space vertex along `direction`.
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
