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

use crate::{TensorComputeError, CODED_TABLE_LEN, PACKED_NIBBLE_BLOCK};

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
