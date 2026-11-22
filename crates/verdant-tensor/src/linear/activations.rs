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
