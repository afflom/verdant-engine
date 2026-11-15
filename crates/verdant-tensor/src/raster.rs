//! Portable `16x16` tile-binned depth-buffered software PBR framebuffer over `uor-matmul`.

use serde::{Deserialize, Serialize};

use crate::linear::batch_transform_positions_exact;
use crate::{
    geometric_epsilon, half_f64, pow2_u64, prism_kernel, two_f64, TensorComputeError,
    DIM_HOMOGENEOUS, DIM_SPATIAL,
};

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
    let tile_edge = prism_kernel::tileEdgePixels().expect("verified Prism tileEdgePixels") as u32;
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

    let channels =
        prism_kernel::rgbaChannelCount().expect("verified Prism rgbaChannelCount") as usize;
    let eight = prism_kernel::sEight().expect("verified Prism sEight") as u32;
    let byte_max = (pow2_u64(eight) - 1) as f32;
    let pct_denom =
        prism_kernel::percentDenominator().expect("verified Prism percentDenominator") as f32;
    let env = prism_kernel::lodEnvelopeByRank(lod_rank).expect("verified Prism lodEnvelopeByRank");

    let sky_r = (env.skyRedPercent as f32) / pct_denom;
    let sky_g = (env.skyGreenPercent as f32) / pct_denom;
    let sky_b = (env.skyBluePercent as f32) / pct_denom;
    let fog_r = (env.fogRedPercent as f32) / pct_denom;
    let fog_g = (env.fogGreenPercent as f32) / pct_denom;
    let fog_b = (env.fogBluePercent as f32) / pct_denom;

    let pixel_count = (width as usize) * (height as usize);
    let mut rgba8 = vec![0_u8; pixel_count * channels];
    let depth = vec![1.0_f32; pixel_count];

    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let h_minus = (height.saturating_sub(1)).max(1) as f32;
    for y in 0..height {
        let t = (y as f32) / h_minus;
        let r = ((sky_r * (1.0 - t) + fog_r * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let g = ((sky_g * (1.0 - t) + fog_g * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let b = ((sky_b * (1.0 - t) + fog_b * t).clamp(0.0, 1.0) * byte_max).round() as u8;
        let mut x = 0_u32;
        while x < width {
            let base = ((y * width + x) as usize) * channels;
            rgba8[base] = r;
            rgba8[base + 1] = g;
            rgba8[base + i2] = b;
            rgba8[base + i3] = byte_max as u8;
            x += 1;
        }
    }

    let tiles_x = width / tile_edge;
    let tiles_y = height / tile_edge;
    let tiles_shaded = tiles_x * tiles_y;
    if triangles.is_empty() {
        return Ok(SoftwareTileFramebuffer {
            width,
            height,
            rgba8,
            depth,
            tiles_shaded,
            rasterized_triangles: 0,
        });
    }

    let mut flat_positions = Vec::with_capacity(triangles.len() * DIM_SPATIAL);
    for tri in triangles {
        flat_positions.push(tri.positions[0]);
        flat_positions.push(tri.positions[1]);
        flat_positions.push(tri.positions[i2]);
    }
    let clip_verts = batch_transform_positions_exact(&flat_positions, view_proj_cols)?;

    let mut depth_buf = depth;
    let mut rasterized_triangles = 0_u32;
    let sun = normalize3(sun_direction);
    let half = half_f64() as f32;
    let two = two_f64() as f32;
    let eps = geometric_epsilon() as f32;
    let w_f = width as f32;
    let h_f = height as f32;
    let shadow_boost = if env.shadowsActive { 1.0_f32 } else { half };
    let ibl_boost = if env.iblActive { 1.0_f32 } else { half };

    for (tri_idx, tri) in triangles.iter().enumerate() {
        let base_v = tri_idx * DIM_SPATIAL;
        let c0 = clip_verts[base_v];
        let c1 = clip_verts[base_v + 1];
        let c2 = clip_verts[base_v + i2];
        if c0[i3] <= eps || c1[i3] <= eps || c2[i3] <= eps {
            continue;
        }

        struct ScreenVert {
            sx: f32,
            sy: f32,
            ndc_z: f32,
        }

        let to_screen = |c: [f32; DIM_HOMOGENEOUS]| -> ScreenVert {
            let inv_w = 1.0 / c[i3];
            let ndc_x = c[0] * inv_w;
            let ndc_y = c[1] * inv_w;
            let ndc_z = (c[i2] * inv_w * half + half).clamp(0.0, 1.0);
            let sx = (ndc_x * half + half) * w_f;
            let sy = (1.0 - (ndc_y * half + half)) * h_f;
            ScreenVert { sx, sy, ndc_z }
        };

        let s0 = to_screen(c0);
        let s1 = to_screen(c1);
        let s2 = to_screen(c2);
        let area = edge_fn(s0.sx, s0.sy, s1.sx, s1.sy, s2.sx, s2.sy);
        if area.abs() <= eps {
            continue;
        }
        let inv_area = 1.0 / area;

        let min_x = s0.sx.min(s1.sx).min(s2.sx).floor().max(0.0) as u32;
        let max_x = (s0.sx.max(s1.sx).max(s2.sx).ceil().max(0.0) as u32).min(width - 1);
        let min_y = s0.sy.min(s1.sy).min(s2.sy).floor().max(0.0) as u32;
        let max_y = (s0.sy.max(s1.sy).max(s2.sy).ceil().max(0.0) as u32).min(height - 1);
        if min_x > max_x || min_y > max_y {
            continue;
        }

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

        let lit_r =
            ((tri.base_color[0] * shade) / (1.0 + tri.base_color[0] * shade) * two).clamp(0.0, 1.0);
        let lit_g =
            ((tri.base_color[1] * shade) / (1.0 + tri.base_color[1] * shade) * two).clamp(0.0, 1.0);
        let lit_b = ((tri.base_color[i2] * shade) / (1.0 + tri.base_color[i2] * shade) * two)
            .clamp(0.0, 1.0);

        let r_u8 = (lit_r * byte_max).round() as u8;
        let g_u8 = (lit_g * byte_max).round() as u8;
        let b_u8 = (lit_b * byte_max).round() as u8;

        let tile_x0 = min_x / tile_edge;
        let tile_x1 = max_x / tile_edge;
        let tile_y0 = min_y / tile_edge;
        let tile_y1 = max_y / tile_edge;

        for ty in tile_y0..=tile_y1 {
            let py_start = (ty * tile_edge).max(min_y);
            let py_end = ((ty + 1) * tile_edge - 1).min(max_y);
            for tx in tile_x0..=tile_x1 {
                let px_start = (tx * tile_edge).max(min_x);
                let px_end = ((tx + 1) * tile_edge - 1).min(max_x);
                for py in py_start..=py_end {
                    let cy = (py as f32) + half;
                    for px in px_start..=px_end {
                        let cx = (px as f32) + half;
                        let w0 = edge_fn(s1.sx, s1.sy, s2.sx, s2.sy, cx, cy) * inv_area;
                        let w1 = edge_fn(s2.sx, s2.sy, s0.sx, s0.sy, cx, cy) * inv_area;
                        let w2 = 1.0 - w0 - w1;
                        if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                            let z = w0 * s0.ndc_z + w1 * s1.ndc_z + w2 * s2.ndc_z;
                            let idx = (py * width + px) as usize;
                            if z < depth_buf[idx] {
                                depth_buf[idx] = z;
                                let c_idx = idx * channels;
                                rgba8[c_idx] = r_u8;
                                rgba8[c_idx + 1] = g_u8;
                                rgba8[c_idx + i2] = b_u8;
                                rgba8[c_idx + i3] = byte_max as u8;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(SoftwareTileFramebuffer {
        width,
        height,
        rgba8,
        depth: depth_buf,
        tiles_shaded,
        rasterized_triangles,
    })
}
