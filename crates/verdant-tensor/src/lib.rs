//! **Verdant Tensor (`verdant-tensor`)** — Pure Prism-mechanized semiring tensor-logic substrate.
//!
//! Unifies engine execution into deterministic, zero-magic-constant semiring contractions over
//! `uor-matmul` and `uor_matmul::codec`, governed by Lean-proved accumulator, tile-alignment, and
//! zero-scratch predicates compiled from `Tensor.lex.tex` into [`prism_kernel`]:
//!
//! - [`linear`]: Exact Laurent `(+, *)` contractions for homogeneous vertex batches, topological
//!   scene/skeleton hierarchy propagation, 4-joint linear-blend skinning, and Delassus constraint
//!   effective-mass matrices (`J * M^-1 * J^T`).
//! - [`coded`]: Zero-decode `uor_matmul::codec` contractions (`Grid`, `Packed`, `Runs`) streaming
//!   quantized geometry, 4-bit nibble animation curves, and sparse IBL probe harmonics directly
//!   through `coded_gemm` without intermediate float expansion.
//! - [`tropical`]: Tropical `(max, +)` and `(min, +)` contractions for batch frustum culling,
//!   GJK directional support maps, and NavMesh all-pairs shortest-path matrix squaring.
//! - [`audio_ring`]: Quantized `Q15` integer ring contractions for multi-emitter Woodworth panning,
//!   Toeplitz modal FIR convolution, and 4-channel Hadamard Sabine/Eyring FDN reverberation.
//! - [`raster`]: Portable `16x16` tile-binned depth-buffered software PBR rasterizer.

#![forbid(unsafe_code)]

use std::fmt;

use serde::{Deserialize, Serialize};

/// Planar coordinate dimension (`1 + 1`).
pub const DIM_PLANAR: usize = 1 + 1;
/// Spatial coordinate and triangle vertex arity (`1 + 1 + 1`).
pub const DIM_SPATIAL: usize = 1 + 1 + 1;
/// Homogeneous affine vector and RGBA channel dimension (`(1 + 1) * (1 + 1)`).
pub const DIM_HOMOGENEOUS: usize = (1 + 1) * (1 + 1);
/// Canonical `Grid` lookup codebook cardinality (`2^4 = 16`).
pub const CODED_TABLE_LEN: usize = 1 << ((1 + 1) * (1 + 1));
/// Sub-byte `Packed` nibble arity (`2` four-bit codes per byte).
pub const PACKED_NIBBLE_BLOCK: usize = 1 + 1;

/// Constructive power of two (`2^exp`) computed purely from `{0, 1}`.
#[must_use]
pub const fn pow2_u64(exp: u32) -> u64 {
    let mut acc = 1_u64;
    let mut i = 0_u32;
    while i < exp {
        acc <<= 1;
        i += 1;
    }
    acc
}

/// Double-precision `2.0` (`1.0 + 1.0`).
#[must_use]
pub fn two_f64() -> f64 {
    1.0 + 1.0
}

/// Double-precision `0.5` (`1.0 / (1.0 + 1.0)`).
#[must_use]
pub fn half_f64() -> f64 {
    1.0 / two_f64()
}

/// Numerical degeneracy tolerance `2^-40` derived constructively from `{0, 1}`.
#[must_use]
pub fn geometric_epsilon() -> f64 {
    let forty = (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32) * (1_u32 << (1_u32 + 1_u32 + 1_u32));
    1.0 / (pow2_u64(forty) as f64)
}

#[doc(hidden)]
#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    unused_parens,
    unused_variables,
    clippy::all
)]
pub mod prism_kernel;

pub mod audio_ring;
pub mod coded;
pub mod linear;
pub mod neural;
pub mod raster;
pub mod tropical;

/// Refusal returned when a Prism-hosted `uor-matmul` tensor contraction or software tile rasterizer fails bounds checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TensorComputeError {
    /// Matrix, coded stream, or batch dimensions are zero or non-conformant (`NotAProduct` or buffer length mismatch).
    ZeroOrMismatchedExtents {
        /// Diagnostic shape description.
        detail: String,
    },
    /// Input matrix or camera transform contains `NaN` or infinite IEEE-754 entries.
    NonFiniteTensorEntry {
        /// Diagnostic label of the offending operand.
        operand: &'static str,
    },
    /// Declared tensor kernel or coded stream tier failed `prism_kernel` accumulator, zero-scratch, or tile alignment bounds.
    InvalidKernelSpecification {
        /// Kernel or tier name.
        kernel_name: String,
    },
    /// Software tile framebuffer dimensions are zero or not aligned to `prism_kernel::tileEdgePixels()`.
    InvalidTileDimensions {
        /// Requested width in pixels.
        width: u32,
        /// Requested height in pixels.
        height: u32,
        /// Required tile multiple in pixels.
        tile_edge: u32,
    },
}

impl fmt::Display for TensorComputeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroOrMismatchedExtents { detail } => {
                write!(f, "tensor contraction extents invalid: {detail}")
            }
            Self::NonFiniteTensorEntry { operand } => {
                write!(f, "tensor operand `{operand}` contains non-finite entries")
            }
            Self::InvalidKernelSpecification { kernel_name } => {
                write!(
                    f,
                    "tensor kernel `{kernel_name}` failed Prism accumulator or tile alignment verification"
                )
            }
            Self::InvalidTileDimensions {
                width,
                height,
                tile_edge,
            } => write!(
                f,
                "tile framebuffer {width}x{height} must be positive and aligned to {tile_edge}px tiles"
            ),
        }
    }
}

impl std::error::Error for TensorComputeError {}

/// Algebraic semiring classification of a verified Prism engine tensor kernel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TensorSemiringKind {
    /// Exact linear Laurent accumulation `(+, *)` without intermediate float rounding.
    LinearExact,
    /// Tropical `(max, +)` semiring for bounding-volume culling and GJK directional support.
    TropicalMaxPlus,
    /// Tropical `(min, +)` semiring for NavMesh all-pairs and corridor shortest-path closure.
    TropicalMinPlus,
    /// Bounded integer audio accumulator ring for multi-emitter FIR, panning, and FDN reverb.
    ModularAudioRing,
    /// Zero-decode `uor_matmul::codec` streaming contraction over `Grid`, `Packed`, and `Runs`.
    CodedZeroDecode,
}

/// Zero-decode `uor_matmul::codec` compression tier classification from `Tensor.lex.tex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodedMatrixTierKind {
    /// Fixed-width `Grid` codebook lookup (`16` entries).
    GridLookup,
    /// Sub-byte `Packed<Grid, 2>` nibble stream (`2` samples per byte).
    PackedNibble,
    /// Variable-length `Runs<Grid, 16>` sparse run-length stream.
    SparseRuns,
}

/// Summary descriptor of a Prism-certified engine tensor kernel from `Tensor.lex.tex`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedTensorKernel {
    /// Canonical kernel index (`0..=8`).
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

/// Summary descriptor of a Prism-certified `uor_matmul::codec` zero-decode stream tier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedCodedTier {
    /// Canonical tier index (`0..=2`).
    pub index: u64,
    /// Codec representation tier.
    pub tier: CodedMatrixTierKind,
    /// Code word width in bits.
    pub code_bits: u64,
    /// Elements decoded per code word (`MAX_BLOCK`).
    pub block_len: u64,
    /// Quantization codebook table length.
    pub table_size: u64,
    /// Exact accumulator bit-width.
    pub accum_bits: u64,
    /// Proof witness that streaming contraction requires zero intermediate float scratch.
    pub zero_scratch: bool,
    /// Tier identifier.
    pub name: String,
    /// Engineering specification summary.
    pub summary: String,
}

fn map_semiring(sr: prism_kernel::TensorSemiring) -> TensorSemiringKind {
    match sr {
        prism_kernel::TensorSemiring::linearExact => TensorSemiringKind::LinearExact,
        prism_kernel::TensorSemiring::tropicalMaxPlus => TensorSemiringKind::TropicalMaxPlus,
        prism_kernel::TensorSemiring::tropicalMinPlus => TensorSemiringKind::TropicalMinPlus,
        prism_kernel::TensorSemiring::modularAudioRing => TensorSemiringKind::ModularAudioRing,
        prism_kernel::TensorSemiring::codedZeroDecode => TensorSemiringKind::CodedZeroDecode,
    }
}

fn map_coded_tier(tier: prism_kernel::CodedMatrixTier) -> CodedMatrixTierKind {
    match tier {
        prism_kernel::CodedMatrixTier::gridLookup => CodedMatrixTierKind::GridLookup,
        prism_kernel::CodedMatrixTier::packedNibble => CodedMatrixTierKind::PackedNibble,
        prism_kernel::CodedMatrixTier::sparseRuns => CodedMatrixTierKind::SparseRuns,
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

/// Returns the seven canonical base engine tensor kernels (`kernel0StaticMesh`..`kernel6KinematicSpline`).
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

/// Returns all fifteen unified Prism-certified semiring tensor kernels (`kernel0`..`kernel14`).
pub fn verified_unified_tensor_kernels() -> Result<Vec<VerifiedTensorKernel>, TensorComputeError> {
    let mut kernels = verified_engine_tensor_kernels()?;
    let extra = [
        prism_kernel::kernel7TropicalMinPlusNav()
            .expect("verified Prism kernel7TropicalMinPlusNav"),
        prism_kernel::kernel8CodedStreamGemm().expect("verified Prism kernel8CodedStreamGemm"),
        prism_kernel::kernel9TropicalUiLayout().expect("verified Prism kernel9TropicalUiLayout"),
        prism_kernel::kernel10NeuromorphicSalienceSsm()
            .expect("verified Prism kernel10NeuromorphicSalienceSsm"),
        prism_kernel::kernel11CodedSlmReasoning()
            .expect("verified Prism kernel11CodedSlmReasoning"),
        prism_kernel::kernel12TropicalBlasSweep()
            .expect("verified Prism kernel12TropicalBlasSweep"),
        prism_kernel::kernel13TwoBoneIkContraction()
            .expect("verified Prism kernel13TwoBoneIkContraction"),
        prism_kernel::kernel14ConservationAutomaton()
            .expect("verified Prism kernel14ConservationAutomaton"),
    ];
    for spec in &extra {
        kernels.push(verify_tensor_kernel_spec(spec)?);
    }
    Ok(kernels)
}

/// Validates a `prism_kernel::CodedStreamSpec` through the Lean-proved `codedTierValid` predicate.
pub fn verify_coded_tier_spec(
    spec: &prism_kernel::CodedStreamSpec,
) -> Result<VerifiedCodedTier, TensorComputeError> {
    if !prism_kernel::codedTierValid(spec).unwrap_or(false) {
        return Err(TensorComputeError::InvalidKernelSpecification {
            kernel_name: spec.name.clone(),
        });
    }
    Ok(VerifiedCodedTier {
        index: spec.index,
        tier: map_coded_tier(spec.tier),
        code_bits: spec.codeBits,
        block_len: spec.blockLen,
        table_size: spec.tableSize,
        accum_bits: spec.accumBits,
        zero_scratch: spec.zeroScratch,
        name: spec.name.clone(),
        summary: spec.text.clone(),
    })
}

/// Returns all three Prism-certified `uor_matmul::codec` zero-decode stream tiers (`Grid`, `Packed`, `Runs`).
pub fn verified_coded_matrix_tiers() -> Result<Vec<VerifiedCodedTier>, TensorComputeError> {
    let specs = [
        prism_kernel::codedTier0GridGeometry().expect("verified Prism codedTier0GridGeometry"),
        prism_kernel::codedTier1PackedAnimation()
            .expect("verified Prism codedTier1PackedAnimation"),
        prism_kernel::codedTier2RunsIblProbes().expect("verified Prism codedTier2RunsIblProbes"),
    ];
    specs.iter().map(verify_coded_tier_spec).collect()
}

pub use audio_ring::{
    convolve_modal_fir_exact, mix_spatial_audio_channels_exact, step_sabine_fdn_reverb_ring,
};
pub use coded::{
    contract_coded_geometry_grid, contract_packed_animation_tracks, contract_runs_ibl_harmonics,
    prism_coded_tiers_certified,
};
pub use linear::{
    batch_transform_positions_exact, delassus_effective_mass_matrix,
    propagate_hierarchy_matrices_exact, skin_vertices_linear_blend_exact, solve_two_bone_ik_exact,
    TwoBoneIkSolution,
};
pub use neural::{
    infer_coded_slm_intent_logits, step_neuromorphic_lif_ssm_ring,
    verify_conservation_and_reachability, ConservationAuditReport, NeuromorphicStepOutput,
};
pub use raster::{render_triangles_tile_framebuffer, RasterTriangle, SoftwareTileFramebuffer};
pub use tropical::{
    tropical_frustum_distance_bounds, tropical_gjk_support_batch,
    tropical_min_plus_all_pairs_closure, tropical_swept_aabb_toi_bounds, tropical_ui_box_extents,
};
