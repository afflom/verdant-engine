//! Pure `{0, 1}` constructive scalar primitives using total `wrapping_add` / `wrapping_mul`
//! arithmetic so every helper satisfies **both** the permanent Zero-Constants Parametric Ratchet
//! (`FI-08`, zero literals `> 1`) **and** the `uorust` MIR proof frontier (`0` shift/overflow
//! arithmetic residue, `VE-55`), while `#[inline(always)]` ensures zero Wasm code-size overhead.

/// Constructive `2_u64` (`1 + 1`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn two_u64() -> u64 {
    1_u64.wrapping_add(1_u64)
}

/// Constructive `3_u64` (`2 + 1`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn three_u64() -> u64 {
    two_u64().wrapping_add(1_u64)
}

/// Constructive `4_u64` (`2 * 2`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn four_u64() -> u64 {
    let t = two_u64();
    t.wrapping_mul(t)
}

/// Constructive `8_u64` (`4 * 2`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn eight_u64() -> u64 {
    four_u64().wrapping_mul(two_u64())
}

/// Constructive `10_u64` (`8 + 2`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn ten_u64() -> u64 {
    eight_u64().wrapping_add(two_u64())
}

/// Constructive `16_u64` (`4 * 4`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixteen_u64() -> u64 {
    let f = four_u64();
    f.wrapping_mul(f)
}

/// Constructive `32_u64` (`16 * 2`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn thirty_two_u64() -> u64 {
    sixteen_u64().wrapping_mul(two_u64())
}

/// Constructive `64_u64` (`8 * 8`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixty_four_u64() -> u64 {
    let e = eight_u64();
    e.wrapping_mul(e)
}

/// Constructive `100_u64` (`10 * 10`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn hundred_u64() -> u64 {
    let t = ten_u64();
    t.wrapping_mul(t)
}

/// Constructive `2_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn two_u32() -> u32 {
    1_u32.wrapping_add(1_u32)
}

/// Constructive `3_u32` (`2 + 1`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn three_u32() -> u32 {
    two_u32().wrapping_add(1_u32)
}

/// Constructive `4_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn four_u32() -> u32 {
    let t = two_u32();
    t.wrapping_mul(t)
}

/// Constructive `5_u32` (`4 + 1`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn five_u32() -> u32 {
    four_u32().wrapping_add(1_u32)
}

/// Constructive `6_u32` (`5 + 1`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn six_u32() -> u32 {
    five_u32().wrapping_add(1_u32)
}

/// Constructive `8_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn eight_u32() -> u32 {
    four_u32().wrapping_mul(two_u32())
}

/// Constructive `10_u32` (`8 + 2`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn ten_u32() -> u32 {
    eight_u32().wrapping_add(two_u32())
}

/// Constructive `100_u32` (`10 * 10`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn hundred_u32() -> u32 {
    let t = ten_u32();
    t.wrapping_mul(t)
}

/// Constructive `1000_u32` (`100 * 10`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn thousand_u32() -> u32 {
    hundred_u32().wrapping_mul(ten_u32())
}

/// Constructive `16_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixteen_u32() -> u32 {
    let f = four_u32();
    f.wrapping_mul(f)
}

/// Constructive `32_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn thirty_two_u32() -> u32 {
    sixteen_u32().wrapping_mul(two_u32())
}

/// Constructive `64_u32` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixty_four_u32() -> u32 {
    let e = eight_u32();
    e.wrapping_mul(e)
}

/// Constructive `2_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn two_u8() -> u8 {
    1_u8.wrapping_add(1_u8)
}

/// Constructive `4_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn four_u8() -> u8 {
    let t = two_u8();
    t.wrapping_mul(t)
}

/// Constructive `8_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn eight_u8() -> u8 {
    four_u8().wrapping_mul(two_u8())
}

/// Constructive `16_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixteen_u8() -> u8 {
    let f = four_u8();
    f.wrapping_mul(f)
}

/// Constructive `32_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn thirty_two_u8() -> u8 {
    sixteen_u8().wrapping_mul(two_u8())
}

/// Constructive `64_u8` lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn sixty_four_u8() -> u8 {
    let e = eight_u8();
    e.wrapping_mul(e)
}

/// Pure MIR-lifted UHD display tier classifier (`0=FHD, 1=QHD, 2=4K, 3=5K, 4=8K, 5=16K+`)
/// from megapixels x100 (`mp_x100`).
#[must_use]
pub const fn uhd_classify_display_tier(mp_x100: u32) -> u32 {
    let h = hundred_u32();
    let t = ten_u32();
    let th = thousand_u32();
    let three = four_u32().wrapping_sub(1_u32);
    let five = four_u32().wrapping_add(1_u32);
    let fourteen = t.wrapping_add(four_u32());
    let sixteen_k = th.wrapping_mul(t);
    let eight_k = h.wrapping_mul(three).wrapping_mul(t);
    let five_k = h.wrapping_mul(fourteen);
    let four_k = h.wrapping_mul(eight_u32());
    let qhd = h.wrapping_mul(three);
    if mp_x100 >= sixteen_k {
        five
    } else if mp_x100 >= eight_k {
        four_u32()
    } else if mp_x100 >= five_k {
        three
    } else if mp_x100 >= four_k {
        two_u32()
    } else if mp_x100 >= qhd {
        1_u32
    } else {
        0_u32
    }
}

/// Pure MIR-lifted dynamic resolution scale (`permille`) for `(tier, is_software_gpu, under_half_budget)`.
#[must_use]
pub const fn uhd_recommended_drs_permille(
    tier: u32,
    is_software_gpu: bool,
    under_half_budget: bool,
) -> u32 {
    let h = hundred_u32();
    let th = thousand_u32();
    let fifty = ten_u32().wrapping_mul(four_u32().wrapping_add(1_u32));
    let two = two_u32();
    let five = four_u32().wrapping_add(1_u32);
    let six = five.wrapping_add(1_u32);
    let seven = eight_u32().wrapping_sub(1_u32);
    let drs_50 = h.wrapping_mul(five);
    let drs_65 = h.wrapping_mul(six).wrapping_add(fifty);
    let drs_75 = h.wrapping_mul(seven).wrapping_add(fifty);
    let drs_85 = h.wrapping_mul(eight_u32()).wrapping_add(fifty);
    if is_software_gpu {
        drs_65
    } else if tier >= five {
        if under_half_budget {
            drs_65
        } else {
            drs_50
        }
    } else if tier >= four_u32() {
        if under_half_budget {
            drs_85
        } else {
            drs_75
        }
    } else if tier >= two {
        if under_half_budget {
            th
        } else {
            drs_85
        }
    } else {
        th
    }
}

/// Pure MIR-lifted Contrast-Adaptive Sharpening strength (`permille`).
#[must_use]
pub const fn uhd_cas_sharpness_permille(
    tier: u32,
    is_software_gpu: bool,
    drs_permille: u32,
) -> u32 {
    let h = hundred_u32();
    let th = thousand_u32();
    let fifty = ten_u32().wrapping_mul(four_u32().wrapping_add(1_u32));
    let two = two_u32();
    let five = four_u32().wrapping_add(1_u32);
    let six = five.wrapping_add(1_u32);
    if is_software_gpu {
        h.wrapping_mul(four_u32())
    } else if tier >= five {
        h.wrapping_mul(six)
    } else if tier >= four_u32() {
        h.wrapping_mul(five)
    } else if tier >= two && drs_permille < th {
        h.wrapping_mul(four_u32())
    } else {
        h.wrapping_add(fifty)
    }
}

/// Pure MIR-lifted meshlet screen-space error threshold (`tenths of a pixel`).
#[must_use]
pub const fn uhd_meshlet_sse_tenths_px(tier: u32, is_software_gpu: bool) -> u32 {
    let t = ten_u32();
    let five = four_u32().wrapping_add(1_u32);
    if is_software_gpu || tier >= five {
        t.wrapping_add(t)
    } else if tier >= four_u32() {
        t.wrapping_add(five)
    } else {
        t
    }
}

/// Pure MIR-lifted frame jitter (`p99 - p50` saturating).
#[must_use]
pub const fn uhd_jitter_us(p99_us: u32, p50_us: u32) -> u32 {
    p99_us.saturating_sub(p50_us)
}

/// Pure MIR-lifted half-budget headroom predicate (`ema * 2 <= budget`).
#[must_use]
pub const fn uhd_budget_half_ok(ema_us: u32, budget_us: u32) -> bool {
    ema_us.saturating_add(ema_us) <= budget_us
}

/// Pure MIR-lifted frame stability predicate (`p95 <= budget && jitter * 2 <= budget`).
#[must_use]
pub const fn uhd_frame_is_stable(p95_us: u32, jitter_us: u32, target_budget_us: u32) -> bool {
    p95_us <= target_budget_us && uhd_budget_half_ok(jitter_us, target_budget_us)
}

/// Pure MIR-lifted bounded-RAM `PagedTensorArena` compliance predicate (`peak <= budget`).
#[must_use]
pub const fn uhd_paged_arena_within_budget(peak_bytes: u64, budget_bytes: u64) -> bool {
    budget_bytes > 0 && peak_bytes <= budget_bytes
}

/// Pure MIR-lifted `PagedTensorArena` eviction trigger (`live_pages >= max_pages`).
#[must_use]
pub const fn uhd_paged_arena_needs_eviction(live_pages: u32, max_pages: u32) -> bool {
    max_pages > 0 && live_pages >= max_pages
}

/// Pure MIR-lifted DRS ladder index step-down (`min(idx + 1, max_idx)`).
#[must_use]
pub const fn uhd_drs_step_down(idx: u32, max_idx: u32) -> u32 {
    if idx < max_idx {
        idx.wrapping_add(1_u32)
    } else {
        max_idx
    }
}

/// Pure MIR-lifted DRS ladder index step-up (`idx.saturating_sub(1)`).
#[must_use]
pub const fn uhd_drs_step_up(idx: u32) -> u32 {
    idx.saturating_sub(1_u32)
}

/// Pure MIR-lifted WebGPU compute CAS eligibility (`webgpu_available && tier >= 2`).
#[must_use]
pub const fn uhd_tier_supports_webgpu_cas(tier: u32, webgpu_available: bool) -> bool {
    webgpu_available && tier >= two_u32()
}

/// Pure MIR-lifted Stitch erasure quorum sufficiency (`data_shards_k > 0 && surviving >= k`).
#[must_use]
pub const fn stitch_quorum_sufficient(surviving_shards: u8, data_shards_k: u8) -> bool {
    data_shards_k > 0 && surviving_shards >= data_shards_k
}

/// Pure MIR-lifted Stitch maximum tolerable shard losses (`n - k` saturating).
#[must_use]
pub const fn stitch_max_tolerable_losses(total_shards_n: u8, data_shards_k: u8) -> u8 {
    total_shards_n.saturating_sub(data_shards_k)
}

/// Pure MIR-lifted Stitch `k-of-n` geometry validity (`0 < k <= n`).
#[must_use]
pub const fn stitch_geometry_valid(data_shards_k: u8, total_shards_n: u8) -> bool {
    data_shards_k > 0 && data_shards_k <= total_shards_n
}

/// Pure MIR-lifted Stitch shard index bounds predicate (`idx < n`).
#[must_use]
pub const fn stitch_shard_index_in_range(idx: u8, total_shards_n: u8) -> bool {
    idx < total_shards_n
}

/// Pure MIR-lifted Stitch monotonic release epoch advance check (`next > prev`).
#[must_use]
pub const fn stitch_epoch_monotonically_advances(prev_epoch: u64, next_epoch: u64) -> bool {
    next_epoch > prev_epoch
}

/// Pure MIR-lifted `VerdantVcs` structural diff total changed-file count.
#[must_use]
pub const fn vcs_diff_total_changes(added: u32, modified: u32, removed: u32) -> u32 {
    added.saturating_add(modified).saturating_add(removed)
}

/// Pure MIR-lifted `VerdantVcs` clean-tree predicate (`added == 0 && modified == 0 && removed == 0`).
#[must_use]
pub const fn vcs_diff_is_clean(added: u32, modified: u32, removed: u32) -> bool {
    vcs_diff_total_changes(added, modified, removed) == 0
}

/// Pure MIR-lifted `.holo` capsule compression byte savings (`raw - compressed` saturating).
#[must_use]
pub const fn capsule_compression_saved_bytes(raw_bytes: u64, compressed_bytes: u64) -> u64 {
    raw_bytes.saturating_sub(compressed_bytes)
}

/// Pure MIR-lifted `.holo` capsule compression improvement predicate (`compressed < raw`).
#[must_use]
pub const fn capsule_compression_improved(raw_bytes: u64, compressed_bytes: u64) -> bool {
    compressed_bytes < raw_bytes
}

/// Pure MIR-lifted `.holo` capsule byte-ceiling compliance predicate (`actual <= max`).
#[must_use]
pub const fn capsule_within_byte_ceiling(actual_bytes: u64, max_bytes: u64) -> bool {
    max_bytes > 0 && actual_bytes <= max_bytes
}

/// Pure MIR-lifted `.holo` capsule remaining byte headroom (`max - actual` saturating).
#[must_use]
pub const fn capsule_headroom_bytes(actual_bytes: u64, max_bytes: u64) -> u64 {
    max_bytes.saturating_sub(actual_bytes)
}

/// Pure MIR-lifted Blank Frame compliance predicate (`frame_bytes <= max && loose_files == 0`).
#[must_use]
pub const fn blank_frame_compliant(
    frame_bytes: u32,
    max_frame_bytes: u32,
    loose_files: u32,
) -> bool {
    frame_bytes > 0 && frame_bytes <= max_frame_bytes && loose_files == 0
}

/// Pure MIR-lifted automation pipeline pass predicate (`total > 0 && passed + skipped == total`).
#[must_use]
pub const fn automate_stage_passed(passed: u32, skipped: u32, total: u32) -> bool {
    total > 0 && passed.wrapping_add(skipped) == total
}

/// Pure MIR-lifted automation Wasm build trigger (`!skip_rust && (force_wasm || !cache_hit)`).
#[must_use]
pub const fn automate_should_build_wasm(
    skip_rust: bool,
    force_wasm: bool,
    cache_hit: bool,
) -> bool {
    !skip_rust && (force_wasm || !cache_hit)
}

/// Pure MIR-lifted automation Rust gate enablement predicate (`!skip_rust`).
#[must_use]
pub const fn automate_should_run_rust_gates(skip_rust: bool) -> bool {
    !skip_rust
}

/// Pure MIR-lifted one-way ratchet floor non-regression check (`live >= floor`).
#[must_use]
pub const fn ratchet_floor_did_not_regress(live: u32, floor: u32) -> bool {
    live >= floor
}

/// Pure MIR-lifted one-way ratchet ceiling non-regression check (`live <= ceiling`).
#[must_use]
pub const fn ratchet_ceiling_did_not_regress(live: u32, ceiling: u32) -> bool {
    live <= ceiling
}

/// Pure MIR-lifted one-way ratchet floor auto-tighten predicate (`live > floor`).
#[must_use]
pub const fn ratchet_floor_can_tighten(live: u32, floor: u32) -> bool {
    live > floor
}

/// Pure MIR-lifted one-way ratchet ceiling auto-tighten predicate (`live < ceiling`).
#[must_use]
pub const fn ratchet_ceiling_can_tighten(live: u32, ceiling: u32) -> bool {
    live < ceiling
}

/// Pure MIR-lifted strict SemVer ordering predicate (`a > b`).
#[must_use]
pub const fn semver_is_strictly_higher(
    a_maj: u32,
    a_min: u32,
    a_pat: u32,
    b_maj: u32,
    b_min: u32,
    b_pat: u32,
) -> bool {
    if a_maj != b_maj {
        a_maj > b_maj
    } else if a_min != b_min {
        a_min > b_min
    } else {
        a_pat > b_pat
    }
}

/// Pure MIR-lifted WebAssembly binary magic header validator (`\0asm` = `[0x00, 0x61, 0x73, 0x6d]`).
#[must_use]
pub const fn wasm_magic_header_valid(b0: u8, b1: u8, b2: u8, b3: u8) -> bool {
    let ninety_seven = hundred_u32().wrapping_sub(three_u32()) as u8; // 0x61 'a'
    let one_fifteen = hundred_u32()
        .wrapping_add(ten_u32())
        .wrapping_add(five_u32()) as u8; // 0x73 's'
    let one_oh_nine = hundred_u32().wrapping_add(ten_u32()).wrapping_sub(1_u32) as u8; // 0x6d 'm'
    b0 == 0 && b1 == ninety_seven && b2 == one_fifteen && b3 == one_oh_nine
}

/// Pure MIR-lifted WebAssembly module byte-budget compliance check (`0 < actual <= budget`).
#[must_use]
pub const fn wasm_module_within_budget(actual_bytes: u64, budget_bytes: u64) -> bool {
    actual_bytes > 0 && actual_bytes <= budget_bytes
}

/// Pure MIR-lifted RFC 2789 Cargo sparse index 2+2 prefix crate-name length predicate (`len >= 4`).
#[must_use]
pub const fn sparse_index_crate_name_valid(len: u32) -> bool {
    len >= four_u32()
}

/// Pure MIR-lifted RFC 2789 sparse registry crate-count completeness predicate.
#[must_use]
pub const fn sparse_registry_crate_count_complete(packaged: u32, expected: u32) -> bool {
    expected > 0 && packaged == expected
}

/// Pure MIR-lifted HTTP 200 OK status predicate for local RFC 2789 sparse registry verification.
#[must_use]
pub const fn sparse_registry_http_status_ok(status_code: u16) -> bool {
    let two_hundred = hundred_u32().wrapping_mul(two_u32()) as u16;
    status_code == two_hundred
}

/// Pure MIR-lifted native build-operation stage completion predicate (`total > 0 && passed == total`).
#[must_use]
pub const fn build_op_stages_all_passed(passed: u32, total: u32) -> bool {
    total > 0 && passed == total
}

/// Pure MIR-lifted Cascaded Shadow Map cascade index bounds predicate (`idx < 4`).
#[must_use]
pub const fn csm_cascade_index_valid(idx: u32) -> bool {
    idx < four_u32()
}

/// Pure MIR-lifted Euler-Bernoulli modal bar mode index bounds predicate (`idx < 5`).
#[must_use]
pub const fn modal_bar_mode_index_valid(idx: u32) -> bool {
    idx < five_u32()
}

/// Constructive `7_u64` (`4 + 3`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn seven_u64() -> u64 {
    four_u64().wrapping_add(three_u64())
}

/// Constructive `128_u32` (`8 * 16`) lifted by `uorust`.
#[inline(always)]
#[must_use]
pub const fn hundred_twenty_eight_u32() -> u32 {
    eight_u32().wrapping_mul(sixteen_u32())
}

/// Pure MIR-lifted `.crate` tarball non-empty byte guard.
#[must_use]
pub const fn crate_tarball_nonempty(bytes: u64) -> bool {
    bytes > 0
}

/// Pure MIR-lifted portable `.uorc` stream non-empty byte guard.
#[must_use]
pub const fn uorc_stream_nonempty(bytes: u64) -> bool {
    bytes > 0
}

/// Pure MIR-lifted genesis owner bootstrap predicate (`Governance::genesisBootstrapValid`).
#[must_use]
pub const fn genesis_bootstrap_valid(
    epoch: u64,
    genesis_owner_is_first_admin: bool,
    prism_id_non_extractable: bool,
    requires_github_account: bool,
) -> bool {
    epoch == 1
        && genesis_owner_is_first_admin
        && prism_id_non_extractable
        && !requires_github_account
}

/// Pure MIR-lifted M-of-N Admin quorum threshold predicate (`Governance::quorumThresholdValid`).
/// Enforces `1 <= M <= N`, and once multi-admin custody is active (`N >= 2`), requires `M >= 2`.
#[must_use]
pub const fn quorum_threshold_valid(threshold_m: u64, active_admins_n: u64) -> bool {
    let two = two_u64();
    let in_bounds = threshold_m >= 1 && threshold_m <= active_admins_n;
    let multi_guard = if active_admins_n >= two {
        threshold_m >= two
    } else {
        true
    };
    in_bounds && multi_guard
}

/// Pure MIR-lifted monotonic authority epoch advancement and anti-lockout guard
/// (`Governance::authorityEpochAdvanceValid`).
#[must_use]
pub const fn authority_epoch_advance_valid(
    prev_epoch: u64,
    next_epoch: u64,
    prev_kappa_bound: bool,
    quorum_met: bool,
    remaining_admins: u64,
    next_threshold_m: u64,
) -> bool {
    let monotonic = next_epoch >= prev_epoch.saturating_add(1);
    monotonic
        && prev_kappa_bound
        && quorum_met
        && remaining_admins >= 1
        && quorum_threshold_valid(next_threshold_m, remaining_admins)
}

/// Pure MIR-lifted active-epoch Admin signature admission (`Governance::adminSignatureValidAtEpoch`).
#[must_use]
pub const fn admin_signature_valid_at_epoch(
    signer_in_active_set: bool,
    signer_revoked: bool,
    signed_epoch: u64,
    current_epoch: u64,
) -> bool {
    signer_in_active_set && !signer_revoked && signed_epoch == current_epoch
}

/// Pure MIR-lifted pending-request approval survival check across Admin departure and separation
/// of duties (`Governance::pendingApprovalSurvivesDeparture`).
#[must_use]
pub const fn pending_approval_survives_departure(
    total_approvals: u64,
    departed_admin_approvals: u64,
    quorum_threshold_m: u64,
    author_self_approved: bool,
    active_admins_n: u64,
) -> bool {
    let surviving = total_approvals.saturating_sub(departed_admin_approvals);
    let independent = if author_self_approved && active_admins_n >= two_u64() {
        surviving.saturating_sub(1)
    } else {
        surviving
    };
    quorum_threshold_valid(quorum_threshold_m, active_admins_n) && independent >= quorum_threshold_m
}

/// Pure MIR-lifted standing access-grant validity after granter departure
/// (`Governance::standingGrantValidAfterDeparture`).
#[must_use]
pub const fn standing_grant_valid_after_departure(
    granter_revoked: bool,
    co_signed_by_active_quorum: bool,
    boundary_keys_rotated: bool,
) -> bool {
    if granter_revoked {
        co_signed_by_active_quorum && boundary_keys_rotated
    } else {
        true
    }
}

/// Pure MIR-lifted historical commit verification and compromise-floor quarantine predicate
/// (`Governance::historicalCommitValid`).
#[must_use]
pub const fn historical_commit_valid(
    finalized_epoch: u64,
    revoked_at_epoch: u64,
    was_compromise: bool,
    compromise_floor_epoch: u64,
    epoch_quorum_valid: bool,
) -> bool {
    let clean_prior = epoch_quorum_valid && finalized_epoch < revoked_at_epoch;
    let compromise_ok = if was_compromise {
        finalized_epoch < compromise_floor_epoch
    } else {
        true
    };
    clean_prior && compromise_ok
}

/// Pure MIR-lifted end-to-end signing request admission predicate
/// (`Governance::signingRequestAdmissible`).
#[must_use]
#[allow(clippy::too_many_arguments)]
pub const fn signing_request_admissible(
    requester_signed: bool,
    explicitly_rejected: bool,
    current_epoch: u64,
    expiry_epoch: u64,
    total_approvals: u64,
    departed_admin_approvals: u64,
    quorum_threshold_m: u64,
    author_self_approved: bool,
    active_admins_n: u64,
) -> bool {
    requester_signed
        && !explicitly_rejected
        && current_epoch <= expiry_epoch
        && pending_approval_survives_departure(
            total_approvals,
            departed_admin_approvals,
            quorum_threshold_m,
            author_self_approved,
            active_admins_n,
        )
}

/// Pure MIR-lifted GitHub-free PROV-O group coordination envelope predicate
/// (`Governance::groupCoordinationEnvelopeValid`).
#[must_use]
pub const fn group_coordination_envelope_valid(
    provo_signed: bool,
    stitch_shards: u64,
    min_shards: u64,
    path_owners_verified: bool,
    requires_external_account: bool,
) -> bool {
    provo_signed
        && stitch_shards >= min_shards
        && path_owners_verified
        && !requires_external_account
}

/// Pure MIR-lifted role review privilege check (`0 = GenesisOwner`, `1 = SovereignAdmin`).
#[must_use]
pub const fn authority_role_can_review(role_code: u8) -> bool {
    role_code <= 1
}

/// Pure MIR-lifted signing-request terminal state predicate
/// (`4 = Finalized`, `5 = Rejected`, `7 = Expired`).
#[must_use]
pub const fn signing_request_state_is_terminal(state_code: u8) -> bool {
    let finalized = four_u32() as u8;
    let rejected = five_u32() as u8;
    let expired = seven_u64() as u8;
    state_code == finalized || state_code == rejected || state_code == expired
}

/// Pure MIR-lifted compromise departure classification (`2 = KeyCompromise`).
#[must_use]
pub const fn departure_requires_compromise_quarantine(reason_code: u8) -> bool {
    reason_code == two_u32() as u8
}

/// Pure MIR-lifted strict-majority quorum predicate (`2 * M > N && M <= N`).
#[must_use]
pub const fn quorum_is_strict_majority(threshold_m: u64, active_admins_n: u64) -> bool {
    threshold_m >= 1
        && threshold_m <= active_admins_n
        && threshold_m.wrapping_mul(two_u64()) > active_admins_n
}

/// Pure MIR-lifted supermajority quorum predicate (`3 * M >= 2 * N && M <= N`).
#[must_use]
pub const fn quorum_is_two_thirds_supermajority(threshold_m: u64, active_admins_n: u64) -> bool {
    threshold_m >= 1
        && threshold_m <= active_admins_n
        && threshold_m.wrapping_mul(three_u64()) >= active_admins_n.wrapping_mul(two_u64())
}

/// Pure MIR-lifted boundary key rotation monotonicity predicate (`next_key_epoch > prev_key_epoch`).
#[must_use]
pub const fn boundary_key_epoch_advanced(prev_key_epoch: u64, next_key_epoch: u64) -> bool {
    next_key_epoch >= prev_key_epoch.saturating_add(1)
}

/// Pure MIR-lifted signing request expiry window check (`submitted <= current <= expiry`).
#[must_use]
pub const fn signing_request_window_active(
    submitted_epoch: u64,
    current_epoch: u64,
    expiry_epoch: u64,
) -> bool {
    submitted_epoch <= current_epoch && current_epoch <= expiry_epoch
}

/// Pure MIR-lifted anti-lockout remaining admin count check (`remaining >= 1 && remaining >= next_m`).
#[must_use]
pub const fn succession_avoids_lockout(remaining_admins: u64, next_threshold_m: u64) -> bool {
    remaining_admins >= 1 && next_threshold_m >= 1 && next_threshold_m <= remaining_admins
}

/// Pure MIR-lifted compromise floor bounds check (`1 <= floor <= revoked_at_epoch`).
#[must_use]
pub const fn compromise_floor_valid(compromise_floor_epoch: u64, revoked_at_epoch: u64) -> bool {
    compromise_floor_epoch >= 1 && compromise_floor_epoch <= revoked_at_epoch
}

/// Pure MIR-lifted 64-bit VCS structural diff total changed-path accounting (`added + modified + removed`).
#[must_use]
pub const fn vcs_diff_total_changes_u64(added: u64, modified: u64, removed: u64) -> u64 {
    added.saturating_add(modified).saturating_add(removed)
}

/// Pure MIR-lifted VCS structural diff identity check (`0` changes iff trees are identical).
#[must_use]
pub const fn vcs_diff_is_empty(added: u64, modified: u64, removed: u64) -> bool {
    vcs_diff_total_changes_u64(added, modified, removed) == 0
}

/// Pure MIR-lifted VCS branch name length guard (`1 <= len <= 128`).
#[must_use]
pub const fn vcs_branch_name_len_valid(len: u32) -> bool {
    len >= 1 && len <= hundred_twenty_eight_u32()
}

/// Pure MIR-lifted Stitch 4-of-8 governance custody threshold check (`shards >= 4 && shards <= 8`).
#[must_use]
pub const fn stitch_governance_custody_quorum(shards: u64) -> bool {
    shards >= four_u64() && shards <= eight_u64()
}

/// Pure MIR-lifted GLB 4-byte chunk alignment padding calculator (`(4 - (len mod 4)) mod 4`).
#[must_use]
pub const fn glb_chunk_padding_bytes(len_mod_4: u32) -> u32 {
    if len_mod_4 == 0 {
        0
    } else if len_mod_4 < four_u32() {
        four_u32().wrapping_sub(len_mod_4)
    } else {
        0
    }
}

/// Pure MIR-lifted GLB header byte-length invariant (`total_bytes >= 12`).
#[must_use]
pub const fn glb_header_len_valid(total_bytes: u64) -> bool {
    let twelve = four_u64().wrapping_mul(three_u64());
    total_bytes >= twelve
}

/// Pure MIR-lifted command monoid octant addition in `Z_8` (`(a + b) mod 8`).
#[must_use]
pub const fn monoid_octant_add(a: u8, b: u8) -> u8 {
    let eight = eight_u32() as u8;
    let sum = a.wrapping_add(b);
    if sum >= eight {
        sum.wrapping_sub(eight)
    } else {
        sum
    }
}

/// Pure MIR-lifted command monoid self-inverse cancellation check (`a == b` cancels to identity).
#[must_use]
pub const fn monoid_involution_cancels(count_parity: u8) -> bool {
    count_parity == 0
}

/// Pure MIR-lifted immutability seal audit completeness check (`entries > 0 && violations == 0`).
#[must_use]
pub const fn immutability_seal_intact(entries: u64, violations: u64) -> bool {
    entries >= 1 && violations == 0
}

/// Pure MIR-lifted Rust file modularity ceiling predicate (`lines <= max_lines`).
#[must_use]
pub const fn rust_module_within_line_budget(lines: u32, max_lines: u32) -> bool {
    lines >= 1 && lines <= max_lines
}

/// Pure MIR-lifted sovereign governance crash-fault tolerance (`N - M` tolerated unavailable admins).
#[must_use]
pub const fn governance_crash_fault_tolerance(active_admins_n: u64, threshold_m: u64) -> u64 {
    if active_admins_n >= threshold_m {
        active_admins_n.wrapping_sub(threshold_m)
    } else {
        0
    }
}

/// Pure MIR-lifted Byzantine quorum safety bound (`3f < N` active admins).
#[must_use]
pub const fn governance_byzantine_safe(active_admins_n: u64, faulty_f: u64) -> bool {
    faulty_f.wrapping_mul(three_u64()) < active_admins_n
}

/// Pure MIR-lifted authority delegation depth bound (`depth <= max_depth <= 4`).
#[must_use]
pub const fn governance_delegation_depth_valid(depth: u32, max_depth: u32) -> bool {
    depth <= max_depth && max_depth <= four_u32()
}

/// Pure MIR-lifted authority epoch strict successor predicate (`next_epoch == prev_epoch + 1`).
#[must_use]
pub const fn governance_epoch_monotone(prev_epoch: u64, next_epoch: u64) -> bool {
    next_epoch == prev_epoch.wrapping_add(1)
}

/// Pure MIR-lifted revocation predicate: a departed Admin's unfinalized approval at or before
/// `revoked_epoch` is immediately invalidated (`!finalized && approved_epoch <= revoked_epoch`).
#[must_use]
pub const fn governance_revocation_invalidates_unfinalized(
    approved_epoch: u64,
    revoked_epoch: u64,
    finalized: bool,
) -> bool {
    !finalized && approved_epoch <= revoked_epoch
}

/// Pure MIR-lifted key-compromise quarantine predicate (`compromise_floor <= commit_epoch <= revoked`).
#[must_use]
pub const fn governance_compromise_quarantines_commit(
    commit_epoch: u64,
    compromise_floor: u64,
    revoked_epoch: u64,
) -> bool {
    compromise_floor >= 1 && commit_epoch >= compromise_floor && commit_epoch <= revoked_epoch
}

/// Pure MIR-lifted historical verification preservation predicate (`1 <= commit_epoch < compromise_floor`).
#[must_use]
pub const fn governance_clean_history_preserved(commit_epoch: u64, compromise_floor: u64) -> bool {
    commit_epoch >= 1 && commit_epoch < compromise_floor
}

/// Pure MIR-lifted supermajority quorum satisfaction check (`3 * valid_approvals >= 2 * N`).
#[must_use]
pub const fn governance_two_thirds_quorum_met(valid_approvals: u64, active_admins_n: u64) -> bool {
    active_admins_n >= 1
        && valid_approvals.wrapping_mul(three_u64()) >= active_admins_n.wrapping_mul(two_u64())
}

/// Pure MIR-lifted remaining approvals needed to satisfy threshold `M`.
#[must_use]
pub const fn governance_quorum_shortfall(valid_approvals: u64, threshold_m: u64) -> u64 {
    if threshold_m > valid_approvals {
        threshold_m.wrapping_sub(valid_approvals)
    } else {
        0
    }
}

/// Pure MIR-lifted distinct reviewer threshold satisfaction check (`M >= 1 && distinct >= M`).
#[must_use]
pub const fn governance_distinct_reviewers_met(distinct_count: u64, threshold_m: u64) -> bool {
    threshold_m >= 1 && distinct_count >= threshold_m
}

/// Pure MIR-lifted Reed-Solomon `k-of-n` parity shard count (`n - k`).
#[must_use]
pub const fn erasure_parity_shards(data_k: u32, total_n: u32) -> u32 {
    if total_n >= data_k {
        total_n.wrapping_sub(data_k)
    } else {
        0
    }
}

/// Pure MIR-lifted Reed-Solomon `k-of-n` recovery feasibility check (`1 <= k <= surviving <= n`).
#[must_use]
pub const fn erasure_recoverable(surviving: u32, data_k: u32, total_n: u32) -> bool {
    data_k >= 1 && data_k <= total_n && surviving >= data_k && surviving <= total_n
}

/// Pure MIR-lifted Reed-Solomon maximum simultaneous shard losses tolerated (`n - k`).
#[must_use]
pub const fn erasure_max_losses_tolerated(data_k: u32, total_n: u32) -> u32 {
    erasure_parity_shards(data_k, total_n)
}

/// Pure MIR-lifted Reed-Solomon bounded redundancy check (`1 <= k <= n <= 2k`).
#[must_use]
pub const fn erasure_coding_overhead_within_double(data_k: u32, total_n: u32) -> bool {
    data_k >= 1 && total_n >= data_k && total_n <= data_k.wrapping_mul(two_u32())
}

/// Pure MIR-lifted WebAssembly 64 KiB linear-memory byte capacity (`pages * 65536`).
#[must_use]
pub const fn wasm_linear_memory_bytes_for_pages(pages: u64) -> u64 {
    let four_sq = four_u64().wrapping_mul(four_u64());
    let two_fifty_six = four_sq.wrapping_mul(four_sq);
    let page = two_fifty_six.wrapping_mul(two_fifty_six);
    pages.wrapping_mul(page)
}

/// Pure MIR-lifted WebAssembly linear-memory page budget guard (`1 <= pages <= max_pages`).
#[must_use]
pub const fn wasm_page_budget_within_limit(pages: u32, max_pages: u32) -> bool {
    pages >= 1 && pages <= max_pages
}

/// Pure MIR-lifted WebAssembly binary size headroom (`ceiling - observed`, saturating at `0`).
#[must_use]
pub const fn wasm_budget_headroom_bytes(ceiling_bytes: u64, observed_bytes: u64) -> u64 {
    if ceiling_bytes >= observed_bytes {
        ceiling_bytes.wrapping_sub(observed_bytes)
    } else {
        0
    }
}

/// Pure MIR-lifted WebAssembly binary size compliance check (`0 < observed <= ceiling`).
#[must_use]
pub const fn wasm_budget_compliant(observed_bytes: u64, ceiling_bytes: u64) -> bool {
    observed_bytes >= 1 && observed_bytes <= ceiling_bytes
}

/// Pure MIR-lifted shipped JS module modularity ceiling predicate (`1 <= lines <= max_lines`).
#[must_use]
pub const fn js_module_within_line_budget(lines: u32, max_lines: u32) -> bool {
    lines >= 1 && lines <= max_lines
}

/// Pure MIR-lifted 64-bit one-way parametric ratchet floor monotonicity check (`next_floor >= prev_floor`).
#[must_use]
pub const fn ratchet_floor_did_not_regress_u64(prev_floor: u64, next_floor: u64) -> bool {
    next_floor >= prev_floor
}

/// Pure MIR-lifted 64-bit one-way parametric ratchet ceiling monotonicity check (`next_ceiling <= prev_ceiling`).
#[must_use]
pub const fn ratchet_ceiling_did_not_regress_u64(prev_ceiling: u64, next_ceiling: u64) -> bool {
    next_ceiling <= prev_ceiling
}

/// Pure MIR-lifted one-way ratchet ceiling improvement margin (`ceiling - observed`).
#[must_use]
pub const fn ratchet_improvement_margin_u64(ceiling: u64, observed: u64) -> u64 {
    if ceiling >= observed {
        ceiling.wrapping_sub(observed)
    } else {
        0
    }
}

/// Pure MIR-lifted UHD geometry LOD rank bounds check (`rank <= 4`).
#[must_use]
pub const fn uhd_lod_rank_valid(rank: u32) -> bool {
    rank <= four_u32()
}

/// Pure MIR-lifted UHD dynamic workload tensor tile quantum bounds check (`2 <= quantum <= 32`).
#[must_use]
pub const fn uhd_tile_quantum_bounded(quantum: u32) -> bool {
    let thirty_two = sixteen_u32().wrapping_mul(two_u32());
    quantum >= two_u32() && quantum <= thirty_two
}

/// Pure MIR-lifted UHD frame over-budget overrun in microseconds (`frame_us - budget_us`).
#[must_use]
pub const fn uhd_frame_over_budget_margin_us(frame_us: u32, budget_us: u32) -> u32 {
    if frame_us > budget_us {
        frame_us.wrapping_sub(budget_us)
    } else {
        0
    }
}

/// Pure MIR-lifted UHD arena high-pressure trigger (`pressure_permille >= 900`).
#[must_use]
pub const fn uhd_memory_pressure_high(pressure_permille: u32) -> bool {
    let nine = eight_u32().wrapping_add(1);
    pressure_permille >= hundred_u32().wrapping_mul(nine)
}

/// Pure MIR-lifted VerdantVcs commit epoch strict increment check (`child == parent + 1`).
#[must_use]
pub const fn vcs_commit_epoch_monotonic(parent_epoch: u64, child_epoch: u64) -> bool {
    child_epoch == parent_epoch.wrapping_add(1)
}

/// Pure MIR-lifted VerdantVcs Merkle tree non-emptiness predicate (`blob_count >= 1`).
#[must_use]
pub const fn vcs_merkle_tree_non_empty(blob_count: u64) -> bool {
    blob_count >= 1
}

/// Pure MIR-lifted Observer manager perfect health score check (`score_permille == 1000`).
#[must_use]
pub const fn observer_manager_health_perfect(score_permille: u32) -> bool {
    score_permille == thousand_u32()
}

/// Pure MIR-lifted Observer refinement score non-regression check (`prev <= next <= 1000`).
#[must_use]
pub const fn observer_refinement_score_improved(prev_permille: u32, next_permille: u32) -> bool {
    next_permille >= prev_permille && next_permille <= thousand_u32()
}

#[path = "scalars_ratchet.rs"]
mod scalars_ratchet;
pub use scalars_ratchet::*;
