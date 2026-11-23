//! Execute the certified grouped affine fitting and bit placement plans.
use super::{
    convert_float_storage, encode_small_integer, evaluate_encoded_word, evaluate_f32_scalar,
};
use crate::{scalars, TensorComputeError};
use verdant_affine_fit::{Adt0 as Program, Adt1 as Instruction};
use verdant_affine_geometry::Adt0 as Format;

const REGISTERS: usize = scalars::eight_u64() as usize + 1;
const GROUPS: usize = scalars::sixteen_u64() as usize;
const WIDTH: usize = scalars::thirty_two_u64() as usize;
const LANES: usize = WIDTH * scalars::eight_u64() as usize;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GroupedAffineEncoding".into(),
    }
}

fn less(a: u64, b: u64) -> Result<bool, TensorComputeError> {
    Ok(evaluate_encoded_word(0, a, b)
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?
        == 0)
}

struct Fit {
    bits: u64,
    level_max: u64,
    trials: u64,
}

impl Fit {
    fn calculate(
        &self,
        program: Program,
        input: &[u64],
    ) -> Result<[u64; REGISTERS], TensorComputeError> {
        let mut registers = [0; REGISTERS];
        registers
            .get_mut(..input.len())
            .ok_or_else(invalid)?
            .copy_from_slice(input);
        for phase in 0.. {
            let read = |slot: u64| registers.get(slot as usize).copied().ok_or_else(invalid);
            let instruction = verdant_affine_fit::run(program.clone(), self.bits, phase);
            let (dst, result) = match instruction {
                Instruction::C0(op, a, b, dst) => {
                    let value = evaluate_f32_scalar(
                        op,
                        f32::from_bits(read(a)? as u32),
                        f32::from_bits(read(b)? as u32),
                    )?;
                    (dst, u64::from(value.to_bits()))
                }
                Instruction::C1(value, dst) => (dst, value),
                Instruction::C2(a, dst) => (
                    dst,
                    encode_small_integer(read(a)?, 0)
                        .map_err(|_| invalid())?
                        .ok_or_else(invalid)?,
                ),
                Instruction::C3(a, limit, bias, dst) => (
                    dst,
                    verdant_affine_quantized::run(read(a)?, read(limit)?, read(bias)?),
                ),
                Instruction::C4(a, dst) => (
                    dst,
                    convert_float_storage(0, 1, read(a)?)
                        .map_err(|_| invalid())?
                        .ok_or_else(invalid)?,
                ),
                Instruction::C5(a, dst) => (
                    dst,
                    convert_float_storage(1, 0, read(a)?)
                        .map_err(|_| invalid())?
                        .ok_or_else(invalid)?,
                ),
                Instruction::C6 => return Ok(registers),
            };
            *registers.get_mut(dst as usize).ok_or_else(invalid)? = result;
        }
        unreachable!()
    }

    fn code(&self, x: u64, minimum: u64, inverse: u64) -> Result<u64, TensorComputeError> {
        Ok(self.calculate(Program::C6, &[x, minimum, inverse, self.level_max])?[0])
    }

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
            let [_, _, _, _, _, sum, ..] =
                self.calculate(Program::C11, &[x, w, level, scale, minimum, error])?;
            error = sum;
        }
        Ok(error)
    }

    fn group(&self, values: &[u64], codes: &mut [u64]) -> Result<(u64, u64), TensorComputeError> {
        let mut minimum = values[0];
        let mut maximum = minimum;
        let mut sum_squares = 0;
        for &x in values {
            if less(x, minimum)? {
                minimum = x;
            }
            if less(maximum, x)? {
                maximum = x;
            }
            if self.bits != scalars::two_u64() {
                sum_squares = self.calculate(Program::C0, &[x, sum_squares])?[1];
            }
        }
        let rms = if self.bits == scalars::two_u64() {
            0
        } else {
            self.calculate(Program::C1, &[sum_squares])?[0]
        };
        if less(0, minimum)? {
            minimum = 0;
        }
        let negate = |x| {
            evaluate_f32_scalar(u64::from(scalars::six_u32()), f32::from_bits(x as u32), 0.0)
                .map(|v| u64::from(v.to_bits()))
        };
        if !less(minimum, maximum)? {
            codes.fill(0);
            return Ok((0, negate(minimum)?));
        }
        let mut weights = [0; WIDTH];
        let mut sum_w = 0;
        let mut sum_x = 0;
        for (&x, w) in values.iter().zip(&mut weights) {
            *w = self.calculate(Program::C2, &[x, rms])?[0];
            let [_, _, next_w, next_x, ..] = self.calculate(Program::C3, &[x, *w, sum_w, sum_x])?;
            (sum_w, sum_x) = (next_w, next_x);
        }
        let [inverse, mut scale, ..] =
            self.calculate(Program::C4, &[maximum, minimum, self.level_max])?;
        for (&x, code) in values.iter().zip(codes.iter_mut()) {
            *code = self.code(x, minimum, inverse)?;
        }
        let mut best = self.error(values, &weights, codes, scale, minimum)?;
        let mut candidate = [0; WIDTH];
        for trial in 0..self.trials {
            let [inverse, ..] =
                self.calculate(Program::C5, &[maximum, minimum, trial, self.level_max])?;
            let (mut sum_l, mut sum_l2, mut sum_xl) = (0, 0, 0);
            for ((&x, &w), slot) in values.iter().zip(&weights).zip(&mut candidate) {
                *slot = self.code(x, minimum, inverse)?;
                let [_, _, _, l, ll, xl, ..] =
                    self.calculate(Program::C7, &[x, w, *slot, sum_l, sum_l2, sum_xl])?;
                (sum_l, sum_l2, sum_xl) = (l, ll, xl);
            }
            let [det, ..] = self.calculate(Program::C8, &[sum_w, sum_l, sum_l2])?;
            if !less(0, det)? {
                continue;
            }
            let [_, _, _, _, _, _, mut slope, mut intercept, ..] =
                self.calculate(Program::C9, &[sum_w, sum_x, sum_l, sum_l2, sum_xl, det])?;
            if less(0, intercept)? {
                intercept = 0;
                slope = self.calculate(Program::C10, &[sum_xl, sum_l2])?[0];
            }
            let error = self.error(values, &weights, &candidate, slope, intercept)?;
            if less(error, best)? {
                (best, scale, minimum) = (error, slope, intercept);
                codes.copy_from_slice(&candidate[..codes.len()]);
            }
        }
        Ok((scale, negate(minimum)?))
    }

    fn scale(&self, largest: u64, max_code: u64) -> Result<(u64, u64, u64), TensorComputeError> {
        if !less(0, largest)? {
            return Ok((0, 0, 0));
        }
        let [_, inverse, half, expanded, ..] =
            self.calculate(Program::C12, &[largest, max_code])?;
        Ok((inverse, half, expanded))
    }

    fn assign(
        &self,
        value: u64,
        inverse: u64,
        base: u64,
        max_code: u64,
    ) -> Result<(u64, u64), TensorComputeError> {
        let [code, ..] = self.calculate(Program::C13, &[value, inverse, max_code])?;
        let [local, ..] = self.calculate(Program::C14, &[base, code])?;
        Ok((code, local))
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
        if less(largest_scale, *scale)? {
            largest_scale = *scale;
        }
        if less(largest_min, *minimum)? {
            largest_min = *minimum;
        }
    }
    let (iscale, half_scale, scale) = fit.scale(largest_scale, max_code)?;
    let (imin, half_min, minimum) = fit.scale(largest_min, max_code)?;
    for (((xs, codes), sc), mn) in values
        .chunks_exact(width)
        .zip(levels.chunks_exact_mut(width))
        .zip(&mut group_scales)
        .zip(&mut group_mins)
    {
        let (code, unit) = fit.assign(*sc, iscale, scale, max_code)?;
        *sc = code;
        let (code, offset) = fit.assign(*mn, imin, minimum, max_code)?;
        *mn = code;
        if less(0, unit)? {
            for (&x, c) in xs.iter().zip(codes) {
                *c = fit.calculate(Program::C15, &[x, unit, offset, level_max])?[0];
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
