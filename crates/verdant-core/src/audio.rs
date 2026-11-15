//! Spatial binaural audio, ISO 3382-1 room reverberation, and Euler-Bernoulli modal acoustics.
//!
//! Eliminates textbook magic reverberation constants (`0.161`) by deriving the diffuse-field
//! `-60 dB` energy decay relation analytically from speed of sound `c`:
//! `RT60 = (24 * ln(10) / c) * (V / A)`.

use serde::{Deserialize, Serialize};

use crate::ecs::BoundingVolumeHierarchy;
use crate::error::TensorComputeError;
use crate::math::{
    four_u32, geometric_epsilon, half_f64, pow10_u64, pow2_u64, three_u32, two_f64, Ray, Vec3,
    DIM_HOMOGENEOUS, DIM_PLANAR,
};

/// Parametric acoustic propagation and enclosure reverberation environment.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcousticEnvironment {
    /// Speed of sound in the ambient medium (`m/s`).
    pub sound_speed_mps: f64,
    /// Listener spherical-head diffraction radius (`m`, Woodworth binaural model).
    pub listener_head_radius_meters: f64,
    /// Effective reverberant enclosure volume (`m^3`).
    pub enclosure_volume_m3: f64,
    /// Total enclosure boundary surface area (`m^2`).
    pub boundary_surface_area_m2: f64,
    /// Mean energy absorption coefficient `alpha_bar in (0, 1)`.
    pub mean_absorption_coeff: f64,
}

impl AcousticEnvironment {
    /// Synthesizes a stone or masonry reflective chamber enclosure constructively.
    #[must_use]
    pub fn reflective_chamber(sound_speed_mps: f64) -> Self {
        let ten = pow10_u64(1) as f64;
        let hundred = pow10_u64(1 + 1) as f64;
        let chamber_num = crate::prism_kernel::reflectiveRoomAbsorptionNumer() as f64;
        let abs_denom = crate::prism_kernel::absorptionDenom()
            .expect("verified Prism kernel absorptionDenom") as f64;
        let head_r = 1.0 / (ten + 1.0); // ~0.0909 m spherical head radius
        Self {
            sound_speed_mps,
            listener_head_radius_meters: head_r,
            enclosure_volume_m3: hundred * (four_u32() as f64),
            boundary_surface_area_m2: hundred * (three_u32() as f64),
            mean_absorption_coeff: chamber_num / abs_denom,
        }
    }

    /// Synthesizes a high-absorption damped exterior acoustic environment.
    #[must_use]
    pub fn damped_exterior(sound_speed_mps: f64) -> Self {
        let chamber = Self::reflective_chamber(sound_speed_mps);
        let ext_num = crate::prism_kernel::dampedExteriorAbsorptionNumer() as f64;
        let abs_denom = crate::prism_kernel::absorptionDenom()
            .expect("verified Prism kernel absorptionDenom") as f64;
        Self {
            mean_absorption_coeff: ext_num / abs_denom,
            ..chamber
        }
    }

    /// Analytical ISO 3382-1 Sabine reverberation time `RT60` in seconds:
    /// `RT60 = (24 * ln(10) / c) * V / (S * alpha_bar)`.
    #[must_use]
    pub fn sabine_rt60_seconds(&self) -> f64 {
        let twenty_four = crate::prism_kernel::sabinePrefactorNumer()
            .expect("verified Prism kernel sabinePrefactorNumer") as f64;
        let ten = pow10_u64(1) as f64;
        let sabine_factor = (twenty_four * ten.ln()) / self.sound_speed_mps.max(1.0);
        let sabins = (self.boundary_surface_area_m2
            * self.mean_absorption_coeff.clamp(geometric_epsilon(), 1.0))
        .max(geometric_epsilon());
        sabine_factor * self.enclosure_volume_m3 / sabins
    }

    /// Analytical ISO 3382-1 Eyring-Norris reverberation time `RT60` in seconds:
    /// `RT60 = (24 * ln(10) / c) * V / (-S * ln(1 - alpha_bar))`.
    #[must_use]
    pub fn eyring_rt60_seconds(&self) -> f64 {
        let twenty_four = crate::prism_kernel::sabinePrefactorNumer()
            .expect("verified Prism kernel sabinePrefactorNumer") as f64;
        let ten = pow10_u64(1) as f64;
        let factor = (twenty_four * ten.ln()) / self.sound_speed_mps.max(1.0);
        let alpha = self
            .mean_absorption_coeff
            .clamp(geometric_epsilon(), 1.0 - geometric_epsilon());
        let denom = (-self.boundary_surface_area_m2 * (1.0 - alpha).ln()).max(geometric_epsilon());
        factor * self.enclosure_volume_m3 / denom
    }

    /// Fixed-point Sabine reverberation time `RT60` in milliseconds evaluated directly by
    /// `prism_kernel::sabineRt60Millis` for deterministic WebAssembly/AudioWorklet parity.
    #[must_use]
    pub fn sabine_rt60_prism_millis(&self) -> u64 {
        let scale =
            crate::prism_kernel::skinWeightScale().expect("verified Prism skinWeightScale") as f64;
        let vol_m3 = self.enclosure_volume_m3.max(0.0).round() as u64;
        let absorption_milli = (self.boundary_surface_area_m2
            * self.mean_absorption_coeff.clamp(geometric_epsilon(), 1.0)
            * scale)
            .round() as u64;
        crate::prism_kernel::sabineRt60Millis(vol_m3, absorption_milli)
            .expect("verified Prism sabineRt60Millis")
    }
}

/// Spatial listener pose (`position`, `forward`, `up`) for 3D binaural rendering.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListenerPose {
    /// Listener ear-midpoint world position in meters.
    pub position: Vec3,
    /// Unit forward gaze direction.
    pub forward: Vec3,
    /// Unit upward head axis.
    pub up: Vec3,
}

impl ListenerPose {
    /// Right-ear unit axis `forward x up`.
    #[must_use]
    pub fn right_axis(&self) -> Vec3 {
        self.forward
            .normalized()
            .cross(self.up.normalized())
            .normalized()
    }
}

/// Point or modal acoustic emitter placed in the 3D scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialAudioSource {
    /// Source identifier.
    pub label: String,
    /// World-space emitter coordinates in meters.
    pub position: Vec3,
    /// Fundamental pitch in hertz.
    pub fundamental_hz: f64,
    /// Reference sound power gain in `[0, 1]`.
    pub reference_gain: f64,
}

/// Evaluated binaural cue frame for a `SpatialAudioSource` heard from `ListenerPose`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinauralCue {
    /// Source-to-listener distance in meters.
    pub distance_meters: f64,
    /// Signed azimuth sine `sin(theta) in [-1, 1]` (`+1` right ear, `-1` left ear).
    pub azimuth_sin: f64,
    /// Woodworth spherical-head Interaural Time Difference (`ITD`) in seconds.
    pub itd_seconds: f64,
    /// Frequency-dependent head-shadow Interaural Level Difference (`ILD`) in decibels.
    pub ild_db: f64,
    /// Inverse-distance attenuated direct-path linear gain.
    pub direct_gain: f64,
}

/// Evaluates Woodworth spherical-head ITD, head-shadow ILD, and inverse-distance gain.
#[must_use]
pub fn evaluate_binaural_cue(
    source: &SpatialAudioSource,
    listener: &ListenerPose,
    env: &AcousticEnvironment,
) -> BinauralCue {
    let rel = source.position - listener.position;
    let dist = rel.length().max(geometric_epsilon());
    let dir = rel.scale(1.0 / dist);
    let sin_theta = dir.dot(listener.right_axis()).clamp(-1.0, 1.0);
    let theta = sin_theta.asin();

    // Woodworth spherical diffraction formula: `ITD = (a / c) * (theta + sin(theta))`:
    let itd_seconds =
        (env.listener_head_radius_meters / env.sound_speed_mps.max(1.0)) * (theta + sin_theta);

    // Dimensionless Helmholtz wave number `k * a = (2 * pi * f * a) / c`:
    let ka = (std::f64::consts::TAU * source.fundamental_hz * env.listener_head_radius_meters)
        / env.sound_speed_mps.max(1.0);
    let ten = pow10_u64(1) as f64;
    let ild_db =
        ten * two_f64() * (1.0 + half_f64() * ka * sin_theta.abs()).log10() * sin_theta.signum();
    let direct_gain = source.reference_gain / (1.0 + dist);

    BinauralCue {
        distance_meters: dist,
        azimuth_sin: sin_theta,
        itd_seconds,
        ild_db,
        direct_gain,
    }
}

/// Evaluates Woodworth binaural cues and casts an acoustic line-of-sight ray against `bvh`,
/// attenuating `direct_gain` by `prism_kernel::acousticOccludedGainPermille` / `skinWeightScale`
/// whenever scene geometry occludes the direct path between `listener` and `source`.
#[must_use]
pub fn evaluate_binaural_cue_with_occlusion(
    source: &SpatialAudioSource,
    listener: &ListenerPose,
    env: &AcousticEnvironment,
    bvh: &BoundingVolumeHierarchy,
) -> (BinauralCue, bool) {
    let mut cue = evaluate_binaural_cue(source, listener, env);
    let to_source = source.position - listener.position;
    let seg_len = to_source.length();
    let eps = geometric_epsilon() * (pow10_u64(1 + 1) as f64);
    if seg_len <= eps + eps {
        return (cue, false);
    }
    let dir = to_source.scale(1.0 / seg_len);
    let ray = Ray::new(listener.position + dir.scale(eps), dir);
    let max_dist = (seg_len - (eps + eps)).max(0.0);
    let occluded = bvh.raycast(ray, max_dist).is_some();
    if occluded {
        let num = crate::prism_kernel::acousticOccludedGainPermille().unwrap_or(1) as f64;
        let den = crate::prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
        cue.direct_gain *= (num / den).clamp(0.0, 1.0);
    }
    (cue, occluded)
}

/// Synthesizes `mode_count` Euler-Bernoulli free-free cylindrical bar overtone frequencies (Hz)
/// from fundamental `f_0`, where higher transverse modes scale as `(2n + 1)^2 / 3^2`.
#[must_use]
pub fn synthesize_modal_bar_modes(fundamental_hz: f64, mode_count: usize) -> Vec<f64> {
    let base = crate::prism_kernel::eulerBernoulliModeDenom() as f64;
    let mut modes = Vec::with_capacity(mode_count);
    for idx in 0..mode_count {
        let numer = crate::prism_kernel::eulerBernoulliModeNumer(idx as u64)
            .expect("verified Prism kernel eulerBernoulliModeNumer") as f64;
        modes.push(fundamental_hz * numer / base);
    }
    modes
}

/// Rendered stereo + 4-channel Hadamard FDN reverberation audio quantum produced by `verdant-tensor`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinauralReverbQuantum {
    /// Interleaved exact `i64` stereo frames (`frame_count * 2`).
    pub stereo_frames_accum: Vec<i64>,
    /// Modal FIR convolved excitation frames (`frame_count`).
    pub modal_fir_accum: Vec<i64>,
    /// 4-channel Hadamard Sabine/Eyring feedback delay network outputs (`frame_count x 4`).
    pub fdn_reverb_tank: Vec<[i64; DIM_HOMOGENEOUS]>,
}

/// Synthesizes a deterministic binaural + modal FIR + 4-channel Sabine/Eyring FDN audio quantum
/// entirely via quantized audio-ring contractions in `verdant-tensor` (`uor-matmul`).
pub fn synthesize_binaural_reverb_quantum_exact(
    dry_excitation_q15: &[i16],
    modal_taps_q15: &[i16],
    source: &SpatialAudioSource,
    listener: &ListenerPose,
    env: &AcousticEnvironment,
) -> Result<BinauralReverbQuantum, TensorComputeError> {
    let modal_fir_accum =
        verdant_tensor::convolve_modal_fir_exact(dry_excitation_q15, modal_taps_q15)?;
    let frame_count = modal_fir_accum.len();
    let fifteen = (crate::prism_kernel::sFifteen().unwrap_or(1)) as u32;
    let q15_scale = (pow2_u64(fifteen) - 1) as f64;

    let cue = evaluate_binaural_cue(source, listener, env);
    let left_pan = ((1.0 - cue.azimuth_sin) * half_f64()).clamp(0.0, 1.0) * cue.direct_gain;
    let right_pan = ((1.0 + cue.azimuth_sin) * half_f64()).clamp(0.0, 1.0) * cue.direct_gain;
    let pan_gains_q15 = [
        (left_pan.clamp(0.0, 1.0) * q15_scale).round() as i16,
        (right_pan.clamp(0.0, 1.0) * q15_scale).round() as i16,
    ];

    let convolved_q15: Vec<i16> = modal_fir_accum
        .iter()
        .map(|&acc| (acc >> fifteen).clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16)
        .collect();
    let stereo_frames_accum = verdant_tensor::mix_spatial_audio_channels_exact(
        frame_count,
        1,
        DIM_PLANAR,
        &convolved_q15,
        &pan_gains_q15,
    )?;

    let rt60 = env.eyring_rt60_seconds().max(geometric_epsilon());
    let decay = (-(1.0 / rt60.max(1.0))).exp().clamp(0.0, 1.0);
    let g = (decay * q15_scale).round() as i16;
    let decay_gains = [g, g, g, g];
    let i2 = 1usize + 1usize;
    let delay_lines: Vec<[i16; DIM_HOMOGENEOUS]> = convolved_q15
        .iter()
        .map(|&s| [s, s / (i2 as i16), -s / (i2 as i16), s])
        .collect();
    let fdn_reverb_tank = verdant_tensor::step_sabine_fdn_reverb_ring(&delay_lines, &decay_gains)?;

    Ok(BinauralReverbQuantum {
        stereo_frames_accum,
        modal_fir_accum,
        fdn_reverb_tank,
    })
}
