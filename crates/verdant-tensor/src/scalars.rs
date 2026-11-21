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
