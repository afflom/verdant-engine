//! UHD (4K `3840x2160`, 5K `5120x2880`, 8K `7680x4320` & beyond) graphics scaling engine
//! for arbitrary models inside Verdant Engine.
//!
//! Provides:
//! - Deterministic 64-triangle GPU **Meshlet Clustering** (`MeshletCluster`, `MeshletGraph`) with
//!   tight bounding spheres and outward normal cones for O(1) cluster backface culling,
//! - Real-time **Frustum + Backface Normal-Cone + Screen-Space Error (SSE)** meshlet culling,
//! - Host-bounded **Multi-Material PBR Texture Atlas Packing** (`1x1` up to `4x4` tiles) so
//!   arbitrary multi-texture `.glb` models render pixel-accurately in a single draw call,
//! - Formal **UHD 4K / 8K Display & Fillrate Law** (`UhdDisplayProfile`), and
//! - `naga`-verified W3C WGSL shaders for GPU compute meshlet culling (`@compute @workgroup_size(64)`)
//!   and UHD 4-channel PBR + Equirectangular HDRI IBL + Contrast-Adaptive Sharpening (`CAS`).

use serde::{Deserialize, Serialize};
use verdant_tensor::scalars::four_u32;
pub use verdant_tensor::scalars::{
    uhd_budget_half_ok, uhd_cas_sharpness_permille, uhd_classify_display_tier, uhd_drs_step_down,
    uhd_drs_step_up, uhd_frame_is_stable, uhd_jitter_us, uhd_meshlet_sse_tenths_px,
    uhd_paged_arena_needs_eviction, uhd_paged_arena_within_budget, uhd_recommended_drs_permille,
    uhd_tier_supports_webgpu_cas,
};

#[cfg(feature = "wgsl-oracle")]
use crate::error::ShaderValidationError;
use crate::geometry::ProceduralMesh;
use crate::math::{half_f64, pow10_u64, two_u32, Frustum, Vec3};
#[cfg(feature = "wgsl-oracle")]
use crate::render::{validate_wgsl_shader, VerifiedShaderModule};

/// Exact triangle capacity per GPU meshlet (`64` triangles, matching `workgroup_size(64)`).
pub const MAX_MESHLET_TRIANGLES: usize = 1 << (1 + 1 + 1 + 1 + 1 + 1);

/// Maximum distinct material textures packed into a single host-bounded `4x4` PBR atlas (`16`).
pub const MAX_ATLAS_TILES: usize = 1 << (1 + 1 + 1 + 1);

/// Fixed ring-buffer window length (`64` frames) for the zero-allocation [`FrameBudgetOracle`].
pub const FRAME_ORACLE_WINDOW: usize = 1 << (1 + 1 + 1 + 1 + 1 + 1);

/// Quantized Dynamic Resolution Scaling step count (`5` lawful levels: `65%`, `75%`, `85%`, `92.5%`, `100%`).
const DRS_LADDER_LEN: usize = 1 + 1 + 1 + 1 + 1;

#[path = "uhd_shaders.rs"]
mod uhd_shaders;
pub use uhd_shaders::*;

/// A 64-triangle GPU meshlet cluster with tight world bounding sphere and outward normal cone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshletCluster {
    /// Zero-based meshlet identifier.
    pub meshlet_id: usize,
    /// Bounding sphere center in world meters.
    pub center: Vec3,
    /// Bounding sphere radius in meters.
    pub radius: f64,
    /// Average outward unit normal of all triangles in this meshlet.
    pub cone_axis: Vec3,
    /// Backface normal-cone cosine cutoff (`>= 1.0` disables backface rejection for wrap-around clusters).
    pub cone_cutoff: f64,
    /// Start index offset in the packed index buffer (`3 * first_tri`).
    pub first_index: usize,
    /// Number of vertex indices in this meshlet (`3 * tri_count`).
    pub index_count: usize,
}

/// Complete spatially-clustered meshlet partition over an arbitrary packed world scene.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshletGraph {
    /// All 64-triangle spatially-sorted meshlet clusters covering the packed scene.
    pub clusters: Vec<MeshletCluster>,
    /// Spatially Morton-ordered triangle indices (`3 * total_triangles`) aligned with `clusters`.
    #[serde(default, skip_serializing)]
    pub reordered_indices: Vec<u32>,
    /// Total triangles indexed by this graph.
    pub total_triangles: usize,
}

/// Result of per-frame UHD Frustum + Backface Normal-Cone + Screen-Space Error meshlet culling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UhdVisibilityTelemetry {
    /// Total meshlets partitioned across the active scene.
    pub total_meshlets: usize,
    /// Meshlets surviving frustum, backface normal-cone, and SSE tests.
    pub visible_meshlets: usize,
    /// Meshlets rejected outside the 6 camera frustum planes.
    pub culled_frustum: usize,
    /// Meshlets rejected via O(1) outward normal-cone backface test.
    pub culled_backface_cone: usize,
    /// Meshlets rejected as sub-pixel at the current UHD viewport height & FOV.
    pub culled_subpixel: usize,
    /// Total scene triangles before culling.
    pub total_triangles: usize,
    /// Triangles submitted to the GPU after meshlet culling.
    pub visible_triangles: usize,
    /// Fraction of scene triangles eliminated prior to rasterization (`0..=1000` permille).
    pub cull_ratio_permille: u32,
}

/// Partitions packed world-space vertex positions (`3 floats/vert`) and triangle indices (`3 u32/tri`)
/// into spatially Morton-sorted 64-triangle GPU [`MeshletCluster`]s with tight bounding spheres and
/// outward normal cones (zero per-cluster heap allocations).
#[must_use]
pub fn build_meshlet_graph(positions_xyz: &[f32], indices: &[u32]) -> MeshletGraph {
    let three = 1usize + 1usize + 1usize;
    let two = 1usize + 1usize;
    let total_triangles = indices.len() / three;
    if total_triangles == 0 || positions_xyz.len() < three {
        return MeshletGraph::default();
    }

    let half = half_f64();
    let third = 1.0_f64 / (three as f64);

    // Pass 1: Compute scene AABB of triangle centroids for 30-bit Z-order Morton spatial clustering:
    let mut g_min_x = f64::INFINITY;
    let mut g_min_y = f64::INFINITY;
    let mut g_min_z = f64::INFINITY;
    let mut g_max_x = f64::NEG_INFINITY;
    let mut g_max_y = f64::NEG_INFINITY;
    let mut g_max_z = f64::NEG_INFINITY;
    let mut tri_keys: Vec<(u32, usize)> = Vec::with_capacity(total_triangles);

    for t in 0..total_triangles {
        let base = t * three;
        let i0 = (indices[base] as usize) * three;
        let i1 = (indices[base + 1] as usize) * three;
        let i2 = (indices[base + two] as usize) * three;
        if i0 + two >= positions_xyz.len()
            || i1 + two >= positions_xyz.len()
            || i2 + two >= positions_xyz.len()
        {
            continue;
        }
        let cx = (f64::from(positions_xyz[i0])
            + f64::from(positions_xyz[i1])
            + f64::from(positions_xyz[i2]))
            * third;
        let cy = (f64::from(positions_xyz[i0 + 1])
            + f64::from(positions_xyz[i1 + 1])
            + f64::from(positions_xyz[i2 + 1]))
            * third;
        let cz = (f64::from(positions_xyz[i0 + two])
            + f64::from(positions_xyz[i1 + two])
            + f64::from(positions_xyz[i2 + two]))
            * third;
        if cx.is_finite() && cy.is_finite() && cz.is_finite() {
            g_min_x = g_min_x.min(cx);
            g_min_y = g_min_y.min(cy);
            g_min_z = g_min_z.min(cz);
            g_max_x = g_max_x.max(cx);
            g_max_y = g_max_y.max(cy);
            g_max_z = g_max_z.max(cz);
        }
    }

    let eps = half * half * half * half * half * half * half * half;
    let inv_ex = if g_min_x.is_finite() && g_max_x.is_finite() {
        1.0 / (g_max_x - g_min_x).max(eps)
    } else {
        0.0
    };
    let inv_ey = if g_min_y.is_finite() && g_max_y.is_finite() {
        1.0 / (g_max_y - g_min_y).max(eps)
    } else {
        0.0
    };
    let inv_ez = if g_min_z.is_finite() && g_max_z.is_finite() {
        1.0 / (g_max_z - g_min_z).max(eps)
    } else {
        0.0
    };

    for t in 0..total_triangles {
        let base = t * three;
        let i0 = (indices[base] as usize) * three;
        let i1 = (indices[base + 1] as usize) * three;
        let i2 = (indices[base + two] as usize) * three;
        let code = if i0 + two < positions_xyz.len()
            && i1 + two < positions_xyz.len()
            && i2 + two < positions_xyz.len()
        {
            let cx = (f64::from(positions_xyz[i0])
                + f64::from(positions_xyz[i1])
                + f64::from(positions_xyz[i2]))
                * third;
            let cy = (f64::from(positions_xyz[i0 + 1])
                + f64::from(positions_xyz[i1 + 1])
                + f64::from(positions_xyz[i2 + 1]))
                * third;
            let cz = (f64::from(positions_xyz[i0 + two])
                + f64::from(positions_xyz[i1 + two])
                + f64::from(positions_xyz[i2 + two]))
                * third;
            let ten_u = 1u32 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1;
            let three_u = 1u32 + 1 + 1;
            let two_u = 1u32 + 1;
            let mask10 = (1u32 << ten_u) - 1;
            let max_coord = f64::from(mask10);
            let spread10 = |v: u32| -> u32 {
                let clamped = v & mask10;
                let mut out = 0u32;
                let mut bit = 0u32;
                while bit < ten_u {
                    out |= ((clamped >> bit) & 1) << (bit * three_u);
                    bit += 1;
                }
                out
            };
            let qx = (((cx - g_min_x) * inv_ex).clamp(0.0, 1.0) * max_coord) as u32;
            let qy = (((cy - g_min_y) * inv_ey).clamp(0.0, 1.0) * max_coord) as u32;
            let qz = (((cz - g_min_z) * inv_ez).clamp(0.0, 1.0) * max_coord) as u32;
            spread10(qx) | (spread10(qy) << 1) | (spread10(qz) << two_u)
        } else {
            u32::MAX
        };
        tri_keys.push((code, t));
    }
    tri_keys.sort_unstable_by_key(|&(code, _)| code);

    let mut reordered_indices = Vec::with_capacity(total_triangles * three);
    for &(_, t) in &tri_keys {
        let base = t * three;
        reordered_indices.push(indices[base]);
        reordered_indices.push(indices[base + 1]);
        reordered_indices.push(indices[base + two]);
    }

    let cluster_count = total_triangles.div_ceil(MAX_MESHLET_TRIANGLES);
    let mut clusters = Vec::with_capacity(cluster_count);
    // Stack-allocated per-cluster normal scratch buffer (ZERO heap allocations inside cluster loop):
    let mut tri_normals_buf = [Vec3::ZERO; MAX_MESHLET_TRIANGLES];

    for m_idx in 0..cluster_count {
        let first_tri = m_idx * MAX_MESHLET_TRIANGLES;
        let tri_end = (first_tri + MAX_MESHLET_TRIANGLES).min(total_triangles);
        let first_index = first_tri * three;
        let index_count = (tri_end - first_tri) * three;

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;
        let mut tri_normal_count = 0usize;
        let mut avg_n = Vec3::ZERO;

        for t in first_tri..tri_end {
            let base = t * three;
            let i0 = (reordered_indices[base] as usize) * three;
            let i1 = (reordered_indices[base + 1] as usize) * three;
            let i2 = (reordered_indices[base + two] as usize) * three;
            if i0 + two >= positions_xyz.len()
                || i1 + two >= positions_xyz.len()
                || i2 + two >= positions_xyz.len()
            {
                continue;
            }
            let p0 = Vec3::new(
                f64::from(positions_xyz[i0]),
                f64::from(positions_xyz[i0 + 1]),
                f64::from(positions_xyz[i0 + two]),
            );
            let p1 = Vec3::new(
                f64::from(positions_xyz[i1]),
                f64::from(positions_xyz[i1 + 1]),
                f64::from(positions_xyz[i1 + two]),
            );
            let p2 = Vec3::new(
                f64::from(positions_xyz[i2]),
                f64::from(positions_xyz[i2 + 1]),
                f64::from(positions_xyz[i2 + two]),
            );
            for p in [p0, p1, p2] {
                if p.x.is_finite() && p.y.is_finite() && p.z.is_finite() {
                    min_x = min_x.min(p.x);
                    min_y = min_y.min(p.y);
                    min_z = min_z.min(p.z);
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                    max_z = max_z.max(p.z);
                }
            }
            let n = (p1 - p0).cross(p2 - p0).normalized();
            if n.length_squared() > 0.0 && tri_normal_count < MAX_MESHLET_TRIANGLES {
                avg_n = avg_n + n;
                tri_normals_buf[tri_normal_count] = n;
                tri_normal_count += 1;
            }
        }

        let center = if min_x.is_finite() && max_x.is_finite() {
            Vec3::new(
                (min_x + max_x) * half,
                (min_y + max_y) * half,
                (min_z + max_z) * half,
            )
        } else {
            Vec3::ZERO
        };
        let mut max_r2 = 0.0_f64;
        for &raw_vi in &reordered_indices[first_index..first_index + index_count] {
            let vi = (raw_vi as usize) * three;
            if vi + two < positions_xyz.len() {
                let p = Vec3::new(
                    f64::from(positions_xyz[vi]),
                    f64::from(positions_xyz[vi + 1]),
                    f64::from(positions_xyz[vi + two]),
                );
                let d2 = (p - center).length_squared();
                if d2.is_finite() && d2 > max_r2 {
                    max_r2 = d2;
                }
            }
        }
        let radius = max_r2.sqrt().max(half * half * half * half);

        let cone_axis = avg_n.normalized();
        let cone_cutoff = if cone_axis.length_squared() > half && tri_normal_count > 0 {
            let mut min_dot = 1.0_f64;
            for n in &tri_normals_buf[..tri_normal_count] {
                min_dot = min_dot.min(n.dot(cone_axis));
            }
            if min_dot <= 0.0 {
                1.0
            } else {
                // Conservative backface cone threshold: `sin(half_angle)` = `sqrt(1 - min_dot^2)`
                (1.0 - min_dot * min_dot).max(0.0).sqrt()
            }
        } else {
            1.0
        };

        clusters.push(MeshletCluster {
            meshlet_id: m_idx,
            center,
            radius,
            cone_axis,
            cone_cutoff,
            first_index,
            index_count,
        });
    }

    MeshletGraph {
        clusters,
        reordered_indices,
        total_triangles,
    }
}

/// Evaluates Frustum + Backface Normal-Cone + UHD Screen-Space Error culling over `graph`,
/// writing surviving triangle indices into `out_visible_indices` with zero steady-state
/// heap reallocations and returning exact telemetry.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_meshlet_visibility(
    graph: &MeshletGraph,
    full_indices: &[u32],
    frustum: &Frustum,
    eye: Vec3,
    viewport_height_px: u32,
    tan_half_fov: f64,
    min_pixel_error: f64,
    out_visible_indices: &mut Vec<u32>,
) -> UhdVisibilityTelemetry {
    let source_indices = if !graph.reordered_indices.is_empty() {
        graph.reordered_indices.as_slice()
    } else {
        full_indices
    };
    out_visible_indices.clear();
    if out_visible_indices.capacity() < source_indices.len() {
        out_visible_indices.reserve(source_indices.len() - out_visible_indices.capacity());
    }
    let three = 1usize + 1usize + 1usize;
    let vh = f64::from(viewport_height_px.max(1));
    let safe_tan = tan_half_fov.max(half_f64() * half_f64() * half_f64());

    let mut visible_meshlets = 0usize;
    let mut culled_frustum = 0usize;
    let mut culled_backface_cone = 0usize;
    let mut culled_subpixel = 0usize;

    for m in &graph.clusters {
        // 1. Six-plane frustum sphere test:
        let mut inside = true;
        for plane in &frustum.planes {
            if plane.normal.dot(m.center) + plane.distance < -m.radius {
                inside = false;
                break;
            }
        }
        if !inside {
            culled_frustum += 1;
            continue;
        }

        // 2. O(1) outward normal-cone backface test (conservative sphere-cone bound):
        let to_center = m.center - eye;
        let dist = to_center.length();
        if m.cone_cutoff < 1.0 && dist > m.radius * (1.0 + 1.0) {
            let inv_d = 1.0 / dist;
            let view_dir = to_center.scale(inv_d);
            // Only cull when every point on the cluster sphere faces strictly away from `eye`:
            let angular_slack = (m.radius * inv_d).min(1.0);
            if view_dir.dot(m.cone_axis) >= m.cone_cutoff + angular_slack {
                culled_backface_cone += 1;
                continue;
            }
        }

        // 3. UHD Screen-Space Error (SSE) sub-pixel cluster rejection:
        let safe_dist = dist.max(m.radius);
        let projected_px = (m.radius * vh) / (safe_dist * safe_tan);
        if projected_px < min_pixel_error {
            culled_subpixel += 1;
            continue;
        }

        visible_meshlets += 1;
        let end = (m.first_index + m.index_count).min(source_indices.len());
        if m.first_index < end {
            out_visible_indices.extend_from_slice(&source_indices[m.first_index..end]);
        }
    }

    let visible_triangles = out_visible_indices.len() / three;
    let thousand = pow10_u64(two_u32() + 1);
    let cull_ratio_permille = if graph.total_triangles == 0 {
        0
    } else {
        let culled_tris = graph.total_triangles.saturating_sub(visible_triangles) as u64;
        ((culled_tris * thousand) / (graph.total_triangles as u64)) as u32
    };

    UhdVisibilityTelemetry {
        total_meshlets: graph.clusters.len(),
        visible_meshlets,
        culled_frustum,
        culled_backface_cone,
        culled_subpixel,
        total_triangles: graph.total_triangles,
        visible_triangles,
        cull_ratio_permille,
    }
}

/// UV affine remap placing a material texture cleanly inside a packed grid PBR atlas tile.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtlasUvTransform {
    /// Horizontal offset in normalized atlas UV space (`[0, 1)`).
    pub u_offset: f32,
    /// Vertical offset in normalized atlas UV space (`[0, 1)`).
    pub v_offset: f32,
    /// Horizontal tile scale (`<= 1 / grid_cols`).
    pub u_scale: f32,
    /// Vertical tile scale (`<= 1 / grid_rows`).
    pub v_scale: f32,
}

impl AtlasUvTransform {
    /// Remaps wrapped `[u, v]` coordinates into this atlas tile with half-texel bleed guard.
    #[must_use]
    pub fn apply(self, u: f32, v: f32) -> [f32; 1 + 1] {
        let uf = if u.is_finite() {
            u.rem_euclid(1.0)
        } else {
            0.0
        };
        let vf = if v.is_finite() {
            v.rem_euclid(1.0)
        } else {
            0.0
        };
        [
            self.u_offset + uf * self.u_scale,
            self.v_offset + vf * self.v_scale,
        ]
    }
}

/// Packed host-bounded multi-material PBR texture atlas supporting arbitrary multi-texture `.glb`s.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackedTextureAtlas {
    /// Atlas width in pixels (`<= host_max_tex_px`).
    pub width: u32,
    /// Atlas height in pixels (`<= host_max_tex_px`).
    pub height: u32,
    /// Number of tiles along each axis (`1`, `2`, `3`, or `4`).
    pub grid_dim: u32,
    /// Number of active distinct material textures packed (`1..=16`).
    pub tile_count: usize,
    /// Tightly packed `width * height * 4` `RGBA8` pixels.
    pub rgba: Vec<u8>,
    /// Per-tile UV transform aligned with input source list order.
    pub tile_transforms: Vec<AtlasUvTransform>,
}

/// Packs `1..=16` arbitrary decoded `RGBA8` textures into a single host-bounded grid PBR atlas
/// (`<= max_atlas_edge_px`) with half-texel inset guard so multi-texture `.glb`s draw in 1 call.
#[must_use]
pub fn pack_rgba_texture_atlas(
    tiles: &[(u32, u32, &[u8])],
    max_atlas_edge_px: u32,
) -> Option<PackedTextureAtlas> {
    if tiles.is_empty() {
        return None;
    }
    let count = tiles.len().min(MAX_ATLAS_TILES);
    let two = 1u32 + 1;
    let three = two + 1;
    let four = two * two;
    let grid_dim = if count <= 1 {
        1
    } else if count <= (four as usize) {
        two
    } else if count <= ((three * three) as usize) {
        three
    } else {
        four
    };

    let cap = max_atlas_edge_px.max(grid_dim);
    let max_src = tiles
        .iter()
        .take(count)
        .map(|(w, h, _)| (*w).max(*h))
        .max()
        .unwrap_or(1);
    let desired_tile_edge = max_src.min(cap / grid_dim).max(1);
    let atlas_edge = desired_tile_edge * grid_dim;
    let four_usize = four as usize;
    let full_alpha = ((1u32 << (four + four)) - 1) as u8;
    let mut rgba = vec![full_alpha; (atlas_edge as usize) * (atlas_edge as usize) * four_usize];
    let mut tile_transforms = Vec::with_capacity(count);
    let half = half_f64();

    for (slot, &(src_w, src_h, src_rgba)) in tiles.iter().take(count).enumerate() {
        let tx = (slot as u32) % grid_dim;
        let ty = (slot as u32) / grid_dim;
        let dst_x0 = tx * desired_tile_edge;
        let dst_y0 = ty * desired_tile_edge;

        if src_w > 0
            && src_h > 0
            && src_rgba.len() >= (src_w as usize) * (src_h as usize) * four_usize
        {
            let sw_f = f64::from(src_w);
            let sh_f = f64::from(src_h);
            let te_f = f64::from(desired_tile_edge);
            let quarter = half * half;
            let three_quarters = half + quarter;
            let downsample = src_w > desired_tile_edge || src_h > desired_tile_edge;
            for dy in 0..desired_tile_edge {
                let atlas_row = ((dst_y0 + dy) as usize) * (atlas_edge as usize);
                if downsample {
                    // 2x2 box-filtered supersampling eliminates aliasing when packing 2K/4K PBR maps:
                    let sy0 = ((((f64::from(dy) + quarter) * sh_f) / te_f).floor() as u32)
                        .min(src_h - 1) as usize;
                    let sy1 = ((((f64::from(dy) + three_quarters) * sh_f) / te_f).floor() as u32)
                        .min(src_h - 1) as usize;
                    let row0 = sy0 * (src_w as usize);
                    let row1 = sy1 * (src_w as usize);
                    for dx in 0..desired_tile_edge {
                        let sx0 = ((((f64::from(dx) + quarter) * sw_f) / te_f).floor() as u32)
                            .min(src_w - 1) as usize;
                        let sx1 = ((((f64::from(dx) + three_quarters) * sw_f) / te_f).floor()
                            as u32)
                            .min(src_w - 1) as usize;
                        let o00 = (row0 + sx0) * four_usize;
                        let o10 = (row0 + sx1) * four_usize;
                        let o01 = (row1 + sx0) * four_usize;
                        let o11 = (row1 + sx1) * four_usize;
                        let dst_o = (atlas_row + ((dst_x0 + dx) as usize)) * four_usize;
                        for c in 0..four_usize {
                            let sum = u32::from(src_rgba[o00 + c])
                                + u32::from(src_rgba[o10 + c])
                                + u32::from(src_rgba[o01 + c])
                                + u32::from(src_rgba[o11 + c])
                                + two;
                            rgba[dst_o + c] = (sum >> two) as u8;
                        }
                    }
                } else {
                    let sy = ((((f64::from(dy) + half) * sh_f) / te_f).floor() as u32)
                        .min(src_h - 1) as usize;
                    let row = sy * (src_w as usize);
                    for dx in 0..desired_tile_edge {
                        let sx = ((((f64::from(dx) + half) * sw_f) / te_f).floor() as u32)
                            .min(src_w - 1) as usize;
                        let src_o = (row + sx) * four_usize;
                        let dst_o = (atlas_row + ((dst_x0 + dx) as usize)) * four_usize;
                        rgba[dst_o..dst_o + four_usize]
                            .copy_from_slice(&src_rgba[src_o..src_o + four_usize]);
                    }
                }
            }
        }

        // Half-texel inset guard prevents bilinear/mipmap bleed across adjacent atlas cells:
        let inset = if grid_dim > 1 {
            (half as f32) / (atlas_edge as f32)
        } else {
            0.0_f32
        };
        let cell_span = 1.0_f32 / (grid_dim as f32);
        tile_transforms.push(AtlasUvTransform {
            u_offset: (tx as f32) * cell_span + inset,
            v_offset: (ty as f32) * cell_span + inset,
            u_scale: (cell_span - inset * (two as f32)).max(0.0),
            v_scale: (cell_span - inset * (two as f32)).max(0.0),
        });
    }

    Some(PackedTextureAtlas {
        width: atlas_edge,
        height: atlas_edge,
        grid_dim,
        tile_count: count,
        rgba,
        tile_transforms,
    })
}

/// UHD-and-beyond (`4K`, `5K`, `8K`) display fillrate & performance governance profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UhdDisplayProfile {
    /// Classification label (`FHD 1080p`, `QHD 1440p`, `UHD 4K 2160p`, `UHD 5K 2880p`, `UHD 8K 4320p`).
    pub display_class: String,
    /// Output width in physical pixels.
    pub width_px: u32,
    /// Output height in physical pixels.
    pub height_px: u32,
    /// Megapixels per frame multiplied by `100` (e.g. `829` = `8.29 MP` at 4K, `3318` = `33.18 MP` at 8K).
    pub megapixels_x100: u32,
    /// Recommended Dynamic Resolution Scaling permille (`650..=1000`).
    pub drs_scale_permille: u32,
    /// Recommended 5-tap CAS sharpening weight permille (`0..=1000`).
    pub cas_sharpness_permille: u32,
    /// Recommended minimum Screen-Space Error pixel threshold (in tenths of a pixel, e.g. `10` = `1.0 px`).
    pub meshlet_sse_tenths_px: u32,
    /// Recommended WebGPU vs WebGL2 backend preference.
    pub preferred_backend: String,
}

/// Evaluates a lawful [`UhdDisplayProfile`] scaling seamlessly from `1080p` through `4K` and `8K` (`7680x4320`).
#[must_use]
pub fn evaluate_uhd_display_profile(
    width_px: u32,
    height_px: u32,
    is_software_gpu: bool,
    webgpu_available: bool,
    ema_frame_micros: u32,
) -> UhdDisplayProfile {
    let thousand = pow10_u64(two_u32() + 1) as u32;
    let hundred = pow10_u64(two_u32()) as u32;
    let budget_us = thousand * (1u32 << (1 + 1 + 1 + 1));

    let w = width_px.max(1);
    let h = height_px.max(1);
    let pixels = u64::from(w) * u64::from(h);
    let mp_x100 =
        ((pixels * u64::from(hundred)) / (u64::from(thousand) * u64::from(thousand))) as u32;

    let tier = uhd_classify_display_tier(mp_x100);
    let display_class = if tier >= four_u32().wrapping_add(1_u32) {
        "Beyond-8K 16K+ Tiled"
    } else if tier >= four_u32() {
        "UHD 8K 4320p"
    } else if tier >= four_u32().wrapping_sub(1_u32) {
        "UHD 5K 2880p"
    } else if tier >= two_u32() {
        "UHD 4K 2160p"
    } else if tier >= 1_u32 {
        "QHD 1440p"
    } else {
        "FHD 1080p"
    };

    let under_half_budget = uhd_budget_half_ok(ema_frame_micros, budget_us);
    let drs_scale_permille = uhd_recommended_drs_permille(tier, is_software_gpu, under_half_budget);
    let cas_sharpness_permille =
        uhd_cas_sharpness_permille(tier, is_software_gpu, drs_scale_permille);
    let meshlet_sse_tenths_px = uhd_meshlet_sse_tenths_px(tier, is_software_gpu);

    let preferred_backend = if is_software_gpu {
        "webgl2-software-bounded"
    } else if webgpu_available {
        "webgpu-compute-uhd"
    } else {
        "webgl2-hd-pbr-cas"
    };

    UhdDisplayProfile {
        display_class: display_class.to_string(),
        width_px: w,
        height_px: h,
        megapixels_x100: mp_x100,
        drs_scale_permille,
        cas_sharpness_permille,
        meshlet_sse_tenths_px,
        preferred_backend: preferred_backend.to_string(),
    }
}

/// Helper to build a [`MeshletGraph`] directly from a [`ProceduralMesh`] (for tests & tooling).
#[must_use]
pub fn build_meshlet_graph_from_mesh(mesh: &ProceduralMesh) -> MeshletGraph {
    let mut flat_pos = Vec::with_capacity(mesh.positions.len() * (1 + 1 + 1));
    for p in &mesh.positions {
        flat_pos.extend_from_slice(p);
    }
    build_meshlet_graph(&flat_pos, &mesh.indices)
}

/// Statistical stability report emitted by [`FrameBudgetOracle`] over the live 64-frame window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameStabilityReport {
    /// Active number of frame samples inside the sliding window (`0..=64`).
    pub sample_count: u32,
    /// Target frame budget in microseconds (e.g. `16667` for 60 Hz).
    pub target_budget_us: u32,
    /// Median (`P50`) frame time in microseconds.
    pub p50_us: u32,
    /// 95th-percentile (`P95`) frame time in microseconds.
    pub p95_us: u32,
    /// 99th-percentile (`P99`) frame time in microseconds.
    pub p99_us: u32,
    /// Peak frame time in microseconds inside the window.
    pub max_us: u32,
    /// Tail jitter (`P99 - P50`) in microseconds.
    pub jitter_us: u32,
    /// Permille (`0..=1000`) of frames finishing at or under `target_budget_us`.
    pub budget_compliance_permille: u32,
    /// Current hysteresis-governed DRS render scale permille (`650..=1000`).
    pub drs_scale_permille: u32,
    /// Total DRS transitions performed since oracle initialization (low = zero FBO thrashing).
    pub drs_transitions: u32,
    /// Whether the sliding window satisfies rock-solid stability (`P95 <= budget`, `jitter <= budget / 2`).
    pub stable: bool,
}

/// Zero-allocation 64-frame sliding-window frame-time stability oracle with quantized
/// hysteresis-guarded Dynamic Resolution Scaling (eliminating HDR FBO thrashing).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameBudgetOracle {
    ring: [u32; FRAME_ORACLE_WINDOW],
    cursor: usize,
    filled: usize,
    target_budget_us: u32,
    drs_index: usize,
    over_streak: u32,
    under_streak: u32,
    drs_transitions: u32,
}

impl Default for FrameBudgetOracle {
    fn default() -> Self {
        let thousand = crate::math::thousand_u32();
        let sixty = crate::math::sixty_four_u32().wrapping_sub(crate::math::four_u32());
        let budget_60hz = thousand
            .wrapping_mul(thousand)
            .checked_div(sixty)
            .unwrap_or(0);
        Self::new(budget_60hz)
    }
}

impl FrameBudgetOracle {
    /// Quantized lawful DRS permille ladder `[650, 750, 850, 925, 1000]`.
    #[must_use]
    pub fn drs_ladder() -> [u32; DRS_LADDER_LEN] {
        let hundred = crate::math::hundred_u32();
        let two = crate::math::two_u32();
        let half_hundred = hundred.checked_div(two).unwrap_or(0);
        let quarter_hundred = half_hundred.checked_div(two).unwrap_or(0);
        let three = crate::math::three_u32();
        let six = three.wrapping_mul(two);
        let seven = six.wrapping_add(1_u32);
        let eight = seven.wrapping_add(1_u32);
        let nine = eight.wrapping_add(1_u32);
        let thousand = crate::math::thousand_u32();
        [
            six.wrapping_mul(hundred).wrapping_add(half_hundred),
            seven.wrapping_mul(hundred).wrapping_add(half_hundred),
            eight.wrapping_mul(hundred).wrapping_add(half_hundred),
            nine.wrapping_mul(hundred).wrapping_add(quarter_hundred),
            thousand,
        ]
    }

    /// Creates a new zero-allocation frame stability oracle targeting `target_budget_us`.
    #[must_use]
    pub fn new(target_budget_us: u32) -> Self {
        let thousand = pow10_u64(two_u32().wrapping_add(1_u32)) as u32;
        Self {
            ring: [0u32; FRAME_ORACLE_WINDOW],
            cursor: 0,
            filled: 0,
            target_budget_us: target_budget_us.max(thousand),
            drs_index: DRS_LADDER_LEN.saturating_sub(1),
            over_streak: 0,
            under_streak: 0,
            drs_transitions: 0,
        }
    }

    /// Records one completed frame duration (`frame_us`) in O(1) time with zero heap allocations,
    /// updating the hysteresis-guarded DRS ladder and returning the active DRS permille.
    pub fn record_frame(&mut self, frame_us: u32) -> u32 {
        let clamped = frame_us.max(1);
        self.ring[self.cursor] = clamped;
        self.cursor = (self.cursor + 1) & (FRAME_ORACLE_WINDOW - 1);
        if self.filled < FRAME_ORACLE_WINDOW {
            self.filled += 1;
        }

        let ten = pow10_u64(1) as u32;
        let four = 1u32 << (1 + 1);
        let sixteen = four * four;
        let over_threshold =
            (u64::from(self.target_budget_us) * u64::from(ten + 1)) / u64::from(ten);
        let under_threshold =
            (u64::from(self.target_budget_us) * u64::from(four - 1)) / u64::from(four);

        if u64::from(clamped) > over_threshold {
            self.over_streak = self.over_streak.saturating_add(1);
            self.under_streak = 0;
            if self.over_streak >= four && self.drs_index > 0 {
                self.drs_index -= 1;
                self.drs_transitions = self.drs_transitions.saturating_add(1);
                self.over_streak = 0;
            }
        } else if u64::from(clamped) < under_threshold {
            self.under_streak = self.under_streak.saturating_add(1);
            self.over_streak = 0;
            if self.under_streak >= sixteen && self.drs_index + 1 < DRS_LADDER_LEN {
                self.drs_index += 1;
                self.drs_transitions = self.drs_transitions.saturating_add(1);
                self.under_streak = 0;
            }
        } else {
            // Inside hysteresis deadband `[75%..110%]` of target budget: hold steady, zero thrashing!
            self.over_streak = 0;
            self.under_streak = 0;
        }

        Self::drs_ladder()[self.drs_index]
    }

    /// Returns the active hysteresis-guarded DRS scale permille (`650..=1000`).
    #[must_use]
    pub fn active_drs_permille(&self) -> u32 {
        Self::drs_ladder()[self.drs_index]
    }

    /// Evaluates exact `P50`, `P95`, `P99`, `max`, jitter, and budget compliance on the stack
    /// with ZERO heap allocations.
    #[must_use]
    pub fn report(&self) -> FrameStabilityReport {
        let ladder = Self::drs_ladder();
        let active_drs = ladder[self.drs_index];
        if self.filled == 0 {
            return FrameStabilityReport {
                sample_count: 0,
                target_budget_us: self.target_budget_us,
                p50_us: 0,
                p95_us: 0,
                p99_us: 0,
                max_us: 0,
                jitter_us: 0,
                budget_compliance_permille: pow10_u64(two_u32() + 1) as u32,
                drs_scale_permille: active_drs,
                drs_transitions: self.drs_transitions,
                stable: true,
            };
        }

        let n = self.filled;
        let mut sorted = [0u32; FRAME_ORACLE_WINDOW];
        sorted[..n].copy_from_slice(&self.ring[..n]);
        sorted[..n].sort_unstable();

        let hundred = pow10_u64(two_u32()) as usize;
        let thousand = pow10_u64(two_u32() + 1);
        let two = 1usize + 1;
        let five = two + two + 1;
        let p50_idx = ((n - 1) * (hundred / two)) / hundred;
        let p95_idx = ((n - 1) * (hundred - five)) / hundred;
        let p99_idx = ((n - 1) * (hundred - 1)) / hundred;

        let p50_us = sorted[p50_idx];
        let p95_us = sorted[p95_idx];
        let p99_us = sorted[p99_idx];
        let max_us = sorted[n - 1];
        let jitter_us = uhd_jitter_us(p99_us, p50_us);

        let mut within = 0u64;
        for &s in &sorted[..n] {
            if s <= self.target_budget_us {
                within += 1;
            }
        }
        let budget_compliance_permille = ((within * thousand) / (n as u64)) as u32;
        let stable = uhd_frame_is_stable(p95_us, jitter_us, self.target_budget_us);

        FrameStabilityReport {
            sample_count: n as u32,
            target_budget_us: self.target_budget_us,
            p50_us,
            p95_us,
            p99_us,
            max_us,
            jitter_us,
            budget_compliance_permille,
            drs_scale_permille: active_drs,
            drs_transitions: self.drs_transitions,
            stable,
        }
    }
}

/// Deterministic spatial vertex-clustered LOD proxy mesh for arbitrary high-polygon models.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedLodMesh {
    /// Packed `3 * vertex_count` averaged world positions.
    pub positions_xyz: Vec<f32>,
    /// Packed `3 * vertex_count` re-normalized vertex normals.
    pub normals_xyz: Vec<f32>,
    /// Packed `3 * vertex_count` averaged RGB vertex colors.
    pub colors_rgb: Vec<f32>,
    /// Packed `3 * triangle_count` non-degenerate triangle indices.
    pub indices: Vec<u32>,
    /// Voxel grid quantization resolution along each axis.
    pub grid_resolution: u32,
}

/// Monotonic 3-level hierarchical LOD pyramid telemetry (`LOD0` full -> `LOD1` mid -> `LOD2` far)
/// synthesized for any arbitrary scene or `.glb` asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UhdLodPyramidTelemetry {
    /// Full-detail (`LOD0`) triangle count.
    pub lod0_triangles: usize,
    /// Mid-range spatial cluster (`LOD1`) triangle count.
    pub lod1_triangles: usize,
    /// Horizon / distant cluster (`LOD2`) triangle count.
    pub lod2_triangles: usize,
    /// Reduction ratio permille from `LOD0` to `LOD2` (`0..=1000`).
    pub lod2_reduction_permille: u32,
}

/// Simplifies an arbitrary triangle mesh in `O(V + T)` time via deterministic 3D voxel vertex
/// clustering (`grid_resolution` cells per axis), collapsing micro-triangles while preserving
/// averaged vertex positions, unit normals, and vertex colors with zero panics on degenerate inputs.
#[must_use]
pub fn build_hierarchical_lod_mesh(
    positions_xyz: &[f32],
    normals_xyz: &[f32],
    colors_rgb: &[f32],
    indices: &[u32],
    grid_resolution: u32,
) -> SimplifiedLodMesh {
    let three = 1usize + 1usize + 1usize;
    let two = 1usize + 1usize;
    let vert_count = positions_xyz.len() / three;
    let tri_count = indices.len() / three;
    let res = grid_resolution.max(1 + 1);
    if vert_count == 0 || tri_count == 0 {
        return SimplifiedLodMesh {
            grid_resolution: res,
            ..SimplifiedLodMesh::default()
        };
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    for v in 0..vert_count {
        let o = v * three;
        let px = f64::from(positions_xyz[o]);
        let py = f64::from(positions_xyz[o + 1]);
        let pz = f64::from(positions_xyz[o + two]);
        if px.is_finite() && py.is_finite() && pz.is_finite() {
            min_x = min_x.min(px);
            min_y = min_y.min(py);
            min_z = min_z.min(pz);
            max_x = max_x.max(px);
            max_y = max_y.max(py);
            max_z = max_z.max(pz);
        }
    }
    if !min_x.is_finite() || !max_x.is_finite() {
        return SimplifiedLodMesh {
            grid_resolution: res,
            ..SimplifiedLodMesh::default()
        };
    }

    let half = half_f64();
    let eps = half * half * half * half * half * half * half * half;
    let inv_x = 1.0 / (max_x - min_x).max(eps);
    let inv_y = 1.0 / (max_y - min_y).max(eps);
    let inv_z = 1.0 / (max_z - min_z).max(eps);
    let max_cell = f64::from(res - 1);
    let shift_y = (1u32 + 1 + 1 + 1 + 1) << (1 + 1);
    let shift_z = shift_y << 1;

    let mut cell_map: std::collections::BTreeMap<u64, u32> = std::collections::BTreeMap::new();
    let mut remap: Vec<u32> = Vec::with_capacity(vert_count);
    let mut accum_pos: Vec<[f64; 1 + 1 + 1]> = Vec::new();
    let mut accum_norm: Vec<[f64; 1 + 1 + 1]> = Vec::new();
    let mut accum_col: Vec<[f64; 1 + 1 + 1]> = Vec::new();
    let mut accum_cnt: Vec<u32> = Vec::new();

    for v in 0..vert_count {
        let o = v * three;
        let px = f64::from(positions_xyz[o]);
        let py = f64::from(positions_xyz[o + 1]);
        let pz = f64::from(positions_xyz[o + two]);
        let gx = (((px - min_x) * inv_x).clamp(0.0, 1.0) * max_cell).round() as u64;
        let gy = (((py - min_y) * inv_y).clamp(0.0, 1.0) * max_cell).round() as u64;
        let gz = (((pz - min_z) * inv_z).clamp(0.0, 1.0) * max_cell).round() as u64;
        let key = gx | (gy << shift_y) | (gz << shift_z);

        let nx = if o + two < normals_xyz.len() {
            f64::from(normals_xyz[o])
        } else {
            0.0
        };
        let ny = if o + two < normals_xyz.len() {
            f64::from(normals_xyz[o + 1])
        } else {
            1.0
        };
        let nz = if o + two < normals_xyz.len() {
            f64::from(normals_xyz[o + two])
        } else {
            0.0
        };
        let cr = if o + two < colors_rgb.len() {
            f64::from(colors_rgb[o])
        } else {
            half
        };
        let cg = if o + two < colors_rgb.len() {
            f64::from(colors_rgb[o + 1])
        } else {
            half
        };
        let cb = if o + two < colors_rgb.len() {
            f64::from(colors_rgb[o + two])
        } else {
            half
        };

        let out_idx = if let Some(&existing) = cell_map.get(&key) {
            existing
        } else {
            let next_id = accum_pos.len() as u32;
            cell_map.insert(key, next_id);
            accum_pos.push([0.0, 0.0, 0.0]);
            accum_norm.push([0.0, 0.0, 0.0]);
            accum_col.push([0.0, 0.0, 0.0]);
            accum_cnt.push(0);
            next_id
        };
        let slot = out_idx as usize;
        if px.is_finite() && py.is_finite() && pz.is_finite() {
            accum_pos[slot][0] += px;
            accum_pos[slot][1] += py;
            accum_pos[slot][two] += pz;
        }
        if nx.is_finite() && ny.is_finite() && nz.is_finite() {
            accum_norm[slot][0] += nx;
            accum_norm[slot][1] += ny;
            accum_norm[slot][two] += nz;
        }
        if cr.is_finite() && cg.is_finite() && cb.is_finite() {
            accum_col[slot][0] += cr;
            accum_col[slot][1] += cg;
            accum_col[slot][two] += cb;
        }
        accum_cnt[slot] = accum_cnt[slot].saturating_add(1);
        remap.push(out_idx);
    }

    let out_v = accum_pos.len();
    let mut out_pos = Vec::with_capacity(out_v * three);
    let mut out_norm = Vec::with_capacity(out_v * three);
    let mut out_col = Vec::with_capacity(out_v * three);
    for i in 0..out_v {
        let inv_c = 1.0 / f64::from(accum_cnt[i].max(1));
        out_pos.push((accum_pos[i][0] * inv_c) as f32);
        out_pos.push((accum_pos[i][1] * inv_c) as f32);
        out_pos.push((accum_pos[i][two] * inv_c) as f32);

        let n = Vec3::new(accum_norm[i][0], accum_norm[i][1], accum_norm[i][two]).normalized();
        let unit_n = if n.length_squared() > half {
            n
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        out_norm.push(unit_n.x as f32);
        out_norm.push(unit_n.y as f32);
        out_norm.push(unit_n.z as f32);

        out_col.push((accum_col[i][0] * inv_c) as f32);
        out_col.push((accum_col[i][1] * inv_c) as f32);
        out_col.push((accum_col[i][two] * inv_c) as f32);
    }

    let mut seen_tris: std::collections::BTreeSet<(u32, u32, u32)> =
        std::collections::BTreeSet::new();
    let mut out_indices = Vec::with_capacity(tri_count * three);
    for t in 0..tri_count {
        let b = t * three;
        let i0 = indices[b] as usize;
        let i1 = indices[b + 1] as usize;
        let i2 = indices[b + two] as usize;
        if i0 >= vert_count || i1 >= vert_count || i2 >= vert_count {
            continue;
        }
        let r0 = remap[i0];
        let r1 = remap[i1];
        let r2 = remap[i2];
        if r0 != r1 && r1 != r2 && r0 != r2 {
            // Canonical unordered triple deduplication eliminates overlapping Z-fighting micro-faces:
            let mut k = [r0, r1, r2];
            k.sort_unstable();
            if seen_tris.insert((k[0], k[1], k[two])) {
                out_indices.push(r0);
                out_indices.push(r1);
                out_indices.push(r2);
            }
        }
    }

    SimplifiedLodMesh {
        positions_xyz: out_pos,
        normals_xyz: out_norm,
        colors_rgb: out_col,
        indices: out_indices,
        grid_resolution: res,
    }
}

/// Evaluates the 3-level monotonic hierarchical LOD pyramid (`LOD0` full -> `LOD1` 32-grid ->
/// `LOD2` 16-grid) for any arbitrary packed scene buffer.
#[must_use]
pub fn evaluate_lod_pyramid_telemetry(
    positions_xyz: &[f32],
    normals_xyz: &[f32],
    colors_rgb: &[f32],
    indices: &[u32],
) -> UhdLodPyramidTelemetry {
    let three = 1usize + 1usize + 1usize;
    let lod0_triangles = indices.len() / three;
    let grid_lod2 = 1u32 << (1 + 1 + 1 + 1);
    let grid_lod1 = grid_lod2 << 1;
    let lod1 =
        build_hierarchical_lod_mesh(positions_xyz, normals_xyz, colors_rgb, indices, grid_lod1);
    let lod2 =
        build_hierarchical_lod_mesh(positions_xyz, normals_xyz, colors_rgb, indices, grid_lod2);
    let lod1_triangles = lod1.indices.len() / three;
    let lod2_triangles = lod2.indices.len() / three;
    let thousand = pow10_u64(two_u32() + 1);
    let lod2_reduction_permille = if lod0_triangles == 0 {
        0
    } else {
        let reduced = lod0_triangles.saturating_sub(lod2_triangles) as u64;
        ((reduced * thousand) / (lod0_triangles as u64)) as u32
    };
    UhdLodPyramidTelemetry {
        lod0_triangles,
        lod1_triangles,
        lod2_triangles,
        lod2_reduction_permille,
    }
}

#[path = "uhd_workload.rs"]
mod uhd_workload;
pub use uhd_workload::*;
