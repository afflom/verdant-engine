//! Zero-decode `uor_matmul::codec` streaming contractions (`Grid`, `Packed`, `Runs`).
//!
//! Pairs progressive `UORC` streams with `uor_matmul::CodedMatrix` and `uor_matmul::coded_gemm`
//! so quantized geometry (`Grid`), 4-bit nibble animation curves (`Packed<Grid, 2>`), and sparse
//! environment irradiance probes (`Runs<Grid, 16>`) contract directly into `i64` accumulators
//! without allocating or expanding intermediate float buffers.

use uor_matmul::{
    as_alphabet_full, coded_gemm, Alphabet, CodedMatrix, CodedTriple, Full, GemmOptions, Grid,
    Linear, MatView, MatViewMut, Packed, Runs,
};

use crate::{
    kernel_consts, prism_kernel, TensorComputeError, CODEBOOK_PRODUCT_LUT_LEN, CODED_TABLE_LEN,
    PACKED_NIBBLE_BLOCK,
};

/// Precomputed `16 x 16` (`256`-entry) exact ring product lookup table adapted from
/// `verdant-aqm` (`codedMatmul.ts`), enabling zero-decode, zero-allocation coded contractions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodebookProductLut {
    products: [i64; CODEBOOK_PRODUCT_LUT_LEN],
}

impl CodebookProductLut {
    /// Precomputes all `256` pairwise products `lhs_codebook[a] * rhs_codebook[b]` once outside inner loops.
    #[must_use]
    pub fn new(
        lhs_codebook: &[i32; CODED_TABLE_LEN],
        rhs_codebook: &[i32; CODED_TABLE_LEN],
    ) -> Self {
        debug_assert_eq!(kernel_consts().codebook_lut_len, CODEBOOK_PRODUCT_LUT_LEN);
        let mut products = [0_i64; CODEBOOK_PRODUCT_LUT_LEN];
        for (a, &lhs_val) in lhs_codebook.iter().enumerate() {
            let lhs = i64::from(lhs_val);
            let row_base = a * CODED_TABLE_LEN;
            for (b, &rhs_val) in rhs_codebook.iter().enumerate() {
                products[row_base + b] = lhs * i64::from(rhs_val);
            }
        }
        Self { products }
    }

    /// Looks up exact product `lhs[a] * rhs[b]` in `O(1)`.
    #[inline]
    #[must_use]
    pub const fn lookup(&self, a_code: u8, b_code: u8) -> i64 {
        let mask = CODED_TABLE_LEN - 1;
        let idx = ((a_code as usize) & mask) * CODED_TABLE_LEN + ((b_code as usize) & mask);
        self.products[idx]
    }
}

/// Zero-allocation coded matrix contraction writing `C[rows x cols]` directly into `out_accum`
/// using a precomputed [`CodebookProductLut`] (`0` heap allocations).
pub fn contract_coded_pair_lut_into(
    lhs_codes: &[u8],
    rhs_codes: &[u8],
    rows: usize,
    inner_k: usize,
    cols: usize,
    lut: &CodebookProductLut,
    out_accum: &mut [i64],
) -> Result<(), TensorComputeError> {
    if rows == 0
        || inner_k == 0
        || cols == 0
        || lhs_codes.len() != rows.saturating_mul(inner_k)
        || rhs_codes.len() != inner_k.saturating_mul(cols)
        || out_accum.len() != rows.saturating_mul(cols)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "CodebookProductLut contraction extents mismatch".to_string(),
        });
    }
    out_accum.fill(0_i64);
    for r in 0..rows {
        let a_row = &lhs_codes[r * inner_k..(r + 1) * inner_k];
        let out_row = &mut out_accum[r * cols..(r + 1) * cols];
        for (p, &a_code) in a_row.iter().enumerate() {
            let b_row = &rhs_codes[p * cols..(p + 1) * cols];
            for (c, &b_code) in b_row.iter().enumerate() {
                out_row[c] += lut.lookup(a_code, b_code);
            }
        }
    }
    Ok(())
}

/// Verifies that all three `uor_matmul::codec` zero-decode streaming tiers (`Grid`, `Packed`, `Runs`)
/// satisfy constructive bit-width, block-size, and zero-scratch invariants in `prism_kernel`.
#[must_use]
pub fn prism_coded_tiers_certified() -> bool {
    prism_kernel::codedTiersDirect() == Ok(true)
}

fn build_alphabet_table(
    codebook: &[i32; CODED_TABLE_LEN],
) -> [Alphabet<i32, Full<i32>>; CODED_TABLE_LEN] {
    core::array::from_fn(|idx| Alphabet::of(codebook[idx]))
}

/// Contracts dense `activations_q` (`rows x inner_k`) against a `Grid`-coded quantized geometry
/// matrix (`inner_k x cols`, `codes.len() == inner_k * cols`) via `uor_matmul::coded_gemm` with
/// zero intermediate float buffer expansion.
pub fn contract_coded_geometry_grid(
    activations_q: &[i32],
    rows: usize,
    inner_k: usize,
    cols: usize,
    codebook: &[i32; CODED_TABLE_LEN],
    codes: &[u16],
) -> Result<Vec<i64>, TensorComputeError> {
    if rows == 0
        || inner_k == 0
        || cols == 0
        || activations_q.len() != rows.saturating_mul(inner_k)
        || codes.len() != inner_k.saturating_mul(cols)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Grid CodedMatrix extents or code buffer length mismatch".to_string(),
        });
    }
    if codes.iter().any(|&c| usize::from(c) >= CODED_TABLE_LEN) {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Grid code index exceeds 16-entry codebook".to_string(),
        });
    }

    let table = build_alphabet_table(codebook);
    let grid = Grid::<i32, Full<i32>, CODED_TABLE_LEN>::new(&table);
    let coded_b = CodedMatrix::new(grid, inner_k, cols, codes).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Grid CodedMatrix::new rejected code layout".to_string(),
        }
    })?;

    let mut out = vec![0_i64; rows * cols];
    let av =
        MatView::row_major(as_alphabet_full(activations_q), rows, inner_k).ok_or_else(|| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "invalid row-major activation view".to_string(),
            }
        })?;
    let cv = MatViewMut::row_major(&mut out, rows, cols).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major output view".to_string(),
        }
    })?;
    let mut triple = CodedTriple::new(av, coded_b, cv).map_err(|_| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Grid CodedTriple shape mismatch".to_string(),
        }
    })?;

    coded_gemm(&mut triple, &Linear::OVERWRITE, GemmOptions::default());
    Ok(out)
}

/// Contracts pose blend weights (`rows x inner_k`) against a sub-byte `Packed<Grid<16>, 2>` nibble
/// animation track matrix (`inner_k x cols`, `2` four-bit track samples per byte) via `coded_gemm`.
pub fn contract_packed_animation_tracks(
    blend_weights_q: &[i32],
    rows: usize,
    inner_k: usize,
    cols: usize,
    codebook: &[i32; CODED_TABLE_LEN],
    packed_nibbles: &[u8],
) -> Result<Vec<i64>, TensorComputeError> {
    if rows == 0
        || inner_k == 0
        || cols == 0
        || !cols.is_multiple_of(PACKED_NIBBLE_BLOCK)
        || blend_weights_q.len() != rows.saturating_mul(inner_k)
        || packed_nibbles.len() != inner_k.saturating_mul(cols / PACKED_NIBBLE_BLOCK)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Packed nibble CodedMatrix requires even cols and inner_k * (cols / 2) bytes"
                .to_string(),
        });
    }

    let table = build_alphabet_table(codebook);
    let grid = Grid::<i32, Full<i32>, CODED_TABLE_LEN>::new(&table);
    let packed = Packed::<_, PACKED_NIBBLE_BLOCK>::new(grid).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Packed<Grid<16>, 2> construction failed".to_string(),
        }
    })?;
    let coded_b = CodedMatrix::new(packed, inner_k, cols, packed_nibbles).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Packed CodedMatrix::new rejected nibble stream".to_string(),
        }
    })?;

    let mut out = vec![0_i64; rows * cols];
    let av =
        MatView::row_major(as_alphabet_full(blend_weights_q), rows, inner_k).ok_or_else(|| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "invalid row-major blend weight view".to_string(),
            }
        })?;
    let cv = MatViewMut::row_major(&mut out, rows, cols).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major packed output view".to_string(),
        }
    })?;
    let mut triple = CodedTriple::new(av, coded_b, cv).map_err(|_| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Packed CodedTriple shape mismatch".to_string(),
        }
    })?;

    coded_gemm(&mut triple, &Linear::OVERWRITE, GemmOptions::default());
    Ok(out)
}

/// Contracts directional spherical-harmonic basis evaluations (`rows x inner_k`) against a
/// variable-length `Runs<Grid<16>, 16>` sparse IBL environment probe matrix (`inner_k x cols`)
/// via `uor_matmul::coded_gemm`.
pub fn contract_runs_ibl_harmonics(
    basis_eval_q: &[i32],
    rows: usize,
    inner_k: usize,
    cols: usize,
    codebook: &[i32; CODED_TABLE_LEN],
    run_dictionary: &[(u32, u16)],
    run_indices: &[u32],
) -> Result<Vec<i64>, TensorComputeError> {
    if rows == 0
        || inner_k == 0
        || cols == 0
        || basis_eval_q.len() != rows.saturating_mul(inner_k)
        || run_dictionary.is_empty()
        || run_indices.is_empty()
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Runs CodedMatrix requires non-empty basis, dictionary, and run indices"
                .to_string(),
        });
    }

    let table = build_alphabet_table(codebook);
    let grid = Grid::<i32, Full<i32>, CODED_TABLE_LEN>::new(&table);
    let sparse_runs: Runs<'_, i32, Full<i32>, _, CODED_TABLE_LEN> = Runs::new(grid, run_dictionary)
        .ok_or_else(|| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Runs::new rejected run lengths or codebook indices".to_string(),
        })?;
    let coded_b = CodedMatrix::new(sparse_runs, inner_k, cols, run_indices).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Runs CodedMatrix::new row run-lengths did not sum to cols".to_string(),
        }
    })?;

    let mut out = vec![0_i64; rows * cols];
    let av =
        MatView::row_major(as_alphabet_full(basis_eval_q), rows, inner_k).ok_or_else(|| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "invalid row-major SH basis view".to_string(),
            }
        })?;
    let cv = MatViewMut::row_major(&mut out, rows, cols).ok_or_else(|| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "invalid row-major Runs output view".to_string(),
        }
    })?;
    let mut triple = CodedTriple::new(av, coded_b, cv).map_err(|_| {
        TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Runs CodedTriple shape mismatch".to_string(),
        }
    })?;

    coded_gemm(&mut triple, &Linear::OVERWRITE, GemmOptions::default());
    Ok(out)
}
