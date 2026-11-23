//! Affine packed rows with signed-byte activation blocks.

use super::{signed_word_f32, AffineQ8Block, TensorComputeError};
use verdant_bitpacked_product::{run, Adt0 as Request, Adt1 as Step};

const LANES: usize = crate::scalars::thirty_two_u64() as usize;

/// Unsigned weight lanes with a binary16 scale and minimum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AffineWeightEncoding {
    /// Four bits per weight.
    FourBit,
    /// Four low bits and an independent plane of high bits.
    FiveBit,
}

/// Contract packed affine weights with prepared signed-byte activations.
///
/// # Errors
/// Rejects a partial weight block or a mismatched activation count before computation.
pub fn dot_affine_weight_row(
    weights: &[u8],
    activations: &[AffineQ8Block<LANES>],
    encoding: AffineWeightEncoding,
) -> Result<f32, TensorComputeError> {
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "BitpackedProduct.affineSum".into(),
    };
    let plan = |request| run(request).map_err(|_| invalid());
    let five = encoding == AffineWeightEncoding::FiveBit;
    let Step::C2(width) = plan(Request::C4(
        five,
        weights.len() as u64,
        activations.len() as u64,
    ))?
    else {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "affine weights and activation block counts differ".into(),
        });
    };
    let mut sum = 0;
    for (weight, activation) in weights.chunks_exact(width as usize).zip(activations) {
        let ([a, b, c, d], data) = weight.split_first_chunk().ok_or_else(invalid)?;
        let (flags, pairs) = if five {
            let (plane, pairs) = data.split_first_chunk().ok_or_else(invalid)?;
            (u32::from_ne_bytes(*plane), pairs)
        } else {
            (0, data)
        };
        let mut coefficients = [0_i8; LANES];
        let (low, high) = coefficients.split_at_mut(crate::scalars::sixteen_u64() as usize);
        for (column, ((l, h), &byte)) in low.iter_mut().zip(high).zip(pairs).enumerate() {
            let Step::C3(x, y) = plan(Request::C5(column as u64, byte.into(), flags.into()))?
            else {
                return Err(invalid());
            };
            (*l, *h) = (x as i8, y as i8);
        }
        let dot = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&coefficients),
            uor_matmul::as_alphabet_full(&activation.symmetric.codes),
        );
        let signed = signed_word_f32(dot as i32)?.to_bits();
        let Step::C2(next) = plan(Request::C6(
            sum,
            signed.into(),
            u16::from_ne_bytes([*a, *b]).into(),
            activation.symmetric.scale.into(),
            u16::from_ne_bytes([*c, *d]).into(),
            activation.scaled_sum.into(),
        ))?
        else {
            return Err(invalid());
        };
        sum = next;
    }
    Ok(f32::from_bits(sum as u32))
}
