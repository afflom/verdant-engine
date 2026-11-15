//! Unified engine tensor-logic facade delegating to `verdant-tensor` (`uor-matmul` + `uor_matmul::codec`).
//!
//! Bridges `verdant-core` spatial types ([`Mat4`], [`Vec3`]) directly onto the pure Prism-mechanized
//! semiring substrate in `verdant-tensor` across linear Laurent `(+, *)`, zero-decode `CodedMatrix`
//! (`Grid`, `Packed`, `Runs`), tropical `(max, +)` / `(min, +)`, quantized audio rings, and
//! `16x16` tile-binned software PBR rasterization.

pub use verdant_tensor::{
    contract_coded_geometry_grid, contract_packed_animation_tracks, contract_runs_ibl_harmonics,
    convolve_modal_fir_exact, delassus_effective_mass_matrix, mix_spatial_audio_channels_exact,
    propagate_hierarchy_matrices_exact, skin_vertices_linear_blend_exact,
    step_sabine_fdn_reverb_ring, tropical_frustum_distance_bounds, tropical_gjk_support_batch,
    tropical_min_plus_all_pairs_closure, verified_coded_matrix_tiers,
    verified_engine_tensor_kernels, verified_unified_tensor_kernels, verify_coded_tier_spec,
    verify_tensor_kernel_spec, CodedMatrixTierKind, RasterTriangle, SoftwareTileFramebuffer,
    TensorSemiringKind, VerifiedCodedTier, VerifiedTensorKernel, CODED_TABLE_LEN,
    PACKED_NIBBLE_BLOCK,
};

use crate::error::TensorComputeError;
use crate::math::{Mat4, Vec3, DIM_HOMOGENEOUS, DIM_SPATIAL};

/// Transforms a batch of 3D positions `N x 4` by a `4 x 4` homogeneous [`Mat4`] using
/// `verdant_tensor::linear::batch_transform_positions_exact` (`uor-matmul`).
pub fn batch_transform_positions_exact(
    positions: &[[f32; DIM_SPATIAL]],
    matrix: &Mat4,
) -> Result<Vec<[f32; DIM_HOMOGENEOUS]>, TensorComputeError> {
    verdant_tensor::batch_transform_positions_exact(positions, &matrix.cols)
}

/// Renders world-space `triangles` into a portable `RGBA8` software tile framebuffer via
/// `verdant_tensor::raster::render_triangles_tile_framebuffer` (`uor-matmul`).
pub fn render_triangles_tile_framebuffer(
    width: u32,
    height: u32,
    view_proj: &Mat4,
    sun_direction: Vec3,
    lod_rank: u64,
    triangles: &[RasterTriangle],
) -> Result<SoftwareTileFramebuffer, TensorComputeError> {
    verdant_tensor::render_triangles_tile_framebuffer(
        width,
        height,
        &view_proj.cols,
        [sun_direction.x, sun_direction.y, sun_direction.z],
        lod_rank,
        triangles,
    )
}
