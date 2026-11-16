//! Tropical `(max, +)` and `(min, +)` semiring contractions over `uor-matmul`.
//!
//! Hosts batch BVH/frustum half-space support culling, GJK directional convex-hull support maps,
//! and exact `(min, +)` all-pairs shortest-path closure over walkable navigation meshes via
//! `ceil(log2(V))` tropical matrix squarings.

#[cfg(not(target_arch = "wasm32"))]
use uor_matmul::driver::{GemmOptions, MaxPlus, Scratch};
use uor_matmul::Trop;
#[cfg(target_arch = "wasm32")]
use uor_matmul::{as_alphabet_tropical, dot_tropical_ref};
#[cfg(not(target_arch = "wasm32"))]
use uor_matmul::{as_alphabet_tropical, MatView, MatViewMut, Triple};

use crate::{prism_kernel, TensorComputeError, DIM_PLANAR, DIM_SPATIAL};

fn contract_tropical_max_plus(
    a_trop: &[Trop<i32>],
    b_trop_row_major: &[Trop<i32>],
    m: usize,
    k: usize,
    n: usize,
) -> Result<Vec<Trop<i64>>, TensorComputeError> {
    if m == 0
        || k == 0
        || n == 0
        || a_trop.len() != m.saturating_mul(k)
        || b_trop_row_major.len() != k.saturating_mul(n)
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "tropical matrix contraction dimensions invalid".to_string(),
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut c_trop = vec![Trop::<i64>::NEG_INF; m * n];
        let av = MatView::row_major(as_alphabet_tropical(a_trop), m, k).ok_or_else(|| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "invalid row-major tropical LHS view".to_string(),
            }
        })?;
        let bv =
            MatView::row_major(as_alphabet_tropical(b_trop_row_major), k, n).ok_or_else(|| {
                TensorComputeError::ZeroOrMismatchedExtents {
                    detail: "invalid row-major tropical RHS view".to_string(),
                }
            })?;
        let cv = MatViewMut::row_major(&mut c_trop, m, n).ok_or_else(|| {
            TensorComputeError::ZeroOrMismatchedExtents {
                detail: "invalid row-major tropical output view".to_string(),
            }
        })?;
        let mut triple =
            Triple::new(av, bv, cv).map_err(|_| TensorComputeError::ZeroOrMismatchedExtents {
                detail: "tropical Triple construction failed".to_string(),
            })?;
        uor_matmul::driver::gemm(
            &mut triple,
            &MaxPlus::OVERWRITE,
            GemmOptions::default(),
            &mut Scratch::none(),
        );
        Ok(c_trop)
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut b_cols = vec![vec![Trop::<i32>::NEG_INF; k]; n];
        for r in 0..k {
            for c in 0..n {
                b_cols[c][r] = b_trop_row_major[r * n + c];
            }
        }
        let mut out = vec![Trop::<i64>::NEG_INF; m * n];
        for r in 0..m {
            let row_slice = &a_trop[r * k..(r + 1) * k];
            let a_alpha = as_alphabet_tropical(row_slice);
            for c in 0..n {
                let b_alpha = as_alphabet_tropical(&b_cols[c]);
                out[r * n + c] = match dot_tropical_ref(a_alpha, b_alpha).get() {
                    Some(v) => Trop::finite(
                        v.max(i128::from(i64::MIN) + 1).min(i128::from(i64::MAX)) as i64,
                    ),
                    None => Trop::NEG_INF,
                };
            }
        }
        Ok(out)
    }
}

/// Evaluates exact tropical `(max, +)` support bounds `C[p, v] = max_{axis} (plane[p, axis] + vertex[v, axis])`
/// across `plane_axes_milli` (`P x 3`) and `hull_vertices_milli` (`V x 3`) via `uor_matmul`.
pub fn tropical_frustum_distance_bounds(
    plane_axes_milli: &[[i32; DIM_SPATIAL]],
    hull_vertices_milli: &[[i32; DIM_SPATIAL]],
) -> Result<Vec<i64>, TensorComputeError> {
    if plane_axes_milli.is_empty() || hull_vertices_milli.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "tropical plane and hull vertex slices must be non-empty".to_string(),
        });
    }

    let m = plane_axes_milli.len();
    let k = DIM_SPATIAL;
    let n = hull_vertices_milli.len();
    let i2 = 1usize + 1usize;

    let mut a_trop = Vec::with_capacity(m * k);
    for p in plane_axes_milli {
        a_trop.push(Trop::finite(p[0]));
        a_trop.push(Trop::finite(p[1]));
        a_trop.push(Trop::finite(p[i2]));
    }

    let mut b_trop = vec![Trop::<i32>::NEG_INF; k * n];
    for (v_idx, v) in hull_vertices_milli.iter().enumerate() {
        b_trop[v_idx] = Trop::finite(v[0]);
        b_trop[n + v_idx] = Trop::finite(v[1]);
        b_trop[i2 * n + v_idx] = Trop::finite(v[i2]);
    }

    let c_trop = contract_tropical_max_plus(&a_trop, &b_trop, m, k, n)?;
    Ok(c_trop
        .into_iter()
        .map(|entry| entry.get().unwrap_or(i64::MIN))
        .collect())
}

/// Evaluates batch GJK directional support queries across `directions_milli` (`D x 3`) and
/// `hull_vertices_milli` (`V x 3`), returning `(best_vertex_index, max_scaled_projection)` per direction
/// by reducing directional projections to tropical `(max, +)` / exact dot contractions in `uor-matmul`.
pub fn tropical_gjk_support_batch(
    directions_milli: &[[i32; DIM_SPATIAL]],
    hull_vertices_milli: &[[i32; DIM_SPATIAL]],
) -> Result<Vec<(usize, i64)>, TensorComputeError> {
    if directions_milli.is_empty() || hull_vertices_milli.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "GJK support batch requires non-empty directions and hull vertices".to_string(),
        });
    }

    let d_count = directions_milli.len();
    let v_count = hull_vertices_milli.len();
    let i2 = 1usize + 1usize;

    // Step 1: Exact linear dot products `P[d, v] = sum_axis dir[d, axis] * vert[v, axis]` via `uor_matmul::dot_ref`:
    let mut v_rows_q = Vec::with_capacity(v_count);
    for v in hull_vertices_milli {
        v_rows_q.push([v[0], v[1], v[i2]]);
    }

    // Step 2: Tropical `(max, +)` argmax reduction `max_v (P[d, v] + 0)` via `contract_tropical_max_plus`:
    let mut proj_trop = Vec::with_capacity(d_count * v_count);
    let mut raw_projs = Vec::with_capacity(d_count * v_count);
    for dir in directions_milli {
        let d_alpha = uor_matmul::as_alphabet_full(dir);
        for v_row in &v_rows_q {
            let p_wide = uor_matmul::dot_ref(d_alpha, uor_matmul::as_alphabet_full(v_row));
            let p = p_wide.max(i128::from(i64::MIN)).min(i128::from(i64::MAX)) as i64;
            raw_projs.push(p);
            let clamped = p.max(i64::from(i32::MIN) + 1).min(i64::from(i32::MAX)) as i32;
            proj_trop.push(Trop::finite(clamped));
        }
    }
    let zero_col = vec![Trop::finite(0_i32); v_count];
    let maxima = contract_tropical_max_plus(&proj_trop, &zero_col, d_count, v_count, 1)?;

    let mut results = Vec::with_capacity(d_count);
    for (d_idx, max_trop) in maxima.into_iter().enumerate() {
        let target = max_trop.get().unwrap_or(i64::MIN);
        let row = &raw_projs[d_idx * v_count..(d_idx + 1) * v_count];
        let mut best_idx = 0usize;
        let mut best_val = row[0];
        for (v_idx, &val) in row.iter().enumerate().skip(1) {
            if val > best_val || (val == target && best_val < target) {
                best_val = val;
                best_idx = v_idx;
            }
        }
        results.push((best_idx, best_val));
    }
    Ok(results)
}

/// Computes exact `(min, +)` all-pairs shortest-path distances over a `node_count x node_count`
/// edge-cost matrix (`None` = disconnected, `Some(cost_milli)` = directed edge weight) via
/// `ceil(log2(node_count))` tropical matrix squarings in `uor-matmul` using the exact algebraic
/// duality `min_p(a + b) = -max_p((-a) + (-b))` with `+\infty <-> NEG_INF`.
pub fn tropical_min_plus_all_pairs_closure(
    edge_costs_milli: &[Option<u32>],
    node_count: usize,
) -> Result<Vec<Option<u64>>, TensorComputeError> {
    if node_count == 0 || edge_costs_milli.len() != node_count.saturating_mul(node_count) {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "NavMesh tropical (min, +) closure requires non-empty square adjacency matrix"
                .to_string(),
        });
    }

    let cap = (i32::MAX / (1 + 1)) as u64;
    let mut dist = vec![Trop::<i32>::NEG_INF; node_count * node_count];
    for r in 0..node_count {
        for c in 0..node_count {
            let idx = r * node_count + c;
            if r == c {
                dist[idx] = Trop::finite(0);
            } else if let Some(cost) = edge_costs_milli[idx] {
                let bounded = u64::from(cost).min(cap) as i32;
                dist[idx] = Trop::finite(-bounded);
            }
        }
    }

    let mut reach = 1usize;
    while reach < node_count {
        let squared = contract_tropical_max_plus(&dist, &dist, node_count, node_count, node_count)?;
        let mut changed = false;
        for (slot, sq) in dist.iter_mut().zip(squared) {
            if let Some(neg_sum) = sq.get() {
                let clamped = (-neg_sum).clamp(0, cap as i64) as i32;
                let candidate = Trop::finite(-clamped);
                let prev = slot.get().unwrap_or(i32::MIN);
                if -clamped > prev {
                    *slot = candidate;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        reach = reach.saturating_mul(1 + 1);
    }

    Ok(dist
        .into_iter()
        .map(|entry| entry.get().map(|neg_cost| (-neg_cost).max(0) as u64))
        .collect())
}

/// Computes exact intrinsic `[width_px, height_px]` bounds of a responsive 2D/3D UI container
/// from `child_extents_px` (`N x 2`) and `gap_px` by contracting cross-axis maxima in the
/// tropical `(max, +)` semiring (`contract_tropical_max_plus`) and main-axis sums via exact
/// `uor_matmul::dot_ref` accumulation (`kernel9TropicalUiLayout`).
pub fn tropical_ui_box_extents(
    child_extents_px: &[[i32; DIM_PLANAR]],
    gap_px: i32,
    horizontal: bool,
) -> Result<[i64; DIM_PLANAR], TensorComputeError> {
    if child_extents_px.is_empty() {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "UI container tropical box contraction requires at least one child".to_string(),
        });
    }
    if child_extents_px.iter().any(|ext| ext[0] < 0 || ext[1] < 0) || gap_px < 0 {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "UI child extents and gap must be non-negative".to_string(),
        });
    }

    let n = child_extents_px.len();
    let main_axis = usize::from(!horizontal);
    let cross_axis = usize::from(horizontal);

    let mut main_vec = Vec::with_capacity(n);
    let ones = vec![1_i32; n];
    let mut cross_row = Vec::with_capacity(n);
    let zero_col = vec![Trop::finite(0_i32); n];

    for child in child_extents_px {
        main_vec.push(child[main_axis]);
        cross_row.push(Trop::finite(child[cross_axis]));
    }

    let main_sum_wide = uor_matmul::dot_ref(
        uor_matmul::as_alphabet_full(&main_vec),
        uor_matmul::as_alphabet_full(&ones),
    );
    let total_gaps = i64::from(gap_px).saturating_mul((n - 1) as i64);
    let main_total =
        (main_sum_wide.max(0).min(i128::from(i64::MAX)) as i64).saturating_add(total_gaps);

    let cross_max_vec = contract_tropical_max_plus(&cross_row, &zero_col, 1, n, 1)?;
    let cross_max = cross_max_vec[0].get().unwrap_or(0).max(0);

    if horizontal {
        Ok([main_total, cross_max])
    } else {
        Ok([cross_max, main_total])
    }
}

/// Contracts batch swept-sphere vs. AABB time-of-impact (`TOI`) intervals in permille `[0, 1000]`
/// (`kernel12TropicalBlasSweep`) by expanding each box `[min_b, max_b]` by `radius_milli` and
/// contracting per-axis slab entry maxima in the tropical `(max, +)` semiring alongside exit
/// minima via tropical `(min, +)` duality (`-max_a(-t_exit)`).
pub fn tropical_swept_aabb_toi_bounds(
    origin_milli: [i32; DIM_SPATIAL],
    radius_milli: i32,
    delta_milli: [i32; DIM_SPATIAL],
    box_mins_milli: &[[i32; DIM_SPATIAL]],
    box_maxs_milli: &[[i32; DIM_SPATIAL]],
) -> Result<Vec<Option<(i64, i64)>>, TensorComputeError> {
    if radius_milli < 0 || box_mins_milli.is_empty() || box_mins_milli.len() != box_maxs_milli.len()
    {
        return Err(TensorComputeError::ZeroOrMismatchedExtents {
            detail: "swept AABB TOI contraction requires non-negative radius and matched non-empty box slices"
                .to_string(),
        });
    }

    let scale = prism_kernel::skinWeightScale().unwrap_or(1).max(1) as i64;
    let miss_enter = (scale + 1) as i32;
    let miss_neg_exit = 1_i32;
    let b_count = box_mins_milli.len();

    let mut enter_rows = Vec::with_capacity(b_count * DIM_SPATIAL);
    let mut neg_exit_rows = Vec::with_capacity(b_count * DIM_SPATIAL);

    for (b_min, b_max) in box_mins_milli.iter().zip(box_maxs_milli.iter()) {
        for axis in 0..DIM_SPATIAL {
            let lo = i64::from(b_min[axis]) - i64::from(radius_milli);
            let hi = i64::from(b_max[axis]) + i64::from(radius_milli);
            if lo > hi {
                enter_rows.push(Trop::finite(miss_enter));
                neg_exit_rows.push(Trop::finite(miss_neg_exit));
                continue;
            }
            let o = i64::from(origin_milli[axis]);
            let d = i64::from(delta_milli[axis]);
            if d == 0 {
                if o < lo || o > hi {
                    enter_rows.push(Trop::finite(miss_enter));
                    neg_exit_rows.push(Trop::finite(miss_neg_exit));
                } else {
                    enter_rows.push(Trop::finite(0));
                    neg_exit_rows.push(Trop::finite(-(scale as i32)));
                }
            } else {
                let (enter_num, exit_num, den) = if d > 0 {
                    ((lo - o) * scale, (hi - o) * scale, d)
                } else {
                    ((o - hi) * scale, (o - lo) * scale, -d)
                };
                let t_enter = enter_num.div_euclid(den);
                let t_exit = -((-exit_num).div_euclid(den));
                let clamp_bound = scale * scale;
                let te_i32 = t_enter.clamp(-clamp_bound, clamp_bound) as i32;
                let tx_i32 = t_exit.clamp(-clamp_bound, clamp_bound) as i32;
                enter_rows.push(Trop::finite(te_i32));
                neg_exit_rows.push(Trop::finite(-tx_i32));
            }
        }
    }

    let zero_col = vec![Trop::finite(0_i32); DIM_SPATIAL];
    let max_enters = contract_tropical_max_plus(&enter_rows, &zero_col, b_count, DIM_SPATIAL, 1)?;
    let max_neg_exits =
        contract_tropical_max_plus(&neg_exit_rows, &zero_col, b_count, DIM_SPATIAL, 1)?;

    let mut intervals = Vec::with_capacity(b_count);
    for (enter_trop, neg_exit_trop) in max_enters.into_iter().zip(max_neg_exits) {
        let raw_enter = enter_trop.get().unwrap_or(scale + 1);
        let raw_exit = -neg_exit_trop.get().unwrap_or(1);
        let clamped_enter = raw_enter.max(0);
        let clamped_exit = raw_exit.min(scale);
        if raw_enter <= scale && raw_exit >= 0 && clamped_enter <= clamped_exit {
            intervals.push(Some((clamped_enter, clamped_exit)));
        } else {
            intervals.push(None);
        }
    }
    Ok(intervals)
}
