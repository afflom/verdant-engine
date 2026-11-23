//! Grouped affine four- and five-bit weight products.

use super::{convert_float_storage, evaluate_f32_scalar, signed_word_f32, GroupedQ8Block};
use crate::{scalars, TensorComputeError};
use verdant_grouped_affine::{run, Adt0 as Request, Adt1 as Step};

const ACCUMULATORS: usize = scalars::eight_u64() as usize;
const GROUP_WIDTH: usize = scalars::thirty_two_u64() as usize;
const HIGH_PLANE_START: usize = scalars::sixteen_u64() as usize;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GroupedAffineProduct".into(),
    }
}

fn plan(request: Request) -> Result<Step, TensorComputeError> {
    run(request).map_err(|_| invalid())
}

fn word(request: Request) -> Result<u64, TensorComputeError> {
    match plan(request)? {
        Step::C1(value) => Ok(value),
        _ => Err(invalid()),
    }
}

pub(super) fn scaled_half(
    bytes: [u8; size_of::<u16>()],
    base: f32,
) -> Result<f32, TensorComputeError> {
    let value = convert_float_storage(1, 0, u64::from(u16::from_ne_bytes(bytes)))
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?;
    evaluate_f32_scalar(scalars::two_u64(), f32::from_bits(value as u32), base)
}

fn blend(subtract: bool, prior: f32, scale: f32, dot: i32) -> Result<f32, TensorComputeError> {
    let term = signed_word_f32(dot)?;
    let Step::C4(product, combine) = plan(Request::C4(subtract))? else {
        return Err(invalid());
    };
    let contribution = evaluate_f32_scalar(product, scale, term)?;
    evaluate_f32_scalar(combine, prior, contribution)
}

fn integer_dots(
    high: bool,
    block: &[u8],
    activation: &GroupedQ8Block,
) -> Result<([i32; ACCUMULATORS], i32), TensorComputeError> {
    let mut coefficients = [[0_i16; GROUP_WIDTH]; ACCUMULATORS];
    let mut inputs = coefficients;
    let mut minima = activation.sums.map(|_| 0_i16);
    for (group, row) in activation.codes.chunks_exact(GROUP_WIDTH).enumerate() {
        let Step::C2(a, b, c) = plan(Request::C1(group as u64))? else {
            return Err(invalid());
        };
        let Step::C3(start, shift, plane, scale, minimum) = plan(Request::C2(
            high,
            group as u64,
            block[a as usize].into(),
            block[b as usize].into(),
            block[c as usize].into(),
        ))?
        else {
            return Err(invalid());
        };
        let min_count = minima.len() / ACCUMULATORS;
        minima[group * min_count..(group + 1) * min_count].fill(minimum as i16);
        for (lane, &sample) in row.iter().enumerate() {
            let index = lane % ACCUMULATORS;
            let column = group * (GROUP_WIDTH / ACCUMULATORS) + lane / ACCUMULATORS;
            coefficients[index][column] = word(Request::C3(
                block[start as usize + lane].into(),
                shift,
                block[HIGH_PLANE_START + lane].into(),
                plane,
                scale,
            ))? as i16;
            inputs[index][column] = i16::from(sample);
        }
    }
    let dot = |left: &[i16], right: &[i16]| {
        let value = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(left),
            uor_matmul::as_alphabet_full(right),
        );
        i32::try_from(value).map_err(|_| invalid())
    };
    let mut sums = [0_i32; ACCUMULATORS];
    for ((sum, lhs), rhs) in sums.iter_mut().zip(&coefficients).zip(&inputs) {
        *sum = dot(lhs, rhs)?;
    }
    Ok((sums, dot(&minima, &activation.sums)?))
}

fn grouped_row(
    high: bool,
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    let width = word(Request::C0(
        high,
        weights.len() as u64,
        activations.len() as u64,
    ))
    .map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
        detail: "grouped affine weights and activation block counts differ".into(),
    })? as usize;
    let mut sums = [0.0; ACCUMULATORS];
    let mut total = 0.0;
    for (block, activation) in weights.chunks_exact(width).zip(activations) {
        let [a, b, c, d] = *block.first_chunk().ok_or_else(invalid)?;
        let scale = scaled_half([a, b], activation.scale)?;
        let minimum_scale = scaled_half([c, d], activation.scale)?;
        let (products, offset) = integer_dots(high, block, activation)?;
        for (acc, product) in sums.iter_mut().zip(products) {
            *acc = blend(false, *acc, scale, product)?;
        }
        total = blend(true, total, minimum_scale, offset)?;
    }
    sums.into_iter()
        .try_fold(total, |sum, value| evaluate_f32_scalar(0, sum, value))
}

/// Contract a Q4_K weight row with prepared grouped signed-byte activations.
///
/// # Errors
/// Returns a typed error for inconsistent row sizes or an invalid certified instruction.
pub fn dot_grouped_q4_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    grouped_row(false, weights, activations)
}

/// Contract a Q5_K weight row with prepared grouped signed-byte activations.
///
/// # Errors
/// Returns a typed error for inconsistent row sizes or an invalid certified instruction.
pub fn dot_grouped_q5_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    grouped_row(true, weights, activations)
}
