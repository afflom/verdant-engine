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
    /// 256 ternary lanes packed as positional base-three digits.
    PackedTrits,
    /// 256 ternary lanes in interleaved two-bit planes.
    TwoBitTernary,
    /// Four-bit floating codebook with one exponent-only scale.
    ExponentOnlyFour,
    /// Four-bit floating codebook with unsigned E4M3 group scales.
    UnsignedScaleFour,
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
    if matches!(
        kind,
        BinarySign
            | CenteredTwo
            | PackedTrits
            | TwoBitTernary
            | ExponentOnlyFour
            | UnsignedScaleFour
    ) {
        use verdant_bitpacked_product::{run, Adt0 as Request, Adt1 as Step};
        let ternary = matches!(kind, PackedTrits | TwoBitTernary);
        let trits = kind == PackedTrits;
        let peak_scale = kind != BinarySign;
        let invalid = || TensorComputeError::InvalidKernelSpecification {
            kernel_name: "BitpackedProduct.encoding".into(),
        };
        let plan = |request| run(request).map_err(|_| invalid());
        let value = |request| match plan(request)? {
            Step::C2(word) => Ok(word),
            _ => Err(invalid()),
        };
        if matches!(kind, ExponentOnlyFour | UnsignedScaleFour) {
            let grouped = kind == UnsignedScaleFour;
            let Step::C1(groups, lanes, scale_operation, table) = plan(Request::C21(
                grouped,
                input.len() as u64,
                output.len() as u64,
            ))?
            else {
                return Err(invalid());
            };
            let mut staged = [0_u8; LANES + size_of::<u32>()];
            let (scales, codes) = staged[..output.len()].split_at_mut(groups as usize);
            for ((fields, header), body) in input
                .chunks_exact(lanes as usize)
                .zip(scales)
                .zip(codes.chunks_exact_mut((lanes / scalars::two_u64()) as usize))
            {
                let mut peak = 0;
                for lane in fields {
                    peak = value(Request::C12(true, peak, lane.to_bits().into()))?;
                }
                let byte = value(Request::C22(grouped, peak))?;
                *header = byte as u8;
                let scale = super::evaluate_small_float(scale_operation, byte)
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?;
                let mut candidates = [0; PAIRS];
                for (slot, candidate) in candidates.iter_mut().enumerate() {
                    let level = super::GGML_QUANTIZED_TABLES[table as usize + slot] as i8;
                    let level = super::signed_word_f32(i32::from(level))?;
                    *candidate = super::evaluate_f32_scalar(
                        scalars::two_u64(),
                        level,
                        f32::from_bits(scale as u32),
                    )?
                    .to_bits();
                }
                let nearest = |word: f32| -> Result<u64, TensorComputeError> {
                    let (mut best, mut error) = (0, u64::from(u32::MAX));
                    for (index, &candidate) in candidates.iter().enumerate() {
                        let Step::C5(next, distance) = plan(Request::C23(
                            word.to_bits().into(),
                            candidate.into(),
                            index as u64,
                            best,
                            error,
                        ))?
                        else {
                            return Err(invalid());
                        };
                        (best, error) = (next, distance);
                    }
                    Ok(best)
                };
                let (low, high) = fields.split_at(body.len());
                for ((dst, &a), &b) in body.iter_mut().zip(low).zip(high) {
                    *dst = value(Request::C24(nearest(a)?, nearest(b)?))? as u8;
                }
            }
            output.copy_from_slice(&staged[..output.len()]);
            return Ok(());
        }
        let layout = if ternary { Request::C15 } else { Request::C11 };
        let slots = value(layout(
            if ternary { trits } else { peak_scale },
            input.len() as u64,
            output.len() as u64,
        ))?;
        let mut peak = 0;
        for lane in input {
            peak = value(Request::C12(peak_scale, peak, lane.to_bits().into()))?;
        }
        let Step::C3(scale, inverse) = plan(Request::C13(peak_scale, peak))? else {
            return Err(invalid());
        };
        if ternary {
            let mut staged = [0; scalars::sixty_four_u64() as usize];
            let packed = staged.get_mut(..slots as usize).ok_or_else(invalid)?;
            for (slot, byte) in packed.iter_mut().enumerate() {
                let Step::C1(start, stride, count, factor) =
                    plan(Request::C16(trits, slot as u64))?
                else {
                    return Err(invalid());
                };
                let mut digits = 0;
                for position in 0..count {
                    let lane = input
                        .get((start + position * stride) as usize)
                        .ok_or_else(invalid)?;
                    digits = value(Request::C17(
                        trits,
                        position,
                        digits,
                        lane.to_bits().into(),
                        inverse,
                    ))?;
                }
                *byte = value(Request::C18(trits, factor, digits))? as u8;
            }
            let (body, trailer) = output.split_at_mut(packed.len());
            body.copy_from_slice(packed);
            trailer.copy_from_slice(&(scale as u16).to_ne_bytes());
            return Ok(());
        }
        let mut packed = [0; PAIRS];
        for (byte, lanes) in packed.iter_mut().zip(input.chunks_exact(slots as usize)) {
            let mut prefix = 0;
            for (index, lane) in lanes.iter().enumerate() {
                prefix = value(Request::C14(
                    peak_scale,
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
