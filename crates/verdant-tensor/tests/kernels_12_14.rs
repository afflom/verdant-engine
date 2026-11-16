//! Integration tests for Prism-verified kernels `k12` (Tropical BLAS Swept TOI),
//! `k13` (Exact Two-Bone IK Contraction), and `k14` (Conservation & Tropical Story Automaton).

use verdant_tensor::{
    solve_two_bone_ik_exact, tropical_swept_aabb_toi_bounds, verified_unified_tensor_kernels,
    verify_conservation_and_reachability, TensorSemiringKind,
};

#[test]
fn unified_fifteen_kernels_verified() {
    let kernels = verified_unified_tensor_kernels().expect("15 unified Prism tensor kernels");
    assert_eq!(kernels.len(), 15);
    assert_eq!(kernels[12].name, "TropicalBlasSweptToi");
    assert_eq!(kernels[12].semiring, TensorSemiringKind::TropicalMinPlus);
    assert_eq!(kernels[13].name, "TwoBoneIkJointSolver");
    assert_eq!(kernels[13].semiring, TensorSemiringKind::LinearExact);
    assert_eq!(kernels[14].name, "ConservationStoryAutomaton");
    assert_eq!(kernels[14].semiring, TensorSemiringKind::TropicalMinPlus);
}

#[test]
fn kernel12_tropical_swept_aabb_toi_prunes_misses_and_bounds_hits() {
    let origin = [-2000, 500, 0];
    let radius = 300;
    let delta = [4000, 0, 0];
    let mins = [[-500, 0, -500], [-500, 2000, -500]];
    let maxs = [[500, 1000, 500], [500, 3000, 500]];

    let bounds = tropical_swept_aabb_toi_bounds(origin, radius, delta, &mins, &maxs)
        .expect("tropical swept AABB contraction");
    assert_eq!(bounds.len(), 2);
    let (enter, exit) = bounds[0].expect("first box hit");
    assert!((250..=320).contains(&enter), "enter={enter}");
    assert!((680..=750).contains(&exit), "exit={exit}");
    assert_eq!(bounds[1], None, "elevated box pruned");
}

#[test]
fn kernel13_two_bone_ik_preserves_limb_lengths_and_pole_direction() {
    let hip = [0.0, 1.0, 0.0];
    let ankle_target = [0.0, 0.25, 0.15];
    let pole_forward = [0.0, 0.0, 1.0];
    let sol = solve_two_bone_ik_exact(hip, ankle_target, pole_forward, 0.48, 0.46)
        .expect("two-bone IK solution");
    assert!(sol.target_within_reach);
    assert!(sol.knee_bend_radians > 0.2);
    assert!(sol.mid_joint_pos[2] > 0.05, "knee bends toward +Z pole");

    let d_upper = ((sol.mid_joint_pos[0] - hip[0]).powi(2)
        + (sol.mid_joint_pos[1] - hip[1]).powi(2)
        + (sol.mid_joint_pos[2] - hip[2]).powi(2))
    .sqrt();
    let d_lower = ((sol.end_effector_pos[0] - sol.mid_joint_pos[0]).powi(2)
        + (sol.end_effector_pos[1] - sol.mid_joint_pos[1]).powi(2)
        + (sol.end_effector_pos[2] - sol.mid_joint_pos[2]).powi(2))
    .sqrt();
    assert!((d_upper - 0.48).abs() < 1e-4, "d_upper={d_upper}");
    assert!((d_lower - 0.46).abs() < 1e-4, "d_lower={d_lower}");
    assert!((sol.end_effector_pos[1] - ankle_target[1]).abs() < 1e-4);
}

#[test]
fn kernel14_conservation_and_tropical_reachability_catches_deadlocks_and_duplication() {
    // 3-state graph: 0 -> 1 (cost 100), 1 -> 2 (cost 150), goal = [2]
    let valid_edges = [
        None,
        Some(100),
        None,
        None,
        None,
        Some(150),
        None,
        None,
        None,
    ];
    let valid_ledger = [(10, 5, 3, 12), (12, 0, 2, 10)];
    let report = verify_conservation_and_reachability(&valid_edges, 3, 0, &[2], &valid_ledger)
        .expect("valid deadlock-free conserved automaton");
    assert_eq!(report.min_cost_to_goal_milli, 250);
    assert_eq!(report.reachable_live_states, 3);
    assert_eq!(report.net_flow_delta, 0);

    // Deadlocked trap branch 0 -> 1 where 1 has no path to 2:
    let deadlocked_edges = [
        None,
        Some(100),
        Some(500),
        None,
        None,
        None,
        None,
        None,
        None,
    ];
    assert!(
        verify_conservation_and_reachability(&deadlocked_edges, 3, 0, &[2], &valid_ledger).is_err(),
        "deadlocked branch state 1 must be refused"
    );

    // Duplication violation (10 + 0 - 0 != 11):
    let dup_ledger = [(10, 0, 0, 11)];
    assert!(
        verify_conservation_and_reachability(&valid_edges, 3, 0, &[2], &dup_ledger).is_err(),
        "resource duplication must be refused"
    );
}
