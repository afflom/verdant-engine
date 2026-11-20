//! Skeletal joint hierarchies, four-joint linear-blend skinning, and keyframe/morph animation.
//!
//! All topological invariants, permille weight normalization, morph channel limits, and fixed-point
//! keyframe interpolations execute through the Lean-verified `crate::prism_kernel` emitted by Prism.

use serde::{Deserialize, Serialize};

use crate::error::AnimationError;
use crate::geometry::ProceduralMesh;
use crate::math::{pow10_u64, three_u32, Mat4, Quat, Vec3, DIM_HOMOGENEOUS};

/// Single joint in an articulated skeletal hierarchy.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Joint {
    /// Joint identifier (`root`, `spine`, `turret`, `limb_l`, etc.).
    pub name: String,
    /// Parent joint index (`0` for the root joint; strictly `< joint_index` for child joints).
    pub parent_index: usize,
    /// True only for root joint `0`.
    pub is_root: bool,
    /// Bind-pose translation relative to parent joint in meters.
    pub rest_translation: Vec3,
    /// Bind-pose unit rotation relative to parent joint.
    pub rest_rotation: Quat,
}

/// Validated acyclic skeletal rig with precomputed inverse-bind matrices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skeleton {
    /// Topologically ordered joints (`parent_index < index` for all non-root joints).
    pub joints: Vec<Joint>,
    /// Rigid world-to-joint inverse-bind matrices in bind pose.
    pub inverse_bind_matrices: Vec<Mat4>,
}

impl Skeleton {
    /// Validates topological acyclicity via `prism_kernel::jointHierarchyAcyclic` and precomputes
    /// world bind and inverse-bind transforms.
    pub fn new(joints: Vec<Joint>) -> Result<Self, AnimationError> {
        if joints.is_empty() {
            return Err(AnimationError::InvalidJointHierarchy {
                joint_index: 0,
                parent_index: 0,
            });
        }

        let unit_scale = Vec3::new(1.0, 1.0, 1.0);
        let mut world_rotations: Vec<Quat> = Vec::with_capacity(joints.len());
        let mut world_translations: Vec<Vec3> = Vec::with_capacity(joints.len());
        let mut inverse_bind_matrices: Vec<Mat4> = Vec::with_capacity(joints.len());

        for (idx, joint) in joints.iter().enumerate() {
            let acyclic = crate::prism_kernel::jointHierarchyAcyclic(
                idx as u64,
                joint.parent_index as u64,
                joint.is_root,
            );
            if !acyclic || joint.parent_index >= joints.len() {
                return Err(AnimationError::InvalidJointHierarchy {
                    joint_index: idx,
                    parent_index: joint.parent_index,
                });
            }

            let local_rot = joint.rest_rotation.normalized();
            let (world_rot, world_trans) = if joint.is_root {
                (local_rot, joint.rest_translation)
            } else {
                let parent_rot = world_rotations[joint.parent_index];
                let parent_trans = world_translations[joint.parent_index];
                (
                    parent_rot * local_rot,
                    parent_trans + parent_rot.rotate_vec3(joint.rest_translation),
                )
            };

            let inv_rot = Quat {
                x: -world_rot.x,
                y: -world_rot.y,
                z: -world_rot.z,
                w: world_rot.w,
            }
            .normalized();
            let inv_trans = inv_rot.rotate_vec3(Vec3::ZERO - world_trans);

            world_rotations.push(world_rot);
            world_translations.push(world_trans);
            inverse_bind_matrices.push(Mat4::from_trs(inv_trans, inv_rot, unit_scale));
        }

        Ok(Self {
            joints,
            inverse_bind_matrices,
        })
    }

    /// Evaluates skinning palette matrices `M_world(j) * M_inv_bind(j)` for a sampled pose
    /// via `verdant_tensor::propagate_hierarchy_matrices_exact` and `multiply_mat4_cols_exact`.
    pub fn evaluate_skinning_palette(
        &self,
        pose: &SampledPose,
    ) -> Result<Vec<Mat4>, AnimationError> {
        if pose.joint_translations.len() != self.joints.len()
            || pose.joint_rotations.len() != self.joints.len()
        {
            return Err(AnimationError::InvalidClipChannel {
                detail: "sampled pose joint count does not match skeleton".to_string(),
            });
        }

        let unit_scale = Vec3::new(1.0, 1.0, 1.0);
        let mut parent_indices = Vec::with_capacity(self.joints.len());
        let mut local_cols = Vec::with_capacity(self.joints.len());
        for (idx, joint) in self.joints.iter().enumerate() {
            parent_indices.push(if joint.is_root {
                None
            } else {
                Some(joint.parent_index)
            });
            local_cols.push(
                Mat4::from_trs(
                    pose.joint_translations[idx],
                    pose.joint_rotations[idx].normalized(),
                    unit_scale,
                )
                .cols,
            );
        }

        let world_cols =
            verdant_tensor::propagate_hierarchy_matrices_exact(&parent_indices, &local_cols)
                .map_err(|err| AnimationError::InvalidClipChannel {
                    detail: err.to_string(),
                })?;

        let mut palette: Vec<Mat4> = Vec::with_capacity(self.joints.len());
        for (idx, w_cols) in world_cols.iter().enumerate() {
            let p_cols = verdant_tensor::linear::multiply_mat4_cols_exact(
                w_cols,
                &self.inverse_bind_matrices[idx].cols,
            )
            .map_err(|err| AnimationError::InvalidClipChannel {
                detail: err.to_string(),
            })?;
            palette.push(Mat4 { cols: p_cols });
        }

        Ok(palette)
    }
}

/// Normalized four-joint vertex skinning influence tuple (`w0 + w1 + w2 + w3 == 1000` permille).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkinWeights4 {
    /// Up to four influencing joint indices (`DIM_HOMOGENEOUS`).
    pub joints: [u16; DIM_HOMOGENEOUS],
    /// Permille influence weights summing strictly to `prism_kernel::skinWeightScale()`.
    pub weights_permille: [u16; DIM_HOMOGENEOUS],
}

impl SkinWeights4 {
    /// Validates four-joint skinning weights against `prism_kernel::skinWeightsNormalized` and `joint_count`.
    pub fn new(
        joints: [u16; DIM_HOMOGENEOUS],
        weights_permille: [u16; DIM_HOMOGENEOUS],
        joint_count: usize,
    ) -> Result<Self, AnimationError> {
        let [w0, w1, w2, w3] = weights_permille;
        let expected_scale = crate::prism_kernel::skinWeightScale().expect("prism");
        let sum = u64::from(w0) + u64::from(w1) + u64::from(w2) + u64::from(w3);
        let normalized = crate::prism_kernel::skinWeightsNormalized(
            u64::from(w0),
            u64::from(w1),
            u64::from(w2),
            u64::from(w3),
        )
        .expect("prism");

        let indices_valid = joints
            .iter()
            .zip(weights_permille.iter())
            .all(|(&j, &w)| w == 0 || usize::from(j) < joint_count);

        if !normalized || !indices_valid || joint_count == 0 {
            return Err(AnimationError::UnnormalizedOrInvalidSkinWeights {
                observed_sum: sum,
                expected_scale,
            });
        }

        Ok(Self {
            joints,
            weights_permille,
        })
    }
}

/// Single timestamped keyframe in a [`SkeletalClip`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyframePose {
    /// Keyframe timestamp in milliseconds from clip start.
    pub time_millis: u64,
    /// Local joint translations in meters.
    pub joint_translations: Vec<Vec3>,
    /// Local unit joint rotations.
    pub joint_rotations: Vec<Quat>,
    /// Optional morph target weights in permille (`0..=1000`).
    pub morph_weights_permille: Vec<u16>,
}

/// Sampled joint and morph pose produced at an arbitrary playback timestamp.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SampledPose {
    /// Interpolated local joint translations.
    pub joint_translations: Vec<Vec3>,
    /// Interpolated unit local joint rotations.
    pub joint_rotations: Vec<Quat>,
    /// Interpolated morph target weights in permille (`0..=1000`).
    pub morph_weights_permille: Vec<u16>,
}

/// Keyframed skeletal and morph-target animation clip.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkeletalClip {
    /// Clip asset name.
    pub name: String,
    /// Total clip duration in milliseconds.
    pub duration_millis: u64,
    /// Monotonically ordered keyframes.
    pub keyframes: Vec<KeyframePose>,
}

fn prism_lerp_signed(a: f64, b: f64, alpha_permille: u64, scale: f64) -> f64 {
    let bias = pow10_u64(three_u32() + three_u32()) as f64;
    let from_u = ((a * scale) + bias).max(0.0).round() as u64;
    let to_u = ((b * scale) + bias).max(0.0).round() as u64;
    let lerped = crate::prism_kernel::lerpPermille(from_u, to_u, alpha_permille).expect("prism");
    ((lerped as f64) - bias) / scale
}

impl SkeletalClip {
    /// Samples the clip at `elapsed_millis` using `prism_kernel::{keyframeAlphaPermille, lerpPermille}`.
    pub fn sample(
        &self,
        skeleton: &Skeleton,
        elapsed_millis: u64,
    ) -> Result<SampledPose, AnimationError> {
        if self.keyframes.is_empty() {
            return Err(AnimationError::InvalidClipChannel {
                detail: "animation clip contains zero keyframes".to_string(),
            });
        }

        let morph_limit = crate::prism_kernel::morphTargetCountLimit().expect("prism") as usize;
        let expected_joints = skeleton.joints.len();
        let expected_morphs = self.keyframes[0].morph_weights_permille.len();
        if expected_morphs > morph_limit {
            return Err(AnimationError::InvalidClipChannel {
                detail: "morph target channel count exceeds limit".to_string(),
            });
        }

        for kf in &self.keyframes {
            if kf.joint_translations.len() != expected_joints
                || kf.joint_rotations.len() != expected_joints
                || kf.morph_weights_permille.len() != expected_morphs
            {
                return Err(AnimationError::InvalidClipChannel {
                    detail: "keyframe channel length mismatch".to_string(),
                });
            }
        }

        let clamped_time = elapsed_millis.min(self.duration_millis);
        let mut prev_idx = 0usize;
        let mut next_idx = 0usize;
        for (idx, kf) in self.keyframes.iter().enumerate() {
            if kf.time_millis <= clamped_time {
                prev_idx = idx;
                next_idx = idx;
            } else {
                next_idx = idx;
                break;
            }
        }

        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];
        let span = next.time_millis.saturating_sub(prev.time_millis);
        let local_elapsed = clamped_time.saturating_sub(prev.time_millis);
        let alpha_permille =
            crate::prism_kernel::keyframeAlphaPermille(local_elapsed, span).expect("prism");
        let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;

        let mut joint_translations = Vec::with_capacity(expected_joints);
        let mut joint_rotations = Vec::with_capacity(expected_joints);
        for j in 0..expected_joints {
            let ta = prev.joint_translations[j];
            let tb = next.joint_translations[j];
            joint_translations.push(Vec3::new(
                prism_lerp_signed(ta.x, tb.x, alpha_permille, scale),
                prism_lerp_signed(ta.y, tb.y, alpha_permille, scale),
                prism_lerp_signed(ta.z, tb.z, alpha_permille, scale),
            ));

            let qa = prev.joint_rotations[j].normalized();
            let mut qb = next.joint_rotations[j].normalized();
            let dot = qa.x * qb.x + qa.y * qb.y + qa.z * qb.z + qa.w * qb.w;
            if dot < 0.0 {
                qb = Quat {
                    x: -qb.x,
                    y: -qb.y,
                    z: -qb.z,
                    w: -qb.w,
                };
            }
            joint_rotations.push(
                Quat {
                    x: prism_lerp_signed(qa.x, qb.x, alpha_permille, scale),
                    y: prism_lerp_signed(qa.y, qb.y, alpha_permille, scale),
                    z: prism_lerp_signed(qa.z, qb.z, alpha_permille, scale),
                    w: prism_lerp_signed(qa.w, qb.w, alpha_permille, scale),
                }
                .normalized(),
            );
        }

        let mut morph_weights_permille = Vec::with_capacity(expected_morphs);
        for m in 0..expected_morphs {
            let w = crate::prism_kernel::lerpPermille(
                u64::from(prev.morph_weights_permille[m]),
                u64::from(next.morph_weights_permille[m]),
                alpha_permille,
            )
            .expect("prism") as u16;
            morph_weights_permille.push(w);
        }

        Ok(SampledPose {
            joint_translations,
            joint_rotations,
            morph_weights_permille,
        })
    }
}

/// Applies morph target deltas and four-joint linear-blend skinning (`LBS`) to `mesh` via
/// `verdant_tensor::skin_vertices_linear_blend_exact` (`uor-matmul`).
pub fn skin_mesh_linear_blend(
    mesh: &ProceduralMesh,
    skeleton: &Skeleton,
    pose: &SampledPose,
    vertex_weights: &[SkinWeights4],
) -> Result<ProceduralMesh, AnimationError> {
    if vertex_weights.len() != mesh.positions.len() || mesh.normals.len() != mesh.positions.len() {
        return Err(AnimationError::InvalidClipChannel {
            detail: "vertex skin weight count does not match mesh vertex count".to_string(),
        });
    }

    let palette = skeleton.evaluate_skinning_palette(pose)?;
    let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;
    let i2 = 1usize + 1usize;

    let mut morphed_positions = Vec::with_capacity(mesh.positions.len());
    let mut joint_indices = Vec::with_capacity(vertex_weights.len());
    let mut weights_permille = Vec::with_capacity(vertex_weights.len());

    for (v_idx, sw) in vertex_weights.iter().enumerate() {
        let [px, py, pz] = mesh.positions[v_idx];
        let mut base_pos = Vec3::new(f64::from(px), f64::from(py), f64::from(pz));
        for (m_idx, target) in mesh.morph_targets.iter().enumerate() {
            if let Some(&w_permille) = pose.morph_weights_permille.get(m_idx) {
                if w_permille > 0 {
                    if let Some(&[dx, dy, dz]) = target.position_deltas.get(v_idx) {
                        let factor = f64::from(w_permille) / scale;
                        base_pos = base_pos
                            + Vec3::new(f64::from(dx), f64::from(dy), f64::from(dz)).scale(factor);
                    }
                }
            }
        }
        morphed_positions.push([base_pos.x as f32, base_pos.y as f32, base_pos.z as f32]);

        for slot in 0..DIM_HOMOGENEOUS {
            if sw.weights_permille[slot] > 0 && usize::from(sw.joints[slot]) >= palette.len() {
                return Err(AnimationError::InvalidClipChannel {
                    detail: "skin weight joint index exceeds skeleton palette".to_string(),
                });
            }
        }
        joint_indices.push(sw.joints);
        weights_permille.push(sw.weights_permille);
    }

    let palette_cols: Vec<[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]> =
        palette.iter().map(|m| m.cols).collect();
    let (skinned_pos, skinned_nrm) = verdant_tensor::skin_vertices_linear_blend_exact(
        &morphed_positions,
        &mesh.normals,
        &palette_cols,
        &joint_indices,
        &weights_permille,
    )
    .map_err(|err| AnimationError::InvalidClipChannel {
        detail: err.to_string(),
    })?;

    let mut out = mesh.clone();
    for v_idx in 0..out.positions.len() {
        out.positions[v_idx] = [
            skinned_pos[v_idx][0],
            skinned_pos[v_idx][1],
            skinned_pos[v_idx][i2],
        ];
        out.normals[v_idx] = [
            skinned_nrm[v_idx][0],
            skinned_nrm[v_idx][1],
            skinned_nrm[v_idx][i2],
        ];
    }
    Ok(out)
}

/// Evaluates zero-decode `Packed<Grid<16>, 2>` nibble animation track offsets directly through
/// `verdant_tensor::contract_packed_animation_tracks` (`coded_gemm`) without intermediate float expansion.
pub fn evaluate_packed_nibble_pose_offsets(
    blend_weights_permille: &[i32],
    track_count: usize,
    channel_cols: usize,
    codebook_milli: &[i32; verdant_tensor::CODED_TABLE_LEN],
    packed_nibbles: &[u8],
) -> Result<Vec<f64>, AnimationError> {
    let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;
    let inv_scale_sq = 1.0 / (scale * scale);
    let accum = verdant_tensor::contract_packed_animation_tracks(
        blend_weights_permille,
        1,
        track_count,
        channel_cols,
        codebook_milli,
        packed_nibbles,
    )
    .map_err(|err| AnimationError::InvalidClipChannel {
        detail: err.to_string(),
    })?;
    Ok(accum
        .into_iter()
        .map(|v| (v as f64) * inv_scale_sq)
        .collect())
}
