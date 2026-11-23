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

/// Supported packed weight representations, including variable block lengths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackedWeightEncoding {
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
    /// Sign bits for 128 lanes with their mean absolute magnitude.
    BinarySign,
    /// Centered two-bit coefficients for 64 lanes with their maximum absolute magnitude.
    CenteredTwo,
}

impl From<WeightBlockEncoding> for PackedWeightEncoding {
    fn from(kind: WeightBlockEncoding) -> Self {
        match kind {
            WeightBlockEncoding::SymmetricEight => PackedWeightEncoding::SymmetricEight,
            WeightBlockEncoding::CenteredFour => PackedWeightEncoding::CenteredFour,
            WeightBlockEncoding::AffineFour => PackedWeightEncoding::AffineFour,
            WeightBlockEncoding::CenteredFive => PackedWeightEncoding::CenteredFive,
            WeightBlockEncoding::AffineFive => PackedWeightEncoding::AffineFive,
        }
    }
}

/// Encode a block into its native scale words and packed coefficients.
/// The representation selects the required lane count and destination width.
///
/// # Errors
/// Returns a typed error for mismatched extents, nonfinite input or an undefined
/// coefficient conversion. Errors preserve the destination bytes.
pub fn encode_weight_block<I: AsRef<[f32]> + ?Sized>(
    input: &I,
    kind: impl Into<PackedWeightEncoding>,
    output: &mut [u8],
) -> Result<(), TensorComputeError> {
    use PackedWeightEncoding::*;
    let input = input.as_ref();
    let kind = kind.into();
    if matches!(kind, BinarySign | CenteredTwo) {
        use verdant_bitpacked_product::{run, Adt0 as Request, Adt1 as Step};
        let two = kind == PackedWeightEncoding::CenteredTwo;
        let invalid = || TensorComputeError::InvalidKernelSpecification {
            kernel_name: "BitpackedProduct.encoding".into(),
        };
        let plan = |request| run(request).map_err(|_| invalid());
        let value = |request| match plan(request)? {
            Step::C2(word) => Ok(word),
            _ => Err(invalid()),
        };
        let slots = value(Request::C11(two, input.len() as u64, output.len() as u64))?;
        let mut peak = 0;
        for lane in input {
            peak = value(Request::C12(two, peak, lane.to_bits().into()))?;
        }
        let Step::C3(scale, inverse) = plan(Request::C13(two, peak))? else {
            return Err(invalid());
        };
        let mut packed = [0; PAIRS];
        for (byte, lanes) in packed.iter_mut().zip(input.chunks_exact(slots as usize)) {
            let mut prefix = 0;
            for (index, lane) in lanes.iter().enumerate() {
                prefix = value(Request::C14(
                    two,
                    index as u64,
                    prefix,
                    lane.to_bits().into(),
                    inverse,
                ))?;
            }
            *byte = prefix as u8;
        }
        let (header, body) = output.split_at_mut(HALF);
        header.copy_from_slice(&(scale as u16).to_ne_bytes());
        body.copy_from_slice(&packed);
        return Ok(());
    }
    let affine = matches!(kind, AffineFour | AffineFive);
    let five = matches!(kind, CenteredFive | AffineFive);
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "SymmetricBlock.plan".into(),
    };
    let input: &[f32; LANES] = input.try_into().map_err(|_| invalid())?;
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
