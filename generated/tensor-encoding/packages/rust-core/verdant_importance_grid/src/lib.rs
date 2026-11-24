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
    let a2 = 6u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 3u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 1u64;
            if nat_lt(a5, a6) {
                let a7 = v0;
                let a8 = 0u64;
                if nat_eq(a7, a8) {
                    Ok(f1(v1, v2, v3, v4))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a9 = v0;
                let a10 = 2u64;
                if nat_lt(a9, a10) {
                    let a11 = v0;
                    let a12 = 1u64;
                    let _ = nat_eq(a11, a12);
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                } else {
                    let a13 = v0;
                    let a14 = 2u64;
                    if nat_eq(a13, a14) {
                        Ok(f2(v2, v3))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        } else {
            let a15 = v0;
            let a16 = 4u64;
            if nat_lt(a15, a16) {
                let a17 = v0;
                let a18 = 3u64;
                if nat_eq(a17, a18) {
                    f3(v2, v3, v4)
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a19 = v0;
                let a20 = 5u64;
                if nat_lt(a19, a20) {
                    let a21 = v0;
                    let a22 = 4u64;
                    if nat_eq(a21, a22) {
                        f4(v1, v2, v3)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a23 = v0;
                    let a24 = 5u64;
                    if nat_eq(a23, a24) {
                        f5(v1, v2)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    } else {
        let a25 = v0;
        let a26 = 9u64;
        if nat_lt(a25, a26) {
            let a27 = v0;
            let a28 = 7u64;
            if nat_lt(a27, a28) {
                let a29 = v0;
                let a30 = 6u64;
                if nat_eq(a29, a30) {
                    Ok(f6(v2))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a31 = v0;
                let a32 = 8u64;
                if nat_lt(a31, a32) {
                    let a33 = v0;
                    let a34 = 7u64;
                    if nat_eq(a33, a34) {
                        f7(v1, v2, v3, v4)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a35 = v0;
                    let a36 = 8u64;
                    if nat_eq(a35, a36) {
                        f8(v1, v2)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        } else {
            let a37 = v0;
            let a38 = 10u64;
            if nat_lt(a37, a38) {
                let a39 = v0;
                let a40 = 9u64;
                if nat_eq(a39, a40) {
                    f9(v1, v2, v3)
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a41 = v0;
                let a42 = 11u64;
                if nat_lt(a41, a42) {
                    let a43 = v0;
                    let a44 = 10u64;
                    if nat_eq(a43, a44) {
                        let v5: u64 = {
                            let a45 = v2;
                            let a46 = 4294967296u64;
                            let a47 = 0u64;
                            nat_quot(a45, a46, a47)
                        };
                        let v6: u64 = {
                            let a48 = v2;
                            let a49 = 4294967296u64;
                            let a50 = 0u64;
                            nat_rem(a48, a49, a50)
                        };
                        let a51 = v5;
                        let a52 = 2u64;
                        if nat_eq(a51, a52) {
                            Ok(f10(v5, {
                                let a53 = v6;
                                let a54 = 256u64;
                                let a55 = 0u64;
                                nat_quot(a53, a54, a55)
                            }, {
                                let a56 = v6;
                                let a57 = 256u64;
                                let a58 = 0u64;
                                nat_rem(a56, a57, a58)
                            }))
                        } else {
                            let a59 = v5;
                            let a60 = 2u64;
                            if nat_lt(a59, a60) {
                                Ok(f10(v5, v6, 0u64))
                            } else {
                                Ok(f10(v5, 0u64, v6))
                            }
                        }
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a61 = v0;
                    let a62 = 11u64;
                    if nat_eq(a61, a62) {
                        Ok(f11(f12(v2, v3, v4)?))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    let a63 = v1;
    let a64 = 256u64;
    if if if nat_eq(a63, a64) {
        let a65 = v2;
        let a66 = 0u64;
        if nat_eq(a65, a66) {
            true
        } else {
            let a67 = v2;
            let a68 = 256u64;
            nat_eq(a67, a68)
        }
    } else {
        false
    } {
        let a69 = v3;
        let a70 = if v0 {
            98u64
        } else {
            110u64
        };
        nat_eq(a69, a70)
    } else {
        false
    } {
        f13(8u64, 32u64, if v0 {
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
    f11(f14({
        let a71 = v0;
        let a72 = 8388608u64;
        let a73 = 0u64;
        nat_rem(a71, a72, a73)
    }, v1, 4194304u64))
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11(f15(v0, v2, 1u64, 8u64, {
        let a74 = v1;
        let a75 = 0u64;
        if nat_eq(a74, a75) {
            1u64
        } else {
            let a76 = v1;
            let a77 = 1u64;
            if nat_eq(a76, a77) {
                8u64
            } else {
                let a78 = v1;
                let a79 = 2u64;
                if nat_eq(a78, a79) {
                    64u64
                } else {
                    let a80 = v1;
                    let a81 = 3u64;
                    if nat_eq(a80, a81) {
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
    Ok(f16({
        let a82 = if v0 {
            0u64
        } else {
            4097u64
        };
        let a83 = v1;
        nat_add(a82, a83)?
    }, {
        let a86 = if v0 {
            15496u64
        } else {
            16520u64
        };
        let a87 = {
            let a84 = 4u64;
            let a85 = v2;
            nat_mul(a84, a85)?
        };
        nat_add(a86, a87)?
    }))
}

pub fn f5(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11({
        let a93 = 1u64;
        let a94 = {
            let a91 = 2u64;
            let a92 = {
                let a88 = v1;
                let a89 = if v0 {
                    8u64
                } else {
                    2u64
                };
                let a90 = 0u64;
                nat_quot(a88, a89, a90)
            };
            nat_mul(a91, a92)?
        };
        nat_add(a93, a94)?
    }))
}

pub fn f6(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f17(v0))
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a95 = v2;
    let a96 = 0u64;
    if nat_eq(a95, a96) {
        Ok(f10({
            let a97 = 2u64;
            let a98 = v1;
            nat_add(a97, a98)?
        }, 0u64, {
            let a99 = v3;
            let a100 = 256u64;
            let a101 = 0u64;
            nat_rem(a99, a100, a101)
        }))
    } else {
        if v0 {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        } else {
            Ok(f10({
                let a105 = 66u64;
                let a106 = {
                    let a102 = v1;
                    let a103 = 8u64;
                    let a104 = 0u64;
                    nat_quot(a102, a103, a104)
                };
                nat_add(a105, a106)?
            }, {
                let a107 = v1;
                let a108 = 8u64;
                let a109 = 0u64;
                nat_rem(a107, a108, a109)
            }, {
                let a113 = {
                    let a110 = v3;
                    let a111 = 256u64;
                    let a112 = 0u64;
                    nat_quot(a110, a111, a112)
                };
                let a114 = 2u64;
                let a115 = 0u64;
                nat_rem(a113, a114, a115)
            }))
        }
    }
}

pub fn f8(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f16({
            let a118 = 69u64;
            let a119 = {
                let a116 = 4u64;
                let a117 = v1;
                nat_mul(a116, a117)?
            };
            nat_add(a118, a119)?
        }, 4u64))
    } else {
        Ok(f16({
            let a123 = 106u64;
            let a124 = {
                let a120 = v1;
                let a121 = 2u64;
                let a122 = 0u64;
                nat_quot(a120, a121, a122)
            };
            nat_add(a123, a124)?
        }, {
            let a128 = 4u64;
            let a129 = {
                let a125 = v1;
                let a126 = 2u64;
                let a127 = 0u64;
                nat_rem(a125, a126, a127)
            };
            nat_mul(a128, a129)?
        }))
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f16({
            let a132 = 66u64;
            let a133 = {
                let a130 = 4u64;
                let a131 = v1;
                nat_mul(a130, a131)?
            };
            nat_add(a132, a133)?
        }, {
            let a134 = 7u64;
            let a135 = v2;
            nat_mul(a134, a135)?
        }))
    } else {
        Ok(f16({
            let a140 = {
                let a138 = 74u64;
                let a139 = {
                    let a136 = 4u64;
                    let a137 = v1;
                    nat_mul(a136, a137)?
                };
                nat_add(a138, a139)?
            };
            let a141 = v2;
            nat_add(a140, a141)?
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
    let a142 = v0;
    let a143 = 3u64;
    if nat_lt(a142, a143) {
        let a144 = v0;
        let a145 = 1u64;
        if nat_lt(a144, a145) {
            let a146 = v0;
            let a147 = 0u64;
            if nat_eq(a146, a147) {
                let a148 = v1;
                let a149 = v2;
                nat_add(a148, a149)
            } else {
                Ok(0u64)
            }
        } else {
            let a150 = v0;
            let a151 = 2u64;
            if nat_lt(a150, a151) {
                let a152 = v0;
                let a153 = 1u64;
                if nat_eq(a152, a153) {
                    let a154 = v1;
                    let a155 = v2;
                    Ok(nat_sub(a154, a155))
                } else {
                    Ok(0u64)
                }
            } else {
                let a156 = v0;
                let a157 = 2u64;
                if nat_eq(a156, a157) {
                    let a158 = v1;
                    let a159 = v2;
                    nat_mul(a158, a159)
                } else {
                    Ok(0u64)
                }
            }
        }
    } else {
        let a160 = v0;
        let a161 = 5u64;
        if nat_lt(a160, a161) {
            let a162 = v0;
            let a163 = 4u64;
            if nat_lt(a162, a163) {
                let a164 = v0;
                let a165 = 3u64;
                if nat_eq(a164, a165) {
                    let a166 = v1;
                    let a167 = v2;
                    let a168 = 0u64;
                    Ok(nat_quot(a166, a167, a168))
                } else {
                    Ok(0u64)
                }
            } else {
                let a169 = v0;
                let a170 = 4u64;
                if nat_eq(a169, a170) {
                    let a171 = v1;
                    let a172 = v2;
                    let a173 = 0u64;
                    Ok(nat_rem(a171, a172, a173))
                } else {
                    Ok(0u64)
                }
            }
        } else {
            let a174 = v0;
            let a175 = 6u64;
            if nat_lt(a174, a175) {
                let a176 = v0;
                let a177 = 5u64;
                if nat_eq(a176, a177) {
                    let a178 = v1;
                    let a179 = v2;
                    if nat_eq(a178, a179) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a180 = v0;
                let a181 = 6u64;
                if nat_eq(a180, a181) {
                    let a182 = v1;
                    let a183 = v2;
                    if nat_lt(a182, a183) {
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

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (6u64, (v0, (v1, (v2, (v3, v4)))))
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> u64 {
    let v3: u64 = {
        let a184 = v0;
        let a185 = v2;
        nat_sub(a184, a185)
    };
    let a186 = v3;
    let a187 = v1;
    if nat_lt(a186, a187) {
        v3
    } else {
        v1
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a196 = v0;
    let a197 = {
        let a194 = {
            let a191 = {
                let a188 = v1;
                let a189 = v2;
                let a190 = 0u64;
                nat_quot(a188, a189, a190)
            };
            let a192 = v3;
            let a193 = 0u64;
            nat_rem(a191, a192, a193)
        };
        let a195 = v4;
        nat_mul(a194, a195)?
    };
    nat_add(a196, a197)
}

pub fn f16(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (7u64, (v0, (v1, (0u64, (0u64, 0u64)))))
}

pub fn f17(v0: u64) -> u64 {
    let a198 = v0;
    let a199 = 0u64;
    if nat_eq(a198, a199) {
        1u64
    } else {
        let a200 = v0;
        let a201 = 1u64;
        if nat_eq(a200, a201) {
            2u64
        } else {
            let a202 = v0;
            let a203 = 2u64;
            if nat_eq(a202, a203) {
                4u64
            } else {
                let a204 = v0;
                let a205 = 3u64;
                if nat_eq(a204, a205) {
                    8u64
                } else {
                    let a206 = v0;
                    let a207 = 4u64;
                    if nat_eq(a206, a207) {
                        16u64
                    } else {
                        let a208 = v0;
                        let a209 = 5u64;
                        if nat_eq(a208, a209) {
                            32u64
                        } else {
                            let a210 = v0;
                            let a211 = 6u64;
                            if nat_eq(a210, a211) {
                                64u64
                            } else {
                                let a212 = v0;
                                let a213 = 7u64;
                                if nat_eq(a212, a213) {
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

pub fn run(p0: u64, p1: bool, p2: u64, p3: u64, p4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    f0(p0, p1, p2, p3, p4)
}
