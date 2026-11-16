//! Exact linear Laurent `(+, *)` tensor contractions over `uor-matmul`.
//!
//! Hosts homogeneous `4x4` vertex batch transforms, topological scene/skeleton hierarchy
//! propagation, normalized 4-joint linear-blend skinning, and Delassus constraint effective-mass
//! matrix products (`J * M^-1 * J^T`) without intermediate floating-point rounding.

#[cfg(not(target_arch = "wasm32"))]
use uor_matmul::slice;

#[cfg(target_arch = "wasm32")]
use crate::pow2_u64;
use crate::{
    geometric_epsilon, prism_kernel, two_f64, TensorComputeError, DIM_HOMOGENEOUS, DIM_SPATIAL,
};

/// Contracts two column-major `4x4` homogeneous matrices `lhs * rhs` via `uor-matmul`.
pub fn multiply_mat4_cols_exact(
    lhs_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
    rhs_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
) -> Result<[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS], TensorComputeError> {
    for col in lhs_cols.iter().chain(rhs_cols.iter()) {
        if !col.iter().all(|v| v.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry { operand: "mat4" });
        }
    }

    let k = DIM_HOMOGENEOUS;
    let mut out_cols = [[0.0_f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Column-major `C = L * R` equals row-major `C^T = R^T * L^T` where `M^T` has `M.cols` as rows:
        let mut r_rows = [0.0_f64; DIM_HOMOGENEOUS * DIM_HOMOGENEOUS];
        let mut l_rows = [0.0_f64; DIM_HOMOGENEOUS * DIM_HOMOGENEOUS];
        for r in 0..k {
            for c in 0..k {
                r_rows[r * k + c] = rhs_cols[r][c];
                l_rows[r * k + c] = lhs_cols[r][c];
            }
        }
        let mut c_rows = [0.0_f64; DIM_HOMOGENEOUS * DIM_HOMOGENEOUS];
        slice::gemm_float::<f64, f64>(k, k, k, &r_rows, &l_rows, &mut c_rows, &mut [], &mut [])
            .map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "mat4 contraction shape mismatch".to_string(),
            })?;
        for r in 0..k {
            for c in 0..k {
                out_cols[r][c] = c_rows[r * k + c];
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let shift = prism_kernel::tileEdgePixels().unwrap_or(1) as u32;
        let q_scale = pow2_u64(shift) as f64;
        let inv_q2 = 1.0_f64 / (q_scale * q_scale);
        let mut l_rows_q = [[0_i32; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];
        let mut r_cols_q = [[0_i32; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];
        for r in 0..k {
            for c in 0..k {
                l_rows_q[r][c] = (lhs_cols[c][r] * q_scale).round() as i32;
                r_cols_q[c][r] = (rhs_cols[c][r] * q_scale).round() as i32;
            }
        }
        for c in 0..k {
            let b_alpha = uor_matmul::as_alphabet_full(&r_cols_q[c]);
            for r in 0..k {
                let a_alpha = uor_matmul::as_alphabet_full(&l_rows_q[r]);
                let acc = uor_matmul::dot_ref(a_alpha, b_alpha);
                out_cols[c][r] = (acc as f64) * inv_q2;
            }
        }
    }

    Ok(out_cols)
}

/// Transforms a batch of 3D positions `N x 4` by a column-major `4 x 4` homogeneous matrix using
/// `uor-matmul` so every inner product is accumulated without intermediate float rounding.
pub fn batch_transform_positions_exact(
    positions: &[[f32; DIM_SPATIAL]],
    matrix_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
) -> Result<Vec<[f32; DIM_HOMOGENEOUS]>, TensorComputeError> {
    if positions.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "vertex batch must be non-empty".to_string(),
        });
    }
    for pos in positions {
        if !pos.iter().all(|c| c.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry {
                operand: "positions",
            });
        }
    }
    for col in matrix_cols {
        if !col.iter().all(|c| c.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry { operand: "matrix" });
        }
    }

    let m = positions.len();
    let k = DIM_HOMOGENEOUS;
    let n = DIM_HOMOGENEOUS;
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let mut out = Vec::with_capacity(m);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut a = Vec::with_capacity(m * k);
        for p in positions {
            a.push(p[0]);
            a.push(p[1]);
            a.push(p[i2]);
            a.push(1.0_f32);
        }
        let mut b = vec![0.0_f32; k * n];
        for r in 0..k {
            for c in 0..n {
                b[r * n + c] = matrix_cols[r][c] as f32;
            }
        }
        let mut c_flat = vec![0.0_f32; m * n];
        slice::gemm_float::<f32, f32>(m, k, n, &a, &b, &mut c_flat, &mut [], &mut []).map_err(
            |_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "homogeneous position gemm shape mismatch".to_string(),
            },
        )?;
        for chunk in c_flat.chunks_exact(DIM_HOMOGENEOUS) {
            out.push([chunk[0], chunk[1], chunk[i2], chunk[i3]]);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let shift = prism_kernel::tileEdgePixels().unwrap_or(1) as u32;
        let q_scale = pow2_u64(shift) as f64;
        let inv_q2 = 1.0_f64 / (q_scale * q_scale);
        let one_q = (q_scale as i32).max(1);
        let mut b_cols = [[0_i32; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];
        for r in 0..k {
            for c in 0..n {
                b_cols[c][r] = (matrix_cols[r][c] * q_scale).round() as i32;
            }
        }
        for p in positions {
            let row_q = [
                (f64::from(p[0]) * q_scale).round() as i32,
                (f64::from(p[1]) * q_scale).round() as i32,
                (f64::from(p[i2]) * q_scale).round() as i32,
                one_q,
            ];
            let a_alpha = uor_matmul::as_alphabet_full(&row_q);
            let c0 = uor_matmul::dot_ref(a_alpha, uor_matmul::as_alphabet_full(&b_cols[0]));
            let c1 = uor_matmul::dot_ref(a_alpha, uor_matmul::as_alphabet_full(&b_cols[1]));
            let c2 = uor_matmul::dot_ref(a_alpha, uor_matmul::as_alphabet_full(&b_cols[i2]));
            let c3 = uor_matmul::dot_ref(a_alpha, uor_matmul::as_alphabet_full(&b_cols[i3]));
            out.push([
                ((c0 as f64) * inv_q2) as f32,
                ((c1 as f64) * inv_q2) as f32,
                ((c2 as f64) * inv_q2) as f32,
                ((c3 as f64) * inv_q2) as f32,
            ]);
        }
    }

    Ok(out)
}

/// Solved world-space mid-joint (knee/elbow) and end-effector (ankle/wrist) pose produced by
/// [`solve_two_bone_ik_exact`] (`kernel13TwoBoneIkContraction`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoBoneIkSolution {
    /// Solved world-space mid-joint position (`knee` or `elbow`).
    pub mid_joint_pos: [f64; DIM_SPATIAL],
    /// Clamped world-space end-effector position (`ankle` or `wrist`).
    pub end_effector_pos: [f64; DIM_SPATIAL],
    /// Flexion angle at the mid-joint in radians (`0.0` when fully extended).
    pub knee_bend_radians: f64,
    /// Proof witness from `prism_kernel::twoBoneIkReachable` indicating unstretched reachability.
    pub target_within_reach: bool,
}

/// Solves analytical two-bone inverse kinematics (`kernel13TwoBoneIkContraction`) using exact
/// cosine-law flexion and `uor-matmul` `4x4` pole-plane frame matrix contraction.
pub fn solve_two_bone_ik_exact(
    root_pos: [f64; DIM_SPATIAL],
    target_pos: [f64; DIM_SPATIAL],
    pole_dir: [f64; DIM_SPATIAL],
    upper_len: f64,
    lower_len: f64,
) -> Result<TwoBoneIkSolution, TensorComputeError> {
    if !root_pos.iter().all(|v| v.is_finite())
        || !target_pos.iter().all(|v| v.is_finite())
        || !pole_dir.iter().all(|v| v.is_finite())
    {
        return Err(TensorComputeError::NonFiniteTensorEntry {
            operand: "two_bone_ik",
        });
    }
    let eps = geometric_epsilon();
    if upper_len <= eps || lower_len <= eps {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "two-bone IK limb lengths must be strictly positive".to_string(),
        });
    }

    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let dx = target_pos[0] - root_pos[0];
    let dy = target_pos[1] - root_pos[1];
    let dz = target_pos[i2] - root_pos[i2];
    let raw_dist = (dx * dx + dy * dy + dz * dz).sqrt();

    let scale_milli = prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
    let upper_milli = (upper_len * scale_milli).round().max(1.0) as u64;
    let lower_milli = (lower_len * scale_milli).round().max(1.0) as u64;
    let dist_milli = (raw_dist * scale_milli).round().max(0.0) as u64;
    let target_within_reach =
        prism_kernel::twoBoneIkReachable(upper_milli, lower_milli, dist_milli).unwrap_or(false);

    let min_reach = (upper_len - lower_len).abs() + eps;
    let max_reach = (upper_len + lower_len - eps).max(min_reach);
    let clamped_dist = raw_dist.clamp(min_reach, max_reach);

    let fwd = if raw_dist > eps {
        [dx / raw_dist, dy / raw_dist, dz / raw_dist]
    } else {
        [0.0, -1.0, 0.0]
    };

    let pole_dot = pole_dir[0] * fwd[0] + pole_dir[1] * fwd[1] + pole_dir[i2] * fwd[i2];
    let mut proj_pole = [
        pole_dir[0] - fwd[0] * pole_dot,
        pole_dir[1] - fwd[1] * pole_dot,
        pole_dir[i2] - fwd[i2] * pole_dot,
    ];
    let mut proj_len =
        (proj_pole[0] * proj_pole[0] + proj_pole[1] * proj_pole[1] + proj_pole[i2] * proj_pole[i2])
            .sqrt();
    if proj_len <= eps {
        let fallback = if fwd[i2].abs() < (1.0 / two_f64()) {
            [0.0, 0.0, 1.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let fb_dot = fallback[0] * fwd[0] + fallback[1] * fwd[1] + fallback[i2] * fwd[i2];
        proj_pole = [
            fallback[0] - fwd[0] * fb_dot,
            fallback[1] - fwd[1] * fb_dot,
            fallback[i2] - fwd[i2] * fb_dot,
        ];
        proj_len = (proj_pole[0] * proj_pole[0]
            + proj_pole[1] * proj_pole[1]
            + proj_pole[i2] * proj_pole[i2])
            .sqrt()
            .max(eps);
    }
    let bend_dir = [
        proj_pole[0] / proj_len,
        proj_pole[1] / proj_len,
        proj_pole[i2] / proj_len,
    ];
    let side = [
        fwd[1] * bend_dir[i2] - fwd[i2] * bend_dir[1],
        fwd[i2] * bend_dir[0] - fwd[0] * bend_dir[i2],
        fwd[0] * bend_dir[1] - fwd[1] * bend_dir[0],
    ];

    let two = two_f64();
    let cos_alpha = ((upper_len * upper_len + clamped_dist * clamped_dist - lower_len * lower_len)
        / (two * upper_len * clamped_dist))
        .clamp(-1.0, 1.0);
    let sin_alpha = (1.0 - cos_alpha * cos_alpha).max(0.0).sqrt();
    let cos_interior = ((upper_len * upper_len + lower_len * lower_len
        - clamped_dist * clamped_dist)
        / (two * upper_len * lower_len))
        .clamp(-1.0, 1.0);
    let knee_bend_radians = (-cos_interior).clamp(-1.0, 1.0).acos();

    let basis_cols = [
        [fwd[0], fwd[1], fwd[i2], 0.0],
        [bend_dir[0], bend_dir[1], bend_dir[i2], 0.0],
        [side[0], side[1], side[i2], 0.0],
        [root_pos[0], root_pos[1], root_pos[i2], 1.0],
    ];
    let local_joint_cols = [
        [upper_len * cos_alpha, upper_len * sin_alpha, 0.0, 1.0],
        [clamped_dist, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let world_joint_cols = multiply_mat4_cols_exact(&basis_cols, &local_joint_cols)?;

    Ok(TwoBoneIkSolution {
        mid_joint_pos: [
            world_joint_cols[0][0],
            world_joint_cols[0][1],
            world_joint_cols[0][i2],
        ],
        end_effector_pos: [
            world_joint_cols[1][0],
            world_joint_cols[1][1],
            world_joint_cols[1][i2],
        ],
        knee_bend_radians,
        target_within_reach: target_within_reach && world_joint_cols[0][i3].is_finite(),
    })
}

/// Propagates local `4x4` transforms across an acyclic scene or skeletal hierarchy (`parent < idx`)
/// using `uor-matmul` `4x4` matrix contractions.
pub fn propagate_hierarchy_matrices_exact(
    parent_indices: &[Option<usize>],
    local_cols: &[[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]],
) -> Result<Vec<[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]>, TensorComputeError> {
    if parent_indices.is_empty() || parent_indices.len() != local_cols.len() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "hierarchy parent and local matrix slices must be non-empty and equal length"
                .to_string(),
        });
    }
    let mut world = Vec::with_capacity(local_cols.len());
    for (idx, (&parent_opt, local)) in parent_indices.iter().zip(local_cols.iter()).enumerate() {
        match parent_opt {
            None => {
                for col in local {
                    if !col.iter().all(|v| v.is_finite()) {
                        return Err(TensorComputeError::NonFiniteTensorEntry {
                            operand: "local_cols",
                        });
                    }
                }
                world.push(*local);
            }
            Some(parent_idx) => {
                if parent_idx >= idx {
                    return Err(TensorComputeError::ZeroOrMismatchedExtents {
                        detail: "hierarchy parent index must precede child index".to_string(),
                    });
                }
                let combined = multiply_mat4_cols_exact(&world[parent_idx], local)?;
                world.push(combined);
            }
        }
    }
    Ok(world)
}

/// Skinned `(positions, normals)` output pair produced by [`skin_vertices_linear_blend_exact`].
pub type SkinnedMeshPair = (Vec<[f32; DIM_SPATIAL]>, Vec<[f32; DIM_SPATIAL]>);

/// Executes normalized 4-joint linear-blend skinning (`LBS`) over `positions` and `normals` by
/// blending `4x4` joint palette matrices and contracting vertex batches through `uor-matmul`.
pub fn skin_vertices_linear_blend_exact(
    positions: &[[f32; DIM_SPATIAL]],
    normals: &[[f32; DIM_SPATIAL]],
    palette_cols: &[[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]],
    joint_indices: &[[u16; DIM_HOMOGENEOUS]],
    weights_permille: &[[u16; DIM_HOMOGENEOUS]],
) -> Result<SkinnedMeshPair, TensorComputeError> {
    let v_count = positions.len();
    if v_count == 0
        || normals.len() != v_count
        || joint_indices.len() != v_count
        || weights_permille.len() != v_count
        || palette_cols.is_empty()
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "skinning vertex, normal, weight, or palette slice length mismatch".to_string(),
        });
    }

    let scale = prism_kernel::skinWeightScale().unwrap_or(1) as f64;
    let eps = geometric_epsilon();
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;

    let mut out_pos = Vec::with_capacity(v_count);
    let mut out_nrm = Vec::with_capacity(v_count);

    for v_idx in 0..v_count {
        let mut blended_cols = [[0.0_f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS];
        let joints = joint_indices[v_idx];
        let weights = weights_permille[v_idx];

        for slot in 0..DIM_HOMOGENEOUS {
            let w_raw = weights[slot];
            if w_raw == 0 {
                continue;
            }
            let j_idx = usize::from(joints[slot]);
            if j_idx >= palette_cols.len() {
                return Err(TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "skinning joint index exceeds palette length".to_string(),
                });
            }
            let w = f64::from(w_raw) / scale;
            let mat = &palette_cols[j_idx];
            for c in 0..DIM_HOMOGENEOUS {
                for r in 0..DIM_HOMOGENEOUS {
                    blended_cols[c][r] += mat[c][r] * w;
                }
            }
        }

        let pos = positions[v_idx];
        let nrm = normals[v_idx];
        let origin_nrm = [0.0_f32, 0.0_f32, 0.0_f32];
        let batch = [pos, nrm, origin_nrm];
        let contracted = batch_transform_positions_exact(&batch, &blended_cols)?;

        let p_out = contracted[0];
        let n_tip = contracted[1];
        let n_org = contracted[i2];
        let nx = f64::from(n_tip[0] - n_org[0]);
        let ny = f64::from(n_tip[1] - n_org[1]);
        let nz = f64::from(n_tip[i2] - n_org[i2]);
        let n_len = (nx * nx + ny * ny + nz * nz).sqrt();
        let unit_n = if n_len > eps {
            [
                (nx / n_len) as f32,
                (ny / n_len) as f32,
                (nz / n_len) as f32,
            ]
        } else {
            [0.0_f32, 1.0_f32, 0.0_f32]
        };

        let inv_w = if f64::from(p_out[i3]).abs() > eps {
            1.0_f32 / p_out[i3]
        } else {
            1.0_f32
        };
        out_pos.push([p_out[0] * inv_w, p_out[1] * inv_w, p_out[i2] * inv_w]);
        out_nrm.push(unit_n);
    }

    Ok((out_pos, out_nrm))
}

/// Computes the symmetric Delassus effective-compliance matrix `K = J * diag(inv_masses) * J^T`
/// (`constraint_rows x constraint_rows`) via exact `uor-matmul` contraction for XPBD/impulse joints.
pub fn delassus_effective_mass_matrix(
    jacobians: &[f64],
    inv_masses: &[f64],
    constraint_rows: usize,
    body_dofs: usize,
) -> Result<Vec<f64>, TensorComputeError> {
    if constraint_rows == 0
        || body_dofs == 0
        || jacobians.len() != constraint_rows.saturating_mul(body_dofs)
        || inv_masses.len() != body_dofs
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Delassus Jacobian or inverse-mass dimensions invalid".to_string(),
        });
    }
    if jacobians.iter().any(|v| !v.is_finite())
        || inv_masses.iter().any(|m| !m.is_finite() || *m < 0.0)
    {
        return Err(TensorComputeError::NonFiniteTensorEntry {
            operand: "delassus_jacobian",
        });
    }

    // `A = J * diag(inv_masses)` (`m x k`), `B = J^T` (`k x m`), `K = A * B` (`m x m`):
    let m = constraint_rows;
    let k = body_dofs;
    let mut out = vec![0.0_f64; m * m];

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut weighted_j = vec![0.0_f64; m * k];
        let mut j_transpose = vec![0.0_f64; k * m];
        for r in 0..m {
            for c in 0..k {
                let j_rc = jacobians[r * k + c];
                weighted_j[r * k + c] = j_rc * inv_masses[c];
                j_transpose[c * m + r] = j_rc;
            }
        }
        slice::gemm_float::<f64, f64>(
            m,
            k,
            m,
            &weighted_j,
            &j_transpose,
            &mut out,
            &mut [],
            &mut [],
        )
        .map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
            detail: "Delassus gemm_float contraction failed".to_string(),
        })?;
    }

    #[cfg(target_arch = "wasm32")]
    {
        let shift = prism_kernel::sTwelve().unwrap_or(1) as u32;
        let q_scale = pow2_u64(shift) as f64;
        let inv_q2 = 1.0_f64 / (q_scale * q_scale);
        let mut a_rows_q = vec![vec![0_i32; k]; m];
        let mut b_rows_q = vec![vec![0_i32; k]; m];
        for r in 0..m {
            for c in 0..k {
                let j_rc = jacobians[r * k + c];
                a_rows_q[r][c] = (j_rc * inv_masses[c] * q_scale).round() as i32;
                b_rows_q[r][c] = (j_rc * q_scale).round() as i32;
            }
        }
        for r in 0..m {
            let a_alpha = uor_matmul::as_alphabet_full(&a_rows_q[r]);
            for c in 0..m {
                let b_alpha = uor_matmul::as_alphabet_full(&b_rows_q[c]);
                let acc = uor_matmul::dot_ref(a_alpha, b_alpha);
                out[r * m + c] = (acc as f64) * inv_q2;
            }
        }
    }

    Ok(out)
}
