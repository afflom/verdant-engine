//! Integer contractions and coefficient decoding for packed weight rows.
use super::{convert_float_storage, evaluate_f32_scalar, signed_word_f32, SymmetricQ8Block};
use crate::{scalars, TensorComputeError};
use verdant_bitpacked_product::{run, Adt0 as Request, Adt1 as Step};

const LANES: usize = scalars::thirty_two_u64() as usize;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "BitpackedProduct".into(),
    }
}

macro_rules! run_bitpacked_row {
    ($two_bit:expr, $weights:expr, $activations:expr) => {{
        let two_bit: bool = $two_bit;
        let weights: &[u8] = $weights;
        let activations: &[SymmetricQ8Block<LANES>] = $activations;
        let scaled_half =
            |bytes: [u8; size_of::<u16>()], base: f32| -> Result<f32, TensorComputeError> {
                let value = convert_float_storage(1, 0, u64::from(u16::from_ne_bytes(bytes)))
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?;
                evaluate_f32_scalar(scalars::two_u64(), f32::from_bits(value as u32), base)
            };
        let Step::C1(width, groups, bytes_per_group, slots) = run(Request::C0(
            two_bit,
            weights.len() as u64,
            activations.len() as u64,
        ))
        .map_err(|_| invalid())?
        else {
            return Err(TensorComputeError::ZeroOrMismatchedExtents {
                detail: "bit-packed weights and activation block counts differ".into(),
            });
        };
        let mut total = 0.0;
        for (block, group) in weights
            .chunks_exact(width as usize)
            .zip(activations.chunks_exact(groups as usize))
        {
            let (scale, packed) = block
                .split_first_chunk::<{ size_of::<u16>() }>()
                .ok_or_else(invalid)?;
            let mut inner = 0.0;
            for (fields, activation) in packed.chunks_exact(bytes_per_group as usize).zip(group) {
                let mut coefficients = [0_i8; LANES];
                for (&byte, lanes) in fields
                    .iter()
                    .zip(coefficients.chunks_exact_mut(slots as usize))
                {
                    for (lane, coefficient) in lanes.iter_mut().enumerate() {
                        let Step::C2(biased) = run(Request::C1(two_bit, byte.into(), lane as u64))
                            .map_err(|_| invalid())?
                        else {
                            return Err(invalid());
                        };
                        *coefficient = biased as i8 - 1;
                    }
                }
                let dot = uor_matmul::dot_ref(
                    uor_matmul::as_alphabet_full(&coefficients),
                    uor_matmul::as_alphabet_full(&activation.codes),
                );
                let value = signed_word_f32(i32::try_from(dot).map_err(|_| invalid())?)?;
                let term = scaled_half(activation.scale.to_ne_bytes(), value)?;
                inner = evaluate_f32_scalar(0, inner, term)?;
            }
            total = evaluate_f32_scalar(0, total, scaled_half(*scale, inner)?)?;
        }
        Ok(total)
    }};
}

/// Contract one-bit sign weights with blocks of symmetric signed-byte activations.
///
/// # Errors
/// Returns a typed error when weights and activation block counts disagree or a certified step fails.
pub fn dot_binary_sign_row(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    run_bitpacked_row!(false, weights, activations)
}

/// Contract two-bit weights with blocks of symmetric signed-byte activations.
///
/// # Errors
/// Returns a typed error when weights and activation block counts disagree or a certified step fails.
pub fn dot_two_bit_row(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    run_bitpacked_row!(true, weights, activations)
}

/// Scale layout for four-bit E2M1 coefficients paired with signed-byte activations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFloatEncoding {
    /// One E8M0 scale for all 32 coefficients.
    ExponentOnly,
    /// One unsigned E4M3 scale per 16 coefficients, with four groups in a block.
    GroupedE4M3,
}

/// Contract complete block-float rows with pre-encoded symmetric activation blocks.
///
/// # Errors
/// Rejects partial weight blocks or an activation count that differs from the encoded width.
pub fn dot_block_float_row(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
    encoding: BlockFloatEncoding,
) -> Result<f32, TensorComputeError> {
    let grouped = encoding == BlockFloatEncoding::GroupedE4M3;
    let query = |request| run(request).map_err(|_| invalid());
    let Step::C1(width, groups, lanes, blocks) = query(Request::C8(
        grouped,
        weights.len() as u64,
        activations.len() as u64,
    ))?
    else {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "block-float weight and activation counts differ".into(),
        });
    };
    let lanes = lanes as usize;
    let mut coefficients = [0_i8; LANES];
    let scale_operation = if grouped { scalars::two_u64() } else { 1 };
    let mut sum = 0;
    for (weight, acts) in weights
        .chunks_exact(width as usize)
        .zip(activations.chunks_exact(blocks as usize))
    {
        let (scales, packed) = weight.split_at(groups as usize);
        let half = lanes / scalars::two_u64() as usize;
        for (g, (&scale, pairs)) in scales.iter().zip(packed.chunks_exact(half)).enumerate() {
            let (low, high) = coefficients[..lanes].split_at_mut(half);
            for ((x, y), &byte) in low.iter_mut().zip(high.iter_mut()).zip(pairs) {
                let Step::C4(a, b) = query(Request::C9(byte.into()))? else {
                    return Err(invalid());
                };
                *x = super::GGML_QUANTIZED_TABLES[a as usize] as i8;
                *y = super::GGML_QUANTIZED_TABLES[b as usize] as i8;
            }
            let start = g * lanes;
            let activation = &acts[start / LANES];
            let codes = &activation.codes[start % LANES..][..lanes];
            let integer = uor_matmul::dot_ref(
                uor_matmul::as_alphabet_full(&coefficients[..lanes]),
                uor_matmul::as_alphabet_full(codes),
            );
            let signed = signed_word_f32(integer as i32)?.to_bits();
            let scale_word = super::evaluate_small_float(scale_operation, u64::from(scale))
                .map_err(|_| invalid())?
                .ok_or_else(invalid)?;
            let Step::C2(next) = query(Request::C10(
                sum,
                signed.into(),
                scale_word,
                activation.scale.into(),
            ))?
            else {
                return Err(invalid());
            };
            sum = next;
        }
    }
    Ok(f32::from_bits(sum as u32))
}
