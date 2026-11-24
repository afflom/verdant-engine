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
    /// Four-lane IQ3_XXS codebook with seven stored sign bits per eight lanes.
    ImportanceThreeCompact,
    /// Four-lane IQ3_S codebook with an independent sign for every lane.
    ImportanceThree,
    /// Eight-lane two-bit codes, full signs and 16 independently fitted scales.
    ImportanceTwo,
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
    /// Signed six-bit coefficients with per-group multipliers and a binary16 block scale.
    GroupedSignedSix,
    /// Sixteen signed three-bit groups with packed six-bit scales.
    GroupedSignedThree,
    /// A nonlinear four-bit codebook and a single binary16 scale.
    NonlinearFour,
    /// Eight nonlinear codebook groups with packed six-bit scale multipliers.
    GroupedNonlinearFour,
    /// Sixteen groups of unsigned two-bit weights with scale and minimum.
    GroupedAffineTwo,
    /// Eight groups of unsigned four-bit weights with scale and minimum.
    GroupedAffineFour,
    /// Eight groups of unsigned five-bit weights with scale and minimum.
    GroupedAffineFive,
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
        ImportanceThreeCompact | ImportanceThree | ImportanceTwo
    ) {
        return super::encode_importance_weight_block(input, None, kind, output);
    }
    let affine = match kind {
        GroupedAffineTwo => Some(scalars::two_u64()),
        GroupedAffineFour => Some(scalars::four_u64()),
        GroupedAffineFive => Some(u64::from(scalars::five_u32())),
        _ => None,
    };
    if let Some(bits) = affine {
        use super::{
            convert_float_storage, encode_small_integer, evaluate_encoded_word, evaluate_f32_scalar,
        };
        use verdant_affine_fit::{Adt0 as Program, Adt1 as Instruction};
        use verdant_affine_geometry::Adt0 as Format;

        const AFFINE_REGISTERS: usize = scalars::eight_u64() as usize + 1;
        const AFFINE_GROUPS: usize = scalars::sixteen_u64() as usize;
        const AFFINE_WIDTH: usize = scalars::thirty_two_u64() as usize;
        const AFFINE_LANES: usize = AFFINE_WIDTH * scalars::eight_u64() as usize;

        macro_rules! invalid {
            () => {
                TensorComputeError::InvalidKernelSpecification {
                    kernel_name: "GroupedAffineEncoding".into(),
                }
            };
        }
        macro_rules! less {
            ($a:expr, $b:expr) => {{
                let cmp = match evaluate_encoded_word(0, $a, $b) {
                    Ok(Some(v)) => v,
                    _ => return Err(invalid!()),
                };
                cmp == 0
            }};
        }
        macro_rules! negate {
            ($x:expr) => {{
                let v = evaluate_f32_scalar(
                    u64::from(scalars::six_u32()),
                    f32::from_bits(($x) as u32),
                    0.0,
                )?;
                u64::from(v.to_bits())
            }};
        }
        macro_rules! calc {
            ($prog:expr, $inp:expr) => {{
                let program: Program = $prog;
                let init_slice: &[u64] = $inp;
                let mut registers = [0; AFFINE_REGISTERS];
                match registers.get_mut(..init_slice.len()) {
                    Some(prefix) => prefix.copy_from_slice(init_slice),
                    None => return Err(invalid!()),
                }
                let mut phase = 0_u64;
                loop {
                    macro_rules! reg {
                        ($slot:expr) => {
                            match registers.get(($slot) as usize) {
                                Some(&v) => v,
                                None => return Err(invalid!()),
                            }
                        };
                    }
                    let instruction = verdant_affine_fit::run(program.clone(), bits, phase);
                    let (dst, result) = match instruction {
                        Instruction::C0(op, a, b, dst) => {
                            let value = evaluate_f32_scalar(
                                op,
                                f32::from_bits(reg!(a) as u32),
                                f32::from_bits(reg!(b) as u32),
                            )?;
                            (dst, u64::from(value.to_bits()))
                        }
                        Instruction::C1(value, dst) => (dst, value),
                        Instruction::C2(a, dst) => {
                            let enc = match encode_small_integer(reg!(a), 0) {
                                Ok(Some(v)) => v,
                                Ok(None) | Err(_) => return Err(invalid!()),
                            };
                            (dst, enc)
                        }
                        Instruction::C3(a, limit, bias, dst) => (
                            dst,
                            verdant_affine_quantized::run(reg!(a), reg!(limit), reg!(bias)),
                        ),
                        Instruction::C4(a, dst) => {
                            let half = match convert_float_storage(0, 1, reg!(a)) {
                                Ok(Some(v)) => v,
                                Ok(None) | Err(_) => return Err(invalid!()),
                            };
                            (dst, half)
                        }
                        Instruction::C5(a, dst) => {
                            let wide = match convert_float_storage(1, 0, reg!(a)) {
                                Ok(Some(v)) => v,
                                Ok(None) | Err(_) => return Err(invalid!()),
                            };
                            (dst, wide)
                        }
                        Instruction::C6 => break registers,
                    };
                    match registers.get_mut(dst as usize) {
                        Some(slot) => *slot = result,
                        None => return Err(invalid!()),
                    }
                    phase += 1;
                }
            }};
        }

        let Format::C1(width, level_max, max_code, trials) =
            verdant_affine_geometry::run(bits, input.len() as u64, output.len() as u64)
        else {
            return Err(invalid!());
        };
        let width = width as usize;

        macro_rules! lane_code {
            ($x:expr, $min:expr, $inv:expr) => {
                calc!(Program::C6, &[$x, $min, $inv, level_max])[0]
            };
        }
        macro_rules! fit_error {
            ($vals:expr, $wts:expr, $cds:expr, $sc:expr, $mn:expr) => {{
                let mut err_acc = 0;
                for ((&vx, &vw), &vlevel) in ($vals).iter().zip($wts).zip($cds) {
                    let [_, _, _, _, _, sum, ..] =
                        calc!(Program::C11, &[vx, vw, vlevel, $sc, $mn, err_acc]);
                    err_acc = sum;
                }
                err_acc
            }};
        }
        macro_rules! fit_scale {
            ($largest:expr) => {{
                let largest_val = $largest;
                if !less!(0, largest_val) {
                    (0_u64, 0_u64, 0_u64)
                } else {
                    let [_, inverse, half, expanded, ..] =
                        calc!(Program::C12, &[largest_val, max_code]);
                    (inverse, half, expanded)
                }
            }};
        }
        macro_rules! fit_assign {
            ($val:expr, $inv:expr, $base:expr) => {{
                let [code, ..] = calc!(Program::C13, &[$val, $inv, max_code]);
                let [local, ..] = calc!(Program::C14, &[$base, code]);
                (code, local)
            }};
        }

        let mut values = [0; AFFINE_LANES];
        for (slot, x) in values.iter_mut().zip(input) {
            *slot = u64::from(x.to_bits());
            if !verdant_affine_finite::run(*slot) {
                return Err(invalid!());
            }
        }
        let mut levels = [0; AFFINE_LANES];
        let mut group_scales = [0; AFFINE_GROUPS];
        let mut group_mins = [0; AFFINE_GROUPS];
        let (mut largest_scale, mut largest_min) = (0, 0);
        for (((xs, codes), scale_out), min_out) in values
            .chunks_exact(width)
            .zip(levels.chunks_exact_mut(width))
            .zip(&mut group_scales)
            .zip(&mut group_mins)
        {
            let mut minimum = xs[0];
            let mut maximum = minimum;
            let mut sum_squares = 0;
            for &x in xs {
                if less!(x, minimum) {
                    minimum = x;
                }
                if less!(maximum, x) {
                    maximum = x;
                }
                if bits != scalars::two_u64() {
                    sum_squares = calc!(Program::C0, &[x, sum_squares])[1];
                }
            }
            let rms = if bits == scalars::two_u64() {
                0
            } else {
                calc!(Program::C1, &[sum_squares])[0]
            };
            if less!(0, minimum) {
                minimum = 0;
            }
            let (grp_scale, grp_min) = if !less!(minimum, maximum) {
                codes.fill(0);
                (0, negate!(minimum))
            } else {
                let mut weights = [0; AFFINE_WIDTH];
                let mut sum_w = 0;
                let mut sum_x = 0;
                for (&x, w) in xs.iter().zip(&mut weights) {
                    *w = calc!(Program::C2, &[x, rms])[0];
                    let [_, _, next_w, next_x, ..] = calc!(Program::C3, &[x, *w, sum_w, sum_x]);
                    (sum_w, sum_x) = (next_w, next_x);
                }
                let [inverse, mut scale, ..] = calc!(Program::C4, &[maximum, minimum, level_max]);
                for (&x, code) in xs.iter().zip(codes.iter_mut()) {
                    *code = lane_code!(x, minimum, inverse);
                }
                let mut best = fit_error!(xs, &weights, &*codes, scale, minimum);
                let mut candidate = [0; AFFINE_WIDTH];
                for trial in 0..trials {
                    let [trial_inv, ..] = calc!(Program::C5, &[maximum, minimum, trial, level_max]);
                    let (mut sum_l, mut sum_l2, mut sum_xl) = (0, 0, 0);
                    for ((&x, &w), slot) in xs.iter().zip(&weights).zip(&mut candidate) {
                        *slot = lane_code!(x, minimum, trial_inv);
                        let [_, _, _, l, ll, xl, ..] =
                            calc!(Program::C7, &[x, w, *slot, sum_l, sum_l2, sum_xl]);
                        (sum_l, sum_l2, sum_xl) = (l, ll, xl);
                    }
                    let [det, ..] = calc!(Program::C8, &[sum_w, sum_l, sum_l2]);
                    if !less!(0, det) {
                        continue;
                    }
                    let [_, _, _, _, _, _, mut slope, mut intercept, ..] =
                        calc!(Program::C9, &[sum_w, sum_x, sum_l, sum_l2, sum_xl, det]);
                    if less!(0, intercept) {
                        intercept = 0;
                        slope = calc!(Program::C10, &[sum_xl, sum_l2])[0];
                    }
                    let err = fit_error!(xs, &weights, &candidate, slope, intercept);
                    if less!(err, best) {
                        (best, scale, minimum) = (err, slope, intercept);
                        codes.copy_from_slice(&candidate[..codes.len()]);
                    }
                }
                (scale, negate!(minimum))
            };
            (*scale_out, *min_out) = (grp_scale, grp_min);
            if less!(largest_scale, *scale_out) {
                largest_scale = *scale_out;
            }
            if less!(largest_min, *min_out) {
                largest_min = *min_out;
            }
        }
        let (iscale, half_scale, scale) = fit_scale!(largest_scale);
        let (imin, half_min, minimum) = fit_scale!(largest_min);
        for (((xs, codes), sc), mn) in values
            .chunks_exact(width)
            .zip(levels.chunks_exact_mut(width))
            .zip(&mut group_scales)
            .zip(&mut group_mins)
        {
            let (code, unit) = fit_assign!(*sc, iscale, scale);
            *sc = code;
            let (code, offset) = fit_assign!(*mn, imin, minimum);
            *mn = code;
            if less!(0, unit) {
                for (&x, c) in xs.iter().zip(codes) {
                    *c = calc!(Program::C15, &[x, unit, offset, level_max])[0];
                }
            }
        }
        let banks: [&[u64]; scalars::five_u32() as usize] = [
            &levels,
            &group_scales,
            &group_mins,
            std::slice::from_ref(&half_scale),
            std::slice::from_ref(&half_min),
        ];
        let mut packet = [0; AFFINE_LANES];
        for (byte, out) in packet[..output.len()].iter_mut().enumerate() {
            let mut packed = 0;
            let mut phase = 0_u64;
            loop {
                let step = match verdant_affine_weight_fields::run(bits, byte as u64, phase) {
                    Ok(s) => s,
                    Err(_) => return Err(invalid!()),
                };
                match step {
                    verdant_affine_weight_fields::Adt0::C1 => break,
                    verdant_affine_weight_fields::Adt0::C2(bank, slot, divisor, radix, place) => {
                        let value = match banks.get(bank as usize) {
                            Some(b) => match b.get(slot as usize) {
                                Some(&v) => v,
                                None => return Err(invalid!()),
                            },
                            None => return Err(invalid!()),
                        };
                        packed = match verdant_affine_insert_field::run(
                            packed, value, divisor, radix, place,
                        ) {
                            Ok(v) => v,
                            Err(_) => return Err(invalid!()),
                        };
                    }
                    _ => return Err(invalid!()),
                }
                phase += 1;
            }
            *out = match u8::try_from(packed) {
                Ok(b) => b,
                Err(_) => return Err(invalid!()),
            };
        }
        output.copy_from_slice(&packet[..output.len()]);
        return Ok(());
    }
    if matches!(
        kind,
        GroupedSignedSix | GroupedSignedThree | NonlinearFour | GroupedNonlinearFour
    ) {
        use verdant_grouped_six_encoding::{run, Adt0 as Request, Adt1 as Frame, Adt2 as Step};
        let invalid = || TensorComputeError::InvalidKernelSpecification {
            kernel_name: "GroupedSixEncoding".into(),
        };
        let plan = |request| run(request).map_err(|_| invalid());
        let word = |step| match step {
            Step::C2(value) => Ok(value),
            _ => Err(invalid()),
        };
        let calculate = |program: fn(u64, Frame) -> Request, a, b, c, d| {
            let mut registers = [0; PLANE];
            let mut phase = 0;
            loop {
                let [x, y, z, t] = registers;
                let frame = Frame::C0(a, b, c, d, x, y, z, t);
                let (target, result) = match plan(program(phase, frame))? {
                    Step::C5(operation, left, right, target) => {
                        let value = super::evaluate_f32_scalar(
                            operation,
                            f32::from_bits(left as u32),
                            f32::from_bits(right as u32),
                        )?;
                        (target, u64::from(value.to_bits()))
                    }
                    Step::C6(code, center, target) => (
                        target,
                        super::encode_small_integer(code, center)
                            .map_err(|_| invalid())?
                            .ok_or_else(invalid)?,
                    ),
                    Step::C7(narrow, word, target) => (
                        target,
                        super::convert_float_storage(u64::from(!narrow), u64::from(narrow), word)
                            .map_err(|_| invalid())?
                            .ok_or_else(invalid)?,
                    ),
                    Step::C8(left, right, target) => (
                        target,
                        super::evaluate_encoded_word(0, left, right)
                            .map_err(|_| invalid())?
                            .ok_or_else(invalid)?,
                    ),
                    Step::C9(input, center, target) => {
                        (target, word(plan(Request::C9(input, center))?)?)
                    }
                    result => return Ok::<_, TensorComputeError>(result),
                };
                *registers.get_mut(target as usize).ok_or_else(invalid)? = result;
                phase += 1;
            }
        };
        let pack_fields = |dest: &mut [u8], nonlinear: Option<bool>, banks: &[&[u64]]| {
            let mut packed = [0; PAIRS * PAIRS];
            let packet = packed.get_mut(..dest.len()).ok_or_else(invalid)?;
            for (byte, out) in packet.iter_mut().enumerate() {
                let mut accumulator = 0;
                for phase in 0.. {
                    let query = match nonlinear {
                        None => Request::C21(byte as u64, phase),
                        Some(grouped) => Request::C28(grouped, byte as u64, phase),
                    };
                    match plan(query)? {
                        Step::C2(0) => break,
                        Step::C10(bank, index, divisor, radix, place) => {
                            let value = banks
                                .get(bank as usize)
                                .and_then(|b| b.get(index as usize))
                                .ok_or_else(invalid)?;
                            accumulator = verdant_affine_insert_field::run(
                                accumulator,
                                *value,
                                divisor,
                                radix,
                                place,
                            )
                            .map_err(|_| invalid())?;
                        }
                        _ => return Err(invalid()),
                    }
                }
                *out = u8::try_from(accumulator).map_err(|_| invalid())?;
            }
            dest.copy_from_slice(packet);
            Ok(())
        };
        if matches!(kind, NonlinearFour | GroupedNonlinearFour) {
            let grouped = kind == GroupedNonlinearFour;
            let Step::C1(width, trials, groups, _) = plan(Request::C22(
                grouped,
                input.len() as u64,
                output.len() as u64,
            ))?
            else {
                return Err(invalid());
            };
            let width = width as usize;
            let mut levels = [0; PAIRS * PAIRS];
            let mut scales = [0; PAIRS];
            let scales = scales.get_mut(..groups as usize).ok_or_else(invalid)?;
            let mut peak_scale = 0;
            for ((group, codes), scale) in input
                .chunks_exact(width)
                .zip(levels.chunks_exact_mut(width))
                .zip(scales.iter_mut())
            {
                let peak = group.iter().try_fold(0, |peak, value| {
                    word(plan(Request::C1(true, peak, value.to_bits().into()))?)
                })?;
                let mut best = 0;
                for trial in 0..trials {
                    let inverse =
                        word(calculate(Request::C23, peak, trial, u64::from(grouped), 0)?)?;
                    if inverse == 0 {
                        break;
                    }
                    let (mut xy, mut yy) = (0, 0);
                    for (lane, original) in group.iter().zip(codes.iter_mut()) {
                        let Step::C1(next_xy, next_yy, level, _) =
                            calculate(Request::C24, lane.to_bits().into(), inverse, xy, yy)?
                        else {
                            return Err(invalid());
                        };
                        (xy, yy) = (next_xy, next_yy);
                        if !grouped {
                            *original = level;
                        }
                    }
                    match calculate(Request::C4, u64::from(trial == 0), xy, yy, best)? {
                        Step::C4(candidate, score) => (*scale, best) = (candidate, score),
                        Step::C2(0) => {}
                        _ => return Err(invalid()),
                    }
                }
                peak_scale = word(plan(Request::C1(false, peak_scale, *scale))?)?;
            }
            let half = if grouped {
                let Step::C1(half, base, inverse, _) =
                    calculate(Request::C26, peak_scale, 0, 0, 0)?
                else {
                    return Err(invalid());
                };
                for ((group, codes), scale) in input
                    .chunks_exact(width)
                    .zip(levels.chunks_exact_mut(width))
                    .zip(scales.iter_mut())
                {
                    let Step::C3(header, local_inverse) =
                        calculate(Request::C27, base, inverse, *scale, 0)?
                    else {
                        return Err(invalid());
                    };
                    *scale = header;
                    for (lane, code) in group.iter().zip(codes) {
                        *code = word(calculate(
                            Request::C25,
                            lane.to_bits().into(),
                            local_inverse,
                            0,
                            0,
                        )?)?;
                    }
                }
                half
            } else {
                super::convert_float_storage(0, 1, scales[0])
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?
            };
            return pack_fields(
                output,
                Some(grouped),
                &[&levels, scales, std::slice::from_ref(&half)],
            );
        }
        if kind == GroupedSignedThree {
            let Step::C1(width, passes, _, _) =
                plan(Request::C10(input.len() as u64, output.len() as u64))?
            else {
                return Err(invalid());
            };
            let pair = |step| match step {
                Step::C3(first, second) => Ok((first, second)),
                _ => Err(invalid()),
            };
            let mut levels = [0; PAIRS * PAIRS];
            let mut scales = [0; PAIRS];
            let mut peak_scale = 0;
            for ((xs, codes), scale) in input
                .chunks_exact(width as usize)
                .zip(levels.chunks_exact_mut(width as usize))
                .zip(&mut scales)
            {
                let mut peak = 0;
                for x in xs {
                    peak = word(plan(Request::C1(true, peak, u64::from(x.to_bits())))?)?;
                }
                let inverse = word(calculate(Request::C11, peak, 0, 0, 0)?)?;
                if inverse == 0 {
                    continue;
                }
                let (mut xy, mut yy) = (0, 0);
                for (x, code) in xs.iter().zip(codes.iter_mut()) {
                    let x = u64::from(x.to_bits());
                    *code = word(calculate(Request::C12, x, inverse, 0, 0)?)?;
                    (xy, yy) = pair(calculate(Request::C14, x, *code, xy, yy)?)?;
                }
                for _ in 0..passes {
                    let mut changed = false;
                    for (x, code) in xs.iter().zip(codes.iter_mut()) {
                        let x = u64::from(x.to_bits());
                        let (remaining_xy, remaining_yy) =
                            pair(calculate(Request::C15, x, *code, xy, yy)?)?;
                        let proposal = word(calculate(
                            Request::C16,
                            x,
                            remaining_xy,
                            remaining_yy,
                            *code,
                        )?)?;
                        if proposal == *code {
                            continue;
                        }
                        let (next_xy, next_yy) = pair(calculate(
                            Request::C14,
                            x,
                            proposal,
                            remaining_xy,
                            remaining_yy,
                        )?)?;
                        if word(calculate(Request::C17, next_xy, next_yy, xy, yy)?)? != 0 {
                            (*code, xy, yy) = (proposal, next_xy, next_yy);
                            changed = true;
                        }
                    }
                    if !changed {
                        break;
                    }
                }
                *scale = word(calculate(Request::C18, xy, yy, 0, 0)?)?;
                peak_scale = word(plan(Request::C1(false, peak_scale, *scale))?)?;
            }
            let Step::C1(inverse, half, unit, _) = calculate(Request::C19, peak_scale, 0, 0, 0)?
            else {
                return Err(invalid());
            };
            for ((xs, codes), scale) in input
                .chunks_exact(width as usize)
                .zip(levels.chunks_exact_mut(width as usize))
                .zip(&mut scales)
            {
                let (header, local) = pair(calculate(Request::C20, inverse, unit, *scale, 0)?)?;
                *scale = header;
                if super::evaluate_encoded_word(0, local, 0)
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?
                    == 1
                {
                    continue;
                }
                for (x, code) in xs.iter().zip(codes) {
                    *code = word(calculate(
                        Request::C13,
                        u64::from(x.to_bits()),
                        local,
                        0,
                        0,
                    )?)?;
                }
            }
            return pack_fields(
                output,
                None,
                &[&levels, &scales, std::slice::from_ref(&half)],
            );
        }
        let Step::C1(width, trials, stride, bank) =
            plan(Request::C0(input.len() as u64, output.len() as u64))?
        else {
            return Err(invalid());
        };
        let mut scales = [0; PAIRS];
        let mut provisional = [0; PAIRS];
        let mut peak_scale = 0;
        for ((group, scale), chosen) in input
            .chunks_exact(width as usize)
            .zip(&mut scales)
            .zip(&mut provisional)
        {
            let peak = group.iter().try_fold(0, |peak, x| {
                word(plan(Request::C1(true, peak, x.to_bits().into()))?)
            })?;
            let first = word(calculate(Request::C2, peak, 0, 0, 0)?)?;
            if first == 0 {
                continue;
            }
            let mut best = 0;
            for trial in 0..trials {
                let inverse = if trial == 0 {
                    first
                } else {
                    word(calculate(Request::C2, peak, trial, 0, 0)?)?
                };
                let mut xy = 0;
                let mut yy = 0;
                for x in group {
                    let Step::C3(a, b) =
                        calculate(Request::C3, x.to_bits().into(), inverse, xy, yy)?
                    else {
                        return Err(invalid());
                    };
                    (xy, yy) = (a, b);
                }
                match calculate(Request::C4, u64::from(trial == 0), xy, yy, best)? {
                    Step::C4(candidate, score) => {
                        (*scale, best, *chosen) = (candidate, score, inverse)
                    }
                    Step::C2(0) => (),
                    _ => return Err(invalid()),
                }
            }
            peak_scale = word(plan(Request::C1(false, peak_scale, *scale))?)?;
        }
        let (inverse, half, unit) = match calculate(Request::C5, peak_scale, 0, 0, 0)? {
            Step::C2(0) => {
                output.fill(0);
                return Ok(());
            }
            Step::C1(inverse, half, unit, _) => (inverse, half, unit),
            _ => return Err(invalid()),
        };
        let mut staged = [0; PAIRS * PAIRS];
        let mut codes = [0; PAIRS * PAIRS];
        let (low, rest) = staged[..output.len()].split_at_mut(bank as usize);
        let (high, rest) = rest.split_at_mut((bank / scalars::two_u64()) as usize);
        let (headers, trailer) = rest.split_at_mut(scales.len());
        trailer.copy_from_slice(&(half as u16).to_ne_bytes());
        for ((((group, lanes), scale), chosen), header) in input
            .chunks_exact(width as usize)
            .zip(codes.chunks_exact_mut(width as usize))
            .zip(scales)
            .zip(provisional)
            .zip(headers)
        {
            let Step::C3(byte, local) = calculate(Request::C6, inverse, unit, scale, 0)? else {
                return Err(invalid());
            };
            *header = byte as u8;
            // A rounded zero group scale retains the codes from the winning search trial.
            let retain = super::evaluate_encoded_word(0, local, 0)
                .map_err(|_| invalid())?
                .ok_or_else(invalid)?
                == 1;
            let factor = if retain { chosen } else { local };
            for (lane, x) in lanes.iter_mut().zip(group) {
                *lane = word(calculate(
                    Request::C7,
                    u64::from(retain),
                    x.to_bits().into(),
                    factor,
                    0,
                )?)? as u8;
            }
        }
        for ((codes, lows), highs) in codes
            .chunks_exact(bank as usize)
            .zip(low.chunks_exact_mut((bank / scalars::two_u64()) as usize))
            .zip(high.chunks_exact_mut(stride as usize))
        {
            let mut bands = codes.chunks_exact(stride as usize);
            let [Some(a), Some(b), Some(c), Some(d)] = std::array::from_fn(|_| bands.next()) else {
                return Err(invalid());
            };
            let (first, second) = lows.split_at_mut(stride as usize);
            for (lane, dst) in highs.iter_mut().enumerate() {
                let Step::C1(x, y, z, _) = plan(Request::C8(
                    a[lane].into(),
                    b[lane].into(),
                    c[lane].into(),
                    d[lane].into(),
                ))?
                else {
                    return Err(invalid());
                };
                (first[lane], second[lane], *dst) = (x as u8, y as u8, z as u8);
            }
        }
        output.copy_from_slice(&staged[..output.len()]);
        return Ok(());
    }
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
