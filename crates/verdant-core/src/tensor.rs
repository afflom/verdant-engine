//! Prism-hosted exact tensor kernels (`uor-matmul`) and portable software tile PBR rasterizer.
//!
//! Replaces conditional browser GPU compute APIs with deterministic, zero-allocation exact
//! contractions across linear Laurent `(+, *)`, tropical `(max, +)`, and quantized audio
//! accumulators, paired with a `16x16` tile-binned depth-buffered software PBR framebuffer that
//! executes identically on native hosts and every WebAssembly browser runtime.

use serde::{Deserialize, Serialize};
use uor_matmul::driver::{GemmOptions, MaxPlus, Scratch};
use uor_matmul::{
    as_alphabet_tropical, slice, suggested_scratch, MatView, MatViewMut, Shape, Triple, Trop,
};

use crate::error::TensorComputeError;
use crate::math::{
    geometric_epsilon, half_f64, pow2_u64, two_f64, Mat4, Vec3, DIM_HOMOGENEOUS, DIM_SPATIAL,
};
use crate::prism_kernel;

/// Algebraic semiring classification of a verified Prism engine tensor kernel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TensorSemiringKind {
    /// Exact linear Laurent accumulation `(+, *)` without intermediate float rounding.
    LinearExact,
    /// Tropical `(max, +)` semiring for bounding-volume support and shortest-path bounds.
    TropicalMaxPlus,
    /// Bounded integer audio accumulator ring for multi-emitter spatial mixing.
    ModularAudioRing,
}

/// Summary descriptor of a Prism-certified engine tensor kernel from `Tensor.lex.tex`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedTensorKernel {
    /// Canonical kernel index (`0..=6`).
    pub index: u64,
    /// Governing semiring.
    pub semiring: TensorSemiringKind,
    /// Inner contraction dimension `k`.
    pub lhs_cols: u64,
    /// Declared exact accumulator width in bits.
    pub accum_bits: u64,
    /// Minimum required accumulator width in bits.
    pub min_accum_bits: u64,
    /// Whether tile-aligned to `16x16` software raster tiles.
    pub tile_aligned: bool,
    /// Whether this kernel drives autonomous spline kinematics.
    pub autonomous_kinematic: bool,
    /// Kernel identifier.
    pub name: String,
    /// Engineering specification summary.
    pub summary: String,
}

fn map_semiring(sr: prism_kernel::TensorSemiring) -> TensorSemiringKind {
    match sr {
        prism_kernel::TensorSemiring::linearExact => TensorSemiringKind::LinearExact,
        prism_kernel::TensorSemiring::tropicalMaxPlus => TensorSemiringKind::TropicalMaxPlus,
        prism_kernel::TensorSemiring::modularAudioRing => TensorSemiringKind::ModularAudioRing,
    }
}

/// Validates a `prism_kernel::TensorKernelSpec` through the Lean-proved `tensorKernelValid` predicate.
pub fn verify_tensor_kernel_spec(
    spec: &prism_kernel::TensorKernelSpec,
) -> Result<VerifiedTensorKernel, TensorComputeError> {
    if !prism_kernel::tensorKernelValid(spec) {
        return Err(TensorComputeError::InvalidKernelSpecification {
            kernel_name: spec.name.clone(),
        });
    }
    Ok(VerifiedTensorKernel {
        index: spec.index,
        semiring: map_semiring(spec.semiring),
        lhs_cols: spec.lhsCols,
        accum_bits: spec.accumBits,
        min_accum_bits: spec.minAccumBits,
        tile_aligned: spec.tileAligned,
        autonomous_kinematic: spec.autonomousKinematic,
        name: spec.name.clone(),
        summary: spec.text.clone(),
    })
}

/// Returns all seven Prism-certified engine tensor kernels (`kernel0StaticMesh`..`kernel6KinematicSpline`).
pub fn verified_engine_tensor_kernels() -> Result<Vec<VerifiedTensorKernel>, TensorComputeError> {
    let specs = [
        prism_kernel::kernel0StaticMesh().expect("verified Prism kernel0StaticMesh"),
        prism_kernel::kernel1LinearSkinning().expect("verified Prism kernel1LinearSkinning"),
        prism_kernel::kernel2TropicalBvhCull().expect("verified Prism kernel2TropicalBvhCull"),
        prism_kernel::kernel3TileRasterizer().expect("verified Prism kernel3TileRasterizer"),
        prism_kernel::kernel4ShadowCascade().expect("verified Prism kernel4ShadowCascade"),
        prism_kernel::kernel5ModalAudioMix().expect("verified Prism kernel5ModalAudioMix"),
        prism_kernel::kernel6KinematicSpline().expect("verified Prism kernel6KinematicSpline"),
    ];
    specs.iter().map(verify_tensor_kernel_spec).collect()
}

/// Transforms a batch of 3D positions `N x 4` by a `4 x 4` homogeneous matrix (`Mat4`) using
/// `uor_matmul::slice::gemm_float` so every inner product is accumulated without intermediate
/// floating-point rounding.
pub fn batch_transform_positions_exact(
    positions: &[[f32; DIM_SPATIAL]],
    matrix: &Mat4,
) -> Result<Vec<[f32; DIM_HOMOGENEOUS]>, TensorComputeError> {
    if positions.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "vertex batch must be non-empty".to_string(),
        });
    }
    for pos in positions {
        if !pos.iter().all(|c| c.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry {
                operand: "positions",
            });
        }
    }
    for col in &matrix.cols {
        if !col.iter().all(|c| c.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry { operand: "matrix" });
        }
    }

    let m = positions.len();
    let k = DIM_HOMOGENEOUS;
    let n = DIM_HOMOGENEOUS;
    let i2 = 1usize + 1usize;

    let i3 = i2 + 1usize;
    let mut out = Vec::with_capacity(m);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut a = Vec::with_capacity(m * k);
        for p in positions {
            a.push(p[0]);
            a.push(p[1]);
            a.push(p[i2]);
            a.push(1.0_f32);
        }
        let mut b = vec![0.0_f32; k * n];
        for r in 0..k {
            for c in 0..n {
                b[r * n + c] = matrix.cols[r][c] as f32;
            }
        }
        let mut c_flat = vec![0.0_f32; m * n];
        slice::gemm_float::<f32, f32>(m, k, n, &a, &b, &mut c_flat, &mut [], &mut []).map_err(
            |err| TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!("{err:?}"),
            },
        )?;
        for chunk in c_flat.chunks_exact(DIM_HOMOGENEOUS) {
            out.push([chunk[0], chunk[1], chunk[i2], chunk[i3]]);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let shift = prism_kernel::tileEdgePixels().expect("verified Prism tileEdgePixels") as u32;
        let q_scale = pow2_u64(shift) as f64;
        let inv_q2 = 1.0_f64 / (q_scale * q_scale);
        let one_q = (q_scale as i32).max(1);
        let mut a_q = Vec::with_capacity(m * k);
        for p in positions {
            a_q.push((f64::from(p[0]) * q_scale).round() as i32);
            a_q.push((f64::from(p[1]) * q_scale).round() as i32);
            a_q.push((f64::from(p[i2]) * q_scale).round() as i32);
            a_q.push(one_q);
        }
        let mut b_q = vec![0_i32; k * n];
        for r in 0..k {
            for c in 0..n {
                b_q[r * n + c] = (matrix.cols[r][c] * q_scale).round() as i32;
            }
        }
        let mut c_q = vec![0_i64; m * n];
        let mut scratch = vec![0_i32; suggested_scratch(Shape { m, k, n })];
        slice::gemm(m, k, n, &a_q, &b_q, &mut c_q, &mut scratch).map_err(|err| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: format!("{err:?}"),
            }
        })?;
        for chunk in c_q.chunks_exact(DIM_HOMOGENEOUS) {
            out.push([
                ((chunk[0] as f64) * inv_q2) as f32,
                ((chunk[1] as f64) * inv_q2) as f32,
                ((chunk[i2] as f64) * inv_q2) as f32,
                ((chunk[i3] as f64) * inv_q2) as f32,
            ]);
        }
    }

    Ok(out)
}

/// Evaluates exact tropical `(max, +)` support bounds `C[p, v] = max_{axis} (plane[p, axis] + vertex[v, axis])`
/// across `plane_axes_milli` (`P x 3`) and `hull_vertices_milli` (`V x 3`) via `uor_matmul`.
pub fn tropical_frustum_distance_bounds(
    plane_axes_milli: &[[i32; DIM_SPATIAL]],
    hull_vertices_milli: &[[i32; DIM_SPATIAL]],
) -> Result<Vec<i64>, TensorComputeError> {
    if plane_axes_milli.is_empty() || hull_vertices_milli.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "tropical plane and hull vertex slices must be non-empty".to_string(),
        });
    }

    let m = plane_axes_milli.len();
    let k = DIM_SPATIAL;
    let n = hull_vertices_milli.len();
    let i2 = 1usize + 1usize;

    let mut a_trop = Vec::with_capacity(m * k);
    for p in plane_axes_milli {
        a_trop.push(Trop::finite(p[0]));
        a_trop.push(Trop::finite(p[1]));
        a_trop.push(Trop::finite(p[i2]));
    }

    // Transpose `hull_vertices_milli` (`n x 3`) into row-major `3 x n`:
    let mut b_trop = vec![Trop::<i32>::NEG_INF; k * n];
    for (v_idx, v) in hull_vertices_milli.iter().enumerate() {
        b_trop[v_idx] = Trop::finite(v[0]);
        b_trop[n + v_idx] = Trop::finite(v[1]);
        b_trop[i2 * n + v_idx] = Trop::finite(v[i2]);
    }

    let mut c_trop = vec![Trop::<i64>::NEG_INF; m * n];
    let av = MatView::row_major(as_alphabet_tropical(&a_trop), m, k).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major tropical plane matrix view".to_string(),
        }
    })?;
    let bv = MatView::row_major(as_alphabet_tropical(&b_trop), k, n).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major tropical hull matrix view".to_string(),
        }
    })?;
    let cv = MatViewMut::row_major(&mut c_trop, m, n).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major tropical output matrix view".to_string(),
        }
    })?;
    let mut triple =
        Triple::new(av, bv, cv).map_err(|err| TensorComputeError::ZeroOrMismatchedExtents {
            detail: format!("{err:?}"),
        })?;

    uor_matmul::driver::gemm(
        &mut triple,
        &MaxPlus::OVERWRITE,
        GemmOptions::default(),
        &mut Scratch::none(),
    );

    Ok(c_trop
        .into_iter()
        .map(|entry| entry.get().unwrap_or(i64::MIN))
        .collect())
}

/// Mixes `emitter_count` Q15 audio sources across `frame_count` audio samples into `channel_count`
/// output channels via exact integer `uor_matmul::slice::gemm::<i16, i64>`.
pub fn mix_spatial_audio_channels_exact(
    frame_count: usize,
    emitter_count: usize,
    channel_count: usize,
    emitter_frames_q15: &[i16],
    pan_gains_q15: &[i16],
) -> Result<Vec<i64>, TensorComputeError> {
    if frame_count == 0 || emitter_count == 0 || channel_count == 0 {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "audio frame, emitter, and channel counts must all be positive".to_string(),
        });
    }
    if emitter_frames_q15.len() != frame_count.saturating_mul(emitter_count)
        || pan_gains_q15.len() != emitter_count.saturating_mul(channel_count)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "audio emitter or pan gain buffer length does not match declared shape"
                .to_string(),
        });
    }

    let mut out = vec![0_i64; frame_count * channel_count];
    let mut scratch = vec![
        0_i16;
        suggested_scratch(Shape {
            m: frame_count,
            k: emitter_count,
            n: channel_count,
        })
    ];
    slice::gemm(
        frame_count,
        emitter_count,
        channel_count,
        emitter_frames_q15,
        pan_gains_q15,
        &mut out,
        &mut scratch,
    )
    .map_err(|err| TensorComputeError::ZeroOrMismatchedExtents {
        detail: format!("{err:?}"),
    })?;
    Ok(out)
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

/// Renders world-space `triangles` into a portable `RGBA8` software tile framebuffer using
/// `batch_transform_positions_exact` (`uor-matmul`) for clip-space projection and `prism_kernel`
/// LOD atmosphere parameters, requiring zero conditional browser GPU extensions.
pub fn render_triangles_tile_framebuffer(
    width: u32,
    height: u32,
    view_proj: &Mat4,
    sun_direction: Vec3,
    lod_rank: u64,
    triangles: &[RasterTriangle],
) -> Result<SoftwareTileFramebuffer, TensorComputeError> {
    let tile_edge = prism_kernel::tileEdgePixels().expect("verified Prism tileEdgePixels") as u32;
    if width == 0 || height == 0 || !width.is_multiple_of(tile_edge) || !height.is_multiple_of(tile_edge) {
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
    let clip_verts = batch_transform_positions_exact(&flat_positions, view_proj)?;

    let mut depth_buf = depth;
    let mut rasterized_triangles = 0_u32;
    let sun = sun_direction.normalized();
    let half = half_f64() as f32;
    let two = two_f64() as f32;
    let eps = geometric_epsilon() as f32;
    let w_f = width as f32;
    let h_f = height as f32;

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
        let nrm = Vec3::new(
            f64::from(tri.normal[0]),
            f64::from(tri.normal[1]),
            f64::from(tri.normal[i2]),
        )
        .normalized();
        let ndotl = (nrm.dot(sun).max(0.0)) as f32;
        let hemi = (nrm.y * f64::from(half) + f64::from(half)).clamp(0.0, 1.0) as f32;
        let ambient = half * half + half * half * hemi;
        let shade = (ambient + (1.0 - half * half) * ndotl).min(two);

        let lit_r = ((tri.base_color[0] * shade) / (1.0 + tri.base_color[0] * shade) * two)
            .clamp(0.0, 1.0);
        let lit_g = ((tri.base_color[1] * shade) / (1.0 + tri.base_color[1] * shade) * two)
            .clamp(0.0, 1.0);
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
