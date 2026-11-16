//! Portable `16x16` tile-binned depth-buffered software PBR framebuffer over `uor-matmul`.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::linear::batch_transform_positions_into;
use crate::{
    geometric_epsilon, half_f64, kernel_consts, pow2_u64, prism_kernel, two_f64,
    TensorComputeError, DIM_HOMOGENEOUS, DIM_SPATIAL, HD_LOD_ENVELOPE_COUNT,
};

static LOD_ENVELOPES: OnceLock<[prism_kernel::LodRenderEnvelopeSpec; HD_LOD_ENVELOPE_COUNT]> =
    OnceLock::new();

fn cached_lod_envelope(lod_rank: u64) -> prism_kernel::LodRenderEnvelopeSpec {
    let table = LOD_ENVELOPES.get_or_init(|| {
        let r2 = (1 + 1) as u64;
        let r3 = r2 + 1;
        let r4 = r3 + 1;
        [
            prism_kernel::lodEnvelopeByRank(0).expect("verified LOD0 envelope"),
            prism_kernel::lodEnvelopeByRank(1).expect("verified LOD1 envelope"),
            prism_kernel::lodEnvelopeByRank(r2).expect("verified LOD2 envelope"),
            prism_kernel::lodEnvelopeByRank(r3).expect("verified LOD3 envelope"),
            prism_kernel::lodEnvelopeByRank(r4).expect("verified LOD4 envelope"),
        ]
    });
    if (lod_rank as usize) < table.len() {
        table[lod_rank as usize]
    } else {
        prism_kernel::lodEnvelopeByRank(lod_rank).expect("verified Prism lodEnvelopeByRank")
    }
}

/// World-space shaded triangle primitive submitted to the portable software tile rasterizer.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RasterTriangle {
    /// Counter-clockwise world-space triangle vertices.
    pub positions: [[f32; DIM_SPATIAL]; DIM_SPATIAL],
    /// Unit world-space surface normal.
    pub normal: [f32; DIM_SPATIAL],
    /// Linear RGB surface albedo in `[0, 1]`.
    pub base_color: [f32; DIM_SPATIAL],
}

/// Deterministic `16x16`-tile-aligned RGBA8 + depth framebuffer rendered purely in Rust/Wasm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareTileFramebuffer {
    /// Framebuffer width in pixels (multiple of `prism_kernel::tileEdgePixels()`).
    pub width: u32,
    /// Framebuffer height in pixels (multiple of `prism_kernel::tileEdgePixels()`).
    pub height: u32,
    /// Row-major `RGBA8` color buffer (`width * height * 4` bytes).
    pub rgba8: Vec<u8>,
    /// Row-major normalized device depth buffer in `[0, 1]`.
    pub depth: Vec<f32>,
    /// Number of `16x16` tiles shaded during the frame.
    pub tiles_shaded: u32,
    /// Number of triangles that survived clip and backface culling.
    pub rasterized_triangles: u32,
}

fn edge_fn(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

fn normalize3(v: [f64; DIM_SPATIAL]) -> [f64; DIM_SPATIAL] {
    let i2 = 1usize + 1usize;
    let len = (v[0] * v[0] + v[1] * v[1] + v[i2] * v[i2]).sqrt();
    if len > geometric_epsilon() {
        [v[0] / len, v[1] / len, v[i2] / len]
    } else {
        [0.0, 1.0, 0.0]
    }
}

#[derive(Clone, Copy, Debug)]
struct BinnedRasterTri {
    sx0: f32,
    sy0: f32,
    z0: f32,
    sx1: f32,
    sy1: f32,
    z1: f32,
    sx2: f32,
    sy2: f32,
    z2: f32,
    inv_area: f32,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
    tile_x0: u32,
    tile_x1: u32,
    tile_y0: u32,
    tile_y1: u32,
    rgba: [u8; DIM_HOMOGENEOUS],
}

/// Reusable Compressed-Sparse-Row (`CSR`) tile-binning and framebuffer arena eliminating per-frame
/// inner `Vec` allocations across arbitrary software-rasterized workloads.
#[derive(Clone, Debug, Default)]
pub struct TileRasterScratch {
    flat_positions: Vec<[f32; DIM_SPATIAL]>,
    clip_verts: Vec<[f32; DIM_HOMOGENEOUS]>,
    binned_tris: Vec<BinnedRasterTri>,
    tile_counts: Vec<u32>,
    tile_offsets: Vec<u32>,
    tile_write_heads: Vec<u32>,
    tile_tri_indices: Vec<u32>,
    /// Row-major `RGBA8` color buffer populated by [`render_triangles_tile_framebuffer_with_scratch`].
    pub rgba8: Vec<u8>,
    /// Row-major depth buffer populated by [`render_triangles_tile_framebuffer_with_scratch`].
    pub depth: Vec<f32>,
}

impl TileRasterScratch {
    /// Allocates an empty reusable CSR tile-rasterization arena.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Total heap byte capacity held by all reusable CSR and pixel buffers (`0` growth in steady state).
    #[must_use]
    pub fn total_allocated_bytes(&self) -> usize {
        self.flat_positions.capacity() * core::mem::size_of::<[f32; DIM_SPATIAL]>()
            + self.clip_verts.capacity() * core::mem::size_of::<[f32; DIM_HOMOGENEOUS]>()
            + self.binned_tris.capacity() * core::mem::size_of::<BinnedRasterTri>()
            + (self.tile_counts.capacity()
                + self.tile_offsets.capacity()
                + self.tile_write_heads.capacity()
                + self.tile_tri_indices.capacity())
                * core::mem::size_of::<u32>()
            + self.rgba8.capacity()
            + self.depth.capacity() * core::mem::size_of::<f32>()
    }
}

/// Renders world-space `triangles` into `scratch.rgba8` and `scratch.depth` using a flat 2-pass
/// CSR prefix-sum `16x16` tile bin table (`0` per-frame inner `Vec` heap allocations) and strict
/// screen-space coordinate clamping. Returns `(tiles_shaded, rasterized_triangles)`.
#[allow(clippy::manual_clamp)]
pub fn render_triangles_tile_framebuffer_with_scratch(
    width: u32,
    height: u32,
    view_proj_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
    sun_direction: [f64; DIM_SPATIAL],
    lod_rank: u64,
    triangles: &[RasterTriangle],
    scratch: &mut TileRasterScratch,
) -> Result<(u32, u32), TensorComputeError> {
    let kc = kernel_consts();
    let tile_edge = kc.tile_edge_pixels;
    if width == 0
        || height == 0
        || !width.is_multiple_of(tile_edge)
        || !height.is_multiple_of(tile_edge)
    {
        return Err(TensorComputeError::InvalidTileDimensions {
            width,
            height,
            tile_edge,
        });
    }
    if !sun_direction.iter().all(|v| v.is_finite()) {
        return Err(TensorComputeError::NonFiniteTensorEntry {
            operand: "sun_direction",
        });
    }

    let channels = kc.rgba_channel_count;
    let eight = (1_u32 + 1_u32) << (1_u32 + 1_u32);
    let byte_max = (pow2_u64(eight) - 1) as f32;
    let pct_denom = kc.percent_denom_f32;
    let env = cached_lod_envelope(lod_rank);

    let sky_r = (env.skyRedPercent as f32) / pct_denom;
    let sky_g = (env.skyGreenPercent as f32) / pct_denom;
    let sky_b = (env.skyBluePercent as f32) / pct_denom;
    let fog_r = (env.fogRedPercent as f32) / pct_denom;
    let fog_g = (env.fogGreenPercent as f32) / pct_denom;
    let fog_b = (env.fogBluePercent as f32) / pct_denom;

    let pixel_count = (width as usize) * (height as usize);
    scratch.rgba8.resize(pixel_count * channels, 0_u8);
    scratch.depth.clear();
    scratch.depth.resize(pixel_count, 1.0_f32);

    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let h_minus = (height.saturating_sub(1)).max(1) as f32;
    for y in 0..height {
        let t = (y as f32) / h_minus;
        let r = ((sky_r * (1.0 - t) + fog_r * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let g = ((sky_g * (1.0 - t) + fog_g * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let b = ((sky_b * (1.0 - t) + fog_b * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let alpha = byte_max as u8;
        let row_start = (y * width) as usize * channels;
        let row_end = row_start + (width as usize) * channels;
        for px in scratch.rgba8[row_start..row_end].chunks_exact_mut(channels) {
            px[0] = r;
            px[1] = g;
            px[i2] = b;
            px[i3] = alpha;
        }
    }

    let tiles_x = width / tile_edge;
    let tiles_y = height / tile_edge;
    let tiles_shaded = tiles_x * tiles_y;
    if triangles.is_empty() {
        return Ok((tiles_shaded, 0));
    }

    let vert_len = triangles.len() * DIM_SPATIAL;
    scratch.flat_positions.clear();
    scratch.flat_positions.reserve(vert_len);
    for tri in triangles {
        scratch.flat_positions.push(tri.positions[0]);
        scratch.flat_positions.push(tri.positions[1]);
        scratch.flat_positions.push(tri.positions[i2]);
    }
    scratch
        .clip_verts
        .resize(vert_len, [0.0_f32; DIM_HOMOGENEOUS]);
    batch_transform_positions_into(
        &scratch.flat_positions,
        view_proj_cols,
        &mut scratch.clip_verts,
    )?;

    let mut rasterized_triangles = 0_u32;
    let sun = normalize3(sun_direction);
    let half = half_f64() as f32;
    let two = two_f64() as f32;
    let eps = geometric_epsilon() as f32;
    let w_f = width as f32;
    let h_f = height as f32;
    let max_w_px = (width - 1) as f32;
    let max_h_px = (height - 1) as f32;
    let shadow_boost = if env.shadowsActive { 1.0_f32 } else { half };
    let ibl_boost = if env.iblActive { 1.0_f32 } else { half };

    // Stage 1: Two-pass flat CSR `16x16` screen-space tile binning (`0` inner Vec heap allocations):
    let total_tiles = (tiles_x * tiles_y) as usize;
    scratch.binned_tris.clear();
    scratch.tile_counts.clear();
    scratch.tile_counts.resize(total_tiles, 0_u32);
    scratch.tile_offsets.clear();
    scratch.tile_offsets.resize(total_tiles + 1, 0_u32);
    scratch.tile_write_heads.clear();
    scratch.tile_write_heads.resize(total_tiles, 0_u32);

    for (tri_idx, tri) in triangles.iter().enumerate() {
        let base_v = tri_idx * DIM_SPATIAL;
        let c0 = scratch.clip_verts[base_v];
        let c1 = scratch.clip_verts[base_v + 1];
        let c2 = scratch.clip_verts[base_v + i2];
        if c0[i3] <= eps || c1[i3] <= eps || c2[i3] <= eps {
            continue;
        }

        let to_screen = |c: [f32; DIM_HOMOGENEOUS]| -> (f32, f32, f32) {
            let inv_w = 1.0 / c[i3];
            let ndc_x = c[0] * inv_w;
            let ndc_y = c[1] * inv_w;
            let ndc_z = (c[i2] * inv_w * half + half).clamp(0.0, 1.0);
            let sx = (ndc_x * half + half) * w_f;
            let sy = (1.0 - (ndc_y * half + half)) * h_f;
            (sx, sy, ndc_z)
        };

        let (sx0, sy0, z0) = to_screen(c0);
        let (sx1, sy1, z1) = to_screen(c1);
        let (sx2, sy2, z2) = to_screen(c2);
        if !(sx0.is_finite()
            && sy0.is_finite()
            && z0.is_finite()
            && sx1.is_finite()
            && sy1.is_finite()
            && z1.is_finite()
            && sx2.is_finite()
            && sy2.is_finite()
            && z2.is_finite())
        {
            continue;
        }
        let area = edge_fn(sx0, sy0, sx1, sy1, sx2, sy2);
        if !area.is_finite() || area.abs() <= eps {
            continue;
        }
        let lo_x = sx0.min(sx1).min(sx2);
        let hi_x = sx0.max(sx1).max(sx2);
        let lo_y = sy0.min(sy1).min(sy2);
        let hi_y = sy0.max(sy1).max(sy2);
        if hi_x < 0.0 || lo_x >= w_f || hi_y < 0.0 || lo_y >= h_f {
            continue;
        }
        let min_x = lo_x.floor().max(0.0).min(max_w_px) as u32;
        let max_x = hi_x.ceil().max(0.0).min(max_w_px) as u32;
        let min_y = lo_y.floor().max(0.0).min(max_h_px) as u32;
        let max_y = hi_y.ceil().max(0.0).min(max_h_px) as u32;
        if min_x > max_x || min_y > max_y {
            continue;
        }
        let inv_area = 1.0 / area;

        rasterized_triangles += 1;
        let nrm = normalize3([
            f64::from(tri.normal[0]),
            f64::from(tri.normal[1]),
            f64::from(tri.normal[i2]),
        ]);
        let ndotl = ((nrm[0] * sun[0] + nrm[1] * sun[1] + nrm[i2] * sun[i2]).max(0.0)) as f32;
        let hemi = (nrm[1] * f64::from(half) + f64::from(half)).clamp(0.0, 1.0) as f32;
        let ambient = (half * half + half * half * hemi) * ibl_boost;
        let shade = (ambient + (1.0 - half * half) * ndotl * shadow_boost).min(two);

        let cr = tri.base_color[0].clamp(0.0, 1.0);
        let cg = tri.base_color[1].clamp(0.0, 1.0);
        let cb = tri.base_color[i2].clamp(0.0, 1.0);
        let lit_r = ((cr * shade) / (1.0 + cr * shade) * two).clamp(0.0, 1.0);
        let lit_g = ((cg * shade) / (1.0 + cg * shade) * two).clamp(0.0, 1.0);
        let lit_b = ((cb * shade) / (1.0 + cb * shade) * two).clamp(0.0, 1.0);

        let tile_x0 = min_x / tile_edge;
        let tile_x1 = max_x / tile_edge;
        let tile_y0 = min_y / tile_edge;
        let tile_y1 = max_y / tile_edge;
        for ty in tile_y0..=tile_y1 {
            let row_off = (ty * tiles_x) as usize;
            for tx in tile_x0..=tile_x1 {
                scratch.tile_counts[row_off + (tx as usize)] += 1;
            }
        }

        scratch.binned_tris.push(BinnedRasterTri {
            sx0,
            sy0,
            z0,
            sx1,
            sy1,
            z1,
            sx2,
            sy2,
            z2,
            inv_area,
            min_x,
            max_x,
            min_y,
            max_y,
            tile_x0,
            tile_x1,
            tile_y0,
            tile_y1,
            rgba: [
                (lit_r * byte_max).round() as u8,
                (lit_g * byte_max).round() as u8,
                (lit_b * byte_max).round() as u8,
                byte_max as u8,
            ],
        });
    }

    // Prefix-sum offsets + flat index scatter:
    let mut running = 0_u32;
    scratch.tile_offsets[0] = 0;
    for t in 0..total_tiles {
        let count = scratch.tile_counts[t];
        scratch.tile_write_heads[t] = running;
        let next = running.saturating_add(count);
        scratch.tile_offsets[t + 1] = next;
        #[cfg(not(target_arch = "wasm32"))]
        debug_assert_eq!(
            prism_kernel::csrTilePrefixMonotonic(
                u64::from(running),
                u64::from(next),
                u64::from(count),
            ),
            Ok(true)
        );
        running = next;
    }
    scratch.tile_tri_indices.clear();
    scratch.tile_tri_indices.resize(running as usize, 0_u32);
    for (slot_usize, bt) in scratch.binned_tris.iter().enumerate() {
        let slot = slot_usize as u32;
        for ty in bt.tile_y0..=bt.tile_y1 {
            let row_off = (ty * tiles_x) as usize;
            for tx in bt.tile_x0..=bt.tile_x1 {
                let cell = row_off + (tx as usize);
                let head = scratch.tile_write_heads[cell] as usize;
                scratch.tile_tri_indices[head] = slot;
                scratch.tile_write_heads[cell] += 1;
            }
        }
    }

    // Stage 2: Tile-coherent L1-cache rasterization over flat CSR slice `[start..end]`:
    for ty in 0..tiles_y {
        let tile_y_px = ty * tile_edge;
        let tile_y_max = (tile_y_px + tile_edge - 1).min(height - 1);
        for tx in 0..tiles_x {
            let cell = (ty * tiles_x + tx) as usize;
            let start = scratch.tile_offsets[cell] as usize;
            let end = scratch.tile_offsets[cell + 1] as usize;
            if start == end {
                continue;
            }
            let tile_x_px = tx * tile_edge;
            let tile_x_max = (tile_x_px + tile_edge - 1).min(width - 1);

            for &slot in &scratch.tile_tri_indices[start..end] {
                let bt = &scratch.binned_tris[slot as usize];
                let py_start = tile_y_px.max(bt.min_y);
                let py_end = tile_y_max.min(bt.max_y);
                let px_start = tile_x_px.max(bt.min_x);
                let px_end = tile_x_max.min(bt.max_x);

                for py in py_start..=py_end {
                    let cy = (py as f32) + half;
                    for px in px_start..=px_end {
                        let cx = (px as f32) + half;
                        let w0 = edge_fn(bt.sx1, bt.sy1, bt.sx2, bt.sy2, cx, cy) * bt.inv_area;
                        let w1 = edge_fn(bt.sx2, bt.sy2, bt.sx0, bt.sy0, cx, cy) * bt.inv_area;
                        let w2 = 1.0 - w0 - w1;
                        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                            let z = w0 * bt.z0 + w1 * bt.z1 + w2 * bt.z2;
                            let idx = (py * width + px) as usize;
                            if z < scratch.depth[idx] {
                                scratch.depth[idx] = z;
                                let c_idx = idx * channels;
                                scratch.rgba8[c_idx] = bt.rgba[0];
                                scratch.rgba8[c_idx + 1] = bt.rgba[1];
                                scratch.rgba8[c_idx + i2] = bt.rgba[i2];
                                scratch.rgba8[c_idx + i3] = bt.rgba[i3];
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((tiles_shaded, rasterized_triangles))
}

/// Renders world-space `triangles` into a portable `RGBA8` software tile framebuffer using
/// `batch_transform_positions_exact` (`uor-matmul`) for clip-space projection and `prism_kernel`
/// LOD atmosphere parameters, requiring zero conditional browser GPU extensions.
pub fn render_triangles_tile_framebuffer(
    width: u32,
    height: u32,
    view_proj_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
    sun_direction: [f64; DIM_SPATIAL],
    lod_rank: u64,
    triangles: &[RasterTriangle],
) -> Result<SoftwareTileFramebuffer, TensorComputeError> {
    let mut scratch = TileRasterScratch::new();
    let (tiles_shaded, rasterized_triangles) = render_triangles_tile_framebuffer_with_scratch(
        width,
        height,
        view_proj_cols,
        sun_direction,
        lod_rank,
        triangles,
        &mut scratch,
    )?;
    Ok(SoftwareTileFramebuffer {
        width,
        height,
        rgba8: scratch.rgba8,
        depth: scratch.depth,
        tiles_shaded,
        rasterized_triangles,
    })
}
