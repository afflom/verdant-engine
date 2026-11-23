//! Integer contractions over sign or two-bit weight coefficients.
use super::{evaluate_f32_scalar, grouped_affine::scaled_half, signed_word_f32, SymmetricQ8Block};
use crate::{scalars, TensorComputeError};
use verdant_bitpacked_product::{run, Adt0 as Request, Adt1 as Step};

const LANES: usize = scalars::thirty_two_u64() as usize;

fn invalid() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "BitpackedProduct".into(),
    }
}

fn grouped_sum(
    two_bit: bool,
    bytes: &[u8],
    slots: usize,
    activation: &SymmetricQ8Block<LANES>,
) -> Result<f32, TensorComputeError> {
    let mut coefficients = [0_i8; LANES];
    for (&byte, lanes) in bytes.iter().zip(coefficients.chunks_exact_mut(slots)) {
        for (lane, coefficient) in lanes.iter_mut().enumerate() {
            let Step::C2(biased) =
                run(Request::C1(two_bit, byte.into(), lane as u64)).map_err(|_| invalid())?
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
    scaled_half(activation.scale.to_ne_bytes(), value)
}

fn row(
    two_bit: bool,
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
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
            let term = grouped_sum(two_bit, fields, slots as usize, activation)?;
            inner = evaluate_f32_scalar(0, inner, term)?;
        }
        total = evaluate_f32_scalar(0, total, scaled_half(*scale, inner)?)?;
    }
    Ok(total)
}

/// Contract one-bit sign weights with blocks of symmetric signed-byte activations.
///
/// # Errors
/// Returns a typed error when weights and activation block counts disagree or a certified step fails.
pub fn dot_binary_sign_row(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    row(false, weights, activations)
}

/// Contract two-bit weights with blocks of symmetric signed-byte activations.
///
/// # Errors
/// Returns a typed error when weights and activation block counts disagree or a certified step fails.
pub fn dot_two_bit_row(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    row(true, weights, activations)
}
