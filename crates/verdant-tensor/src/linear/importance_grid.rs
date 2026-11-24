//! Importance-weighted four-lane grid encoding. All arithmetic and packed field
//! locations come from the certified encoding project. The imported directory
//! retains GGML's candidate order, including its tie break by grid index.

use super::{
    convert_float_storage, evaluate_encoded_word, evaluate_f32_scalar as scalar, signed_word_f32,
};
use super::{PackedWeightEncoding, GGML_QUANTIZED_TABLES};
use crate::{scalars, TensorComputeError};

const TABLE: &[u8] = include_bytes!("../iq3_neighbors.bin");
const FOUR: usize = scalars::four_u64() as usize;
const EIGHT: usize = scalars::eight_u64() as usize;
const WIDTH: usize = scalars::thirty_two_u64() as usize;
const VALUE: u64 = 1;
const LITERAL: u64 = scalars::two_u64();
const SCALAR: u64 = scalars::three_u64();
const GREATER: u64 = scalars::four_u64();
const FINISH: u64 = scalars::five_u32() as u64;
const GEOMETRY: u64 = scalars::six_u32() as u64;
const PAIR: u64 = scalars::seven_u64();
const FIELD: u64 = scalars::eight_u64();
const STEP_FIELDS: usize = scalars::six_u32() as usize;

#[repr(u64)]
enum Query {
    Geometry,
    Decode,
    Digit,
    AppendLevel,
    Tables,
    CodeLevel,
    SignMask,
    Emit,
    ScaleField,
    SignField,
}

type Result<T> = std::result::Result<T, TensorComputeError>;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "ImportanceGridEncoding".into(),
    }
}

fn plan(tag: Query, compact: bool, a: u64, b: u64, c: u64) -> Result<[u64; STEP_FIELDS]> {
    let (kind, (a, (b, (c, (d, e))))) =
        verdant_importance_grid::run(tag as u64, compact, a, b, c).map_err(|_| invalid())?;
    Ok([kind, a, b, c, d, e])
}

fn value(tag: Query, compact: bool, a: u64, b: u64, c: u64) -> Result<u64> {
    match plan(tag, compact, a, b, c)? {
        [VALUE, word, ..] => Ok(word),
        _ => Err(invalid()),
    }
}

fn less(a: f32, b: f32) -> Result<bool> {
    Ok(
        evaluate_encoded_word(0, a.to_bits().into(), b.to_bits().into())
            .map_err(|_| invalid())?
            .ok_or_else(invalid)?
            == 0,
    )
}

#[repr(u64)]
enum Program {
    SumSquare,
    Variance,
    Weight,
    SignCost,
    InitialScale,
    TrialInverse,
    Reciprocal,
    RoundedLevel,
    Distance,
    Moment,
    Gain,
    BaseScale,
    CompactScale,
    FullScale,
}

fn calculate(program: Program, inputs: &[f32]) -> Result<f32> {
    let id = program as u64;
    type Instructions = fn(u64, u64) -> Option<u64>;
    let programs: [Instructions; FOUR] = [
        |program, step| verdant_iq3_fit::run(program, step).ok(),
        |program, step| verdant_iq3_level::run(program, step).ok(),
        |program, step| verdant_iq3_score::run(program, step).ok(),
        |program, step| verdant_iq3_scale::run(program, step).ok(),
    ];
    let instruction = programs[id as usize / FOUR];
    let mut registers = [0.0; EIGHT];
    registers[..inputs.len()].copy_from_slice(inputs);
    let mut phase = 0;
    loop {
        let (target, word) = match plan(
            Query::Decode,
            false,
            instruction(id % scalars::four_u64(), phase).ok_or_else(invalid)?,
            0,
            0,
        )? {
            [LITERAL, bits, target, ..] => (target, f32::from_bits(bits as u32)),
            [SCALAR, op, a, b, target, ..] => (
                target,
                scalar(op, registers[a as usize], registers[b as usize])?,
            ),
            [GREATER, a, b, target, ..] => (
                target,
                if less(registers[b as usize], registers[a as usize])? {
                    1.0
                } else {
                    0.0
                },
            ),
            [FINISH, index, ..] => return Ok(registers[index as usize]),
            _ => return Err(invalid()),
        };
        registers[target as usize] = word;
        phase += 1;
    }
}

fn table_word(index: usize) -> Result<usize> {
    let width = size_of::<u16>();
    let bytes = TABLE
        .get(index * width..(index + 1) * width)
        .ok_or_else(invalid)?;
    Ok(usize::from(u16::from_le_bytes(
        bytes.try_into().map_err(|_| invalid())?,
    )))
}

struct GroupFit {
    scale: f32,
    codes: [usize; EIGHT],
    signs: [u8; FOUR],
}

struct Grid {
    compact: bool,
}

impl Grid {
    fn address(&self, key: u64, code: usize) -> Result<(usize, usize)> {
        match plan(Query::Tables, self.compact, key, code as u64, 0)? {
            [PAIR, a, b, ..] => Ok((a as usize, b as usize)),
            _ => Err(invalid()),
        }
    }
    fn levels(&self, code: usize) -> Result<[f32; FOUR]> {
        let (_, at) = self.address(0, code)?;
        let mut out = [0.0; FOUR];
        for (dst, &byte) in out.iter_mut().zip(
            GGML_QUANTIZED_TABLES
                .get(at..at + FOUR)
                .ok_or_else(invalid)?,
        ) {
            *dst =
                signed_word_f32(value(Query::CodeLevel, self.compact, byte.into(), 0, 0)? as i32)?;
        }
        Ok(out)
    }
    fn rounded(&self, inverse: f32, x: f32, maximum: usize) -> Result<u64> {
        let rounded = calculate(Program::RoundedLevel, &[inverse, x])?;
        value(
            Query::Digit,
            false,
            rounded.to_bits().into(),
            maximum as u64,
            0,
        )
    }
    fn project(&self, x: &[f32], weights: &[f32], inv: f32, scale: f32) -> Result<(usize, bool)> {
        let mut key = 0;
        for (lane, &word) in x.iter().enumerate() {
            key = value(
                Query::AppendLevel,
                false,
                key,
                lane as u64,
                self.rounded(inv, word, EIGHT - 1)?,
            )?;
        }
        let (offset, _) = self.address(key, 0)?;
        let start = table_word(1 + offset)?;
        let end = table_word(1 + offset + 1)?;
        let data = 1 + table_word(0)?;
        let mut best = table_word(data + start)?;
        if end == start + 1 {
            return Ok((best, true));
        }
        let mut minimum = f32::MAX;
        for index in start..end {
            let code = table_word(data + index)?;
            let mut cost = 0.0;
            for ((&raw, &weight), q) in x.iter().zip(weights).zip(self.levels(code)?) {
                cost = calculate(Program::Distance, &[cost, scale, q, raw, weight])?;
            }
            if less(cost, minimum)? {
                minimum = cost;
                best = code;
            }
        }
        Ok((best, false))
    }
    fn moments(
        &self,
        codes: &[usize; EIGHT],
        x: &[f32; WIDTH],
        weights: &[f32; WIDTH],
    ) -> Result<(f32, f32)> {
        let (mut xy, mut yy) = (0.0, 0.0);
        for (part, &code) in codes.iter().enumerate() {
            for (lane, q) in self.levels(code)?.into_iter().enumerate() {
                let i = part * FOUR + lane;
                xy = calculate(Program::Moment, &[xy, weights[i], x[i], q])?;
                yy = calculate(Program::Moment, &[yy, weights[i], q, q])?;
            }
        }
        Ok((xy, yy))
    }

    fn group(
        &self,
        input: &[f32],
        importance: Option<&[f32]>,
        variance: f32,
        trials: i32,
        threshold: f32,
    ) -> Result<Option<GroupFit>> {
        let mut x = [0.0; WIDTH];
        let mut weights = [0.0; WIDTH];
        let mut sqrt_weights = [0.0; WIDTH];
        let mut signs = [0_u8; FOUR];
        let multiply = scalars::two_u64();
        let negate = u64::from(scalars::six_u32());
        for (part, sign) in signs.iter_mut().enumerate() {
            let mut parity = false;
            let mut cheapest = 0;
            let mut min_cost = f32::MAX;
            for lane in 0..EIGHT {
                let i = part * EIGHT + lane;
                let raw = input[i];
                weights[i] = match importance {
                    Some(w) => calculate(Program::Weight, &[raw, variance, w[i]])?,
                    None => scalar(multiply, raw, raw)?,
                };
                sqrt_weights[i] = scalar(scalars::four_u64(), weights[i], 0.0)?;
                let negative = less(raw, 0.0)?;
                x[i] = if negative {
                    scalar(negate, raw, 0.0)?
                } else {
                    raw
                };
                if negative {
                    *sign |= value(Query::SignMask, false, lane as u64, 0, 0)? as u8;
                    parity = !parity;
                }
                if self.compact {
                    let cost = calculate(Program::SignCost, &[weights[i], raw])?;
                    if lane == 0 || less(cost, min_cost)? {
                        min_cost = cost;
                        cheapest = lane;
                    }
                }
            }
            if self.compact && parity {
                *sign ^= value(Query::SignMask, false, cheapest as u64, 0, 0)? as u8;
                let i = part * EIGHT + cheapest;
                x[i] = scalar(negate, x[i], 0.0)?;
            }
        }
        let mut peak = x[0];
        for &word in &x[1..] {
            if less(peak, word)? {
                peak = word;
            }
        }
        let mut codes = [0; EIGHT];
        if peak == 0.0 || less(peak, threshold)? {
            return Ok(None);
        }
        let mut scale = calculate(Program::InitialScale, &[peak])?;
        let mut best = 0.0;
        let mut on_grid = [self.compact; EIGHT];
        for trial in -trials..=trials {
            let inverse = calculate(Program::TrialInverse, &[peak, signed_word_f32(trial)?])?;
            let candidate_scale = calculate(Program::Reciprocal, &[inverse])?;
            let mut proposed = [0; EIGHT];
            let mut exact = [false; EIGHT];
            for (part, (xs, ws)) in x
                .chunks_exact(FOUR)
                .zip(sqrt_weights.chunks_exact(FOUR))
                .enumerate()
            {
                (proposed[part], exact[part]) = self.project(xs, ws, inverse, candidate_scale)?;
            }
            let (xy, yy) = self.moments(&proposed, &x, &weights)?;
            if less(0.0, yy)? && calculate(Program::Gain, &[xy, best, yy])? != 0.0 {
                scale = scalar(scalars::three_u64(), xy, yy)?;
                best = scalar(multiply, scale, xy)?;
                codes = proposed;
                on_grid = exact;
            }
        }
        if on_grid.contains(&false) && less(0.0, scale)? {
            let inverse = calculate(Program::Reciprocal, &[scale])?;
            for (part, (xs, ws)) in x
                .chunks_exact(FOUR)
                .zip(sqrt_weights.chunks_exact(FOUR))
                .enumerate()
            {
                if !self.compact || !on_grid[part] {
                    codes[part] = self.project(xs, ws, inverse, scale)?.0;
                }
            }
            let (xy, yy) = self.moments(&codes, &x, &weights)?;
            if less(0.0, yy)? {
                scale = scalar(scalars::three_u64(), xy, yy)?;
            }
        }
        if less(scale, 0.0)? {
            scale = scalar(negate, scale, 0.0)?;
            for sign in &mut signs {
                *sign = !*sign;
            }
        }
        Ok(Some(GroupFit {
            scale,
            codes,
            signs,
        }))
    }
}

/// Encode an importance-grid block with optional per-lane weighting.
/// An absent importance vector selects the unweighted GGML reference fit.
///
/// # Errors
/// Returns a typed error for an unsupported representation or extent, a nonfinite
/// input, a negative importance value, or an unrepresentable calculation. A
/// refused block preserves its complete destination.
pub fn encode_importance_weight_block(
    input: &[f32],
    importance: Option<&[f32]>,
    kind: PackedWeightEncoding,
    output: &mut [u8],
) -> Result<()> {
    let compact = match kind {
        PackedWeightEncoding::ImportanceThreeCompact => true,
        PackedWeightEncoding::ImportanceThree => false,
        _ => return Err(invalid()),
    };
    let grid = Grid { compact };
    let weights = importance.map_or(0, <[f32]>::len);
    let [GEOMETRY, groups, width, trials, threshold, _] = plan(
        Query::Geometry,
        grid.compact,
        input.len() as u64,
        weights as u64,
        output.len() as u64,
    )?
    else {
        return Err(invalid());
    };
    if importance.is_some_and(|w| w.len() != input.len())
        || !input
            .iter()
            .all(|v| verdant_affine_finite::run(v.to_bits().into()))
    {
        return Err(invalid());
    }
    for &weight in importance.unwrap_or_default() {
        if !verdant_affine_finite::run(weight.to_bits().into()) || less(weight, 0.0)? {
            return Err(invalid());
        }
    }
    let mut encoded = vec![0u8; output.len()];
    let mut variance = 0.0;
    if importance.is_some() {
        for &word in input {
            variance = calculate(Program::SumSquare, &[variance, word])?;
        }
        variance = calculate(Program::Variance, &[variance])?;
    }
    let mut scales = vec![0.0; groups as usize];
    let mut peak = 0.0;
    let mut emitted = 0;
    for (group, x) in input.chunks_exact(width as usize).enumerate() {
        let at = group * width as usize;
        let weights = importance.map(|w| &w[at..at + x.len()]);
        let Some(GroupFit {
            scale,
            codes,
            signs,
        }) = grid.group(
            x,
            weights,
            variance,
            trials as i32,
            f32::from_bits(threshold as u32),
        )?
        else {
            continue;
        };
        // GGML advances the full-layout low-byte and sign pointers only for nonzero groups.
        // Its high-bit and scale planes keep the original group indices.
        let packed_group = if grid.compact { group } else { emitted };
        emitted += 1;
        scales[group] = scale;
        if less(peak, scale)? {
            peak = scale;
        }
        for (k, code) in codes.into_iter().enumerate() {
            for part in 0..=usize::from(!grid.compact) {
                let plane_group = if part == 0 { packed_group } else { group };
                let index = (plane_group * EIGHT + k) as u64;
                let [FIELD, dst, shift, bits, ..] =
                    plan(Query::Emit, grid.compact, index, part as u64, code as u64)?
                else {
                    return Err(invalid());
                };
                encoded[dst as usize] |= (bits << shift) as u8;
            }
        }
        for (k, sign) in signs.into_iter().enumerate() {
            let [PAIR, dst, shift, ..] = plan(
                Query::SignField,
                grid.compact,
                packed_group as u64,
                k as u64,
                0,
            )?
            else {
                return Err(invalid());
            };
            if grid.compact {
                let dst = &mut encoded[dst as usize..dst as usize + size_of::<u32>()];
                let prior = u32::from_le_bytes((&*dst).try_into().map_err(|_| invalid())?);
                let bits = sign & (u8::MAX >> 1);
                dst.copy_from_slice(&(prior | (u32::from(bits) << shift)).to_le_bytes());
            } else {
                encoded[dst as usize] = sign;
            }
        }
    }
    if peak != 0.0 {
        let scale = calculate(Program::BaseScale, &[peak])?;
        let word = calculate(
            if grid.compact {
                Program::CompactScale
            } else {
                Program::FullScale
            },
            &[scale],
        )?;
        let half = convert_float_storage(0, 1, word.to_bits().into())
            .map_err(|_| invalid())?
            .ok_or_else(invalid)? as u16;
        encoded[..size_of::<u16>()].copy_from_slice(&half.to_ne_bytes());
        let inverse = calculate(Program::Reciprocal, &[scale])?;
        for (g, local) in scales.into_iter().enumerate() {
            let code = grid.rounded(inverse, local, scalars::sixteen_u64() as usize - 1)?;
            let [PAIR, dst, shift, ..] = plan(Query::ScaleField, grid.compact, g as u64, 0, 0)?
            else {
                return Err(invalid());
            };
            encoded[dst as usize] |= (code << shift) as u8;
        }
    } else {
        encoded.fill(0);
    }
    output.copy_from_slice(&encoded);
    Ok(())
}
