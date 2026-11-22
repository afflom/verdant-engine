//! Exact linear Laurent `(+, *)` tensor contractions over `uor-matmul`.
//!
//! Hosts homogeneous `4x4` vertex batch transforms, topological scene/skeleton hierarchy
//! propagation, normalized 4-joint linear-blend skinning, and Delassus constraint effective-mass
//! matrix products (`J * M^-1 * J^T`) without intermediate floating-point rounding.

#[cfg(not(target_arch = "wasm32"))]
use uor_matmul::slice;

use crate::{
    geometric_epsilon, kernel_consts, prism_kernel, two_f64, TensorComputeError, DIM_HOMOGENEOUS,
    DIM_SPATIAL,
};

mod activations;
pub use activations::{exponential_f32, sin_cos_f32, swiglu_f32};

#[inline]
fn transform_triple_exact(
    triple: &[[f32; DIM_SPATIAL]; DIM_SPATIAL],
    matrix_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
) -> [[f32; DIM_HOMOGENEOUS]; DIM_SPATIAL] {
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let mut out = [[0.0_f32; DIM_HOMOGENEOUS]; DIM_SPATIAL];
    for (row_idx, p) in triple.iter().enumerate() {
        let px = f64::from(p[0]);
        let py = f64::from(p[1]);
        let pz = f64::from(p[i2]);
        for c in 0..DIM_HOMOGENEOUS {
            let val = px * matrix_cols[0][c]
                + py * matrix_cols[1][c]
                + pz * matrix_cols[i2][c]
                + matrix_cols[i3][c];
            out[row_idx][c] = val as f32;
        }
    }
    out
}

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
        let kc = kernel_consts();
        let q_scale = kc.q16_scale_f64;
        let inv_q2 = kc.inv_q16_sq_f64;
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

/// Zero-allocation variant of [`batch_transform_positions_exact`] writing `N x 4` homogeneous
/// outputs directly into `out_slice` (`out_slice.len() == positions.len()`).
pub fn batch_transform_positions_into(
    positions: &[[f32; DIM_SPATIAL]],
    matrix_cols: &[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS],
    out_slice: &mut [[f32; DIM_HOMOGENEOUS]],
) -> Result<(), TensorComputeError> {
    if positions.is_empty() || out_slice.len() != positions.len() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "vertex batch must be non-empty and match output slice".to_string(),
        });
    }
    for col in matrix_cols {
        if !col.iter().all(|c| c.is_finite()) {
            return Err(TensorComputeError::NonFiniteTensorEntry { operand: "matrix" });
        }
    }
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;
    let c0 = [
        matrix_cols[0][0] as f32,
        matrix_cols[0][1] as f32,
        matrix_cols[0][i2] as f32,
        matrix_cols[0][i3] as f32,
    ];
    let c1 = [
        matrix_cols[1][0] as f32,
        matrix_cols[1][1] as f32,
        matrix_cols[1][i2] as f32,
        matrix_cols[1][i3] as f32,
    ];
    let c2 = [
        matrix_cols[i2][0] as f32,
        matrix_cols[i2][1] as f32,
        matrix_cols[i2][i2] as f32,
        matrix_cols[i2][i3] as f32,
    ];
    let c3 = [
        matrix_cols[i3][0] as f32,
        matrix_cols[i3][1] as f32,
        matrix_cols[i3][i2] as f32,
        matrix_cols[i3][i3] as f32,
    ];

    for (p, dst) in positions.iter().zip(out_slice.iter_mut()) {
        if !p[0].is_finite() || !p[1].is_finite() || !p[i2].is_finite() {
            return Err(TensorComputeError::NonFiniteTensorEntry {
                operand: "positions",
            });
        }
        let x = p[0];
        let y = p[1];
        let z = p[i2];
        *dst = [
            x * c0[0] + y * c1[0] + z * c2[0] + c3[0],
            x * c0[1] + y * c1[1] + z * c2[1] + c3[1],
            x * c0[i2] + y * c1[i2] + z * c2[i2] + c3[i2],
            x * c0[i3] + y * c1[i3] + z * c2[i3] + c3[i3],
        ];
    }
    Ok(())
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
        let kc = kernel_consts();
        let q_scale = kc.q16_scale_f64;
        let inv_q2 = kc.inv_q16_sq_f64;
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

    let scale_milli = kernel_consts().skin_weight_scale_f64;
    let upper_milli = (upper_len * scale_milli).round().max(1.0) as u64;
    let lower_milli = (lower_len * scale_milli).round().max(1.0) as u64;
    let dist_milli = (raw_dist * scale_milli).round().max(0.0) as u64;
    let target_within_reach = dist_milli <= upper_milli.saturating_add(lower_milli);
    debug_assert_eq!(
        prism_kernel::twoBoneIkReachable(upper_milli, lower_milli, dist_milli),
        Ok(target_within_reach)
    );

    let min_reach = (upper_len - lower_len).abs() + eps;
    let max_reach = (upper_len + lower_len - eps).max(min_reach);
    let clamped_dist = raw_dist.max(min_reach).min(max_reach);

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

/// Zero-allocation linear-blend skinning writing deformed `(positions, normals)` into pre-sized
/// output buffers (`0` heap allocations per vertex or per frame).
pub fn skin_vertices_linear_blend_into(
    positions: &[[f32; DIM_SPATIAL]],
    normals: &[[f32; DIM_SPATIAL]],
    palette_cols: &[[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]],
    joint_indices: &[[u16; DIM_HOMOGENEOUS]],
    weights_permille: &[[u16; DIM_HOMOGENEOUS]],
    out_pos: &mut [[f32; DIM_SPATIAL]],
    out_nrm: &mut [[f32; DIM_SPATIAL]],
) -> Result<(), TensorComputeError> {
    let v_count = positions.len();
    if v_count == 0
        || normals.len() != v_count
        || joint_indices.len() != v_count
        || weights_permille.len() != v_count
        || out_pos.len() != v_count
        || out_nrm.len() != v_count
        || palette_cols.is_empty()
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "skinning vertex, normal, weight, or palette slice length mismatch".to_string(),
        });
    }

    let inv_scale = 1.0 / kernel_consts().skin_weight_scale_f64;
    let eps = geometric_epsilon();
    let i2 = 1usize + 1usize;
    let i3 = i2 + 1usize;

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
            let w = f64::from(w_raw) * inv_scale;
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
        let contracted = transform_triple_exact(&[pos, nrm, origin_nrm], &blended_cols);

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
        out_pos[v_idx] = [p_out[0] * inv_w, p_out[1] * inv_w, p_out[i2] * inv_w];
        out_nrm[v_idx] = unit_n;
    }

    Ok(())
}

/// Executes normalized 4-joint linear-blend skinning (`LBS`) over `positions` and `normals` by
/// blending `4x4` joint palette matrices and contracting vertices with zero per-vertex allocations.
pub fn skin_vertices_linear_blend_exact(
    positions: &[[f32; DIM_SPATIAL]],
    normals: &[[f32; DIM_SPATIAL]],
    palette_cols: &[[[f64; DIM_HOMOGENEOUS]; DIM_HOMOGENEOUS]],
    joint_indices: &[[u16; DIM_HOMOGENEOUS]],
    weights_permille: &[[u16; DIM_HOMOGENEOUS]],
) -> Result<SkinnedMeshPair, TensorComputeError> {
    let v_count = positions.len();
    let mut out_pos = vec![[0.0_f32; DIM_SPATIAL]; v_count];
    let mut out_nrm = vec![[0.0_f32; DIM_SPATIAL]; v_count];
    skin_vertices_linear_blend_into(
        positions,
        normals,
        palette_cols,
        joint_indices,
        weights_permille,
        &mut out_pos,
        &mut out_nrm,
    )?;
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
        let kc = kernel_consts();
        let q_scale = kc.q12_scale_f64;
        let inv_q2 = kc.inv_q12_sq_f64;
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

/// Contract arbitrary row-major F32 operands into caller-owned output storage.
///
/// Prism admits the exact operand extents before any output write. All scalar
/// computation, including non-finite IEEE-754 inputs, is delegated to the same
/// uor-matmul substrate used by Verdant's specialized linear contractions.
/// Unlike the geometry APIs, this entry point imposes no fixed inner dimension.
///
/// # Errors
/// Returns `TensorComputeError::ZeroOrMismatchedExtents` for an overflowing
/// extent product, mismatched slice length, or a rejected upstream product.
pub fn multiply_matrix_rows_f32(
    rows: usize,
    inner: usize,
    cols: usize,
    left: &[f32],
    right: &[f32],
    output: &mut [f32],
) -> Result<(), TensorComputeError> {
    if !matches!(
        verdant_matrix_shape::run(
            rows as u64,
            inner as u64,
            cols as u64,
            left.len() as u64,
            right.len() as u64,
            output.len() as u64,
        ),
        Ok(true)
    ) {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "matrix product extents do not match exact storage lengths".to_string(),
        });
    }
    uor_matmul::slice::gemm_float::<f32, f32>(
        rows,
        inner,
        cols,
        left,
        right,
        output,
        &mut [],
        &mut [],
    )
    .map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
        detail: "matrix product rejected by uor-matmul".to_string(),
    })
}

/// Classify F32 comparison (0), compare integer U32 words (1), add U64 words (2),
/// or preserve an encoded word (3). Comparison returns less/equal/greater (0/1/2),
/// right NaN (3), or left NaN with a non-NaN right operand (4).
/// Unknown operations and invalid U32 widths return None; addition overflow is typed.
pub use verdant_encoded_word::run as evaluate_encoded_word;

/// Certified GGML packed-scale and F8 word conversion with bounded input admission.
pub use verdant_small_float::run as evaluate_small_float;

/// Extract `(word * factor / divisor) % modulus`; zero denominators return None.
/// An intermediate product exceeding U64 returns the certified SDK overflow error.
pub use verdant_packed_field::run as extract_packed_field;

/// Encode `word - zero_point` as F32 bits for word < 256 and zero_point <= 256.
/// Oversized inputs return None; integer zero always encodes positive zero.
pub use verdant_integer_f32::run as encode_small_integer;

/// Native binary16 scale bits followed by signed quantized lane codes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymmetricQ8Block<const LANES: usize> {
    /// Bits of the stored IEEE binary16 scale.
    pub scale: u16,
    /// Signed integer coefficients in input order.
    pub codes: [i8; LANES],
}

/// Encode 32 F32 lanes as a native binary16 scale and 32 signed codes.
/// The symmetric reference format scales the lanes with the original F32
/// step before narrowing the stored scale. Its ties round away from zero.
///
/// # Errors
/// Returns `InvalidKernelSpecification` for a wrong lane count, nonfinite input or an undefined
/// float-to-byte conversion. The encoded block is returned atomically.
pub fn encode_symmetric_q8_block<const LANES: usize>(
    input: &[f32; LANES],
) -> Result<SymmetricQ8Block<LANES>, TensorComputeError> {
    use verdant_symmetric_q8::{run, Adt0 as Request, Adt1 as Step};
    let invalid = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "SymmetricBlock.plan".to_string(),
    };
    let plan = |request| run(request).map_err(|_| invalid());
    let Step::C1(mut maximum) = plan(Request::C5(LANES as u64))? else {
        return Err(invalid());
    };
    for &lane in input {
        let Step::C1(next) = plan(Request::C0(maximum, u64::from(lane.to_bits())))? else {
            return Err(invalid());
        };
        maximum = next;
    }
    let evaluate = |step| match step {
        Step::C3(operation, left, right) => verdant_scalar_f32::run(operation, left, right)
            .map_err(|_| invalid())?
            .ok_or_else(invalid),
        _ => Err(invalid()),
    };
    let scale = evaluate(plan(Request::C1(maximum))?)?;
    let inverse = match plan(Request::C2(scale))? {
        Step::C2 => {
            return Ok(SymmetricQ8Block {
                scale: 0,
                codes: [0; LANES],
            })
        }
        instruction => evaluate(instruction)?,
    };
    let half = convert_float_storage(0, 1, scale)
        .map_err(|_| invalid())?
        .ok_or_else(invalid)?;
    let mut output = SymmetricQ8Block {
        scale: half as u16,
        codes: [0; LANES],
    };
    for (code, &lane) in output.codes.iter_mut().zip(input) {
        let scaled = evaluate(plan(Request::C3(u64::from(lane.to_bits()), inverse))?)?;
        let Step::C1(byte) = plan(Request::C4(scaled))? else {
            return Err(invalid());
        };
        *code = byte as i8;
    }
    Ok(output)
}

/// Contract encoded symmetric weight blocks with a prepared activation row.
/// Each block's signed integer dot is exact. Prism widens the two scales,
/// rounds their product, weights the dot and adds it to the running F32 sum.
///
/// # Errors
/// A partial weight block, incorrect lane count or mismatched row lengths
/// returns `ZeroOrMismatchedExtents` before either row is read.
pub fn dot_symmetric_q8_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, false)
}

/// Contract symmetric four-bit weights with prepared signed-byte activation blocks.
/// Each block rounds the signed dot times the weight scale before multiplying
/// by the activation scale. Low and high nibbles occupy separate lane halves.
///
/// # Errors
/// Rejects partial blocks, mismatched extents and lane counts other than 32.
pub fn dot_symmetric_q4_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
) -> Result<f32, TensorComputeError> {
    dot_symmetric_row(weights, activations, true)
}

fn dot_symmetric_row<const LANES: usize>(
    weights: &[u8],
    activations: &[SymmetricQ8Block<LANES>],
    packed: bool,
) -> Result<f32, TensorComputeError> {
    use verdant_packed_dot::{Adt0 as Packed, Adt1 as Step};
    let invalid = || TensorComputeError::ZeroOrMismatchedExtents {
        detail: "symmetric block rows do not have matching extents".into(),
    };
    let packed_step = |request| verdant_packed_dot::run(request).map_err(|_| invalid());
    let packed_word = |request| match packed_step(request)? {
        Step::C1(word) => Ok(word),
        _ => Err(invalid()),
    };
    let lanes = LANES as u64;
    let bytes = weights.len() as u64;
    let blocks = activations.len() as u64;
    let width = if packed {
        packed_word(Packed::C0(lanes, bytes, blocks))?
    } else {
        verdant_symmetric_dot_layout::run(lanes, bytes, blocks)
    } as usize;
    if width == 0 {
        return Err(invalid());
    }
    let mut sum = 0;
    for (weight, activation) in weights.chunks_exact(width).zip(activations) {
        let (scale, codes) = weight
            .split_first_chunk::<{ std::mem::size_of::<u16>() }>()
            .ok_or_else(invalid)?;
        let signed = if packed {
            let mut signed = [0i8; LANES];
            let (low, high) = signed.split_at_mut(codes.len());
            for ((l, h), &byte) in low.iter_mut().zip(high).zip(codes) {
                let pair = packed_word(Packed::C1(u64::from(byte)))? as u16;
                let [first, second] = pair.to_le_bytes();
                *l = first as i8;
                *h = second as i8;
            }
            signed
        } else {
            let codes: &[u8; LANES] = codes.try_into().map_err(|_| invalid())?;
            codes.map(|byte| byte as i8)
        };
        let dot = uor_matmul::dot_ref(
            uor_matmul::as_alphabet_full(&signed),
            uor_matmul::as_alphabet_full(&activation.codes),
        );
        let negative = dot.is_negative();
        let magnitude = dot.unsigned_abs() as u64;
        let left = u64::from(u16::from_ne_bytes(*scale));
        let right = u64::from(activation.scale);
        sum = if packed {
            let widen = |half| {
                convert_float_storage(1, 0, half)
                    .ok()
                    .flatten()
                    .ok_or_else(invalid)
            };
            let (left, right) = (widen(left)?, widen(right)?);
            let mut current = verdant_count_f32::run(magnitude)
                .ok()
                .flatten()
                .ok_or_else(invalid)?;
            let mut phase = 0;
            loop {
                match packed_step(Packed::C2(phase, negative, sum, current, left, right))? {
                    Step::C2(op, a, b) => {
                        current = verdant_scalar_f32::run(op, a, b)
                            .ok()
                            .flatten()
                            .ok_or_else(invalid)?
                    }
                    Step::C1(value) => break value,
                    Step::C0 => return Err(invalid()),
                }
                phase += 1;
            }
        } else {
            verdant_symmetric_dot::run(sum, negative, magnitude, left, right)
                .ok()
                .flatten()
                .ok_or_else(|| TensorComputeError::InvalidKernelSpecification {
                    kernel_name: "SymmetricDot.accumulate".into(),
                })?
        };
    }
    Ok(f32::from_bits(sum as u32))
}

/// Canonical GGML CPU quantized tables reproduced from the pinned external header.
pub const GGML_QUANTIZED_TABLES: &[u8] = include_bytes!("ggml_tables.bin");
/// Exact table extents, byte spans, source pin and content digests.
pub const GGML_QUANTIZED_TABLE_CATALOG: &str = include_str!("ggml_tables.json");

/// Decode one block of the thirty GGML GET_ROWS CPU source formats into strided U32 words.
/// Prism admits the full block before writes and orders every load and arithmetic step.
/// Unsupported kinds return `InvalidKernelSpecification`; malformed extents and
/// address overflow return `ZeroOrMismatchedExtents` without changing the destination.
pub fn decode_storage_block(
    kind: u64,
    source: &[u8],
    destination: &mut [u8],
    stride: usize,
) -> Result<(), TensorComputeError> {
    use verdant_storage_block::{run, Adt0 as Request, Adt1 as Instruction};
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "StorageBlock.evaluate".to_string(),
    };
    let layout = || TensorComputeError::ZeroOrMismatchedExtents {
        detail: "storage block extents or stride do not admit every decoded word".to_string(),
    };
    let (lanes, prefix_steps, lane_steps) = match run(Request::C0(
        kind,
        source.len() as u64,
        destination.len() as u64,
        stride as u64,
    ))
    .map_err(|_| layout())?
    {
        Instruction::C0(0) => return Err(failure()),
        Instruction::C0(_) => return Err(layout()),
        Instruction::C1(lanes, _, prefix_steps, lane_steps) => (lanes, prefix_steps, lane_steps),
        _ => return Err(failure()),
    };
    const REGISTER_WORDS: usize = crate::scalars::eight_u64() as usize;
    const SCALAR_OPERATIONS: u64 = crate::scalars::eight_u64();
    let mut execute =
        |instruction, registers: &mut [u64; REGISTER_WORDS]| -> Result<(), TensorComputeError> {
            match instruction {
                Instruction::C2(bank, width, offset, register) => {
                    let input = match bank {
                        0 => source,
                        1 => GGML_QUANTIZED_TABLES,
                        _ => return Err(failure()),
                    };
                    let offset = usize::try_from(offset).map_err(|_| failure())?;
                    let width = usize::try_from(width).map_err(|_| failure())?;
                    let end = offset.checked_add(width).ok_or_else(failure)?;
                    let bytes = input.get(offset..end).ok_or_else(failure)?;
                    let mut word = [0; std::mem::size_of::<u64>()];
                    word.get_mut(..width)
                        .ok_or_else(failure)?
                        .copy_from_slice(bytes);
                    *registers
                        .get_mut(usize::try_from(register).map_err(|_| failure())?)
                        .ok_or_else(failure)? = u64::from_le_bytes(word);
                }
                Instruction::C3(operation, a, b, register) => {
                    let word = match operation {
                        op if op < SCALAR_OPERATIONS => {
                            verdant_scalar_f32::run(operation, a, b).map_err(|_| failure())?
                        }
                        op if op == SCALAR_OPERATIONS => {
                            encode_small_integer(a, b).map_err(|_| failure())?
                        }
                        op if op == SCALAR_OPERATIONS.wrapping_add(1) => {
                            evaluate_small_float(a, b).map_err(|_| failure())?
                        }
                        op if op == SCALAR_OPERATIONS.wrapping_add(crate::scalars::two_u64()) => {
                            convert_float_storage(a, 0, b).map_err(|_| failure())?
                        }
                        op if op
                            == SCALAR_OPERATIONS
                                .wrapping_add(u64::from(crate::scalars::three_u32())) =>
                        {
                            evaluate_encoded_word(u64::from(crate::scalars::three_u32()), a, 0)
                                .map_err(|_| failure())?
                        }
                        _ => return Err(failure()),
                    }
                    .ok_or_else(failure)?;
                    *registers
                        .get_mut(usize::try_from(register).map_err(|_| failure())?)
                        .ok_or_else(failure)? = word;
                }
                Instruction::C4(offset, word) => {
                    let offset = usize::try_from(offset).map_err(|_| failure())?;
                    let bytes = u32::try_from(word).map_err(|_| failure())?.to_le_bytes();
                    let end = offset.checked_add(bytes.len()).ok_or_else(failure)?;
                    destination
                        .get_mut(offset..end)
                        .ok_or_else(failure)?
                        .copy_from_slice(&bytes);
                }
                _ => return Err(failure()),
            }
            Ok(())
        };
    let mut registers = [0; REGISTER_WORDS];
    for step in 0..prefix_steps {
        let [a, b, c, d, e, f, g, h] = registers;
        execute(
            run(Request::C1(kind, step, a, b, c, d, e, f, g, h)).map_err(|_| failure())?,
            &mut registers,
        )?;
    }
    for lane in 0..lanes {
        for step in 0..lane_steps {
            let [a, b, c, d, e, f, g, h] = registers;
            execute(
                run(Request::C2(
                    kind,
                    lane,
                    stride as u64,
                    step,
                    a,
                    b,
                    c,
                    d,
                    e,
                    f,
                    g,
                    h,
                ))
                .map_err(|_| failure())?,
                &mut registers,
            )?;
        }
    }
    Ok(())
}

/// Convert encoded F32 (type 0), F16 (type 1) or BF16 (type 30) storage words.
/// Invalid types and input widths return `None`; checked model overflow is typed.
/// Identity conversions preserve every bit, including signaling NaNs.
pub use verdant_float_storage::run as convert_float_storage;

/// Transform encoded F32/F16/BF16 with scale-bias (0), clamp (1), or leaky ReLU (2).
/// Parameters are raw F32 words. Invalid codes, types and widths return `None`.
/// Composition follows the generic GGML CPU reference's unfused arithmetic.
pub use verdant_float_transform::run as transform_float_storage;

/// Evaluate encoded F64 addition (0), F32-to-F64 widening (1),
/// F64-to-F32 narrowing (2), unsigned-count-to-F32 conversion (3),
/// F64 division (4), or unsigned-count-to-F64 conversion (5).
/// Unknown operations and invalid F32 input widths return None.
///
/// # Errors
/// Returns InvalidKernelSpecification if a certified component or word binding overflows.
pub fn evaluate_wide(operation: u64, a: u64, b: u64) -> Result<Option<u64>, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "ScalarF64.evaluate".to_string(),
    };
    macro_rules! checked {
        ($expression:expr) => {
            match $expression {
                Ok(value) => value,
                Err(_) => return Err(failure()),
            }
        };
    }
    let Some(route) = verdant_quotient_route::run(operation) else {
        return Ok(None);
    };
    let (fields, requires_rounding) = match route {
        verdant_quotient_route::Adt0::C0(operation) => {
            let Some(mode) = verdant_wide_mode::run(operation) else {
                return Ok(None);
            };
            match mode {
                verdant_wide_mode::Adt0::C0 => match checked!(verdant_add_f64::run(a, b)) {
                    None => return Ok(None),
                    Some(verdant_add_f64::Adt0::C0(bits)) => return Ok(Some(bits)),
                    Some(verdant_add_f64::Adt0::C1(s, e, m)) => {
                        let Some(verdant_normalize_f64::Adt0::C0(s, e, m)) =
                            checked!(verdant_normalize_f64::run(s, e, m))
                        else {
                            return Ok(None);
                        };
                        (Some((s, e, m)), true)
                    }
                },
                verdant_wide_mode::Adt0::C1 => {
                    let Some(verdant_widen_f32::Adt0::C0(s, e, f)) =
                        checked!(verdant_widen_f32::run(a))
                    else {
                        return Ok(None);
                    };
                    (Some((s, e, f)), false)
                }
                verdant_wide_mode::Adt0::C2 => {
                    return verdant_narrow_f64::run(a).map_err(|_| failure())
                }
                verdant_wide_mode::Adt0::C3 => {
                    return verdant_count_f32::run(a).map_err(|_| failure())
                }
            }
        }
        verdant_quotient_route::Adt0::C1 => match checked!(verdant_division_class::run(a, b)) {
            None => return Ok(None),
            Some(verdant_division_class::Adt0::C0(bits)) => return Ok(Some(bits)),
            Some(verdant_division_class::Adt0::C1(negative)) => {
                let verdant_division_norm::Adt0::C0(ae, am, _, _) =
                    checked!(verdant_division_norm::run(a));
                let verdant_division_norm::Adt0::C0(be, bm, _, _) =
                    checked!(verdant_division_norm::run(b));
                let Some(quotient) = checked!(verdant_quotient_f64::run(am, bm)) else {
                    return Ok(None);
                };
                match checked!(verdant_division_finish::run(
                    negative, ae, am, be, bm, quotient
                )) {
                    None => return Ok(None),
                    Some(verdant_division_finish::Adt0::C0(bits)) => return Ok(Some(bits)),
                    Some(verdant_division_finish::Adt0::C1(s, e, m)) => (Some((s, e, m)), true),
                }
            }
        },
        verdant_quotient_route::Adt0::C2 => match checked!(verdant_count_f64::run(a)) {
            None => return Ok(None),
            Some(verdant_count_f64::Adt0::C0(bits)) => return Ok(Some(bits)),
            Some(verdant_count_f64::Adt0::C1(s, e, m)) => (Some((s, e, m)), true),
        },
    };
    let Some((s, e, m)) = fields else {
        return Ok(None);
    };
    let (s, e, f) = if requires_rounding {
        let rounded = checked!(verdant_round_mantissa_f64::run(m));
        let Some(verdant_pack_plan_f64::Adt0::C0(s, e, f)) =
            checked!(verdant_pack_plan_f64::run(s, e, rounded))
        else {
            return Ok(None);
        };
        (s, e, f)
    } else {
        (s, e, m)
    };
    let Some(high) = verdant_high_f64::run(s, e, f).map_err(|_| failure())? else {
        return Ok(None);
    };
    let low = verdant_low_f64::run(f);
    let [h0, h1, h2, h3] = u32::try_from(high).map_err(|_| failure())?.to_be_bytes();
    let [l0, l1, l2, l3] = u32::try_from(low).map_err(|_| failure())?.to_be_bytes();
    Ok(Some(u64::from_be_bytes([h0, h1, h2, h3, l0, l1, l2, l3])))
}

/// Evaluate ABS, SGN, NEG, STEP, RELU, SQR, SQRT, HARDSIGMOID, HARDSWISH,
/// FLOOR, CEIL, ROUND and TRUNC (codes 0..12) on encoded float storage.
/// Source/destination pairs are F32/F32, F16/F16, BF16/BF16, F16/F32 and BF16/F32.
/// Unsupported codes/pairs and out-of-width words return `None`.
///
/// # Errors
/// Returns `InvalidKernelSpecification` if a certified component overflows.
pub fn evaluate_float_unary(
    operation: u64,
    source: u64,
    destination: u64,
    bits: u64,
) -> Result<Option<u64>, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "FloatUnary.evaluate".to_string(),
    };
    let Some(plan) =
        verdant_unary_plan::run(operation, source, destination, bits).map_err(|_| failure())?
    else {
        return Ok(None);
    };
    let answer = match plan {
        verdant_unary_plan::Adt0::C0(kind, word) => {
            verdant_scalar_f32::run(kind, word, 0).map_err(|_| failure())
        }
        verdant_unary_plan::Adt0::C1(kind, word) => {
            verdant_unary_select::run(kind, word).map_err(|_| failure())
        }
        verdant_unary_plan::Adt0::C2(kind, word) => {
            verdant_unary_activate::run(kind, word).map_err(|_| failure())
        }
        verdant_unary_plan::Adt0::C3(kind, word) => {
            verdant_unary_round::run(kind, word).map_err(|_| failure())
        }
    }?;
    match answer {
        None => Ok(None),
        Some(word) => convert_float_storage(0, destination, word).map_err(|_| failure()),
    }
}

/// Evaluate an F32 operation using the certified Prism integer encoding.
///
/// Operation codes are add, subtract, multiply, divide, square root, square,
/// negate and absolute value in that order, beginning at zero. Rounding is
/// nearest-even; exceptional operands follow the scalar model. Unknown codes
/// or a model overflow return `InvalidKernelSpecification`.
pub fn evaluate_f32_scalar(
    operation: u64,
    left: f32,
    right: f32,
) -> Result<f32, TensorComputeError> {
    let failure = || TensorComputeError::InvalidKernelSpecification {
        kernel_name: "ScalarF32.evaluate".to_string(),
    };
    let value = verdant_scalar_f32::run(
        operation,
        u64::from(left.to_bits()),
        u64::from(right.to_bits()),
    )
    .map_err(|_| failure())?
    .ok_or_else(failure)?;
    let bits = u32::try_from(value).map_err(|_| failure())?;
    Ok(f32::from_bits(bits))
}

#[cfg(test)]
#[path = "../tests/models/matrix_product_tests.rs"]
pub(crate) mod matrix_product_tests;

#[cfg(test)]
#[path = "../tests/models/scalar_kernel_tests.rs"]
mod scalar_kernel_tests;

#[cfg(test)]
#[path = "../tests/models/wide_accumulator_tests.rs"]
mod wide_accumulator_tests;

#[cfg(test)]
#[path = "../tests/models/small_float_tests.rs"]
mod small_float_tests;

#[cfg(test)]
#[path = "../tests/models/packed_word_tests.rs"]
mod packed_word_tests;
