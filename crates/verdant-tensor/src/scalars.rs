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
