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

pub fn f0(v0: u64, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    f1(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a9 = v0;
                let a10 = 2u64;
                if nat_lt(a9, a10) {
                    let a11 = v0;
                    let a12 = 1u64;
                    if nat_eq(a11, a12) {
                        f3(v1)
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a13 = v0;
                    let a14 = 2u64;
                    if nat_eq(a13, a14) {
                        f4(v1)
                    } else {
                        Ok(f2())
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
                    f5(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a19 = v0;
                let a20 = 5u64;
                if nat_lt(a19, a20) {
                    let a21 = v0;
                    let a22 = 4u64;
                    if nat_eq(a21, a22) {
                        f6(v1)
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a23 = v0;
                    let a24 = 5u64;
                    if nat_eq(a23, a24) {
                        f7(v1)
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    } else {
        let a25 = v0;
        let a26 = 7u64;
        if nat_lt(a25, a26) {
            let a27 = v0;
            let a28 = 6u64;
            if nat_eq(a27, a28) {
                f8(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a29 = v0;
            let a30 = 7u64;
            if nat_eq(a29, a30) {
                f9(v1)
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f1(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a31 = v0;
        let a32 = 8u64;
        let a33 = 0u64;
        nat_quot(a31, a32, a33)
    };
    let a34 = v1;
    let a35 = 3u64;
    if nat_lt(a34, a35) {
        let a36 = v1;
        let a37 = 1u64;
        if nat_lt(a36, a37) {
            let a38 = v1;
            let a39 = 0u64;
            if nat_eq(a38, a39) {
                f10({
                    let a40 = v0;
                    let a41 = 8u64;
                    let a42 = 0u64;
                    nat_rem(a40, a41, a42)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a43 = v1;
            let a44 = 2u64;
            if nat_lt(a43, a44) {
                let a45 = v1;
                let a46 = 1u64;
                if nat_eq(a45, a46) {
                    Ok(f11({
                        let a47 = v0;
                        let a48 = 8u64;
                        let a49 = 0u64;
                        nat_rem(a47, a48, a49)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a50 = v1;
                let a51 = 2u64;
                if nat_eq(a50, a51) {
                    f12({
                        let a52 = v0;
                        let a53 = 8u64;
                        let a54 = 0u64;
                        nat_rem(a52, a53, a54)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a55 = v1;
        let a56 = 4u64;
        if nat_lt(a55, a56) {
            let a57 = v1;
            let a58 = 3u64;
            if nat_eq(a57, a58) {
                Ok(f13({
                    let a59 = v0;
                    let a60 = 8u64;
                    let a61 = 0u64;
                    nat_rem(a59, a60, a61)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a62 = v1;
            let a63 = 5u64;
            if nat_lt(a62, a63) {
                let a64 = v1;
                let a65 = 4u64;
                if nat_eq(a64, a65) {
                    f14({
                        let a66 = v0;
                        let a67 = 8u64;
                        let a68 = 0u64;
                        nat_rem(a66, a67, a68)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a69 = v1;
                let a70 = 5u64;
                if nat_eq(a69, a70) {
                    Ok(f15({
                        let a71 = v0;
                        let a72 = 8u64;
                        let a73 = 0u64;
                        nat_rem(a71, a72, a73)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a74 = v0;
        let a75 = 8u64;
        let a76 = 0u64;
        nat_quot(a74, a75, a76)
    };
    let a77 = v1;
    let a78 = 3u64;
    if nat_lt(a77, a78) {
        let a79 = v1;
        let a80 = 1u64;
        if nat_lt(a79, a80) {
            let a81 = v1;
            let a82 = 0u64;
            if nat_eq(a81, a82) {
                f16({
                    let a83 = v0;
                    let a84 = 8u64;
                    let a85 = 0u64;
                    nat_rem(a83, a84, a85)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a86 = v1;
            let a87 = 2u64;
            if nat_lt(a86, a87) {
                let a88 = v1;
                let a89 = 1u64;
                if nat_eq(a88, a89) {
                    f17({
                        let a90 = v0;
                        let a91 = 8u64;
                        let a92 = 0u64;
                        nat_rem(a90, a91, a92)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a93 = v1;
                let a94 = 2u64;
                if nat_eq(a93, a94) {
                    f18({
                        let a95 = v0;
                        let a96 = 8u64;
                        let a97 = 0u64;
                        nat_rem(a95, a96, a97)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a98 = v1;
        let a99 = 5u64;
        if nat_lt(a98, a99) {
            let a100 = v1;
            let a101 = 4u64;
            if nat_lt(a100, a101) {
                let a102 = v1;
                let a103 = 3u64;
                if nat_eq(a102, a103) {
                    f19({
                        let a104 = v0;
                        let a105 = 8u64;
                        let a106 = 0u64;
                        nat_rem(a104, a105, a106)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a107 = v1;
                let a108 = 4u64;
                if nat_eq(a107, a108) {
                    f20({
                        let a109 = v0;
                        let a110 = 8u64;
                        let a111 = 0u64;
                        nat_rem(a109, a110, a111)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a112 = v1;
            let a113 = 6u64;
            if nat_lt(a112, a113) {
                let a114 = v1;
                let a115 = 5u64;
                if nat_eq(a114, a115) {
                    Ok(f21({
                        let a116 = v0;
                        let a117 = 8u64;
                        let a118 = 0u64;
                        nat_rem(a116, a117, a118)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a119 = v1;
                let a120 = 6u64;
                if nat_eq(a119, a120) {
                    Ok(f22({
                        let a121 = v0;
                        let a122 = 8u64;
                        let a123 = 0u64;
                        nat_rem(a121, a122, a123)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a124 = v0;
        let a125 = 8u64;
        let a126 = 0u64;
        nat_quot(a124, a125, a126)
    };
    let a127 = v1;
    let a128 = 3u64;
    if nat_lt(a127, a128) {
        let a129 = v1;
        let a130 = 1u64;
        if nat_lt(a129, a130) {
            let a131 = v1;
            let a132 = 0u64;
            if nat_eq(a131, a132) {
                f23({
                    let a133 = v0;
                    let a134 = 8u64;
                    let a135 = 0u64;
                    nat_rem(a133, a134, a135)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a136 = v1;
            let a137 = 2u64;
            if nat_lt(a136, a137) {
                let a138 = v1;
                let a139 = 1u64;
                if nat_eq(a138, a139) {
                    Ok(f24({
                        let a140 = v0;
                        let a141 = 8u64;
                        let a142 = 0u64;
                        nat_rem(a140, a141, a142)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a143 = v1;
                let a144 = 2u64;
                if nat_eq(a143, a144) {
                    f25({
                        let a145 = v0;
                        let a146 = 8u64;
                        let a147 = 0u64;
                        nat_rem(a145, a146, a147)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a148 = v1;
        let a149 = 4u64;
        if nat_lt(a148, a149) {
            let a150 = v1;
            let a151 = 3u64;
            if nat_eq(a150, a151) {
                Ok(f13({
                    let a152 = v0;
                    let a153 = 8u64;
                    let a154 = 0u64;
                    nat_rem(a152, a153, a154)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a155 = v1;
            let a156 = 5u64;
            if nat_lt(a155, a156) {
                let a157 = v1;
                let a158 = 4u64;
                if nat_eq(a157, a158) {
                    f26({
                        let a159 = v0;
                        let a160 = 8u64;
                        let a161 = 0u64;
                        nat_rem(a159, a160, a161)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a162 = v1;
                let a163 = 5u64;
                if nat_eq(a162, a163) {
                    Ok(f15({
                        let a164 = v0;
                        let a165 = 8u64;
                        let a166 = 0u64;
                        nat_rem(a164, a165, a166)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a167 = v0;
        let a168 = 8u64;
        let a169 = 0u64;
        nat_quot(a167, a168, a169)
    };
    let a170 = v1;
    let a171 = 4u64;
    if nat_lt(a170, a171) {
        let a172 = v1;
        let a173 = 2u64;
        if nat_lt(a172, a173) {
            let a174 = v1;
            let a175 = 1u64;
            if nat_lt(a174, a175) {
                let a176 = v1;
                let a177 = 0u64;
                if nat_eq(a176, a177) {
                    f27({
                        let a178 = v0;
                        let a179 = 8u64;
                        let a180 = 0u64;
                        nat_rem(a178, a179, a180)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a181 = v1;
                let a182 = 1u64;
                if nat_eq(a181, a182) {
                    f28({
                        let a183 = v0;
                        let a184 = 8u64;
                        let a185 = 0u64;
                        nat_rem(a183, a184, a185)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a186 = v1;
            let a187 = 3u64;
            if nat_lt(a186, a187) {
                let a188 = v1;
                let a189 = 2u64;
                if nat_eq(a188, a189) {
                    f29({
                        let a190 = v0;
                        let a191 = 8u64;
                        let a192 = 0u64;
                        nat_rem(a190, a191, a192)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a193 = v1;
                let a194 = 3u64;
                if nat_eq(a193, a194) {
                    f30({
                        let a195 = v0;
                        let a196 = 8u64;
                        let a197 = 0u64;
                        nat_rem(a195, a196, a197)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a198 = v1;
        let a199 = 6u64;
        if nat_lt(a198, a199) {
            let a200 = v1;
            let a201 = 5u64;
            if nat_lt(a200, a201) {
                let a202 = v1;
                let a203 = 4u64;
                if nat_eq(a202, a203) {
                    f31({
                        let a204 = v0;
                        let a205 = 8u64;
                        let a206 = 0u64;
                        nat_rem(a204, a205, a206)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a207 = v1;
                let a208 = 5u64;
                if nat_eq(a207, a208) {
                    f32({
                        let a209 = v0;
                        let a210 = 8u64;
                        let a211 = 0u64;
                        nat_rem(a209, a210, a211)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a212 = v1;
            let a213 = 7u64;
            if nat_lt(a212, a213) {
                let a214 = v1;
                let a215 = 6u64;
                if nat_eq(a214, a215) {
                    Ok(f33({
                        let a216 = v0;
                        let a217 = 8u64;
                        let a218 = 0u64;
                        nat_rem(a216, a217, a218)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a219 = v1;
                let a220 = 7u64;
                if nat_eq(a219, a220) {
                    Ok(f34({
                        let a221 = v0;
                        let a222 = 8u64;
                        let a223 = 0u64;
                        nat_rem(a221, a222, a223)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a224 = v0;
        let a225 = 8u64;
        let a226 = 0u64;
        nat_quot(a224, a225, a226)
    };
    let a227 = v1;
    let a228 = 10u64;
    if nat_lt(a227, a228) {
        let a229 = v1;
        let a230 = 5u64;
        if nat_lt(a229, a230) {
            let a231 = v1;
            let a232 = 2u64;
            if nat_lt(a231, a232) {
                let a233 = v1;
                let a234 = 1u64;
                if nat_lt(a233, a234) {
                    let a235 = v1;
                    let a236 = 0u64;
                    if nat_eq(a235, a236) {
                        f35({
                            let a237 = v0;
                            let a238 = 8u64;
                            let a239 = 0u64;
                            nat_rem(a237, a238, a239)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a240 = v1;
                    let a241 = 1u64;
                    if nat_eq(a240, a241) {
                        f36({
                            let a242 = v0;
                            let a243 = 8u64;
                            let a244 = 0u64;
                            nat_rem(a242, a243, a244)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a245 = v1;
                let a246 = 3u64;
                if nat_lt(a245, a246) {
                    let a247 = v1;
                    let a248 = 2u64;
                    if nat_eq(a247, a248) {
                        Ok(f37({
                            let a249 = v0;
                            let a250 = 8u64;
                            let a251 = 0u64;
                            nat_rem(a249, a250, a251)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a252 = v1;
                    let a253 = 4u64;
                    if nat_lt(a252, a253) {
                        let a254 = v1;
                        let a255 = 3u64;
                        if nat_eq(a254, a255) {
                            f38({
                                let a256 = v0;
                                let a257 = 8u64;
                                let a258 = 0u64;
                                nat_rem(a256, a257, a258)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a259 = v1;
                        let a260 = 4u64;
                        if nat_eq(a259, a260) {
                            f39({
                                let a261 = v0;
                                let a262 = 8u64;
                                let a263 = 0u64;
                                nat_rem(a261, a262, a263)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        } else {
            let a264 = v1;
            let a265 = 7u64;
            if nat_lt(a264, a265) {
                let a266 = v1;
                let a267 = 6u64;
                if nat_lt(a266, a267) {
                    let a268 = v1;
                    let a269 = 5u64;
                    if nat_eq(a268, a269) {
                        Ok(f40({
                            let a270 = v0;
                            let a271 = 8u64;
                            let a272 = 0u64;
                            nat_rem(a270, a271, a272)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a273 = v1;
                    let a274 = 6u64;
                    if nat_eq(a273, a274) {
                        f41({
                            let a275 = v0;
                            let a276 = 8u64;
                            let a277 = 0u64;
                            nat_rem(a275, a276, a277)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a278 = v1;
                let a279 = 8u64;
                if nat_lt(a278, a279) {
                    let a280 = v1;
                    let a281 = 7u64;
                    if nat_eq(a280, a281) {
                        f42({
                            let a282 = v0;
                            let a283 = 8u64;
                            let a284 = 0u64;
                            nat_rem(a282, a283, a284)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a285 = v1;
                    let a286 = 9u64;
                    if nat_lt(a285, a286) {
                        let a287 = v1;
                        let a288 = 8u64;
                        if nat_eq(a287, a288) {
                            f43({
                                let a289 = v0;
                                let a290 = 8u64;
                                let a291 = 0u64;
                                nat_rem(a289, a290, a291)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a292 = v1;
                        let a293 = 9u64;
                        if nat_eq(a292, a293) {
                            f44({
                                let a294 = v0;
                                let a295 = 8u64;
                                let a296 = 0u64;
                                nat_rem(a294, a295, a296)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        }
    } else {
        let a297 = v1;
        let a298 = 15u64;
        if nat_lt(a297, a298) {
            let a299 = v1;
            let a300 = 12u64;
            if nat_lt(a299, a300) {
                let a301 = v1;
                let a302 = 11u64;
                if nat_lt(a301, a302) {
                    let a303 = v1;
                    let a304 = 10u64;
                    if nat_eq(a303, a304) {
                        f45({
                            let a305 = v0;
                            let a306 = 8u64;
                            let a307 = 0u64;
                            nat_rem(a305, a306, a307)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a308 = v1;
                    let a309 = 11u64;
                    if nat_eq(a308, a309) {
                        f46({
                            let a310 = v0;
                            let a311 = 8u64;
                            let a312 = 0u64;
                            nat_rem(a310, a311, a312)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a313 = v1;
                let a314 = 13u64;
                if nat_lt(a313, a314) {
                    let a315 = v1;
                    let a316 = 12u64;
                    if nat_eq(a315, a316) {
                        f47({
                            let a317 = v0;
                            let a318 = 8u64;
                            let a319 = 0u64;
                            nat_rem(a317, a318, a319)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a320 = v1;
                    let a321 = 14u64;
                    if nat_lt(a320, a321) {
                        let a322 = v1;
                        let a323 = 13u64;
                        if nat_eq(a322, a323) {
                            f48({
                                let a324 = v0;
                                let a325 = 8u64;
                                let a326 = 0u64;
                                nat_rem(a324, a325, a326)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a327 = v1;
                        let a328 = 14u64;
                        if nat_eq(a327, a328) {
                            Ok(f49({
                                let a329 = v0;
                                let a330 = 8u64;
                                let a331 = 0u64;
                                nat_rem(a329, a330, a331)
                            }))
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        } else {
            let a332 = v1;
            let a333 = 18u64;
            if nat_lt(a332, a333) {
                let a334 = v1;
                let a335 = 16u64;
                if nat_lt(a334, a335) {
                    let a336 = v1;
                    let a337 = 15u64;
                    if nat_eq(a336, a337) {
                        f50({
                            let a338 = v0;
                            let a339 = 8u64;
                            let a340 = 0u64;
                            nat_rem(a338, a339, a340)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a341 = v1;
                    let a342 = 17u64;
                    if nat_lt(a341, a342) {
                        let a343 = v1;
                        let a344 = 16u64;
                        if nat_eq(a343, a344) {
                            Ok(f51({
                                let a345 = v0;
                                let a346 = 8u64;
                                let a347 = 0u64;
                                nat_rem(a345, a346, a347)
                            }))
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a348 = v1;
                        let a349 = 17u64;
                        if nat_eq(a348, a349) {
                            f52({
                                let a350 = v0;
                                let a351 = 8u64;
                                let a352 = 0u64;
                                nat_rem(a350, a351, a352)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            } else {
                let a353 = v1;
                let a354 = 19u64;
                if nat_lt(a353, a354) {
                    let a355 = v1;
                    let a356 = 18u64;
                    if nat_eq(a355, a356) {
                        Ok(f53({
                            let a357 = v0;
                            let a358 = 8u64;
                            let a359 = 0u64;
                            nat_rem(a357, a358, a359)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a360 = v1;
                    let a361 = 20u64;
                    if nat_lt(a360, a361) {
                        let a362 = v1;
                        let a363 = 19u64;
                        if nat_eq(a362, a363) {
                            f54({
                                let a364 = v0;
                                let a365 = 8u64;
                                let a366 = 0u64;
                                nat_rem(a364, a365, a366)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a367 = v1;
                        let a368 = 20u64;
                        if nat_eq(a367, a368) {
                            Ok(f55({
                                let a369 = v0;
                                let a370 = 8u64;
                                let a371 = 0u64;
                                nat_rem(a369, a370, a371)
                            }))
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a372 = v0;
        let a373 = 8u64;
        let a374 = 0u64;
        nat_quot(a372, a373, a374)
    };
    let a375 = v1;
    let a376 = 4u64;
    if nat_lt(a375, a376) {
        let a377 = v1;
        let a378 = 2u64;
        if nat_lt(a377, a378) {
            let a379 = v1;
            let a380 = 1u64;
            if nat_lt(a379, a380) {
                let a381 = v1;
                let a382 = 0u64;
                if nat_eq(a381, a382) {
                    f27({
                        let a383 = v0;
                        let a384 = 8u64;
                        let a385 = 0u64;
                        nat_rem(a383, a384, a385)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a386 = v1;
                let a387 = 1u64;
                if nat_eq(a386, a387) {
                    f28({
                        let a388 = v0;
                        let a389 = 8u64;
                        let a390 = 0u64;
                        nat_rem(a388, a389, a390)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a391 = v1;
            let a392 = 3u64;
            if nat_lt(a391, a392) {
                let a393 = v1;
                let a394 = 2u64;
                if nat_eq(a393, a394) {
                    f29({
                        let a395 = v0;
                        let a396 = 8u64;
                        let a397 = 0u64;
                        nat_rem(a395, a396, a397)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a398 = v1;
                let a399 = 3u64;
                if nat_eq(a398, a399) {
                    f30({
                        let a400 = v0;
                        let a401 = 8u64;
                        let a402 = 0u64;
                        nat_rem(a400, a401, a402)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a403 = v1;
        let a404 = 6u64;
        if nat_lt(a403, a404) {
            let a405 = v1;
            let a406 = 5u64;
            if nat_lt(a405, a406) {
                let a407 = v1;
                let a408 = 4u64;
                if nat_eq(a407, a408) {
                    f31({
                        let a409 = v0;
                        let a410 = 8u64;
                        let a411 = 0u64;
                        nat_rem(a409, a410, a411)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a412 = v1;
                let a413 = 5u64;
                if nat_eq(a412, a413) {
                    f32({
                        let a414 = v0;
                        let a415 = 8u64;
                        let a416 = 0u64;
                        nat_rem(a414, a415, a416)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a417 = v1;
            let a418 = 7u64;
            if nat_lt(a417, a418) {
                let a419 = v1;
                let a420 = 6u64;
                if nat_eq(a419, a420) {
                    Ok(f33({
                        let a421 = v0;
                        let a422 = 8u64;
                        let a423 = 0u64;
                        nat_rem(a421, a422, a423)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a424 = v1;
                let a425 = 7u64;
                if nat_eq(a424, a425) {
                    Ok(f34({
                        let a426 = v0;
                        let a427 = 8u64;
                        let a428 = 0u64;
                        nat_rem(a426, a427, a428)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a429 = v0;
        let a430 = 8u64;
        let a431 = 0u64;
        nat_quot(a429, a430, a431)
    };
    let a432 = v1;
    let a433 = 7u64;
    if nat_lt(a432, a433) {
        let a434 = v1;
        let a435 = 3u64;
        if nat_lt(a434, a435) {
            let a436 = v1;
            let a437 = 1u64;
            if nat_lt(a436, a437) {
                let a438 = v1;
                let a439 = 0u64;
                if nat_eq(a438, a439) {
                    f56({
                        let a440 = v0;
                        let a441 = 8u64;
                        let a442 = 0u64;
                        nat_rem(a440, a441, a442)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a443 = v1;
                let a444 = 2u64;
                if nat_lt(a443, a444) {
                    let a445 = v1;
                    let a446 = 1u64;
                    if nat_eq(a445, a446) {
                        f57({
                            let a447 = v0;
                            let a448 = 8u64;
                            let a449 = 0u64;
                            nat_rem(a447, a448, a449)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a450 = v1;
                    let a451 = 2u64;
                    if nat_eq(a450, a451) {
                        f58({
                            let a452 = v0;
                            let a453 = 8u64;
                            let a454 = 0u64;
                            nat_rem(a452, a453, a454)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            }
        } else {
            let a455 = v1;
            let a456 = 5u64;
            if nat_lt(a455, a456) {
                let a457 = v1;
                let a458 = 4u64;
                if nat_lt(a457, a458) {
                    let a459 = v1;
                    let a460 = 3u64;
                    if nat_eq(a459, a460) {
                        f59({
                            let a461 = v0;
                            let a462 = 8u64;
                            let a463 = 0u64;
                            nat_rem(a461, a462, a463)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a464 = v1;
                    let a465 = 4u64;
                    if nat_eq(a464, a465) {
                        Ok(f60({
                            let a466 = v0;
                            let a467 = 8u64;
                            let a468 = 0u64;
                            nat_rem(a466, a467, a468)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a469 = v1;
                let a470 = 6u64;
                if nat_lt(a469, a470) {
                    let a471 = v1;
                    let a472 = 5u64;
                    if nat_eq(a471, a472) {
                        f61({
                            let a473 = v0;
                            let a474 = 8u64;
                            let a475 = 0u64;
                            nat_rem(a473, a474, a475)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a476 = v1;
                    let a477 = 6u64;
                    if nat_eq(a476, a477) {
                        f62({
                            let a478 = v0;
                            let a479 = 8u64;
                            let a480 = 0u64;
                            nat_rem(a478, a479, a480)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    } else {
        let a481 = v1;
        let a482 = 10u64;
        if nat_lt(a481, a482) {
            let a483 = v1;
            let a484 = 8u64;
            if nat_lt(a483, a484) {
                let a485 = v1;
                let a486 = 7u64;
                if nat_eq(a485, a486) {
                    f63({
                        let a487 = v0;
                        let a488 = 8u64;
                        let a489 = 0u64;
                        nat_rem(a487, a488, a489)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a490 = v1;
                let a491 = 9u64;
                if nat_lt(a490, a491) {
                    let a492 = v1;
                    let a493 = 8u64;
                    if nat_eq(a492, a493) {
                        f64({
                            let a494 = v0;
                            let a495 = 8u64;
                            let a496 = 0u64;
                            nat_rem(a494, a495, a496)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a497 = v1;
                    let a498 = 9u64;
                    if nat_eq(a497, a498) {
                        Ok(f65({
                            let a499 = v0;
                            let a500 = 8u64;
                            let a501 = 0u64;
                            nat_rem(a499, a500, a501)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            }
        } else {
            let a502 = v1;
            let a503 = 12u64;
            if nat_lt(a502, a503) {
                let a504 = v1;
                let a505 = 11u64;
                if nat_lt(a504, a505) {
                    let a506 = v1;
                    let a507 = 10u64;
                    if nat_eq(a506, a507) {
                        Ok(f66({
                            let a508 = v0;
                            let a509 = 8u64;
                            let a510 = 0u64;
                            nat_rem(a508, a509, a510)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a511 = v1;
                    let a512 = 11u64;
                    if nat_eq(a511, a512) {
                        Ok(f67({
                            let a513 = v0;
                            let a514 = 8u64;
                            let a515 = 0u64;
                            nat_rem(a513, a514, a515)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a516 = v1;
                let a517 = 13u64;
                if nat_lt(a516, a517) {
                    let a518 = v1;
                    let a519 = 12u64;
                    if nat_eq(a518, a519) {
                        f68({
                            let a520 = v0;
                            let a521 = 8u64;
                            let a522 = 0u64;
                            nat_rem(a520, a521, a522)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a523 = v1;
                    let a524 = 13u64;
                    if nat_eq(a523, a524) {
                        f69({
                            let a525 = v0;
                            let a526 = 8u64;
                            let a527 = 0u64;
                            nat_rem(a525, a526, a527)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a528 = v0;
        let a529 = 8u64;
        let a530 = 0u64;
        nat_quot(a528, a529, a530)
    };
    let a531 = v1;
    let a532 = 5u64;
    if nat_lt(a531, a532) {
        let a533 = v1;
        let a534 = 2u64;
        if nat_lt(a533, a534) {
            let a535 = v1;
            let a536 = 1u64;
            if nat_lt(a535, a536) {
                let a537 = v1;
                let a538 = 0u64;
                if nat_eq(a537, a538) {
                    Ok(f70({
                        let a539 = v0;
                        let a540 = 8u64;
                        let a541 = 0u64;
                        nat_rem(a539, a540, a541)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a542 = v1;
                let a543 = 1u64;
                if nat_eq(a542, a543) {
                    f71({
                        let a544 = v0;
                        let a545 = 8u64;
                        let a546 = 0u64;
                        nat_rem(a544, a545, a546)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a547 = v1;
            let a548 = 3u64;
            if nat_lt(a547, a548) {
                let a549 = v1;
                let a550 = 2u64;
                if nat_eq(a549, a550) {
                    f72({
                        let a551 = v0;
                        let a552 = 8u64;
                        let a553 = 0u64;
                        nat_rem(a551, a552, a553)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a554 = v1;
                let a555 = 4u64;
                if nat_lt(a554, a555) {
                    let a556 = v1;
                    let a557 = 3u64;
                    if nat_eq(a556, a557) {
                        f73({
                            let a558 = v0;
                            let a559 = 8u64;
                            let a560 = 0u64;
                            nat_rem(a558, a559, a560)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a561 = v1;
                    let a562 = 4u64;
                    if nat_eq(a561, a562) {
                        f74({
                            let a563 = v0;
                            let a564 = 8u64;
                            let a565 = 0u64;
                            nat_rem(a563, a564, a565)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    } else {
        let a566 = v1;
        let a567 = 7u64;
        if nat_lt(a566, a567) {
            let a568 = v1;
            let a569 = 6u64;
            if nat_lt(a568, a569) {
                let a570 = v1;
                let a571 = 5u64;
                if nat_eq(a570, a571) {
                    f75({
                        let a572 = v0;
                        let a573 = 8u64;
                        let a574 = 0u64;
                        nat_rem(a572, a573, a574)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a575 = v1;
                let a576 = 6u64;
                if nat_eq(a575, a576) {
                    f76({
                        let a577 = v0;
                        let a578 = 8u64;
                        let a579 = 0u64;
                        nat_rem(a577, a578, a579)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a580 = v1;
            let a581 = 8u64;
            if nat_lt(a580, a581) {
                let a582 = v1;
                let a583 = 7u64;
                if nat_eq(a582, a583) {
                    Ok(f77({
                        let a584 = v0;
                        let a585 = 8u64;
                        let a586 = 0u64;
                        nat_rem(a584, a585, a586)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a587 = v1;
                let a588 = 9u64;
                if nat_lt(a587, a588) {
                    let a589 = v1;
                    let a590 = 8u64;
                    if nat_eq(a589, a590) {
                        f78({
                            let a591 = v0;
                            let a592 = 8u64;
                            let a593 = 0u64;
                            nat_rem(a591, a592, a593)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a594 = v1;
                    let a595 = 9u64;
                    if nat_eq(a594, a595) {
                        Ok(f79({
                            let a596 = v0;
                            let a597 = 8u64;
                            let a598 = 0u64;
                            nat_rem(a596, a597, a598)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a599 = v0;
    let a600 = 4u64;
    if nat_lt(a599, a600) {
        let a601 = v0;
        let a602 = 2u64;
        if nat_lt(a601, a602) {
            let a603 = v0;
            let a604 = 1u64;
            if nat_lt(a603, a604) {
                let a605 = v0;
                let a606 = 0u64;
                if nat_eq(a605, a606) {
                    Ok(f80(3u64, f81(), 1097859072u64, f82()))
                } else {
                    Ok(f2())
                }
            } else {
                let a607 = v0;
                let a608 = 1u64;
                if nat_eq(a607, a608) {
                    Ok(f83(0u64, f84()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a609 = v0;
            let a610 = 3u64;
            if nat_lt(a609, a610) {
                let a611 = v0;
                let a612 = 2u64;
                if nat_eq(a611, a612) {
                    Ok(f83(0u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a613 = v0;
                let a614 = 3u64;
                if nat_eq(a613, a614) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a615 = v0;
        let a616 = 6u64;
        if nat_lt(a615, a616) {
            let a617 = v0;
            let a618 = 5u64;
            if nat_lt(a617, a618) {
                let a619 = v0;
                let a620 = 4u64;
                if nat_eq(a619, a620) {
                    Ok(f83(0u64, f87(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a621 = v0;
                let a622 = 5u64;
                if nat_eq(a621, a622) {
                    Ok(f83(f88(), f89(f86())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a623 = v0;
            let a624 = 7u64;
            if nat_lt(a623, a624) {
                let a625 = v0;
                let a626 = 6u64;
                if nat_eq(a625, a626) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a627 = v0;
                let a628 = 7u64;
                if nat_eq(a627, a628) {
                    Ok(f91(1u64, f86(), 8u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a629 = v0;
    let a630 = 4u64;
    if nat_lt(a629, a630) {
        let a631 = v0;
        let a632 = 2u64;
        if nat_lt(a631, a632) {
            let a633 = v0;
            let a634 = 1u64;
            if nat_lt(a633, a634) {
                let a635 = v0;
                let a636 = 0u64;
                if nat_eq(a635, a636) {
                    f92(f85(), f93())
                } else {
                    f2()
                }
            } else {
                let a637 = v0;
                let a638 = 1u64;
                if nat_eq(a637, a638) {
                    f92(f94(), f95())
                } else {
                    f2()
                }
            }
        } else {
            let a639 = v0;
            let a640 = 3u64;
            if nat_lt(a639, a640) {
                let a641 = v0;
                let a642 = 2u64;
                if nat_eq(a641, a642) {
                    f80(1u64, f93(), f95(), f93())
                } else {
                    f2()
                }
            } else {
                let a643 = v0;
                let a644 = 3u64;
                if nat_eq(a643, a644) {
                    f80(2u64, f93(), 1045220557u64, f93())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a645 = v0;
        let a646 = 6u64;
        if nat_lt(a645, a646) {
            let a647 = v0;
            let a648 = 5u64;
            if nat_lt(a647, a648) {
                let a649 = v0;
                let a650 = 4u64;
                if nat_eq(a649, a650) {
                    f80(0u64, 1097859072u64, f93(), f93())
                } else {
                    f2()
                }
            } else {
                let a651 = v0;
                let a652 = 5u64;
                if nat_eq(a651, a652) {
                    f80(3u64, f93(), f81(), f96())
                } else {
                    f2()
                }
            }
        } else {
            let a653 = v0;
            let a654 = 7u64;
            if nat_lt(a653, a654) {
                let a655 = v0;
                let a656 = 6u64;
                if nat_eq(a655, a656) {
                    f80(3u64, 1065353216u64, f96(), f97())
                } else {
                    f2()
                }
            } else {
                let a657 = v0;
                let a658 = 7u64;
                if nat_eq(a657, a658) {
                    f83(0u64, f86())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a659 = v0;
    let a660 = 4u64;
    if nat_lt(a659, a660) {
        let a661 = v0;
        let a662 = 2u64;
        if nat_lt(a661, a662) {
            let a663 = v0;
            let a664 = 1u64;
            if nat_lt(a663, a664) {
                let a665 = v0;
                let a666 = 0u64;
                if nat_eq(a665, a666) {
                    Ok(f83(1042u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a667 = v0;
                let a668 = 1u64;
                if nat_eq(a667, a668) {
                    Ok(f99(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a669 = v0;
            let a670 = 3u64;
            if nat_lt(a669, a670) {
                let a671 = v0;
                let a672 = 2u64;
                if nat_eq(a671, a672) {
                    Ok(f83(f100(), f101(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a673 = v0;
                let a674 = 3u64;
                if nat_eq(a673, a674) {
                    Ok(f83(f102(), f103(f86())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a675 = v0;
        let a676 = 6u64;
        if nat_lt(a675, a676) {
            let a677 = v0;
            let a678 = 5u64;
            if nat_lt(a677, a678) {
                let a679 = v0;
                let a680 = 4u64;
                if nat_eq(a679, a680) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a681 = v0;
                let a682 = 5u64;
                if nat_eq(a681, a682) {
                    Ok(f91(1u64, f86(), 8u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a683 = v0;
            let a684 = 7u64;
            if nat_lt(a683, a684) {
                let a685 = v0;
                let a686 = 6u64;
                if nat_eq(a685, a686) {
                    Ok(f83(0u64, f104()))
                } else {
                    Ok(f2())
                }
            } else {
                let a687 = v0;
                let a688 = 7u64;
                if nat_eq(a687, a688) {
                    Ok(f83(1049u64, f105()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a689 = v0;
    let a690 = 4u64;
    if nat_lt(a689, a690) {
        let a691 = v0;
        let a692 = 2u64;
        if nat_lt(a691, a692) {
            let a693 = v0;
            let a694 = 1u64;
            if nat_lt(a693, a694) {
                let a695 = v0;
                let a696 = 0u64;
                if nat_eq(a695, a696) {
                    f99(2048u64)
                } else {
                    f2()
                }
            } else {
                let a697 = v0;
                let a698 = 1u64;
                if nat_eq(a697, a698) {
                    f91(2u64, 0u64, f106(), 1051u64)
                } else {
                    f2()
                }
            }
        } else {
            let a699 = v0;
            let a700 = 3u64;
            if nat_lt(a699, a700) {
                let a701 = v0;
                let a702 = 2u64;
                if nat_eq(a701, a702) {
                    f99(1062u64)
                } else {
                    f2()
                }
            } else {
                let a703 = v0;
                let a704 = 3u64;
                if nat_eq(a703, a704) {
                    f80(2u64, f107(), f107(), f95())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a705 = v0;
        let a706 = 6u64;
        if nat_lt(a705, a706) {
            let a707 = v0;
            let a708 = 5u64;
            if nat_lt(a707, a708) {
                let a709 = v0;
                let a710 = 4u64;
                if nat_eq(a709, a710) {
                    f80(2u64, f84(), f106(), f93())
                } else {
                    f2()
                }
            } else {
                let a711 = v0;
                let a712 = 5u64;
                if nat_eq(a711, a712) {
                    f91(2u64, f93(), f95(), 1055u64)
                } else {
                    f2()
                }
            }
        } else {
            let a713 = v0;
            let a714 = 7u64;
            if nat_lt(a713, a714) {
                let a715 = v0;
                let a716 = 6u64;
                if nat_eq(a715, a716) {
                    f99(1062u64)
                } else {
                    f2()
                }
            } else {
                let a717 = v0;
                let a718 = 7u64;
                if nat_eq(a717, a718) {
                    f80(3u64, f107(), f106(), f82())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a719 = v0;
    let a720 = 4u64;
    if nat_lt(a719, a720) {
        let a721 = v0;
        let a722 = 2u64;
        if nat_lt(a721, a722) {
            let a723 = v0;
            let a724 = 1u64;
            if nat_lt(a723, a724) {
                let a725 = v0;
                let a726 = 0u64;
                if nat_eq(a725, a726) {
                    Ok(f80(2u64, f82(), f107(), f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a727 = v0;
                let a728 = 1u64;
                if nat_eq(a727, a728) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a729 = v0;
            let a730 = 3u64;
            if nat_lt(a729, a730) {
                let a731 = v0;
                let a732 = 2u64;
                if nat_eq(a731, a732) {
                    Ok(f83(f101(f86())?, f87(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a733 = v0;
                let a734 = 3u64;
                if nat_eq(a733, a734) {
                    Ok(f83(f103(f86())?, f89(f86())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a735 = v0;
        let a736 = 6u64;
        if nat_lt(a735, a736) {
            let a737 = v0;
            let a738 = 5u64;
            if nat_lt(a737, a738) {
                let a739 = v0;
                let a740 = 4u64;
                if nat_eq(a739, a740) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a741 = v0;
                let a742 = 5u64;
                if nat_eq(a741, a742) {
                    Ok(f91(1u64, f86(), 8u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a743 = v0;
            let a744 = 7u64;
            if nat_lt(a743, a744) {
                let a745 = v0;
                let a746 = 6u64;
                if nat_eq(a745, a746) {
                    Ok(f90(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a747 = v0;
                let a748 = 7u64;
                if nat_eq(a747, a748) {
                    Ok(f90(2u64, f94(), 2u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a749 = v0;
    let a750 = 1u64;
    if nat_lt(a749, a750) {
        let a751 = v0;
        let a752 = 0u64;
        if nat_eq(a751, a752) {
            f90(0u64, f95(), 1u64, f95())
        } else {
            f2()
        }
    } else {
        let a753 = v0;
        let a754 = 2u64;
        if nat_lt(a753, a754) {
            let a755 = v0;
            let a756 = 1u64;
            if nat_eq(a755, a756) {
                f91(1u64, f85(), f95(), 1032u64)
            } else {
                f2()
            }
        } else {
            let a757 = v0;
            let a758 = 2u64;
            if nat_eq(a757, a758) {
                f99(2560u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a759 = v0;
    let a760 = 4u64;
    if nat_lt(a759, a760) {
        let a761 = v0;
        let a762 = 2u64;
        if nat_lt(a761, a762) {
            let a763 = v0;
            let a764 = 1u64;
            if nat_lt(a763, a764) {
                let a765 = v0;
                let a766 = 0u64;
                if nat_eq(a765, a766) {
                    Ok(f83(0u64, f108()))
                } else {
                    Ok(f2())
                }
            } else {
                let a767 = v0;
                let a768 = 1u64;
                if nat_eq(a767, a768) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a769 = v0;
            let a770 = 3u64;
            if nat_lt(a769, a770) {
                let a771 = v0;
                let a772 = 2u64;
                if nat_eq(a771, a772) {
                    Ok(f90(2u64, f86(), 4u64, f110()))
                } else {
                    Ok(f2())
                }
            } else {
                let a773 = v0;
                let a774 = 3u64;
                if nat_eq(a773, a774) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a775 = v0;
        let a776 = 6u64;
        if nat_lt(a775, a776) {
            let a777 = v0;
            let a778 = 5u64;
            if nat_lt(a777, a778) {
                let a779 = v0;
                let a780 = 4u64;
                if nat_eq(a779, a780) {
                    Ok(f80(2u64, f96(), f112(f111())?, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a781 = v0;
                let a782 = 5u64;
                if nat_eq(a781, a782) {
                    Ok(f80(1u64, f95(), 1065353216u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a783 = v0;
            let a784 = 7u64;
            if nat_lt(a783, a784) {
                let a785 = v0;
                let a786 = 6u64;
                if nat_eq(a785, a786) {
                    Ok(f80(2u64, 1056964608u64, f95(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a787 = v0;
                let a788 = 7u64;
                if nat_eq(a787, a788) {
                    Ok(f80(0u64, f95(), 1262485504u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a789 = v0;
    let a790 = 4u64;
    if nat_lt(a789, a790) {
        let a791 = v0;
        let a792 = 2u64;
        if nat_lt(a791, a792) {
            let a793 = v0;
            let a794 = 1u64;
            if nat_lt(a793, a794) {
                let a795 = v0;
                let a796 = 0u64;
                if nat_eq(a795, a796) {
                    Ok(f113(2u64, f95(), 7u64, 0u64, f114()))
                } else {
                    Ok(f2())
                }
            } else {
                let a797 = v0;
                let a798 = 1u64;
                if nat_eq(a797, a798) {
                    Ok(f113(3u64, f108(), f109(), f114(), f108()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a799 = v0;
            let a800 = 3u64;
            if nat_lt(a799, a800) {
                let a801 = v0;
                let a802 = 2u64;
                if nat_eq(a801, a802) {
                    Ok(f90(0u64, f109(), 1u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a803 = v0;
                let a804 = 3u64;
                if nat_eq(a803, a804) {
                    Ok(f91(1u64, f109(), 4u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a805 = v0;
        let a806 = 6u64;
        if nat_lt(a805, a806) {
            let a807 = v0;
            let a808 = 5u64;
            if nat_lt(a807, a808) {
                let a809 = v0;
                let a810 = 4u64;
                if nat_eq(a809, a810) {
                    Ok(f115(4u64, f108(), 0u64, 0u64, f116(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a811 = v0;
                let a812 = 5u64;
                if nat_eq(a811, a812) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a813 = v0;
            let a814 = 7u64;
            if nat_lt(a813, a814) {
                let a815 = v0;
                let a816 = 6u64;
                if nat_eq(a815, a816) {
                    Ok(f83(f117(f116())?, f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a817 = v0;
                let a818 = 7u64;
                if nat_eq(a817, a818) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a819 = v0;
    let a820 = 4u64;
    if nat_lt(a819, a820) {
        let a821 = v0;
        let a822 = 2u64;
        if nat_lt(a821, a822) {
            let a823 = v0;
            let a824 = 1u64;
            if nat_lt(a823, a824) {
                let a825 = v0;
                let a826 = 0u64;
                if nat_eq(a825, a826) {
                    Ok(f83(f117(f116())?, f119()))
                } else {
                    Ok(f2())
                }
            } else {
                let a827 = v0;
                let a828 = 1u64;
                if nat_eq(a827, a828) {
                    Ok(f83(0u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a829 = v0;
            let a830 = 3u64;
            if nat_lt(a829, a830) {
                let a831 = v0;
                let a832 = 2u64;
                if nat_eq(a831, a832) {
                    Ok(f83(f117(f116())?, f120()))
                } else {
                    Ok(f2())
                }
            } else {
                let a833 = v0;
                let a834 = 3u64;
                if nat_eq(a833, a834) {
                    Ok(f90(0u64, f120(), 1u64, f120()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a835 = v0;
        let a836 = 6u64;
        if nat_lt(a835, a836) {
            let a837 = v0;
            let a838 = 5u64;
            if nat_lt(a837, a838) {
                let a839 = v0;
                let a840 = 4u64;
                if nat_eq(a839, a840) {
                    Ok(f90(0u64, f118(), f120(), f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a841 = v0;
                let a842 = 5u64;
                if nat_eq(a841, a842) {
                    Ok(f90(0u64, f119(), f120(), f119()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a843 = v0;
            let a844 = 7u64;
            if nat_lt(a843, a844) {
                let a845 = v0;
                let a846 = 6u64;
                if nat_eq(a845, a846) {
                    Ok(f83(f117(f118())?, f100()))
                } else {
                    Ok(f2())
                }
            } else {
                let a847 = v0;
                let a848 = 7u64;
                if nat_eq(a847, a848) {
                    Ok(f90(0u64, f118(), 1u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a849 = v0;
    let a850 = 4u64;
    if nat_lt(a849, a850) {
        let a851 = v0;
        let a852 = 2u64;
        if nat_lt(a851, a852) {
            let a853 = v0;
            let a854 = 1u64;
            if nat_lt(a853, a854) {
                let a855 = v0;
                let a856 = 0u64;
                if nat_eq(a855, a856) {
                    Ok(f83(1u64, f102()))
                } else {
                    Ok(f2())
                }
            } else {
                let a857 = v0;
                let a858 = 1u64;
                if nat_eq(a857, a858) {
                    Ok(f91(0u64, f119(), f95(), 1586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a859 = v0;
            let a860 = 3u64;
            if nat_lt(a859, a860) {
                let a861 = v0;
                let a862 = 2u64;
                if nat_eq(a861, a862) {
                    Ok(f83(0u64, f102()))
                } else {
                    Ok(f2())
                }
            } else {
                let a863 = v0;
                let a864 = 3u64;
                if nat_eq(a863, a864) {
                    Ok(f83(2139095039u64, f121()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a865 = v0;
        let a866 = 6u64;
        if nat_lt(a865, a866) {
            let a867 = v0;
            let a868 = 5u64;
            if nat_lt(a867, a868) {
                let a869 = v0;
                let a870 = 4u64;
                if nat_eq(a869, a870) {
                    Ok(f83(f118(), f122()))
                } else {
                    Ok(f2())
                }
            } else {
                let a871 = v0;
                let a872 = 5u64;
                if nat_eq(a871, a872) {
                    Ok(f83(f117(f122())?, f123()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a873 = v0;
            let a874 = 7u64;
            if nat_lt(a873, a874) {
                let a875 = v0;
                let a876 = 6u64;
                if nat_eq(a875, a876) {
                    Ok(f83(0u64, f124()))
                } else {
                    Ok(f2())
                }
            } else {
                let a877 = v0;
                let a878 = 7u64;
                if nat_eq(a877, a878) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a879 = v0;
    let a880 = 4u64;
    if nat_lt(a879, a880) {
        let a881 = v0;
        let a882 = 2u64;
        if nat_lt(a881, a882) {
            let a883 = v0;
            let a884 = 1u64;
            if nat_lt(a883, a884) {
                let a885 = v0;
                let a886 = 0u64;
                if nat_eq(a885, a886) {
                    Ok(f115(4u64, 0u64, f123(), 0u64, f95(), f125()))
                } else {
                    Ok(f2())
                }
            } else {
                let a887 = v0;
                let a888 = 1u64;
                if nat_eq(a887, a888) {
                    Ok(f90(0u64, f125(), f109(), f125()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a889 = v0;
            let a890 = 3u64;
            if nat_lt(a889, a890) {
                let a891 = v0;
                let a892 = 2u64;
                if nat_eq(a891, a892) {
                    Ok(f113(5u64, f126(f125())?, 0u64, 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a893 = v0;
                let a894 = 3u64;
                if nat_eq(a893, a894) {
                    Ok(f92(f127(), f127()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a895 = v0;
        let a896 = 6u64;
        if nat_lt(a895, a896) {
            let a897 = v0;
            let a898 = 5u64;
            if nat_lt(a897, a898) {
                let a899 = v0;
                let a900 = 4u64;
                if nat_eq(a899, a900) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a901 = v0;
                let a902 = 5u64;
                if nat_eq(a901, a902) {
                    Ok(f80(2u64, f97(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a903 = v0;
            let a904 = 7u64;
            if nat_lt(a903, a904) {
                let a905 = v0;
                let a906 = 6u64;
                if nat_eq(a905, a906) {
                    Ok(f80(1u64, f93(), f112(f111())?, f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a907 = v0;
                let a908 = 7u64;
                if nat_eq(a907, a908) {
                    Ok(f80(2u64, f128(f111())?, f93(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a909 = v0;
    let a910 = 4u64;
    if nat_lt(a909, a910) {
        let a911 = v0;
        let a912 = 2u64;
        if nat_lt(a911, a912) {
            let a913 = v0;
            let a914 = 1u64;
            if nat_lt(a913, a914) {
                let a915 = v0;
                let a916 = 0u64;
                if nat_eq(a915, a916) {
                    f80(2u64, f95(), f93(), f95())
                } else {
                    f2()
                }
            } else {
                let a917 = v0;
                let a918 = 1u64;
                if nat_eq(a917, a918) {
                    f80(0u64, f124(), f95(), f124())
                } else {
                    f2()
                }
            }
        } else {
            let a919 = v0;
            let a920 = 3u64;
            if nat_lt(a919, a920) {
                let a921 = v0;
                let a922 = 2u64;
                if nat_eq(a921, a922) {
                    f90(0u64, f109(), 1u64, f109())
                } else {
                    f2()
                }
            } else {
                let a923 = v0;
                let a924 = 3u64;
                if nat_eq(a923, a924) {
                    f91(1u64, f109(), 4u64, 1568u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a925 = v0;
        let a926 = 6u64;
        if nat_lt(a925, a926) {
            let a927 = v0;
            let a928 = 5u64;
            if nat_lt(a927, a928) {
                let a929 = v0;
                let a930 = 4u64;
                if nat_eq(a929, a930) {
                    f91(2u64, f124(), f121(), 1582u64)
                } else {
                    f2()
                }
            } else {
                let a931 = v0;
                let a932 = 5u64;
                if nat_eq(a931, a932) {
                    f99(1584u64)
                } else {
                    f2()
                }
            }
        } else {
            let a933 = v0;
            let a934 = 7u64;
            if nat_lt(a933, a934) {
                let a935 = v0;
                let a936 = 6u64;
                if nat_eq(a935, a936) {
                    f83(f124(), f121())
                } else {
                    f2()
                }
            } else {
                let a937 = v0;
                let a938 = 7u64;
                if nat_eq(a937, a938) {
                    f83(f123(), f100())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a939 = v0;
    let a940 = 1u64;
    if nat_lt(a939, a940) {
        let a941 = v0;
        let a942 = 0u64;
        if nat_eq(a941, a942) {
            f90(0u64, f122(), 1u64, f122())
        } else {
            f2()
        }
    } else {
        let a943 = v0;
        let a944 = 2u64;
        if nat_lt(a943, a944) {
            let a945 = v0;
            let a946 = 1u64;
            if nat_eq(a945, a946) {
                f91(1u64, f122(), f119(), 1565u64)
            } else {
                f2()
            }
        } else {
            let a947 = v0;
            let a948 = 2u64;
            if nat_eq(a947, a948) {
                f129(f98())
            } else {
                f2()
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a949 = v0;
    let a950 = 4u64;
    if nat_lt(a949, a950) {
        let a951 = v0;
        let a952 = 2u64;
        if nat_lt(a951, a952) {
            let a953 = v0;
            let a954 = 1u64;
            if nat_lt(a953, a954) {
                let a955 = v0;
                let a956 = 0u64;
                if nat_eq(a955, a956) {
                    Ok(f80(3u64, f81(), 1084227584u64, f82()))
                } else {
                    Ok(f2())
                }
            } else {
                let a957 = v0;
                let a958 = 1u64;
                if nat_eq(a957, a958) {
                    Ok(f83(0u64, f84()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a959 = v0;
            let a960 = 3u64;
            if nat_lt(a959, a960) {
                let a961 = v0;
                let a962 = 2u64;
                if nat_eq(a961, a962) {
                    Ok(f83(0u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a963 = v0;
                let a964 = 3u64;
                if nat_eq(a963, a964) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a965 = v0;
        let a966 = 6u64;
        if nat_lt(a965, a966) {
            let a967 = v0;
            let a968 = 5u64;
            if nat_lt(a967, a968) {
                let a969 = v0;
                let a970 = 4u64;
                if nat_eq(a969, a970) {
                    Ok(f83(0u64, f87(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a971 = v0;
                let a972 = 5u64;
                if nat_eq(a971, a972) {
                    Ok(f83(1u64, f89(f86())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a973 = v0;
            let a974 = 7u64;
            if nat_lt(a973, a974) {
                let a975 = v0;
                let a976 = 6u64;
                if nat_eq(a975, a976) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a977 = v0;
                let a978 = 7u64;
                if nat_eq(a977, a978) {
                    Ok(f91(1u64, f86(), 2u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a979 = v0;
    let a980 = 4u64;
    if nat_lt(a979, a980) {
        let a981 = v0;
        let a982 = 2u64;
        if nat_lt(a981, a982) {
            let a983 = v0;
            let a984 = 1u64;
            if nat_lt(a983, a984) {
                let a985 = v0;
                let a986 = 0u64;
                if nat_eq(a985, a986) {
                    f92(f85(), f93())
                } else {
                    f2()
                }
            } else {
                let a987 = v0;
                let a988 = 1u64;
                if nat_eq(a987, a988) {
                    f92(f94(), f95())
                } else {
                    f2()
                }
            }
        } else {
            let a989 = v0;
            let a990 = 3u64;
            if nat_lt(a989, a990) {
                let a991 = v0;
                let a992 = 2u64;
                if nat_eq(a991, a992) {
                    f80(1u64, f93(), f95(), f93())
                } else {
                    f2()
                }
            } else {
                let a993 = v0;
                let a994 = 3u64;
                if nat_eq(a993, a994) {
                    f80(2u64, f93(), 1036831949u64, f93())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a995 = v0;
        let a996 = 6u64;
        if nat_lt(a995, a996) {
            let a997 = v0;
            let a998 = 5u64;
            if nat_lt(a997, a998) {
                let a999 = v0;
                let a1000 = 4u64;
                if nat_eq(a999, a1000) {
                    f80(0u64, 1084227584u64, f93(), f93())
                } else {
                    f2()
                }
            } else {
                let a1001 = v0;
                let a1002 = 5u64;
                if nat_eq(a1001, a1002) {
                    f80(3u64, f93(), f81(), f96())
                } else {
                    f2()
                }
            }
        } else {
            let a1003 = v0;
            let a1004 = 7u64;
            if nat_lt(a1003, a1004) {
                let a1005 = v0;
                let a1006 = 6u64;
                if nat_eq(a1005, a1006) {
                    f80(3u64, 1065353216u64, f96(), f97())
                } else {
                    f2()
                }
            } else {
                let a1007 = v0;
                let a1008 = 7u64;
                if nat_eq(a1007, a1008) {
                    f83(0u64, f86())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1009 = v0;
    let a1010 = 4u64;
    if nat_lt(a1009, a1010) {
        let a1011 = v0;
        let a1012 = 2u64;
        if nat_lt(a1011, a1012) {
            let a1013 = v0;
            let a1014 = 1u64;
            if nat_lt(a1013, a1014) {
                let a1015 = v0;
                let a1016 = 0u64;
                if nat_eq(a1015, a1016) {
                    Ok(f83(1042u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1017 = v0;
                let a1018 = 1u64;
                if nat_eq(a1017, a1018) {
                    Ok(f99(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1019 = v0;
            let a1020 = 3u64;
            if nat_lt(a1019, a1020) {
                let a1021 = v0;
                let a1022 = 2u64;
                if nat_eq(a1021, a1022) {
                    Ok(f83(f100(), f101(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1023 = v0;
                let a1024 = 3u64;
                if nat_eq(a1023, a1024) {
                    Ok(f83(f102(), f103(f86())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1025 = v0;
        let a1026 = 6u64;
        if nat_lt(a1025, a1026) {
            let a1027 = v0;
            let a1028 = 5u64;
            if nat_lt(a1027, a1028) {
                let a1029 = v0;
                let a1030 = 4u64;
                if nat_eq(a1029, a1030) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1031 = v0;
                let a1032 = 5u64;
                if nat_eq(a1031, a1032) {
                    Ok(f91(1u64, f86(), 2u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1033 = v0;
            let a1034 = 7u64;
            if nat_lt(a1033, a1034) {
                let a1035 = v0;
                let a1036 = 6u64;
                if nat_eq(a1035, a1036) {
                    Ok(f83(0u64, f104()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1037 = v0;
                let a1038 = 7u64;
                if nat_eq(a1037, a1038) {
                    Ok(f83(1049u64, f105()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1039 = v0;
    let a1040 = 4u64;
    if nat_lt(a1039, a1040) {
        let a1041 = v0;
        let a1042 = 2u64;
        if nat_lt(a1041, a1042) {
            let a1043 = v0;
            let a1044 = 1u64;
            if nat_lt(a1043, a1044) {
                let a1045 = v0;
                let a1046 = 0u64;
                if nat_eq(a1045, a1046) {
                    Ok(f80(2u64, f82(), f107(), f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1047 = v0;
                let a1048 = 1u64;
                if nat_eq(a1047, a1048) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1049 = v0;
            let a1050 = 3u64;
            if nat_lt(a1049, a1050) {
                let a1051 = v0;
                let a1052 = 2u64;
                if nat_eq(a1051, a1052) {
                    Ok(f83(f101(f86())?, f87(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1053 = v0;
                let a1054 = 3u64;
                if nat_eq(a1053, a1054) {
                    Ok(f83(f103(f86())?, f89(f86())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1055 = v0;
        let a1056 = 6u64;
        if nat_lt(a1055, a1056) {
            let a1057 = v0;
            let a1058 = 5u64;
            if nat_lt(a1057, a1058) {
                let a1059 = v0;
                let a1060 = 4u64;
                if nat_eq(a1059, a1060) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1061 = v0;
                let a1062 = 5u64;
                if nat_eq(a1061, a1062) {
                    Ok(f91(1u64, f86(), 2u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1063 = v0;
            let a1064 = 7u64;
            if nat_lt(a1063, a1064) {
                let a1065 = v0;
                let a1066 = 6u64;
                if nat_eq(a1065, a1066) {
                    Ok(f90(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1067 = v0;
                let a1068 = 7u64;
                if nat_eq(a1067, a1068) {
                    Ok(f90(2u64, f94(), 2u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1069 = v0;
    let a1070 = 4u64;
    if nat_lt(a1069, a1070) {
        let a1071 = v0;
        let a1072 = 2u64;
        if nat_lt(a1071, a1072) {
            let a1073 = v0;
            let a1074 = 1u64;
            if nat_lt(a1073, a1074) {
                let a1075 = v0;
                let a1076 = 0u64;
                if nat_eq(a1075, a1076) {
                    Ok(f83(0u64, f108()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1077 = v0;
                let a1078 = 1u64;
                if nat_eq(a1077, a1078) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1079 = v0;
            let a1080 = 3u64;
            if nat_lt(a1079, a1080) {
                let a1081 = v0;
                let a1082 = 2u64;
                if nat_eq(a1081, a1082) {
                    Ok(f90(2u64, f86(), 8u64, f110()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1083 = v0;
                let a1084 = 3u64;
                if nat_eq(a1083, a1084) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1085 = v0;
        let a1086 = 6u64;
        if nat_lt(a1085, a1086) {
            let a1087 = v0;
            let a1088 = 5u64;
            if nat_lt(a1087, a1088) {
                let a1089 = v0;
                let a1090 = 4u64;
                if nat_eq(a1089, a1090) {
                    Ok(f80(2u64, f96(), f112(f111())?, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1091 = v0;
                let a1092 = 5u64;
                if nat_eq(a1091, a1092) {
                    Ok(f80(1u64, f95(), 1065353216u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1093 = v0;
            let a1094 = 7u64;
            if nat_lt(a1093, a1094) {
                let a1095 = v0;
                let a1096 = 6u64;
                if nat_eq(a1095, a1096) {
                    Ok(f80(2u64, 1056964608u64, f95(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1097 = v0;
                let a1098 = 7u64;
                if nat_eq(a1097, a1098) {
                    Ok(f80(0u64, f95(), 1262485504u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1099 = v0;
    let a1100 = 4u64;
    if nat_lt(a1099, a1100) {
        let a1101 = v0;
        let a1102 = 2u64;
        if nat_lt(a1101, a1102) {
            let a1103 = v0;
            let a1104 = 1u64;
            if nat_lt(a1103, a1104) {
                let a1105 = v0;
                let a1106 = 0u64;
                if nat_eq(a1105, a1106) {
                    Ok(f113(2u64, f95(), 2u64, 0u64, f114()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1107 = v0;
                let a1108 = 1u64;
                if nat_eq(a1107, a1108) {
                    Ok(f113(3u64, f108(), f109(), f114(), f108()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1109 = v0;
            let a1110 = 3u64;
            if nat_lt(a1109, a1110) {
                let a1111 = v0;
                let a1112 = 2u64;
                if nat_eq(a1111, a1112) {
                    Ok(f90(0u64, f109(), 1u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1113 = v0;
                let a1114 = 3u64;
                if nat_eq(a1113, a1114) {
                    Ok(f91(1u64, f109(), 8u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1115 = v0;
        let a1116 = 6u64;
        if nat_lt(a1115, a1116) {
            let a1117 = v0;
            let a1118 = 5u64;
            if nat_lt(a1117, a1118) {
                let a1119 = v0;
                let a1120 = 4u64;
                if nat_eq(a1119, a1120) {
                    Ok(f115(4u64, f108(), 0u64, 0u64, f116(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1121 = v0;
                let a1122 = 5u64;
                if nat_eq(a1121, a1122) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1123 = v0;
            let a1124 = 7u64;
            if nat_lt(a1123, a1124) {
                let a1125 = v0;
                let a1126 = 6u64;
                if nat_eq(a1125, a1126) {
                    Ok(f83(f117(f116())?, f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1127 = v0;
                let a1128 = 7u64;
                if nat_eq(a1127, a1128) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1129 = v0;
    let a1130 = 4u64;
    if nat_lt(a1129, a1130) {
        let a1131 = v0;
        let a1132 = 2u64;
        if nat_lt(a1131, a1132) {
            let a1133 = v0;
            let a1134 = 1u64;
            if nat_lt(a1133, a1134) {
                let a1135 = v0;
                let a1136 = 0u64;
                if nat_eq(a1135, a1136) {
                    Ok(f83(f117(f116())?, f119()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1137 = v0;
                let a1138 = 1u64;
                if nat_eq(a1137, a1138) {
                    Ok(f91(0u64, f88(), 0u64, 1560u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1139 = v0;
            let a1140 = 3u64;
            if nat_lt(a1139, a1140) {
                let a1141 = v0;
                let a1142 = 2u64;
                if nat_eq(a1141, a1142) {
                    Ok(f90(0u64, f116(), 6561u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1143 = v0;
                let a1144 = 3u64;
                if nat_eq(a1143, a1144) {
                    Ok(f90(2u64, f117(f116())?, 65536u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1145 = v0;
        let a1146 = 6u64;
        if nat_lt(a1145, a1146) {
            let a1147 = v0;
            let a1148 = 5u64;
            if nat_lt(a1147, a1148) {
                let a1149 = v0;
                let a1150 = 4u64;
                if nat_eq(a1149, a1150) {
                    Ok(f90(0u64, f118(), f95(), f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1151 = v0;
                let a1152 = 5u64;
                if nat_eq(a1151, a1152) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1153 = v0;
            let a1154 = 7u64;
            if nat_lt(a1153, a1154) {
                let a1155 = v0;
                let a1156 = 6u64;
                if nat_eq(a1155, a1156) {
                    Ok(f90(2u64, f117(f116())?, 65536u64, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1157 = v0;
                let a1158 = 7u64;
                if nat_eq(a1157, a1158) {
                    Ok(f90(0u64, f119(), f95(), f119()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1159 = v0;
    let a1160 = 4u64;
    if nat_lt(a1159, a1160) {
        let a1161 = v0;
        let a1162 = 2u64;
        if nat_lt(a1161, a1162) {
            let a1163 = v0;
            let a1164 = 1u64;
            if nat_lt(a1163, a1164) {
                let a1165 = v0;
                let a1166 = 0u64;
                if nat_eq(a1165, a1166) {
                    Ok(f83(0u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1167 = v0;
                let a1168 = 1u64;
                if nat_eq(a1167, a1168) {
                    Ok(f83(f117(f116())?, f120()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1169 = v0;
            let a1170 = 3u64;
            if nat_lt(a1169, a1170) {
                let a1171 = v0;
                let a1172 = 2u64;
                if nat_eq(a1171, a1172) {
                    Ok(f90(0u64, f120(), 1u64, f120()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1173 = v0;
                let a1174 = 3u64;
                if nat_eq(a1173, a1174) {
                    Ok(f90(0u64, f118(), f120(), f118()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1175 = v0;
        let a1176 = 6u64;
        if nat_lt(a1175, a1176) {
            let a1177 = v0;
            let a1178 = 5u64;
            if nat_lt(a1177, a1178) {
                let a1179 = v0;
                let a1180 = 4u64;
                if nat_eq(a1179, a1180) {
                    Ok(f90(0u64, f119(), f120(), f119()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1181 = v0;
                let a1182 = 5u64;
                if nat_eq(a1181, a1182) {
                    Ok(f83(f117(f118())?, f100()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1183 = v0;
            let a1184 = 7u64;
            if nat_lt(a1183, a1184) {
                let a1185 = v0;
                let a1186 = 6u64;
                if nat_eq(a1185, a1186) {
                    Ok(f90(3u64, f100(), 32768u64, f102()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1187 = v0;
                let a1188 = 7u64;
                if nat_eq(a1187, a1188) {
                    Ok(f90(4u64, f100(), 32768u64, f100()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1189 = v0;
    let a1190 = 4u64;
    if nat_lt(a1189, a1190) {
        let a1191 = v0;
        let a1192 = 2u64;
        if nat_lt(a1191, a1192) {
            let a1193 = v0;
            let a1194 = 1u64;
            if nat_lt(a1193, a1194) {
                let a1195 = v0;
                let a1196 = 0u64;
                if nat_eq(a1195, a1196) {
                    Ok(f91(0u64, f102(), 1u64, 1593u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1197 = v0;
                let a1198 = 1u64;
                if nat_eq(a1197, a1198) {
                    Ok(f83(0u64, f102()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1199 = v0;
            let a1200 = 3u64;
            if nat_lt(a1199, a1200) {
                let a1201 = v0;
                let a1202 = 2u64;
                if nat_eq(a1201, a1202) {
                    Ok(f83(2139095039u64, f121()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1203 = v0;
                let a1204 = 3u64;
                if nat_eq(a1203, a1204) {
                    Ok(f83(f118(), f122()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1205 = v0;
        let a1206 = 6u64;
        if nat_lt(a1205, a1206) {
            let a1207 = v0;
            let a1208 = 5u64;
            if nat_lt(a1207, a1208) {
                let a1209 = v0;
                let a1210 = 4u64;
                if nat_eq(a1209, a1210) {
                    Ok(f83(f117(f122())?, f123()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1211 = v0;
                let a1212 = 5u64;
                if nat_eq(a1211, a1212) {
                    Ok(f83(0u64, f124()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1213 = v0;
            let a1214 = 7u64;
            if nat_lt(a1213, a1214) {
                let a1215 = v0;
                let a1216 = 6u64;
                if nat_eq(a1215, a1216) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1217 = v0;
                let a1218 = 7u64;
                if nat_eq(a1217, a1218) {
                    Ok(f115(4u64, 0u64, f123(), 0u64, f95(), f125()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f32(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1219 = v0;
    let a1220 = 4u64;
    if nat_lt(a1219, a1220) {
        let a1221 = v0;
        let a1222 = 2u64;
        if nat_lt(a1221, a1222) {
            let a1223 = v0;
            let a1224 = 1u64;
            if nat_lt(a1223, a1224) {
                let a1225 = v0;
                let a1226 = 0u64;
                if nat_eq(a1225, a1226) {
                    Ok(f90(0u64, f125(), f109(), f125()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1227 = v0;
                let a1228 = 1u64;
                if nat_eq(a1227, a1228) {
                    Ok(f113(5u64, f126(f125())?, 0u64, 0u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1229 = v0;
            let a1230 = 3u64;
            if nat_lt(a1229, a1230) {
                let a1231 = v0;
                let a1232 = 2u64;
                if nat_eq(a1231, a1232) {
                    Ok(f92(f127(), f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1233 = v0;
                let a1234 = 3u64;
                if nat_eq(a1233, a1234) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1235 = v0;
        let a1236 = 6u64;
        if nat_lt(a1235, a1236) {
            let a1237 = v0;
            let a1238 = 5u64;
            if nat_lt(a1237, a1238) {
                let a1239 = v0;
                let a1240 = 4u64;
                if nat_eq(a1239, a1240) {
                    Ok(f80(2u64, f97(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1241 = v0;
                let a1242 = 5u64;
                if nat_eq(a1241, a1242) {
                    Ok(f80(1u64, f93(), f112(f111())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1243 = v0;
            let a1244 = 7u64;
            if nat_lt(a1243, a1244) {
                let a1245 = v0;
                let a1246 = 6u64;
                if nat_eq(a1245, a1246) {
                    Ok(f80(2u64, f128(f111())?, f93(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1247 = v0;
                let a1248 = 7u64;
                if nat_eq(a1247, a1248) {
                    Ok(f80(2u64, f95(), f93(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1249 = v0;
    let a1250 = 4u64;
    if nat_lt(a1249, a1250) {
        let a1251 = v0;
        let a1252 = 2u64;
        if nat_lt(a1251, a1252) {
            let a1253 = v0;
            let a1254 = 1u64;
            if nat_lt(a1253, a1254) {
                let a1255 = v0;
                let a1256 = 0u64;
                if nat_eq(a1255, a1256) {
                    f80(0u64, f124(), f95(), f124())
                } else {
                    f2()
                }
            } else {
                let a1257 = v0;
                let a1258 = 1u64;
                if nat_eq(a1257, a1258) {
                    f90(0u64, f109(), 1u64, f109())
                } else {
                    f2()
                }
            }
        } else {
            let a1259 = v0;
            let a1260 = 3u64;
            if nat_lt(a1259, a1260) {
                let a1261 = v0;
                let a1262 = 2u64;
                if nat_eq(a1261, a1262) {
                    f91(1u64, f109(), 8u64, 1575u64)
                } else {
                    f2()
                }
            } else {
                let a1263 = v0;
                let a1264 = 3u64;
                if nat_eq(a1263, a1264) {
                    f91(2u64, f124(), f121(), 1589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1265 = v0;
        let a1266 = 6u64;
        if nat_lt(a1265, a1266) {
            let a1267 = v0;
            let a1268 = 5u64;
            if nat_lt(a1267, a1268) {
                let a1269 = v0;
                let a1270 = 4u64;
                if nat_eq(a1269, a1270) {
                    f99(1591u64)
                } else {
                    f2()
                }
            } else {
                let a1271 = v0;
                let a1272 = 5u64;
                if nat_eq(a1271, a1272) {
                    f83(f124(), f121())
                } else {
                    f2()
                }
            }
        } else {
            let a1273 = v0;
            let a1274 = 7u64;
            if nat_lt(a1273, a1274) {
                let a1275 = v0;
                let a1276 = 6u64;
                if nat_eq(a1275, a1276) {
                    f83(f123(), f100())
                } else {
                    f2()
                }
            } else {
                let a1277 = v0;
                let a1278 = 7u64;
                if nat_eq(a1277, a1278) {
                    f90(0u64, f122(), 1u64, f122())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1279 = v0;
    let a1280 = 1u64;
    if nat_lt(a1279, a1280) {
        let a1281 = v0;
        let a1282 = 0u64;
        if nat_eq(a1281, a1282) {
            f91(1u64, f122(), f119(), 1572u64)
        } else {
            f2()
        }
    } else {
        let a1283 = v0;
        let a1284 = 1u64;
        if nat_eq(a1283, a1284) {
            f129(f98())
        } else {
            f2()
        }
    }
}

pub fn f35(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1285 = v0;
    let a1286 = 4u64;
    if nat_lt(a1285, a1286) {
        let a1287 = v0;
        let a1288 = 2u64;
        if nat_lt(a1287, a1288) {
            let a1289 = v0;
            let a1290 = 1u64;
            if nat_lt(a1289, a1290) {
                let a1291 = v0;
                let a1292 = 0u64;
                if nat_eq(a1291, a1292) {
                    Ok(f80(3u64, 1082130432u64, f81(), f96()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1293 = v0;
                let a1294 = 1u64;
                if nat_eq(a1293, a1294) {
                    Ok(f80(3u64, 1065353216u64, f96(), f97()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1295 = v0;
            let a1296 = 3u64;
            if nat_lt(a1295, a1296) {
                let a1297 = v0;
                let a1298 = 2u64;
                if nat_eq(a1297, a1298) {
                    Ok(f83(0u64, f124()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1299 = v0;
                let a1300 = 3u64;
                if nat_eq(a1299, a1300) {
                    Ok(f83(0u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1301 = v0;
        let a1302 = 6u64;
        if nat_lt(a1301, a1302) {
            let a1303 = v0;
            let a1304 = 5u64;
            if nat_lt(a1303, a1304) {
                let a1305 = v0;
                let a1306 = 4u64;
                if nat_eq(a1305, a1306) {
                    Ok(f80(2u64, f96(), f112(f111())?, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1307 = v0;
                let a1308 = 5u64;
                if nat_eq(a1307, a1308) {
                    Ok(f80(0u64, f95(), 1262485504u64, f130()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1309 = v0;
            let a1310 = 7u64;
            if nat_lt(a1309, a1310) {
                let a1311 = v0;
                let a1312 = 6u64;
                if nat_eq(a1311, a1312) {
                    Ok(f80(1u64, f130(), 1262485504u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1313 = v0;
                let a1314 = 7u64;
                if nat_eq(a1313, a1314) {
                    Ok(f90(4u64, f130(), 256u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f36(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1315 = v0;
    let a1316 = 4u64;
    if nat_lt(a1315, a1316) {
        let a1317 = v0;
        let a1318 = 2u64;
        if nat_lt(a1317, a1318) {
            let a1319 = v0;
            let a1320 = 1u64;
            if nat_lt(a1319, a1320) {
                let a1321 = v0;
                let a1322 = 0u64;
                if nat_eq(a1321, a1322) {
                    Ok(f92(f95(), f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1323 = v0;
                let a1324 = 1u64;
                if nat_eq(a1323, a1324) {
                    Ok(f80(2u64, f97(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1325 = v0;
            let a1326 = 3u64;
            if nat_lt(a1325, a1326) {
                let a1327 = v0;
                let a1328 = 2u64;
                if nat_eq(a1327, a1328) {
                    Ok(f80(1u64, f112(f111())?, f95(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1329 = v0;
                let a1330 = 3u64;
                if nat_eq(a1329, a1330) {
                    Ok(f80(2u64, f131(f111())?, f95(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1331 = v0;
        let a1332 = 6u64;
        if nat_lt(a1331, a1332) {
            let a1333 = v0;
            let a1334 = 5u64;
            if nat_lt(a1333, a1334) {
                let a1335 = v0;
                let a1336 = 4u64;
                if nat_eq(a1335, a1336) {
                    Ok(f80(2u64, f93(), f95(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1337 = v0;
                let a1338 = 5u64;
                if nat_eq(a1337, a1338) {
                    Ok(f80(0u64, f124(), f93(), f124()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1339 = v0;
            let a1340 = 7u64;
            if nat_lt(a1339, a1340) {
                let a1341 = v0;
                let a1342 = 6u64;
                if nat_eq(a1341, a1342) {
                    Ok(f90(0u64, f111(), 1u64, f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1343 = v0;
                let a1344 = 7u64;
                if nat_eq(a1343, a1344) {
                    Ok(f91(1u64, f111(), 32u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f37(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1345 = v0;
    let a1346 = 4u64;
    if nat_lt(a1345, a1346) {
        let a1347 = v0;
        let a1348 = 2u64;
        if nat_lt(a1347, a1348) {
            let a1349 = v0;
            let a1350 = 1u64;
            if nat_lt(a1349, a1350) {
                let a1351 = v0;
                let a1352 = 0u64;
                if nat_eq(a1351, a1352) {
                    f83(f124(), f121())
                } else {
                    f2()
                }
            } else {
                let a1353 = v0;
                let a1354 = 1u64;
                if nat_eq(a1353, a1354) {
                    f83(f96(), f132())
                } else {
                    f2()
                }
            }
        } else {
            let a1355 = v0;
            let a1356 = 3u64;
            if nat_lt(a1355, a1356) {
                let a1357 = v0;
                let a1358 = 2u64;
                if nat_eq(a1357, a1358) {
                    f83(0u64, f85())
                } else {
                    f2()
                }
            } else {
                let a1359 = v0;
                let a1360 = 3u64;
                if nat_eq(a1359, a1360) {
                    f91(0u64, f85(), 4u64, 1070u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1361 = v0;
        let a1362 = 6u64;
        if nat_lt(a1361, a1362) {
            let a1363 = v0;
            let a1364 = 5u64;
            if nat_lt(a1363, a1364) {
                let a1365 = v0;
                let a1366 = 4u64;
                if nat_eq(a1365, a1366) {
                    f92(f85(), f93())
                } else {
                    f2()
                }
            } else {
                let a1367 = v0;
                let a1368 = 5u64;
                if nat_eq(a1367, a1368) {
                    f80(1u64, f93(), 1082130432u64, f93())
                } else {
                    f2()
                }
            }
        } else {
            let a1369 = v0;
            let a1370 = 7u64;
            if nat_lt(a1369, a1370) {
                let a1371 = v0;
                let a1372 = 6u64;
                if nat_eq(a1371, a1372) {
                    f80(2u64, f93(), 1036831949u64, f93())
                } else {
                    f2()
                }
            } else {
                let a1373 = v0;
                let a1374 = 7u64;
                if nat_eq(a1373, a1374) {
                    f80(0u64, f93(), 1082130432u64, f93())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f38(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1375 = v0;
    let a1376 = 4u64;
    if nat_lt(a1375, a1376) {
        let a1377 = v0;
        let a1378 = 2u64;
        if nat_lt(a1377, a1378) {
            let a1379 = v0;
            let a1380 = 1u64;
            if nat_lt(a1379, a1380) {
                let a1381 = v0;
                let a1382 = 0u64;
                if nat_eq(a1381, a1382) {
                    Ok(f80(3u64, f93(), f81(), f96()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1383 = v0;
                let a1384 = 1u64;
                if nat_eq(a1383, a1384) {
                    Ok(f80(3u64, 1065353216u64, f96(), f97()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1385 = v0;
            let a1386 = 3u64;
            if nat_lt(a1385, a1386) {
                let a1387 = v0;
                let a1388 = 2u64;
                if nat_eq(a1387, a1388) {
                    Ok(f83(0u64, f124()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1389 = v0;
                let a1390 = 3u64;
                if nat_eq(a1389, a1390) {
                    Ok(f83(0u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1391 = v0;
        let a1392 = 6u64;
        if nat_lt(a1391, a1392) {
            let a1393 = v0;
            let a1394 = 5u64;
            if nat_lt(a1393, a1394) {
                let a1395 = v0;
                let a1396 = 4u64;
                if nat_eq(a1395, a1396) {
                    Ok(f80(2u64, f96(), f112(f111())?, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1397 = v0;
                let a1398 = 5u64;
                if nat_eq(a1397, a1398) {
                    Ok(f80(0u64, f95(), 1262485504u64, f130()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1399 = v0;
            let a1400 = 7u64;
            if nat_lt(a1399, a1400) {
                let a1401 = v0;
                let a1402 = 6u64;
                if nat_eq(a1401, a1402) {
                    Ok(f80(1u64, f130(), 1262485504u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1403 = v0;
                let a1404 = 7u64;
                if nat_eq(a1403, a1404) {
                    Ok(f91(2u64, 1082130432u64, f127(), 1057u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f39(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1405 = v0;
    let a1406 = 4u64;
    if nat_lt(a1405, a1406) {
        let a1407 = v0;
        let a1408 = 2u64;
        if nat_lt(a1407, a1408) {
            let a1409 = v0;
            let a1410 = 1u64;
            if nat_lt(a1409, a1410) {
                let a1411 = v0;
                let a1412 = 0u64;
                if nat_eq(a1411, a1412) {
                    Ok(f99(1059u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1413 = v0;
                let a1414 = 1u64;
                if nat_eq(a1413, a1414) {
                    Ok(f83(1082130432u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1415 = v0;
            let a1416 = 3u64;
            if nat_lt(a1415, a1416) {
                let a1417 = v0;
                let a1418 = 2u64;
                if nat_eq(a1417, a1418) {
                    Ok(f83(4u64, f130()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1419 = v0;
                let a1420 = 3u64;
                if nat_eq(a1419, a1420) {
                    Ok(f80(2u64, f97(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1421 = v0;
        let a1422 = 6u64;
        if nat_lt(a1421, a1422) {
            let a1423 = v0;
            let a1424 = 5u64;
            if nat_lt(a1423, a1424) {
                let a1425 = v0;
                let a1426 = 4u64;
                if nat_eq(a1425, a1426) {
                    Ok(f80(1u64, f112(f111())?, f95(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1427 = v0;
                let a1428 = 5u64;
                if nat_eq(a1427, a1428) {
                    Ok(f80(2u64, f131(f111())?, f95(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1429 = v0;
            let a1430 = 7u64;
            if nat_lt(a1429, a1430) {
                let a1431 = v0;
                let a1432 = 6u64;
                if nat_eq(a1431, a1432) {
                    Ok(f80(2u64, f93(), f95(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1433 = v0;
                let a1434 = 7u64;
                if nat_eq(a1433, a1434) {
                    Ok(f80(0u64, f124(), f93(), f124()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f40(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1435 = v0;
    let a1436 = 4u64;
    if nat_lt(a1435, a1436) {
        let a1437 = v0;
        let a1438 = 2u64;
        if nat_lt(a1437, a1438) {
            let a1439 = v0;
            let a1440 = 1u64;
            if nat_lt(a1439, a1440) {
                let a1441 = v0;
                let a1442 = 0u64;
                if nat_eq(a1441, a1442) {
                    f90(0u64, f111(), 1u64, f111())
                } else {
                    f2()
                }
            } else {
                let a1443 = v0;
                let a1444 = 1u64;
                if nat_eq(a1443, a1444) {
                    f91(1u64, f111(), 32u64, 1052u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1445 = v0;
            let a1446 = 3u64;
            if nat_lt(a1445, a1446) {
                let a1447 = v0;
                let a1448 = 2u64;
                if nat_eq(a1447, a1448) {
                    f91(2u64, f124(), f121(), 1068u64)
                } else {
                    f2()
                }
            } else {
                let a1449 = v0;
                let a1450 = 3u64;
                if nat_eq(a1449, a1450) {
                    f99(1070u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1451 = v0;
        let a1452 = 6u64;
        if nat_lt(a1451, a1452) {
            let a1453 = v0;
            let a1454 = 5u64;
            if nat_lt(a1453, a1454) {
                let a1455 = v0;
                let a1456 = 4u64;
                if nat_eq(a1455, a1456) {
                    f83(f124(), f121())
                } else {
                    f2()
                }
            } else {
                let a1457 = v0;
                let a1458 = 5u64;
                if nat_eq(a1457, a1458) {
                    f83(f96(), f132())
                } else {
                    f2()
                }
            }
        } else {
            let a1459 = v0;
            let a1460 = 7u64;
            if nat_lt(a1459, a1460) {
                let a1461 = v0;
                let a1462 = 6u64;
                if nat_eq(a1461, a1462) {
                    f90(0u64, f85(), 1u64, f85())
                } else {
                    f2()
                }
            } else {
                let a1463 = v0;
                let a1464 = 7u64;
                if nat_eq(a1463, a1464) {
                    f91(1u64, f85(), 9u64, 1043u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f41(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1465 = v0;
    let a1466 = 4u64;
    if nat_lt(a1465, a1466) {
        let a1467 = v0;
        let a1468 = 2u64;
        if nat_lt(a1467, a1468) {
            let a1469 = v0;
            let a1470 = 1u64;
            if nat_lt(a1469, a1470) {
                let a1471 = v0;
                let a1472 = 0u64;
                if nat_eq(a1471, a1472) {
                    Ok(f83(0u64, f107()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1473 = v0;
                let a1474 = 1u64;
                if nat_eq(a1473, a1474) {
                    Ok(f83(0u64, f106()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1475 = v0;
            let a1476 = 3u64;
            if nat_lt(a1475, a1476) {
                let a1477 = v0;
                let a1478 = 2u64;
                if nat_eq(a1477, a1478) {
                    Ok(f83(0u64, f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1479 = v0;
                let a1480 = 3u64;
                if nat_eq(a1479, a1480) {
                    Ok(f80(2u64, f132(), f112(f111())?, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1481 = v0;
        let a1482 = 6u64;
        if nat_lt(a1481, a1482) {
            let a1483 = v0;
            let a1484 = 5u64;
            if nat_lt(a1483, a1484) {
                let a1485 = v0;
                let a1486 = 4u64;
                if nat_eq(a1485, a1486) {
                    Ok(f80(0u64, f95(), 1262485504u64, f130()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1487 = v0;
                let a1488 = 5u64;
                if nat_eq(a1487, a1488) {
                    Ok(f80(1u64, f130(), 1262485504u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1489 = v0;
            let a1490 = 7u64;
            if nat_lt(a1489, a1490) {
                let a1491 = v0;
                let a1492 = 6u64;
                if nat_eq(a1491, a1492) {
                    Ok(f91(2u64, 1082130432u64, f127(), 1080u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1493 = v0;
                let a1494 = 7u64;
                if nat_eq(a1493, a1494) {
                    Ok(f99(1082u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f42(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1495 = v0;
    let a1496 = 4u64;
    if nat_lt(a1495, a1496) {
        let a1497 = v0;
        let a1498 = 2u64;
        if nat_lt(a1497, a1498) {
            let a1499 = v0;
            let a1500 = 1u64;
            if nat_lt(a1499, a1500) {
                let a1501 = v0;
                let a1502 = 0u64;
                if nat_eq(a1501, a1502) {
                    Ok(f83(1082130432u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1503 = v0;
                let a1504 = 1u64;
                if nat_eq(a1503, a1504) {
                    Ok(f83(4u64, f130()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1505 = v0;
            let a1506 = 3u64;
            if nat_lt(a1505, a1506) {
                let a1507 = v0;
                let a1508 = 2u64;
                if nat_eq(a1507, a1508) {
                    Ok(f90(4u64, f130(), 256u64, f133(f111())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1509 = v0;
                let a1510 = 3u64;
                if nat_eq(a1509, a1510) {
                    Ok(f80(2u64, f131(f111())?, f112(f111())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1511 = v0;
        let a1512 = 6u64;
        if nat_lt(a1511, a1512) {
            let a1513 = v0;
            let a1514 = 5u64;
            if nat_lt(a1513, a1514) {
                let a1515 = v0;
                let a1516 = 4u64;
                if nat_eq(a1515, a1516) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1517 = v0;
                let a1518 = 5u64;
                if nat_eq(a1517, a1518) {
                    Ok(f80(0u64, f107(), f93(), f107()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1519 = v0;
            let a1520 = 7u64;
            if nat_lt(a1519, a1520) {
                let a1521 = v0;
                let a1522 = 6u64;
                if nat_eq(a1521, a1522) {
                    Ok(f80(2u64, f131(f111())?, f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1523 = v0;
                let a1524 = 7u64;
                if nat_eq(a1523, a1524) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f43(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1525 = v0;
    let a1526 = 4u64;
    if nat_lt(a1525, a1526) {
        let a1527 = v0;
        let a1528 = 2u64;
        if nat_lt(a1527, a1528) {
            let a1529 = v0;
            let a1530 = 1u64;
            if nat_lt(a1529, a1530) {
                let a1531 = v0;
                let a1532 = 0u64;
                if nat_eq(a1531, a1532) {
                    Ok(f80(0u64, f106(), f93(), f106()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1533 = v0;
                let a1534 = 1u64;
                if nat_eq(a1533, a1534) {
                    Ok(f90(0u64, f111(), 1u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1535 = v0;
            let a1536 = 3u64;
            if nat_lt(a1535, a1536) {
                let a1537 = v0;
                let a1538 = 2u64;
                if nat_eq(a1537, a1538) {
                    Ok(f91(1u64, f111(), 32u64, 1075u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1539 = v0;
                let a1540 = 3u64;
                if nat_eq(a1539, a1540) {
                    Ok(f83(0u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1541 = v0;
        let a1542 = 6u64;
        if nat_lt(a1541, a1542) {
            let a1543 = v0;
            let a1544 = 5u64;
            if nat_lt(a1543, a1544) {
                let a1545 = v0;
                let a1546 = 4u64;
                if nat_eq(a1545, a1546) {
                    Ok(f83(0u64, f134()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1547 = v0;
                let a1548 = 5u64;
                if nat_eq(a1547, a1548) {
                    Ok(f83(0u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1549 = v0;
            let a1550 = 7u64;
            if nat_lt(a1549, a1550) {
                let a1551 = v0;
                let a1552 = 6u64;
                if nat_eq(a1551, a1552) {
                    Ok(f92(f133(f111())?, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1553 = v0;
                let a1554 = 7u64;
                if nat_eq(a1553, a1554) {
                    Ok(f83(f127(), f135()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f44(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1555 = v0;
    let a1556 = 4u64;
    if nat_lt(a1555, a1556) {
        let a1557 = v0;
        let a1558 = 2u64;
        if nat_lt(a1557, a1558) {
            let a1559 = v0;
            let a1560 = 1u64;
            if nat_lt(a1559, a1560) {
                let a1561 = v0;
                let a1562 = 0u64;
                if nat_eq(a1561, a1562) {
                    Ok(f83(f107(), f136()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1563 = v0;
                let a1564 = 1u64;
                if nat_eq(a1563, a1564) {
                    Ok(f83(f106(), f137()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1565 = v0;
            let a1566 = 3u64;
            if nat_lt(a1565, a1566) {
                let a1567 = v0;
                let a1568 = 2u64;
                if nat_eq(a1567, a1568) {
                    Ok(f80(2u64, f131(f111())?, f112(f111())?, f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1569 = v0;
                let a1570 = 3u64;
                if nat_eq(a1569, a1570) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1571 = v0;
        let a1572 = 6u64;
        if nat_lt(a1571, a1572) {
            let a1573 = v0;
            let a1574 = 5u64;
            if nat_lt(a1573, a1574) {
                let a1575 = v0;
                let a1576 = 4u64;
                if nat_eq(a1575, a1576) {
                    Ok(f80(1u64, f136(), f93(), f136()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1577 = v0;
                let a1578 = 5u64;
                if nat_eq(a1577, a1578) {
                    Ok(f80(2u64, f131(f111())?, f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1579 = v0;
            let a1580 = 7u64;
            if nat_lt(a1579, a1580) {
                let a1581 = v0;
                let a1582 = 6u64;
                if nat_eq(a1581, a1582) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1583 = v0;
                let a1584 = 7u64;
                if nat_eq(a1583, a1584) {
                    Ok(f80(1u64, f137(), f93(), f137()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f45(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1585 = v0;
    let a1586 = 4u64;
    if nat_lt(a1585, a1586) {
        let a1587 = v0;
        let a1588 = 2u64;
        if nat_lt(a1587, a1588) {
            let a1589 = v0;
            let a1590 = 1u64;
            if nat_lt(a1589, a1590) {
                let a1591 = v0;
                let a1592 = 0u64;
                if nat_eq(a1591, a1592) {
                    Ok(f91(2u64, 0u64, f136(), 1106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1593 = v0;
                let a1594 = 1u64;
                if nat_eq(a1593, a1594) {
                    Ok(f99(1133u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1595 = v0;
            let a1596 = 3u64;
            if nat_lt(a1595, a1596) {
                let a1597 = v0;
                let a1598 = 2u64;
                if nat_eq(a1597, a1598) {
                    Ok(f91(2u64, 0u64, f137(), 1108u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1599 = v0;
                let a1600 = 3u64;
                if nat_eq(a1599, a1600) {
                    Ok(f99(1133u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1601 = v0;
        let a1602 = 6u64;
        if nat_lt(a1601, a1602) {
            let a1603 = v0;
            let a1604 = 5u64;
            if nat_lt(a1603, a1604) {
                let a1605 = v0;
                let a1606 = 4u64;
                if nat_eq(a1605, a1606) {
                    Ok(f80(2u64, f112(f111())?, f137(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1607 = v0;
                let a1608 = 5u64;
                if nat_eq(a1607, a1608) {
                    Ok(f80(3u64, f95(), f136(), f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1609 = v0;
            let a1610 = 7u64;
            if nat_lt(a1609, a1610) {
                let a1611 = v0;
                let a1612 = 6u64;
                if nat_eq(a1611, a1612) {
                    Ok(f80(0u64, f95(), 1262485504u64, f130()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1613 = v0;
                let a1614 = 7u64;
                if nat_eq(a1613, a1614) {
                    Ok(f80(1u64, f130(), 1262485504u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f46(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1615 = v0;
    let a1616 = 4u64;
    if nat_lt(a1615, a1616) {
        let a1617 = v0;
        let a1618 = 2u64;
        if nat_lt(a1617, a1618) {
            let a1619 = v0;
            let a1620 = 1u64;
            if nat_lt(a1619, a1620) {
                let a1621 = v0;
                let a1622 = 0u64;
                if nat_eq(a1621, a1622) {
                    Ok(f91(2u64, 1082130432u64, f127(), 1114u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1623 = v0;
                let a1624 = 1u64;
                if nat_eq(a1623, a1624) {
                    Ok(f99(1116u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1625 = v0;
            let a1626 = 3u64;
            if nat_lt(a1625, a1626) {
                let a1627 = v0;
                let a1628 = 2u64;
                if nat_eq(a1627, a1628) {
                    Ok(f83(1082130432u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1629 = v0;
                let a1630 = 3u64;
                if nat_eq(a1629, a1630) {
                    Ok(f83(4u64, f130()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1631 = v0;
        let a1632 = 6u64;
        if nat_lt(a1631, a1632) {
            let a1633 = v0;
            let a1634 = 5u64;
            if nat_lt(a1633, a1634) {
                let a1635 = v0;
                let a1636 = 4u64;
                if nat_eq(a1635, a1636) {
                    Ok(f91(0u64, f127(), f135(), 1133u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1637 = v0;
                let a1638 = 5u64;
                if nat_eq(a1637, a1638) {
                    Ok(f80(2u64, f131(f111())?, f112(f111())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1639 = v0;
            let a1640 = 7u64;
            if nat_lt(a1639, a1640) {
                let a1641 = v0;
                let a1642 = 6u64;
                if nat_eq(a1641, a1642) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1643 = v0;
                let a1644 = 7u64;
                if nat_eq(a1643, a1644) {
                    Ok(f80(0u64, f136(), f93(), f136()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f47(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1645 = v0;
    let a1646 = 4u64;
    if nat_lt(a1645, a1646) {
        let a1647 = v0;
        let a1648 = 2u64;
        if nat_lt(a1647, a1648) {
            let a1649 = v0;
            let a1650 = 1u64;
            if nat_lt(a1649, a1650) {
                let a1651 = v0;
                let a1652 = 0u64;
                if nat_eq(a1651, a1652) {
                    Ok(f80(2u64, f131(f111())?, f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1653 = v0;
                let a1654 = 1u64;
                if nat_eq(a1653, a1654) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1655 = v0;
            let a1656 = 3u64;
            if nat_lt(a1655, a1656) {
                let a1657 = v0;
                let a1658 = 2u64;
                if nat_eq(a1657, a1658) {
                    Ok(f80(0u64, f137(), f93(), f137()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1659 = v0;
                let a1660 = 3u64;
                if nat_eq(a1659, a1660) {
                    Ok(f80(2u64, f136(), f136(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1661 = v0;
        let a1662 = 6u64;
        if nat_lt(a1661, a1662) {
            let a1663 = v0;
            let a1664 = 5u64;
            if nat_lt(a1663, a1664) {
                let a1665 = v0;
                let a1666 = 4u64;
                if nat_eq(a1665, a1666) {
                    Ok(f80(2u64, f93(), f106(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1667 = v0;
                let a1668 = 5u64;
                if nat_eq(a1667, a1668) {
                    Ok(f80(2u64, f107(), f107(), f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1669 = v0;
            let a1670 = 7u64;
            if nat_lt(a1669, a1670) {
                let a1671 = v0;
                let a1672 = 6u64;
                if nat_eq(a1671, a1672) {
                    Ok(f80(2u64, f95(), f137(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1673 = v0;
                let a1674 = 7u64;
                if nat_eq(a1673, a1674) {
                    Ok(f91(2u64, f95(), f93(), 1129u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f48(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1675 = v0;
    let a1676 = 4u64;
    if nat_lt(a1675, a1676) {
        let a1677 = v0;
        let a1678 = 2u64;
        if nat_lt(a1677, a1678) {
            let a1679 = v0;
            let a1680 = 1u64;
            if nat_lt(a1679, a1680) {
                let a1681 = v0;
                let a1682 = 0u64;
                if nat_eq(a1681, a1682) {
                    Ok(f99(1133u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1683 = v0;
                let a1684 = 1u64;
                if nat_eq(a1683, a1684) {
                    Ok(f90(4u64, f130(), 256u64, f133(f111())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1685 = v0;
            let a1686 = 3u64;
            if nat_lt(a1685, a1686) {
                let a1687 = v0;
                let a1688 = 2u64;
                if nat_eq(a1687, a1688) {
                    Ok(f83(f136(), f107()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1689 = v0;
                let a1690 = 3u64;
                if nat_eq(a1689, a1690) {
                    Ok(f83(f137(), f106()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1691 = v0;
        let a1692 = 6u64;
        if nat_lt(a1691, a1692) {
            let a1693 = v0;
            let a1694 = 5u64;
            if nat_lt(a1693, a1694) {
                let a1695 = v0;
                let a1696 = 4u64;
                if nat_eq(a1695, a1696) {
                    Ok(f83(1u64, f134()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1697 = v0;
                let a1698 = 5u64;
                if nat_eq(a1697, a1698) {
                    Ok(f90(0u64, f111(), 1u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1699 = v0;
            let a1700 = 7u64;
            if nat_lt(a1699, a1700) {
                let a1701 = v0;
                let a1702 = 6u64;
                if nat_eq(a1701, a1702) {
                    Ok(f91(1u64, f111(), 32u64, 1094u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1703 = v0;
                let a1704 = 7u64;
                if nat_eq(a1703, a1704) {
                    Ok(f91(0u64, f134(), 0u64, 1138u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f49(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1705 = v0;
    let a1706 = 4u64;
    if nat_lt(a1705, a1706) {
        let a1707 = v0;
        let a1708 = 2u64;
        if nat_lt(a1707, a1708) {
            let a1709 = v0;
            let a1710 = 1u64;
            if nat_lt(a1709, a1710) {
                let a1711 = v0;
                let a1712 = 0u64;
                if nat_eq(a1711, a1712) {
                    f90(0u64, f85(), 1u64, f85())
                } else {
                    f2()
                }
            } else {
                let a1713 = v0;
                let a1714 = 1u64;
                if nat_eq(a1713, a1714) {
                    f91(1u64, f85(), 5u64, 1092u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1715 = v0;
            let a1716 = 3u64;
            if nat_lt(a1715, a1716) {
                let a1717 = v0;
                let a1718 = 2u64;
                if nat_eq(a1717, a1718) {
                    f91(2u64, 0u64, f106(), 1140u64)
                } else {
                    f2()
                }
            } else {
                let a1719 = v0;
                let a1720 = 3u64;
                if nat_eq(a1719, a1720) {
                    f99(3091u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1721 = v0;
        let a1722 = 6u64;
        if nat_lt(a1721, a1722) {
            let a1723 = v0;
            let a1724 = 5u64;
            if nat_lt(a1723, a1724) {
                let a1725 = v0;
                let a1726 = 4u64;
                if nat_eq(a1725, a1726) {
                    f80(3u64, f107(), f106(), f82())
                } else {
                    f2()
                }
            } else {
                let a1727 = v0;
                let a1728 = 5u64;
                if nat_eq(a1727, a1728) {
                    f80(2u64, f82(), 1077936128u64, f81())
                } else {
                    f2()
                }
            }
        } else {
            let a1729 = v0;
            let a1730 = 7u64;
            if nat_lt(a1729, a1730) {
                let a1731 = v0;
                let a1732 = 6u64;
                if nat_eq(a1731, a1732) {
                    f91(2u64, 0u64, f81(), 1144u64)
                } else {
                    f2()
                }
            } else {
                let a1733 = v0;
                let a1734 = 7u64;
                if nat_eq(a1733, a1734) {
                    f99(3091u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f50(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1735 = v0;
    let a1736 = 4u64;
    if nat_lt(a1735, a1736) {
        let a1737 = v0;
        let a1738 = 2u64;
        if nat_lt(a1737, a1738) {
            let a1739 = v0;
            let a1740 = 1u64;
            if nat_lt(a1739, a1740) {
                let a1741 = v0;
                let a1742 = 0u64;
                if nat_eq(a1741, a1742) {
                    Ok(f83(0u64, f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1743 = v0;
                let a1744 = 1u64;
                if nat_eq(a1743, a1744) {
                    Ok(f83(0u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1745 = v0;
            let a1746 = 3u64;
            if nat_lt(a1745, a1746) {
                let a1747 = v0;
                let a1748 = 2u64;
                if nat_eq(a1747, a1748) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1749 = v0;
                let a1750 = 3u64;
                if nat_eq(a1749, a1750) {
                    Ok(f83(0u64, f87(f86())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1751 = v0;
        let a1752 = 6u64;
        if nat_lt(a1751, a1752) {
            let a1753 = v0;
            let a1754 = 5u64;
            if nat_lt(a1753, a1754) {
                let a1755 = v0;
                let a1756 = 4u64;
                if nat_eq(a1755, a1756) {
                    Ok(f83(1u64, f89(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1757 = v0;
                let a1758 = 5u64;
                if nat_eq(a1757, a1758) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1759 = v0;
            let a1760 = 7u64;
            if nat_lt(a1759, a1760) {
                let a1761 = v0;
                let a1762 = 6u64;
                if nat_eq(a1761, a1762) {
                    Ok(f91(1u64, f86(), 4u64, 1147u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1763 = v0;
                let a1764 = 7u64;
                if nat_eq(a1763, a1764) {
                    Ok(f92(f85(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f51(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1765 = v0;
    let a1766 = 4u64;
    if nat_lt(a1765, a1766) {
        let a1767 = v0;
        let a1768 = 2u64;
        if nat_lt(a1767, a1768) {
            let a1769 = v0;
            let a1770 = 1u64;
            if nat_lt(a1769, a1770) {
                let a1771 = v0;
                let a1772 = 0u64;
                if nat_eq(a1771, a1772) {
                    f92(f94(), f95())
                } else {
                    f2()
                }
            } else {
                let a1773 = v0;
                let a1774 = 1u64;
                if nat_eq(a1773, a1774) {
                    f80(1u64, f93(), f95(), f93())
                } else {
                    f2()
                }
            }
        } else {
            let a1775 = v0;
            let a1776 = 3u64;
            if nat_lt(a1775, a1776) {
                let a1777 = v0;
                let a1778 = 2u64;
                if nat_eq(a1777, a1778) {
                    f80(2u64, f93(), 1036831949u64, f93())
                } else {
                    f2()
                }
            } else {
                let a1779 = v0;
                let a1780 = 3u64;
                if nat_eq(a1779, a1780) {
                    f80(0u64, 1084227584u64, f93(), f93())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1781 = v0;
        let a1782 = 6u64;
        if nat_lt(a1781, a1782) {
            let a1783 = v0;
            let a1784 = 5u64;
            if nat_lt(a1783, a1784) {
                let a1785 = v0;
                let a1786 = 4u64;
                if nat_eq(a1785, a1786) {
                    f80(3u64, f93(), f81(), f96())
                } else {
                    f2()
                }
            } else {
                let a1787 = v0;
                let a1788 = 5u64;
                if nat_eq(a1787, a1788) {
                    f80(3u64, 1065353216u64, f96(), f97())
                } else {
                    f2()
                }
            }
        } else {
            let a1789 = v0;
            let a1790 = 7u64;
            if nat_lt(a1789, a1790) {
                let a1791 = v0;
                let a1792 = 6u64;
                if nat_eq(a1791, a1792) {
                    f83(0u64, f86())
                } else {
                    f2()
                }
            } else {
                let a1793 = v0;
                let a1794 = 7u64;
                if nat_eq(a1793, a1794) {
                    f83(1161u64, f98())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f52(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1795 = v0;
    let a1796 = 4u64;
    if nat_lt(a1795, a1796) {
        let a1797 = v0;
        let a1798 = 2u64;
        if nat_lt(a1797, a1798) {
            let a1799 = v0;
            let a1800 = 1u64;
            if nat_lt(a1799, a1800) {
                let a1801 = v0;
                let a1802 = 0u64;
                if nat_eq(a1801, a1802) {
                    Ok(f99(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1803 = v0;
                let a1804 = 1u64;
                if nat_eq(a1803, a1804) {
                    Ok(f83(f100(), f101(f86())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1805 = v0;
            let a1806 = 3u64;
            if nat_lt(a1805, a1806) {
                let a1807 = v0;
                let a1808 = 2u64;
                if nat_eq(a1807, a1808) {
                    Ok(f83(f102(), f103(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1809 = v0;
                let a1810 = 3u64;
                if nat_eq(a1809, a1810) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1811 = v0;
        let a1812 = 6u64;
        if nat_lt(a1811, a1812) {
            let a1813 = v0;
            let a1814 = 5u64;
            if nat_lt(a1813, a1814) {
                let a1815 = v0;
                let a1816 = 4u64;
                if nat_eq(a1815, a1816) {
                    Ok(f91(1u64, f86(), 4u64, 1159u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1817 = v0;
                let a1818 = 5u64;
                if nat_eq(a1817, a1818) {
                    Ok(f83(0u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1819 = v0;
            let a1820 = 7u64;
            if nat_lt(a1819, a1820) {
                let a1821 = v0;
                let a1822 = 6u64;
                if nat_eq(a1821, a1822) {
                    Ok(f83(1168u64, f105()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1823 = v0;
                let a1824 = 7u64;
                if nat_eq(a1823, a1824) {
                    Ok(f99(2048u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f53(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1825 = v0;
    let a1826 = 4u64;
    if nat_lt(a1825, a1826) {
        let a1827 = v0;
        let a1828 = 2u64;
        if nat_lt(a1827, a1828) {
            let a1829 = v0;
            let a1830 = 1u64;
            if nat_lt(a1829, a1830) {
                let a1831 = v0;
                let a1832 = 0u64;
                if nat_eq(a1831, a1832) {
                    f91(2u64, 0u64, f106(), 1170u64)
                } else {
                    f2()
                }
            } else {
                let a1833 = v0;
                let a1834 = 1u64;
                if nat_eq(a1833, a1834) {
                    f99(1181u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1835 = v0;
            let a1836 = 3u64;
            if nat_lt(a1835, a1836) {
                let a1837 = v0;
                let a1838 = 2u64;
                if nat_eq(a1837, a1838) {
                    f80(2u64, f107(), f107(), f95())
                } else {
                    f2()
                }
            } else {
                let a1839 = v0;
                let a1840 = 3u64;
                if nat_eq(a1839, a1840) {
                    f80(2u64, f84(), f106(), f93())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1841 = v0;
        let a1842 = 6u64;
        if nat_lt(a1841, a1842) {
            let a1843 = v0;
            let a1844 = 5u64;
            if nat_lt(a1843, a1844) {
                let a1845 = v0;
                let a1846 = 4u64;
                if nat_eq(a1845, a1846) {
                    f91(2u64, f93(), f95(), 1174u64)
                } else {
                    f2()
                }
            } else {
                let a1847 = v0;
                let a1848 = 5u64;
                if nat_eq(a1847, a1848) {
                    f99(1181u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1849 = v0;
            let a1850 = 7u64;
            if nat_lt(a1849, a1850) {
                let a1851 = v0;
                let a1852 = 6u64;
                if nat_eq(a1851, a1852) {
                    f80(3u64, f107(), f106(), f82())
                } else {
                    f2()
                }
            } else {
                let a1853 = v0;
                let a1854 = 7u64;
                if nat_eq(a1853, a1854) {
                    f80(2u64, f82(), f107(), f84())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f54(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1855 = v0;
    let a1856 = 4u64;
    if nat_lt(a1855, a1856) {
        let a1857 = v0;
        let a1858 = 2u64;
        if nat_lt(a1857, a1858) {
            let a1859 = v0;
            let a1860 = 1u64;
            if nat_lt(a1859, a1860) {
                let a1861 = v0;
                let a1862 = 0u64;
                if nat_eq(a1861, a1862) {
                    Ok(f83(0u64, f86()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1863 = v0;
                let a1864 = 1u64;
                if nat_eq(a1863, a1864) {
                    Ok(f83(f101(f86())?, f87(f86())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1865 = v0;
            let a1866 = 3u64;
            if nat_lt(a1865, a1866) {
                let a1867 = v0;
                let a1868 = 2u64;
                if nat_eq(a1867, a1868) {
                    Ok(f83(f103(f86())?, f89(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1869 = v0;
                let a1870 = 3u64;
                if nat_eq(a1869, a1870) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1871 = v0;
        let a1872 = 6u64;
        if nat_lt(a1871, a1872) {
            let a1873 = v0;
            let a1874 = 5u64;
            if nat_lt(a1873, a1874) {
                let a1875 = v0;
                let a1876 = 4u64;
                if nat_eq(a1875, a1876) {
                    Ok(f91(1u64, f86(), 4u64, 1177u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1877 = v0;
                let a1878 = 5u64;
                if nat_eq(a1877, a1878) {
                    Ok(f90(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1879 = v0;
            let a1880 = 7u64;
            if nat_lt(a1879, a1880) {
                let a1881 = v0;
                let a1882 = 6u64;
                if nat_eq(a1881, a1882) {
                    Ok(f90(2u64, f94(), 2u64, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1883 = v0;
                let a1884 = 7u64;
                if nat_eq(a1883, a1884) {
                    Ok(f90(0u64, f95(), 1u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f55(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1885 = v0;
    let a1886 = 1u64;
    if nat_lt(a1885, a1886) {
        let a1887 = v0;
        let a1888 = 0u64;
        if nat_eq(a1887, a1888) {
            f91(1u64, f85(), f95(), 1151u64)
        } else {
            f2()
        }
    } else {
        let a1889 = v0;
        let a1890 = 1u64;
        if nat_eq(a1889, a1890) {
            f99(2560u64)
        } else {
            f2()
        }
    }
}

pub fn f56(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1891 = v0;
    let a1892 = 4u64;
    if nat_lt(a1891, a1892) {
        let a1893 = v0;
        let a1894 = 2u64;
        if nat_lt(a1893, a1894) {
            let a1895 = v0;
            let a1896 = 1u64;
            if nat_lt(a1895, a1896) {
                let a1897 = v0;
                let a1898 = 0u64;
                if nat_eq(a1897, a1898) {
                    Ok(f83(4286578687u64, f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1899 = v0;
                let a1900 = 1u64;
                if nat_eq(a1899, a1900) {
                    Ok(f83(0u64, f134()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1901 = v0;
            let a1902 = 3u64;
            if nat_lt(a1901, a1902) {
                let a1903 = v0;
                let a1904 = 2u64;
                if nat_eq(a1903, a1904) {
                    Ok(f83(0u64, f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1905 = v0;
                let a1906 = 3u64;
                if nat_eq(a1905, a1906) {
                    Ok(f83(f118(), f119()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1907 = v0;
        let a1908 = 6u64;
        if nat_lt(a1907, a1908) {
            let a1909 = v0;
            let a1910 = 5u64;
            if nat_lt(a1909, a1910) {
                let a1911 = v0;
                let a1912 = 4u64;
                if nat_eq(a1911, a1912) {
                    Ok(f91(0u64, f88(), 1u64, 1059u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1913 = v0;
                let a1914 = 5u64;
                if nat_eq(a1913, a1914) {
                    Ok(f83(0u64, f138()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1915 = v0;
            let a1916 = 7u64;
            if nat_lt(a1915, a1916) {
                let a1917 = v0;
                let a1918 = 6u64;
                if nat_eq(a1917, a1918) {
                    Ok(f113(1u64, 0u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1919 = v0;
                let a1920 = 7u64;
                if nat_eq(a1919, a1920) {
                    Ok(f80(1u64, f139(f118())?, f139(f140())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f57(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1921 = v0;
    let a1922 = 4u64;
    if nat_lt(a1921, a1922) {
        let a1923 = v0;
        let a1924 = 2u64;
        if nat_lt(a1923, a1924) {
            let a1925 = v0;
            let a1926 = 1u64;
            if nat_lt(a1925, a1926) {
                let a1927 = v0;
                let a1928 = 0u64;
                if nat_eq(a1927, a1928) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1929 = v0;
                let a1930 = 1u64;
                if nat_eq(a1929, a1930) {
                    Ok(f80(1u64, f141(f118())?, f141(f140())?, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1931 = v0;
            let a1932 = 3u64;
            if nat_lt(a1931, a1932) {
                let a1933 = v0;
                let a1934 = 2u64;
                if nat_eq(a1933, a1934) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1935 = v0;
                let a1936 = 3u64;
                if nat_eq(a1935, a1936) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1937 = v0;
        let a1938 = 6u64;
        if nat_lt(a1937, a1938) {
            let a1939 = v0;
            let a1940 = 5u64;
            if nat_lt(a1939, a1940) {
                let a1941 = v0;
                let a1942 = 4u64;
                if nat_eq(a1941, a1942) {
                    Ok(f83(f93(), f107()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1943 = v0;
                let a1944 = 5u64;
                if nat_eq(a1943, a1944) {
                    Ok(f83(f95(), f106()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1945 = v0;
            let a1946 = 7u64;
            if nat_lt(a1945, a1946) {
                let a1947 = v0;
                let a1948 = 6u64;
                if nat_eq(a1947, a1948) {
                    Ok(f113(1u64, 1u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1949 = v0;
                let a1950 = 7u64;
                if nat_eq(a1949, a1950) {
                    Ok(f80(1u64, f139(f119())?, f139(f118())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f58(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1951 = v0;
    let a1952 = 4u64;
    if nat_lt(a1951, a1952) {
        let a1953 = v0;
        let a1954 = 2u64;
        if nat_lt(a1953, a1954) {
            let a1955 = v0;
            let a1956 = 1u64;
            if nat_lt(a1955, a1956) {
                let a1957 = v0;
                let a1958 = 0u64;
                if nat_eq(a1957, a1958) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1959 = v0;
                let a1960 = 1u64;
                if nat_eq(a1959, a1960) {
                    Ok(f80(1u64, f141(f119())?, f141(f118())?, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1961 = v0;
            let a1962 = 3u64;
            if nat_lt(a1961, a1962) {
                let a1963 = v0;
                let a1964 = 2u64;
                if nat_eq(a1963, a1964) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1965 = v0;
                let a1966 = 3u64;
                if nat_eq(a1965, a1966) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1967 = v0;
        let a1968 = 6u64;
        if nat_lt(a1967, a1968) {
            let a1969 = v0;
            let a1970 = 5u64;
            if nat_lt(a1969, a1970) {
                let a1971 = v0;
                let a1972 = 4u64;
                if nat_eq(a1971, a1972) {
                    Ok(f80(0u64, f107(), f93(), f107()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1973 = v0;
                let a1974 = 5u64;
                if nat_eq(a1973, a1974) {
                    Ok(f80(0u64, f106(), f95(), f106()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1975 = v0;
            let a1976 = 7u64;
            if nat_lt(a1975, a1976) {
                let a1977 = v0;
                let a1978 = 6u64;
                if nat_eq(a1977, a1978) {
                    Ok(f113(1u64, 2u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1979 = v0;
                let a1980 = 7u64;
                if nat_eq(a1979, a1980) {
                    Ok(f80(1u64, f139(f94())?, f139(f119())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f59(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1981 = v0;
    let a1982 = 4u64;
    if nat_lt(a1981, a1982) {
        let a1983 = v0;
        let a1984 = 2u64;
        if nat_lt(a1983, a1984) {
            let a1985 = v0;
            let a1986 = 1u64;
            if nat_lt(a1985, a1986) {
                let a1987 = v0;
                let a1988 = 0u64;
                if nat_eq(a1987, a1988) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1989 = v0;
                let a1990 = 1u64;
                if nat_eq(a1989, a1990) {
                    Ok(f80(1u64, f141(f94())?, f141(f119())?, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1991 = v0;
            let a1992 = 3u64;
            if nat_lt(a1991, a1992) {
                let a1993 = v0;
                let a1994 = 2u64;
                if nat_eq(a1993, a1994) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1995 = v0;
                let a1996 = 3u64;
                if nat_eq(a1995, a1996) {
                    Ok(f80(2u64, f95(), f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1997 = v0;
        let a1998 = 6u64;
        if nat_lt(a1997, a1998) {
            let a1999 = v0;
            let a2000 = 5u64;
            if nat_lt(a1999, a2000) {
                let a2001 = v0;
                let a2002 = 4u64;
                if nat_eq(a2001, a2002) {
                    Ok(f80(0u64, f107(), f93(), f107()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2003 = v0;
                let a2004 = 5u64;
                if nat_eq(a2003, a2004) {
                    Ok(f80(0u64, f106(), f95(), f106()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2005 = v0;
            let a2006 = 7u64;
            if nat_lt(a2005, a2006) {
                let a2007 = v0;
                let a2008 = 6u64;
                if nat_eq(a2007, a2008) {
                    Ok(f83(1056u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2009 = v0;
                let a2010 = 7u64;
                if nat_eq(a2009, a2010) {
                    Ok(f99(1099u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f60(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2011 = v0;
    let a2012 = 4u64;
    if nat_lt(a2011, a2012) {
        let a2013 = v0;
        let a2014 = 2u64;
        if nat_lt(a2013, a2014) {
            let a2015 = v0;
            let a2016 = 1u64;
            if nat_lt(a2015, a2016) {
                let a2017 = v0;
                let a2018 = 0u64;
                if nat_eq(a2017, a2018) {
                    f90(0u64, f138(), 1u64, f138())
                } else {
                    f2()
                }
            } else {
                let a2019 = v0;
                let a2020 = 1u64;
                if nat_eq(a2019, a2020) {
                    f91(1u64, f138(), 2u64, 1030u64)
                } else {
                    f2()
                }
            }
        } else {
            let a2021 = v0;
            let a2022 = 3u64;
            if nat_lt(a2021, a2022) {
                let a2023 = v0;
                let a2024 = 2u64;
                if nat_eq(a2023, a2024) {
                    f99(1093u64)
                } else {
                    f2()
                }
            } else {
                let a2025 = v0;
                let a2026 = 3u64;
                if nat_eq(a2025, a2026) {
                    f83(0u64, f86())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2027 = v0;
        let a2028 = 6u64;
        if nat_lt(a2027, a2028) {
            let a2029 = v0;
            let a2030 = 5u64;
            if nat_lt(a2029, a2030) {
                let a2031 = v0;
                let a2032 = 4u64;
                if nat_eq(a2031, a2032) {
                    f90(2u64, f86(), 17u64, f116())
                } else {
                    f2()
                }
            } else {
                let a2033 = v0;
                let a2034 = 5u64;
                if nat_eq(a2033, a2034) {
                    f90(0u64, f116(), f118(), f142())
                } else {
                    f2()
                }
            }
        } else {
            let a2035 = v0;
            let a2036 = 7u64;
            if nat_lt(a2035, a2036) {
                let a2037 = v0;
                let a2038 = 6u64;
                if nat_eq(a2037, a2038) {
                    f90(0u64, f116(), f119(), f143())
                } else {
                    f2()
                }
            } else {
                let a2039 = v0;
                let a2040 = 7u64;
                if nat_eq(a2039, a2040) {
                    f90(0u64, f116(), f94(), f116())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f61(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2041 = v0;
    let a2042 = 4u64;
    if nat_lt(a2041, a2042) {
        let a2043 = v0;
        let a2044 = 2u64;
        if nat_lt(a2043, a2044) {
            let a2045 = v0;
            let a2046 = 1u64;
            if nat_lt(a2045, a2046) {
                let a2047 = v0;
                let a2048 = 0u64;
                if nat_eq(a2047, a2048) {
                    Ok(f80(0u64, f139(f142())?, f139(f143())?, f124()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2049 = v0;
                let a2050 = 1u64;
                if nat_eq(a2049, a2050) {
                    Ok(f83(0u64, f138()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2051 = v0;
            let a2052 = 3u64;
            if nat_lt(a2051, a2052) {
                let a2053 = v0;
                let a2054 = 2u64;
                if nat_eq(a2053, a2054) {
                    Ok(f90(2u64, f86(), 2u64, f144()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2055 = v0;
                let a2056 = 3u64;
                if nat_eq(a2055, a2056) {
                    Ok(f90(0u64, f144(), f138(), f144()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2057 = v0;
        let a2058 = 6u64;
        if nat_lt(a2057, a2058) {
            let a2059 = v0;
            let a2060 = 5u64;
            if nat_lt(a2059, a2060) {
                let a2061 = v0;
                let a2062 = 4u64;
                if nat_eq(a2061, a2062) {
                    Ok(f113(1u64, 2u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2063 = v0;
                let a2064 = 5u64;
                if nat_eq(a2063, a2064) {
                    Ok(f80(2u64, f139(f116())?, f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2065 = v0;
            let a2066 = 7u64;
            if nat_lt(a2065, a2066) {
                let a2067 = v0;
                let a2068 = 6u64;
                if nat_eq(a2067, a2068) {
                    Ok(f80(1u64, f93(), f124(), f145(f144())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a2069 = v0;
                let a2070 = 7u64;
                if nat_eq(a2069, a2070) {
                    Ok(f80(2u64, f141(f116())?, f127(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f62(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2071 = v0;
    let a2072 = 4u64;
    if nat_lt(a2071, a2072) {
        let a2073 = v0;
        let a2074 = 2u64;
        if nat_lt(a2073, a2074) {
            let a2075 = v0;
            let a2076 = 1u64;
            if nat_lt(a2075, a2076) {
                let a2077 = v0;
                let a2078 = 0u64;
                if nat_eq(a2077, a2078) {
                    Ok(f80(2u64, f93(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2079 = v0;
                let a2080 = 1u64;
                if nat_eq(a2079, a2080) {
                    Ok(f113(1u64, 3u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2081 = v0;
            let a2082 = 3u64;
            if nat_lt(a2081, a2082) {
                let a2083 = v0;
                let a2084 = 2u64;
                if nat_eq(a2083, a2084) {
                    Ok(f80(2u64, f141(f142())?, f127(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2085 = v0;
                let a2086 = 3u64;
                if nat_eq(a2085, a2086) {
                    Ok(f80(0u64, f93(), f95(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2087 = v0;
        let a2088 = 6u64;
        if nat_lt(a2087, a2088) {
            let a2089 = v0;
            let a2090 = 5u64;
            if nat_lt(a2089, a2090) {
                let a2091 = v0;
                let a2092 = 4u64;
                if nat_eq(a2091, a2092) {
                    Ok(f113(1u64, 4u64, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2093 = v0;
                let a2094 = 5u64;
                if nat_eq(a2093, a2094) {
                    Ok(f80(2u64, f141(f143())?, f127(), f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2095 = v0;
            let a2096 = 7u64;
            if nat_lt(a2095, a2096) {
                let a2097 = v0;
                let a2098 = 6u64;
                if nat_eq(a2097, a2098) {
                    Ok(f80(0u64, f93(), f95(), f146(f144())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a2099 = v0;
                let a2100 = 7u64;
                if nat_eq(a2099, a2100) {
                    Ok(f90(0u64, f138(), 1u64, f138()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f63(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2101 = v0;
    let a2102 = 4u64;
    if nat_lt(a2101, a2102) {
        let a2103 = v0;
        let a2104 = 2u64;
        if nat_lt(a2103, a2104) {
            let a2105 = v0;
            let a2106 = 1u64;
            if nat_lt(a2105, a2106) {
                let a2107 = v0;
                let a2108 = 0u64;
                if nat_eq(a2107, a2108) {
                    Ok(f91(1u64, f138(), 2u64, 1066u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2109 = v0;
                let a2110 = 1u64;
                if nat_eq(a2109, a2110) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2111 = v0;
            let a2112 = 3u64;
            if nat_lt(a2111, a2112) {
                let a2113 = v0;
                let a2114 = 2u64;
                if nat_eq(a2113, a2114) {
                    Ok(f91(1u64, f86(), 2u64, 1060u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2115 = v0;
                let a2116 = 3u64;
                if nat_eq(a2115, a2116) {
                    Ok(f83(0u64, f138()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2117 = v0;
        let a2118 = 6u64;
        if nat_lt(a2117, a2118) {
            let a2119 = v0;
            let a2120 = 5u64;
            if nat_lt(a2119, a2120) {
                let a2121 = v0;
                let a2122 = 4u64;
                if nat_eq(a2121, a2122) {
                    Ok(f90(3u64, f138(), 2u64, f143()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2123 = v0;
                let a2124 = 5u64;
                if nat_eq(a2123, a2124) {
                    Ok(f90(4u64, f138(), 2u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2125 = v0;
            let a2126 = 7u64;
            if nat_lt(a2125, a2126) {
                let a2127 = v0;
                let a2128 = 6u64;
                if nat_eq(a2127, a2128) {
                    Ok(f90(0u64, f116(), 2u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2129 = v0;
                let a2130 = 7u64;
                if nat_eq(a2129, a2130) {
                    Ok(f80(0u64, f145(f143())?, f145(f116())?, f107()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f64(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2131 = v0;
    let a2132 = 4u64;
    if nat_lt(a2131, a2132) {
        let a2133 = v0;
        let a2134 = 2u64;
        if nat_lt(a2133, a2134) {
            let a2135 = v0;
            let a2136 = 1u64;
            if nat_lt(a2135, a2136) {
                let a2137 = v0;
                let a2138 = 0u64;
                if nat_eq(a2137, a2138) {
                    Ok(f80(0u64, f146(f143())?, f146(f116())?, f106()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2139 = v0;
                let a2140 = 1u64;
                if nat_eq(a2139, a2140) {
                    Ok(f83(1091u64, f98()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2141 = v0;
            let a2142 = 3u64;
            if nat_lt(a2141, a2142) {
                let a2143 = v0;
                let a2144 = 2u64;
                if nat_eq(a2143, a2144) {
                    Ok(f99(1099u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2145 = v0;
                let a2146 = 3u64;
                if nat_eq(a2145, a2146) {
                    Ok(f90(0u64, f138(), 1u64, f138()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2147 = v0;
        let a2148 = 6u64;
        if nat_lt(a2147, a2148) {
            let a2149 = v0;
            let a2150 = 5u64;
            if nat_lt(a2149, a2150) {
                let a2151 = v0;
                let a2152 = 4u64;
                if nat_eq(a2151, a2152) {
                    Ok(f91(1u64, f138(), 4u64, 1084u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2153 = v0;
                let a2154 = 5u64;
                if nat_eq(a2153, a2154) {
                    Ok(f90(0u64, f119(), 1u64, f119()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2155 = v0;
            let a2156 = 7u64;
            if nat_lt(a2155, a2156) {
                let a2157 = v0;
                let a2158 = 6u64;
                if nat_eq(a2157, a2158) {
                    Ok(f91(1u64, f94(), f119(), 1096u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2159 = v0;
                let a2160 = 7u64;
                if nat_eq(a2159, a2160) {
                    Ok(f99(1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f65(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2161 = v0;
    let a2162 = 4u64;
    if nat_lt(a2161, a2162) {
        let a2163 = v0;
        let a2164 = 2u64;
        if nat_lt(a2163, a2164) {
            let a2165 = v0;
            let a2166 = 1u64;
            if nat_lt(a2165, a2166) {
                let a2167 = v0;
                let a2168 = 0u64;
                if nat_eq(a2167, a2168) {
                    f90(0u64, f118(), 1u64, f118())
                } else {
                    f2()
                }
            } else {
                let a2169 = v0;
                let a2170 = 1u64;
                if nat_eq(a2169, a2170) {
                    f91(1u64, f94(), f118(), 1112u64)
                } else {
                    f2()
                }
            }
        } else {
            let a2171 = v0;
            let a2172 = 3u64;
            if nat_lt(a2171, a2172) {
                let a2173 = v0;
                let a2174 = 2u64;
                if nat_eq(a2173, a2174) {
                    f99(1027u64)
                } else {
                    f2()
                }
            } else {
                let a2175 = v0;
                let a2176 = 3u64;
                if nat_eq(a2175, a2176) {
                    f91(2u64, 0u64, f106(), 1101u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2177 = v0;
        let a2178 = 6u64;
        if nat_lt(a2177, a2178) {
            let a2179 = v0;
            let a2180 = 5u64;
            if nat_lt(a2179, a2180) {
                let a2181 = v0;
                let a2182 = 4u64;
                if nat_eq(a2181, a2182) {
                    f129(f98())
                } else {
                    f2()
                }
            } else {
                let a2183 = v0;
                let a2184 = 5u64;
                if nat_eq(a2183, a2184) {
                    f80(2u64, f107(), f107(), f95())
                } else {
                    f2()
                }
            }
        } else {
            let a2185 = v0;
            let a2186 = 7u64;
            if nat_lt(a2185, a2186) {
                let a2187 = v0;
                let a2188 = 6u64;
                if nat_eq(a2187, a2188) {
                    f80(2u64, f84(), f106(), f93())
                } else {
                    f2()
                }
            } else {
                let a2189 = v0;
                let a2190 = 7u64;
                if nat_eq(a2189, a2190) {
                    f91(2u64, f93(), f95(), 1105u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f66(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2191 = v0;
    let a2192 = 4u64;
    if nat_lt(a2191, a2192) {
        let a2193 = v0;
        let a2194 = 2u64;
        if nat_lt(a2193, a2194) {
            let a2195 = v0;
            let a2196 = 1u64;
            if nat_lt(a2195, a2196) {
                let a2197 = v0;
                let a2198 = 0u64;
                if nat_eq(a2197, a2198) {
                    f129(f98())
                } else {
                    f2()
                }
            } else {
                let a2199 = v0;
                let a2200 = 1u64;
                if nat_eq(a2199, a2200) {
                    f80(3u64, f107(), f106(), f82())
                } else {
                    f2()
                }
            }
        } else {
            let a2201 = v0;
            let a2202 = 3u64;
            if nat_lt(a2201, a2202) {
                let a2203 = v0;
                let a2204 = 2u64;
                if nat_eq(a2203, a2204) {
                    f80(2u64, f82(), f107(), f84())
                } else {
                    f2()
                }
            } else {
                let a2205 = v0;
                let a2206 = 3u64;
                if nat_eq(a2205, a2206) {
                    f83(f118(), f108())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2207 = v0;
        let a2208 = 6u64;
        if nat_lt(a2207, a2208) {
            let a2209 = v0;
            let a2210 = 5u64;
            if nat_lt(a2209, a2210) {
                let a2211 = v0;
                let a2212 = 4u64;
                if nat_eq(a2211, a2212) {
                    f83(f119(), f147())
                } else {
                    f2()
                }
            } else {
                let a2213 = v0;
                let a2214 = 5u64;
                if nat_eq(a2213, a2214) {
                    f83(f138(), f148())
                } else {
                    f2()
                }
            }
        } else {
            let a2215 = v0;
            let a2216 = 7u64;
            if nat_lt(a2215, a2216) {
                let a2217 = v0;
                let a2218 = 6u64;
                if nat_eq(a2217, a2218) {
                    f83(1u64, f134())
                } else {
                    f2()
                }
            } else {
                let a2219 = v0;
                let a2220 = 7u64;
                if nat_eq(a2219, a2220) {
                    f129(f98())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f67(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2221 = v0;
    let a2222 = 4u64;
    if nat_lt(a2221, a2222) {
        let a2223 = v0;
        let a2224 = 2u64;
        if nat_lt(a2223, a2224) {
            let a2225 = v0;
            let a2226 = 1u64;
            if nat_lt(a2225, a2226) {
                let a2227 = v0;
                let a2228 = 0u64;
                if nat_eq(a2227, a2228) {
                    f91(0u64, f134(), 0u64, 2586u64)
                } else {
                    f2()
                }
            } else {
                let a2229 = v0;
                let a2230 = 1u64;
                if nat_eq(a2229, a2230) {
                    f83(0u64, f111())
                } else {
                    f2()
                }
            }
        } else {
            let a2231 = v0;
            let a2232 = 3u64;
            if nat_lt(a2231, a2232) {
                let a2233 = v0;
                let a2234 = 2u64;
                if nat_eq(a2233, a2234) {
                    f91(2u64, f82(), 0u64, 1117u64)
                } else {
                    f2()
                }
            } else {
                let a2235 = v0;
                let a2236 = 3u64;
                if nat_eq(a2235, a2236) {
                    f83(0u64, f149())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2237 = v0;
        let a2238 = 6u64;
        if nat_lt(a2237, a2238) {
            let a2239 = v0;
            let a2240 = 5u64;
            if nat_lt(a2239, a2240) {
                let a2241 = v0;
                let a2242 = 4u64;
                if nat_eq(a2241, a2242) {
                    f99(1123u64)
                } else {
                    f2()
                }
            } else {
                let a2243 = v0;
                let a2244 = 5u64;
                if nat_eq(a2243, a2244) {
                    f80(6u64, f82(), 0u64, f82())
                } else {
                    f2()
                }
            }
        } else {
            let a2245 = v0;
            let a2246 = 7u64;
            if nat_lt(a2245, a2246) {
                let a2247 = v0;
                let a2248 = 6u64;
                if nat_eq(a2247, a2248) {
                    f83(1u64, f149())
                } else {
                    f2()
                }
            } else {
                let a2249 = v0;
                let a2250 = 7u64;
                if nat_eq(a2249, a2250) {
                    f83(1u64, f95())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f68(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2251 = v0;
    let a2252 = 4u64;
    if nat_lt(a2251, a2252) {
        let a2253 = v0;
        let a2254 = 2u64;
        if nat_lt(a2253, a2254) {
            let a2255 = v0;
            let a2256 = 1u64;
            if nat_lt(a2255, a2256) {
                let a2257 = v0;
                let a2258 = 0u64;
                if nat_eq(a2257, a2258) {
                    Ok(f91(0u64, f88(), 0u64, 1122u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2259 = v0;
                let a2260 = 1u64;
                if nat_eq(a2259, a2260) {
                    Ok(f83(3u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2261 = v0;
            let a2262 = 3u64;
            if nat_lt(a2261, a2262) {
                let a2263 = v0;
                let a2264 = 2u64;
                if nat_eq(a2263, a2264) {
                    Ok(f90(1u64, f95(), f148(), f148()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2265 = v0;
                let a2266 = 3u64;
                if nat_eq(a2265, a2266) {
                    Ok(f83(f150(f111())?, f116()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2267 = v0;
        let a2268 = 6u64;
        if nat_lt(a2267, a2268) {
            let a2269 = v0;
            let a2270 = 5u64;
            if nat_lt(a2269, a2270) {
                let a2271 = v0;
                let a2272 = 4u64;
                if nat_eq(a2271, a2272) {
                    Ok(f83(0u64, f114()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2273 = v0;
                let a2274 = 5u64;
                if nat_eq(a2273, a2274) {
                    Ok(f91(1u64, f111(), f108(), 1129u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2275 = v0;
            let a2276 = 7u64;
            if nat_lt(a2275, a2276) {
                let a2277 = v0;
                let a2278 = 6u64;
                if nat_eq(a2277, a2278) {
                    Ok(f90(0u64, f114(), 1u64, f114()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2279 = v0;
                let a2280 = 7u64;
                if nat_eq(a2279, a2280) {
                    Ok(f91(1u64, f111(), f147(), 1129u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f69(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2281 = v0;
    let a2282 = 3u64;
    if nat_lt(a2281, a2282) {
        let a2283 = v0;
        let a2284 = 1u64;
        if nat_lt(a2283, a2284) {
            let a2285 = v0;
            let a2286 = 0u64;
            if nat_eq(a2285, a2286) {
                Ok(f90(0u64, f114(), 1u64, f114()))
            } else {
                Ok(f2())
            }
        } else {
            let a2287 = v0;
            let a2288 = 2u64;
            if nat_lt(a2287, a2288) {
                let a2289 = v0;
                let a2290 = 1u64;
                if nat_eq(a2289, a2290) {
                    Ok(f91(0u64, f149(), 0u64, 1131u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2291 = v0;
                let a2292 = 2u64;
                if nat_eq(a2291, a2292) {
                    Ok(f90(1u64, 2u64, f114(), f114()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2293 = v0;
        let a2294 = 5u64;
        if nat_lt(a2293, a2294) {
            let a2295 = v0;
            let a2296 = 4u64;
            if nat_lt(a2295, a2296) {
                let a2297 = v0;
                let a2298 = 3u64;
                if nat_eq(a2297, a2298) {
                    Ok(f83(f114(), f151(f116())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a2299 = v0;
                let a2300 = 4u64;
                if nat_eq(a2299, a2300) {
                    Ok(f90(0u64, f111(), 1u64, f111()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2301 = v0;
            let a2302 = 6u64;
            if nat_lt(a2301, a2302) {
                let a2303 = v0;
                let a2304 = 5u64;
                if nat_eq(a2303, a2304) {
                    Ok(f91(1u64, f111(), f94(), 1123u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2305 = v0;
                let a2306 = 6u64;
                if nat_eq(a2305, a2306) {
                    Ok(f99(1536u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f70(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2307 = v0;
    let a2308 = 4u64;
    if nat_lt(a2307, a2308) {
        let a2309 = v0;
        let a2310 = 2u64;
        if nat_lt(a2309, a2310) {
            let a2311 = v0;
            let a2312 = 1u64;
            if nat_lt(a2311, a2312) {
                let a2313 = v0;
                let a2314 = 0u64;
                if nat_eq(a2313, a2314) {
                    f83(0u64, f86())
                } else {
                    f2()
                }
            } else {
                let a2315 = v0;
                let a2316 = 1u64;
                if nat_eq(a2315, a2316) {
                    f83(0u64, f152())
                } else {
                    f2()
                }
            }
        } else {
            let a2317 = v0;
            let a2318 = 3u64;
            if nat_lt(a2317, a2318) {
                let a2319 = v0;
                let a2320 = 2u64;
                if nat_eq(a2319, a2320) {
                    f91(0u64, f88(), 0u64, 1544u64)
                } else {
                    f2()
                }
            } else {
                let a2321 = v0;
                let a2322 = 3u64;
                if nat_eq(a2321, a2322) {
                    f91(0u64, f86(), 0u64, 1542u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2323 = v0;
        let a2324 = 6u64;
        if nat_lt(a2323, a2324) {
            let a2325 = v0;
            let a2326 = 5u64;
            if nat_lt(a2325, a2326) {
                let a2327 = v0;
                let a2328 = 4u64;
                if nat_eq(a2327, a2328) {
                    f90(4u64, f148(), 2u64, f138())
                } else {
                    f2()
                }
            } else {
                let a2329 = v0;
                let a2330 = 5u64;
                if nat_eq(a2329, a2330) {
                    f99(1545u64)
                } else {
                    f2()
                }
            }
        } else {
            let a2331 = v0;
            let a2332 = 7u64;
            if nat_lt(a2331, a2332) {
                let a2333 = v0;
                let a2334 = 6u64;
                if nat_eq(a2333, a2334) {
                    f90(3u64, f148(), 2u64, f138())
                } else {
                    f2()
                }
            } else {
                let a2335 = v0;
                let a2336 = 7u64;
                if nat_eq(a2335, a2336) {
                    f99(1545u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f71(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2337 = v0;
    let a2338 = 4u64;
    if nat_lt(a2337, a2338) {
        let a2339 = v0;
        let a2340 = 2u64;
        if nat_lt(a2339, a2340) {
            let a2341 = v0;
            let a2342 = 1u64;
            if nat_lt(a2341, a2342) {
                let a2343 = v0;
                let a2344 = 0u64;
                if nat_eq(a2343, a2344) {
                    Ok(f83(f148(), f138()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2345 = v0;
                let a2346 = 1u64;
                if nat_eq(a2345, a2346) {
                    Ok(f90(2u64, f86(), 8u64, f110()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2347 = v0;
            let a2348 = 3u64;
            if nat_lt(a2347, a2348) {
                let a2349 = v0;
                let a2350 = 2u64;
                if nat_eq(a2349, a2350) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2351 = v0;
                let a2352 = 3u64;
                if nat_eq(a2351, a2352) {
                    Ok(f83(0u64, f108()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2353 = v0;
        let a2354 = 6u64;
        if nat_lt(a2353, a2354) {
            let a2355 = v0;
            let a2356 = 5u64;
            if nat_lt(a2355, a2356) {
                let a2357 = v0;
                let a2358 = 4u64;
                if nat_eq(a2357, a2358) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2359 = v0;
                let a2360 = 5u64;
                if nat_eq(a2359, a2360) {
                    Ok(f113(3u64, f108(), f109(), f151(f111())?, f108()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2361 = v0;
            let a2362 = 7u64;
            if nat_lt(a2361, a2362) {
                let a2363 = v0;
                let a2364 = 6u64;
                if nat_eq(a2363, a2364) {
                    Ok(f90(0u64, f109(), 1u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2365 = v0;
                let a2366 = 7u64;
                if nat_eq(a2365, a2366) {
                    Ok(f91(1u64, f109(), 8u64, 1548u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f72(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2367 = v0;
    let a2368 = 4u64;
    if nat_lt(a2367, a2368) {
        let a2369 = v0;
        let a2370 = 2u64;
        if nat_lt(a2369, a2370) {
            let a2371 = v0;
            let a2372 = 1u64;
            if nat_lt(a2371, a2372) {
                let a2373 = v0;
                let a2374 = 0u64;
                if nat_eq(a2373, a2374) {
                    Ok(f90(0u64, f108(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2375 = v0;
                let a2376 = 1u64;
                if nat_eq(a2375, a2376) {
                    Ok(f83(f117(f116())?, f118()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2377 = v0;
            let a2378 = 3u64;
            if nat_lt(a2377, a2378) {
                let a2379 = v0;
                let a2380 = 2u64;
                if nat_eq(a2379, a2380) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2381 = v0;
                let a2382 = 3u64;
                if nat_eq(a2381, a2382) {
                    Ok(f83(f117(f116())?, f119()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2383 = v0;
        let a2384 = 6u64;
        if nat_lt(a2383, a2384) {
            let a2385 = v0;
            let a2386 = 5u64;
            if nat_lt(a2385, a2386) {
                let a2387 = v0;
                let a2388 = 4u64;
                if nat_eq(a2387, a2388) {
                    Ok(f90(0u64, f116(), 6561u64, f116()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2389 = v0;
                let a2390 = 5u64;
                if nat_eq(a2389, a2390) {
                    Ok(f90(2u64, f117(f116())?, 65536u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2391 = v0;
            let a2392 = 7u64;
            if nat_lt(a2391, a2392) {
                let a2393 = v0;
                let a2394 = 6u64;
                if nat_eq(a2393, a2394) {
                    Ok(f90(0u64, f118(), f95(), f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2395 = v0;
                let a2396 = 7u64;
                if nat_eq(a2395, a2396) {
                    Ok(f90(0u64, f116(), 1u64, f116()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f73(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2397 = v0;
    let a2398 = 4u64;
    if nat_lt(a2397, a2398) {
        let a2399 = v0;
        let a2400 = 2u64;
        if nat_lt(a2399, a2400) {
            let a2401 = v0;
            let a2402 = 1u64;
            if nat_lt(a2401, a2402) {
                let a2403 = v0;
                let a2404 = 0u64;
                if nat_eq(a2403, a2404) {
                    Ok(f90(2u64, f117(f116())?, 65536u64, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2405 = v0;
                let a2406 = 1u64;
                if nat_eq(a2405, a2406) {
                    Ok(f90(0u64, f119(), f95(), f119()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2407 = v0;
            let a2408 = 3u64;
            if nat_lt(a2407, a2408) {
                let a2409 = v0;
                let a2410 = 2u64;
                if nat_eq(a2409, a2410) {
                    Ok(f90(0u64, f118(), 13125u64, f118()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2411 = v0;
                let a2412 = 3u64;
                if nat_eq(a2411, a2412) {
                    Ok(f90(0u64, f119(), 13125u64, f119()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2413 = v0;
        let a2414 = 6u64;
        if nat_lt(a2413, a2414) {
            let a2415 = v0;
            let a2416 = 5u64;
            if nat_lt(a2415, a2416) {
                let a2417 = v0;
                let a2418 = 4u64;
                if nat_eq(a2417, a2418) {
                    Ok(f83(f117(f118())?, f100()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2419 = v0;
                let a2420 = 5u64;
                if nat_eq(a2419, a2420) {
                    Ok(f91(1u64, f100(), 32768u64, 1568u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2421 = v0;
            let a2422 = 7u64;
            if nat_lt(a2421, a2422) {
                let a2423 = v0;
                let a2424 = 6u64;
                if nat_eq(a2423, a2424) {
                    Ok(f90(1u64, f100(), 32768u64, f100()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2425 = v0;
                let a2426 = 7u64;
                if nat_eq(a2425, a2426) {
                    Ok(f99(1604u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f74(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2427 = v0;
    let a2428 = 4u64;
    if nat_lt(a2427, a2428) {
        let a2429 = v0;
        let a2430 = 2u64;
        if nat_lt(a2429, a2430) {
            let a2431 = v0;
            let a2432 = 1u64;
            if nat_lt(a2431, a2432) {
                let a2433 = v0;
                let a2434 = 0u64;
                if nat_eq(a2433, a2434) {
                    Ok(f83(1u64, f152()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2435 = v0;
                let a2436 = 1u64;
                if nat_eq(a2435, a2436) {
                    Ok(f83(0u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2437 = v0;
            let a2438 = 3u64;
            if nat_lt(a2437, a2438) {
                let a2439 = v0;
                let a2440 = 2u64;
                if nat_eq(a2439, a2440) {
                    Ok(f83(f118(), f122()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2441 = v0;
                let a2442 = 3u64;
                if nat_eq(a2441, a2442) {
                    Ok(f83(2048u64, f100()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2443 = v0;
        let a2444 = 6u64;
        if nat_lt(a2443, a2444) {
            let a2445 = v0;
            let a2446 = 5u64;
            if nat_lt(a2445, a2446) {
                let a2447 = v0;
                let a2448 = 4u64;
                if nat_eq(a2447, a2448) {
                    Ok(f83(2139095039u64, f121()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2449 = v0;
                let a2450 = 5u64;
                if nat_eq(a2449, a2450) {
                    Ok(f91(0u64, f104(), 1u64, 1576u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2451 = v0;
            let a2452 = 7u64;
            if nat_lt(a2451, a2452) {
                let a2453 = v0;
                let a2454 = 6u64;
                if nat_eq(a2453, a2454) {
                    Ok(f83(f117(f122())?, f123()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2455 = v0;
                let a2456 = 7u64;
                if nat_eq(a2455, a2456) {
                    Ok(f99(1577u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f75(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2457 = v0;
    let a2458 = 4u64;
    if nat_lt(a2457, a2458) {
        let a2459 = v0;
        let a2460 = 2u64;
        if nat_lt(a2459, a2460) {
            let a2461 = v0;
            let a2462 = 1u64;
            if nat_lt(a2461, a2462) {
                let a2463 = v0;
                let a2464 = 0u64;
                if nat_eq(a2463, a2464) {
                    Ok(f83(f122(), f123()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2465 = v0;
                let a2466 = 1u64;
                if nat_eq(a2465, a2466) {
                    Ok(f83(0u64, f124()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2467 = v0;
            let a2468 = 3u64;
            if nat_lt(a2467, a2468) {
                let a2469 = v0;
                let a2470 = 2u64;
                if nat_eq(a2469, a2470) {
                    Ok(f83(0u64, f109()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2471 = v0;
                let a2472 = 3u64;
                if nat_eq(a2471, a2472) {
                    Ok(f115(4u64, 0u64, f123(), 0u64, f95(), f125()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2473 = v0;
        let a2474 = 6u64;
        if nat_lt(a2473, a2474) {
            let a2475 = v0;
            let a2476 = 5u64;
            if nat_lt(a2475, a2476) {
                let a2477 = v0;
                let a2478 = 4u64;
                if nat_eq(a2477, a2478) {
                    Ok(f90(0u64, f125(), f109(), f125()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2479 = v0;
                let a2480 = 5u64;
                if nat_eq(a2479, a2480) {
                    Ok(f113(5u64, f126(f125())?, f138(), 0u64, f127()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2481 = v0;
            let a2482 = 7u64;
            if nat_lt(a2481, a2482) {
                let a2483 = v0;
                let a2484 = 6u64;
                if nat_eq(a2483, a2484) {
                    Ok(f90(0u64, f110(), f109(), f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2485 = v0;
                let a2486 = 7u64;
                if nat_eq(a2485, a2486) {
                    Ok(f90(0u64, f153(), f111(), f154()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f76(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2487 = v0;
    let a2488 = 4u64;
    if nat_lt(a2487, a2488) {
        let a2489 = v0;
        let a2490 = 2u64;
        if nat_lt(a2489, a2490) {
            let a2491 = v0;
            let a2492 = 1u64;
            if nat_lt(a2491, a2492) {
                let a2493 = v0;
                let a2494 = 0u64;
                if nat_eq(a2493, a2494) {
                    Ok(f80(2u64, f82(), f127(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2495 = v0;
                let a2496 = 1u64;
                if nat_eq(a2495, a2496) {
                    Ok(f80(1u64, f93(), f155(f154())?, f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2497 = v0;
            let a2498 = 3u64;
            if nat_lt(a2497, a2498) {
                let a2499 = v0;
                let a2500 = 2u64;
                if nat_eq(a2499, a2500) {
                    Ok(f80(2u64, f156(f111())?, f93(), f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2501 = v0;
                let a2502 = 3u64;
                if nat_eq(a2501, a2502) {
                    Ok(f80(2u64, f95(), f93(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a2503 = v0;
        let a2504 = 6u64;
        if nat_lt(a2503, a2504) {
            let a2505 = v0;
            let a2506 = 5u64;
            if nat_lt(a2505, a2506) {
                let a2507 = v0;
                let a2508 = 4u64;
                if nat_eq(a2507, a2508) {
                    Ok(f80(0u64, f124(), f95(), f124()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2509 = v0;
                let a2510 = 5u64;
                if nat_eq(a2509, a2510) {
                    Ok(f90(0u64, f109(), 1u64, f109()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2511 = v0;
            let a2512 = 7u64;
            if nat_lt(a2511, a2512) {
                let a2513 = v0;
                let a2514 = 6u64;
                if nat_eq(a2513, a2514) {
                    Ok(f91(1u64, f109(), 8u64, 1579u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2515 = v0;
                let a2516 = 7u64;
                if nat_eq(a2515, a2516) {
                    Ok(f91(2u64, f124(), f121(), 1593u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f77(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2517 = v0;
    let a2518 = 4u64;
    if nat_lt(a2517, a2518) {
        let a2519 = v0;
        let a2520 = 2u64;
        if nat_lt(a2519, a2520) {
            let a2521 = v0;
            let a2522 = 1u64;
            if nat_lt(a2521, a2522) {
                let a2523 = v0;
                let a2524 = 0u64;
                if nat_eq(a2523, a2524) {
                    f99(1595u64)
                } else {
                    f2()
                }
            } else {
                let a2525 = v0;
                let a2526 = 1u64;
                if nat_eq(a2525, a2526) {
                    f83(f124(), f121())
                } else {
                    f2()
                }
            }
        } else {
            let a2527 = v0;
            let a2528 = 3u64;
            if nat_lt(a2527, a2528) {
                let a2529 = v0;
                let a2530 = 2u64;
                if nat_eq(a2529, a2530) {
                    f83(f123(), f100())
                } else {
                    f2()
                }
            } else {
                let a2531 = v0;
                let a2532 = 3u64;
                if nat_eq(a2531, a2532) {
                    f90(0u64, f122(), 1u64, f122())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a2533 = v0;
        let a2534 = 6u64;
        if nat_lt(a2533, a2534) {
            let a2535 = v0;
            let a2536 = 5u64;
            if nat_lt(a2535, a2536) {
                let a2537 = v0;
                let a2538 = 4u64;
                if nat_eq(a2537, a2538) {
                    f91(1u64, f122(), f119(), 1573u64)
                } else {
                    f2()
                }
            } else {
                let a2539 = v0;
                let a2540 = 5u64;
                if nat_eq(a2539, a2540) {
                    f91(1u64, f100(), 2048u64, 1604u64)
                } else {
                    f2()
                }
            }
        } else {
            let a2541 = v0;
            let a2542 = 7u64;
            if nat_lt(a2541, a2542) {
                let a2543 = v0;
                let a2544 = 6u64;
                if nat_eq(a2543, a2544) {
                    f91(0u64, f104(), 1u64, 1603u64)
                } else {
                    f2()
                }
            } else {
                let a2545 = v0;
                let a2546 = 7u64;
                if nat_eq(a2545, a2546) {
                    f83(1u64, f104())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f78(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a2547 = v0;
    let a2548 = 4u64;
    if nat_lt(a2547, a2548) {
        let a2549 = v0;
        let a2550 = 2u64;
        if nat_lt(a2549, a2550) {
            let a2551 = v0;
            let a2552 = 1u64;
            if nat_lt(a2551, a2552) {
                let a2553 = v0;
                let a2554 = 0u64;
                if nat_eq(a2553, a2554) {
                    Ok(f83(0u64, f122()))
                } else {
                    Ok(f2())
                }
            } else {
                let a2555 = v0;
                let a2556 = 1u64;
                if nat_eq(a2555, a2556) {
                    Ok(f83(2048u64, f119()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2557 = v0;
            let a2558 = 3u64;
            if nat_lt(a2557, a2558) {
                let a2559 = v0;
                let a2560 = 2u64;
                if nat_eq(a2559, a2560) {
                    Ok(f99(1573u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2561 = v0;
                let a2562 = 3u64;
                let _ = nat_eq(a2561, a2562);
                Ok(f2())
            }
        }
    } else {
        let a2563 = v0;
        let a2564 = 6u64;
        if nat_lt(a2563, a2564) {
            let a2565 = v0;
            let a2566 = 5u64;
            if nat_lt(a2565, a2566) {
                let a2567 = v0;
                let a2568 = 4u64;
                if nat_eq(a2567, a2568) {
                    Ok(f83(f100(), f150(f86())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a2569 = v0;
                let a2570 = 5u64;
                if nat_eq(a2569, a2570) {
                    Ok(f90(0u64, f86(), 1u64, f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a2571 = v0;
            let a2572 = 7u64;
            if nat_lt(a2571, a2572) {
                let a2573 = v0;
                let a2574 = 6u64;
                if nat_eq(a2573, a2574) {
                    Ok(f91(1u64, f86(), f157(), 1538u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a2575 = v0;
                let a2576 = 7u64;
                if nat_eq(a2575, a2576) {
                    Ok(f91(0u64, f152(), 0u64, 2560u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f79(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a2577 = v0;
    let a2578 = 0u64;
    if nat_eq(a2577, a2578) {
        f99(2048u64)
    } else {
        f2()
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f81() -> u64 {
    4294967315u64
}

pub fn f82() -> u64 {
    4294967316u64
}

pub fn f83(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f84() -> u64 {
    4294967317u64
}

pub fn f85() -> u64 {
    4294967318u64
}

pub fn f86() -> u64 {
    4294967323u64
}

pub fn f87(v0: u64) -> R<u64> {
    let a2582 = 8589980672u64;
    let a2583 = {
        let a2579 = v0;
        let a2580 = 4294967296u64;
        let a2581 = 0u64;
        nat_rem(a2579, a2580, a2581)
    };
    nat_add(a2582, a2583)
}

pub fn f88() -> u64 {
    4294967297u64
}

pub fn f89(v0: u64) -> R<u64> {
    let a2587 = 8589982720u64;
    let a2588 = {
        let a2584 = v0;
        let a2585 = 4294967296u64;
        let a2586 = 0u64;
        nat_rem(a2584, a2585, a2586)
    };
    nat_add(a2587, a2588)
}

pub fn f90(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f92(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f93() -> u64 {
    4294967337u64
}

pub fn f94() -> u64 {
    4294967299u64
}

pub fn f95() -> u64 {
    4294967338u64
}

pub fn f96() -> u64 {
    4294967319u64
}

pub fn f97() -> u64 {
    4294967320u64
}

pub fn f98() -> u64 {
    4294967350u64
}

pub fn f99(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f100() -> u64 {
    4294967321u64
}

pub fn f101(v0: u64) -> R<u64> {
    let a2592 = 8589976576u64;
    let a2593 = {
        let a2589 = v0;
        let a2590 = 4294967296u64;
        let a2591 = 0u64;
        nat_rem(a2589, a2590, a2591)
    };
    nat_add(a2592, a2593)
}

pub fn f102() -> u64 {
    4294967322u64
}

pub fn f103(v0: u64) -> R<u64> {
    let a2597 = 8589978624u64;
    let a2598 = {
        let a2594 = v0;
        let a2595 = 4294967296u64;
        let a2596 = 0u64;
        nat_rem(a2594, a2595, a2596)
    };
    nat_add(a2597, a2598)
}

pub fn f104() -> u64 {
    4294967324u64
}

pub fn f105() -> u64 {
    4294967351u64
}

pub fn f106() -> u64 {
    4294967336u64
}

pub fn f107() -> u64 {
    4294967335u64
}

pub fn f108() -> u64 {
    4294967326u64
}

pub fn f109() -> u64 {
    4294967305u64
}

pub fn f110() -> u64 {
    4294967325u64
}

pub fn f111() -> u64 {
    4294967303u64
}

pub fn f112(v0: u64) -> R<u64> {
    let a2602 = 8589950976u64;
    let a2603 = {
        let a2599 = v0;
        let a2600 = 4294967296u64;
        let a2601 = 0u64;
        nat_rem(a2599, a2600, a2601)
    };
    nat_add(a2602, a2603)
}

pub fn f113(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f114() -> u64 {
    4294967331u64
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f116() -> u64 {
    4294967343u64
}

pub fn f117(v0: u64) -> R<u64> {
    let a2607 = 21474836480u64;
    let a2608 = {
        let a2604 = v0;
        let a2605 = 4294967296u64;
        let a2606 = 0u64;
        nat_rem(a2604, a2605, a2606)
    };
    nat_add(a2607, a2608)
}

pub fn f118() -> u64 {
    4294967327u64
}

pub fn f119() -> u64 {
    4294967328u64
}

pub fn f120() -> u64 {
    4294967339u64
}

pub fn f121() -> u64 {
    4294967334u64
}

pub fn f122() -> u64 {
    4294967329u64
}

pub fn f123() -> u64 {
    4294967353u64
}

pub fn f124() -> u64 {
    4294967333u64
}

pub fn f125() -> u64 {
    4294967330u64
}

pub fn f126(v0: u64) -> R<u64> {
    let a2612 = 25769803776u64;
    let a2613 = {
        let a2609 = v0;
        let a2610 = 4294967296u64;
        let a2611 = 0u64;
        nat_rem(a2609, a2610, a2611)
    };
    nat_add(a2612, a2613)
}

pub fn f127() -> u64 {
    4294967332u64
}

pub fn f128(v0: u64) -> R<u64> {
    let a2617 = 8589967360u64;
    let a2618 = {
        let a2614 = v0;
        let a2615 = 4294967296u64;
        let a2616 = 0u64;
        nat_rem(a2614, a2615, a2616)
    };
    nat_add(a2617, a2618)
}

pub fn f129(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f130() -> u64 {
    4294967308u64
}

pub fn f131(v0: u64) -> R<u64> {
    let a2622 = 8589959168u64;
    let a2623 = {
        let a2619 = v0;
        let a2620 = 4294967296u64;
        let a2621 = 0u64;
        nat_rem(a2619, a2620, a2621)
    };
    nat_add(a2622, a2623)
}

pub fn f132() -> u64 {
    4294967357u64
}

pub fn f133(v0: u64) -> R<u64> {
    let a2627 = 8589988864u64;
    let a2628 = {
        let a2624 = v0;
        let a2625 = 4294967296u64;
        let a2626 = 0u64;
        nat_rem(a2624, a2625, a2626)
    };
    nat_add(a2627, a2628)
}

pub fn f134() -> u64 {
    4294967355u64
}

pub fn f135() -> u64 {
    4294967356u64
}

pub fn f136() -> u64 {
    4294967310u64
}

pub fn f137() -> u64 {
    4294967311u64
}

pub fn f138() -> u64 {
    4294967314u64
}

pub fn f139(v0: u64) -> R<u64> {
    let a2632 = 8589967360u64;
    let a2633 = {
        let a2629 = v0;
        let a2630 = 4294967296u64;
        let a2631 = 0u64;
        nat_rem(a2629, a2630, a2631)
    };
    nat_add(a2632, a2633)
}

pub fn f140() -> u64 {
    4294967296u64
}

pub fn f141(v0: u64) -> R<u64> {
    let a2637 = 8589976064u64;
    let a2638 = {
        let a2634 = v0;
        let a2635 = 4294967296u64;
        let a2636 = 0u64;
        nat_rem(a2634, a2635, a2636)
    };
    nat_add(a2637, a2638)
}

pub fn f142() -> u64 {
    4294967312u64
}

pub fn f143() -> u64 {
    4294967306u64
}

pub fn f144() -> u64 {
    4294967341u64
}

pub fn f145(v0: u64) -> R<u64> {
    let a2642 = 8589984768u64;
    let a2643 = {
        let a2639 = v0;
        let a2640 = 4294967296u64;
        let a2641 = 0u64;
        nat_rem(a2639, a2640, a2641)
    };
    nat_add(a2642, a2643)
}

pub fn f146(v0: u64) -> R<u64> {
    let a2647 = 8589985792u64;
    let a2648 = {
        let a2644 = v0;
        let a2645 = 4294967296u64;
        let a2646 = 0u64;
        nat_rem(a2644, a2645, a2646)
    };
    nat_add(a2647, a2648)
}

pub fn f147() -> u64 {
    4294967342u64
}

pub fn f148() -> u64 {
    4294967346u64
}

pub fn f149() -> u64 {
    4294967313u64
}

pub fn f150(v0: u64) -> R<u64> {
    let a2652 = 8589959168u64;
    let a2653 = {
        let a2649 = v0;
        let a2650 = 4294967296u64;
        let a2651 = 0u64;
        nat_rem(a2649, a2650, a2651)
    };
    nat_add(a2652, a2653)
}

pub fn f151(v0: u64) -> R<u64> {
    let a2657 = 8589984768u64;
    let a2658 = {
        let a2654 = v0;
        let a2655 = 4294967296u64;
        let a2656 = 0u64;
        nat_rem(a2654, a2655, a2656)
    };
    nat_add(a2657, a2658)
}

pub fn f152() -> u64 {
    4294967352u64
}

pub fn f153() -> u64 {
    4294967354u64
}

pub fn f154() -> u64 {
    4294967307u64
}

pub fn f155(v0: u64) -> R<u64> {
    let a2662 = 12884901888u64;
    let a2663 = {
        let a2659 = v0;
        let a2660 = 4294967296u64;
        let a2661 = 0u64;
        nat_rem(a2659, a2660, a2661)
    };
    nat_add(a2662, a2663)
}

pub fn f156(v0: u64) -> R<u64> {
    let a2667 = 8589950976u64;
    let a2668 = {
        let a2664 = v0;
        let a2665 = 4294967296u64;
        let a2666 = 0u64;
        nat_rem(a2664, a2665, a2666)
    };
    nat_add(a2667, a2668)
}

pub fn f157() -> u64 {
    4294967302u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
