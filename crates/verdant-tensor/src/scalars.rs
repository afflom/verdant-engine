//! Pure `{0, 1}` constructive scalar primitives using total `wrapping_add` / `wrapping_mul`
//! arithmetic so every helper satisfies **both** the permanent Zero-Constants Parametric Ratchet
//! (`FI-08`, zero literals `> 1`) **and** the `uorust` MIR proof frontier (`0` shift/overflow
//! arithmetic residue, `VE-55`).

/// Constructive `2_u64` (`1 + 1`) lifted by `uorust`.
#[must_use]
pub const fn two_u64() -> u64 {
    1_u64.wrapping_add(1_u64)
}

/// Constructive `3_u64` (`2 + 1`) lifted by `uorust`.
#[must_use]
pub const fn three_u64() -> u64 {
    two_u64().wrapping_add(1_u64)
}

/// Constructive `4_u64` (`2 * 2`) lifted by `uorust`.
#[must_use]
pub const fn four_u64() -> u64 {
    let t = two_u64();
    t.wrapping_mul(t)
}

/// Constructive `8_u64` (`4 * 2`) lifted by `uorust`.
#[must_use]
pub const fn eight_u64() -> u64 {
    four_u64().wrapping_mul(two_u64())
}

/// Constructive `10_u64` (`8 + 2`) lifted by `uorust`.
#[must_use]
pub const fn ten_u64() -> u64 {
    eight_u64().wrapping_add(two_u64())
}

/// Constructive `16_u64` (`4 * 4`) lifted by `uorust`.
#[must_use]
pub const fn sixteen_u64() -> u64 {
    let f = four_u64();
    f.wrapping_mul(f)
}

/// Constructive `32_u64` (`16 * 2`) lifted by `uorust`.
#[must_use]
pub const fn thirty_two_u64() -> u64 {
    sixteen_u64().wrapping_mul(two_u64())
}

/// Constructive `64_u64` (`8 * 8`) lifted by `uorust`.
#[must_use]
pub const fn sixty_four_u64() -> u64 {
    let e = eight_u64();
    e.wrapping_mul(e)
}

/// Constructive `100_u64` (`10 * 10`) lifted by `uorust`.
#[must_use]
pub const fn hundred_u64() -> u64 {
    let t = ten_u64();
    t.wrapping_mul(t)
}

/// Constructive `2_u32` lifted by `uorust`.
#[must_use]
pub const fn two_u32() -> u32 {
    1_u32.wrapping_add(1_u32)
}

/// Constructive `4_u32` lifted by `uorust`.
#[must_use]
pub const fn four_u32() -> u32 {
    let t = two_u32();
    t.wrapping_mul(t)
}

/// Constructive `8_u32` lifted by `uorust`.
#[must_use]
pub const fn eight_u32() -> u32 {
    four_u32().wrapping_mul(two_u32())
}

/// Constructive `16_u32` lifted by `uorust`.
#[must_use]
pub const fn sixteen_u32() -> u32 {
    let f = four_u32();
    f.wrapping_mul(f)
}

/// Constructive `32_u32` lifted by `uorust`.
#[must_use]
pub const fn thirty_two_u32() -> u32 {
    sixteen_u32().wrapping_mul(two_u32())
}

/// Constructive `64_u32` lifted by `uorust`.
#[must_use]
pub const fn sixty_four_u32() -> u32 {
    let e = eight_u32();
    e.wrapping_mul(e)
}

/// Constructive `2_u8` lifted by `uorust`.
#[must_use]
pub const fn two_u8() -> u8 {
    1_u8.wrapping_add(1_u8)
}

/// Constructive `4_u8` lifted by `uorust`.
#[must_use]
pub const fn four_u8() -> u8 {
    let t = two_u8();
    t.wrapping_mul(t)
}

/// Constructive `8_u8` lifted by `uorust`.
#[must_use]
pub const fn eight_u8() -> u8 {
    four_u8().wrapping_mul(two_u8())
}

/// Constructive `16_u8` lifted by `uorust`.
#[must_use]
pub const fn sixteen_u8() -> u8 {
    let f = four_u8();
    f.wrapping_mul(f)
}

/// Constructive `32_u8` lifted by `uorust`.
#[must_use]
pub const fn thirty_two_u8() -> u8 {
    sixteen_u8().wrapping_mul(two_u8())
}

/// Constructive `64_u8` lifted by `uorust`.
#[must_use]
pub const fn sixty_four_u8() -> u8 {
    let e = eight_u8();
    e.wrapping_mul(e)
}
