//! Decayed rank-one state updates with scaled query projection.

use super::{exponential_f32, multiply_matrix_rows_f32, TensorComputeError};

/// Inputs for one time step of a decayed rank-one update and query projection.
#[derive(Clone, Copy)]
pub struct GatedDeltaInput<'a> {
    /// Query, key and value have the same positive width.
    pub query: &'a [f32],
    /// Vector used to predict the value and correct the projection matrix.
    pub key: &'a [f32],
    /// Observed value vector.
    pub value: &'a [f32],
    /// One log decay for the whole state, or one per input coordinate.
    pub log_decay: &'a [f32],
    /// Gain applied to each prediction residual.
    pub gain: f32,
}

/// Decay a row-major square projection state, apply a rank-one correction,
/// and emit its scaled query projection. Scratch holds at least n² + 2n words.
/// Callers can reuse that allocation across time steps.
///
/// # Errors
/// A bad extent or failed certified operation preserves both state and scores.
pub fn gated_delta_step_f32(
    input: GatedDeltaInput<'_>,
    state: &mut [f32],
    scores: &mut [f32],
    scratch: &mut [f32],
) -> Result<(), TensorComputeError> {
    use verdant_delta_rule::Adt0 as Op;
    let extent = || TensorComputeError::ZeroOrMismatchedExtents {
        detail: "gated delta projection extents".to_owned(),
    };
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "DeltaRule".to_owned(),
    };
    let cpu_word = |value: f32| u64::from(value.to_bits());
    let evaluate = |op| {
        let bits = verdant_delta_rule::run(op).map_err(|_| invalid())?;
        u32::try_from(bits)
            .map(f32::from_bits)
            .map_err(|_| invalid())
    };
    let width = input.query.len();
    let cells = width.checked_mul(width).ok_or_else(extent)?;
    let needed = cells
        .checked_add(width)
        .and_then(|v| v.checked_add(width))
        .ok_or_else(extent)?;
    if width == 0
        || state.len() != cells
        || scores.len() != width
        || input.key.len() != width
        || input.value.len() != width
        || (input.log_decay.len() != 1 && input.log_decay.len() != width)
    {
        return Err(extent());
    }
    let (next, vectors) = scratch
        .get_mut(..needed)
        .ok_or_else(extent)?
        .split_at_mut(cells);
    let (delta, factors) = vectors.split_at_mut(width);
    let count = verdant_count_f32::run(width as u64)
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?;
    let scale = cpu_word(evaluate(Op::C0(count))?);
    if let [gate] = input.log_decay {
        factors.fill(exponential_f32(*gate)?);
    } else {
        for (dst, &gate) in factors.iter_mut().zip(input.log_decay) {
            *dst = exponential_f32(gate)?;
        }
    }
    for (row, prior) in next.chunks_exact_mut(width).zip(state.chunks_exact(width)) {
        for ((x, &s), &g) in row.iter_mut().zip(prior).zip(&*factors) {
            *x = evaluate(Op::C1(cpu_word(s), cpu_word(g)))?;
        }
    }
    multiply_matrix_rows_f32(width, width, 1, next, input.key, delta)?;
    for ((row, &v), predicted) in next.chunks_exact_mut(width).zip(input.value).zip(delta) {
        let correction = cpu_word(evaluate(Op::C2(
            cpu_word(v),
            cpu_word(*predicted),
            cpu_word(input.gain),
        ))?);
        for (x, &k) in row.iter_mut().zip(input.key) {
            *x = evaluate(Op::C3(cpu_word(*x), cpu_word(k), correction))?;
        }
    }
    multiply_matrix_rows_f32(width, width, 1, next, input.query, factors)?;
    for score in &mut *factors {
        *score = evaluate(Op::C4(cpu_word(*score), scale))?;
    }
    state.copy_from_slice(next);
    scores.copy_from_slice(factors);
    Ok(())
}
