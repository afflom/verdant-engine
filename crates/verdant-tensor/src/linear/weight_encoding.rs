//! Fixed-width block encoding for centered and affine weights.
use crate::{scalars, TensorComputeError};

const LANES: usize = scalars::thirty_two_u64() as usize;
const PAIRS: usize = scalars::sixteen_u64() as usize;
const HALF: usize = size_of::<u16>();
const PLANE: usize = size_of::<u32>();
use verdant_symmetric_q8::{run, Adt0 as Request, Adt1 as Step};

/// Representation of a 32-lane weight block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightBlockEncoding {
    /// Binary16 scale and one signed byte per lane.
    SymmetricEight,
    /// Binary16 scale and centered four-bit coefficients.
    CenteredFour,
    /// Binary16 scale, minimum and unsigned four-bit coefficients.
    AffineFour,
    /// Binary16 scale and centered five-bit coefficients.
    CenteredFive,
    /// Binary16 scale, minimum and unsigned five-bit coefficients.
    AffineFive,
}

/// Encode 32 binary32 lanes into native scale words and paired nibbles.
/// Five-bit representations store a native upper-bit plane before the nibbles.
///
/// # Errors
/// Returns `InvalidKernelSpecification` for wrong output length, nonfinite lanes
/// or an undefined coefficient conversion. An error preserves the output.
pub fn encode_weight_block(
    input: &[f32; LANES],
    kind: WeightBlockEncoding,
    output: &mut [u8],
) -> Result<(), TensorComputeError> {
    use WeightBlockEncoding::*;
    let affine = matches!(kind, AffineFour | AffineFive);
    let five = matches!(kind, CenteredFive | AffineFive);
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "SymmetricBlock.plan".into(),
    };
    if kind == SymmetricEight {
        let output: &mut [u8; LANES + HALF] = output.try_into().map_err(|_| invalid())?;
        let block = super::encode_symmetric_q8_block(input)?;
        output[..HALF].copy_from_slice(&block.scale.to_ne_bytes());
        output[HALF..].copy_from_slice(&block.codes.map(|code| code as u8));
        return Ok(());
    }
    let plan = |request| run(request).map_err(|_| invalid());
    let Step::C1(header) = plan(Request::C6(affine, five, output.len() as u64))? else {
        return Err(invalid());
    };
    let mut low = u64::from(f32::MAX.to_bits());
    let mut high = u64::from(f32::MIN.to_bits());
    let mut peak = 0;
    for &lane in input {
        let Step::C4(l, h, p) = plan(Request::C7(low, high, peak, lane.to_bits().into()))? else {
            return Err(invalid());
        };
        (low, high, peak) = (l, h, p);
    }
    let Step::C5(scale, minimum, inverse) = plan(Request::C8(affine, five, low, high, peak))?
    else {
        return Err(invalid());
    };
    let code = |lane: f32| match plan(Request::C9(
        affine,
        five,
        low,
        inverse,
        lane.to_bits().into(),
    ))? {
        Step::C1(word) => Ok(word),
        _ => Err(invalid()),
    };
    let mut nibbles = [0; PAIRS];
    let mut upper = 0u32;
    for (i, byte) in nibbles.iter_mut().enumerate() {
        let Step::C6(pair, mask) = plan(Request::C10(
            i as u64,
            code(input[i])?,
            code(input[i + PAIRS])?,
        ))?
        else {
            return Err(invalid());
        };
        *byte = pair as u8;
        upper |= mask as u32;
    }
    let (prefix, coefficients) = output.split_at_mut(header as usize);
    prefix[..HALF].copy_from_slice(&(scale as u16).to_ne_bytes());
    if affine {
        prefix[HALF..(HALF + HALF)].copy_from_slice(&(minimum as u16).to_ne_bytes());
    }
    if five {
        let start = prefix.len() - PLANE;
        prefix[start..].copy_from_slice(&upper.to_ne_bytes());
    }
    coefficients.copy_from_slice(&nibbles);
    Ok(())
}
