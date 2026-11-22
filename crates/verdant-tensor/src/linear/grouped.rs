//! Signed activation blocks used by grouped quantized row products.

use crate::TensorComputeError;
use verdant_grouped_q8::{run, Adt0 as Request, Adt1 as Step};

// The encoding arranges a square of sixteen groups by sixteen lanes.
const GROUP_WIDTH: usize = crate::scalars::sixteen_u64() as usize;
const BLOCK_LANES: usize = GROUP_WIDTH * GROUP_WIDTH;

/// Native Q8_K block with a binary32 scale and signed partial sums.
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupedQ8Block {
    /// Shared scale used to widen the coefficients.
    pub scale: f32,
    /// Quantized coefficients in input order.
    pub codes: [i8; BLOCK_LANES],
    /// Sum of each consecutive group of sixteen coefficients.
    pub sums: [i16; GROUP_WIDTH],
}

/// Encode a complete block using the first signed absolute maximum and nearest-even rounding.
///
/// # Errors
/// Returns a typed failure for nonfinite lanes or an unrepresentable inverse scale.
pub fn encode_grouped_q8_block(
    input: &[f32; BLOCK_LANES],
) -> Result<GroupedQ8Block, TensorComputeError> {
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "GroupedBlock.plan".to_owned(),
    };
    let plan = |request| run(request).map_err(|_| invalid());
    let value = |step| match step {
        Step::C1(word) => Ok(word),
        Step::C3(operation, a, b) => verdant_scalar_f32::run(operation, a, b)
            .map_err(|_| invalid())?
            .ok_or_else(invalid),
        _ => Err(invalid()),
    };
    let mut maximum = 0;
    for lane in input {
        maximum = value(plan(Request::C0(maximum, lane.to_bits().into()))?)?;
    }
    let mut block = GroupedQ8Block {
        scale: 0.0,
        codes: [0; BLOCK_LANES],
        sums: [0; GROUP_WIDTH],
    };
    let step = plan(Request::C1(maximum))?;
    if matches!(step, Step::C2) {
        return Ok(block);
    }
    let inverse = value(step)?;
    block.scale = f32::from_bits(value(plan(Request::C2(inverse))?)? as u32);
    for ((lanes, codes), sum) in input
        .chunks_exact(GROUP_WIDTH)
        .zip(block.codes.chunks_exact_mut(GROUP_WIDTH))
        .zip(&mut block.sums)
    {
        let mut accumulator = 0;
        for (lane, code) in lanes.iter().zip(codes) {
            let scaled = value(plan(Request::C3(lane.to_bits().into(), inverse))?)?;
            let byte = value(plan(Request::C4(scaled))?)?;
            *code = byte as i8;
            accumulator = value(plan(Request::C5(accumulator, byte))?)?;
        }
        *sum = accumulator as i16;
    }
    Ok(block)
}
