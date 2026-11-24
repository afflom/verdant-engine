use super::{convert_float_storage, TensorComputeError};

/// Evaluate an F32 operation using the certified Prism integer encoding.
///
/// Operation codes are add, subtract, multiply, divide, square root, square,
/// negate and absolute value in that order, beginning at zero. Rounding is
/// nearest-even; exceptional operands follow the scalar model. Unknown codes
/// or a model overflow return `InvalidKernelSpecification`.
pub fn evaluate_f32_scalar(
    operation: u64,
    left: f32,
    right: f32,
) -> Result<f32, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "ScalarF32.evaluate".to_string(),
    };
    let value = verdant_scalar_f32::run(
        operation,
        u64::from(left.to_bits()),
        u64::from(right.to_bits()),
    )
    .map_err(|_| failure())?
    .ok_or_else(failure)?;
    let bits = u32::try_from(value).map_err(|_| failure())?;
    Ok(f32::from_bits(bits))
}

/// Multiply the SiLU activation of `value` by `gate` in binary32.
///
/// # Errors
/// Propagates a typed error from a certified scalar operation.
pub fn swiglu_f32(value: f32, gate: f32) -> Result<f32, TensorComputeError> {
    use verdant_logistic::Adt0 as Action;
    let decode = |word: u64| f32::from_bits(word as u32);
    let (x, multiplier) = (u64::from(value.to_bits()), u64::from(gate.to_bits()));
    let mut prior = 0;
    let mut stage = 0;
    loop {
        let out = match verdant_logistic::run(false, true, stage, x, multiplier, prior) {
            Action::C0(operation, a, b) => evaluate_f32_scalar(operation, decode(a), decode(b))?,
            Action::C1(a) => exponential_f32(decode(a))?,
            Action::C2(a) => return Ok(decode(a)),
        };
        prior = u64::from(out.to_bits());
        stage += 1;
    }
}

/// Compute the binary32 geometric slope for one head of a linear attention bias.
///
/// # Errors
/// Rejects nonfinite biases, empty head groups and out-of-range head indices.
pub fn attention_head_slope_f32(
    bias: f32,
    heads: u32,
    head: u32,
) -> Result<f32, TensorComputeError> {
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "HeadSlope.evaluate".into(),
    };
    let bias = u64::from(bias.to_bits());
    let head = u64::from(head);
    let heads = u64::from(heads);
    let bias = verdant_head_slope::run(bias, heads, head).ok_or_else(invalid)?;
    let pivot = verdant_head_slope_group::run(heads);
    let (denominator, count) =
        verdant_head_slope_exponent::run(head, pivot).map_err(|_| invalid())?;
    let divisor = verdant_count_f32::run(denominator)
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?;
    let decode = |bits: u64| f32::from_bits(bits as u32);
    let scaled = evaluate_f32_scalar(crate::scalars::three_u64(), decode(bias), decode(divisor))?;
    let scaled = u64::from(scaled.to_bits());
    let mut count = verdant_head_slope_index::run(count).map_err(|_| invalid())?;
    let multiply = |a, b| {
        let (high, (cross, low)) = verdant_head_slope_limbs::run(a, b).map_err(|_| invalid())?;
        verdant_head_slope_multiply::run(high, cross, low).map_err(|_| invalid())
    };
    let to_word = |f, down| verdant_head_slope_word::run(f, down).map_err(|_| invalid());
    let base = match verdant_head_slope_base::run(scaled).map_err(|_| invalid())? {
        verdant_head_slope_base::Adt0::C0(word) => word,
        verdant_head_slope_base::Adt0::C1(whole, fraction, logarithm, mut order, mut term) => {
            let argument = multiply(fraction, logarithm)?;
            while order != 0 {
                (order, term) = verdant_head_slope_term::run(multiply(argument, term)?, order)
                    .map_err(|_| invalid())?;
            }
            to_word(term, whole)?
        }
    };
    let product = |a: (u64, u64), b: (u64, u64)| {
        verdant_head_slope_normalize::run(multiply(a.0, b.0)?, a.1, b.1).map_err(|_| invalid())
    };
    let word = match verdant_head_slope_power::run(base, count).map_err(|_| invalid())? {
        verdant_head_slope_power::Adt0::C0(word) => word,
        verdant_head_slope_power::Adt0::C1(f, down, unit, exponent) => {
            let mut factor = (f, down);
            let mut value = (unit, exponent);
            while count != 0 {
                let (next, odd) = verdant_head_slope_digit::run(count);
                if odd {
                    value = product(value, factor)?;
                }
                if next != 0 {
                    factor = product(factor, factor)?;
                }
                count = next;
            }
            to_word(value.0, value.1)?
        }
    };
    Ok(f32::from_bits(u32::try_from(word).map_err(|_| invalid())?))
}

/// Evaluate binary32 hyperbolic tangent with sign-preserving zeros and saturation.
///
/// # Errors
/// Propagates a typed failure from certified word operations or exponentiation.
pub fn tanh_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_tanh::Adt0 as Action;
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "HyperbolicTangent.evaluate".into(),
    };
    let evaluate = |exp: Option<f32>| {
        verdant_tanh::run(
            u64::from(value.to_bits()),
            exp.is_some(),
            u64::from(exp.unwrap_or_default().to_bits()),
        )
        .map_err(|_| invalid())
    };
    let result = match evaluate(None)? {
        Action::C0(word) => word,
        Action::C1(argument) => {
            let exp = exponential_f32(f32::from_bits(argument as u32))?;
            let Action::C0(word) = evaluate(Some(exp))? else {
                return Err(invalid());
            };
            word
        }
    };
    Ok(f32::from_bits(result as u32))
}

/// Evaluate the binary32 hyperbolic-tangent approximation to the Gaussian error linear unit.
///
/// # Errors
/// Propagates failures from the certified word operations.
pub fn gelu_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_gelu::Adt0 as Action;
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GaussianActivation.evaluate".into(),
    };
    let request = |response: Option<f32>| {
        verdant_gelu::run(
            false,
            u64::from(value.to_bits()),
            response.is_some(),
            u64::from(response.unwrap_or_default().to_bits()),
        )
        .map_err(|_| invalid())
    };
    let intermediate = match request(None)? {
        Action::C1(word) => tanh_f32(f32::from_bits(word as u32))?,
        Action::C2(word) => sigmoid_f32(f32::from_bits(word as u32))?,
        Action::C0(word) => return Ok(f32::from_bits(word as u32)),
    };
    let Action::C0(word) = request(Some(intermediate))? else {
        return Err(invalid());
    };
    Ok(f32::from_bits(word as u32))
}

/// Evaluate the binary32 logistic approximation to the Gaussian error linear unit.
///
/// # Errors
/// Propagates failures from the certified word operations.
pub fn gelu_quick_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_gelu::Adt0 as Action;
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GaussianActivation.evaluate".into(),
    };
    let request = |response: Option<f32>| {
        verdant_gelu::run(
            true,
            u64::from(value.to_bits()),
            response.is_some(),
            u64::from(response.unwrap_or_default().to_bits()),
        )
        .map_err(|_| invalid())
    };
    let intermediate = match request(None)? {
        Action::C1(word) => tanh_f32(f32::from_bits(word as u32))?,
        Action::C2(word) => sigmoid_f32(f32::from_bits(word as u32))?,
        Action::C0(word) => return Ok(f32::from_bits(word as u32)),
    };
    let Action::C0(word) = request(Some(intermediate))? else {
        return Err(invalid());
    };
    Ok(f32::from_bits(word as u32))
}

/// Evaluate the binary32 error function, or GELU with that error function when `gelu` is true.
///
/// # Errors
/// Propagates a typed error from the certified scalar program.
pub fn gaussian_error_f32(value: f32, gelu: bool) -> Result<f32, TensorComputeError> {
    use verdant_error_function::Adt0 as Action;
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GaussianActivation.evaluateError".into(),
    };
    let x = u64::from(value.to_bits());
    let mut registers = [x, 0, 0, 0, 0, 0, 0];
    let mut pc = 0;
    let decode = |word| f32::from_bits(word as u32);
    loop {
        let [argument, basis, numerator, denominator, temp, held, magnitude] = registers;
        let action = verdant_error_function::run(
            gelu,
            x,
            pc,
            argument,
            basis,
            numerator,
            denominator,
            temp,
            held,
            magnitude,
        )
        .map_err(|_| invalid())?;
        let (value, slot, next) = match action {
            Action::C0(word) => return Ok(decode(word)),
            Action::C1(op, a, b, slot, next) => {
                (evaluate_f32_scalar(op, decode(a), decode(b))?, slot, next)
            }
            Action::C2(a, slot, next) => (exponential_f32(decode(a))?, slot, next),
        };
        *registers.get_mut(slot as usize).ok_or_else(invalid)? = value.to_bits().into();
        pc = next;
    }
}

/// Evaluate the binary32 logistic sigmoid.
///
/// # Errors
/// Propagates a typed error from a certified scalar operation.
pub fn sigmoid_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_logistic::Adt0 as Action;
    let decode = |word: u64| f32::from_bits(word as u32);
    let x = u64::from(value.to_bits());
    let mut prior = 0;
    let mut stage = 0;
    loop {
        let out = match verdant_logistic::run(true, false, stage, x, 0, prior) {
            Action::C0(operation, a, b) => evaluate_f32_scalar(operation, decode(a), decode(b))?,
            Action::C1(a) => exponential_f32(decode(a))?,
            Action::C2(a) => return Ok(decode(a)),
        };
        prior = u64::from(out.to_bits());
        stage += 1;
    }
}

/// Evaluate the binary32 sigmoid linear unit.
///
/// # Errors
/// Propagates a typed error from a certified scalar operation.
pub fn silu_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_logistic::Adt0 as Action;
    let decode = |word: u64| f32::from_bits(word as u32);
    let x = u64::from(value.to_bits());
    let mut prior = 0;
    let mut stage = 0;
    loop {
        let out = match verdant_logistic::run(false, false, stage, x, 0, prior) {
            Action::C0(operation, a, b) => evaluate_f32_scalar(operation, decode(a), decode(b))?,
            Action::C1(a) => exponential_f32(decode(a))?,
            Action::C2(a) => return Ok(decode(a)),
        };
        prior = u64::from(out.to_bits());
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

/// Evaluate softplus for a binary32 input. Large positive values pass through,
/// and a compensated logarithm preserves small exponential increments.
///
/// # Errors
/// Returns a typed kernel error if a certified word operation overflows.
pub fn softplus_f32(input: f32) -> Result<f32, TensorComputeError> {
    use verdant_softplus::{run, Adt0 as Request, Adt1 as Step};
    let mut request = Request::C0(input.to_bits().into());
    loop {
        request =
            match run(request).map_err(|_| TensorComputeError::InvalidKernelSpecification {
                kernel_name: "Softplus".into(),
            })? {
                Step::C0(word) => return Ok(f32::from_bits(word as u32)),
                Step::C1(word) => Request::C1(
                    exponential_f32(f32::from_bits(word as u32))?
                        .to_bits()
                        .into(),
                ),
                Step::C2(unit, value) => Request::C2(
                    unit,
                    value,
                    natural_log_f32(f32::from_bits(unit as u32))?
                        .to_bits()
                        .into(),
                ),
            };
    }
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

/// Evaluate the binary32 natural logarithm, including IEEE zeros and nonfinite inputs.
///
/// # Errors
/// Returns a typed kernel error if a certified word operation overflows.
pub fn natural_log_f32(value: f32) -> Result<f32, TensorComputeError> {
    use verdant_logarithmic_series::{Adt0 as State, Adt1 as Action};
    let word = match verdant_logarithmic_prepare::run(u64::from(value.to_bits()))
        .map_err(|_| log_failure())?
    {
        verdant_logarithmic_prepare::Adt0::C0(word) => word,
        verdant_logarithmic_prepare::Adt0::C1(negative, whole, ratio) => {
            let mut state = State::C0;
            let mut product = 0;
            let magnitude = loop {
                match verdant_logarithmic_series::run(negative, whole, ratio, state, product)
                    .map_err(|_| log_failure())?
                {
                    Action::C0(a, b, next) => {
                        product =
                            verdant_logarithmic_product::run(a, b).map_err(|_| log_failure())?;
                        state = next;
                    }
                    Action::C1(v) => break v,
                }
            };
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
    use verdant_logarithmic_series::{Adt0 as State, Adt1 as Action};
    let base = u64::from(base.to_bits());
    if !verdant_logarithmic_admit::run(base, u64::from(degree)) {
        return Err(log_failure());
    }
    let verdant_logarithmic_prepare::Adt0::C1(negative, whole, ratio) =
        verdant_logarithmic_prepare::run(base).map_err(|_| log_failure())?
    else {
        return Err(log_failure());
    };
    let mut state = State::C0;
    let mut product = 0;
    let magnitude = loop {
        match verdant_logarithmic_series::run(negative, whole, ratio, state, product)
            .map_err(|_| log_failure())?
        {
            Action::C0(a, b, next) => {
                product = verdant_logarithmic_product::run(a, b).map_err(|_| log_failure())?;
                state = next;
            }
            Action::C1(v) => break v,
        }
    };
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

/// Evaluate ABS, SGN, NEG, STEP, RELU, SQR, SQRT, HARDSIGMOID, HARDSWISH,
/// FLOOR, CEIL, ROUND, TRUNC and EXPM1 (codes 0..13) on encoded float storage.
/// Source/destination pairs are F32/F32, F16/F16, BF16/BF16, F16/F32 and BF16/F32.
/// Unsupported codes/pairs and out-of-width words return `None`.
///
/// # Errors
/// Returns `InvalidKernelSpecification` if a certified component overflows.
pub fn evaluate_float_unary(
    operation: u64,
    source: u64,
    destination: u64,
    bits: u64,
) -> Result<Option<u64>, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "FloatUnary.evaluate".to_string(),
    };
    let Some(plan) =
        verdant_unary_dispatch::run(operation, source, destination, bits).map_err(|_| failure())?
    else {
        return Ok(None);
    };
    let answer = match plan {
        verdant_unary_dispatch::Adt0::C0(ordinary) => match ordinary {
            verdant_unary_dispatch::Adt1::C0(kind, word) => {
                verdant_scalar_f32::run(kind, word, 0).map_err(|_| failure())
            }
            verdant_unary_dispatch::Adt1::C1(kind, word) => {
                verdant_unary_select::run(kind, word).map_err(|_| failure())
            }
            verdant_unary_dispatch::Adt1::C2(kind, word) => {
                verdant_unary_activate::run(kind, word).map_err(|_| failure())
            }
            verdant_unary_dispatch::Adt1::C3(kind, word) => {
                verdant_unary_round::run(kind, word).map_err(|_| failure())
            }
        },
        verdant_unary_dispatch::Adt0::C1(word) => {
            use verdant_exponential_increment::Adt0 as Increment;
            let value = match verdant_exponential_increment::run(word).map_err(|_| failure())? {
                Increment::C0 => return Ok(None),
                Increment::C1(value) => value,
                Increment::C2(argument, unit) => {
                    let exp = exponential_f32(f32::from_bits(argument as u32))?;
                    let result = evaluate_f32_scalar(1, exp, f32::from_bits(unit as u32))?;
                    u64::from(result.to_bits())
                }
                Increment::C3(
                    negative,
                    sign,
                    exponent,
                    mantissa,
                    argument,
                    low,
                    high,
                    mut factor,
                ) => {
                    let unit = factor;
                    for order in (low..=high).rev() {
                        let positive = verdant_exponential_step::run(argument, order, factor)
                            .map_err(|_| failure())?;
                        factor = verdant_increment_term::run(negative, unit, positive);
                    }
                    verdant_increment_pack::run(sign, exponent, mantissa, factor)
                        .map_err(|_| failure())?
                }
            };
            Ok(Some(value))
        }
    }?;
    match answer {
        None => Ok(None),
        Some(word) => convert_float_storage(0, destination, word).map_err(|_| failure()),
    }
}
