//! Stable normalization of a binary32 score row.

use super::{evaluate_f32_scalar, evaluate_wide, exponential_f32, TensorComputeError};
use crate::scalars::{four_u64, two_u64};
use verdant_normalized_exponential::{run, Adt0 as Request};

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "NormalizedExponential".into(),
    }
}

/// Replace each score with its normalized exponential. An optional extra score
/// contributes to the denominator. Negative infinity represents a masked lane.
///
/// # Errors
/// Returns a typed error for NaN, positive infinity, or a nonempty row whose
/// normalization mass is undefined. These inputs are rejected before writing.
pub fn normalize_exponential_f32(
    row: &mut [f32],
    extra: Option<f32>,
) -> Result<(), TensorComputeError> {
    if row.is_empty() {
        return Ok(());
    }
    let step = |request: Request| -> Result<u32, TensorComputeError> {
        let word = run(request).map_err(|_| invalid())?.ok_or_else(invalid)?;
        u32::try_from(word).map_err(|_| invalid())
    };
    let wide = |op: u64, left: u64, right: u64| -> Result<u64, TensorComputeError> {
        evaluate_wide(op, left, right)?.ok_or_else(invalid)
    };
    let mut maximum = f32::NEG_INFINITY.to_bits();
    for &score in row.iter().chain(extra.as_ref()) {
        maximum = step(Request::C0(u64::from(maximum), u64::from(score.to_bits())))?;
    }
    let weight = |score: f32| {
        let centered = step(Request::C1(u64::from(score.to_bits()), u64::from(maximum)))?;
        exponential_f32(f32::from_bits(centered))
    };
    let mut mass = 0;
    for score in row.iter_mut() {
        let value = weight(*score)?;
        mass = wide(0, mass, wide(1, u64::from(value.to_bits()), 0)?)?;
        *score = value;
    }
    if let Some(score) = extra {
        mass = wide(0, mass, wide(1, u64::from(weight(score)?.to_bits()), 0)?)?;
    }
    let reciprocal = wide(four_u64(), 1.0_f64.to_bits(), mass)?;
    let narrowed = wide(two_u64(), reciprocal, 0)?;
    let factor = f32::from_bits(u32::try_from(narrowed).map_err(|_| invalid())?);
    for score in row {
        *score = evaluate_f32_scalar(two_u64(), *score, factor)?;
    }
    Ok(())
}
