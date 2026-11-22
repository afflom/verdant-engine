//! Binary32 rotary frequency scaling and two-coordinate rotations.

use super::{
    evaluate_f32_scalar, evaluate_float_unary, evaluate_wide, inverse_root_f32, natural_log_f32,
    sin_cos_f32, TensorComputeError,
};
use verdant_rotation_step::{Adt0 as Program, Adt1 as Action, Adt2 as Operand};

fn failure() -> TensorComputeError {
    TensorComputeError::InvalidKernelSpecification {
        kernel_name: "PlaneRotation".to_owned(),
    }
}

fn bits(value: f32) -> u64 {
    u64::from(value.to_bits())
}

fn value(word: u64) -> f32 {
    f32::from_bits(word as u32)
}

fn unsigned_float(word: u64) -> Result<f32, TensorComputeError> {
    evaluate_wide(3, word, 0)?.map(value).ok_or_else(failure)
}

/// Convert a signed 32-bit coordinate to binary32, rounding to nearest even.
///
/// # Errors
/// Propagates an error from the certified word conversion.
pub fn signed_word_f32(coordinate: i32) -> Result<f32, TensorComputeError> {
    let verdant_rotation_signed::Adt0::C0(negative, magnitude) =
        verdant_rotation_signed::run(u64::from(coordinate as u32));
    let result = unsigned_float(magnitude)?;
    if negative {
        evaluate_f32_scalar(6, result, 0.0)
    } else {
        Ok(result)
    }
}

type Slots = [u64; 15];

fn execute(program: Program, slots: &mut Slots) -> Result<(), TensorComputeError> {
    let mut stage = 0;
    loop {
        let read = |operand| match operand {
            Operand::C0(slot) => slots.get(slot as usize).copied().ok_or_else(failure),
            Operand::C1(word) => Ok(word),
        };
        let write = match verdant_rotation_step::run(program.clone(), stage) {
            Action::C0 => return Ok(()),
            Action::C1(op, dst, a, b) => Some((
                dst,
                bits(evaluate_f32_scalar(op, value(read(a)?), value(read(b)?))?),
            )),
            Action::C2(maximum, dst, a, b) => Some((
                dst,
                verdant_rotation_limit::run(maximum, read(a)?, read(b)?).map_err(|_| failure())?,
            )),
            Action::C3(dst, a) => Some((dst, bits(natural_log_f32(value(read(a)?))?))),
            Action::C4(signed, dst, a) => {
                let input = read(a)?;
                let converted = if signed {
                    signed_word_f32(input as i32)?
                } else {
                    unsigned_float(input)?
                };
                Some((dst, bits(converted)))
            }
            Action::C5(op, dst, a) => Some((
                dst,
                evaluate_float_unary(op, 0, 0, read(a)?)?.ok_or_else(failure)?,
            )),
            Action::C6(cosine, sine, a) => {
                let (s, c) = sin_cos_f32(value(read(a)?))?;
                *slots.get_mut(cosine as usize).ok_or_else(failure)? = bits(c);
                Some((sine, bits(s)))
            }
            Action::C7 => None,
        };
        if let Some((slot, word)) = write {
            *slots.get_mut(slot as usize).ok_or_else(failure)? = word;
        }
        stage += 1;
    }
}

/// Geometric frequency and YaRN interpolation parameters for a rotation.
#[derive(Clone, Copy, Debug)]
pub struct RotaryParameters {
    /// Even number of rotated coordinates.
    pub dimensions: u32,
    /// Length of the original context.
    pub context: i32,
    /// Positive finite geometric frequency base.
    pub base: f32,
    /// Interpolation frequency multiplier.
    pub frequency_scale: f32,
    /// Interpolation/extrapolation blend.
    pub extension: f32,
    /// Requested coefficient magnitude.
    pub magnitude: f32,
    /// Fast and slow rotation counts defining the correction interval.
    pub rotations: [f32; 2],
    /// Round the ends of the correction interval outward.
    pub truncate: bool,
}

/// Reusable frequency ratio, correction interval and magnitude for a spectrum.
#[derive(Clone, Copy, Debug)]
pub struct RotarySpectrum {
    ratio: f32,
    scale: f32,
    extension: f32,
    extended: bool,
    magnitude: u64,
    low: u64,
    high: u64,
}

impl RotarySpectrum {
    /// Prepare the frequency ratio and the interpolation parameters once.
    ///
    /// # Errors
    /// Returns a typed kernel error for an invalid dimension or frequency base.
    pub fn new(p: RotaryParameters) -> Result<Self, TensorComputeError> {
        let verdant_rotation_geometry::Adt0::C1(degree, last, extended) =
            verdant_rotation_geometry::run(u64::from(p.dimensions), bits(p.extension))
                .map_err(|_| failure())?
        else {
            return Err(failure());
        };
        let ratio = inverse_root_f32(p.base, u32::try_from(degree).map_err(|_| failure())?)?;
        let mut slots = [0; 15];
        slots[..8].copy_from_slice(&[
            u64::from(p.dimensions),
            u64::from(p.context as u32),
            bits(p.base),
            bits(p.frequency_scale),
            bits(p.extension),
            bits(p.magnitude),
            bits(p.rotations[0]),
            bits(p.rotations[1]),
        ]);
        slots[14] = last;
        execute(Program::C0(extended, p.truncate), &mut slots)?;
        Ok(Self {
            ratio,
            scale: p.frequency_scale,
            extension: p.extension,
            extended,
            magnitude: slots[5],
            low: slots[8],
            high: slots[9],
        })
    }

    /// Advance one geometric frequency from this phase.
    ///
    /// # Errors
    /// Propagates a certified scalar multiplication error.
    pub fn advance(&self, phase: f32) -> Result<f32, TensorComputeError> {
        evaluate_f32_scalar(2, phase, self.ratio)
    }

    /// Compute cosine and signed sine for one frequency pair, including its factor.
    ///
    /// # Errors
    /// Propagates a typed error from a coefficient operation.
    pub fn weights(
        &self,
        phase: f32,
        factor: f32,
        lane: u32,
        inverse: bool,
    ) -> Result<(f32, f32), TensorComputeError> {
        let mut slots = [0; 15];
        slots[..8].copy_from_slice(&[
            bits(phase),
            bits(factor),
            u64::from(lane),
            bits(self.scale),
            bits(self.extension),
            self.magnitude,
            self.low,
            self.high,
        ]);
        execute(Program::C1(self.extended, inverse), &mut slots)?;
        Ok((value(slots[8]), value(slots[9])))
    }
}

/// Apply a pair of cosine and signed sine coefficients to two coordinates.
///
/// # Errors
/// Propagates a typed error from a certified scalar operation.
pub fn rotate_pair_f32(
    coordinates: (f32, f32),
    coefficients: (f32, f32),
) -> Result<(f32, f32), TensorComputeError> {
    let mut slots = [0; 15];
    slots[..4].copy_from_slice(&[
        bits(coordinates.0),
        bits(coordinates.1),
        bits(coefficients.0),
        bits(coefficients.1),
    ]);
    execute(Program::C2, &mut slots)?;
    Ok((value(slots[4]), value(slots[5])))
}
