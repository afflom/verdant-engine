use super::{evaluate_f32_scalar, TensorComputeError};

/// Multiply the SiLU activation of `value` by `gate` in binary32.
///
/// # Errors
/// Propagates a typed error from a certified scalar operation.
pub fn swiglu_f32(value: f32, gate: f32) -> Result<f32, TensorComputeError> {
    use verdant_swiglu::Adt0 as Action;
    let decode = |word: u64| f32::from_bits(word as u32);
    let (x, multiplier) = (u64::from(value.to_bits()), u64::from(gate.to_bits()));
    let mut prior = 0;
    let mut stage = 0;
    loop {
        let value = match verdant_swiglu::run(stage, x, multiplier, prior) {
            Action::C0(operation, a, b) => evaluate_f32_scalar(operation, decode(a), decode(b))?,
            Action::C1(a) => exponential_f32(decode(a))?,
            Action::C2(a) => return Ok(decode(a)),
        };
        prior = u64::from(value.to_bits());
        stage += 1;
    }
}

/// Evaluate the natural exponential of a binary32 value, including gradual underflow.
///
/// # Errors
/// Returns `InvalidKernelSpecification` if the certified evaluator exceeds its word width.
pub fn exponential_f32(value: f32) -> Result<f32, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "NaturalExponential.evaluate".to_owned(),
    };
    let reduction =
        verdant_exponential_f32::run(u64::from(value.to_bits())).map_err(|_| failure())?;
    let word = match reduction {
        verdant_exponential_f32::Adt0::C0(exact) => exact,
        verdant_exponential_f32::Adt0::C1(scale, residual, order, mut value) => {
            for term in (1..=order).rev() {
                value =
                    verdant_exponential_step::run(residual, term, value).map_err(|_| failure())?;
            }
            verdant_exponential_scale::run(scale, value).map_err(|_| failure())?
        }
    };
    Ok(f32::from_bits(u32::try_from(word).map_err(|_| failure())?))
}

/// Evaluate the sine and cosine of one binary32 angle in radians.
///
/// # Errors
/// Returns a typed kernel error if a certified word operation overflows.
pub fn sin_cos_f32(radians: f32) -> Result<(f32, f32), TensorComputeError> {
    let error = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "CircularFunctions".to_owned(),
    };
    let (sin, cos) =
        match verdant_circular_prepare::run(u64::from(radians.to_bits())).map_err(|_| error())? {
            verdant_circular_prepare::Adt0::C0(sin, cos) => (sin, cos),
            verdant_circular_prepare::Adt0::C1(negative, mantissa, shift) => {
                let verdant_circular_reduce::Adt0::C0(quadrant, upper, fraction) =
                    verdant_circular_reduce::run(mantissa, shift).map_err(|_| error())?;
                let square = verdant_circular_square::run(fraction).map_err(|_| error())?;
                let verdant_circular_magnitudes::Adt0::C0(sin, cos) =
                    verdant_circular_magnitudes::run(fraction, square).map_err(|_| error())?;
                let verdant_circular_orient::Adt0::C0(sin, cos) =
                    verdant_circular_orient::run(negative, quadrant, upper, sin, cos)
                        .map_err(|_| error())?;
                (sin, cos)
            }
        };
    let decode = |word| u32::try_from(word).map(f32::from_bits).map_err(|_| error());
    Ok((decode(sin)?, decode(cos)?))
}

fn log_failure() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "LogarithmicScale".to_owned(),
    }
}

fn logarithm_magnitude(negative: bool, whole: u64, ratio: u64) -> Result<u64, TensorComputeError> {
    use verdant_logarithmic_series::{Adt0 as State, Adt1 as Action};
    let mut state = State::C0;
    let mut product = 0;
    loop {
        match verdant_logarithmic_series::run(negative, whole, ratio, state, product)
            .map_err(|_| log_failure())?
        {
            Action::C0(a, b, next) => {
                product = verdant_logarithmic_product::run(a, b).map_err(|_| log_failure())?;
                state = next;
            }
            Action::C1(value) => return Ok(value),
        }
    }
}

/// Evaluate the binary32 natural logarithm, including IEEE zeros and nonfinite inputs.
///
/// # Errors
/// Returns a typed kernel error if a certified word operation overflows.
pub fn natural_log_f32(value: f32) -> Result<f32, TensorComputeError> {
    let word = match verdant_logarithmic_prepare::run(u64::from(value.to_bits()))
        .map_err(|_| log_failure())?
    {
        verdant_logarithmic_prepare::Adt0::C0(word) => word,
        verdant_logarithmic_prepare::Adt0::C1(negative, whole, ratio) => {
            let magnitude = logarithm_magnitude(negative, whole, ratio)?;
            verdant_logarithmic_word::run(negative, magnitude).map_err(|_| log_failure())?
        }
    };
    Ok(f32::from_bits(
        u32::try_from(word).map_err(|_| log_failure())?,
    ))
}

/// Raise a positive finite base to the negative binary32 reciprocal of the root degree.
///
/// # Errors
/// Returns a typed kernel error for a zero degree, a nonpositive or nonfinite
/// base, or failure of a certified component.
pub fn inverse_root_f32(base: f32, degree: u32) -> Result<f32, TensorComputeError> {
    let base = u64::from(base.to_bits());
    if !verdant_logarithmic_admit::run(base, u64::from(degree)) {
        return Err(log_failure());
    }
    let verdant_logarithmic_prepare::Adt0::C1(negative, whole, ratio) =
        verdant_logarithmic_prepare::run(base).map_err(|_| log_failure())?
    else {
        return Err(log_failure());
    };
    let magnitude = logarithm_magnitude(negative, whole, ratio)?;
    let degree = verdant_count_f32::run(u64::from(degree))
        .map_err(|_| log_failure())?
        .ok_or_else(log_failure)?;
    let divisor = f32::from_bits(u32::try_from(degree).map_err(|_| log_failure())?);
    let inverse = evaluate_f32_scalar(crate::scalars::three_u64(), 1.0, divisor)?;
    let verdant_logarithmic_inverse::Adt0::C0(scale, residual, order, mut current) =
        verdant_logarithmic_inverse::run(negative, magnitude, u64::from(inverse.to_bits()))
            .map_err(|_| log_failure())?;
    for term in (1..=order).rev() {
        let product =
            verdant_logarithmic_product::run(residual, current).map_err(|_| log_failure())?;
        current = verdant_logarithmic_term::run(product, term).map_err(|_| log_failure())?;
    }
    let word = verdant_logarithmic_scale::run(scale, current).map_err(|_| log_failure())?;
    Ok(f32::from_bits(
        u32::try_from(word).map_err(|_| log_failure())?,
    ))
}
