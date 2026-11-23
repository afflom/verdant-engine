//! Packed weight rows with a shared symmetric signed-byte activation format.
use super::{bitpacked_product, convert_float_storage, SymmetricQ8Block, TensorComputeError};

/// Contract encoded symmetric weight blocks with a prepared activation row.
/// Each block's signed integer dot is exact. Prism widens the two scales,
/// rounds their product, weights the dot and adds it to the running F32 sum.
///
/// # Errors
/// A partial weight block, incorrect lane count or mismatched row lengths
/// returns `ZeroOrMismatchedExtents` before either row is read.
pub fn dot_symmetric_q8_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, WeightEncoding::SignedByte)
}

/// Contract symmetric four-bit weights with prepared signed-byte activation blocks.
/// Each block rounds the signed dot times the weight scale before multiplying
/// by the activation scale. Low and high nibbles occupy separate lane halves.
///
/// # Errors
/// Rejects partial blocks, mismatched extents and lane counts other than 32.
pub fn dot_symmetric_q4_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, WeightEncoding::SignedFour)
}

/// Contract signed five-bit coefficients with independent half scales and signed-byte activations.
///
/// # Errors
/// Rejects incomplete or mismatched blocks and lane counts other than 32.
pub fn dot_symmetric_q5_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, WeightEncoding::SignedFive)
}

/// Contract nonlinear four-bit codebook indices with symmetric signed-byte activations.
///
/// # Errors
/// Rejects incomplete or mismatched blocks and lane counts other than 32.
pub fn dot_nonlinear_q4_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, WeightEncoding::NonlinearFour)
}

#[derive(Clone, Copy, PartialEq)]
enum WeightEncoding {
    SignedByte,
    SignedFour,
    SignedFive,
    NonlinearFour,
}

fn dot_symmetric_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
    encoding: WeightEncoding,
) -> Result<f32, TensorComputeError> {
    use verdant_packed_dot::{Adt0 as Packed, Adt1 as Step};
    let invalid = || TensorComputeError::ZeroOrMismatchedExtents {
        detail: "symmetric block rows do not have matching extents".into(),
    };
    let packed_step = |request| verdant_packed_dot::run(request).map_err(|_| invalid());
    let packed_word = |request| match packed_step(request)? {
        Step::C1(word) => Ok(word),
        _ => Err(invalid()),
    };
    let lanes = LANES as u64;
    let bytes = weights.len() as u64;
    let blocks = activations.len() as u64;
    let packed = encoding == WeightEncoding::SignedFour;
    let nonlinear = encoding == WeightEncoding::NonlinearFour;
    let compact = nonlinear || encoding == WeightEncoding::SignedFive;
    let width = if compact {
        let verdant_bitpacked_product::Adt1::C2(width) = verdant_bitpacked_product::run(
            verdant_bitpacked_product::Adt0::C2(nonlinear, lanes, bytes, blocks),
        )
        .map_err(|_| invalid())?
        else {
            return Err(invalid());
        };
        width
    } else if packed {
        packed_word(Packed::C0(lanes, bytes, blocks))?
    } else {
        verdant_symmetric_dot_layout::run(lanes, bytes, blocks)
    } as usize;
    if width == 0 {
        return Err(invalid());
    }
    let mut sum = 0;
    for (weight, activation) in weights.chunks_exact(width).zip(activations) {
        let (scale, codes) = weight
            .split_first_chunk::<{ std::mem::size_of::<u16>() }>()
            .ok_or_else(invalid)?;
        let signed = if compact {
            bitpacked_product::compact_coefficients(nonlinear, codes)?
        } else if packed {
            let mut signed = [0i8; LANES];
            let (low, high) = signed.split_at_mut(codes.len());
            for ((l, h), &byte) in low.iter_mut().zip(high).zip(codes) {
                let pair = packed_word(Packed::C1(u64::from(byte)))? as u16;
                let [first, second] = pair.to_le_bytes();
                *l = first as i8;
                *h = second as i8;
            }
            signed
        } else {
            let codes: &[u8; LANES] = codes.try_into().map_err(|_| invalid())?;
            codes.map(|byte| byte as i8)
        };
        let dot = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&signed),
            uor_matmul::as_alphabet_full(&activation.codes),
        );
        let negative = dot.is_negative();
        let magnitude = dot.unsigned_abs() as u64;
        let left = u64::from(u16::from_ne_bytes(*scale));
        let right = u64::from(activation.scale);
        sum = if packed {
            let widen = |half| {
                convert_float_storage(1, 0, half)
                    .ok()
                    .flatten()
                    .ok_or_else(invalid)
            };
            let (left, right) = (widen(left)?, widen(right)?);
            let mut current = verdant_count_f32::run(magnitude)
                .ok()
                .flatten()
                .ok_or_else(invalid)?;
            let mut phase = 0;
            loop {
                match packed_step(Packed::C2(phase, negative, sum, current, left, right))? {
                    Step::C2(op, a, b) => {
                        current = verdant_scalar_f32::run(op, a, b)
                            .ok()
                            .flatten()
                            .ok_or_else(invalid)?
                    }
                    Step::C1(value) => break value,
                    Step::C0 => return Err(invalid()),
                }
                phase += 1;
            }
        } else {
            verdant_symmetric_dot::run(sum, negative, magnitude, left, right)
                .ok()
                .flatten()
                .ok_or_else(|| TensorComputeError::InvalidKernelSpecification {
                    kernel_name: "SymmetricDot.accumulate".into(),
                })?
        };
    }
    Ok(f32::from_bits(sum as u32))
}
