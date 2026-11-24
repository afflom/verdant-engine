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

/// Complete 22-joint articulated character with smooth LBS mesh, morph targets, and Foot IK state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticulatedCharacter {
    /// Character identifier.
    pub name: String,
    /// Role palette.
    pub archetype: CharacterArchetype,
    /// Verified 22-joint acyclic skeleton (`prism_kernel::humanoidJointCount() == 22`).
    pub skeleton: Skeleton,
    /// Bind-pose smooth anatomical + armored mesh with 3 morph target channels.
    pub bind_mesh: ProceduralMesh,
    /// Validated per-vertex 4-joint normalized skinning weights (`sum == 1000`).
    pub skin_weights: Vec<SkinWeights4>,
    /// Per-vertex flag marking head/visor vertices hidden during 1st-person possession.
    pub head_vertex_mask: Vec<bool>,
    /// World-space foot base position (`(x, y, z)`).
    pub foot_position: Vec3,
    /// Smoothed character body facing yaw in radians.
    pub body_yaw_radians: f64,
    /// Continuous locomotion stride phase in radians.
    pub gait_phase_radians: f64,
    /// Smoothed pelvis vertical IK sink in meters.
    pub pelvis_sink_meters: f64,
    /// Thigh bone length in meters (`Thigh -> Calf`).
    pub thigh_length_meters: f64,
    /// Calf bone length in meters (`Calf -> Foot`).
    pub calf_length_meters: f64,
    /// Ankle height above sole in meters.
    pub ankle_height_meters: f64,
    /// Latest sampled 22-joint local pose.
    pub current_pose: SampledPose,
    /// Latest two-bone Foot IK telemetry.
    pub last_ik: FootIkTelemetry,
}

pub(super) fn archetype_palette(
    archetype: CharacterArchetype,
) -> (
    [f32; DIM_HOMOGENEOUS],
    [f32; DIM_HOMOGENEOUS],
    [f32; DIM_HOMOGENEOUS],
    [f32; DIM_HOMOGENEOUS],
) {
    let half = half_f64() as f32;
    let q = half * half;
    let e = q * half;
    let skin = [1.0 - q, half + q - e * half, half + e, 1.0];
    match archetype {
        CharacterArchetype::Explorer => (
            [q - e * half, q + e, half + e, 1.0],
            [1.0 - e, 1.0 - e, 1.0 - q * half, 1.0],
            [q * half, 1.0 - e * half, 1.0, 1.0],
            skin,
        ),
        CharacterArchetype::Archivist => (
            [1.0 - e, 1.0 - e - e * half, 1.0 - q, 1.0],
            [1.0 - e, half + q, q, 1.0],
            [half, 1.0 - q * half, 1.0 - e, 1.0],
            skin,
        ),
        CharacterArchetype::BridgeEngineer => (
            [half + q, half - e, q, 1.0],
            [q + e, q + e, q + e + e * half, 1.0],
            [1.0, half + q, q, 1.0],
            skin,
        ),
        CharacterArchetype::ObservatoryKeeper => (
            [q * half + e * half, q + e, q + e * half, 1.0],
            [half + q, half + q, half + e, 1.0],
            [q, 1.0, half + q, 1.0],
            skin,
        ),
    }
}

impl ArticulatedCharacter {
    /// Evaluates linear-blend skinning (`uor-matmul`) on the current pose and returns the deformed world-space mesh.
    pub fn evaluate_skinned_mesh(&self) -> Result<ProceduralMesh, AnimationError> {
        skin_mesh_linear_blend(
            &self.bind_mesh,
            &self.skeleton,
            &self.current_pose,
            &self.skin_weights,
        )
    }

    /// Appends posed, LBS-skinned character triangles into `out` (`PackedSceneRenderBuffers`),
    /// skipping software `tile_triangles` materialization when `build_software_tiles` is `false`.
    pub fn append_skinned_to_buffers_opts(
        &self,
        out: &mut PackedSceneRenderBuffers,
        hide_head_for_first_person: bool,
        build_software_tiles: bool,
    ) -> Result<usize, AnimationError> {
        let skinned = self.evaluate_skinned_mesh()?;
        let base_vertex = (out.positions_xyz.len() / DIM_SPATIAL) as u32;
        let i2 = 1usize + 1usize;

        out.positions_xyz
            .reserve(skinned.positions.len() * DIM_SPATIAL);
        out.normals_xyz.reserve(skinned.normals.len() * DIM_SPATIAL);
        out.colors_rgb.reserve(skinned.colors.len() * DIM_SPATIAL);

        let (_, _, visor, _) = archetype_palette(self.archetype);
        let body_class = out.surface_class;
        for v_idx in 0..skinned.positions.len() {
            out.positions_xyz
                .extend_from_slice(&skinned.positions[v_idx]);
            out.normals_xyz.extend_from_slice(&skinned.normals[v_idx]);
            let c = skinned.colors[v_idx];
            let rgb = [c[0], c[1], c[i2]];
            let class = if rgb == [visor[0], visor[1], visor[i2]] {
                GpuSurfaceClass::EmissiveGlass
            } else {
                body_class
            };
            out.colors_rgb.extend_from_slice(&class.encode(rgb));
        }

        let mut added_tris = 0usize;
        for tri in skinned.indices.chunks_exact(DIM_SPATIAL) {
            let ia = tri[0] as usize;
            let ib = tri[1] as usize;
            let ic = tri[i2] as usize;
            if hide_head_for_first_person
                && (self.head_vertex_mask.get(ia).copied().unwrap_or(false)
                    || self.head_vertex_mask.get(ib).copied().unwrap_or(false)
                    || self.head_vertex_mask.get(ic).copied().unwrap_or(false))
            {
                continue;
            }
            let a = base_vertex + tri[0];
            let b = base_vertex + tri[1];
            let c = base_vertex + tri[i2];
            out.indices.extend_from_slice(&[a, b, c]);
            added_tris += 1;
        }
        if build_software_tiles {
            out.ensure_tile_triangles_built();
        }
        Ok(added_tris)
    }

    /// Appends the posed, LBS-skinned character triangles into `out` (`PackedSceneRenderBuffers`),
    /// optionally hiding head/visor triangles when possessed in `CameraMode::FirstPersonEye`.
    pub fn append_skinned_to_buffers(
        &self,
        out: &mut PackedSceneRenderBuffers,
        hide_head_for_first_person: bool,
    ) -> Result<usize, AnimationError> {
        self.append_skinned_to_buffers_opts(out, hide_head_for_first_person, true)
    }
}
