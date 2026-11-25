#![no_std]
#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
}

pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }

pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}

pub fn f0(v0: u64, v1: u64) -> u64 {
    let a1 = v0;
    let a2 = 3u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 1u64;
        if nat_lt(a3, a4) {
            f1(v1)
        } else {
            let a5 = v0;
            let a6 = 2u64;
            if nat_lt(a5, a6) {
                f2(v1)
            } else {
                f3(v1)
            }
        }
    } else {
        let a7 = v0;
        let a8 = 8u64;
        if nat_lt(a7, a8) {
            let a9 = v0;
            let a10 = 6u64;
            if nat_lt(a9, a10) {
                f4(v1, {
                    let a11 = v0;
                    let a12 = 5u64;
                    nat_eq(a11, a12)
                })
            } else {
                f5(v1, {
                    let a13 = v0;
                    let a14 = 7u64;
                    nat_eq(a13, a14)
                })
            }
        } else {
            let a15 = v0;
            let a16 = 8u64;
            if nat_eq(a15, a16) {
                f6(v1)
            } else {
                f7(v1)
            }
        }
    }
}

pub fn f1(v0: u64) -> u64 {
    let a17 = v0;
    let a18 = 1u64;
    if nat_lt(a17, a18) {
        65538u64
    } else {
        0u64
    }
}

pub fn f2(v0: u64) -> u64 {
    let a19 = v0;
    let a20 = 3u64;
    if nat_lt(a19, a20) {
        let a21 = v0;
        let a22 = 1u64;
        if nat_lt(a21, a22) {
            65538u64
        } else {
            let a23 = v0;
            let a24 = 2u64;
            if nat_lt(a23, a24) {
                67663u64
            } else {
                65641u64
            }
        }
    } else {
        let a25 = v0;
        let a26 = 5u64;
        if nat_lt(a25, a26) {
            let a27 = v0;
            let a28 = 4u64;
            if nat_lt(a27, a28) {
                69776u64
            } else {
                71860u64
            }
        } else {
            let a29 = v0;
            let a30 = 6u64;
            if nat_lt(a29, a30) {
                73939u64
            } else {
                0u64
            }
        }
    }
}

pub fn f3(v0: u64) -> u64 {
    let a31 = v0;
    let a32 = 4u64;
    if nat_lt(a31, a32) {
        let a33 = v0;
        let a34 = 2u64;
        if nat_lt(a33, a34) {
            let a35 = v0;
            let a36 = 1u64;
            if nat_lt(a35, a36) {
                75824u64
            } else {
                73811u64
            }
        } else {
            let a37 = v0;
            let a38 = 3u64;
            if nat_lt(a37, a38) {
                75889u64
            } else {
                65672u64
            }
        }
    } else {
        let a39 = v0;
        let a40 = 6u64;
        if nat_lt(a39, a40) {
            let a41 = v0;
            let a42 = 5u64;
            if nat_lt(a41, a42) {
                77999u64
            } else {
                65538u64
            }
        } else {
            let a43 = v0;
            let a44 = 7u64;
            if nat_lt(a43, a44) {
                12513u64
            } else {
                0u64
            }
        }
    }
}

pub fn f4(v0: u64, v1: bool) -> u64 {
    let a45 = v0;
    let a46 = 6u64;
    if nat_lt(a45, a46) {
        let a47 = v0;
        let a48 = 3u64;
        if nat_lt(a47, a48) {
            let a49 = v0;
            let a50 = 1u64;
            if nat_lt(a49, a50) {
                15u64
            } else {
                let a51 = v0;
                let a52 = 2u64;
                if nat_lt(a51, a52) {
                    if v1 {
                        96272u64
                    } else {
                        65552u64
                    }
                } else {
                    4207u64
                }
            }
        } else {
            let a53 = v0;
            let a54 = 4u64;
            if nat_lt(a53, a54) {
                98383u64
            } else {
                let a55 = v0;
                let a56 = 5u64;
                if nat_lt(a55, a56) {
                    11809u64
                } else {
                    4239u64
                }
            }
        }
    } else {
        let a57 = v0;
        let a58 = 9u64;
        if nat_lt(a57, a58) {
            let a59 = v0;
            let a60 = 7u64;
            if nat_lt(a59, a60) {
                102543u64
            } else {
                let a61 = v0;
                let a62 = 8u64;
                if nat_lt(a61, a62) {
                    16481u64
                } else {
                    12527u64
                }
            }
        } else {
            let a63 = v0;
            let a64 = 10u64;
            if nat_lt(a63, a64) {
                20769u64
            } else {
                let a65 = v0;
                let a66 = 11u64;
                if nat_lt(a65, a66) {
                    65891u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: bool) -> u64 {
    let a67 = v0;
    let a68 = 6u64;
    if nat_lt(a67, a68) {
        let a69 = v0;
        let a70 = 3u64;
        if nat_lt(a69, a70) {
            let a71 = v0;
            let a72 = 1u64;
            if nat_lt(a71, a72) {
                15u64
            } else {
                let a73 = v0;
                let a74 = 2u64;
                if nat_lt(a73, a74) {
                    4175u64
                } else {
                    if v1 {
                        106575u64
                    } else {
                        104527u64
                    }
                }
            }
        } else {
            let a75 = v0;
            let a76 = 4u64;
            if nat_lt(a75, a76) {
                if v1 {
                    9921u64
                } else {
                    9889u64
                }
            } else {
                let a77 = v0;
                let a78 = 5u64;
                if nat_lt(a77, a78) {
                    if v1 {
                        114767u64
                    } else {
                        112719u64
                    }
                } else {
                    if v1 {
                        14145u64
                    } else {
                        14113u64
                    }
                }
            }
        }
    } else {
        let a79 = v0;
        let a80 = 9u64;
        if nat_lt(a79, a80) {
            let a81 = v0;
            let a82 = 7u64;
            if nat_lt(a81, a82) {
                4207u64
            } else {
                let a83 = v0;
                let a84 = 8u64;
                if nat_lt(a83, a84) {
                    if v1 {
                        122991u64
                    } else {
                        120943u64
                    }
                } else {
                    18657u64
                }
            }
        } else {
            let a85 = v0;
            let a86 = 11u64;
            if nat_lt(a85, a86) {
                let a87 = v0;
                let a88 = 10u64;
                if nat_lt(a87, a88) {
                    10511u64
                } else {
                    22849u64
                }
            } else {
                let a89 = v0;
                let a90 = 12u64;
                if nat_lt(a89, a90) {
                    65923u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> u64 {
    let a91 = v0;
    let a92 = 10u64;
    if nat_lt(a91, a92) {
        let a93 = v0;
        let a94 = 5u64;
        if nat_lt(a93, a94) {
            let a95 = v0;
            let a96 = 2u64;
            if nat_lt(a95, a96) {
                let a97 = v0;
                let a98 = 1u64;
                if nat_lt(a97, a98) {
                    79892u64
                } else {
                    82003u64
                }
            } else {
                let a99 = v0;
                let a100 = 3u64;
                if nat_lt(a99, a100) {
                    69652u64
                } else {
                    let a101 = v0;
                    let a102 = 4u64;
                    if nat_lt(a101, a102) {
                        84115u64
                    } else {
                        65718u64
                    }
                }
            }
        } else {
            let a103 = v0;
            let a104 = 7u64;
            if nat_lt(a103, a104) {
                let a105 = v0;
                let a106 = 6u64;
                if nat_lt(a105, a106) {
                    69651u64
                } else {
                    69872u64
                }
            } else {
                let a107 = v0;
                let a108 = 8u64;
                if nat_lt(a107, a108) {
                    12562u64
                } else {
                    let a109 = v0;
                    let a110 = 9u64;
                    if nat_lt(a109, a110) {
                        65653u64
                    } else {
                        86128u64
                    }
                }
            }
        }
    } else {
        let a111 = v0;
        let a112 = 15u64;
        if nat_lt(a111, a112) {
            let a113 = v0;
            let a114 = 12u64;
            if nat_lt(a113, a114) {
                let a115 = v0;
                let a116 = 11u64;
                if nat_lt(a115, a116) {
                    65909u64
                } else {
                    84080u64
                }
            } else {
                let a117 = v0;
                let a118 = 13u64;
                if nat_lt(a117, a118) {
                    65973u64
                } else {
                    let a119 = v0;
                    let a120 = 14u64;
                    if nat_lt(a119, a120) {
                        20786u64
                    } else {
                        24882u64
                    }
                }
            }
        } else {
            let a121 = v0;
            let a122 = 18u64;
            if nat_lt(a121, a122) {
                let a123 = v0;
                let a124 = 16u64;
                if nat_lt(a123, a124) {
                    28978u64
                } else {
                    let a125 = v0;
                    let a126 = 17u64;
                    if nat_lt(a125, a126) {
                        88562u64
                    } else {
                        88628u64
                    }
                }
            } else {
                let a127 = v0;
                let a128 = 19u64;
                if nat_lt(a127, a128) {
                    39504u64
                } else {
                    let a129 = v0;
                    let a130 = 20u64;
                    if nat_lt(a129, a130) {
                        33424u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> u64 {
    let a131 = v0;
    let a132 = 3u64;
    if nat_lt(a131, a132) {
        let a133 = v0;
        let a134 = 1u64;
        if nat_lt(a133, a134) {
            90128u64
        } else {
            let a135 = v0;
            let a136 = 2u64;
            if nat_lt(a135, a136) {
                92244u64
            } else {
                92274u64
            }
        }
    } else {
        let a137 = v0;
        let a138 = 5u64;
        if nat_lt(a137, a138) {
            let a139 = v0;
            let a140 = 4u64;
            if nat_lt(a139, a140) {
                8209u64
            } else {
                65704u64
            }
        } else {
            let a141 = v0;
            let a142 = 6u64;
            if nat_lt(a141, a142) {
                94415u64
            } else {
                0u64
            }
        }
    }
}
