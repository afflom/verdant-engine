//! Canonical 22-joint humanoid joint IDs, archetypes, and analytical two-bone Foot IK telemetry (`character_types.rs`).

use super::*;

/// Canonical 22-joint humanoid hierarchy indices (`0..22`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanoidJointId {
    /// Root pelvis joint (`0`).
    Pelvis,
    /// Lower lumbar spine (`1`).
    Spine1,
    /// Mid thoracic spine (`2`).
    Spine2,
    /// Upper chest / ribcage (`3`).
    Chest,
    /// Cervical neck (`4`).
    Neck,
    /// Cranium and neuromorphic visor (`5`).
    Head,
    /// Left clavicle (`6`).
    ClavicleL,
    /// Left shoulder / upper arm (`7`).
    UpperArmL,
    /// Left elbow / forearm (`8`).
    ForearmL,
    /// Left wrist / hand (`9`).
    HandL,
    /// Right clavicle (`10`).
    ClavicleR,
    /// Right shoulder / upper arm (`11`).
    UpperArmR,
    /// Right elbow / forearm (`12`).
    ForearmR,
    /// Right wrist / hand (`13`).
    HandR,
    /// Left hip / thigh (`14`).
    ThighL,
    /// Left knee / calf (`15`).
    CalfL,
    /// Left ankle / foot (`16`).
    FootL,
    /// Left forefoot / toe (`17`).
    ToeL,
    /// Right hip / thigh (`18`).
    ThighR,
    /// Right knee / calf (`19`).
    CalfR,
    /// Right ankle / foot (`20`).
    FootR,
    /// Right forefoot / toe (`21`).
    ToeR,
}

impl HumanoidJointId {
    /// Zero-based topological joint index (`0..22`).
    #[must_use]
    pub const fn index(self) -> usize {
        let two = 1_u32.wrapping_add(1_u32);
        let three = two.wrapping_add(1_u32);
        let four = two.wrapping_mul(two);
        let five = four.wrapping_add(1_u32);
        let six = three.wrapping_mul(two);
        let ten = five.wrapping_mul(two);
        let idx = match self {
            Self::Pelvis => 0_u32,
            Self::Spine1 => 1_u32,
            Self::Spine2 => two,
            Self::Chest => three,
            Self::Neck => four,
            Self::Head => five,
            Self::ClavicleL => six,
            Self::UpperArmL => six.wrapping_add(1_u32),
            Self::ForearmL => four.wrapping_mul(two),
            Self::HandL => four.wrapping_mul(two).wrapping_add(1_u32),
            Self::ClavicleR => ten,
            Self::UpperArmR => ten.wrapping_add(1_u32),
            Self::ForearmR => six.wrapping_mul(two),
            Self::HandR => six.wrapping_mul(two).wrapping_add(1_u32),
            Self::ThighL => ten.wrapping_add(four),
            Self::CalfL => ten.wrapping_add(five),
            Self::FootL => four.wrapping_mul(four),
            Self::ToeL => four.wrapping_mul(four).wrapping_add(1_u32),
            Self::ThighR => six.wrapping_mul(three),
            Self::CalfR => six.wrapping_mul(three).wrapping_add(1_u32),
            Self::FootR => ten.wrapping_mul(two),
            Self::ToeR => ten.wrapping_mul(two).wrapping_add(1_u32),
        };
        idx as usize
    }
}

/// Visual role and garment/armor palette of an articulated character in the universe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterArchetype {
    /// Player-possessed cybernetic Explorer in cobalt-slate armor with cyan visor.
    Explorer,
    /// Citadel Hall Archivist in alabaster-and-gold ceremonial robes.
    Archivist,
    /// River Arch Bridge Engineer in bronze-terracotta field gear.
    BridgeEngineer,
    /// High Observatory Keeper in deep indigo-emerald mantle.
    ObservatoryKeeper,
}

/// Live analytical two-bone Foot IK telemetry (`kernel13TwoBoneIkContraction`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FootIkTelemetry {
    /// True when the left foot raycast contacted walkable geometry within reach.
    pub left_planted: bool,
    /// True when the right foot raycast contacted walkable geometry within reach.
    pub right_planted: bool,
    /// Solved left knee flexion angle in radians.
    pub left_knee_bend_radians: f64,
    /// Solved right knee flexion angle in radians.
    pub right_knee_bend_radians: f64,
    /// Smooth pelvis vertical adaptation offset in meters (`<= 0`).
    pub pelvis_sink_meters: f64,
    /// Solved world-space left ankle position.
    pub left_ankle_world: Vec3,
    /// Solved world-space right ankle position.
    pub right_ankle_world: Vec3,
}
