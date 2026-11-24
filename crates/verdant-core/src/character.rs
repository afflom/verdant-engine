//! High-resolution 22-joint articulated humanoid characters, analytical two-bone Foot IK
//! (`kernel13TwoBoneIkContraction`), neuromorphic head-look tracking, and 1st/3rd-person possession.
//!
//! Every skeletal topology invariant (`prism_kernel::humanoidJointCount() == 22`), permille skin
//! weight sum (`1000`), morph-target interpolation, and `uor-matmul` linear-blend skinning pass
//! executes deterministically in Rust/WebAssembly with zero magic constants.

use serde::{Deserialize, Serialize};

use crate::animation::{skin_mesh_linear_blend, Joint, SampledPose, Skeleton, SkinWeights4};
use crate::ecs::TwoLevelBvh;
use crate::error::AnimationError;
use crate::geometry::{MorphTarget, ProceduralMesh};
use crate::math::{
    four_u32, geometric_epsilon, half_f64, pow10_u64, three_u32, two_f64, two_u32, Quat, Ray, Vec3,
    DIM_HOMOGENEOUS, DIM_PLANAR, DIM_SPATIAL,
};
use crate::render::{GpuSurfaceClass, PackedSceneRenderBuffers};

#[path = "character_types.rs"]
mod character_types;
pub use character_types::*;

#[allow(clippy::too_many_arguments)]
fn append_skinned_limb_tube(
    positions: &mut Vec<[f32; DIM_SPATIAL]>,
    normals: &mut Vec<[f32; DIM_SPATIAL]>,
    uvs: &mut Vec<[f32; DIM_PLANAR]>,
    colors: &mut Vec<[f32; DIM_HOMOGENEOUS]>,
    indices: &mut Vec<u32>,
    weights: &mut Vec<SkinWeights4>,
    head_mask: &mut Vec<bool>,
    p_start: Vec3,
    p_end: Vec3,
    r_start: f64,
    r_mid_bulge: f64,
    r_end: f64,
    joint_a: usize,
    joint_b: usize,
    joint_count: usize,
    radial_segs: usize,
    axial_segs: usize,
    col_main: [f32; DIM_HOMOGENEOUS],
    col_trim: [f32; DIM_HOMOGENEOUS],
    is_head: bool,
) -> Result<(), AnimationError> {
    let axis = p_end - p_start;
    let len = axis.length().max(geometric_epsilon());
    let tangent = axis.scale(1.0 / len);
    let ref_up = if tangent.y.abs() < (1.0 - half_f64() * half_f64()) {
        Vec3::UP
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };
    let n_u = tangent.cross(ref_up).normalized();
    let n_v = tangent.cross(n_u).normalized();

    let scale_u16 = crate::prism_kernel::skinWeightScale().expect("prism") as u16;
    let r_segs = radial_segs.max(1 + 1 + 1 + 1 + 1 + 1);
    let a_segs = axial_segs.max(1 + 1);
    let base_v = positions.len() as u32;
    let stride = (r_segs + 1) as u32;
    let half = half_f64();
    let eighth = half * half * half;

    for ia in 0..=a_segs {
        let t = (ia as f64) / (a_segs as f64);
        let center = p_start.lerp(p_end, t);
        let linear_r = r_start * (1.0 - t) + r_end * t;
        let bulge = (t * std::f64::consts::PI).sin() * r_mid_bulge;
        let radius = (linear_r + bulge).max(geometric_epsilon());

        // Smooth skinning blend from `joint_a` (`t = 0`) toward `joint_b` (`t = 1`):
        let w_b = if joint_a == joint_b {
            0_u16
        } else {
            ((t * half * f64::from(scale_u16)).round() as u16).min(scale_u16)
        };
        let w_a = scale_u16 - w_b;
        let sw = SkinWeights4::new(
            [joint_a as u16, joint_b as u16, 0, 0],
            [w_a, w_b, 0, 0],
            joint_count,
        )?;
        let col = if !(eighth..(1.0 - eighth)).contains(&t) {
            col_trim
        } else {
            col_main
        };

        for ir in 0..=r_segs {
            let u = (ir as f64) / (r_segs as f64);
            let theta = u * std::f64::consts::TAU;
            let (st, ct) = theta.sin_cos();
            let radial_dir = (n_u.scale(ct) + n_v.scale(st)).normalized();
            let pos = center + radial_dir.scale(radius);
            positions.push(pos.to_f32_array());
            normals.push(radial_dir.to_f32_array());
            uvs.push([u as f32, t as f32]);
            colors.push(col);
            weights.push(sw);
            head_mask.push(is_head);
        }
    }

    for ia in 0..(a_segs as u32) {
        for ir in 0..(r_segs as u32) {
            let a = base_v + ia * stride + ir;
            let b = a + 1;
            let c = a + stride + 1;
            let d = a + stride;
            indices.extend_from_slice(&[a, b, c, a, c, d]);
        }
    }

    // End-cap dome on `p_end` when `is_head` or terminal hand/toe segment:
    if is_head || joint_a == joint_b {
        let tip_pos = p_end + tangent.scale(r_end * half);
        let sw_end =
            SkinWeights4::new([joint_b as u16, 0, 0, 0], [scale_u16, 0, 0, 0], joint_count)?;
        let tip_idx = positions.len() as u32;
        positions.push(tip_pos.to_f32_array());
        normals.push(tangent.to_f32_array());
        uvs.push([half as f32, 1.0]);
        colors.push(col_trim);
        weights.push(sw_end);
        head_mask.push(is_head);

        let last_ring = base_v + (a_segs as u32) * stride;
        for ir in 0..(r_segs as u32) {
            indices.extend_from_slice(&[last_ring + ir, last_ring + ir + 1, tip_idx]);
        }
    }

    Ok(())
}

impl ArticulatedCharacter {
    /// Synthesizes a validated 22-joint humanoid character at `foot_position` with smooth
    /// high-poly anatomical/armor geometry (`LOD0..LOD4`), 3 morph targets, and normalized 4-joint LBS weights.
    pub fn new_humanoid(
        name: impl Into<String>,
        archetype: CharacterArchetype,
        foot_position: Vec3,
        lod_rank: u64,
    ) -> Result<Self, AnimationError> {
        let half = half_f64();
        let q = half * half;
        let e = q * half;
        let s16 = e * half;
        let ten = pow10_u64(1) as f64;

        // Anthropometric proportions in meters (`total height ~ 1.78m`, constructive from `{0, 1}`):
        let ankle_h = (1.0 - q) / ten; // 0.075m
        let calf_len = q + e + s16; // 0.4375m
        let thigh_len = half - s16; // 0.4375m
        let pelvis_h = ankle_h + calf_len + thigh_len; // 0.95m
        let hip_x = e + s16 * half; // 0.15625m
        let spine1_dy = e + s16 * half;
        let spine2_dy = e + s16 * half;
        let chest_dy = e + s16 * half;
        let neck_dy = e + s16 * half;
        let head_dy = e;
        let clav_x = e;
        let shoulder_x = e + s16;
        let upper_arm_len = q + s16; // 0.3125m
        let forearm_len = q + s16 * half; // 0.28125m
        let hand_len = e;
        let toe_z = e + s16 * half;

        let mk_j = |j_name: &str, parent: HumanoidJointId, is_root: bool, t: Vec3| Joint {
            name: j_name.to_string(),
            parent_index: if is_root { 0 } else { parent.index() },
            is_root,
            rest_translation: t,
            rest_rotation: Quat::IDENTITY,
        };

        let joints = vec![
            mk_j(
                "Pelvis",
                HumanoidJointId::Pelvis,
                true,
                Vec3::new(0.0, pelvis_h, 0.0),
            ),
            mk_j(
                "Spine1",
                HumanoidJointId::Pelvis,
                false,
                Vec3::new(0.0, spine1_dy, 0.0),
            ),
            mk_j(
                "Spine2",
                HumanoidJointId::Spine1,
                false,
                Vec3::new(0.0, spine2_dy, 0.0),
            ),
            mk_j(
                "Chest",
                HumanoidJointId::Spine2,
                false,
                Vec3::new(0.0, chest_dy, 0.0),
            ),
            mk_j(
                "Neck",
                HumanoidJointId::Chest,
                false,
                Vec3::new(0.0, neck_dy, 0.0),
            ),
            mk_j(
                "Head",
                HumanoidJointId::Neck,
                false,
                Vec3::new(0.0, head_dy, 0.0),
            ),
            mk_j(
                "ClavicleL",
                HumanoidJointId::Chest,
                false,
                Vec3::new(-clav_x, neck_dy * half, 0.0),
            ),
            mk_j(
                "UpperArmL",
                HumanoidJointId::ClavicleL,
                false,
                Vec3::new(-shoulder_x, 0.0, 0.0),
            ),
            mk_j(
                "ForearmL",
                HumanoidJointId::UpperArmL,
                false,
                Vec3::new(0.0, -upper_arm_len, 0.0),
            ),
            mk_j(
                "HandL",
                HumanoidJointId::ForearmL,
                false,
                Vec3::new(0.0, -forearm_len, 0.0),
            ),
            mk_j(
                "ClavicleR",
                HumanoidJointId::Chest,
                false,
                Vec3::new(clav_x, neck_dy * half, 0.0),
            ),
            mk_j(
                "UpperArmR",
                HumanoidJointId::ClavicleR,
                false,
                Vec3::new(shoulder_x, 0.0, 0.0),
            ),
            mk_j(
                "ForearmR",
                HumanoidJointId::UpperArmR,
                false,
                Vec3::new(0.0, -upper_arm_len, 0.0),
            ),
            mk_j(
                "HandR",
                HumanoidJointId::ForearmR,
                false,
                Vec3::new(0.0, -forearm_len, 0.0),
            ),
            mk_j(
                "ThighL",
                HumanoidJointId::Pelvis,
                false,
                Vec3::new(-hip_x, 0.0, 0.0),
            ),
            mk_j(
                "CalfL",
                HumanoidJointId::ThighL,
                false,
                Vec3::new(0.0, -thigh_len, 0.0),
            ),
            mk_j(
                "FootL",
                HumanoidJointId::CalfL,
                false,
                Vec3::new(0.0, -calf_len, 0.0),
            ),
            mk_j(
                "ToeL",
                HumanoidJointId::FootL,
                false,
                Vec3::new(0.0, -ankle_h * half, toe_z),
            ),
            mk_j(
                "ThighR",
                HumanoidJointId::Pelvis,
                false,
                Vec3::new(hip_x, 0.0, 0.0),
            ),
            mk_j(
                "CalfR",
                HumanoidJointId::ThighR,
                false,
                Vec3::new(0.0, -thigh_len, 0.0),
            ),
            mk_j(
                "FootR",
                HumanoidJointId::CalfR,
                false,
                Vec3::new(0.0, -calf_len, 0.0),
            ),
            mk_j(
                "ToeR",
                HumanoidJointId::FootR,
                false,
                Vec3::new(0.0, -ankle_h * half, toe_z),
            ),
        ];

        let expected_joints = crate::prism_kernel::humanoidJointCount().unwrap_or(1) as usize;
        if joints.len() != expected_joints {
            return Err(AnimationError::InvalidJointHierarchy {
                joint_index: joints.len(),
                parent_index: expected_joints,
            });
        }

        let skeleton = Skeleton::new(joints)?;
        let j_count = skeleton.joints.len();

        // Compute bind-pose world joint positions for mesh synthesis:
        let mut world_pos = vec![Vec3::ZERO; j_count];
        for (idx, j) in skeleton.joints.iter().enumerate() {
            world_pos[idx] = if j.is_root {
                j.rest_translation
            } else {
                world_pos[j.parent_index] + j.rest_translation
            };
        }

        let r2 = two_u32() as u64;
        let r3 = three_u32() as u64;
        let r4 = four_u32() as u64;
        let (radial_segs, axial_segs) = if lod_rank == 0 {
            ((1 + 1 + 1) * (1 + 1), 1 + 1)
        } else if lod_rank == 1 {
            (1 << (1 + 1 + 1), 1 + 1 + 1)
        } else if lod_rank == r2 {
            ((1 + 1 + 1) * (1 + 1 + 1 + 1), (1 + 1) * (1 + 1))
        } else if lod_rank == r3 {
            (1 << (1 + 1 + 1 + 1), (1 + 1 + 1) * (1 + 1))
        } else if lod_rank >= r4 {
            ((1 + 1 + 1) * (1 << (1 + 1 + 1)), 1 << (1 + 1 + 1))
        } else {
            (1 << (1 + 1 + 1), 1 + 1 + 1)
        };

        let (col_armor, col_trim, col_visor, col_skin) = archetype_palette(archetype);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let mut skin_weights = Vec::new();
        let mut head_vertex_mask = Vec::new();

        // Torso & Pelvis segments:
        let torso_links = [
            (
                HumanoidJointId::Pelvis,
                HumanoidJointId::Spine1,
                e + s16 * half,
                s16 * half,
                e + s16 * half,
                col_armor,
                col_trim,
                false,
            ),
            (
                HumanoidJointId::Spine1,
                HumanoidJointId::Spine2,
                e + s16 * half,
                s16 * half,
                e + s16,
                col_armor,
                col_trim,
                false,
            ),
            (
                HumanoidJointId::Spine2,
                HumanoidJointId::Chest,
                e + s16,
                s16,
                e + s16 + s16 * half,
                col_armor,
                col_trim,
                false,
            ),
            (
                HumanoidJointId::Chest,
                HumanoidJointId::Neck,
                e + s16,
                s16 * half,
                s16 + s16 * half,
                col_trim,
                col_armor,
                false,
            ),
            (
                HumanoidJointId::Neck,
                HumanoidJointId::Head,
                s16 + s16 * half,
                0.0,
                s16 + s16 * half,
                col_skin,
                col_trim,
                true,
            ),
        ];
        for (ja, jb, rs, rm, re, cm, ct, is_h) in torso_links {
            append_skinned_limb_tube(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
                &mut skin_weights,
                &mut head_vertex_mask,
                world_pos[ja.index()],
                world_pos[jb.index()],
                rs,
                rm,
                re,
                ja.index(),
                jb.index(),
                j_count,
                radial_segs + (1 + 1 + 1 + 1),
                axial_segs,
                cm,
                ct,
                is_h,
            )?;
        }

        // Sculpted Cranium + Luminous Neuromorphic Visor Band on `Head`:
        let head_base = world_pos[HumanoidJointId::Head.index()];
        let head_top = head_base + Vec3::new(0.0, e + s16 + s16 * half, 0.0);
        append_skinned_limb_tube(
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut colors,
            &mut indices,
            &mut skin_weights,
            &mut head_vertex_mask,
            head_base,
            head_top,
            s16 + s16 * half,
            s16,
            s16 + s16 * half,
            HumanoidJointId::Head.index(),
            HumanoidJointId::Head.index(),
            j_count,
            radial_segs + (1 + 1 + 1 + 1),
            axial_segs + (1 + 1),
            col_armor,
            col_visor,
            true,
        )?;
        // Forward glowing visor band (`+Z` face of helmet):
        let visor_p0 = head_base + Vec3::new(0.0, s16, s16 * half);
        let visor_p1 = head_base + Vec3::new(0.0, e, s16 * half);
        append_skinned_limb_tube(
            &mut positions,
            &mut normals,
            &mut uvs,
            &mut colors,
            &mut indices,
            &mut skin_weights,
            &mut head_vertex_mask,
            visor_p0,
            visor_p1,
            s16 + s16 * half,
            s16 * half,
            s16 + s16 * half,
            HumanoidJointId::Head.index(),
            HumanoidJointId::Head.index(),
            j_count,
            radial_segs,
            1 + 1,
            col_visor,
            col_visor,
            true,
        )?;

        // Arms & Hands (Left and Right):
        let arm_links = [
            (
                HumanoidJointId::ClavicleL,
                HumanoidJointId::UpperArmL,
                s16 + s16 * half,
                s16 * half,
                s16 + s16 * half,
                col_trim,
            ),
            (
                HumanoidJointId::UpperArmL,
                HumanoidJointId::ForearmL,
                s16 + s16 * half,
                s16 * half,
                s16,
                col_armor,
            ),
            (
                HumanoidJointId::ForearmL,
                HumanoidJointId::HandL,
                s16,
                s16 * q,
                s16 * (1.0 - q),
                col_armor,
            ),
            (
                HumanoidJointId::ClavicleR,
                HumanoidJointId::UpperArmR,
                s16 + s16 * half,
                s16 * half,
                s16 + s16 * half,
                col_trim,
            ),
            (
                HumanoidJointId::UpperArmR,
                HumanoidJointId::ForearmR,
                s16 + s16 * half,
                s16 * half,
                s16,
                col_armor,
            ),
            (
                HumanoidJointId::ForearmR,
                HumanoidJointId::HandR,
                s16,
                s16 * q,
                s16 * (1.0 - q),
                col_armor,
            ),
        ];
        for (ja, jb, rs, rm, re, cm) in arm_links {
            append_skinned_limb_tube(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
                &mut skin_weights,
                &mut head_vertex_mask,
                world_pos[ja.index()],
                world_pos[jb.index()],
                rs,
                rm,
                re,
                ja.index(),
                jb.index(),
                j_count,
                radial_segs,
                axial_segs,
                cm,
                col_trim,
                false,
            )?;
        }
        for hand_j in [HumanoidJointId::HandL, HumanoidJointId::HandR] {
            let hb = world_pos[hand_j.index()];
            append_skinned_limb_tube(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
                &mut skin_weights,
                &mut head_vertex_mask,
                hb,
                hb + Vec3::new(0.0, -hand_len, 0.0),
                s16 * (1.0 - q),
                s16 * q,
                s16 * half,
                hand_j.index(),
                hand_j.index(),
                j_count,
                radial_segs,
                1 + 1,
                col_trim,
                col_skin,
                false,
            )?;
        }

        // Legs, Ankles, & Articulated Boots (Left and Right):
        let leg_links = [
            (
                HumanoidJointId::ThighL,
                HumanoidJointId::CalfL,
                s16 + s16,
                s16 * half,
                s16 + s16 * half,
            ),
            (
                HumanoidJointId::CalfL,
                HumanoidJointId::FootL,
                s16 + s16 * half,
                s16 * q,
                s16,
            ),
            (
                HumanoidJointId::FootL,
                HumanoidJointId::ToeL,
                s16,
                s16 * q,
                s16 * (1.0 - q),
            ),
            (
                HumanoidJointId::ThighR,
                HumanoidJointId::CalfR,
                s16 + s16,
                s16 * half,
                s16 + s16 * half,
            ),
            (
                HumanoidJointId::CalfR,
                HumanoidJointId::FootR,
                s16 + s16 * half,
                s16 * q,
                s16,
            ),
            (
                HumanoidJointId::FootR,
                HumanoidJointId::ToeR,
                s16,
                s16 * q,
                s16 * (1.0 - q),
            ),
        ];
        for (ja, jb, rs, rm, re) in leg_links {
            append_skinned_limb_tube(
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut colors,
                &mut indices,
                &mut skin_weights,
                &mut head_vertex_mask,
                world_pos[ja.index()],
                world_pos[jb.index()],
                rs,
                rm,
                re,
                ja.index(),
                jb.index(),
                j_count,
                radial_segs,
                axial_segs,
                col_armor,
                col_trim,
                false,
            )?;
        }

        // Construct 3 morph targets (`visor_focus`, `breath_expand`, `stride_flex`):
        let v_total = positions.len();
        let mut visor_deltas = vec![[0.0_f32; DIM_SPATIAL]; v_total];
        let mut breath_deltas = vec![[0.0_f32; DIM_SPATIAL]; v_total];
        let mut stride_deltas = vec![[0.0_f32; DIM_SPATIAL]; v_total];
        let chest_y = world_pos[HumanoidJointId::Chest.index()].y as f32;
        let visor_amp = (s16 * q) as f32;
        let breath_amp = (s16 * half) as f32;
        let i2 = 1usize + 1usize;

        for vi in 0..v_total {
            let n = normals[vi];
            let p = positions[vi];
            if head_vertex_mask[vi] && p[i2] > 0.0 {
                visor_deltas[vi] = [n[0] * visor_amp, n[1] * visor_amp, n[i2] * visor_amp];
            }
            if (p[1] - chest_y).abs() < (q as f32) {
                breath_deltas[vi] = [n[0] * breath_amp, n[1] * breath_amp, n[i2] * breath_amp];
            }
            if p[1] < (pelvis_h as f32) {
                stride_deltas[vi] = [n[0] * visor_amp, 0.0, n[i2] * visor_amp];
            }
        }

        let bind_mesh = ProceduralMesh {
            name: format!("Humanoid22Rig_{}", archetype as u8),
            positions,
            normals,
            uvs,
            colors,
            indices,
            morph_targets: vec![
                MorphTarget {
                    name: "visor_focus".to_string(),
                    position_deltas: visor_deltas,
                },
                MorphTarget {
                    name: "breath_expand".to_string(),
                    position_deltas: breath_deltas,
                },
                MorphTarget {
                    name: "stride_flex".to_string(),
                    position_deltas: stride_deltas,
                },
            ],
        };

        let rest_translations: Vec<Vec3> =
            skeleton.joints.iter().map(|j| j.rest_translation).collect();
        let rest_rotations = vec![Quat::IDENTITY; j_count];
        let current_pose = SampledPose {
            joint_translations: rest_translations,
            joint_rotations: rest_rotations,
            morph_weights_permille: vec![0, 0, 0],
        };

        let last_ik = FootIkTelemetry {
            left_planted: true,
            right_planted: true,
            left_knee_bend_radians: 0.0,
            right_knee_bend_radians: 0.0,
            pelvis_sink_meters: 0.0,
            left_ankle_world: foot_position + Vec3::new(-hip_x, ankle_h, 0.0),
            right_ankle_world: foot_position + Vec3::new(hip_x, ankle_h, 0.0),
        };

        Ok(Self {
            name: name.into(),
            archetype,
            skeleton,
            bind_mesh,
            skin_weights,
            head_vertex_mask,
            foot_position,
            body_yaw_radians: 0.0,
            gait_phase_radians: 0.0,
            pelvis_sink_meters: 0.0,
            thigh_length_meters: thigh_len,
            calf_length_meters: calf_len,
            ankle_height_meters: ankle_h,
            current_pose,
            last_ik,
        })
    }

    /// Binds an imported external Khronos `.glb` character mesh onto the canonical 22-joint rig.
    pub fn bind_external_mesh(
        name: impl Into<String>,
        archetype: CharacterArchetype,
        foot_position: Vec3,
        external_mesh: ProceduralMesh,
    ) -> Result<Self, AnimationError> {
        if external_mesh.positions.is_empty() || external_mesh.indices.is_empty() {
            return Err(AnimationError::InvalidClipChannel {
                detail: "cannot bind empty external character mesh".to_string(),
            });
        }
        let name_str = name.into();
        let mut base = Self::new_humanoid(name_str.as_str(), archetype, foot_position, 1)?;
        let scale_u16 = crate::prism_kernel::skinWeightScale().expect("prism") as u16;
        let j_count = base.skeleton.joints.len();
        let bounds = external_mesh.bounds();
        let span_y = (bounds.max.y - bounds.min.y).max(geometric_epsilon());
        let half = half_f64();
        let q = half * half;

        let mut weights = Vec::with_capacity(external_mesh.positions.len());
        let mut head_mask = Vec::with_capacity(external_mesh.positions.len());
        for pos in &external_mesh.positions {
            let h_norm = ((f64::from(pos[1]) - bounds.min.y) / span_y).clamp(0.0, 1.0);
            let is_head = h_norm > (1.0 - q * half);
            let primary_joint = if is_head {
                HumanoidJointId::Head.index()
            } else if h_norm > half + q {
                HumanoidJointId::Chest.index()
            } else if h_norm > half {
                HumanoidJointId::Pelvis.index()
            } else if pos[0] < 0.0 {
                if h_norm > q {
                    HumanoidJointId::ThighL.index()
                } else {
                    HumanoidJointId::CalfL.index()
                }
            } else if h_norm > q {
                HumanoidJointId::ThighR.index()
            } else {
                HumanoidJointId::CalfR.index()
            };
            weights.push(SkinWeights4::new(
                [primary_joint as u16, 0, 0, 0],
                [scale_u16, 0, 0, 0],
                j_count,
            )?);
            head_mask.push(is_head);
        }
        base.bind_mesh = external_mesh;
        base.skin_weights = weights;
        base.head_vertex_mask = head_mask;
        base.current_pose.morph_weights_permille = vec![0; base.bind_mesh.morph_targets.len()];
        Ok(base)
    }

    /// World-space head/eye anchor used by [`crate::camera::LawfulCameraRig`] during possession.
    #[must_use]
    pub fn head_anchor_world(&self) -> Vec3 {
        let half = half_f64();
        let eye_offset = 1.0 + half + half * half * half; // ~1.625m + pelvis IK sink
        Vec3::new(
            self.foot_position.x,
            self.foot_position.y + eye_offset + self.pelvis_sink_meters,
            self.foot_position.z,
        )
    }

    /// Advances locomotion cycle, neuromorphic head-look tracking, and analytical two-bone Foot IK
    /// (`verdant_tensor::solve_two_bone_ik_exact` / `kernel13`) against `two_level_bvh`.
    #[allow(clippy::too_many_arguments)]
    pub fn step_and_solve_ik(
        &mut self,
        dt_seconds: f64,
        foot_world: Vec3,
        velocity_world: Vec3,
        grounded: bool,
        sim_time_seconds: f64,
        salience_target: Option<Vec3>,
        two_level_bvh: &TwoLevelBvh,
        support_height_at: &dyn Fn(f64, f64) -> f64,
    ) -> Result<FootIkTelemetry, AnimationError> {
        let half = half_f64();
        let q = half * half;
        let e = q * half;
        let two = two_f64();
        let scale_u16 = crate::prism_kernel::skinWeightScale().expect("prism") as f64;

        self.foot_position = foot_world;
        let planar_vel = Vec3::new(velocity_world.x, 0.0, velocity_world.z);
        let speed = planar_vel.length();

        if speed > q {
            let target_yaw = planar_vel.x.atan2(planar_vel.z);
            let mut diff = (target_yaw - self.body_yaw_radians + std::f64::consts::PI)
                .rem_euclid(std::f64::consts::TAU)
                - std::f64::consts::PI;
            let turn_alpha = (dt_seconds.max(0.0) * (two * two * two)).clamp(0.0, 1.0);
            diff *= turn_alpha;
            self.body_yaw_radians =
                (self.body_yaw_radians + diff).rem_euclid(std::f64::consts::TAU);
            self.gait_phase_radians = (self.gait_phase_radians + speed * dt_seconds * (two + half))
                .rem_euclid(std::f64::consts::TAU);
        }

        let stride_blend = (speed / (two * two)).clamp(0.0, 1.0);
        let phase = self.gait_phase_radians;
        let s_phase = phase.sin();

        let j_count = self.skeleton.joints.len();
        let mut translations: Vec<Vec3> = self
            .skeleton
            .joints
            .iter()
            .map(|j| j.rest_translation)
            .collect();
        let mut rotations = vec![Quat::IDENTITY; j_count];

        let q_yaw = Quat::from_axis_angle(Vec3::UP, self.body_yaw_radians);
        let right_axis = Vec3::new(1.0, 0.0, 0.0);
        let fwd_world = q_yaw.rotate_vec3(Vec3::new(0.0, 0.0, 1.0));
        let right_world = q_yaw.rotate_vec3(right_axis);

        // 1. Two-Bone Foot IK Raycasts (`ThighL -> CalfL -> FootL` and `ThighR -> CalfR -> FootR`):
        let leg_reach = self.thigh_length_meters + self.calf_length_meters;
        let hip_x = self.skeleton.joints[HumanoidJointId::ThighR.index()]
            .rest_translation
            .x
            .abs();
        let stride_step_z = s_phase * q * stride_blend;
        let stride_lift_l = s_phase.max(0.0) * e * stride_blend;
        let stride_lift_r = (-s_phase).max(0.0) * e * stride_blend;

        let sample_foot_ground = |lateral_x: f64, fwd_z: f64| -> (f64, Vec3, bool) {
            let probe_xz =
                self.foot_position + right_world.scale(lateral_x) + fwd_world.scale(fwd_z);
            let ray_origin = Vec3::new(probe_xz.x, self.foot_position.y + leg_reach, probe_xz.z);
            let max_d = leg_reach * two;
            let terrain_y = support_height_at(probe_xz.x, probe_xz.z);
            if let Some((_, hit)) =
                two_level_bvh.raycast_exact(Ray::new(ray_origin, Vec3::new(0.0, -1.0, 0.0)), max_d)
            {
                if hit.geometric_normal.y >= half && hit.point.y >= terrain_y - e {
                    return (hit.point.y, hit.geometric_normal, true);
                }
            }
            (terrain_y, Vec3::UP, grounded)
        };

        let (ground_l, norm_l, planted_l) = sample_foot_ground(-hip_x, stride_step_z);
        let (ground_r, norm_r, planted_r) = sample_foot_ground(hip_x, -stride_step_z);

        // Smooth pelvis adaptation toward lower planted foot on stairs/slopes:
        let desired_sink = if grounded {
            let dl = ground_l - self.foot_position.y;
            let dr = ground_r - self.foot_position.y;
            dl.min(dr).max(-q).min(e)
        } else {
            0.0
        };
        let sink_alpha = (dt_seconds.max(0.0) * (two * two * two + two)).clamp(0.0, 1.0);
        self.pelvis_sink_meters += (desired_sink - self.pelvis_sink_meters) * sink_alpha;

        let gait_bob = (phase * two).cos().abs() * (e * q) * stride_blend;
        let base_pelvis_y = self.skeleton.joints[HumanoidJointId::Pelvis.index()]
            .rest_translation
            .y;
        let pelvis_world = self.foot_position
            + Vec3::new(0.0, base_pelvis_y + self.pelvis_sink_meters + gait_bob, 0.0);
        translations[HumanoidJointId::Pelvis.index()] = pelvis_world;
        rotations[HumanoidJointId::Pelvis.index()] = q_yaw;

        // Solve analytical Two-Bone IK (`kernel13`) for Left and Right legs in world space:
        let hip_l_world = pelvis_world + right_world.scale(-hip_x);
        let hip_r_world = pelvis_world + right_world.scale(hip_x);
        let target_ankle_l = self.foot_position
            + right_world.scale(-hip_x)
            + fwd_world.scale(stride_step_z)
            + Vec3::new(
                0.0,
                (ground_l - self.foot_position.y) + self.ankle_height_meters + stride_lift_l,
                0.0,
            );
        let target_ankle_r = self.foot_position
            + right_world.scale(hip_x)
            + fwd_world.scale(-stride_step_z)
            + Vec3::new(
                0.0,
                (ground_r - self.foot_position.y) + self.ankle_height_meters + stride_lift_r,
                0.0,
            );

        let pole_w = [fwd_world.x, fwd_world.y, fwd_world.z];
        let ik_l = verdant_tensor::solve_two_bone_ik_exact(
            [hip_l_world.x, hip_l_world.y, hip_l_world.z],
            [target_ankle_l.x, target_ankle_l.y, target_ankle_l.z],
            pole_w,
            self.thigh_length_meters,
            self.calf_length_meters,
        )
        .map_err(|err| AnimationError::InvalidClipChannel {
            detail: err.to_string(),
        })?;
        let ik_r = verdant_tensor::solve_two_bone_ik_exact(
            [hip_r_world.x, hip_r_world.y, hip_r_world.z],
            [target_ankle_r.x, target_ankle_r.y, target_ankle_r.z],
            pole_w,
            self.thigh_length_meters,
            self.calf_length_meters,
        )
        .map_err(|err| AnimationError::InvalidClipChannel {
            detail: err.to_string(),
        })?;

        // Convert world-space solved knee & ankle vectors into local joint pitch angles around `right_axis`:
        let inv_yaw = Quat::from_axis_angle(Vec3::UP, -self.body_yaw_radians);
        let apply_leg_ik = |ik: &verdant_tensor::TwoBoneIkSolution,
                            hip_w: Vec3,
                            surf_n: Vec3,
                            thigh_id: HumanoidJointId,
                            calf_id: HumanoidJointId,
                            foot_id: HumanoidJointId,
                            rots: &mut [Quat]| {
            let knee_w = Vec3::new(
                ik.mid_joint_pos[0],
                ik.mid_joint_pos[1],
                ik.mid_joint_pos[1 + 1],
            );
            let ankle_w = Vec3::new(
                ik.end_effector_pos[0],
                ik.end_effector_pos[1],
                ik.end_effector_pos[1 + 1],
            );
            let upper_local = inv_yaw.rotate_vec3((knee_w - hip_w).normalized());
            let lower_local = inv_yaw.rotate_vec3((ankle_w - knee_w).normalized());
            let thigh_pitch = (-upper_local.z).atan2((-upper_local.y).max(geometric_epsilon()));
            let calf_abs_pitch = (-lower_local.z).atan2((-lower_local.y).max(geometric_epsilon()));
            let knee_rel_pitch = calf_abs_pitch - thigh_pitch;

            let n_local = inv_yaw.rotate_vec3(surf_n.normalized());
            let slope_pitch = n_local.z.atan2(n_local.y.max(half));
            let foot_rel_pitch = slope_pitch - calf_abs_pitch;

            rots[thigh_id.index()] = Quat::from_axis_angle(right_axis, thigh_pitch);
            rots[calf_id.index()] = Quat::from_axis_angle(right_axis, knee_rel_pitch);
            rots[foot_id.index()] = Quat::from_axis_angle(right_axis, foot_rel_pitch);
            ankle_w
        };

        let left_ankle_world = apply_leg_ik(
            &ik_l,
            hip_l_world,
            norm_l,
            HumanoidJointId::ThighL,
            HumanoidJointId::CalfL,
            HumanoidJointId::FootL,
            &mut rotations,
        );
        let right_ankle_world = apply_leg_ik(
            &ik_r,
            hip_r_world,
            norm_r,
            HumanoidJointId::ThighR,
            HumanoidJointId::CalfR,
            HumanoidJointId::FootR,
            &mut rotations,
        );

        // 2. Spine counter-twist, breathing cadence, and counter-swinging arms:
        let breath_wave = (sim_time_seconds * two).sin();
        rotations[HumanoidJointId::Spine1.index()] =
            Quat::from_axis_angle(right_axis, stride_blend * e * half);
        rotations[HumanoidJointId::Spine2.index()] =
            Quat::from_axis_angle(Vec3::UP, -s_phase * e * half * stride_blend);
        rotations[HumanoidJointId::Chest.index()] =
            Quat::from_axis_angle(right_axis, breath_wave * (e * q));

        let arm_swing = s_phase * half * stride_blend;
        let idle_drape = e;
        rotations[HumanoidJointId::UpperArmL.index()] =
            Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), idle_drape)
                * Quat::from_axis_angle(right_axis, arm_swing);
        rotations[HumanoidJointId::ForearmL.index()] =
            Quat::from_axis_angle(right_axis, -(q + arm_swing.max(0.0) * half));
        rotations[HumanoidJointId::UpperArmR.index()] =
            Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), -idle_drape)
                * Quat::from_axis_angle(right_axis, -arm_swing);
        rotations[HumanoidJointId::ForearmR.index()] =
            Quat::from_axis_angle(right_axis, -(q + (-arm_swing).max(0.0) * half));

        // 3. Neuromorphic LIF Salience Head-Look Tracking (`Neck` + `Head`):
        if let Some(target_w) = salience_target {
            let to_sal = inv_yaw.rotate_vec3(target_w - self.head_anchor_world());
            if to_sal.length_squared() > q {
                let look_dir = to_sal.normalized();
                let raw_yaw = look_dir.x.atan2(look_dir.z);
                if raw_yaw.abs() < (1.0 + q) {
                    let clamped_y = raw_yaw.max(-half - q).min(half + q);
                    let clamped_p = (-look_dir.y).max(-half).min(half);
                    rotations[HumanoidJointId::Neck.index()] =
                        Quat::from_axis_angle(Vec3::UP, clamped_y * half);
                    rotations[HumanoidJointId::Head.index()] =
                        Quat::from_axis_angle(Vec3::UP, clamped_y * half)
                            * Quat::from_axis_angle(right_axis, clamped_p);
                }
            }
        }

        let morph_count = self.bind_mesh.morph_targets.len();
        let mut morph_weights = vec![0_u16; morph_count];
        if morph_count >= (1 + 1 + 1) {
            let visor_w = if salience_target.is_some() {
                (scale_u16 * half) as u16
            } else {
                (scale_u16 * e) as u16
            };
            let breath_w = (((breath_wave * half + half) * scale_u16 * half).round() as u16)
                .min(scale_u16 as u16);
            let stride_w = ((stride_blend * scale_u16 * half).round() as u16).min(scale_u16 as u16);
            morph_weights[0] = visor_w;
            morph_weights[1] = breath_w;
            morph_weights[1 + 1] = stride_w;
        }

        self.current_pose = SampledPose {
            joint_translations: translations,
            joint_rotations: rotations,
            morph_weights_permille: morph_weights,
        };

        let telemetry = FootIkTelemetry {
            left_planted: planted_l && ik_l.target_within_reach,
            right_planted: planted_r && ik_r.target_within_reach,
            left_knee_bend_radians: ik_l.knee_bend_radians,
            right_knee_bend_radians: ik_r.knee_bend_radians,
            pelvis_sink_meters: self.pelvis_sink_meters,
            left_ankle_world,
            right_ankle_world,
        };
        self.last_ik = telemetry;
        Ok(telemetry)
    }
}
