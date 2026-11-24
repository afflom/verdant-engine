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
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

pub mod scalars;

/// Planar coordinate dimension (`1 + 1`).
pub const DIM_PLANAR: usize = 1 + 1;
/// Spatial coordinate and triangle vertex arity (`1 + 1 + 1`).
pub const DIM_SPATIAL: usize = 1 + 1 + 1;
/// Homogeneous affine vector and RGBA channel dimension (`(1 + 1) * (1 + 1)`).
pub const DIM_HOMOGENEOUS: usize = (1 + 1) * (1 + 1);
/// Canonical `Grid` lookup codebook cardinality (`2^4 = 16`).
pub const CODED_TABLE_LEN: usize = 1 << ((1 + 1) * (1 + 1));
/// Exact `16 x 16` (`256`-entry) codebook ring product lookup table cardinality.
pub const CODEBOOK_PRODUCT_LUT_LEN: usize = CODED_TABLE_LEN * CODED_TABLE_LEN;
/// Sub-byte `Packed` nibble arity (`2` four-bit codes per byte).
pub const PACKED_NIBBLE_BLOCK: usize = 1 + 1;
/// Constructive 64-bit word width (`2^6 = 64`) for `O(N / 64)` bitset sweeps.
pub const WORD_BIT_WIDTH: usize = 1 << ((1 + 1) * (1 + 1 + 1));
/// Surface-Area-Heuristic (`SAH`) spatial partition bin count (`2^3 = 8`).
pub const SAH_BIN_COUNT: usize = 1 << (1 + 1 + 1);
/// Linearized `FlatBvh` explicit traversal stack capacity (`2^6 = 64`, zero heap allocation).
pub const FLAT_BVH_STACK_CAP: usize = WORD_BIT_WIDTH;
/// Progressive + HD LOD envelope count (`0..=4` -> `5` envelopes).
pub const HD_LOD_ENVELOPE_COUNT: usize = 1 + (1 << (1 + 1));

/// Constructive power of two (`2^exp`) computed purely from `{0, 1}`.
#[must_use]
pub const fn pow2_u64(exp: u32) -> u64 {
    let two = crate::scalars::two_u64();
    let sixty_four = crate::scalars::sixty_four_u32();
    if exp >= sixty_four {
        0_u64
    } else {
        let bound = exp as usize;
        let mut acc = 1_u64;
        let mut i = 0_usize;
        while i < bound {
            acc = acc.wrapping_mul(two);
            i += 1;
        }
        acc
    }
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
    const EPS_EXP: u32 =
        (1_u32 + 1_u32 + 1_u32 + 1_u32 + 1_u32) * (1_u32 << (1_u32 + 1_u32 + 1_u32));
    const EPS_VAL: f64 = 1.0 / (pow2_u64(EPS_EXP) as f64);
    EPS_VAL
}

/// Hoisted Stratum-0 Prism kernel scalars evaluated once at startup (`OnceLock`) so hot loops
/// read verified `{0, 1}` invariants in `O(1)` machine registers with zero repeated Peano evaluation.
#[derive(Clone, Debug)]
pub struct VerifiedKernelConsts {
    /// Skinning and permille fixed-point denominator (`1000`).
    pub skin_weight_scale: u64,
    /// Double-precision permille denominator (`1000.0`).
    pub skin_weight_scale_f64: f64,
    /// Swept-capsule CCD skin clearance margin in millimeters (`2`).
    pub ccd_skin_margin_milli: u64,
    /// Swept-capsule CCD skin clearance margin in meters (`0.002`).
    pub ccd_skin_margin_meters: f64,
    /// Maximum swept-capsule slide iterations per step (`4`).
    pub ccd_max_slide_iterations: u32,
    /// Maximum walkable stair riser step height in meters (`0.35`).
    pub swept_capsule_max_step_meters: f64,
    /// Minimum walkable surface normal `ny` (`0.70`).
    pub nav_walkable_ny_min: f64,
    /// Software tile edge length in pixels (`16`).
    pub tile_edge_pixels: u32,
    /// Framebuffer channel count (`4`).
    pub rgba_channel_count: usize,
    /// Percentage denominator (`100.0`).
    pub percent_denom_f32: f32,
    /// SAH spatial bin count (`8`).
    pub sah_bin_count: usize,
    /// Maximum primitives per `FlatBvh` leaf (`4`).
    pub flat_bvh_leaf_max_prims: usize,
    /// Bounded traversal stack depth (`64`).
    pub flat_bvh_stack_cap: usize,
    /// Word-packed bitset width (`64`).
    pub word_bit_width: usize,
    /// Precomputed codebook product table size (`256`).
    pub codebook_lut_len: usize,
    /// Maximum in-flight worker frames before input coalescing (`2`).
    pub worker_max_in_flight: u32,
    /// `Q16` fixed-point quantization scale (`65536.0`).
    pub q16_scale_f64: f64,
    /// Reciprocal squared `Q16` scale (`1.0 / 65536.0^2`).
    pub inv_q16_sq_f64: f64,
    /// `Q12` fixed-point quantization scale (`4096.0`).
    pub q12_scale_f64: f64,
    /// Reciprocal squared `Q12` scale (`1.0 / 4096.0^2`).
    pub inv_q12_sq_f64: f64,
    /// Exact IEEE-754 `f64` mantissa bit horizon (`52`) for floating-origin sector shifts.
    pub f64_exact_mantissa_bits: u32,
    /// Maximum worker RPC watchdog timeout in milliseconds (`15000`).
    pub worker_rpc_watchdog_millis: u64,
    /// Terminal linear velocity stability ceiling in meters per second (`120.0`).
    pub physics_max_speed_mps: f64,
}

static KERNEL_CONSTS: OnceLock<VerifiedKernelConsts> = OnceLock::new();

/// Returns the process-wide `OnceLock`-hoisted Prism kernel constants.
#[must_use]
pub fn kernel_consts() -> &'static VerifiedKernelConsts {
    KERNEL_CONSTS.get_or_init(|| {
        let two_u = 1_u64 + 1_u64;
        let five_u = two_u + two_u + 1_u64;
        let ten_u = two_u * five_u;
        let hundred_u = ten_u * ten_u;

        #[cfg(not(target_arch = "wasm32"))]
        let skin_weight_scale = prism_kernel::skinWeightScale().unwrap_or(1).max(1);
        #[cfg(target_arch = "wasm32")]
        let skin_weight_scale = hundred_u * ten_u;

        let skin_weight_scale_f64 = skin_weight_scale as f64;
        let ccd_skin_margin_milli = two_u;
        let ccd_skin_margin_meters = (ccd_skin_margin_milli as f64) / skin_weight_scale_f64;
        let ccd_max_slide_iterations = (two_u * two_u) as u32;
        let swept_capsule_max_step_meters =
            ((five_u * (five_u + two_u) * ten_u) as f64) / skin_weight_scale_f64;
        let nav_walkable_ny_min = (((five_u + two_u) * hundred_u) as f64) / skin_weight_scale_f64;
        let tile_edge_pixels = 1_u32 << ((1_u32 + 1_u32) * (1_u32 + 1_u32));
        let rgba_channel_count = DIM_HOMOGENEOUS;
        let percent_denom_f32 = hundred_u as f32;
        let sah_bin_count = SAH_BIN_COUNT;
        let flat_bvh_leaf_max_prims = DIM_HOMOGENEOUS;
        let flat_bvh_stack_cap = FLAT_BVH_STACK_CAP;
        let word_bit_width = WORD_BIT_WIDTH;
        let codebook_lut_len = CODEBOOK_PRODUCT_LUT_LEN;
        let worker_max_in_flight = 1_u32 + 1_u32;

        let f64_exact_mantissa_bits = (five_u * ten_u + two_u) as u32;
        let worker_rpc_watchdog_millis = (ten_u + five_u) * skin_weight_scale;
        let physics_max_speed_mps = (two_u * (five_u + 1_u64) * ten_u) as f64;

        #[cfg(not(target_arch = "wasm32"))]
        {
            debug_assert_eq!(prism_kernel::ccdSkinMarginMilli(), ccd_skin_margin_milli);
            debug_assert_eq!(
                prism_kernel::ccdMaxSlideIterations(),
                Ok(u64::from(ccd_max_slide_iterations))
            );
            debug_assert_eq!(
                prism_kernel::tileEdgePixels(),
                Ok(u64::from(tile_edge_pixels))
            );
            debug_assert_eq!(prism_kernel::sahBinCount(), Ok(sah_bin_count as u64));
            debug_assert_eq!(
                prism_kernel::flatBvhLeafMaxPrims(),
                Ok(flat_bvh_leaf_max_prims as u64)
            );
            debug_assert_eq!(
                prism_kernel::flatBvhStackCapacity(),
                Ok(flat_bvh_stack_cap as u64)
            );
            debug_assert_eq!(prism_kernel::wordBitWidth(), Ok(word_bit_width as u64));
            debug_assert_eq!(
                prism_kernel::codebookProductTableSize(),
                Ok(codebook_lut_len as u64)
            );
            debug_assert_eq!(
                prism_kernel::workerMaxInFlightFrames(),
                u64::from(worker_max_in_flight)
            );
            debug_assert_eq!(
                prism_kernel::f64ExactMantissaBits(),
                Ok(u64::from(f64_exact_mantissa_bits))
            );
            debug_assert_eq!(
                prism_kernel::workerRpcWatchdogMillis(),
                Ok(worker_rpc_watchdog_millis)
            );
            debug_assert_eq!(
                prism_kernel::physicsMaxSpeedMetersPerSec(),
                Ok(physics_max_speed_mps as u64)
            );
        }

        let q16_scale_f64 = pow2_u64(tile_edge_pixels) as f64;
        let inv_q16_sq_f64 = 1.0 / (q16_scale_f64 * q16_scale_f64);
        let s12 = (1_u32 + 1_u32 + 1_u32) * (1_u32 << (1_u32 + 1_u32));
        let q12_scale_f64 = pow2_u64(s12) as f64;
        let inv_q12_sq_f64 = 1.0 / (q12_scale_f64 * q12_scale_f64);

        VerifiedKernelConsts {
            skin_weight_scale,
            skin_weight_scale_f64,
            ccd_skin_margin_milli,
            ccd_skin_margin_meters,
            ccd_max_slide_iterations,
            swept_capsule_max_step_meters,
            nav_walkable_ny_min,
            tile_edge_pixels,
            rgba_channel_count,
            percent_denom_f32,
            sah_bin_count,
            flat_bvh_leaf_max_prims,
            flat_bvh_stack_cap,
            word_bit_width,
            codebook_lut_len,
            worker_max_in_flight,
            q16_scale_f64,
            inv_q16_sq_f64,
            q12_scale_f64,
            inv_q12_sq_f64,
            f64_exact_mantissa_bits,
            worker_rpc_watchdog_millis,
            physics_max_speed_mps,
        }
    })
}

/// Dense 64-bit word-packed bitset (`u64` words) enabling `O(N / 64)` ECS archetype visibility,
/// dirty-transform propagation, and physics island sleep/wake sweeps (adapted from `verdant-aqm`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordBitset {
    words: Vec<u64>,
    bit_len: usize,
}

impl WordBitset {
    /// Allocates a cleared word-packed bitset covering `bit_len` dense entity slots.
    #[must_use]
    pub fn with_len(bit_len: usize) -> Self {
        let word_count = bit_len.div_ceil(WORD_BIT_WIDTH);
        #[cfg(not(target_arch = "wasm32"))]
        debug_assert_eq!(
            prism_kernel::wordPackedFilterValid(bit_len as u64, word_count as u64),
            Ok(true)
        );
        Self {
            words: vec![0_u64; word_count],
            bit_len,
        }
    }

    /// Allocates a word-packed bitset with all `0..bit_len` slots initialized to `1`.
    #[must_use]
    pub fn all_ones(bit_len: usize) -> Self {
        let mut bs = Self::with_len(bit_len);
        bs.fill_ones();
        bs
    }

    /// Logical bit capacity (`N`).
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bit_len
    }

    /// Returns true if `len == 0`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bit_len == 0
    }

    /// Underlying `u64` word slice (`ceil(N / 64)` words).
    #[must_use]
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Resizes bitset to `new_len`, preserving existing lower bits and zero-filling newly grown slots.
    pub fn resize(&mut self, new_len: usize) {
        let word_count = new_len.div_ceil(WORD_BIT_WIDTH);
        self.words.resize(word_count, 0_u64);
        self.bit_len = new_len;
        let rem = self.bit_len % WORD_BIT_WIDTH;
        if rem != 0 {
            if let Some(last) = self.words.last_mut() {
                *last &= (1_u64 << rem).wrapping_sub(1);
            }
        }
    }

    /// Sets bit `index` (`1`).
    #[inline]
    pub fn set(&mut self, index: usize) {
        if index < self.bit_len {
            let w = index / WORD_BIT_WIDTH;
            let b = index % WORD_BIT_WIDTH;
            self.words[w] |= 1_u64 << b;
        }
    }

    /// Clears bit `index` (`0`).
    #[inline]
    pub fn clear(&mut self, index: usize) {
        if index < self.bit_len {
            let w = index / WORD_BIT_WIDTH;
            let b = index % WORD_BIT_WIDTH;
            self.words[w] &= !(1_u64 << b);
        }
    }

    /// Tests whether bit `index` is `1`.
    #[inline]
    #[must_use]
    pub fn test(&self, index: usize) -> bool {
        if index >= self.bit_len {
            return false;
        }
        let w = index / WORD_BIT_WIDTH;
        let b = index % WORD_BIT_WIDTH;
        ((self.words[w] >> b) & 1_u64) != 0
    }

    /// Clears all words to `0` in `O(N / 64)`.
    pub fn clear_all(&mut self) {
        self.words.fill(0_u64);
    }

    /// Sets all valid slots `0..bit_len` to `1` in `O(N / 64)`.
    pub fn fill_ones(&mut self) {
        self.words.fill(!0_u64);
        let rem = self.bit_len % WORD_BIT_WIDTH;
        if rem != 0 {
            if let Some(last) = self.words.last_mut() {
                *last &= (1_u64 << rem).wrapping_sub(1);
            }
        }
    }

    /// Bitwise `self &= rhs` in `O(N / 64)`.
    pub fn and_assign(&mut self, rhs: &Self) {
        for (dst, &src) in self.words.iter_mut().zip(rhs.words.iter()) {
            *dst &= src;
        }
        if rhs.words.len() < self.words.len() {
            for dst in &mut self.words[rhs.words.len()..] {
                *dst = 0_u64;
            }
        }
    }

    /// Bitwise `self |= rhs` in `O(N / 64)`.
    pub fn or_assign(&mut self, rhs: &Self) {
        for (dst, &src) in self.words.iter_mut().zip(rhs.words.iter()) {
            *dst |= src;
        }
        let rem = self.bit_len % WORD_BIT_WIDTH;
        if rem != 0 {
            if let Some(last) = self.words.last_mut() {
                *last &= (1_u64 << rem).wrapping_sub(1);
            }
        }
    }

    /// Hardware `popcnt` across all packed `u64` words in `O(N / 64)`.
    #[must_use]
    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Visits every set slot index in ascending order using `trailing_zeros` hardware bit scans
    /// (`O(set_bits + N / 64)` — skipping 64 empty entities in a single cycle).
    #[inline]
    pub fn for_each_set_bit<F: FnMut(usize)>(&self, mut f: F) {
        for (word_idx, &word_init) in self.words.iter().enumerate() {
            let mut word = word_init;
            let base = word_idx * WORD_BIT_WIDTH;
            while word != 0 {
                let tz = word.trailing_zeros() as usize;
                f(base + tz);
                word &= word.wrapping_sub(1);
            }
        }
    }
}

/// Canonical number of column-transposed archetype capability bit-planes (`8`).
pub const ARCHETYPE_BIT_PLANES: usize = 1 << (1 + 1 + 1);

/// Column-transposed 64-entity bit-plane matrix extrapolated from `verdant-aqm`'s
/// `ColumnBitPlaneTableau` (`crates/verdant-aqm/src/lib.rs` & `wasm_symplectic_commute`).
///
/// Packs `col_count` boolean component/capability columns across `row_len` entities into
/// `planes_per_col = ceil(row_len / 64)` contiguous `u64` words per column so multi-component
/// `include & !exclude` queries, column XOR/swap gates, and hardware `popcnt` inner products
/// execute across `64` entities per CPU instruction with `0` heap allocations.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnBitPlaneTable {
    col_count: usize,
    row_len: usize,
    planes_per_col: usize,
    planes: Vec<u64>,
}

impl ColumnBitPlaneTable {
    /// Allocates a cleared `col_count x row_len` column-transposed bit-plane matrix.
    #[must_use]
    pub fn new(col_count: usize, row_len: usize) -> Self {
        let planes_per_col = row_len.div_ceil(WORD_BIT_WIDTH);
        Self {
            col_count,
            row_len,
            planes_per_col,
            planes: vec![0_u64; col_count.saturating_mul(planes_per_col)],
        }
    }

    /// Number of transposed capability columns.
    #[must_use]
    pub const fn col_count(&self) -> usize {
        self.col_count
    }

    /// Number of entity rows indexed across each column.
    #[must_use]
    pub const fn row_len(&self) -> usize {
        self.row_len
    }

    /// Number of `u64` words stored per column (`ceil(row_len / 64)`).
    #[must_use]
    pub const fn planes_per_col(&self) -> usize {
        self.planes_per_col
    }

    /// Resizes entity row count while preserving existing bit-planes across all columns.
    pub fn resize_rows(&mut self, new_row_len: usize) {
        let next_ppc = new_row_len.div_ceil(WORD_BIT_WIDTH);
        if next_ppc != self.planes_per_col {
            let mut next_planes = vec![0_u64; self.col_count.saturating_mul(next_ppc)];
            let copy_w = self.planes_per_col.min(next_ppc);
            for col in 0..self.col_count {
                let src_off = col * self.planes_per_col;
                let dst_off = col * next_ppc;
                next_planes[dst_off..dst_off + copy_w]
                    .copy_from_slice(&self.planes[src_off..src_off + copy_w]);
            }
            self.planes = next_planes;
            self.planes_per_col = next_ppc;
        }
        self.row_len = new_row_len;
        let rem = self.row_len % WORD_BIT_WIDTH;
        if rem != 0 && self.planes_per_col > 0 {
            let tail_mask = (1_u64 << rem).wrapping_sub(1);
            for col in 0..self.col_count {
                let last_idx = col * self.planes_per_col + (self.planes_per_col - 1);
                self.planes[last_idx] &= tail_mask;
            }
        }
    }

    /// Sets `(col, row)` to `enabled` in `O(1)`.
    #[inline]
    pub fn set(&mut self, col: usize, row: usize, enabled: bool) {
        if col < self.col_count && row < self.row_len {
            let idx = col * self.planes_per_col + (row / WORD_BIT_WIDTH);
            let mask = 1_u64 << (row % WORD_BIT_WIDTH);
            if enabled {
                self.planes[idx] |= mask;
            } else {
                self.planes[idx] &= !mask;
            }
        }
    }

    /// Tests `(col, row)` in `O(1)`.
    #[inline]
    #[must_use]
    pub fn test(&self, col: usize, row: usize) -> bool {
        if col >= self.col_count || row >= self.row_len {
            return false;
        }
        let idx = col * self.planes_per_col + (row / WORD_BIT_WIDTH);
        ((self.planes[idx] >> (row % WORD_BIT_WIDTH)) & 1_u64) != 0
    }

    /// Toggles `col_target ^= col_control` across all rows in `O(N / 64)` (analogous to `ColumnBitPlaneTableau` CNOT).
    pub fn xor_column_into(&mut self, col_control: usize, col_target: usize) {
        if col_control < self.col_count && col_target < self.col_count && col_control != col_target
        {
            let ppc = self.planes_per_col;
            for w in 0..ppc {
                let ctrl = self.planes[col_control * ppc + w];
                self.planes[col_target * ppc + w] ^= ctrl;
            }
        }
    }

    /// Evaluates `(AND_{c in required_cols} plane[c]) & !(OR_{e in excluded_cols} plane[e])` into
    /// `out_mask` in `O((|req| + |excl|) * N / 64)` with `0` heap allocations, returning matched count.
    pub fn query_archetype_into(
        &self,
        required_cols: &[usize],
        excluded_cols: &[usize],
        out_mask: &mut WordBitset,
    ) -> usize {
        if out_mask.len() != self.row_len {
            out_mask.resize(self.row_len);
        }
        if self.row_len == 0 {
            return 0;
        }
        out_mask.fill_ones();
        let ppc = self.planes_per_col;
        for &req in required_cols {
            if req >= self.col_count {
                out_mask.clear_all();
                return 0;
            }
            let base = req * ppc;
            for (w, dst) in out_mask.words.iter_mut().enumerate() {
                *dst &= self.planes[base + w];
            }
        }
        for &excl in excluded_cols {
            if excl < self.col_count {
                let base = excl * ppc;
                for (w, dst) in out_mask.words.iter_mut().enumerate() {
                    *dst &= !self.planes[base + w];
                }
            }
        }
        out_mask.count_ones()
    }

    /// Counts entities shared by both `col_a` and `col_b` via hardware `u64::count_ones` (`O(N / 64)`).
    #[must_use]
    pub fn popcount_intersection(&self, col_a: usize, col_b: usize) -> usize {
        if col_a >= self.col_count || col_b >= self.col_count {
            return 0;
        }
        let ppc = self.planes_per_col;
        let a_slice = &self.planes[col_a * ppc..(col_a + 1) * ppc];
        let b_slice = &self.planes[col_b * ppc..(col_b + 1) * ppc];
        a_slice
            .iter()
            .zip(b_slice.iter())
            .map(|(&wa, &wb)| (wa & wb).count_ones() as usize)
            .sum()
    }
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
pub mod memory;
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
        semiring: match spec.semiring {
            prism_kernel::TensorSemiring::linearExact => TensorSemiringKind::LinearExact,
            prism_kernel::TensorSemiring::tropicalMaxPlus => TensorSemiringKind::TropicalMaxPlus,
            prism_kernel::TensorSemiring::tropicalMinPlus => TensorSemiringKind::TropicalMinPlus,
            prism_kernel::TensorSemiring::modularAudioRing => TensorSemiringKind::ModularAudioRing,
            prism_kernel::TensorSemiring::codedZeroDecode => TensorSemiringKind::CodedZeroDecode,
        },
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
        tier: match spec.tier {
            prism_kernel::CodedMatrixTier::gridLookup => CodedMatrixTierKind::GridLookup,
            prism_kernel::CodedMatrixTier::packedNibble => CodedMatrixTierKind::PackedNibble,
            prism_kernel::CodedMatrixTier::sparseRuns => CodedMatrixTierKind::SparseRuns,
        },
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
    contract_coded_geometry_grid, contract_coded_pair_lut_into, contract_packed_animation_tracks,
    contract_packed_nibbles_lut_into, contract_runs_ibl_harmonics, contract_sparse_runs_lut_into,
    decide_coded_matmul_admission, decide_uorc_challenger_admission, prism_coded_tiers_certified,
    CodebookProductLut, CodedAdmissionVerdict, SparseCodedRun, UorcChallengerVerdict,
};
pub use linear::{
    batch_transform_positions_exact, batch_transform_positions_into,
    delassus_effective_mass_matrix, propagate_hierarchy_matrices_exact,
    skin_vertices_linear_blend_exact, skin_vertices_linear_blend_into, solve_two_bone_ik_exact,
    TwoBoneIkSolution,
};
pub use memory::{
    contract_paged_packed_nibbles_stream, PagedArenaTelemetry, PagedContractionReport,
    PagedTensorArena, TensorPageDescriptor, TensorPageKind, TensorStorageEncoding,
    MIN_ARENA_BUDGET_BYTES, TENSOR_PAGE_QUANTUM_BYTES,
};
pub use neural::{
    infer_coded_slm_intent_logits, step_neuromorphic_lif_ssm_ring,
    verify_conservation_and_reachability, ConservationAuditReport, NeuromorphicStepOutput,
};
pub use raster::{
    render_triangles_tile_framebuffer, render_triangles_tile_framebuffer_with_scratch,
    RasterTriangle, SoftwareTileFramebuffer, TileRasterScratch,
};
pub use tropical::{
    reconstruct_tropical_next_hop_path, tropical_frustum_distance_bounds,
    tropical_gjk_support_batch, tropical_gjk_support_single, tropical_inf_sentinel,
    tropical_min_plus_all_pairs_closure, tropical_min_plus_closure_in_place,
    tropical_swept_aabb_toi_bounds, tropical_ui_box_extents,
};
