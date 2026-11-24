//! Transport for the certified importance-grid encoder. Prism supplies the
//! instruction sequence, branch targets, indexed cells and complete packing
//! layout. Verdant evaluates its binary32 operations. The imported neighborhood
//! directory preserves the reference order of equidistant codebook points.

use super::{convert_float_storage, evaluate_encoded_word, evaluate_f32_scalar};
use super::{PackedWeightEncoding, GGML_QUANTIZED_TABLES};
use crate::{scalars, TensorComputeError};

const DIRECTORY: &[u8] = include_bytes!("../iq3_neighbors.bin");
const CELLS: usize = (u8::MAX as usize) + 1;
const SECTION: usize = CELLS * (scalars::two_u64() as usize);
const VALUE: u64 = 1;
const GEOMETRY: u64 = scalars::six_u32() as u64;
const PAIR: u64 = scalars::seven_u64();
const FIELD: u64 = scalars::eight_u64();

#[derive(Clone, Copy)]
enum Bank {
    Literal,
    Register,
    Workspace,
    Input,
    Importance,
    Directory,
    Grid,
    Output,
}
const BANKS: [Bank; scalars::eight_u64() as usize] = [
    Bank::Literal,
    Bank::Register,
    Bank::Workspace,
    Bank::Input,
    Bank::Importance,
    Bank::Directory,
    Bank::Grid,
    Bank::Output,
];
#[derive(Clone, Copy)]
enum Action {
    Fail,
    Done,
    Clear,
    Move,
    Natural,
    Scalar,
    ToFloat,
    ToHalf,
    FormatValue,
    FormatPair,
    FormatField,
    Branch,
    Jump,
    Resume,
    Insert,
    InsertWide,
}
const ACTIONS: [Action; scalars::sixteen_u64() as usize] = [
    Action::Fail,
    Action::Done,
    Action::Clear,
    Action::Move,
    Action::Natural,
    Action::Scalar,
    Action::ToFloat,
    Action::ToHalf,
    Action::FormatValue,
    Action::FormatPair,
    Action::FormatField,
    Action::Branch,
    Action::Jump,
    Action::Resume,
    Action::Insert,
    Action::InsertWide,
];

/// Encode an importance-grid block, using optional nonnegative importance weights.
///
/// # Errors
/// Unsupported representations, extents or inputs are refused before publication.
/// A refusal preserves every destination byte.
pub fn encode_importance_weight_block(
    input: &[f32],
    importance: Option<&[f32]>,
    kind: PackedWeightEncoding,
    output: &mut [u8],
) -> Result<(), TensorComputeError> {
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "ImportanceGridEncoding".into(),
    };
    let (family, compact, directory) = match kind {
        PackedWeightEncoding::ImportanceThree => (0, false, DIRECTORY),
        PackedWeightEncoding::ImportanceThreeCompact => (0, true, DIRECTORY),
        PackedWeightEncoding::ImportanceTwo => {
            (1, false, include_bytes!("../iq2s_neighbors.bin").as_slice())
        }
        PackedWeightEncoding::ImportanceTwoParity => {
            (1, true, include_bytes!("../iq2xs_neighbors.bin").as_slice())
        }
        PackedWeightEncoding::ImportanceTwoGroupedParity => (
            scalars::two_u64(),
            true,
            include_bytes!("../iq2xxs_neighbors.bin").as_slice(),
        ),
        _ => return Err(invalid()),
    };
    let format = |tag, a, b, c| {
        let tag = if tag < scalars::ten_u64() {
            tag + family * scalars::sixteen_u64()
        } else {
            tag
        };
        let (kind, (a, (b, (c, (d, e))))) =
            verdant_importance_grid::run(tag, compact, a, b, c).map_err(|_| invalid())?;
        Ok([kind, a, b, c, d, e])
    };
    let [GEOMETRY, _, _, trials, threshold, _] = format(
        0,
        input.len() as u64,
        importance.map_or(0, <[f32]>::len) as u64,
        output.len() as u64,
    )?
    else {
        return Err(invalid());
    };
    let finite = |value: &f32| verdant_affine_finite::run(value.to_bits().into());
    if !input.iter().all(finite)
        || importance.is_some_and(|weights| {
            weights.len() != input.len() || !weights.iter().all(|w| finite(w) && *w >= 0.0)
        })
    {
        return Err(invalid());
    }
    let mut cells = [0_u64; CELLS];
    let initial = [
        0,
        u64::from(compact),
        u64::from(importance.is_some()),
        trials,
        threshold,
    ];
    cells[..initial.len()].copy_from_slice(&initial);
    let mut staging = vec![0_u8; output.len()];
    let address = |word, cells: &[u64]| {
        let [FIELD, bank, base, index, ..] = format(scalars::ten_u64(), word, 0, 0)? else {
            return Err(invalid());
        };
        let bank = *BANKS.get(bank as usize).ok_or_else(invalid)?;
        let index = cells.get(index as usize).ok_or_else(invalid)?;
        let at = base
            .checked_add(*index)
            .and_then(|v| usize::try_from(v).ok())
            .ok_or_else(invalid)?;
        Ok((bank, at))
    };
    let read = |word, cells: &[u64], staging: &[u8]| {
        let (bank, at) = address(word, cells)?;
        let value = match bank {
            Bank::Literal => Some(at as u64),
            Bank::Register | Bank::Workspace => cells.get(at).copied(),
            Bank::Input => input.get(at).map(|v| u64::from(v.to_bits())),
            Bank::Importance => importance
                .and_then(|v| v.get(at))
                .map(|v| u64::from(v.to_bits())),
            Bank::Directory => directory
                .as_chunks::<{ size_of::<u16>() }>()
                .0
                .get(at)
                .map(|v| u64::from(u16::from_le_bytes(*v))),
            Bank::Grid => GGML_QUANTIZED_TABLES.get(at).copied().map(u64::from),
            Bank::Output => staging.get(at).copied().map(u64::from),
        };
        value.ok_or_else(invalid)
    };
    let write = |word, value, cells: &mut [u64], staging: &mut [u8]| {
        let (bank, at) = address(word, cells)?;
        match bank {
            Bank::Register | Bank::Workspace => *cells.get_mut(at).ok_or_else(invalid)? = value,
            Bank::Output => {
                *staging.get_mut(at).ok_or_else(invalid)? =
                    u8::try_from(value).map_err(|_| invalid())?
            }
            _ => return Err(invalid()),
        }
        Ok(())
    };
    let natural =
        |operation, left, right| match format(scalars::ten_u64() + 1, operation, left, right)? {
            [VALUE, word, ..] => Ok(word),
            _ => Err(invalid()),
        };
    let mut pc = 0_usize;
    loop {
        let section = pc / SECTION;
        let local = (section % (scalars::two_u64() as usize)) as u64 + family * scalars::two_u64();
        let step = (pc % SECTION) as u64;
        let instruction = match section / (scalars::two_u64() as usize) {
            0 => verdant_iq3_fit::run(local, step).ok(),
            1 => verdant_iq3_level::run(local, step).ok(),
            n if n == scalars::two_u64() as usize => verdant_iq3_score::run(local, step).ok(),
            n if n == scalars::three_u64() as usize => verdant_iq3_scale::run(local, step).ok(),
            _ => None,
        };
        let (action, (a, (b, (c, (d, (e, (f, g))))))) = instruction.ok_or_else(invalid)?;
        let action = *ACTIONS.get(action as usize).ok_or_else(invalid)?;
        let get = |operand| read(operand, &cells, &staging);
        pc += 1;
        let (target, value) = match action {
            Action::Fail => return Err(invalid()),
            Action::Clear => {
                staging.fill(0);
                break;
            }
            Action::Done => break,
            Action::Move => (b, get(a)?),
            Action::Natural => (d, natural(a, get(b)?, get(c)?)?),
            Action::Scalar => {
                let left = f32::from_bits(get(b)? as u32);
                let right = f32::from_bits(get(c)? as u32);
                (d, u64::from(evaluate_f32_scalar(a, left, right)?.to_bits()))
            }
            Action::ToFloat => (
                b,
                verdant_count_f32::run(get(a)?)
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?,
            ),
            Action::ToHalf => (
                b,
                convert_float_storage(0, 1, get(a)?)
                    .map_err(|_| invalid())?
                    .ok_or_else(invalid)?,
            ),
            Action::FormatValue | Action::FormatPair | Action::FormatField => {
                let fields = format(a, get(b)?, get(c)?, get(d)?)?;
                match (action, fields) {
                    (Action::FormatValue, [VALUE, one, ..]) => (e, one),
                    (Action::FormatPair, [PAIR, one, two, ..]) => {
                        write(f, two, &mut cells, &mut staging)?;
                        (e, one)
                    }
                    (Action::FormatField, [FIELD, one, two, three, ..]) => {
                        write(f, two, &mut cells, &mut staging)?;
                        write(g, three, &mut cells, &mut staging)?;
                        (e, one)
                    }
                    _ => return Err(invalid()),
                }
            }
            Action::Branch => {
                let (left, right) = (get(b)?, get(c)?);
                let taken = if a == scalars::two_u64() {
                    evaluate_encoded_word(0, left, right)
                        .map_err(|_| invalid())?
                        .ok_or_else(invalid)?
                        == 0
                } else {
                    natural(u64::from(scalars::five_u32()) + a, left, right)? != 0
                };
                if taken {
                    pc = usize::try_from(get(d)?).map_err(|_| invalid())?;
                }
                continue;
            }
            Action::Jump | Action::Resume => {
                pc = usize::try_from(get(a)?).map_err(|_| invalid())?;
                continue;
            }
            Action::Insert | Action::InsertWide => {
                let at = usize::try_from(get(a)?).map_err(|_| invalid())?;
                let shift = u32::try_from(get(b)?).map_err(|_| invalid())?;
                let value = u32::try_from(get(c)?)
                    .map_err(|_| invalid())?
                    .checked_shl(shift)
                    .ok_or_else(invalid)?;
                let width = if matches!(action, Action::InsertWide) {
                    size_of::<u32>()
                } else {
                    1
                };
                let end = at.checked_add(width).ok_or_else(invalid)?;
                let bytes = staging.get_mut(at..end).ok_or_else(invalid)?;
                for (dst, src) in bytes.iter_mut().zip(value.to_le_bytes()) {
                    *dst |= src;
                }
                continue;
            }
        };
        write(target, value, &mut cells, &mut staging)?;
    }
    output.copy_from_slice(&staging);
    Ok(())
}
