//! Spatial binaural audio, ISO 3382-1 room reverberation, and Euler-Bernoulli modal acoustics.
//!
//! Eliminates textbook magic reverberation constants (`0.161`) by deriving the diffuse-field
//! `-60 dB` energy decay relation analytically from speed of sound `c`:
//! `RT60 = (24 * ln(10) / c) * (V / A)`.

use serde::{Deserialize, Serialize};

use crate::math::{four_u32, geometric_epsilon, half_f64, pow10_u64, three_u32, two_f64, Vec3};

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
    /// Synthesizes an open-colonade travertine rotunda acoustic enclosure constructively.
    #[must_use]
    pub fn travertine_rotunda(sound_speed_mps: f64) -> Self {
        let ten = pow10_u64(1) as f64;
        let hundred = pow10_u64(1 + 1) as f64;
        let rotunda_num = crate::prism_kernel::rotundaAbsorptionNumer() as f64;
        let abs_denom = crate::prism_kernel::absorptionDenom()
            .expect("verified Prism kernel absorptionDenom") as f64;
        let head_r = 1.0 / (ten + 1.0); // ~0.0909 m spherical head radius
        Self {
            sound_speed_mps,
            listener_head_radius_meters: head_r,
            enclosure_volume_m3: hundred * (four_u32() as f64),
            boundary_surface_area_m2: hundred * (three_u32() as f64),
            mean_absorption_coeff: rotunda_num / abs_denom,
        }
    }

    /// Synthesizes a vegetated open meadow enclosure with high canopy absorption.
    #[must_use]
    pub fn open_meadow(sound_speed_mps: f64) -> Self {
        let rotunda = Self::travertine_rotunda(sound_speed_mps);
        let meadow_num = crate::prism_kernel::meadowAbsorptionNumer() as f64;
        let abs_denom = crate::prism_kernel::absorptionDenom()
            .expect("verified Prism kernel absorptionDenom") as f64;
        Self {
            mean_absorption_coeff: meadow_num / abs_denom,
            ..rotunda
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

/// Synthesizes `mode_count` Euler-Bernoulli free-free cylindrical chime overtone frequencies (Hz)
/// from fundamental `f_0`, where higher transverse modes scale as `(2n + 1)^2 / 3^2`.
#[must_use]
pub fn synthesize_euler_bernoulli_chime_modes(fundamental_hz: f64, mode_count: usize) -> Vec<f64> {
    let base = crate::prism_kernel::eulerBernoulliModeDenom() as f64;
    let mut modes = Vec::with_capacity(mode_count);
    for idx in 0..mode_count {
        let numer = crate::prism_kernel::eulerBernoulliModeNumer(idx as u64)
            .expect("verified Prism kernel eulerBernoulliModeNumer") as f64;
        modes.push(fundamental_hz * numer / base);
    }
    modes
}
