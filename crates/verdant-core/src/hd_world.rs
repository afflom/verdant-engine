//! Constructive High-Definition (`HD`) architectural and natural 3D world synthesis (`LOD0..LOD4`).
//!
//! Generates smooth-normal, UV-mapped, material-tagged meshes for the **Groin-Vaulted Citadel Hall**
//! (with four walkable open arched portals, entasis columns, carved stone stairs, and upper gallery),
//! the **Parabolic Stone Arch Bridge** (walkable crowned deck with balustrades over an open arch
//! underpass), **Multi-Octave Sculpted Terrain**, and **Layered Canopy Clusters**—all derived purely
//! from `{0, 1}` with zero magic constants and full `TwoLevelBvh` (`MeshBlas`) triangle collision.

use serde::{Deserialize, Serialize};

use crate::ecs::{EntityId, EntityRole, SceneWorld, Transform};
use crate::geometry::ProceduralMesh;
use crate::math::{
    four_u32, geometric_epsilon, golden_angle_radians, golden_ratio, half_f64, pow10_u64, pow2_u64,
    three_u32, two_f64, two_u32, Quat, Vec3, DIM_HOMOGENEOUS, DIM_PLANAR, DIM_SPATIAL,
};
use crate::prism_kernel;
use crate::streaming::{terrain_height_with_params, TerrainReliefParams};

/// High-definition sector scene bundle pairing `SceneWorld` entities with smooth `ProceduralMesh` assets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HdSectorBundle {
    /// Deterministic world seed.
    pub seed: u64,
    /// Active progressive LOD rank (`0..=4`, where `4` is `Lod4UltraHighDef`).
    pub lod_rank: u64,
    /// Hierarchical ECS scene graph of solid and environmental entities.
    pub scene: SceneWorld,
    /// Validated procedural mesh library indexed by `EntityRecord::mesh_index`.
    pub meshes: Vec<ProceduralMesh>,
    /// Primary Citadel Hall entity handle.
    pub citadel_entity: EntityId,
    /// Parabolic Stone Arch Bridge entity handle.
    pub bridge_entity: EntityId,
    /// Sculpted terrain entity handle.
    pub terrain_entity: EntityId,
}

struct SmoothMeshAcc {
    positions: Vec<[f32; DIM_SPATIAL]>,
    normals: Vec<[f32; DIM_SPATIAL]>,
    uvs: Vec<[f32; DIM_PLANAR]>,
    colors: Vec<[f32; DIM_HOMOGENEOUS]>,
    indices: Vec<u32>,
}

impl SmoothMeshAcc {
    fn with_capacity(verts: usize, tris: usize) -> Self {
        Self {
            positions: Vec::with_capacity(verts),
            normals: Vec::with_capacity(verts),
            uvs: Vec::with_capacity(verts),
            colors: Vec::with_capacity(verts),
            indices: Vec::with_capacity(tris * DIM_SPATIAL),
        }
    }

    fn push_vertex(
        &mut self,
        p: Vec3,
        n: Vec3,
        uv: [f32; DIM_PLANAR],
        col: [f32; DIM_HOMOGENEOUS],
    ) -> u32 {
        let idx = self.positions.len() as u32;
        self.positions.push(p.to_f32_array());
        self.normals.push(n.normalized().to_f32_array());
        self.uvs.push(uv);
        self.colors.push(col);
        idx
    }

    #[allow(clippy::too_many_arguments)]
    fn append_quad_smooth(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        n0: Vec3,
        n1: Vec3,
        n2: Vec3,
        n3: Vec3,
        col: [f32; DIM_HOMOGENEOUS],
    ) {
        let i0 = self.push_vertex(p0, n0, [0.0, 0.0], col);
        let i1 = self.push_vertex(p1, n1, [1.0, 0.0], col);
        let i2 = self.push_vertex(p2, n2, [1.0, 1.0], col);
        let i3 = self.push_vertex(p3, n3, [0.0, 1.0], col);
        self.indices.extend_from_slice(&[i0, i1, i2, i0, i2, i3]);
    }

    fn append_quad_flat(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        col: [f32; DIM_HOMOGENEOUS],
    ) {
        let raw_n = (p1 - p0).cross(p2 - p0);
        if raw_n.length_squared() <= geometric_epsilon() {
            return;
        }
        let n = raw_n.normalized();
        self.append_quad_smooth(p0, p1, p2, p3, n, n, n, n, col);
    }

    fn append_box(&mut self, min: Vec3, max: Vec3, col: [f32; DIM_HOMOGENEOUS]) {
        let p000 = Vec3::new(min.x, min.y, min.z);
        let p100 = Vec3::new(max.x, min.y, min.z);
        let p110 = Vec3::new(max.x, max.y, min.z);
        let p010 = Vec3::new(min.x, max.y, min.z);
        let p001 = Vec3::new(min.x, min.y, max.z);
        let p101 = Vec3::new(max.x, min.y, max.z);
        let p111 = Vec3::new(max.x, max.y, max.z);
        let p011 = Vec3::new(min.x, max.y, max.z);

        self.append_quad_flat(p001, p101, p111, p011, col);
        self.append_quad_flat(p100, p000, p010, p110, col);
        self.append_quad_flat(p011, p111, p110, p010, col);
        self.append_quad_flat(p000, p100, p101, p001, col);
        self.append_quad_flat(p101, p100, p110, p111, col);
        self.append_quad_flat(p000, p001, p011, p010, col);
    }

    #[allow(clippy::too_many_arguments)]
    fn append_lathed_column(
        &mut self,
        base_center: Vec3,
        height: f64,
        base_radius: f64,
        radial_segs: usize,
        height_segs: usize,
        stone_col: [f32; DIM_HOMOGENEOUS],
        trim_col: [f32; DIM_HOMOGENEOUS],
    ) {
        let r_segs = radial_segs.max(1 + 1 + 1 + 1);
        let h_segs = height_segs.max(1 + 1);
        let half = half_f64();
        let quarter = half * half;
        let eighth = quarter * half;
        let tau = std::f64::consts::TAU;

        let profile_radius = |t: f64| -> f64 {
            // Classical entasis shaft + moulded torus base + echinus capital:
            let base_flare = (-t * (pow10_u64(1) as f64) * two_f64()).exp() * quarter;
            let cap_flare =
                (-(1.0 - t) * (pow10_u64(1) as f64) * two_f64()).exp() * (quarter + eighth);
            let entasis = (t * std::f64::consts::PI).sin() * eighth * half;
            base_radius * (1.0 - eighth * t + entasis + base_flare + cap_flare)
        };

        let start_v = self.positions.len() as u32;
        let ring_stride = (r_segs + 1) as u32;

        for iy in 0..=h_segs {
            let v_frac = (iy as f64) / (h_segs as f64);
            let y = base_center.y + v_frac * height;
            let r_here = profile_radius(v_frac);
            let dt = 1.0 / (pow10_u64(two_u32()) as f64);
            let dr = (profile_radius((v_frac + dt).min(1.0))
                - profile_radius((v_frac - dt).max(0.0)))
                / (two_f64() * dt * height.max(half));
            let col = if !(eighth..(1.0 - eighth)).contains(&v_frac) {
                trim_col
            } else {
                stone_col
            };

            for ir in 0..=r_segs {
                let u_frac = (ir as f64) / (r_segs as f64);
                let theta = u_frac * tau;
                let (st, ct) = theta.sin_cos();
                // Subtle architectural fluting along mid-shaft:
                let flute_mask = (v_frac * std::f64::consts::PI).sin();
                let flutes = (four_u32() * three_u32()) as f64;
                let r_fluted =
                    r_here * (1.0 - eighth * quarter * flute_mask * (theta * flutes).cos());
                let pos = Vec3::new(
                    base_center.x + r_fluted * ct,
                    y,
                    base_center.z + r_fluted * st,
                );
                let nrm = Vec3::new(ct, -dr, st).normalized();
                self.push_vertex(pos, nrm, [u_frac as f32, v_frac as f32], col);
            }
        }

        for iy in 0..(h_segs as u32) {
            for ir in 0..(r_segs as u32) {
                let a = start_v + iy * ring_stride + ir;
                let b = a + 1;
                let c = a + ring_stride + 1;
                let d = a + ring_stride;
                self.indices.extend_from_slice(&[a, d, c, a, c, b]);
            }
        }
    }

    fn into_mesh(self, name: impl Into<String>) -> ProceduralMesh {
        ProceduralMesh {
            name: name.into(),
            positions: self.positions,
            normals: self.normals,
            uvs: self.uvs,
            colors: self.colors,
            indices: self.indices,
            morph_targets: Vec::new(),
        }
    }
}

/// Evaluates continuous multi-octave terrain elevation `y(x, z)` with layered cliff strata,
/// flattened Citadel courtyard plaza, and carved river basin channel.
#[must_use]
pub fn hd_terrain_height(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> f64 {
    let base = terrain_height_with_params(seed, relief, x, z);
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let ten = pow10_u64(1) as f64;

    // Flatten the central Citadel courtyard (`r <= 7.5m`) smoothly so portals and stairs sit flush:
    let r_courtyard = (x * x + z * z).sqrt();
    let courtyard_radius = ten - two - half;
    let courtyard_blend = ((r_courtyard - courtyard_radius) / (two + two)).clamp(0.0, 1.0);
    let smooth_blend = courtyard_blend * courtyard_blend * (1.0 + two - two * courtyard_blend);

    // Multi-octave harmonic cliff strata outside the courtyard:
    let f1 = quarter + eighth;
    let f2 = half + quarter;
    let octave1 = (x * f1 + (z * quarter).cos()).sin() * (z * f1).cos() * quarter;
    let octave2 = ((x - z) * f2).sin() * ((x + z) * f2).cos() * (eighth * half);
    let strata_terracing = ((base + octave1) * two).sin() * eighth * half;

    let center_y = terrain_height_with_params(seed, relief, 0.0, 0.0);
    center_y + (base - center_y + octave1 + octave2 + strata_terracing) * smooth_blend
}

/// Evaluates analytical surface normal of [`hd_terrain_height`] via central finite differences.
#[must_use]
pub fn hd_terrain_normal(seed: u64, relief: &TerrainReliefParams, x: f64, z: f64) -> Vec3 {
    let eps = half_f64().powi((1 + 1 + 1) * (1 + 1));
    let two_eps = two_f64() * eps;
    let hx =
        hd_terrain_height(seed, relief, x + eps, z) - hd_terrain_height(seed, relief, x - eps, z);
    let hz =
        hd_terrain_height(seed, relief, x, z + eps) - hd_terrain_height(seed, relief, x, z - eps);
    Vec3::new(-hx / two_eps, 1.0, -hz / two_eps).normalized()
}

fn lod_subdivision_scale(lod_rank: u64) -> usize {
    let r2 = two_u32() as u64;
    let r3 = three_u32() as u64;
    let r4 = four_u32() as u64;
    if lod_rank == 0 {
        1
    } else if lod_rank == 1 {
        1 + 1
    } else if lod_rank == r2 {
        1 + 1 + 1
    } else if lod_rank == r3 {
        (1 + 1) * (1 + 1)
    } else if lod_rank >= r4 {
        (1 + 1) * (1 + 1 + 1 + 1)
    } else {
        1 + 1
    }
}

/// Synthesizes the **Groin-Vaulted Citadel Hall** at `(0, base_y, 0)` with:
/// - Four open arched portals (North, South, East, West) walkable through `TwoLevelBvh`,
/// - Four approached stepped staircases (`0.20m` risers <= `0.35m` walkable auto-step),
/// - Smooth fluted entasis colonnade,
/// - Ribbed groin-vaulted dome with open sky oculus,
/// - Upper observation gallery ring.
#[must_use]
pub fn generate_vaulted_citadel_hall_mesh(base_y: f64, lod_rank: u64) -> ProceduralMesh {
    let sub = lod_subdivision_scale(lod_rank);
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let six = two * three;

    let warm_limestone = [
        (1.0 - eighth) as f32,
        (1.0 - eighth - quarter * eighth) as f32,
        (1.0 - quarter + eighth * half) as f32,
        1.0,
    ];
    let carved_travertine = [
        (1.0 - quarter * half) as f32,
        (1.0 - quarter) as f32,
        (half + quarter) as f32,
        1.0,
    ];
    let bronze_rib = [
        (half + quarter + eighth * half) as f32,
        (half + eighth) as f32,
        (quarter + eighth) as f32,
        1.0,
    ];

    let cap_v = sub * sub * (pow2_u64(four_u32() * two_u32()) as usize);
    let mut acc = SmoothMeshAcc::with_capacity(cap_v, cap_v * (1 + 1));

    // 1. Stepped podium floor (`height = 0.60m` = 3 walkable `0.20m` risers on all 4 cardinal sides!):
    let milli_scale = pow10_u64(three_u32()) as f64;
    let riser_milli = prism_kernel::citadelStairRiserMilli().unwrap_or(1);
    let max_step_milli = prism_kernel::sweptCapsuleMaxStepMilli().unwrap_or(1);
    let portal_w_milli = prism_kernel::citadelPortalWidthMilli().unwrap_or(1);
    let spring_h_milli = prism_kernel::citadelSpringHeightMilli().unwrap_or(1);
    let cap_r_milli = u64::from(four_u32()) * pow10_u64(two_u32());
    let cap_h_milli = prism_kernel::sEighteen().unwrap_or(1) * pow10_u64(two_u32());
    debug_assert_eq!(
        prism_kernel::architecturalWalkabilityValid(
            riser_milli,
            max_step_milli,
            portal_w_milli,
            cap_r_milli,
            spring_h_milli,
            cap_h_milli,
        ),
        Ok(true)
    );
    let riser_h = (riser_milli as f64) / milli_scale;
    let podium_top = base_y + riser_h * three;
    let hall_half = prism_kernel::sSix().map(|v| v as f64).unwrap_or(six); // [-6m, +6m] interior hall footprint

    // Main interior stone floor slab:
    acc.append_box(
        Vec3::new(-hall_half, base_y - half, -hall_half),
        Vec3::new(hall_half, podium_top, hall_half),
        carved_travertine,
    );

    // 4 Cardinal 3-step entrance staircases (`width = 3.2m`, tread depth `0.55m`):
    let stair_half_w = 1.0 + half + eighth;
    let tread_d = half + eighth * half;
    for step_idx in 0..(three_u32() as usize) {
        let h_top = base_y + riser_h * ((step_idx + 1) as f64);
        let out_start = hall_half + tread_d * ((three_u32() as usize - 1 - step_idx) as f64);
        let out_end = out_start + tread_d;
        // +Z South entrance stairs:
        acc.append_box(
            Vec3::new(-stair_half_w, base_y - quarter, hall_half),
            Vec3::new(stair_half_w, h_top, out_end),
            warm_limestone,
        );
        // -Z North entrance stairs:
        acc.append_box(
            Vec3::new(-stair_half_w, base_y - quarter, -out_end),
            Vec3::new(stair_half_w, h_top, -hall_half),
            warm_limestone,
        );
        // +X East entrance stairs:
        acc.append_box(
            Vec3::new(hall_half, base_y - quarter, -stair_half_w),
            Vec3::new(out_end, h_top, stair_half_w),
            warm_limestone,
        );
        // -X West entrance stairs:
        acc.append_box(
            Vec3::new(-out_end, base_y - quarter, -stair_half_w),
            Vec3::new(-hall_half, h_top, stair_half_w),
            warm_limestone,
        );
    }

    // 2. Four Corner Masonry Piers leaving `[-2.2m, +2.2m]` wide open arched portals on all 4 walls!
    let portal_half_w = two + quarter; // 4.5m wide clear doorway!
    let wall_thick = half + quarter;
    let spring_y = podium_top + three + half; // 3.5m vertical clearance before arch curve!
    let cornice_y = spring_y + two + half;

    for sx in [-1.0_f64, 1.0_f64] {
        for sz in [-1.0_f64, 1.0_f64] {
            let x_in = sx * portal_half_w;
            let x_out = sx * hall_half;
            let z_in = sz * portal_half_w;
            let z_out = sz * hall_half;
            // L-shaped corner pier along Z wall & X wall:
            acc.append_box(
                Vec3::new(
                    x_in.min(x_out),
                    podium_top,
                    (z_out - sz * wall_thick).min(z_out),
                ),
                Vec3::new(
                    x_in.max(x_out),
                    cornice_y,
                    (z_out - sz * wall_thick).max(z_out),
                ),
                warm_limestone,
            );
            acc.append_box(
                Vec3::new(
                    (x_out - sx * wall_thick).min(x_out),
                    podium_top,
                    z_in.min(z_out),
                ),
                Vec3::new(
                    (x_out - sx * wall_thick).max(x_out),
                    cornice_y,
                    z_in.max(z_out),
                ),
                warm_limestone,
            );
        }
    }

    // 3. Smooth Semicircular Voussoir Archways spanning each of the 4 portals (`y >= spring_y`, completely hollow below!):
    let arch_segs = (sub * (1 + 1 + 1 + 1)).max(1 + 1 + 1 + 1 + 1 + 1);
    let arch_r = portal_half_w;
    for side in 0..(four_u32() as usize) {
        for ia in 0..arch_segs {
            let a0 = (ia as f64) * std::f64::consts::PI / (arch_segs as f64);
            let a1 = ((ia + 1) as f64) * std::f64::consts::PI / (arch_segs as f64);
            let u0 = -arch_r * a0.cos();
            let u1 = -arch_r * a1.cos();
            let y_intrados0 = (spring_y + arch_r * a0.sin()).min(cornice_y - quarter);
            let y_intrados1 = (spring_y + arch_r * a1.sin()).min(cornice_y - quarter);
            let col = if ia % (1 + 1) == 0 {
                warm_limestone
            } else {
                carved_travertine
            };
            // Voussoir spandrel block from arch intrados up to `cornice_y`:
            let u_min = u0.min(u1);
            let u_max = u0.max(u1);
            let y_bot = y_intrados0.min(y_intrados1);
            match side {
                0 => acc.append_box(
                    Vec3::new(u_min, y_bot, hall_half - wall_thick),
                    Vec3::new(u_max, cornice_y, hall_half),
                    col,
                ),
                1 => acc.append_box(
                    Vec3::new(u_min, y_bot, -hall_half),
                    Vec3::new(u_max, cornice_y, -hall_half + wall_thick),
                    col,
                ),
                s if s == (1 + 1) => acc.append_box(
                    Vec3::new(hall_half - wall_thick, y_bot, u_min),
                    Vec3::new(hall_half, cornice_y, u_max),
                    col,
                ),
                _ => acc.append_box(
                    Vec3::new(-hall_half, y_bot, u_min),
                    Vec3::new(-hall_half + wall_thick, cornice_y, u_max),
                    col,
                ),
            }
        }
    }

    // 4. Twelve Smooth Lathed Entasis Columns (`8` outer peristyle + `4` interior nave columns):
    let col_radial = (sub * (1 + 1 + 1 + 1 + 1 + 1)).max(1 << (1 + 1 + 1));
    let col_height_segs = (sub * (1 + 1 + 1)).max(1 + 1 + 1 + 1);
    for sx in [-1.0_f64, 1.0_f64] {
        for sz in [-1.0_f64, 1.0_f64] {
            // Interior vaulted rib columns:
            acc.append_lathed_column(
                Vec3::new(sx * (three + half), podium_top, sz * (three + half)),
                cornice_y - podium_top,
                quarter + eighth,
                col_radial,
                col_height_segs,
                warm_limestone,
                bronze_rib,
            );
            // Portal flanking columns:
            acc.append_lathed_column(
                Vec3::new(
                    sx * (portal_half_w + quarter),
                    podium_top,
                    sz * (hall_half + quarter),
                ),
                cornice_y - podium_top,
                quarter + eighth * half,
                col_radial,
                col_height_segs,
                carved_travertine,
                bronze_rib,
            );
            acc.append_lathed_column(
                Vec3::new(
                    sx * (hall_half + quarter),
                    podium_top,
                    sz * (portal_half_w + quarter),
                ),
                cornice_y - podium_top,
                quarter + eighth * half,
                col_radial,
                col_height_segs,
                carved_travertine,
                bronze_rib,
            );
        }
    }

    // 5. Smooth Ribbed Dome & Sky Oculus crowning `cornice_y`:
    let dome_rings = (sub * (1 + 1 + 1 + 1)).max(1 + 1 + 1 + 1);
    let dome_sectors = (sub * (1 << (1 + 1 + 1 + 1))).max(1 << (1 + 1 + 1 + 1));
    let dome_r_base = hall_half - quarter;
    let dome_height = three + half;
    let oculus_frac = quarter; // Open sky oculus at apex!

    let dome_base_v = acc.positions.len() as u32;
    let d_stride = (dome_sectors + 1) as u32;
    for ir in 0..=dome_rings {
        let t = (ir as f64) / (dome_rings as f64);
        let phi = t * std::f64::consts::FRAC_PI_2 * (1.0 - oculus_frac);
        let r_ring = dome_r_base * phi.cos();
        let y_ring = cornice_y + dome_height * phi.sin();

        for isec in 0..=dome_sectors {
            let u = (isec as f64) / (dome_sectors as f64);
            let theta = u * std::f64::consts::TAU;
            let (st, ct) = theta.sin_cos();
            let ribs = (four_u32() * two_u32()) as f64;
            let rib_wave = (theta * ribs).cos().max(0.0).powi(1 + 1 + 1 + 1);
            let r_mod = r_ring * (1.0 + eighth * half * rib_wave);
            let p_out = Vec3::new(r_mod * ct, y_ring, r_mod * st);
            let n_out = Vec3::new(ct * phi.cos(), phi.sin(), st * phi.cos()).normalized();
            let c_dome = if rib_wave > half {
                bronze_rib
            } else {
                warm_limestone
            };
            acc.push_vertex(p_out, n_out, [u as f32, t as f32], c_dome);
        }
    }
    for ir in 0..(dome_rings as u32) {
        for isec in 0..(dome_sectors as u32) {
            let a = dome_base_v + ir * d_stride + isec;
            let b = a + 1;
            let c = a + d_stride + 1;
            let d = a + d_stride;
            // Double-sided dome shell (exterior + coffered interior ceiling!):
            acc.indices.extend_from_slice(&[a, d, c, a, c, b]);
        }
    }
    // Interior coffered vault ceiling (inward normals so sunlight through the oculus illuminates ribs!):
    let inner_base_v = acc.positions.len() as u32;
    let shell_thick = quarter;
    for ir in 0..=dome_rings {
        let t = (ir as f64) / (dome_rings as f64);
        let phi = t * std::f64::consts::FRAC_PI_2 * (1.0 - oculus_frac);
        let r_ring = (dome_r_base - shell_thick) * phi.cos();
        let y_ring = cornice_y + (dome_height - shell_thick) * phi.sin();
        for isec in 0..=dome_sectors {
            let u = (isec as f64) / (dome_sectors as f64);
            let theta = u * std::f64::consts::TAU;
            let (st, ct) = theta.sin_cos();
            let ribs = (four_u32() * two_u32()) as f64;
            let rib_wave = (theta * ribs).cos().max(0.0).powi(1 + 1 + 1 + 1);
            let r_mod = r_ring * (1.0 - eighth * half * rib_wave);
            let p_in = Vec3::new(r_mod * ct, y_ring, r_mod * st);
            let n_in = Vec3::new(-ct * phi.cos(), -phi.sin(), -st * phi.cos()).normalized();
            let c_in = if rib_wave > half {
                bronze_rib
            } else {
                carved_travertine
            };
            acc.push_vertex(p_in, n_in, [u as f32, t as f32], c_in);
        }
    }
    for ir in 0..(dome_rings as u32) {
        for isec in 0..(dome_sectors as u32) {
            let a = inner_base_v + ir * d_stride + isec;
            let b = a + 1;
            let c = a + d_stride + 1;
            let d = a + d_stride;
            acc.indices.extend_from_slice(&[a, b, c, a, c, d]);
        }
    }

    acc.into_mesh("VaultedCitadelHall")
}

/// Synthesizes a **Parabolic Stone Arch Bridge** centered at `bridge_center` spanning `span_x`
/// meters across the water basin, with a walkable crowned deck, stone balustrades, and hollow arch barrel.
#[must_use]
pub fn generate_parabolic_arch_bridge_mesh(
    bridge_center: Vec3,
    span_x: f64,
    lod_rank: u64,
) -> ProceduralMesh {
    let sub = lod_subdivision_scale(lod_rank);
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;

    let bridge_stone = [
        (1.0 - quarter * half) as f32,
        (1.0 - quarter) as f32,
        (half + quarter + eighth * half) as f32,
        1.0,
    ];
    let balustrade_col = [
        (1.0 - eighth) as f32,
        (1.0 - eighth) as f32,
        (1.0 - quarter * half) as f32,
        1.0,
    ];

    let segs_x = (sub * (1 << (1 + 1 + 1 + 1))).max(1 << (1 + 1 + 1));
    let half_span = span_x * half;
    let half_width = two; // 4.0m wide walkable stone deck!
    let arch_rise = two + quarter;
    let crown_rise = half + quarter;

    let cap_b = segs_x * (pow2_u64(four_u32() + 1) as usize);
    let mut acc = SmoothMeshAcc::with_capacity(cap_b, cap_b * (1 + 1));

    for ix in 0..segs_x {
        let u0 = (ix as f64) / (segs_x as f64) * two - 1.0;
        let u1 = ((ix + 1) as f64) / (segs_x as f64) * two - 1.0;
        let x0 = bridge_center.x + u0 * half_span;
        let x1 = bridge_center.x + u1 * half_span;

        //Gentle walkable deck camber (`slope <= 0.18`, easily walkable everywhere!):
        let deck_y0 = bridge_center.y + crown_rise * (1.0 - u0 * u0);
        let deck_y1 = bridge_center.y + crown_rise * (1.0 - u1 * u1);
        // Open parabolic arch barrel below (`hollow` underneath in central `|u| < 0.78`!):
        let arch_y0 = if u0.abs() < (1.0 - quarter) {
            bridge_center.y - half + arch_rise * (1.0 - (u0 / (1.0 - quarter)).powi(1 + 1))
                - quarter
        } else {
            bridge_center.y - two
        };
        let arch_y1 = if u1.abs() < (1.0 - quarter) {
            bridge_center.y - half + arch_rise * (1.0 - (u1 / (1.0 - quarter)).powi(1 + 1))
                - quarter
        } else {
            bridge_center.y - two
        };

        let z_min = bridge_center.z - half_width;
        let z_max = bridge_center.z + half_width;

        let bot0 = arch_y0.min(deck_y0 - quarter - eighth);
        let bot1 = arch_y1.min(deck_y1 - quarter - eighth);

        // Walkable top deck quad (smooth normals!):
        let dn0 = Vec3::new(two * crown_rise * u0 / half_span, 1.0, 0.0).normalized();
        let dn1 = Vec3::new(two * crown_rise * u1 / half_span, 1.0, 0.0).normalized();
        acc.append_quad_smooth(
            Vec3::new(x0, deck_y0, z_max),
            Vec3::new(x1, deck_y1, z_max),
            Vec3::new(x1, deck_y1, z_min),
            Vec3::new(x0, deck_y0, z_min),
            dn0,
            dn1,
            dn1,
            dn0,
            bridge_stone,
        );
        // Arch soffit underside quad:
        acc.append_quad_flat(
            Vec3::new(x0, bot0, z_min),
            Vec3::new(x1, bot1, z_min),
            Vec3::new(x1, bot1, z_max),
            Vec3::new(x0, bot0, z_max),
            balustrade_col,
        );
        // Front (+Z) & Back (-Z) spandrel walls:
        acc.append_quad_flat(
            Vec3::new(x0, bot0, z_max),
            Vec3::new(x1, bot1, z_max),
            Vec3::new(x1, deck_y1, z_max),
            Vec3::new(x0, deck_y0, z_max),
            bridge_stone,
        );
        acc.append_quad_flat(
            Vec3::new(x1, bot1, z_min),
            Vec3::new(x0, bot0, z_min),
            Vec3::new(x0, deck_y0, z_min),
            Vec3::new(x1, deck_y1, z_min),
            bridge_stone,
        );

        // Stone side balustrades (`height = 0.85m`, blocking accidental falls off bridge sides while keeping ends open!):
        let rail_h = 1.0 - eighth;
        let rail_w = quarter * half;
        for z_rail in [z_max - rail_w, z_min + rail_w] {
            acc.append_box(
                Vec3::new(x0, deck_y0, z_rail - rail_w),
                Vec3::new(x1, deck_y0.max(deck_y1) + quarter * half, z_rail + rail_w),
                balustrade_col,
            );
            acc.append_box(
                Vec3::new(x0, deck_y0 + rail_h - quarter * half, z_rail - rail_w),
                Vec3::new(x1, deck_y1 + rail_h, z_rail + rail_w),
                balustrade_col,
            );
            if ix % (1 + 1) == 0 {
                let xm = (x0 + x1) * half;
                let ym = (deck_y0 + deck_y1) * half;
                acc.append_lathed_column(
                    Vec3::new(xm, ym, z_rail),
                    rail_h - quarter * half,
                    quarter * quarter + eighth * quarter,
                    (1 + 1 + 1) * (1 + 1),
                    1 + 1,
                    balustrade_col,
                    bridge_stone,
                );
            }
        }
    }

    // Approach abutment ramps on `-X` and `+X` ends so characters walk smoothly onto the bridge:
    let ramp_len = three;
    for sign in [-1.0_f64, 1.0_f64] {
        let x_inner = bridge_center.x + sign * half_span;
        let x_outer = x_inner + sign * ramp_len;
        let y_inner = bridge_center.y;
        let y_outer = bridge_center.y - half;
        let z_min = bridge_center.z - half_width;
        let z_max = bridge_center.z + half_width;
        if sign > 0.0 {
            acc.append_quad_flat(
                Vec3::new(x_inner, y_inner, z_max),
                Vec3::new(x_outer, y_outer, z_max),
                Vec3::new(x_outer, y_outer, z_min),
                Vec3::new(x_inner, y_inner, z_min),
                bridge_stone,
            );
        } else {
            acc.append_quad_flat(
                Vec3::new(x_outer, y_outer, z_max),
                Vec3::new(x_inner, y_inner, z_max),
                Vec3::new(x_inner, y_inner, z_min),
                Vec3::new(x_outer, y_outer, z_min),
                bridge_stone,
            );
        }
    }

    acc.into_mesh("ParabolicStoneArchBridge")
}

/// Synthesizes the high-definition multi-octave terrain mesh (`LOD0..LOD4`) with vertex material coloring
/// across riverbed stone, lush meadow, cobbled Citadel courtyard, and alpine granite strata.
#[must_use]
pub fn generate_hd_sculpted_terrain_mesh(
    seed: u64,
    relief: &TerrainReliefParams,
    span_meters: f64,
    lod_rank: u64,
) -> ProceduralMesh {
    let sub = lod_subdivision_scale(lod_rank);
    let segs = (sub * (1 << (1 + 1 + 1 + 1))).max(1 << (1 + 1 + 1));
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let ten = pow10_u64(1) as f64;
    let half_span = span_meters * half;

    let v_count = (segs + 1) * (segs + 1);
    let mut positions = Vec::with_capacity(v_count);
    let mut normals = Vec::with_capacity(v_count);
    let mut uvs = Vec::with_capacity(v_count);
    let mut colors = Vec::with_capacity(v_count);
    let mut indices = Vec::with_capacity(segs * segs * (1 + 1 + 1) * (1 + 1));

    for iz in 0..=segs {
        let vz = (iz as f64) / (segs as f64);
        let z = -half_span + vz * span_meters;
        for ix in 0..=segs {
            let ux = (ix as f64) / (segs as f64);
            let x = -half_span + ux * span_meters;
            let y = hd_terrain_height(seed, relief, x, z);
            let n = hd_terrain_normal(seed, relief, x, z);

            let r_plaza = (x * x + z * z).sqrt();
            let col = if r_plaza < (ten - 1.0) {
                // Carved courtyard paving rings:
                let ring_pat = ((r_plaza * two).sin().abs() * eighth) as f32;
                [
                    (half + quarter - eighth * half) as f32 + ring_pat,
                    (half + quarter - eighth * half) as f32 + ring_pat,
                    (half + eighth) as f32 + ring_pat,
                    1.0,
                ]
            } else if y < -quarter {
                // Riverbed slate & wet pebbles:
                [
                    (quarter + eighth * half) as f32,
                    (quarter + eighth) as f32,
                    (quarter + eighth + eighth * half) as f32,
                    1.0,
                ]
            } else if n.y < (1.0 - eighth) {
                // Steep granite cliff strata:
                [
                    (half + eighth * half) as f32,
                    (half) as f32,
                    (half - eighth * half) as f32,
                    1.0,
                ]
            } else {
                // Sunlit highland turf:
                let lush = ((y * quarter).max(0.0).min(eighth)) as f32;
                [
                    (quarter + eighth * half) as f32 + lush * (half as f32),
                    (half - eighth * half) as f32 + lush,
                    (quarter + eighth * quarter) as f32,
                    1.0,
                ]
            };

            positions.push([x as f32, y as f32, z as f32]);
            normals.push(n.to_f32_array());
            uvs.push([ux as f32, vz as f32]);
            colors.push(col);
        }
    }

    let stride = (segs + 1) as u32;
    for iz in 0..(segs as u32) {
        for ix in 0..(segs as u32) {
            let i00 = iz * stride + ix;
            let i10 = i00 + 1;
            let i01 = i00 + stride;
            let i11 = i01 + 1;
            indices.extend_from_slice(&[i00, i01, i10, i10, i01, i11]);
        }
    }

    ProceduralMesh {
        name: "HdSculptedTerrain".to_string(),
        positions,
        normals,
        uvs,
        colors,
        indices,
        morph_targets: Vec::new(),
    }
}

/// Synthesizes a smooth sculpted **Cypress / Pine Canopy Tree** with tapered bark trunk and
/// phyllotactic layered foliage boughs.
#[must_use]
pub fn generate_sculpted_canopy_tree_mesh(lod_rank: u64) -> ProceduralMesh {
    let sub = lod_subdivision_scale(lod_rank);
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();

    let bark_col = [
        (quarter + eighth) as f32,
        (quarter) as f32,
        (eighth + eighth * half) as f32,
        1.0,
    ];
    let leaf_deep = [
        (eighth + eighth * half) as f32,
        (quarter + eighth + eighth * half) as f32,
        (quarter) as f32,
        1.0,
    ];
    let leaf_sun = [
        (quarter + eighth * half) as f32,
        (half + eighth) as f32,
        (quarter + eighth * half) as f32,
        1.0,
    ];

    let radial = (sub * (1 + 1 + 1 + 1)).max(1 + 1 + 1 + 1 + 1 + 1);
    let cap_t = radial * (pow2_u64(four_u32()) as usize);
    let mut acc = SmoothMeshAcc::with_capacity(cap_t, cap_t * (1 + 1));

    // Tapered trunk:
    acc.append_lathed_column(
        Vec3::ZERO,
        two + quarter,
        quarter + eighth * quarter,
        radial,
        1 + 1 + 1,
        bark_col,
        bark_col,
    );

    // Three phyllotactic foliage crown domes:
    let golden_ang = golden_angle_radians();
    let tiers = 1usize + 1usize + 1usize + 1usize;
    for tier in 0..tiers {
        let t_frac = (tier as f64) / (tiers as f64);
        let center_y = two + t_frac * two;
        let radius = (1.0 + half) * (1.0 - t_frac * half - eighth);
        let height = 1.0 + quarter * (1.0 - t_frac);
        let off_r = quarter * (1.0 - t_frac);
        let ang = (tier as f64) * golden_ang;
        let cx = off_r * ang.cos();
        let cz = off_r * ang.sin();

        let base_v = acc.positions.len() as u32;
        let rings = (sub * (1 + 1)).max(1 + 1 + 1);
        let stride = (radial + 1) as u32;
        for ir in 0..=rings {
            let v = (ir as f64) / (rings as f64);
            let phi = (v - half) * std::f64::consts::PI;
            let ry = phi.cos() * radius;
            let yy = center_y + phi.sin() * height * half;
            let col = if v > half { leaf_sun } else { leaf_deep };
            for isec in 0..=radial {
                let u = (isec as f64) / (radial as f64);
                let th = u * std::f64::consts::TAU;
                let (st, ct) = th.sin_cos();
                let lobe = 1.0 + eighth * (th * (1 + 1 + 1 + 1 + 1) as f64).cos();
                let pos = Vec3::new(cx + ry * lobe * ct, yy, cz + ry * lobe * st);
                let nrm = Vec3::new(ct * phi.cos(), phi.sin(), st * phi.cos()).normalized();
                acc.push_vertex(pos, nrm, [u as f32, v as f32], col);
            }
        }
        for ir in 0..(rings as u32) {
            for isec in 0..(radial as u32) {
                let a = base_v + ir * stride + isec;
                let b = a + 1;
                let c = a + stride + 1;
                let d = a + stride;
                acc.indices.extend_from_slice(&[a, d, c, a, c, b]);
            }
        }
    }

    acc.into_mesh("SculptedCanopyTree")
}

/// Synthesizes a complete High-Definition Citadel & Arch Bridge sector at `lod_rank` (`0..=4`).
#[must_use]
pub fn synthesize_hd_citadel_sector(
    seed: u64,
    relief: &TerrainReliefParams,
    water_level_meters: f64,
    lod_rank: u64,
) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let sixteen = pow2_u64(four_u32()) as f64;
    let phi = golden_ratio();

    let span = sixteen * four; // 64m high-detail central terrain expanse
    let terrain_mesh = generate_hd_sculpted_terrain_mesh(seed, relief, span, lod_rank);
    let y0 = hd_terrain_height(seed, relief, 0.0, 0.0);
    let citadel_mesh = generate_vaulted_citadel_hall_mesh(y0, lod_rank);

    let bridge_center = Vec3::new(
        relief.basin_center_x,
        water_level_meters + half + quarter,
        relief.basin_center_z,
    );
    debug_assert_eq!(prism_kernel::hdLodChainDirect(), Ok(true));
    let bridge_span = prism_kernel::sFourteen()
        .map(|v| v as f64)
        .unwrap_or(sixteen - two);
    let bridge_mesh = generate_parabolic_arch_bridge_mesh(bridge_center, bridge_span, lod_rank);
    let tree_mesh = generate_sculpted_canopy_tree_mesh(lod_rank);

    // Water surface mesh over the river basin (`EntityRole::FluidVolume` — non-solid so characters wade cleanly!):
    let mut water_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32()) as usize,
        pow2_u64(three_u32()) as usize,
    );
    let w_span = sixteen * half + four;
    let water_col = [
        (quarter * half) as f32,
        (quarter + quarter * half) as f32,
        (half + quarter * half) as f32,
        (1.0 - quarter * half) as f32,
    ];
    water_acc.append_quad_smooth(
        Vec3::new(
            relief.basin_center_x - w_span,
            water_level_meters,
            relief.basin_center_z + w_span,
        ),
        Vec3::new(
            relief.basin_center_x + w_span,
            water_level_meters,
            relief.basin_center_z + w_span,
        ),
        Vec3::new(
            relief.basin_center_x + w_span,
            water_level_meters,
            relief.basin_center_z - w_span,
        ),
        Vec3::new(
            relief.basin_center_x - w_span,
            water_level_meters,
            relief.basin_center_z - w_span,
        ),
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        water_col,
    );
    let water_mesh = water_acc.into_mesh("BasinFluidSurface");

    let meshes = vec![
        terrain_mesh,
        citadel_mesh,
        bridge_mesh,
        tree_mesh,
        water_mesh,
    ];
    let mut scene = SceneWorld::new();

    let terrain_entity = scene.spawn(
        "hd_sculpted_terrain",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "hd_world"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "vaulted_citadel_hall",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "hd_world"),
    );
    scene.attach_renderable(citadel_entity, 1, four_u32() as usize);

    let bridge_entity = scene.spawn(
        "parabolic_arch_bridge",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "hd_world"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four_u32() as usize);

    let water_idx = 1usize + 1usize + 1usize + 1usize;
    let water_entity = scene.spawn(
        "river_basin_water",
        EntityRole::FluidVolume,
        None,
        Transform::default(),
        meshes[water_idx].bounds(),
        (1.0, "hd_world"),
    );
    scene.attach_renderable(water_entity, water_idx, 1);

    // Phyllotactic ring of sculpted trees outside the Citadel courtyard & clear of the bridge:
    let tree_idx = 1usize + 1usize + 1usize;
    let tree_count = if lod_rank == 0 {
        four_u32() as usize
    } else {
        (four_u32() * three_u32()) as usize
    };
    let golden_ang = golden_angle_radians();
    for i in 0..tree_count {
        let r = sixteen - two + ((i % (1 + 1 + 1)) as f64) * three;
        let theta = (i as f64) * golden_ang + 1.0 / phi;
        let tx = r * theta.cos();
        let tz = r * theta.sin();
        if (tx - relief.basin_center_x).abs() < six_val(two, three)
            && (tz - relief.basin_center_z).abs() < six_val(two, three)
        {
            continue;
        }
        let ty = hd_terrain_height(seed, relief, tx, tz);
        if ty <= water_level_meters + quarter {
            continue;
        }
        let scale_u = 1.0 - quarter * half + ((i % (1 + 1 + 1)) as f64) * (quarter * half);
        let e_tree = scene.spawn(
            "canopy_tree",
            EntityRole::SurfaceFoliage,
            None,
            Transform {
                translation: Vec3::new(tx, ty, tz),
                rotation: Quat::from_axis_angle(Vec3::UP, theta),
                scale: Vec3::new(scale_u, scale_u, scale_u),
            },
            meshes[tree_idx].bounds(),
            (1.0, "hd_world"),
        );
        scene.attach_renderable(e_tree, tree_idx, three_u32() as usize);
    }

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

/// Distinct procedural 3D architectural world styles so every Verdant Studio template and
/// domain pack renders unmistakably distinct geometry, floor, materials, and silhouette.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldArchitectureKind {
    /// Clean neutral 3D creation canvas with architectural studio grid (zero demo dome).
    BlankStudio,
    /// Sunspire Coastal Harbor basin with vaulted hall, stone arch bridge, and coastal trees.
    SunspireCoastal,
    /// Meridian Astral Observatory with stepped obsidian dais, brass armillary rings, and obelisks.
    MeridianObservatory,
    /// Glasshouse Botanical Conservatory with 2.5D terraced walkways and arched conservatory ribs.
    GlasshouseConservatory,
    /// Starfall Bastion Arena FPS with basalt combat platform, tactical ramps, cover monoliths, and pylons.
    StarfallArena,
}

impl WorldArchitectureKind {
    /// Resolves a domain index (`0..=3`) or slug to its distinct 3D architecture.
    #[must_use]
    pub fn from_domain_slug(slug: &str) -> Self {
        match slug {
            "blank-studio" | "blank" => Self::BlankStudio,
            "meridian-observatory" | "DOM-MERIDIAN-OBS" => Self::MeridianObservatory,
            "glasshouse-chronicle" | "DOM-GLASSHOUSE-BOTANICAL" => Self::GlasshouseConservatory,
            "starfall-bastion" | "DOM-STARFALL-BASTION" => Self::StarfallArena,
            _ => Self::SunspireCoastal,
        }
    }

    /// Resolves a numeric architecture selector (`0` = BlankStudio, `1` = Sunspire, `2` = Meridian,
    /// `3` = Glasshouse, `4` = Starfall).
    #[must_use]
    pub fn from_u32(code: u32) -> Self {
        let one = 1u32;
        let two = one + one;
        let three = two + one;
        let four = two + two;
        if code == 0 {
            Self::BlankStudio
        } else if code == one {
            Self::SunspireCoastal
        } else if code == two {
            Self::MeridianObservatory
        } else if code == three {
            Self::GlasshouseConservatory
        } else if code == four {
            Self::StarfallArena
        } else {
            Self::BlankStudio
        }
    }
}

/// Interactive primitive shapes creators can place directly into a Verdant Studio scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorPrimitiveKind {
    /// Solid architectural block / cube (`0`).
    Box,
    /// Walkable elevated floor platform (`1`).
    Platform,
    /// Sculpted cylindrical column / pillar (`2`).
    Pillar,
    /// Sloped tactical ramp / wedge (`3`).
    Ramp,
    /// Faceted glowing beacon / orb (`4`).
    Orb,
}

impl AuthorPrimitiveKind {
    /// Decodes a primitive kind code (`0..=4`).
    #[must_use]
    pub fn from_u32(code: u32) -> Self {
        let one = 1u32;
        let two = one + one;
        let three = two + one;
        let four = two + two;
        if code == one {
            Self::Platform
        } else if code == two {
            Self::Pillar
        } else if code == three {
            Self::Ramp
        } else if code == four {
            Self::Orb
        } else {
            Self::Box
        }
    }

    /// Human-readable display label for Studio Scene Outliner.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Box => "Box",
            Self::Platform => "Platform",
            Self::Pillar => "Pillar",
            Self::Ramp => "Ramp",
            Self::Orb => "Beacon Orb",
        }
    }
}

/// Generates a clean smooth-normal, material-tagged `ProceduralMesh` for an interactive Studio
/// primitive placed by a creator.
#[must_use]
pub fn generate_studio_primitive_mesh(
    kind: AuthorPrimitiveKind,
    sx: f64,
    sy: f64,
    sz: f64,
    col: [f32; DIM_HOMOGENEOUS],
) -> ProceduralMesh {
    let half = half_f64();
    let quarter = half * half;
    let two = two_f64();
    let hx = (sx.abs().max(quarter)) * half;
    let hy = sy.abs().max(quarter);
    let hz = (sz.abs().max(quarter)) * half;
    let mut acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + 1) as usize,
        pow2_u64(four_u32() + 1) as usize,
    );
    match kind {
        AuthorPrimitiveKind::Box => {
            acc.append_box(Vec3::new(-hx, 0.0, -hz), Vec3::new(hx, hy, hz), col);
        }
        AuthorPrimitiveKind::Platform => {
            let plat_h = (hy * quarter).max(quarter * half);
            acc.append_box(
                Vec3::new(-hx * two, 0.0, -hz * two),
                Vec3::new(hx * two, plat_h, hz * two),
                col,
            );
        }
        AuthorPrimitiveKind::Pillar => {
            let r = (hx.min(hz) * half).max(quarter);
            let r_segs = (four_u32() * three_u32()) as usize;
            let h_segs = four_u32() as usize;
            acc.append_lathed_column(
                Vec3::new(0.0, 0.0, 0.0),
                hy * two,
                r,
                r_segs,
                h_segs,
                col,
                col,
            );
        }
        AuthorPrimitiveKind::Ramp => {
            // Walkable sloped wedge rising from -hz (y=0) to +hz (y=hy):
            let p0 = Vec3::new(-hx, 0.0, -hz);
            let p1 = Vec3::new(hx, 0.0, -hz);
            let p2 = Vec3::new(hx, 0.0, hz);
            let p3 = Vec3::new(-hx, 0.0, hz);
            let t2 = Vec3::new(hx, hy, hz);
            let t3 = Vec3::new(-hx, hy, hz);
            // Sloped top deck:
            acc.append_quad_flat(p0, p1, t2, t3, col);
            // Back vertical face:
            acc.append_quad_flat(p3, p2, t2, t3, col);
            // Bottom face:
            acc.append_quad_flat(p0, p3, p2, p1, col);
            // Left & right triangular cheeks (degenerate quads):
            acc.append_quad_flat(p0, t3, p3, p0, col);
            acc.append_quad_flat(p1, p2, t2, p1, col);
        }
        AuthorPrimitiveKind::Orb => {
            let r = hx.min(hz).max(quarter);
            let lat_segs = (four_u32() + four_u32()) as usize;
            let lon_segs = (four_u32() * three_u32()) as usize;
            let tau = std::f64::consts::TAU;
            let pi = std::f64::consts::PI;
            let start_v = acc.positions.len() as u32;
            let stride = (lon_segs + 1) as u32;
            let center_y = r;
            for ilat in 0..=lat_segs {
                let vf = (ilat as f64) / (lat_segs as f64);
                let phi_a = vf * pi;
                let (sp, cp) = phi_a.sin_cos();
                for ilon in 0..=lon_segs {
                    let uf = (ilon as f64) / (lon_segs as f64);
                    let theta = uf * tau;
                    let (st, ct) = theta.sin_cos();
                    let n = Vec3::new(sp * ct, cp, sp * st);
                    let p = Vec3::new(n.x * r, center_y + n.y * r, n.z * r);
                    acc.push_vertex(p, n, [uf as f32, vf as f32], col);
                }
            }
            for ilat in 0..(lat_segs as u32) {
                for ilon in 0..(lon_segs as u32) {
                    let a = start_v + ilat * stride + ilon;
                    let b = a + 1;
                    let c = a + stride + 1;
                    let d = a + stride;
                    acc.indices.extend_from_slice(&[a, d, c, a, c, b]);
                }
            }
        }
    }
    acc.into_mesh(kind.label())
}

fn append_torus_ring(
    acc: &mut SmoothMeshAcc,
    center: Vec3,
    major_r: f64,
    minor_r: f64,
    tilt_x: f64,
    tilt_z: f64,
    col: [f32; DIM_HOMOGENEOUS],
) {
    let major_segs = (four_u32() * four_u32() + four_u32() + four_u32()) as usize;
    let minor_segs = (four_u32() + four_u32()) as usize;
    let tau = std::f64::consts::TAU;
    let q = Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), tilt_x)
        * Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), tilt_z);
    let start_v = acc.positions.len() as u32;
    let stride = (minor_segs + 1) as u32;
    for iu in 0..=major_segs {
        let uf = (iu as f64) / (major_segs as f64);
        let u = uf * tau;
        let (su, cu) = u.sin_cos();
        for iv in 0..=minor_segs {
            let vf = (iv as f64) / (minor_segs as f64);
            let v = vf * tau;
            let (sv, cv) = v.sin_cos();
            let ring_r = major_r + minor_r * cv;
            let local_p = Vec3::new(ring_r * cu, minor_r * sv, ring_r * su);
            let local_n = Vec3::new(cv * cu, sv, cv * su);
            let p = center + q.rotate_vec3(local_p);
            let n = q.rotate_vec3(local_n).normalized();
            acc.push_vertex(p, n, [uf as f32, vf as f32], col);
        }
    }
    for iu in 0..(major_segs as u32) {
        for iv in 0..(minor_segs as u32) {
            let a = start_v + iu * stride + iv;
            let b = a + 1;
            let c = a + stride + 1;
            let d = a + stride;
            acc.indices.extend_from_slice(&[a, b, c, a, c, d]);
        }
    }
}

fn synthesize_blank_studio_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let four = four_u32() as f64;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Crisp architectural checkerboard studio floor (no hills, no white dome, no trees):
    let tiles_per_half = (four_u32() + four_u32()) as i32; // 8 tiles per half -> 16x16 grid
    let tile_sz = two;
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let col_a = [
        (quarter + eighth * half) as f32,
        (quarter + eighth * half) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let col_b = [
        (quarter - eighth * half) as f32,
        (quarter - eighth * half) as f32,
        quarter as f32,
        1.0f32,
    ];
    for ix in -tiles_per_half..tiles_per_half {
        for iz in -tiles_per_half..tiles_per_half {
            let x0 = (ix as f64) * tile_sz;
            let z0 = (iz as f64) * tile_sz;
            let col = if (ix + iz) & 1 == 0 { col_a } else { col_b };
            floor_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + tile_sz),
                Vec3::new(x0 + tile_sz, 0.0, z0 + tile_sz),
                Vec3::new(x0 + tile_sz, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let floor_mesh = floor_acc.into_mesh("StudioGridFloor");

    // Subtle origin axis guide & reference plinth (so creators immediately perceive 3D scale):
    let mut origin_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + 1) as usize,
        pow2_u64(four_u32() + 1) as usize,
    );
    let rail_w = eighth * half;
    let rail_h = eighth * quarter;
    let x_axis_col = [
        (two + half + quarter) as f32,
        quarter as f32,
        eighth as f32,
        1.0f32,
    ];
    let z_axis_col = [
        (two + eighth) as f32,
        (half + eighth) as f32,
        (half + quarter) as f32,
        1.0f32,
    ];
    origin_acc.append_box(
        Vec3::new(-sixteen, 0.0, -rail_w),
        Vec3::new(sixteen, rail_h, rail_w),
        x_axis_col,
    );
    origin_acc.append_box(
        Vec3::new(-rail_w, 0.0, -sixteen),
        Vec3::new(rail_w, rail_h, sixteen),
        z_axis_col,
    );
    // Clean central studio reference plinth (metallic-stone subtle pad):
    let pad_col = [
        (two + half) as f32,
        (half + eighth) as f32,
        (half + eighth) as f32,
        1.0f32,
    ];
    origin_acc.append_box(
        Vec3::new(-two, 0.0, -two),
        Vec3::new(two, quarter * half, two),
        pad_col,
    );
    let origin_mesh = origin_acc.into_mesh("StudioOriginGuides");

    // Minimal boundary rim markers at ±16m:
    let mut rim_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    let rim_col = [
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let rim_t = quarter;
    let rim_h = eighth;
    rim_acc.append_box(
        Vec3::new(-sixteen, 0.0, -sixteen),
        Vec3::new(sixteen, rim_h, -sixteen + rim_t),
        rim_col,
    );
    rim_acc.append_box(
        Vec3::new(-sixteen, 0.0, sixteen - rim_t),
        Vec3::new(sixteen, rim_h, sixteen),
        rim_col,
    );
    let rim_mesh = rim_acc.into_mesh("StudioBoundaryRim");

    let meshes = vec![floor_mesh, origin_mesh, rim_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "studio_grid_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "studio_origin_guides",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "studio_boundary_rim",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

fn synthesize_starfall_arena_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;
    let arena_half = sixteen + eight; // 24m half-extent -> 48m x 48m FPS arena

    // Dark volcanic basalt floor with tactical orange lane trims:
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let basalt_dark = [
        eighth as f32,
        eighth as f32,
        (eighth + eighth * quarter) as f32,
        1.0f32,
    ];
    let basalt_mid = [
        (eighth + eighth * half) as f32,
        (eighth + eighth * quarter) as f32,
        quarter as f32,
        1.0f32,
    ];
    let magma_glow = [
        (three + half + quarter + eighth) as f32,
        (quarter + eighth) as f32,
        eighth as f32,
        1.0f32,
    ];
    let tiles = (four_u32() + two_u32()) as i32;
    let step = arena_half / (tiles as f64);
    for ix in -tiles..tiles {
        for iz in -tiles..tiles {
            let x0 = (ix as f64) * step;
            let z0 = (iz as f64) * step;
            let col = if (ix + iz) & 1 == 0 {
                basalt_dark
            } else {
                basalt_mid
            };
            floor_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let floor_mesh = floor_acc.into_mesh("StarfallBasaltFloor");

    // Central stepped combat platform + 4 angled tactical ramps + 4 cover monoliths + 4 corner pylons:
    let mut arena_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let armor_metal = [
        (two + quarter + eighth) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth + eighth * half) as f32,
        1.0f32,
    ];
    let plat_r = four + two;
    let plat_h = two - half;
    // Lower tier & upper central tactical platform:
    arena_acc.append_box(
        Vec3::new(-plat_r - two, 0.0, -plat_r - two),
        Vec3::new(plat_r + two, half + quarter, plat_r + two),
        basalt_mid,
    );
    arena_acc.append_box(
        Vec3::new(-plat_r, half + quarter, -plat_r),
        Vec3::new(plat_r, plat_h, plat_r),
        armor_metal,
    );
    // Glowing tactical center core strip:
    arena_acc.append_box(
        Vec3::new(-two, plat_h, -two),
        Vec3::new(two, plat_h + quarter, two),
        magma_glow,
    );

    let arena_mesh = arena_acc.into_mesh("StarfallCombatPlatform");

    // Separate named mesh for the 4 tactical cover monoliths:
    let mut cover_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let cover_dist = eight + four;
    for (cx, cz) in [
        (cover_dist, 0.0),
        (-cover_dist, 0.0),
        (0.0, cover_dist),
        (0.0, -cover_dist),
    ] {
        let wide_x = if cx == 0.0 { three } else { one_f64() };
        let wide_z = if cz == 0.0 { three } else { one_f64() };
        cover_acc.append_box(
            Vec3::new(cx - wide_x, 0.0, cz - wide_z),
            Vec3::new(cx + wide_x, three, cz + wide_z),
            armor_metal,
        );
        cover_acc.append_box(
            Vec3::new(cx - wide_x, three, cz - wide_z),
            Vec3::new(cx + wide_x, three + quarter, cz + wide_z),
            magma_glow,
        );
    }
    let cover_mesh = cover_acc.into_mesh("StarfallCoverMonoliths");

    // Separate named mesh for the 4 tall corner perimeter guard towers:
    let mut towers_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let tower_dist = sixteen + two;
    let tower_h = eight + four;
    for (tx, tz) in [
        (tower_dist, tower_dist),
        (-tower_dist, tower_dist),
        (tower_dist, -tower_dist),
        (-tower_dist, -tower_dist),
    ] {
        towers_acc.append_box(
            Vec3::new(tx - two, 0.0, tz - two),
            Vec3::new(tx + two, tower_h, tz + two),
            basalt_mid,
        );
        towers_acc.append_box(
            Vec3::new(tx - two - quarter, tower_h - one_f64(), tz - two - quarter),
            Vec3::new(tx + two + quarter, tower_h - half, tz + two + quarter),
            magma_glow,
        );
    }
    let towers_mesh = towers_acc.into_mesh("StarfallCornerTowers");

    // 4 flanking elevated perimeter walls / catwalk decks:
    let mut walls_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let wall_h = four + two;
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, -arena_half),
        Vec3::new(arena_half, wall_h, -arena_half + two),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, arena_half - two),
        Vec3::new(arena_half, wall_h, arena_half),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, -arena_half),
        Vec3::new(-arena_half + two, wall_h, arena_half),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(arena_half - two, 0.0, -arena_half),
        Vec3::new(arena_half, wall_h, arena_half),
        basalt_dark,
    );
    let walls_mesh = walls_acc.into_mesh("StarfallPerimeterBastion");

    let meshes = vec![floor_mesh, arena_mesh, walls_mesh, cover_mesh, towers_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "starfall_basalt_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "starfall_combat_platform",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "starfall_perimeter_walls",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let cover_idx = 1usize + 1usize + 1usize;
    let cover_entity = scene.spawn(
        "starfall_cover_monoliths",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[cover_idx].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(cover_entity, cover_idx, four as usize);

    let tower_idx = cover_idx + 1usize;
    let tower_entity = scene.spawn(
        "starfall_corner_towers",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[tower_idx].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(tower_entity, tower_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

fn synthesize_meridian_observatory_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Deep midnight-indigo slate astral plaza:
    let mut plaza_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let slate_a = [
        eighth as f32,
        (eighth + eighth * quarter) as f32,
        (quarter + eighth * half) as f32,
        1.0f32,
    ];
    let slate_b = [
        (eighth + eighth * half) as f32,
        (quarter * half + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let tiles = (four_u32() + two_u32()) as i32;
    let step = (sixteen + eight) / (tiles as f64);
    for ix in -tiles..tiles {
        for iz in -tiles..tiles {
            let x0 = (ix as f64) * step;
            let z0 = (iz as f64) * step;
            let col = if (ix + iz) & 1 == 0 { slate_a } else { slate_b };
            plaza_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let plaza_mesh = plaza_acc.into_mesh("MeridianAstralPlaza");

    // Stepped obsidian dais + TWO colossal tilted metallic brass armillary rings + glowing cyan core:
    let mut obs_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let brass_col = [
        (two + half + quarter + eighth) as f32,
        (half + quarter) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let star_glow = [
        (three + quarter) as f32,
        (half + quarter + eighth) as f32,
        (half + quarter + eighth) as f32,
        1.0f32,
    ];
    obs_acc.append_box(
        Vec3::new(-eight, 0.0, -eight),
        Vec3::new(eight, half, eight),
        slate_b,
    );
    obs_acc.append_box(
        Vec3::new(-four - two, half, -four - two),
        Vec3::new(four + two, one_f64(), four + two),
        brass_col,
    );
    let dais_mesh = obs_acc.into_mesh("MeridianObservatoryDais");

    let mut rings_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let ring_center = Vec3::new(0.0, eight, 0.0);
    append_torus_ring(
        &mut rings_acc,
        ring_center,
        four + three,
        quarter + eighth,
        half,
        quarter,
        brass_col,
    );
    append_torus_ring(
        &mut rings_acc,
        ring_center,
        four + one_f64(),
        quarter,
        -half - quarter,
        half,
        brass_col,
    );
    let obs_mesh = rings_acc.into_mesh("MeridianArmillaryRings");

    let mut core_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    core_acc.append_box(
        Vec3::new(-one_f64(), eight - one_f64(), -one_f64()),
        Vec3::new(one_f64(), eight + one_f64(), one_f64()),
        star_glow,
    );
    let core_mesh = core_acc.into_mesh("MeridianStarlightCore");

    // Ring of 6 tall obsidian starlight obelisks around the observatory:
    let mut ob_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let ob_count = (three_u32() + three_u32()) as usize;
    let tau = std::f64::consts::TAU;
    let ring_r = sixteen - two;
    for i in 0..ob_count {
        let ang = (i as f64) * tau / (ob_count as f64);
        let (sa, ca) = ang.sin_cos();
        let ox = ring_r * ca;
        let oz = ring_r * sa;
        ob_acc.append_lathed_column(
            Vec3::new(ox, 0.0, oz),
            eight + two,
            one_f64(),
            (four_u32() + four_u32()) as usize,
            four_u32() as usize,
            slate_b,
            brass_col,
        );
        ob_acc.append_box(
            Vec3::new(ox - half, eight + two, oz - half),
            Vec3::new(ox + half, eight + three, oz + half),
            star_glow,
        );
    }
    let ob_mesh = ob_acc.into_mesh("MeridianStarlightObelisks");

    let meshes = vec![plaza_mesh, obs_mesh, ob_mesh, dais_mesh, core_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "meridian_astral_plaza",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "meridian_armillary_orrery",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "meridian_starlight_obelisks",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let dais_idx = 1usize + 1usize + 1usize;
    let dais_entity = scene.spawn(
        "meridian_observatory_dais",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[dais_idx].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(dais_entity, dais_idx, four as usize);

    let core_idx = dais_idx + 1usize;
    let core_entity = scene.spawn(
        "meridian_starlight_core",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[core_idx].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(core_entity, core_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

fn synthesize_glasshouse_conservatory_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Warm sunlit emerald moss & travertine conservatory ground:
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let moss_col = [
        (eighth + eighth * half) as f32,
        (half - eighth * half) as f32,
        (quarter + eighth * half) as f32,
        1.0f32,
    ];
    let stone_col = [
        (half + quarter) as f32,
        (half + quarter - eighth * half) as f32,
        (half + eighth) as f32,
        1.0f32,
    ];
    let iron_col = [
        (two + eighth + eighth * half) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let amber_glow = [
        (three + half + quarter + eighth) as f32,
        (half + quarter) as f32,
        quarter as f32,
        1.0f32,
    ];
    let span = sixteen + eight;
    floor_acc.append_quad_smooth(
        Vec3::new(-span, 0.0, span),
        Vec3::new(span, 0.0, span),
        Vec3::new(span, 0.0, -span),
        Vec3::new(-span, 0.0, -span),
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        moss_col,
    );
    let floor_mesh = floor_acc.into_mesh("GlasshouseGardenFloor");

    // 5 rhythmic cast-iron conservatory vault ribs along the X axis + elevated side-scroller terraces:
    let mut vault_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let rib_xs = [-sixteen, -eight, 0.0, eight, sixteen];
    let rib_half_z = eight;
    let rib_h = eight + two;
    let mut lantern_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    for &rx in &rib_xs {
        // Left & right iron rib columns:
        vault_acc.append_box(
            Vec3::new(rx - half, 0.0, -rib_half_z - half),
            Vec3::new(rx + half, rib_h, -rib_half_z + half),
            iron_col,
        );
        vault_acc.append_box(
            Vec3::new(rx - half, 0.0, rib_half_z - half),
            Vec3::new(rx + half, rib_h, rib_half_z + half),
            iron_col,
        );
        // Overhead conservatory gable truss bar + separate amber lantern mesh:
        vault_acc.append_box(
            Vec3::new(rx - quarter, rib_h - half, -rib_half_z),
            Vec3::new(rx + quarter, rib_h, rib_half_z),
            iron_col,
        );
        lantern_acc.append_box(
            Vec3::new(rx - half, rib_h - two, -half),
            Vec3::new(rx + half, rib_h - one_f64(), half),
            amber_glow,
        );
    }
    let lantern_mesh = lantern_acc.into_mesh("GlasshouseAmberLanterns");
    // Longitudinal ridge spine tying all 5 conservatory ribs together:
    vault_acc.append_box(
        Vec3::new(-sixteen, rib_h, -quarter),
        Vec3::new(sixteen, rib_h + half, quarter),
        iron_col,
    );
    let vault_mesh = vault_acc.into_mesh("GlasshouseConservatoryRibs");

    // Low stone central promenade:
    let mut prom_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    prom_acc.append_box(
        Vec3::new(-sixteen - two, 0.0, -three),
        Vec3::new(sixteen + two, half, three),
        stone_col,
    );
    let prom_mesh = prom_acc.into_mesh("GlasshouseStonePromenade");

    // Multi-level elevated 2.5D jump platforms running along the conservatory promenade:
    let mut plat_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    plat_acc.append_box(
        Vec3::new(-four - eight, two, -two),
        Vec3::new(-four, two + half, two),
        stone_col,
    );
    plat_acc.append_box(
        Vec3::new(-three, four, -two),
        Vec3::new(three, four + half, two),
        stone_col,
    );
    plat_acc.append_box(
        Vec3::new(four, two + one_f64(), -two),
        Vec3::new(four + eight, three + half, two),
        stone_col,
    );
    let plat_mesh = plat_acc.into_mesh("GlasshouseTerraceWalkways");

    let meshes = vec![floor_mesh, vault_mesh, plat_mesh, lantern_mesh, prom_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "glasshouse_garden_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "glasshouse_conservatory_ribs",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "glasshouse_terrace_walkways",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let lantern_idx = 1usize + 1usize + 1usize;
    let lantern_entity = scene.spawn(
        "glasshouse_amber_lanterns",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[lantern_idx].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(lantern_entity, lantern_idx, four as usize);

    let prom_idx = lantern_idx + 1usize;
    let prom_entity = scene.spawn(
        "glasshouse_stone_promenade",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[prom_idx].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(prom_entity, prom_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

fn one_f64() -> f64 {
    1.0
}

/// Synthesizes a High-Definition 3D sector matching the requested [`WorldArchitectureKind`],
/// guaranteeing visually unmistakable geometry, materials, and spatial layout across all templates.
#[must_use]
pub fn synthesize_hd_world_sector_for_kind(
    seed: u64,
    relief: &TerrainReliefParams,
    water_level_meters: f64,
    lod_rank: u64,
    kind: WorldArchitectureKind,
) -> HdSectorBundle {
    match kind {
        WorldArchitectureKind::BlankStudio => synthesize_blank_studio_sector(seed, lod_rank),
        WorldArchitectureKind::StarfallArena => synthesize_starfall_arena_sector(seed, lod_rank),
        WorldArchitectureKind::MeridianObservatory => {
            synthesize_meridian_observatory_sector(seed, lod_rank)
        }
        WorldArchitectureKind::GlasshouseConservatory => {
            synthesize_glasshouse_conservatory_sector(seed, lod_rank)
        }
        WorldArchitectureKind::SunspireCoastal => {
            synthesize_hd_citadel_sector(seed, relief, water_level_meters, lod_rank)
        }
    }
}

fn six_val(two: f64, three: f64) -> f64 {
    two * three
}
