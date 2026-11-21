//! Pure `{0, 1}` MIR-lifted governance, BVH/CCD, UORC, modularity, and parametric ratchet
//! predicates (`scalars_ratchet.rs`).

use super::*;

/// Pure MIR-lifted `ApprovalCertificate` quorum satisfaction predicate (`M >= 1 && approvers >= M`).
#[must_use]
pub const fn approval_certificate_quorum_met(approver_count: u64, quorum_m: u64) -> bool {
    quorum_m >= 1 && approver_count >= quorum_m
}

/// Pure MIR-lifted `ApprovalCertificate` epoch bounds check (`1 <= finalized_epoch <= current_epoch`).
#[must_use]
pub const fn approval_certificate_epoch_valid(finalized_epoch: u64, current_epoch: u64) -> bool {
    finalized_epoch >= 1 && finalized_epoch <= current_epoch
}

/// Pure MIR-lifted authority-gated `VerdantVcs` commit admission predicate.
#[must_use]
pub const fn vcs_authority_commit_admissible(
    cert_valid: bool,
    target_matches: bool,
    chain_valid: bool,
    quarantine_clear: bool,
) -> bool {
    cert_valid && target_matches && chain_valid && quarantine_clear
}

/// Pure MIR-lifted authority-gated `.holo` release capsule sealing predicate.
#[must_use]
pub const fn release_capsule_authority_valid(
    cert_valid: bool,
    release_action_matches: bool,
    vcs_kappa_matches: bool,
    epoch_valid: bool,
) -> bool {
    cert_valid && release_action_matches && vcs_kappa_matches && epoch_valid
}

/// Pure MIR-lifted GLB binary chunk bounds predicate (`bin_len >= declared_len`).
#[must_use]
pub const fn glb_bin_chunk_bounds_valid(bin_len: u64, declared_len: u64) -> bool {
    bin_len >= declared_len
}

/// Pure MIR-lifted GLB texture dimension cap check (`1 <= dim <= max_edge`).
#[must_use]
pub const fn glb_texture_edge_within_cap(dim: u32, max_edge: u32) -> bool {
    dim >= 1 && max_edge >= 1 && dim <= max_edge
}

/// Pure MIR-lifted GLB PBR channel bitmask count (`albedo + mr + normal + emissive` in `0..=4`).
#[must_use]
pub const fn glb_pbr_active_channel_count(
    has_albedo: bool,
    has_mr: bool,
    has_normal: bool,
    has_emissive: bool,
) -> u32 {
    let a = if has_albedo { 1_u32 } else { 0_u32 };
    let m = if has_mr { 1_u32 } else { 0_u32 };
    let n = if has_normal { 1_u32 } else { 0_u32 };
    let e = if has_emissive { 1_u32 } else { 0_u32 };
    a.wrapping_add(m).wrapping_add(n).wrapping_add(e)
}

/// Pure MIR-lifted Wasm reduction byte delta (`before - after`, saturating at `0`).
#[must_use]
pub const fn wasm_size_reduction_bytes(before_bytes: u64, after_bytes: u64) -> u64 {
    if before_bytes >= after_bytes {
        before_bytes.wrapping_sub(after_bytes)
    } else {
        0
    }
}

/// Pure MIR-lifted Wasm sub-ceiling compliance check (`observed <= ceiling && observed >= 1`).
#[must_use]
pub const fn wasm_binary_under_ceiling(observed_bytes: u64, ceiling_bytes: u64) -> bool {
    observed_bytes >= 1 && observed_bytes <= ceiling_bytes
}

/// Pure MIR-lifted lifecycle gate CI parity predicate (`bdd_ok && engine_ok && clippy_ok && fidelity_ok`).
#[must_use]
pub const fn lifecycle_ci_parity_met(
    bdd_ok: bool,
    engine_ok: bool,
    clippy_ok: bool,
    fidelity_ok: bool,
) -> bool {
    bdd_ok && engine_ok && clippy_ok && fidelity_ok
}

/// Pure MIR-lifted release tag qualification predicate (`version_synced && bootstrap_green && lifecycle_ok`).
#[must_use]
pub const fn release_tag_qualified(
    version_synced: bool,
    bootstrap_green: bool,
    lifecycle_ok: bool,
) -> bool {
    version_synced && bootstrap_green && lifecycle_ok
}

/// Pure MIR-lifted Observer dynamic next Rust modularity target (`max(1000, current - 100)`).
#[must_use]
pub const fn observer_next_rust_loc_target(current_max_lines: u32) -> u32 {
    let floor = thousand_u32();
    let step = hundred_u32();
    if current_max_lines > floor.wrapping_add(step) {
        current_max_lines.wrapping_sub(step)
    } else {
        floor
    }
}

/// Pure MIR-lifted Observer dynamic next JS modularity target (`max(900, current - 100)`).
#[must_use]
pub const fn observer_next_js_loc_target(current_max_lines: u32) -> u32 {
    let nine = eight_u32().wrapping_add(1_u32);
    let floor = hundred_u32().wrapping_mul(nine);
    let step = hundred_u32();
    if current_max_lines > floor.wrapping_add(step) {
        current_max_lines.wrapping_sub(step)
    } else {
        floor
    }
}

/// Pure MIR-lifted sovereign authority non-empty admin quorum check (`N >= 1 && M >= 1 && M <= N`).
#[must_use]
pub const fn authority_quorum_invariants_hold(active_n: u64, threshold_m: u64) -> bool {
    active_n >= 1 && threshold_m >= 1 && threshold_m <= active_n
}

/// Pure MIR-lifted sovereign authority separation-of-duties check (`N == 1 || reviewer != requester`).
#[must_use]
pub const fn authority_separation_of_duties_ok(active_n: u64, reviewer_is_requester: bool) -> bool {
    active_n == 1 || !reviewer_is_requester
}

/// Pure MIR-lifted sovereign authority revocation count bound (`revoked <= prior_approvals`).
#[must_use]
pub const fn authority_revocation_count_bounded(revoked: u64, prior_approvals: u64) -> bool {
    revoked <= prior_approvals
}

/// Pure MIR-lifted sovereign authority boundary key epoch strict advance (`next == prev + 1`).
#[must_use]
pub const fn authority_boundary_key_rotated(prev_key_epoch: u64, next_key_epoch: u64) -> bool {
    next_key_epoch == prev_key_epoch.wrapping_add(1)
}

/// Pure MIR-lifted Stitch capsule compression effectiveness check (`compressed_bytes < raw_bytes`).
#[must_use]
pub const fn capsule_gzip_effective(compressed_bytes: u64, raw_bytes: u64) -> bool {
    compressed_bytes >= 1 && compressed_bytes < raw_bytes
}

/// Pure MIR-lifted Stitch capsule zero-trailing-byte verification (`consumed == total && inflated == declared`).
#[must_use]
pub const fn capsule_body_exact_bounds(
    consumed: u64,
    total_raw: u64,
    inflated: u64,
    declared_payload: u64,
) -> bool {
    consumed == total_raw && inflated == declared_payload
}

/// Pure MIR-lifted Blank Frame headroom predicate (`used <= max_budget`).
#[must_use]
pub const fn blank_frame_within_budget(used_bytes: u64, max_budget_bytes: u64) -> bool {
    used_bytes >= 1 && used_bytes <= max_budget_bytes
}

/// Pure MIR-lifted FR-03 cold-boot transfer compliance predicate (`bytes <= ceiling && requests <= max_requests`).
#[must_use]
pub const fn cold_boot_transfer_compliant(
    transfer_bytes: u64,
    ceiling_bytes: u64,
    requests: u32,
    max_requests: u32,
) -> bool {
    transfer_bytes >= 1
        && transfer_bytes <= ceiling_bytes
        && requests >= 1
        && requests <= max_requests
}

/// Pure MIR-lifted frontier net residue reduction predicate (`next_residue <= prev_residue`).
#[must_use]
pub const fn frontier_residue_non_increasing(prev_residue: u64, next_residue: u64) -> bool {
    next_residue <= prev_residue
}

/// Pure MIR-lifted frontier strict lift growth predicate (`next_lifted > prev_lifted`).
#[must_use]
pub const fn frontier_lifted_strictly_grew(prev_lifted: u64, next_lifted: u64) -> bool {
    next_lifted > prev_lifted
}

/// Pure MIR-lifted frontier uncertified budget non-regression (`next_uncertified <= prev_uncertified`).
#[must_use]
pub const fn frontier_uncertified_non_increasing(
    prev_uncertified: u64,
    next_uncertified: u64,
) -> bool {
    next_uncertified <= prev_uncertified
}

/// Pure MIR-lifted Manager health perfect-or-high band check (`score >= 900 && score <= 1000`).
#[must_use]
pub const fn manager_health_high_band(score_permille: u32) -> bool {
    let nine = eight_u32().wrapping_add(1_u32);
    score_permille >= hundred_u32().wrapping_mul(nine) && score_permille <= thousand_u32()
}

/// Pure MIR-lifted release milestone completion check (`passed_kpis == total_kpis && total_kpis >= 1`).
#[must_use]
pub const fn milestone_kpis_all_green(passed_kpis: u32, total_kpis: u32) -> bool {
    total_kpis >= 1 && passed_kpis == total_kpis
}

/// Pure MIR-lifted lifted-lifecycle 7-category completeness check (every category count `>= 1`).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub const fn lifecycle_seven_categories_complete(
    rust_n: u32,
    test_n: u32,
    lex_n: u32,
    feat_n: u32,
    model_n: u32,
    script_n: u32,
    view_n: u32,
) -> bool {
    rust_n >= 1
        && test_n >= 1
        && lex_n >= 1
        && feat_n >= 1
        && model_n >= 1
        && script_n >= 1
        && view_n >= 1
}

/// Pure MIR-lifted lifted-lifecycle total module count sum across all 7 categories.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub const fn lifecycle_seven_category_total(
    rust_n: u32,
    test_n: u32,
    lex_n: u32,
    feat_n: u32,
    model_n: u32,
    script_n: u32,
    view_n: u32,
) -> u32 {
    rust_n
        .wrapping_add(test_n)
        .wrapping_add(lex_n)
        .wrapping_add(feat_n)
        .wrapping_add(model_n)
        .wrapping_add(script_n)
        .wrapping_add(view_n)
}

/// Pure MIR-lifted Stitch multi-origin availability check (`alive_origins >= 1 && total_origins >= 2`).
#[must_use]
pub const fn stitch_multi_origin_survivable(alive_origins: u32, total_origins: u32) -> bool {
    total_origins >= two_u32() && alive_origins >= 1 && alive_origins <= total_origins
}

/// Pure MIR-lifted PROV-O revision chain integrity check (`revisions >= 1 && active_version >= 1 && active_version <= revisions`).
#[must_use]
pub const fn provo_revision_version_valid(active_version: u32, total_revisions: u32) -> bool {
    total_revisions >= 1 && active_version >= 1 && active_version <= total_revisions
}

/// Pure MIR-lifted Two-Level BVH instance-to-BLAS pool invariant (`instances <= blas_pool`).
#[must_use]
pub const fn tlas_instance_blas_bound_valid(instance_count: u64, blas_pool_len: u64) -> bool {
    instance_count <= blas_pool_len
}

/// Pure MIR-lifted BVH traversal stack depth safety bound (`sp < stack_cap`).
#[must_use]
pub const fn bvh_stack_push_safe(stack_pointer: u32, stack_capacity: u32) -> bool {
    stack_pointer < stack_capacity
}

/// Pure MIR-lifted swept-capsule CCD slide iteration bound (`1 <= iter <= max_slides`).
#[must_use]
pub const fn capsule_slide_iteration_valid(iteration: u32, max_slides: u32) -> bool {
    iteration >= 1 && iteration <= max_slides
}

/// Pure MIR-lifted capsule skin clearance invariant (`clearance_mm >= radius_mm + skin_mm`).
#[must_use]
pub const fn capsule_skin_clearance_safe(clearance_mm: u64, radius_mm: u64, skin_mm: u64) -> bool {
    clearance_mm >= radius_mm.wrapping_add(skin_mm)
}

/// Pure MIR-lifted walkable stair riser height predicate (`riser_mm <= max_step_mm`).
#[must_use]
pub const fn capsule_stair_riser_walkable(riser_mm: u32, max_step_mm: u32) -> bool {
    riser_mm >= 1 && riser_mm <= max_step_mm
}

/// Pure MIR-lifted multi-plane wedge halt trigger (`plane_count >= 3`).
#[must_use]
pub const fn capsule_three_plane_wedge_halt(plane_count: u32) -> bool {
    plane_count >= three_u32()
}

/// Pure MIR-lifted UORC self-delimiting frame header flag validity (`conditional <= 1 && program <= 1`).
#[must_use]
pub const fn uorc_header_flags_valid(conditional: u8, program: u8) -> bool {
    conditional <= 1 && program <= 1
}

/// Pure MIR-lifted UORC progressive stream frame count sufficiency (`frames >= target_rank + 1`).
#[must_use]
pub const fn uorc_stream_frame_count_sufficient(frame_count: u64, target_rank: u64) -> bool {
    frame_count >= target_rank.wrapping_add(1)
}

/// Pure MIR-lifted UORC SHA-256 chain block size (`64` bytes).
#[must_use]
pub const fn uorc_chain_block_bytes() -> u32 {
    sixty_four_u32()
}

/// Pure MIR-lifted UORC SHA-256 seal length (`32` bytes).
#[must_use]
pub const fn uorc_seal_digest_bytes() -> u32 {
    thirty_two_u32()
}

/// Pure MIR-lifted Rust file modularity sub-1500L target compliance (`1 <= lines <= 1500`).
#[must_use]
pub const fn rust_module_sub_1500_lines(lines: u32) -> bool {
    let limit = thousand_u32().wrapping_add(hundred_u32().wrapping_mul(five_u32()));
    lines >= 1 && lines <= limit
}

/// Pure MIR-lifted shipped JS module sub-1125L target compliance (`1 <= lines <= 1125`).
#[must_use]
pub const fn js_module_sub_1125_lines(lines: u32) -> bool {
    let twenty_five = five_u32().wrapping_mul(five_u32());
    let limit = thousand_u32()
        .wrapping_add(hundred_u32())
        .wrapping_add(twenty_five);
    lines >= 1 && lines <= limit
}

/// Pure MIR-lifted PagedTensorArena minimum budget in bytes (`65536` = 64 KiB).
#[must_use]
pub const fn paged_arena_min_budget_bytes() -> u64 {
    sixty_four_u64()
        .wrapping_mul(thirty_two_u64())
        .wrapping_mul(thirty_two_u64())
}

/// Pure MIR-lifted PagedTensorArena default budget in bytes (`67108864` = 64 MiB).
#[must_use]
pub const fn paged_arena_default_budget_bytes() -> u64 {
    let kb = thirty_two_u64().wrapping_mul(thirty_two_u64());
    sixty_four_u64().wrapping_mul(kb).wrapping_mul(kb)
}

/// Pure MIR-lifted PagedTensorArena budget clamping (`max(requested, min_budget)`).
#[must_use]
pub const fn paged_arena_clamped_budget_bytes(requested_bytes: u64) -> u64 {
    let min_b = paged_arena_min_budget_bytes();
    if requested_bytes >= min_b {
        requested_bytes
    } else {
        min_b
    }
}

/// Pure MIR-lifted PagedTensorArena resident watermark compliance (`resident <= budget && peak <= budget`).
#[must_use]
pub const fn paged_arena_resident_within_budget(
    resident_bytes: u64,
    peak_bytes: u64,
    budget_bytes: u64,
) -> bool {
    resident_bytes <= budget_bytes && peak_bytes <= budget_bytes
}

/// Pure MIR-lifted `uorust` residue net reduction predicate (`after < before`).
#[must_use]
pub const fn frontier_residue_strictly_reduced(before_residue: u64, after_residue: u64) -> bool {
    after_residue < before_residue
}

/// Pure MIR-lifted `uorust` uncertified net reduction predicate (`after < before`).
#[must_use]
pub const fn frontier_uncertified_strictly_reduced(
    before_uncertified: u64,
    after_uncertified: u64,
) -> bool {
    after_uncertified < before_uncertified
}

/// Pure MIR-lifted `uorust` lifted-function gain delta (`after - before`, saturating at `0`).
#[must_use]
pub const fn frontier_lifted_gain(before_lifted: u64, after_lifted: u64) -> u64 {
    if after_lifted >= before_lifted {
        after_lifted.wrapping_sub(before_lifted)
    } else {
        0
    }
}

/// Pure MIR-lifted `uorust` residue reduction delta (`before - after`, saturating at `0`).
#[must_use]
pub const fn frontier_residue_reduction(before_residue: u64, after_residue: u64) -> u64 {
    if before_residue >= after_residue {
        before_residue.wrapping_sub(after_residue)
    } else {
        0
    }
}

/// Pure MIR-lifted ContainerOrgan sandbox permission predicate (`!denied && (granted || always_sandboxed)`).
#[must_use]
pub const fn container_organ_access_admitted(
    explicitly_denied: bool,
    explicitly_granted: bool,
    always_sandboxed: bool,
) -> bool {
    !explicitly_denied && (explicitly_granted || always_sandboxed)
}

/// Pure MIR-lifted FrameBudgetProfiler peak comparison step (`max(acc, sample)`).
#[must_use]
pub const fn frame_profiler_max_step(current_max_us: u64, sample_us: u64) -> u64 {
    if sample_us > current_max_us {
        sample_us
    } else {
        current_max_us
    }
}

/// Pure MIR-lifted FrameBudgetProfiler compliance check (`over_budget == 0 && sample_count >= 1`).
#[must_use]
pub const fn frame_profiler_budget_met(over_budget_frames: u64, sample_count: u64) -> bool {
    over_budget_frames == 0 && sample_count >= 1
}

/// Pure MIR-lifted Sabine acoustic room parameter admissibility (`vol >= 10 && abs >= 100`).
#[must_use]
pub const fn sabine_room_params_admissible(volume_m3: u32, absorption_milli_m2: u32) -> bool {
    volume_m3 >= ten_u32() && absorption_milli_m2 >= hundred_u32()
}

/// Pure MIR-lifted Stitch 2-of-3 fragment pair distinctness predicate (`idx_a != idx_b && both < 3`).
#[must_use]
pub const fn stitch_pair_indices_distinct(idx_a: u32, idx_b: u32) -> bool {
    idx_a < three_u32() && idx_b < three_u32() && idx_a != idx_b
}

/// Pure MIR-lifted Stitch 4-of-8 release shard survivability (`available >= 4 && total == 8`).
#[must_use]
pub const fn stitch_release_quorum_survivable(available_shards: u32, total_shards: u32) -> bool {
    total_shards == eight_u32()
        && available_shards >= four_u32()
        && available_shards <= total_shards
}

/// Pure MIR-lifted FR-03 first-visit byte budget check (`observed <= 122880`).
#[must_use]
pub const fn fr03_app_weight_compliant(observed_bytes: u64, max_bytes: u64) -> bool {
    observed_bytes >= 1 && observed_bytes <= max_bytes
}

/// Pure MIR-lifted Blank Frame byte budget check (`observed <= 30720`).
#[must_use]
pub const fn blank_frame_weight_compliant(observed_bytes: u64, max_bytes: u64) -> bool {
    observed_bytes >= 1 && observed_bytes <= max_bytes
}

/// Pure MIR-lifted Observer KPI 100% pass predicate (`total >= 1 && passed == total`).
#[must_use]
pub const fn observer_all_kpis_passed(passed: u64, total: u64) -> bool {
    total >= 1 && passed == total
}

/// Pure MIR-lifted release milestone monotonicity predicate (`current_count >= required_count`).
#[must_use]
pub const fn roadmap_milestones_achieved(current_count: u64, required_count: u64) -> bool {
    current_count >= required_count
}
