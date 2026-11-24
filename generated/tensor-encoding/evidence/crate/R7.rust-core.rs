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

pub fn f0(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a1 = v0;
    let a2 = 48u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 32u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 16u64;
            if nat_lt(a5, a6) {
                let a7 = v0;
                let a8 = 6u64;
                if nat_lt(a7, a8) {
                    let a9 = v0;
                    let a10 = 3u64;
                    if nat_lt(a9, a10) {
                        let a11 = v0;
                        let a12 = 1u64;
                        if nat_lt(a11, a12) {
                            let a13 = v0;
                            let a14 = 0u64;
                            if nat_eq(a13, a14) {
                                Ok(f1(v1, v2, v3, v4))
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a15 = v0;
                            let a16 = 2u64;
                            if nat_lt(a15, a16) {
                                let a17 = v0;
                                let a18 = 1u64;
                                let _ = nat_eq(a17, a18);
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            } else {
                                let a19 = v0;
                                let a20 = 2u64;
                                if nat_eq(a19, a20) {
                                    Ok(f2(v2, v3))
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            }
                        }
                    } else {
                        let a21 = v0;
                        let a22 = 4u64;
                        if nat_lt(a21, a22) {
                            let a23 = v0;
                            let a24 = 3u64;
                            if nat_eq(a23, a24) {
                                f3(v2, v3, v4)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a25 = v0;
                            let a26 = 5u64;
                            if nat_lt(a25, a26) {
                                let a27 = v0;
                                let a28 = 4u64;
                                if nat_eq(a27, a28) {
                                    f4(v1, v2, v3)
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            } else {
                                let a29 = v0;
                                let a30 = 5u64;
                                if nat_eq(a29, a30) {
                                    f5(v1, v2)
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            }
                        }
                    }
                } else {
                    let a31 = v0;
                    let a32 = 9u64;
                    if nat_lt(a31, a32) {
                        let a33 = v0;
                        let a34 = 7u64;
                        if nat_lt(a33, a34) {
                            let a35 = v0;
                            let a36 = 6u64;
                            if nat_eq(a35, a36) {
                                Ok(f6(v2))
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a37 = v0;
                            let a38 = 8u64;
                            if nat_lt(a37, a38) {
                                let a39 = v0;
                                let a40 = 7u64;
                                if nat_eq(a39, a40) {
                                    f7(v1, v2, v3, v4)
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            } else {
                                let a41 = v0;
                                let a42 = 8u64;
                                if nat_eq(a41, a42) {
                                    f8(v1, v2)
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            }
                        }
                    } else {
                        let a43 = v0;
                        let a44 = 10u64;
                        if nat_lt(a43, a44) {
                            let a45 = v0;
                            let a46 = 9u64;
                            if nat_eq(a45, a46) {
                                f9(v1, v2, v3)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a47 = v0;
                            let a48 = 11u64;
                            if nat_lt(a47, a48) {
                                let a49 = v0;
                                let a50 = 10u64;
                                if nat_eq(a49, a50) {
                                    let v5: u64 = {
                                        let a51 = v2;
                                        let a52 = 4294967296u64;
                                        let a53 = 0u64;
                                        nat_quot(a51, a52, a53)
                                    };
                                    let v6: u64 = {
                                        let a54 = v2;
                                        let a55 = 4294967296u64;
                                        let a56 = 0u64;
                                        nat_rem(a54, a55, a56)
                                    };
                                    let a57 = v5;
                                    let a58 = 2u64;
                                    if nat_eq(a57, a58) {
                                        Ok(f10(v5, {
                                            let a59 = v6;
                                            let a60 = 256u64;
                                            let a61 = 0u64;
                                            nat_quot(a59, a60, a61)
                                        }, {
                                            let a62 = v6;
                                            let a63 = 256u64;
                                            let a64 = 0u64;
                                            nat_rem(a62, a63, a64)
                                        }))
                                    } else {
                                        let a65 = v5;
                                        let a66 = 2u64;
                                        if nat_lt(a65, a66) {
                                            Ok(f10(v5, v6, 0u64))
                                        } else {
                                            Ok(f10(v5, 0u64, v6))
                                        }
                                    }
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            } else {
                                let a67 = v0;
                                let a68 = 11u64;
                                if nat_eq(a67, a68) {
                                    Ok(f11(f12(v2, v3, v4)?))
                                } else {
                                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                                }
                            }
                        }
                    }
                }
            } else {
                f13({
                    let a69 = v0;
                    let a70 = 16u64;
                    nat_sub(a69, a70)
                }, v1, v2, v3, v4)
            }
        } else {
            f14({
                let a71 = v0;
                let a72 = 32u64;
                nat_sub(a71, a72)
            }, v2, v3, v4)
        }
    } else {
        f15({
            let a73 = v0;
            let a74 = 48u64;
            nat_sub(a73, a74)
        }, v1, v2, v3, v4)
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    let a75 = v1;
    let a76 = 256u64;
    if if if nat_eq(a75, a76) {
        let a77 = v2;
        let a78 = 0u64;
        if nat_eq(a77, a78) {
            true
        } else {
            let a79 = v2;
            let a80 = 256u64;
            nat_eq(a79, a80)
        }
    } else {
        false
    } {
        let a81 = v3;
        let a82 = if v0 {
            98u64
        } else {
            110u64
        };
        nat_eq(a81, a82)
    } else {
        false
    } {
        f16(8u64, 32u64, if v0 {
            15u64
        } else {
            9u64
        }, if v0 {
            841731191u64
        } else {
            0u64
        }, if v0 {
            19241u64
        } else {
            25245u64
        })
    } else {
        (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
    }
}

pub fn f2(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f17({
        let a83 = v0;
        let a84 = 8388608u64;
        let a85 = 0u64;
        nat_rem(a83, a84, a85)
    }, v1, 4194304u64))
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11(f18(v0, v2, 1u64, 8u64, {
        let a86 = v1;
        let a87 = 0u64;
        if nat_eq(a86, a87) {
            1u64
        } else {
            let a88 = v1;
            let a89 = 1u64;
            if nat_eq(a88, a89) {
                8u64
            } else {
                let a90 = v1;
                let a91 = 2u64;
                if nat_eq(a90, a91) {
                    64u64
                } else {
                    let a92 = v1;
                    let a93 = 3u64;
                    if nat_eq(a92, a93) {
                        512u64
                    } else {
                        0u64
                    }
                }
            }
        }
    })?))
}

pub fn f4(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f19({
        let a94 = if v0 {
            0u64
        } else {
            4097u64
        };
        let a95 = v1;
        nat_add(a94, a95)?
    }, {
        let a98 = if v0 {
            15496u64
        } else {
            16520u64
        };
        let a99 = {
            let a96 = 4u64;
            let a97 = v2;
            nat_mul(a96, a97)?
        };
        nat_add(a98, a99)?
    }))
}

pub fn f5(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11({
        let a105 = 1u64;
        let a106 = {
            let a103 = 2u64;
            let a104 = {
                let a100 = v1;
                let a101 = if v0 {
                    8u64
                } else {
                    2u64
                };
                let a102 = 0u64;
                nat_quot(a100, a101, a102)
            };
            nat_mul(a103, a104)?
        };
        nat_add(a105, a106)?
    }))
}

pub fn f6(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f20(v0))
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a107 = v2;
    let a108 = 0u64;
    if nat_eq(a107, a108) {
        Ok(f10({
            let a109 = 2u64;
            let a110 = v1;
            nat_add(a109, a110)?
        }, 0u64, {
            let a111 = v3;
            let a112 = 256u64;
            let a113 = 0u64;
            nat_rem(a111, a112, a113)
        }))
    } else {
        if v0 {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        } else {
            Ok(f10({
                let a117 = 66u64;
                let a118 = {
                    let a114 = v1;
                    let a115 = 8u64;
                    let a116 = 0u64;
                    nat_quot(a114, a115, a116)
                };
                nat_add(a117, a118)?
            }, {
                let a119 = v1;
                let a120 = 8u64;
                let a121 = 0u64;
                nat_rem(a119, a120, a121)
            }, {
                let a125 = {
                    let a122 = v3;
                    let a123 = 256u64;
                    let a124 = 0u64;
                    nat_quot(a122, a123, a124)
                };
                let a126 = 2u64;
                let a127 = 0u64;
                nat_rem(a125, a126, a127)
            }))
        }
    }
}

pub fn f8(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f19({
            let a130 = 69u64;
            let a131 = {
                let a128 = 4u64;
                let a129 = v1;
                nat_mul(a128, a129)?
            };
            nat_add(a130, a131)?
        }, 4u64))
    } else {
        Ok(f19({
            let a135 = 106u64;
            let a136 = {
                let a132 = v1;
                let a133 = 2u64;
                let a134 = 0u64;
                nat_quot(a132, a133, a134)
            };
            nat_add(a135, a136)?
        }, {
            let a140 = 4u64;
            let a141 = {
                let a137 = v1;
                let a138 = 2u64;
                let a139 = 0u64;
                nat_rem(a137, a138, a139)
            };
            nat_mul(a140, a141)?
        }))
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f19({
            let a144 = 66u64;
            let a145 = {
                let a142 = 4u64;
                let a143 = v1;
                nat_mul(a142, a143)?
            };
            nat_add(a144, a145)?
        }, {
            let a146 = 7u64;
            let a147 = v2;
            nat_mul(a146, a147)?
        }))
    } else {
        Ok(f19({
            let a152 = {
                let a150 = 74u64;
                let a151 = {
                    let a148 = 4u64;
                    let a149 = v1;
                    nat_mul(a148, a149)?
                };
                nat_add(a150, a151)?
            };
            let a153 = v2;
            nat_add(a152, a153)?
        }, 0u64))
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (8u64, (v0, (v1, (v2, (0u64, 0u64)))))
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (1u64, (v0, (0u64, (0u64, (0u64, 0u64)))))
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a154 = v0;
    let a155 = 3u64;
    if nat_lt(a154, a155) {
        let a156 = v0;
        let a157 = 1u64;
        if nat_lt(a156, a157) {
            let a158 = v0;
            let a159 = 0u64;
            if nat_eq(a158, a159) {
                let a160 = v1;
                let a161 = v2;
                nat_add(a160, a161)
            } else {
                Ok(0u64)
            }
        } else {
            let a162 = v0;
            let a163 = 2u64;
            if nat_lt(a162, a163) {
                let a164 = v0;
                let a165 = 1u64;
                if nat_eq(a164, a165) {
                    let a166 = v1;
                    let a167 = v2;
                    Ok(nat_sub(a166, a167))
                } else {
                    Ok(0u64)
                }
            } else {
                let a168 = v0;
                let a169 = 2u64;
                if nat_eq(a168, a169) {
                    let a170 = v1;
                    let a171 = v2;
                    nat_mul(a170, a171)
                } else {
                    Ok(0u64)
                }
            }
        }
    } else {
        let a172 = v0;
        let a173 = 5u64;
        if nat_lt(a172, a173) {
            let a174 = v0;
            let a175 = 4u64;
            if nat_lt(a174, a175) {
                let a176 = v0;
                let a177 = 3u64;
                if nat_eq(a176, a177) {
                    let a178 = v1;
                    let a179 = v2;
                    let a180 = 0u64;
                    Ok(nat_quot(a178, a179, a180))
                } else {
                    Ok(0u64)
                }
            } else {
                let a181 = v0;
                let a182 = 4u64;
                if nat_eq(a181, a182) {
                    let a183 = v1;
                    let a184 = v2;
                    let a185 = 0u64;
                    Ok(nat_rem(a183, a184, a185))
                } else {
                    Ok(0u64)
                }
            }
        } else {
            let a186 = v0;
            let a187 = 6u64;
            if nat_lt(a186, a187) {
                let a188 = v0;
                let a189 = 5u64;
                if nat_eq(a188, a189) {
                    let a190 = v1;
                    let a191 = v2;
                    if nat_eq(a190, a191) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a192 = v0;
                let a193 = 6u64;
                if nat_eq(a192, a193) {
                    let a194 = v1;
                    let a195 = v2;
                    if nat_lt(a194, a195) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f13(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a196 = v0;
    let a197 = 4u64;
    if nat_lt(a196, a197) {
        let a198 = v0;
        let a199 = 2u64;
        if nat_lt(a198, a199) {
            let a200 = v0;
            let a201 = 1u64;
            if nat_lt(a200, a201) {
                let a202 = v0;
                let a203 = 0u64;
                if nat_eq(a202, a203) {
                    if v1 {
                        let a204 = v2;
                        let a205 = 256u64;
                        if if if nat_eq(a204, a205) {
                            let a206 = v3;
                            let a207 = 256u64;
                            nat_eq(a206, a207)
                        } else {
                            false
                        } {
                            let a208 = v4;
                            let a209 = 74u64;
                            nat_eq(a208, a209)
                        } else {
                            false
                        } {
                            Ok(f16(16u64, 16u64, 9u64, 646978941u64, 182233u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a210 = v2;
                        let a211 = 256u64;
                        if if if nat_eq(a210, a211) {
                            let a212 = v3;
                            let a213 = 0u64;
                            if nat_eq(a212, a213) {
                                true
                            } else {
                                let a214 = v3;
                                let a215 = 256u64;
                                nat_eq(a214, a215)
                            }
                        } else {
                            false
                        } {
                            let a216 = v4;
                            let a217 = 82u64;
                            nat_eq(a216, a217)
                        } else {
                            false
                        } {
                            Ok(f16(16u64, 16u64, 9u64, 841731191u64, 45740u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a218 = v0;
                let a219 = 1u64;
                let _ = nat_eq(a218, a219);
                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
            }
        } else {
            let a220 = v0;
            let a221 = 3u64;
            if nat_lt(a220, a221) {
                let a222 = v0;
                let a223 = 2u64;
                if nat_eq(a222, a223) {
                    Ok(f2(v2, v3))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a224 = v0;
                let a225 = 3u64;
                if nat_eq(a224, a225) {
                    Ok(f11({
                        let a258 = v2;
                        let a259 = {
                            let a256 = v4;
                            let a257 = {
                                let a226 = v3;
                                let a227 = 4u64;
                                if nat_lt(a226, a227) {
                                    let a228 = v3;
                                    let a229 = 2u64;
                                    if nat_lt(a228, a229) {
                                        let a230 = v3;
                                        let a231 = 1u64;
                                        if nat_lt(a230, a231) {
                                            let a232 = v3;
                                            let a233 = 0u64;
                                            if nat_eq(a232, a233) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a234 = v3;
                                            let a235 = 1u64;
                                            if nat_eq(a234, a235) {
                                                3u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a236 = v3;
                                        let a237 = 3u64;
                                        if nat_lt(a236, a237) {
                                            let a238 = v3;
                                            let a239 = 2u64;
                                            if nat_eq(a238, a239) {
                                                9u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a240 = v3;
                                            let a241 = 3u64;
                                            if nat_eq(a240, a241) {
                                                27u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                } else {
                                    let a242 = v3;
                                    let a243 = 6u64;
                                    if nat_lt(a242, a243) {
                                        let a244 = v3;
                                        let a245 = 5u64;
                                        if nat_lt(a244, a245) {
                                            let a246 = v3;
                                            let a247 = 4u64;
                                            if nat_eq(a246, a247) {
                                                81u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a248 = v3;
                                            let a249 = 5u64;
                                            if nat_eq(a248, a249) {
                                                243u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a250 = v3;
                                        let a251 = 7u64;
                                        if nat_lt(a250, a251) {
                                            let a252 = v3;
                                            let a253 = 6u64;
                                            if nat_eq(a252, a253) {
                                                729u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a254 = v3;
                                            let a255 = 7u64;
                                            if nat_eq(a254, a255) {
                                                2187u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                }
                            };
                            nat_mul(a256, a257)?
                        };
                        nat_add(a258, a259)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        }
    } else {
        let a260 = v0;
        let a261 = 6u64;
        if nat_lt(a260, a261) {
            let a262 = v0;
            let a263 = 5u64;
            if nat_lt(a262, a263) {
                let a264 = v0;
                let a265 = 4u64;
                if nat_eq(a264, a265) {
                    Ok(f19(v2, {
                        let a268 = if v1 {
                            3208u64
                        } else {
                            7304u64
                        };
                        let a269 = {
                            let a266 = v3;
                            let a267 = 8u64;
                            nat_mul(a266, a267)?
                        };
                        nat_add(a268, a269)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a270 = v0;
                let a271 = 5u64;
                if nat_eq(a270, a271) {
                    Ok(f11({
                        let a277 = 1u64;
                        let a278 = {
                            let a275 = 2u64;
                            let a276 = {
                                let a272 = v2;
                                let a273 = 17u64;
                                let a274 = 0u64;
                                nat_quot(a272, a273, a274)
                            };
                            nat_mul(a275, a276)?
                        };
                        nat_add(a277, a278)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        } else {
            let a279 = v0;
            let a280 = 7u64;
            if nat_lt(a279, a280) {
                let a281 = v0;
                let a282 = 6u64;
                if nat_eq(a281, a282) {
                    Ok(f6(v2))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a283 = v0;
                let a284 = 8u64;
                if nat_lt(a283, a284) {
                    let a285 = v0;
                    let a286 = 7u64;
                    if nat_eq(a285, a286) {
                        if v1 {
                            let a287 = v3;
                            let a288 = 0u64;
                            if nat_eq(a287, a288) {
                                Ok(f10({
                                    let a291 = 2u64;
                                    let a292 = {
                                        let a289 = v2;
                                        let a290 = 2u64;
                                        nat_mul(a289, a290)?
                                    };
                                    nat_add(a291, a292)?
                                }, 0u64, {
                                    let a293 = v4;
                                    let a294 = 256u64;
                                    let a295 = 0u64;
                                    nat_rem(a293, a294, a295)
                                }))
                            } else {
                                Ok(f10({
                                    let a298 = 3u64;
                                    let a299 = {
                                        let a296 = v2;
                                        let a297 = 2u64;
                                        nat_mul(a296, a297)?
                                    };
                                    nat_add(a298, a299)?
                                }, 0u64, {
                                    let a300 = v4;
                                    let a301 = 256u64;
                                    let a302 = 0u64;
                                    nat_quot(a300, a301, a302)
                                }))
                            }
                        } else {
                            let a303 = v3;
                            let a304 = 0u64;
                            if nat_eq(a303, a304) {
                                Ok(f10({
                                    let a305 = 2u64;
                                    let a306 = v2;
                                    nat_add(a305, a306)?
                                }, 0u64, {
                                    let a307 = v4;
                                    let a308 = 256u64;
                                    let a309 = 0u64;
                                    nat_rem(a307, a308, a309)
                                }))
                            } else {
                                Ok(f10({
                                    let a313 = 66u64;
                                    let a314 = {
                                        let a310 = v2;
                                        let a311 = 4u64;
                                        let a312 = 0u64;
                                        nat_quot(a310, a311, a312)
                                    };
                                    nat_add(a313, a314)?
                                }, {
                                    let a318 = 2u64;
                                    let a319 = {
                                        let a315 = v2;
                                        let a316 = 4u64;
                                        let a317 = 0u64;
                                        nat_rem(a315, a316, a317)
                                    };
                                    nat_mul(a318, a319)?
                                }, {
                                    let a320 = v4;
                                    let a321 = 256u64;
                                    let a322 = 0u64;
                                    nat_quot(a320, a321, a322)
                                }))
                            }
                        }
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a323 = v0;
                    let a324 = 8u64;
                    if nat_eq(a323, a324) {
                        Ok(f19({
                            let a328 = if v1 {
                                66u64
                            } else {
                                74u64
                            };
                            let a329 = {
                                let a325 = v2;
                                let a326 = 2u64;
                                let a327 = 0u64;
                                nat_quot(a325, a326, a327)
                            };
                            nat_add(a328, a329)?
                        }, {
                            let a333 = 4u64;
                            let a334 = {
                                let a330 = v2;
                                let a331 = 2u64;
                                let a332 = 0u64;
                                nat_rem(a330, a331, a332)
                            };
                            nat_mul(a333, a334)?
                        }))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a335 = v0;
    let a336 = 0u64;
    if nat_eq(a335, a336) {
        let a337 = v1;
        let a338 = 256u64;
        if if if nat_eq(a337, a338) {
            let a339 = v2;
            let a340 = 256u64;
            nat_eq(a339, a340)
        } else {
            false
        } {
            let a341 = v3;
            let a342 = 66u64;
            nat_eq(a341, a342)
        } else {
            false
        } {
            Ok(f16(8u64, 32u64, 6u64, 646978941u64, 113413u64))
        } else {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        }
    } else {
        let a343 = v0;
        let a344 = 4u64;
        if nat_eq(a343, a344) {
            Ok(f19(v1, {
                let a347 = 1160u64;
                let a348 = {
                    let a345 = v2;
                    let a346 = 8u64;
                    nat_mul(a345, a346)?
                };
                nat_add(a347, a348)?
            }))
        } else {
            let a349 = v0;
            let a350 = 7u64;
            if nat_eq(a349, a350) {
                Ok(f10({
                    let a355 = {
                        let a353 = 2u64;
                        let a354 = {
                            let a351 = v1;
                            let a352 = 8u64;
                            nat_mul(a351, a352)?
                        };
                        nat_add(a353, a354)?
                    };
                    let a356 = v2;
                    nat_add(a355, a356)?
                }, 0u64, v3))
            } else {
                let a357 = v0;
                let a358 = 8u64;
                if nat_eq(a357, a358) {
                    Ok(f19({
                        let a361 = 9u64;
                        let a362 = {
                            let a359 = v1;
                            let a360 = 8u64;
                            nat_mul(a359, a360)?
                        };
                        nat_add(a361, a362)?
                    }, 4u64))
                } else {
                    let a363 = v0;
                    let a364 = 9u64;
                    if nat_eq(a363, a364) {
                        Ok(f19({
                            let a367 = 6u64;
                            let a368 = {
                                let a365 = v1;
                                let a366 = 8u64;
                                nat_mul(a365, a366)?
                            };
                            nat_add(a367, a368)?
                        }, {
                            let a369 = v2;
                            let a370 = 7u64;
                            nat_mul(a369, a370)?
                        }))
                    } else {
                        f13(v0, true, v1, v2, v3)
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a371 = v0;
    let a372 = 4u64;
    if nat_lt(a371, a372) {
        let a373 = v0;
        let a374 = 2u64;
        if nat_lt(a373, a374) {
            let a375 = v0;
            let a376 = 1u64;
            if nat_lt(a375, a376) {
                let a377 = v0;
                let a378 = 0u64;
                if nat_eq(a377, a378) {
                    if v1 {
                        let a379 = v2;
                        let a380 = 256u64;
                        if if if nat_eq(a379, a380) {
                            let a381 = v3;
                            let a382 = 0u64;
                            if nat_eq(a381, a382) {
                                true
                            } else {
                                let a383 = v3;
                                let a384 = 256u64;
                                nat_eq(a383, a384)
                            }
                        } else {
                            false
                        } {
                            let a385 = v4;
                            let a386 = 56u64;
                            nat_eq(a385, a386)
                        } else {
                            false
                        } {
                            Ok(f16(16u64, 16u64, 16u64, 869711765u64, 329769u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a387 = v2;
                        let a388 = 256u64;
                        if if if nat_eq(a387, a388) {
                            let a389 = v3;
                            let a390 = 256u64;
                            nat_eq(a389, a390)
                        } else {
                            false
                        } {
                            let a391 = v4;
                            let a392 = 50u64;
                            nat_eq(a391, a392)
                        } else {
                            false
                        } {
                            Ok(f16(8u64, 32u64, 32u64, 730643660u64, 329769u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a393 = v0;
                let a394 = 1u64;
                if nat_eq(a393, a394) {
                    Ok(f21(v2, v3))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        } else {
            let a395 = v0;
            let a396 = 3u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 2u64;
                if nat_eq(a397, a398) {
                    Ok(f2(v2, v3))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a399 = v0;
                let a400 = 3u64;
                if nat_eq(a399, a400) {
                    f13(3u64, false, v2, v3, v4)
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        }
    } else {
        let a401 = v0;
        let a402 = 6u64;
        if nat_lt(a401, a402) {
            let a403 = v0;
            let a404 = 5u64;
            if nat_lt(a403, a404) {
                let a405 = v0;
                let a406 = 4u64;
                if nat_eq(a405, a406) {
                    Ok(f19(v2, {
                        let a409 = 18600u64;
                        let a410 = {
                            let a407 = v3;
                            let a408 = 8u64;
                            nat_mul(a407, a408)?
                        };
                        nat_add(a409, a410)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a411 = v0;
                let a412 = 5u64;
                if nat_eq(a411, a412) {
                    Ok(f21({
                        let a413 = v2;
                        let a414 = 255u64;
                        if nat_eq(a413, a414) {
                            0u64
                        } else {
                            let a415 = v2;
                            let a416 = 0u64;
                            if nat_eq(a415, a416) {
                                1u64
                            } else {
                                2u64
                            }
                        }
                    }, v3))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        } else {
            let a417 = v0;
            let a418 = 7u64;
            if nat_lt(a417, a418) {
                let a419 = v0;
                let a420 = 6u64;
                if nat_eq(a419, a420) {
                    Ok(f11({
                        let a421 = v2;
                        let a422 = 2u64;
                        if nat_lt(a421, a422) {
                            let a423 = v2;
                            let a424 = 1u64;
                            if nat_lt(a423, a424) {
                                let a425 = v2;
                                let a426 = 0u64;
                                let _ = nat_eq(a425, a426);
                                0u64
                            } else {
                                let a427 = v2;
                                let a428 = 1u64;
                                if nat_eq(a427, a428) {
                                    128u64
                                } else {
                                    0u64
                                }
                            }
                        } else {
                            let a429 = v2;
                            let a430 = 3u64;
                            if nat_lt(a429, a430) {
                                let a431 = v2;
                                let a432 = 2u64;
                                if nat_eq(a431, a432) {
                                    8u64
                                } else {
                                    0u64
                                }
                            } else {
                                let a433 = v2;
                                let a434 = 3u64;
                                if nat_eq(a433, a434) {
                                    136u64
                                } else {
                                    0u64
                                }
                            }
                        }
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a435 = v0;
                let a436 = 8u64;
                if nat_lt(a435, a436) {
                    let a437 = v0;
                    let a438 = 7u64;
                    if nat_eq(a437, a438) {
                        if v1 {
                            Ok(f10({
                                let a441 = {
                                    let a439 = v2;
                                    let a440 = 2u64;
                                    nat_mul(a439, a440)?
                                };
                                let a442 = v3;
                                nat_add(a441, a442)?
                            }, {
                                let a443 = v3;
                                let a444 = 0u64;
                                if nat_eq(a443, a444) {
                                    1u64
                                } else {
                                    16u64
                                }
                            }, {
                                let a445 = v4;
                                let a446 = 256u64;
                                let a447 = 0u64;
                                nat_rem(a445, a446, a447)
                            }))
                        } else {
                            Ok(f10({
                                let a452 = {
                                    let a450 = 2u64;
                                    let a451 = {
                                        let a448 = v2;
                                        let a449 = 4u64;
                                        nat_mul(a448, a449)?
                                    };
                                    nat_add(a450, a451)?
                                };
                                let a453 = v3;
                                nat_add(a452, a453)?
                            }, f22(v3), {
                                let a454 = v4;
                                let a455 = 256u64;
                                let a456 = 0u64;
                                nat_rem(a454, a455, a456)
                            }))
                        }
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a457 = v0;
                    let a458 = 8u64;
                    if nat_eq(a457, a458) {
                        Ok(f19({
                            let a464 = 48u64;
                            let a465 = {
                                let a462 = 2u64;
                                let a463 = {
                                    let a459 = v2;
                                    let a460 = 4u64;
                                    let a461 = 0u64;
                                    nat_quot(a459, a460, a461)
                                };
                                nat_mul(a462, a463)?
                            };
                            nat_add(a464, a465)?
                        }, f22({
                            let a466 = v2;
                            let a467 = 4u64;
                            let a468 = 0u64;
                            nat_rem(a466, a467, a468)
                        })))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (6u64, (v0, (v1, (v2, (v3, v4)))))
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> u64 {
    let v3: u64 = {
        let a469 = v0;
        let a470 = v2;
        nat_sub(a469, a470)
    };
    let a471 = v3;
    let a472 = v1;
    if nat_lt(a471, a472) {
        v3
    } else {
        v1
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a481 = v0;
    let a482 = {
        let a479 = {
            let a476 = {
                let a473 = v1;
                let a474 = v2;
                let a475 = 0u64;
                nat_quot(a473, a474, a475)
            };
            let a477 = v3;
            let a478 = 0u64;
            nat_rem(a476, a477, a478)
        };
        let a480 = v4;
        nat_mul(a479, a480)?
    };
    nat_add(a481, a482)
}

pub fn f19(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (7u64, (v0, (v1, (0u64, (0u64, 0u64)))))
}

pub fn f20(v0: u64) -> u64 {
    let a483 = v0;
    let a484 = 0u64;
    if nat_eq(a483, a484) {
        1u64
    } else {
        let a485 = v0;
        let a486 = 1u64;
        if nat_eq(a485, a486) {
            2u64
        } else {
            let a487 = v0;
            let a488 = 2u64;
            if nat_eq(a487, a488) {
                4u64
            } else {
                let a489 = v0;
                let a490 = 3u64;
                if nat_eq(a489, a490) {
                    8u64
                } else {
                    let a491 = v0;
                    let a492 = 4u64;
                    if nat_eq(a491, a492) {
                        16u64
                    } else {
                        let a493 = v0;
                        let a494 = 5u64;
                        if nat_eq(a493, a494) {
                            32u64
                        } else {
                            let a495 = v0;
                            let a496 = 6u64;
                            if nat_eq(a495, a496) {
                                64u64
                            } else {
                                let a497 = v0;
                                let a498 = 7u64;
                                if nat_eq(a497, a498) {
                                    128u64
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    let a499 = v0;
    let a500 = 2u64;
    if nat_lt(a499, a500) {
        let a501 = v0;
        let a502 = 1u64;
        if nat_lt(a501, a502) {
            let a503 = v0;
            let a504 = 0u64;
            if nat_eq(a503, a504) {
                f11({
                    let a505 = v1;
                    let a506 = 0u64;
                    if nat_eq(a505, a506) {
                        3210739712u64
                    } else {
                        3213885440u64
                    }
                })
            } else {
                (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
            }
        } else {
            let a507 = v0;
            let a508 = 1u64;
            if nat_eq(a507, a508) {
                f11({
                    let a509 = v1;
                    let a510 = 0u64;
                    if nat_eq(a509, a510) {
                        1040187392u64
                    } else {
                        3187671040u64
                    }
                })
            } else {
                (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
            }
        }
    } else {
        let a511 = v0;
        let a512 = 3u64;
        if nat_lt(a511, a512) {
            let a513 = v0;
            let a514 = 2u64;
            if nat_eq(a513, a514) {
                f11({
                    let a515 = v1;
                    let a516 = 0u64;
                    if nat_eq(a515, a516) {
                        1066401792u64
                    } else {
                        1063256064u64
                    }
                })
            } else {
                (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
            }
        } else {
            let a517 = v0;
            let a518 = 4u64;
            if nat_lt(a517, a518) {
                let a519 = v0;
                let a520 = 3u64;
                if nat_eq(a519, a520) {
                    f11({
                        let a521 = v1;
                        let a522 = 0u64;
                        if nat_eq(a521, a522) {
                            1061158912u64
                        } else {
                            1067450368u64
                        }
                    })
                } else {
                    (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
                }
            } else {
                let a523 = v0;
                let a524 = 4u64;
                if nat_eq(a523, a524) {
                    f11({
                        let a525 = v1;
                        let a526 = 0u64;
                        if nat_eq(a525, a526) {
                            3214934016u64
                        } else {
                            3208642560u64
                        }
                    })
                } else {
                    (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> u64 {
    let a527 = v0;
    let a528 = 2u64;
    if nat_lt(a527, a528) {
        let a529 = v0;
        let a530 = 1u64;
        if nat_lt(a529, a530) {
            let a531 = v0;
            let a532 = 0u64;
            if nat_eq(a531, a532) {
                1u64
            } else {
                0u64
            }
        } else {
            let a533 = v0;
            let a534 = 1u64;
            if nat_eq(a533, a534) {
                8u64
            } else {
                0u64
            }
        }
    } else {
        let a535 = v0;
        let a536 = 3u64;
        if nat_lt(a535, a536) {
            let a537 = v0;
            let a538 = 2u64;
            if nat_eq(a537, a538) {
                64u64
            } else {
                0u64
            }
        } else {
            let a539 = v0;
            let a540 = 3u64;
            if nat_eq(a539, a540) {
                512u64
            } else {
                0u64
            }
        }
    }
}
