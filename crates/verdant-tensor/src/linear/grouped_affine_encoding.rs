//! Execute the certified grouped affine fitting and bit placement plans.
use super::evaluate_f32_scalar;
use crate::{scalars, TensorComputeError};
use verdant_affine_fit::{Adt0 as Equation, Adt1::C0 as Terms};
use verdant_affine_geometry::Adt0 as Format;
use verdant_affine_less::run as less;

const GROUPS: usize = scalars::sixteen_u64() as usize;
const WIDTH: usize = scalars::thirty_two_u64() as usize;
const LANES: usize = WIDTH * scalars::eight_u64() as usize;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GroupedAffineEncoding".into(),
    }
}

struct Fit {
    bits: u64,
    level_max: u64,
    trials: u64,
}

impl Fit {
    fn error(
        &self,
        values: &[u64],
        weights: &[u64],
        codes: &[u64],
        scale: u64,
        minimum: u64,
    ) -> Result<u64, TensorComputeError> {
        let mut error = 0;
        for ((&x, &w), &level) in values.iter().zip(weights).zip(codes) {
            let Terms(sum, _, _) = verdant_affine_fit::run(
                Equation::C10(x, w, level, scale, minimum, error),
                self.bits,
            )
            .map_err(|_| invalid())?;
            error = sum;
        }
        Ok(error)
    }

    fn group(&self, values: &[u64], codes: &mut [u64]) -> Result<(u64, u64), TensorComputeError> {
        let mut minimum = values[0];
        let mut maximum = minimum;
        let mut sum_squares = 0;
        for &x in values {
            if less(x, minimum) {
                minimum = x;
            }
            if less(maximum, x) {
                maximum = x;
            }
            if self.bits != scalars::two_u64() {
                sum_squares = {
                    let Terms(value, _, _) =
                        verdant_affine_fit::run(Equation::C0(x, sum_squares), self.bits)
                            .map_err(|_| invalid())?;
                    value
                };
            }
        }
        let rms = if self.bits == scalars::two_u64() {
            0
        } else {
            {
                let Terms(value, _, _) =
                    verdant_affine_fit::run(Equation::C1(sum_squares), self.bits)
                        .map_err(|_| invalid())?;
                value
            }
        };
        if less(0, minimum) {
            minimum = 0;
        }
        let negate = |x| {
            evaluate_f32_scalar(u64::from(scalars::six_u32()), f32::from_bits(x as u32), 0.0)
                .map(|v| u64::from(v.to_bits()))
        };
        if !less(minimum, maximum) {
            codes.fill(0);
            return Ok((0, negate(minimum)?));
        }
        let mut weights = [0; WIDTH];
        let mut sum_w = 0;
        let mut sum_x = 0;
        for (&x, w) in values.iter().zip(&mut weights) {
            *w = {
                let Terms(value, _, _) = verdant_affine_fit::run(Equation::C2(x, rms), self.bits)
                    .map_err(|_| invalid())?;
                value
            };
            let Terms(next_w, next_x, _) =
                verdant_affine_fit::run(Equation::C3(x, *w, sum_w, sum_x), self.bits)
                    .map_err(|_| invalid())?;
            (sum_w, sum_x) = (next_w, next_x);
        }
        let Terms(inverse, mut scale, _) =
            verdant_affine_fit::run(Equation::C4(maximum, minimum, self.level_max), self.bits)
                .map_err(|_| invalid())?;
        for (&x, code) in values.iter().zip(codes.iter_mut()) {
            *code = verdant_affine_lane_code::run(x, minimum, inverse, self.level_max)
                .map_err(|_| invalid())?;
        }
        let mut best = self.error(values, &weights, codes, scale, minimum)?;
        let mut candidate = [0; WIDTH];
        for trial in 0..self.trials {
            let Terms(inverse, _, _) = verdant_affine_fit::run(
                Equation::C5(maximum, minimum, trial, self.level_max),
                self.bits,
            )
            .map_err(|_| invalid())?;
            let (mut sum_l, mut sum_l2, mut sum_xl) = (0, 0, 0);
            for ((&x, &w), slot) in values.iter().zip(&weights).zip(&mut candidate) {
                *slot = verdant_affine_lane_code::run(x, minimum, inverse, self.level_max)
                    .map_err(|_| invalid())?;
                let Terms(l, ll, xl) = verdant_affine_fit::run(
                    Equation::C6(x, w, *slot, sum_l, sum_l2, sum_xl),
                    self.bits,
                )
                .map_err(|_| invalid())?;
                (sum_l, sum_l2, sum_xl) = (l, ll, xl);
            }
            let Terms(det, _, _) =
                verdant_affine_fit::run(Equation::C7(sum_w, sum_l, sum_l2), self.bits)
                    .map_err(|_| invalid())?;
            if !less(0, det) {
                continue;
            }
            let Terms(mut slope, mut intercept, _) = verdant_affine_fit::run(
                Equation::C8(sum_w, sum_x, sum_l, sum_l2, sum_xl, det),
                self.bits,
            )
            .map_err(|_| invalid())?;
            if less(0, intercept) {
                intercept = 0;
                slope = {
                    let Terms(value, _, _) =
                        verdant_affine_fit::run(Equation::C9(sum_xl, sum_l2), self.bits)
                            .map_err(|_| invalid())?;
                    value
                };
            }
            let error = self.error(values, &weights, &candidate, slope, intercept)?;
            if less(error, best) {
                (best, scale, minimum) = (error, slope, intercept);
                codes.copy_from_slice(&candidate[..codes.len()]);
            }
        }
        Ok((scale, negate(minimum)?))
    }
}

pub(super) fn encode(
    bits: u64,
    input: &[f32],
    output: &mut [u8],
) -> Result<(), TensorComputeError> {
    let Format::C1(width, level_max, max_code, trials) =
        verdant_affine_geometry::run(bits, input.len() as u64, output.len() as u64)
    else {
        return Err(invalid());
    };
    let width = width as usize;
    let fit = Fit {
        bits,
        level_max,
        trials,
    };
    let mut values = [0; LANES];
    for (slot, x) in values.iter_mut().zip(input) {
        *slot = u64::from(x.to_bits());
        if !verdant_affine_finite::run(*slot) {
            return Err(invalid());
        }
    }
    let mut levels = [0; LANES];
    let mut group_scales = [0; GROUPS];
    let mut group_mins = [0; GROUPS];
    let (mut largest_scale, mut largest_min) = (0, 0);
    for (((xs, codes), scale), minimum) in values
        .chunks_exact(width)
        .zip(levels.chunks_exact_mut(width))
        .zip(&mut group_scales)
        .zip(&mut group_mins)
    {
        (*scale, *minimum) = fit.group(xs, codes)?;
        if less(largest_scale, *scale) {
            largest_scale = *scale;
        }
        if less(largest_min, *minimum) {
            largest_min = *minimum;
        }
    }
    let verdant_affine_block_scale::Adt0::C0(iscale, half_scale, scale) =
        verdant_affine_block_scale::run(largest_scale, max_code).map_err(|_| invalid())?;
    let verdant_affine_block_scale::Adt0::C0(imin, half_min, minimum) =
        verdant_affine_block_scale::run(largest_min, max_code).map_err(|_| invalid())?;
    for (((xs, codes), sc), mn) in values
        .chunks_exact(width)
        .zip(levels.chunks_exact_mut(width))
        .zip(&mut group_scales)
        .zip(&mut group_mins)
    {
        let verdant_affine_group_scale::Adt0::C0(code, unit) =
            verdant_affine_group_scale::run(*sc, iscale, scale, max_code).map_err(|_| invalid())?;
        *sc = code;
        let verdant_affine_group_scale::Adt0::C0(code, offset) =
            verdant_affine_group_scale::run(*mn, imin, minimum, max_code).map_err(|_| invalid())?;
        *mn = code;
        if less(0, unit) {
            for (&x, c) in xs.iter().zip(codes) {
                *c = {
                    let Terms(value, _, _) = verdant_affine_fit::run(
                        Equation::C11(x, unit, offset, level_max),
                        fit.bits,
                    )
                    .map_err(|_| invalid())?;
                    value
                };
            }
        }
    }
    let banks: [&[u64]; scalars::five_u32() as usize] = [
        &levels,
        &group_scales,
        &group_mins,
        std::slice::from_ref(&half_scale),
        std::slice::from_ref(&half_min),
    ];
    let mut packet = [0; LANES];
    for (byte, out) in packet[..output.len()].iter_mut().enumerate() {
        let mut packed = 0;
        for phase in 0.. {
            match verdant_affine_weight_fields::run(bits, byte as u64, phase)
                .map_err(|_| invalid())?
            {
                verdant_affine_weight_fields::Adt0::C1 => break,
                verdant_affine_weight_fields::Adt0::C2(bank, slot, divisor, radix, place) => {
                    let value = banks
                        .get(bank as usize)
                        .and_then(|b| b.get(slot as usize))
                        .ok_or_else(invalid)?;
                    packed =
                        verdant_affine_insert_field::run(packed, *value, divisor, radix, place)
                            .map_err(|_| invalid())?;
                }
                _ => return Err(invalid()),
            }
        }
        *out = u8::try_from(packed).map_err(|_| invalid())?;
    }
    output.copy_from_slice(&packet[..output.len()]);
    Ok(())
}
