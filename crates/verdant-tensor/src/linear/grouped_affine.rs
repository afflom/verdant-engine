//! Grouped signed, affine, and codebook weight products.

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

macro_rules! run_grouped_affine_row {
    ($high:expr, $signed:expr, $weights:expr, $activations:expr) => {{
        let high: bool = $high;
        let signed: bool = $signed;
        let weights: &[u8] = $weights;
        let activations: &[GroupedQ8Block] = $activations;
        let plan = |request: Request| run(request).map_err(|_| invalid());
        let word = |request: Request| match plan(request)? {
            Step::C1(value) => Ok(value),
            _ => Err(invalid()),
        };
        let scaled_half =
            |bytes: [u8; size_of::<u16>()], base: f32| -> Result<f32, TensorComputeError> {
                let value = convert_float_storage(1, 0, u64::from(u16::from_ne_bytes(bytes)))
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?;
                evaluate_f32_scalar(scalars::two_u64(), f32::from_bits(value as u32), base)
            };
        let blend =
            |subtract: bool, prior: f32, scale: f32, dot: i32| -> Result<f32, TensorComputeError> {
                let term = signed_word_f32(dot)?;
                let Step::C4(product, combine) = plan(Request::C4(subtract))? else {
                    return Err(invalid());
                };
                let contribution = evaluate_f32_scalar(product, scale, term)?;
                evaluate_f32_scalar(combine, prior, contribution)
            };
        let layout = if signed { Request::C9 } else { Request::C0 };
        let width =
            word(layout(high, weights.len() as u64, activations.len() as u64)).map_err(|_| {
                TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "grouped affine weights and activation block counts differ".into(),
                }
            })? as usize;
        let mut sums = [0.0; ACCUMULATORS];
        let mut total = 0.0;
        for (block, activation) in weights.chunks_exact(width).zip(activations) {
            let (scale_bytes, minimum_bytes) = if signed {
                (*block.last_chunk().ok_or_else(invalid)?, None)
            } else {
                let [a, b, c, d] = *block.first_chunk().ok_or_else(invalid)?;
                ([a, b], Some([c, d]))
            };
            let scale = scaled_half(scale_bytes, activation.scale)?;
            let mut coefficients = [[0_i16; GROUP_WIDTH]; ACCUMULATORS];
            let mut inputs = coefficients;
            let mut minima = activation.sums.map(|_| 0_i16);
            if signed {
                for (lane, &sample) in activation.codes.iter().enumerate() {
                    let Step::C8(lo, hi, scale_low, scale_high) =
                        plan(Request::C10(high, lane as u64))?
                    else {
                        return Err(invalid());
                    };
                    let [lo, hi, scale_low, scale_high] =
                        [lo, hi, scale_low, scale_high].map(|i| u64::from(block[i as usize]));
                    coefficients[lane % ACCUMULATORS][lane / ACCUMULATORS] = word(Request::C11(
                        high,
                        lane as u64,
                        lo,
                        hi,
                        scale_low,
                        scale_high,
                    ))?
                        as i16;
                    inputs[lane % ACCUMULATORS][lane / ACCUMULATORS] = i16::from(sample);
                }
            } else {
                for (group, row) in activation.codes.chunks_exact(GROUP_WIDTH).enumerate() {
                    let Step::C2(ga, gb, gc) = plan(Request::C1(group as u64))? else {
                        return Err(invalid());
                    };
                    let Step::C3(start, shift, plane, gscale, minimum) = plan(Request::C2(
                        high,
                        group as u64,
                        block[ga as usize].into(),
                        block[gb as usize].into(),
                        block[gc as usize].into(),
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
                            gscale,
                        ))? as i16;
                        inputs[index][column] = i16::from(sample);
                    }
                }
            }
            let dot = |left: &[i16], right: &[i16]| {
                let value = uor_matmul::dot_ref(
                    uor_matmul::as_alphabet_full(left),
                    uor_matmul::as_alphabet_full(right),
                );
                i32::try_from(value).map_err(|_| invalid())
            };
            let mut products = [0_i32; ACCUMULATORS];
            for ((sum, lhs), rhs) in products.iter_mut().zip(&coefficients).zip(&inputs) {
                *sum = dot(lhs, rhs)?;
            }
            for (acc, product) in sums.iter_mut().zip(products) {
                *acc = blend(false, *acc, scale, product)?;
            }
            if let Some(bytes) = minimum_bytes {
                total = blend(
                    true,
                    total,
                    scaled_half(bytes, activation.scale)?,
                    dot(&minima, &activation.sums)?,
                )?;
            }
        }
        sums.into_iter()
            .try_fold(total, |sum, value| evaluate_f32_scalar(0, sum, value))
    }};
}

/// Contract a Q4_K weight row with prepared grouped signed-byte activations.
///
/// # Errors
/// Returns a typed error for inconsistent row sizes or an invalid certified instruction.
pub fn dot_grouped_q4_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    run_grouped_affine_row!(false, false, weights, activations)
}

/// Contract a Q5_K weight row with prepared grouped signed-byte activations.
///
/// # Errors
/// Returns a typed error for inconsistent row sizes or an invalid certified instruction.
pub fn dot_grouped_q5_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    run_grouped_affine_row!(true, false, weights, activations)
}

/// Layout of eight-lane signed codewords in a grouped two-bit weight block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodebookEncoding {
    /// Eight-bit grid index and four packed seven-bit sign selectors.
    IndexedSigns,
    /// Nine-bit grid index with a seven-bit sign selector in the same word.
    ExtendedIndex,
    /// Ten-bit grid index with separate high bits and an explicit sign byte.
    ExplicitSigns,
    /// Four-lane eight-bit grids with packed seven-bit sign selectors.
    FourLaneIndexedSigns,
    /// Four-lane nine-bit grids with explicit signs.
    FourLaneExplicitSigns,
    /// Eight-lane signed grids with a shared half and per-group eighth shifts.
    ShiftedGrid,
    /// Eight-lane shifted grids with half-scale nibbles distributed across group words.
    DistributedScale,
}

/// Packed weight formats accepted by the grouped product.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroupedWeightEncoding {
    /// Two-bit affine coefficients and packed group minima.
    AffineTwo,
    /// Four-bit affine coefficients and packed group minima.
    AffineFour,
    /// Five-bit affine coefficients and packed group minima.
    AffineFive,
    /// Two-bit codebook with a packed index and signs.
    Codebook(CodebookEncoding),
    /// Three-bit signed coefficients and six-bit signed group scales.
    SignedThree,
    /// Six-bit signed coefficients and eight-bit signed group scales.
    SignedSix,
    /// Nonlinear four-bit codewords with signed six-bit group scales.
    NonlinearFour,
    /// Five packed trits per byte with a four-trit tail and a trailing scale.
    PackedTrits,
    /// Four two-bit ternary fields per byte with a trailing scale.
    TwoBitTernary,
}

impl From<CodebookEncoding> for GroupedWeightEncoding {
    fn from(value: CodebookEncoding) -> GroupedWeightEncoding {
        GroupedWeightEncoding::Codebook(value)
    }
}

/// Contract complete grouped weight blocks with prepared grouped Q8 activations.
///
/// # Errors
/// Rejects inconsistent weight and activation lengths or an invalid certified instruction.
pub fn dot_grouped_weight_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
    encoding: impl Into<GroupedWeightEncoding>,
) -> Result<f32, TensorComputeError> {
    const HALF_BYTES: u64 = size_of::<u16>() as u64;
    const WORD_BYTES: u64 = size_of::<u32>() as u64;
    let encoding = encoding.into();
    let bytes = weights.len() as u64;
    let blocks = activations.len() as u64;
    let layout = match encoding {
        GroupedWeightEncoding::AffineTwo => return super::dot_grouped_q2_row(weights, activations),
        GroupedWeightEncoding::AffineFour => return dot_grouped_q4_row(weights, activations),
        GroupedWeightEncoding::AffineFive => return dot_grouped_q5_row(weights, activations),
        GroupedWeightEncoding::Codebook(codebook) => Request::C5(codebook as u64, bytes, blocks),
        GroupedWeightEncoding::PackedTrits | GroupedWeightEncoding::TwoBitTernary => {
            use verdant_bitpacked_product::{run, Adt0 as Packed, Adt1 as Fields};
            let trits = encoding == GroupedWeightEncoding::PackedTrits;
            let plan = |request| run(request).map_err(|_| invalid());
            let value = |request| match plan(request)? {
                Fields::C2(word) => Ok(word),
                _ => Err(invalid()),
            };
            let width = value(Packed::C19(trits, bytes, blocks))?;
            let mut sum = 0;
            for (block, activation) in weights.chunks_exact(width as usize).zip(activations) {
                let (payload, scale) = block
                    .split_last_chunk::<{ size_of::<u16>() }>()
                    .ok_or_else(invalid)?;
                let mut coefficients = activation.codes.map(|_| 0_i8);
                for (slot, byte) in payload.iter().enumerate() {
                    let Fields::C1(start, stride, count, _) =
                        plan(Packed::C16(trits, slot as u64))?
                    else {
                        return Err(invalid());
                    };
                    let decoded = value(Packed::C20(trits, u64::from(*byte)))?.to_le_bytes();
                    for (position, code) in decoded[..count as usize].iter().enumerate() {
                        coefficients[start as usize + position * stride as usize] = *code as i8;
                    }
                }
                let dot = uor_matmul::dot_ref(
                    uor_matmul::as_alphabet_full(&coefficients),
                    uor_matmul::as_alphabet_full(&activation.codes),
                );
                let dot = signed_word_f32(i32::try_from(dot).map_err(|_| invalid())?)?;
                sum = value(Packed::C10(
                    sum,
                    dot.to_bits().into(),
                    activation.scale.to_bits().into(),
                    u16::from_ne_bytes(*scale).into(),
                ))?;
            }
            return Ok(f32::from_bits(sum as u32));
        }
        GroupedWeightEncoding::NonlinearFour => Request::C12(bytes, blocks),
        GroupedWeightEncoding::SignedThree | GroupedWeightEncoding::SignedSix => {
            return run_grouped_affine_row!(
                encoding == GroupedWeightEncoding::SignedSix,
                true,
                weights,
                activations
            )
        }
    };
    let query = |request| run(request).map_err(|_| invalid());
    let (width, normalization, grid_width, grid_unit) = match query(layout)? {
        Step::C5(width, normalization) => (width, normalization, ACCUMULATORS, None),
        Step::C9(width, normalization, lanes) => (width, normalization, lanes as usize, None),
        Step::C12(width, normalization, lanes, unit) => (
            width,
            normalization,
            lanes as usize,
            Some(f32::from_bits(unit as u32)),
        ),
        _ => {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: "codebook weights and activation block counts differ".into(),
            })
        }
    };
    let tables = super::GGML_QUANTIZED_TABLES;
    let mut total = 0.0;
    for (block, activation) in weights.chunks_exact(width as usize).zip(activations) {
        let field = |offset: u64, size: u64| -> Result<u64, TensorComputeError> {
            let tail = block.get(offset as usize..).ok_or_else(invalid)?;
            match size {
                1 => Ok(u64::from(*tail.first().ok_or_else(invalid)?)),
                HALF_BYTES => Ok(u64::from(u16::from_ne_bytes(
                    *tail.first_chunk().ok_or_else(invalid)?,
                ))),
                WORD_BYTES => Ok(u64::from(u32::from_ne_bytes(
                    *tail.first_chunk().ok_or_else(invalid)?,
                ))),
                _ => Err(invalid()),
            }
        };
        let stored_scale =
            if encoding == GroupedWeightEncoding::Codebook(CodebookEncoding::DistributedScale) {
                let Step::C8(a, b, c, d) = query(Request::C15)? else {
                    return Err(invalid());
                };
                let Step::C1(word) = query(Request::C16(
                    field(a, HALF_BYTES)?,
                    field(b, HALF_BYTES)?,
                    field(c, HALF_BYTES)?,
                    field(d, HALF_BYTES)?,
                ))?
                else {
                    return Err(invalid());
                };
                word
            } else {
                field(0, HALF_BYTES)?
            };
        let half = convert_float_storage(1, 0, stored_scale)
            .map_err(|_| invalid())?
            .ok_or_else(invalid)?;
        let scale = evaluate_f32_scalar(
            scalars::two_u64(),
            f32::from_bits(half as u32),
            activation.scale,
        )?;
        let GroupedWeightEncoding::Codebook(kind) = encoding else {
            for (group, inputs) in activation.codes.chunks_exact(GROUP_WIDTH).enumerate() {
                let group = group as u64;
                let Step::C2(low, high, start) = query(Request::C13(group))? else {
                    return Err(invalid());
                };
                let Step::C1(multiplier) =
                    query(Request::C14(group, field(low, 1)?, field(high, 1)?))?
                else {
                    return Err(invalid());
                };
                let mut coefficients = [0_i8; GROUP_WIDTH];
                let (lo, hi) = coefficients.split_at_mut(HIGH_PLANE_START);
                for (column, (a, b)) in lo.iter_mut().zip(hi).enumerate() {
                    use verdant_bitpacked_product::{run, Adt0 as Packed, Adt1 as Pair};
                    let byte = field(start + column as u64, 1)?;
                    let Pair::C4(first, second) =
                        run(Packed::C3(true, column as u64, byte, 0)).map_err(|_| invalid())?
                    else {
                        return Err(invalid());
                    };
                    *a = tables[first as usize] as i8;
                    *b = tables[second as usize] as i8;
                }
                let dot = uor_matmul::dot_ref(
                    uor_matmul::as_alphabet_full(&coefficients),
                    uor_matmul::as_alphabet_full(inputs),
                );
                let multiplier = signed_word_f32(i32::from(multiplier as i8))?;
                let scaled = evaluate_f32_scalar(scalars::two_u64(), scale, multiplier)?;
                let value = signed_word_f32(i32::try_from(dot).map_err(|_| invalid())?)?;
                let term = evaluate_f32_scalar(scalars::two_u64(), scaled, value)?;
                total = evaluate_f32_scalar(0, total, term)?;
            }
            continue;
        };
        let kind = kind as u64;
        let mut coefficients = activation.codes.map(|_| 0_i16);
        for (group, lanes) in coefficients.chunks_exact_mut(grid_width).enumerate() {
            let group = group as u64;
            let Step::C6(code, code_width, control, control_width, scale, signs) =
                query(Request::C6(kind, group))?
            else {
                return Err(invalid());
            };
            let lookup = query(Request::C7(
                kind,
                group,
                field(code, code_width)?,
                field(control, control_width)?,
                field(scale, 1)?,
                field(signs, 1)?,
            ))?;
            if let Step::C11(grid, scale, down) = lookup {
                for (lane, dst) in lanes.iter_mut().enumerate() {
                    let raw = tables[grid as usize + lane];
                    let Step::C1(word) = query(Request::C17(raw.into(), scale, down))? else {
                        return Err(invalid());
                    };
                    *dst = word as i16;
                }
                continue;
            }
            let (grid, indexed, mask, multiplier, sign_lane) = match lookup {
                Step::C7(grid, indexed, mask, scale) => (grid, indexed, mask, scale, 0),
                Step::C10(grid, indexed, mask, scale, start) => (grid, indexed, mask, scale, start),
                _ => return Err(invalid()),
            };
            let mask = if indexed {
                u64::from(tables[mask as usize])
            } else {
                mask
            };
            for (lane, dst) in lanes.iter_mut().enumerate() {
                let magnitude = tables[grid as usize + lane];
                let Step::C1(value) = query(Request::C8(
                    sign_lane + lane as u64,
                    mask,
                    magnitude.into(),
                    multiplier,
                ))?
                else {
                    return Err(invalid());
                };
                *dst = value as i16;
            }
        }
        let codes = activation.codes.map(i16::from);
        let product = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&coefficients),
            uor_matmul::as_alphabet_full(&codes),
        );
        let signed = signed_word_f32(i32::try_from(product).map_err(|_| invalid())?)?;
        let signed = match grid_unit {
            Some(unit) => evaluate_f32_scalar(scalars::two_u64(), signed, unit)?,
            None => signed,
        };
        let term = evaluate_f32_scalar(scalars::two_u64(), scale, signed)?;
        total = evaluate_f32_scalar(0, total, term)?;
    }
    evaluate_f32_scalar(
        scalars::two_u64(),
        f32::from_bits(normalization as u32),
        total,
    )
}
