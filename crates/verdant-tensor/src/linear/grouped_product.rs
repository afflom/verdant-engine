//! Affine two-bit weight products with grouped signed-byte activations.

use super::{convert_float_storage, evaluate_f32_scalar, signed_word_f32, GroupedQ8Block};
use crate::TensorComputeError;
use verdant_grouped_dot::{run, Adt0 as Request, Adt1 as Step};

/// Contract a row of encoded Q2_K weights with a prepared Q8_K activation row.
///
/// # Errors
/// Returns a typed failure when weight and activation block counts differ.
pub fn dot_grouped_q2_row(
    weights: &[u8],
    activations: &[GroupedQ8Block],
) -> Result<f32, TensorComputeError> {
    let invalid = || TensorComputeError::ZeroOrMismatchedExtents {
        detail: "grouped two-bit weights and activation blocks do not match".into(),
    };
    let plan = |request| run(request).map_err(|_| invalid());
    let Step::C1(width) = plan(Request::C0(weights.len() as u64, activations.len() as u64))? else {
        return Err(invalid());
    };
    let widen = |half: u16| {
        convert_float_storage(1, 0, u64::from(half))
            .map_err(|_| invalid())?
            .map(|bits| f32::from_bits(bits as u32))
            .ok_or_else(invalid)
    };
    let mut sum = 0.0;
    for (weight, activation) in weights.chunks_exact(width as usize).zip(activations) {
        let mut coefficients = activation.codes.map(|_| 0i8);
        let mut minimums = activation.sums.map(|_| 0i16);
        let group_width = coefficients.len() / minimums.len();
        for (i, (codes, min)) in coefficients
            .chunks_exact_mut(group_width)
            .zip(&mut minimums)
            .enumerate()
        {
            let Step::C2(start, divisor, scale, minimum) =
                plan(Request::C1(i as u64, weight[i].into()))?
            else {
                return Err(invalid());
            };
            *min = minimum as i16;
            for (&byte, code) in weight[start as usize..].iter().zip(codes) {
                let Step::C1(value) = plan(Request::C2(byte.into(), divisor, scale))? else {
                    return Err(invalid());
                };
                *code = value as i8;
            }
        }
        let dot = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&coefficients),
            uor_matmul::as_alphabet_full(&activation.codes),
        );
        let offset = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&minimums),
            uor_matmul::as_alphabet_full(&activation.sums),
        );
        let &[a, b, c, d] = weight.last_chunk().ok_or_else(invalid)?;
        let mut values = [
            sum,
            activation.scale,
            widen(u16::from_ne_bytes([a, b]))?,
            widen(u16::from_ne_bytes([c, d]))?,
            signed_word_f32(i32::try_from(dot).map_err(|_| invalid())?)?,
            signed_word_f32(i32::try_from(offset).map_err(|_| invalid())?)?,
        ];
        for phase in 0.. {
            match plan(Request::C3(phase))? {
                Step::C3(op, left, right, target) => {
                    values[target as usize] =
                        evaluate_f32_scalar(op, values[left as usize], values[right as usize])?
                }
                Step::C4(index) => {
                    sum = values[index as usize];
                    break;
                }
                _ => return Err(invalid()),
            }
        }
    }
    Ok(sum)
}
