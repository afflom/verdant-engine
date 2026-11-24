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
    let a2 = 3u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 1u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 0u64;
            if nat_eq(a5, a6) {
                f1(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a7 = v0;
            let a8 = 2u64;
            if nat_lt(a7, a8) {
                let a9 = v0;
                let a10 = 1u64;
                if nat_eq(a9, a10) {
                    f3(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a11 = v0;
                let a12 = 2u64;
                if nat_eq(a11, a12) {
                    f4(v1)
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a13 = v0;
        let a14 = 4u64;
        if nat_lt(a13, a14) {
            let a15 = v0;
            let a16 = 3u64;
            if nat_eq(a15, a16) {
                f5(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a17 = v0;
            let a18 = 5u64;
            if nat_lt(a17, a18) {
                let a19 = v0;
                let a20 = 4u64;
                if nat_eq(a19, a20) {
                    f6(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a21 = v0;
                let a22 = 5u64;
                if nat_eq(a21, a22) {
                    f7(v1)
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a23 = v0;
        let a24 = 8u64;
        let a25 = 0u64;
        nat_quot(a23, a24, a25)
    };
    let a26 = v1;
    let a27 = 2u64;
    if nat_lt(a26, a27) {
        let a28 = v1;
        let a29 = 1u64;
        if nat_lt(a28, a29) {
            let a30 = v1;
            let a31 = 0u64;
            if nat_eq(a30, a31) {
                f8({
                    let a32 = v0;
                    let a33 = 8u64;
                    let a34 = 0u64;
                    nat_rem(a32, a33, a34)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a35 = v1;
            let a36 = 1u64;
            if nat_eq(a35, a36) {
                Ok(f9({
                    let a37 = v0;
                    let a38 = 8u64;
                    let a39 = 0u64;
                    nat_rem(a37, a38, a39)
                }))
            } else {
                Ok(f2())
            }
        }
    } else {
        let a40 = v1;
        let a41 = 3u64;
        if nat_lt(a40, a41) {
            let a42 = v1;
            let a43 = 2u64;
            if nat_eq(a42, a43) {
                f10({
                    let a44 = v0;
                    let a45 = 8u64;
                    let a46 = 0u64;
                    nat_rem(a44, a45, a46)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a47 = v1;
            let a48 = 4u64;
            if nat_lt(a47, a48) {
                let a49 = v1;
                let a50 = 3u64;
                if nat_eq(a49, a50) {
                    f11({
                        let a51 = v0;
                        let a52 = 8u64;
                        let a53 = 0u64;
                        nat_rem(a51, a52, a53)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a54 = v1;
                let a55 = 4u64;
                if nat_eq(a54, a55) {
                    f12({
                        let a56 = v0;
                        let a57 = 8u64;
                        let a58 = 0u64;
                        nat_rem(a56, a57, a58)
                    })
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
        let a59 = v0;
        let a60 = 8u64;
        let a61 = 0u64;
        nat_quot(a59, a60, a61)
    };
    let a62 = v1;
    let a63 = 2u64;
    if nat_lt(a62, a63) {
        let a64 = v1;
        let a65 = 1u64;
        if nat_lt(a64, a65) {
            let a66 = v1;
            let a67 = 0u64;
            if nat_eq(a66, a67) {
                Ok(f13({
                    let a68 = v0;
                    let a69 = 8u64;
                    let a70 = 0u64;
                    nat_rem(a68, a69, a70)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a71 = v1;
            let a72 = 1u64;
            if nat_eq(a71, a72) {
                f14({
                    let a73 = v0;
                    let a74 = 8u64;
                    let a75 = 0u64;
                    nat_rem(a73, a74, a75)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a76 = v1;
        let a77 = 3u64;
        if nat_lt(a76, a77) {
            let a78 = v1;
            let a79 = 2u64;
            if nat_eq(a78, a79) {
                Ok(f15({
                    let a80 = v0;
                    let a81 = 8u64;
                    let a82 = 0u64;
                    nat_rem(a80, a81, a82)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a83 = v1;
            let a84 = 3u64;
            if nat_eq(a83, a84) {
                Ok(f16({
                    let a85 = v0;
                    let a86 = 8u64;
                    let a87 = 0u64;
                    nat_rem(a85, a86, a87)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a88 = v0;
        let a89 = 8u64;
        let a90 = 0u64;
        nat_quot(a88, a89, a90)
    };
    let a91 = v1;
    let a92 = 2u64;
    if nat_lt(a91, a92) {
        let a93 = v1;
        let a94 = 1u64;
        if nat_lt(a93, a94) {
            let a95 = v1;
            let a96 = 0u64;
            if nat_eq(a95, a96) {
                f17({
                    let a97 = v0;
                    let a98 = 8u64;
                    let a99 = 0u64;
                    nat_rem(a97, a98, a99)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a100 = v1;
            let a101 = 1u64;
            if nat_eq(a100, a101) {
                f18({
                    let a102 = v0;
                    let a103 = 8u64;
                    let a104 = 0u64;
                    nat_rem(a102, a103, a104)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a105 = v1;
        let a106 = 3u64;
        if nat_lt(a105, a106) {
            let a107 = v1;
            let a108 = 2u64;
            if nat_eq(a107, a108) {
                f19({
                    let a109 = v0;
                    let a110 = 8u64;
                    let a111 = 0u64;
                    nat_rem(a109, a110, a111)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a112 = v1;
            let a113 = 3u64;
            if nat_eq(a112, a113) {
                f20({
                    let a114 = v0;
                    let a115 = 8u64;
                    let a116 = 0u64;
                    nat_rem(a114, a115, a116)
                })
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a117 = v0;
        let a118 = 8u64;
        let a119 = 0u64;
        nat_quot(a117, a118, a119)
    };
    let a120 = v1;
    let a121 = 2u64;
    if nat_lt(a120, a121) {
        let a122 = v1;
        let a123 = 1u64;
        if nat_lt(a122, a123) {
            let a124 = v1;
            let a125 = 0u64;
            if nat_eq(a124, a125) {
                Ok(f21({
                    let a126 = v0;
                    let a127 = 8u64;
                    let a128 = 0u64;
                    nat_rem(a126, a127, a128)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a129 = v1;
            let a130 = 1u64;
            if nat_eq(a129, a130) {
                f14({
                    let a131 = v0;
                    let a132 = 8u64;
                    let a133 = 0u64;
                    nat_rem(a131, a132, a133)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a134 = v1;
        let a135 = 3u64;
        if nat_lt(a134, a135) {
            let a136 = v1;
            let a137 = 2u64;
            if nat_eq(a136, a137) {
                Ok(f22({
                    let a138 = v0;
                    let a139 = 8u64;
                    let a140 = 0u64;
                    nat_rem(a138, a139, a140)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a141 = v1;
            let a142 = 3u64;
            if nat_eq(a141, a142) {
                Ok(f16({
                    let a143 = v0;
                    let a144 = 8u64;
                    let a145 = 0u64;
                    nat_rem(a143, a144, a145)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a146 = v0;
        let a147 = 8u64;
        let a148 = 0u64;
        nat_quot(a146, a147, a148)
    };
    let a149 = v1;
    let a150 = 1u64;
    if nat_lt(a149, a150) {
        let a151 = v1;
        let a152 = 0u64;
        if nat_eq(a151, a152) {
            f8({
                let a153 = v0;
                let a154 = 8u64;
                let a155 = 0u64;
                nat_rem(a153, a154, a155)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a156 = v1;
        let a157 = 2u64;
        if nat_lt(a156, a157) {
            let a158 = v1;
            let a159 = 1u64;
            if nat_eq(a158, a159) {
                f23({
                    let a160 = v0;
                    let a161 = 8u64;
                    let a162 = 0u64;
                    nat_rem(a160, a161, a162)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a163 = v1;
            let a164 = 2u64;
            if nat_eq(a163, a164) {
                Ok(f24({
                    let a165 = v0;
                    let a166 = 8u64;
                    let a167 = 0u64;
                    nat_rem(a165, a166, a167)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a168 = v0;
        let a169 = 8u64;
        let a170 = 0u64;
        nat_quot(a168, a169, a170)
    };
    let a171 = v1;
    let a172 = 2u64;
    if nat_lt(a171, a172) {
        let a173 = v1;
        let a174 = 1u64;
        if nat_lt(a173, a174) {
            let a175 = v1;
            let a176 = 0u64;
            if nat_eq(a175, a176) {
                Ok(f21({
                    let a177 = v0;
                    let a178 = 8u64;
                    let a179 = 0u64;
                    nat_rem(a177, a178, a179)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a180 = v1;
            let a181 = 1u64;
            if nat_eq(a180, a181) {
                f14({
                    let a182 = v0;
                    let a183 = 8u64;
                    let a184 = 0u64;
                    nat_rem(a182, a183, a184)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a185 = v1;
        let a186 = 3u64;
        if nat_lt(a185, a186) {
            let a187 = v1;
            let a188 = 2u64;
            if nat_eq(a187, a188) {
                Ok(f15({
                    let a189 = v0;
                    let a190 = 8u64;
                    let a191 = 0u64;
                    nat_rem(a189, a190, a191)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a192 = v1;
            let a193 = 3u64;
            if nat_eq(a192, a193) {
                Ok(f16({
                    let a194 = v0;
                    let a195 = 8u64;
                    let a196 = 0u64;
                    nat_rem(a194, a195, a196)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a197 = v0;
    let a198 = 4u64;
    if nat_lt(a197, a198) {
        let a199 = v0;
        let a200 = 2u64;
        if nat_lt(a199, a200) {
            let a201 = v0;
            let a202 = 1u64;
            if nat_lt(a201, a202) {
                let a203 = v0;
                let a204 = 0u64;
                if nat_eq(a203, a204) {
                    Ok(f25(2u64, f26(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a205 = v0;
                let a206 = 1u64;
                if nat_eq(a205, a206) {
                    Ok(f27(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a207 = v0;
            let a208 = 3u64;
            if nat_lt(a207, a208) {
                let a209 = v0;
                let a210 = 2u64;
                if nat_eq(a209, a210) {
                    Ok(f28(6u64, f26(), 0u64, f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a211 = v0;
                let a212 = 3u64;
                if nat_eq(a211, a212) {
                    Ok(f29(0u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a213 = v0;
        let a214 = 6u64;
        if nat_lt(a213, a214) {
            let a215 = v0;
            let a216 = 5u64;
            if nat_lt(a215, a216) {
                let a217 = v0;
                let a218 = 4u64;
                if nat_eq(a217, a218) {
                    Ok(f31(1u64, 255u64, f32(f30())?, f32(f30())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a219 = v0;
                let a220 = 5u64;
                if nat_eq(a219, a220) {
                    Ok(f31(0u64, f30(), 1u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a221 = v0;
            let a222 = 7u64;
            if nat_lt(a221, a222) {
                let a223 = v0;
                let a224 = 6u64;
                if nat_eq(a223, a224) {
                    Ok(f25(1u64, f30(), 4u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a225 = v0;
                let a226 = 7u64;
                if nat_eq(a225, a226) {
                    Ok(f29(f26(), f33(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a227 = v0;
    let a228 = 4u64;
    if nat_lt(a227, a228) {
        let a229 = v0;
        let a230 = 2u64;
        if nat_lt(a229, a230) {
            let a231 = v0;
            let a232 = 1u64;
            if nat_lt(a231, a232) {
                let a233 = v0;
                let a234 = 0u64;
                if nat_eq(a233, a234) {
                    f25(2u64, f35(), f26(), 3082u64)
                } else {
                    f2()
                }
            } else {
                let a235 = v0;
                let a236 = 1u64;
                if nat_eq(a235, a236) {
                    f27(3083u64)
                } else {
                    f2()
                }
            }
        } else {
            let a237 = v0;
            let a238 = 3u64;
            if nat_lt(a237, a238) {
                let a239 = v0;
                let a240 = 2u64;
                if nat_eq(a239, a240) {
                    f29(f26(), f35())
                } else {
                    f2()
                }
            } else {
                let a241 = v0;
                let a242 = 3u64;
                if nat_eq(a241, a242) {
                    f29(f36(), f37())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a243 = v0;
        let a244 = 6u64;
        if nat_lt(a243, a244) {
            let a245 = v0;
            let a246 = 5u64;
            if nat_lt(a245, a246) {
                let a247 = v0;
                let a248 = 4u64;
                if nat_eq(a247, a248) {
                    f25(0u64, f38(), 0u64, 3086u64)
                } else {
                    f2()
                }
            } else {
                let a249 = v0;
                let a250 = 5u64;
                if nat_eq(a249, a250) {
                    f29(f34(), f37())
                } else {
                    f2()
                }
            }
        } else {
            let a251 = v0;
            let a252 = 7u64;
            if nat_lt(a251, a252) {
                let a253 = v0;
                let a254 = 6u64;
                if nat_eq(a253, a254) {
                    f31(0u64, f36(), 1u64, f36())
                } else {
                    f2()
                }
            } else {
                let a255 = v0;
                let a256 = 7u64;
                if nat_eq(a255, a256) {
                    f29(0u64, f39())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a257 = v0;
    let a258 = 4u64;
    if nat_lt(a257, a258) {
        let a259 = v0;
        let a260 = 2u64;
        if nat_lt(a259, a260) {
            let a261 = v0;
            let a262 = 1u64;
            if nat_lt(a261, a262) {
                let a263 = v0;
                let a264 = 0u64;
                if nat_eq(a263, a264) {
                    Ok(f31(2u64, f37(), 8u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a265 = v0;
                let a266 = 1u64;
                if nat_eq(a265, a266) {
                    Ok(f31(0u64, f40(), f39(), f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a267 = v0;
            let a268 = 3u64;
            if nat_lt(a267, a268) {
                let a269 = v0;
                let a270 = 2u64;
                if nat_eq(a269, a270) {
                    Ok(f41(7u64, f40(), 0u64, f42(f39())?, f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a271 = v0;
                let a272 = 3u64;
                if nat_eq(a271, a272) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a273 = v0;
        let a274 = 6u64;
        if nat_lt(a273, a274) {
            let a275 = v0;
            let a276 = 5u64;
            if nat_lt(a275, a276) {
                let a277 = v0;
                let a278 = 4u64;
                if nat_eq(a277, a278) {
                    Ok(f25(0u64, f38(), 1u64, 3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a279 = v0;
                let a280 = 5u64;
                if nat_eq(a279, a280) {
                    Ok(f31(2u64, f34(), 8u64, f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a281 = v0;
            let a282 = 7u64;
            if nat_lt(a281, a282) {
                let a283 = v0;
                let a284 = 6u64;
                if nat_eq(a283, a284) {
                    Ok(f31(0u64, f40(), f39(), f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a285 = v0;
                let a286 = 7u64;
                if nat_eq(a285, a286) {
                    Ok(f41(7u64, f40(), 1u64, f42(f39())?, f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a287 = v0;
    let a288 = 4u64;
    if nat_lt(a287, a288) {
        let a289 = v0;
        let a290 = 2u64;
        if nat_lt(a289, a290) {
            let a291 = v0;
            let a292 = 1u64;
            if nat_lt(a291, a292) {
                let a293 = v0;
                let a294 = 0u64;
                if nat_eq(a293, a294) {
                    Ok(f47(f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a295 = v0;
                let a296 = 1u64;
                if nat_eq(a295, a296) {
                    Ok(f31(0u64, f39(), 1u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a297 = v0;
            let a298 = 3u64;
            if nat_lt(a297, a298) {
                let a299 = v0;
                let a300 = 2u64;
                if nat_eq(a299, a300) {
                    Ok(f25(1u64, f39(), 8u64, 3088u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a301 = v0;
                let a302 = 3u64;
                if nat_eq(a301, a302) {
                    Ok(f29(0u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a303 = v0;
        let a304 = 6u64;
        if nat_lt(a303, a304) {
            let a305 = v0;
            let a306 = 5u64;
            if nat_lt(a305, a306) {
                let a307 = v0;
                let a308 = 4u64;
                if nat_eq(a307, a308) {
                    Ok(f48(9u64, f37(), f30(), 0u64, f43(), f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a309 = v0;
                let a310 = 5u64;
                if nat_eq(a309, a310) {
                    Ok(f29(f32(f30())?, f45()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a311 = v0;
            let a312 = 7u64;
            if nat_lt(a311, a312) {
                let a313 = v0;
                let a314 = 6u64;
                if nat_eq(a313, a314) {
                    Ok(f25(0u64, f38(), 0u64, 3106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a315 = v0;
                let a316 = 7u64;
                if nat_eq(a315, a316) {
                    Ok(f31(4u64, f45(), 128u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a317 = v0;
    let a318 = 4u64;
    if nat_lt(a317, a318) {
        let a319 = v0;
        let a320 = 2u64;
        if nat_lt(a319, a320) {
            let a321 = v0;
            let a322 = 1u64;
            if nat_lt(a321, a322) {
                let a323 = v0;
                let a324 = 0u64;
                if nat_eq(a323, a324) {
                    Ok(f49(f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a325 = v0;
                let a326 = 1u64;
                if nat_eq(a325, a326) {
                    Ok(f27(3107u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a327 = v0;
            let a328 = 3u64;
            if nat_lt(a327, a328) {
                let a329 = v0;
                let a330 = 2u64;
                if nat_eq(a329, a330) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a331 = v0;
                let a332 = 3u64;
                if nat_eq(a331, a332) {
                    Ok(f31(0u64, f30(), 1u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a333 = v0;
        let a334 = 6u64;
        if nat_lt(a333, a334) {
            let a335 = v0;
            let a336 = 5u64;
            if nat_lt(a335, a336) {
                let a337 = v0;
                let a338 = 4u64;
                if nat_eq(a337, a338) {
                    Ok(f25(1u64, f30(), 4u64, 3100u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a339 = v0;
                let a340 = 5u64;
                if nat_eq(a339, a340) {
                    Ok(f31(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a341 = v0;
            let a342 = 7u64;
            if nat_lt(a341, a342) {
                let a343 = v0;
                let a344 = 6u64;
                if nat_eq(a343, a344) {
                    Ok(f25(1u64, f34(), 8u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a345 = v0;
                let a346 = 7u64;
                if nat_eq(a345, a346) {
                    Ok(f27(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a347 = v0;
    let a348 = 4u64;
    if nat_lt(a347, a348) {
        let a349 = v0;
        let a350 = 2u64;
        if nat_lt(a349, a350) {
            let a351 = v0;
            let a352 = 1u64;
            if nat_lt(a351, a352) {
                let a353 = v0;
                let a354 = 0u64;
                if nat_eq(a353, a354) {
                    f25(0u64, f35(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a355 = v0;
                let a356 = 1u64;
                if nat_eq(a355, a356) {
                    f28(3u64, f35(), 1106771968u64, f26())
                } else {
                    f2()
                }
            }
        } else {
            let a357 = v0;
            let a358 = 3u64;
            if nat_lt(a357, a358) {
                let a359 = v0;
                let a360 = 2u64;
                if nat_eq(a359, a360) {
                    f29(1065458074u64, f50())
                } else {
                    f2()
                }
            } else {
                let a361 = v0;
                let a362 = 3u64;
                if nat_eq(a361, a362) {
                    f25(0u64, f38(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a363 = v0;
        let a364 = 6u64;
        if nat_lt(a363, a364) {
            let a365 = v0;
            let a366 = 5u64;
            if nat_lt(a365, a366) {
                let a367 = v0;
                let a368 = 4u64;
                if nat_eq(a367, a368) {
                    f29(1065630040u64, f50())
                } else {
                    f2()
                }
            } else {
                let a369 = v0;
                let a370 = 5u64;
                if nat_eq(a369, a370) {
                    f28(2u64, f26(), f50(), f51())
                } else {
                    f2()
                }
            }
        } else {
            let a371 = v0;
            let a372 = 7u64;
            if nat_lt(a371, a372) {
                let a373 = v0;
                let a374 = 6u64;
                if nat_eq(a373, a374) {
                    f52(f51(), f51())
                } else {
                    f2()
                }
            } else {
                let a375 = v0;
                let a376 = 7u64;
                if nat_eq(a375, a376) {
                    f31(4u64, f51(), 256u64, f45())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a377 = v0;
    let a378 = 4u64;
    if nat_lt(a377, a378) {
        let a379 = v0;
        let a380 = 2u64;
        if nat_lt(a379, a380) {
            let a381 = v0;
            let a382 = 1u64;
            if nat_lt(a381, a382) {
                let a383 = v0;
                let a384 = 0u64;
                if nat_eq(a383, a384) {
                    Ok(f29(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a385 = v0;
                let a386 = 1u64;
                if nat_eq(a385, a386) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a387 = v0;
            let a388 = 3u64;
            if nat_lt(a387, a388) {
                let a389 = v0;
                let a390 = 2u64;
                if nat_eq(a389, a390) {
                    Ok(f31(3u64, f51(), 256u64, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a391 = v0;
                let a392 = 3u64;
                if nat_eq(a391, a392) {
                    Ok(f31(0u64, f43(), 1u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a393 = v0;
        let a394 = 6u64;
        if nat_lt(a393, a394) {
            let a395 = v0;
            let a396 = 5u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 4u64;
                if nat_eq(a397, a398) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a399 = v0;
                let a400 = 5u64;
                if nat_eq(a399, a400) {
                    Ok(f28(3u64, 1065353216u64, f26(), f53()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a401 = v0;
            let a402 = 7u64;
            if nat_lt(a401, a402) {
                let a403 = v0;
                let a404 = 6u64;
                if nat_eq(a403, a404) {
                    Ok(f29(0u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a405 = v0;
                let a406 = 7u64;
                if nat_eq(a405, a406) {
                    Ok(f28(2u64, f53(), f33(f34())?, f54()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a407 = v0;
    let a408 = 4u64;
    if nat_lt(a407, a408) {
        let a409 = v0;
        let a410 = 2u64;
        if nat_lt(a409, a410) {
            let a411 = v0;
            let a412 = 1u64;
            if nat_lt(a411, a412) {
                let a413 = v0;
                let a414 = 0u64;
                if nat_eq(a413, a414) {
                    f28(1u64, f54(), 1065353216u64, f54())
                } else {
                    f2()
                }
            } else {
                let a415 = v0;
                let a416 = 1u64;
                if nat_eq(a415, a416) {
                    f28(2u64, 1056964608u64, f54(), f54())
                } else {
                    f2()
                }
            }
        } else {
            let a417 = v0;
            let a418 = 3u64;
            if nat_lt(a417, a418) {
                let a419 = v0;
                let a420 = 2u64;
                if nat_eq(a419, a420) {
                    f28(0u64, f54(), 1262485504u64, f54())
                } else {
                    f2()
                }
            } else {
                let a421 = v0;
                let a422 = 3u64;
                if nat_eq(a421, a422) {
                    f55(2u64, f54(), 15u64, 0u64, f45())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a423 = v0;
        let a424 = 6u64;
        if nat_lt(a423, a424) {
            let a425 = v0;
            let a426 = 5u64;
            if nat_lt(a425, a426) {
                let a427 = v0;
                let a428 = 4u64;
                if nat_eq(a427, a428) {
                    f48(8u64, f34(), 0u64, 0u64, f43(), f44())
                } else {
                    f2()
                }
            } else {
                let a429 = v0;
                let a430 = 5u64;
                if nat_eq(a429, a430) {
                    f47(f43(), f44(), f45())
                } else {
                    f2()
                }
            }
        } else {
            let a431 = v0;
            let a432 = 7u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 6u64;
                if nat_eq(a433, a434) {
                    f31(0u64, f34(), 1u64, f34())
                } else {
                    f2()
                }
            } else {
                let a435 = v0;
                let a436 = 7u64;
                if nat_eq(a435, a436) {
                    f25(1u64, f34(), 8u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a437 = v0;
    let a438 = 1u64;
    if nat_lt(a437, a438) {
        let a439 = v0;
        let a440 = 0u64;
        if nat_eq(a439, a440) {
            f56()
        } else {
            f2()
        }
    } else {
        let a441 = v0;
        let a442 = 1u64;
        if nat_eq(a441, a442) {
            f57()
        } else {
            f2()
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a443 = v0;
    let a444 = 4u64;
    if nat_lt(a443, a444) {
        let a445 = v0;
        let a446 = 2u64;
        if nat_lt(a445, a446) {
            let a447 = v0;
            let a448 = 1u64;
            if nat_lt(a447, a448) {
                let a449 = v0;
                let a450 = 0u64;
                if nat_eq(a449, a450) {
                    Ok(f25(2u64, f26(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a451 = v0;
                let a452 = 1u64;
                if nat_eq(a451, a452) {
                    Ok(f27(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a453 = v0;
            let a454 = 3u64;
            if nat_lt(a453, a454) {
                let a455 = v0;
                let a456 = 2u64;
                if nat_eq(a455, a456) {
                    Ok(f28(6u64, f26(), 0u64, f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a457 = v0;
                let a458 = 3u64;
                if nat_eq(a457, a458) {
                    Ok(f29(0u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a459 = v0;
        let a460 = 6u64;
        if nat_lt(a459, a460) {
            let a461 = v0;
            let a462 = 5u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 4u64;
                if nat_eq(a463, a464) {
                    Ok(f31(1u64, 255u64, f32(f30())?, f32(f30())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a465 = v0;
                let a466 = 5u64;
                if nat_eq(a465, a466) {
                    Ok(f31(0u64, f30(), 1u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a467 = v0;
            let a468 = 7u64;
            if nat_lt(a467, a468) {
                let a469 = v0;
                let a470 = 6u64;
                if nat_eq(a469, a470) {
                    Ok(f25(1u64, f30(), 2u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a471 = v0;
                let a472 = 7u64;
                if nat_eq(a471, a472) {
                    Ok(f29(f26(), f33(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a473 = v0;
    let a474 = 4u64;
    if nat_lt(a473, a474) {
        let a475 = v0;
        let a476 = 2u64;
        if nat_lt(a475, a476) {
            let a477 = v0;
            let a478 = 1u64;
            if nat_lt(a477, a478) {
                let a479 = v0;
                let a480 = 0u64;
                if nat_eq(a479, a480) {
                    Ok(f25(2u64, f35(), f26(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a481 = v0;
                let a482 = 1u64;
                if nat_eq(a481, a482) {
                    Ok(f27(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a483 = v0;
            let a484 = 3u64;
            if nat_lt(a483, a484) {
                let a485 = v0;
                let a486 = 2u64;
                if nat_eq(a485, a486) {
                    Ok(f29(f26(), f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a487 = v0;
                let a488 = 3u64;
                if nat_eq(a487, a488) {
                    Ok(f29(0u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a489 = v0;
        let a490 = 6u64;
        if nat_lt(a489, a490) {
            let a491 = v0;
            let a492 = 5u64;
            if nat_lt(a491, a492) {
                let a493 = v0;
                let a494 = 4u64;
                if nat_eq(a493, a494) {
                    Ok(f31(2u64, f34(), 2u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a495 = v0;
                let a496 = 5u64;
                if nat_eq(a495, a496) {
                    Ok(f31(0u64, f40(), f39(), f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a497 = v0;
            let a498 = 7u64;
            if nat_lt(a497, a498) {
                let a499 = v0;
                let a500 = 6u64;
                if nat_eq(a499, a500) {
                    Ok(f41(7u64, f40(), 0u64, f42(f39())?, f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a501 = v0;
                let a502 = 7u64;
                if nat_eq(a501, a502) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a503 = v0;
    let a504 = 4u64;
    if nat_lt(a503, a504) {
        let a505 = v0;
        let a506 = 2u64;
        if nat_lt(a505, a506) {
            let a507 = v0;
            let a508 = 1u64;
            if nat_lt(a507, a508) {
                let a509 = v0;
                let a510 = 0u64;
                if nat_eq(a509, a510) {
                    Ok(f41(7u64, f40(), 1u64, f42(f39())?, f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a511 = v0;
                let a512 = 1u64;
                if nat_eq(a511, a512) {
                    Ok(f47(f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a513 = v0;
            let a514 = 3u64;
            if nat_lt(a513, a514) {
                let a515 = v0;
                let a516 = 2u64;
                if nat_eq(a515, a516) {
                    Ok(f25(0u64, f38(), 0u64, 3095u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a517 = v0;
                let a518 = 3u64;
                if nat_eq(a517, a518) {
                    Ok(f31(4u64, f32(f39())?, 128u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a519 = v0;
        let a520 = 6u64;
        if nat_lt(a519, a520) {
            let a521 = v0;
            let a522 = 5u64;
            if nat_lt(a521, a522) {
                let a523 = v0;
                let a524 = 4u64;
                if nat_eq(a523, a524) {
                    Ok(f29(1u64, f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a525 = v0;
                let a526 = 5u64;
                if nat_eq(a525, a526) {
                    Ok(f47(f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a527 = v0;
            let a528 = 7u64;
            if nat_lt(a527, a528) {
                let a529 = v0;
                let a530 = 6u64;
                if nat_eq(a529, a530) {
                    Ok(f27(3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a531 = v0;
                let a532 = 7u64;
                if nat_eq(a531, a532) {
                    Ok(f31(0u64, f40(), 34u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a533 = v0;
    let a534 = 3u64;
    if nat_lt(a533, a534) {
        let a535 = v0;
        let a536 = 1u64;
        if nat_lt(a535, a536) {
            let a537 = v0;
            let a538 = 0u64;
            if nat_eq(a537, a538) {
                Ok(f29(f32(f39())?, f46(f43())?))
            } else {
                Ok(f2())
            }
        } else {
            let a539 = v0;
            let a540 = 2u64;
            if nat_lt(a539, a540) {
                let a541 = v0;
                let a542 = 1u64;
                if nat_eq(a541, a542) {
                    Ok(f31(0u64, f39(), 1u64, f39()))
                } else {
                    Ok(f2())
                }
            } else {
                let a543 = v0;
                let a544 = 2u64;
                if nat_eq(a543, a544) {
                    Ok(f25(1u64, f39(), 2u64, 3084u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a545 = v0;
        let a546 = 4u64;
        if nat_lt(a545, a546) {
            let a547 = v0;
            let a548 = 3u64;
            if nat_eq(a547, a548) {
                Ok(f31(0u64, f34(), 1u64, f34()))
            } else {
                Ok(f2())
            }
        } else {
            let a549 = v0;
            let a550 = 5u64;
            if nat_lt(a549, a550) {
                let a551 = v0;
                let a552 = 4u64;
                if nat_eq(a551, a552) {
                    Ok(f25(1u64, f34(), 16u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a553 = v0;
                let a554 = 5u64;
                if nat_eq(a553, a554) {
                    Ok(f27(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a555 = v0;
    let a556 = 4u64;
    if nat_lt(a555, a556) {
        let a557 = v0;
        let a558 = 2u64;
        if nat_lt(a557, a558) {
            let a559 = v0;
            let a560 = 1u64;
            if nat_lt(a559, a560) {
                let a561 = v0;
                let a562 = 0u64;
                if nat_eq(a561, a562) {
                    f25(0u64, f35(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a563 = v0;
                let a564 = 1u64;
                if nat_eq(a563, a564) {
                    f28(3u64, f35(), 1106771968u64, f26())
                } else {
                    f2()
                }
            }
        } else {
            let a565 = v0;
            let a566 = 3u64;
            if nat_lt(a565, a566) {
                let a567 = v0;
                let a568 = 2u64;
                if nat_eq(a567, a568) {
                    f29(1065353216u64, f50())
                } else {
                    f2()
                }
            } else {
                let a569 = v0;
                let a570 = 3u64;
                if nat_eq(a569, a570) {
                    f25(0u64, f38(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a571 = v0;
        let a572 = 6u64;
        if nat_lt(a571, a572) {
            let a573 = v0;
            let a574 = 5u64;
            if nat_lt(a573, a574) {
                let a575 = v0;
                let a576 = 4u64;
                if nat_eq(a575, a576) {
                    f29(1065143501u64, f50())
                } else {
                    f2()
                }
            } else {
                let a577 = v0;
                let a578 = 5u64;
                if nat_eq(a577, a578) {
                    f28(2u64, f26(), f50(), f51())
                } else {
                    f2()
                }
            }
        } else {
            let a579 = v0;
            let a580 = 7u64;
            if nat_lt(a579, a580) {
                let a581 = v0;
                let a582 = 6u64;
                if nat_eq(a581, a582) {
                    f52(f51(), f51())
                } else {
                    f2()
                }
            } else {
                let a583 = v0;
                let a584 = 7u64;
                if nat_eq(a583, a584) {
                    f31(4u64, f51(), 256u64, f45())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a585 = v0;
    let a586 = 4u64;
    if nat_lt(a585, a586) {
        let a587 = v0;
        let a588 = 2u64;
        if nat_lt(a587, a588) {
            let a589 = v0;
            let a590 = 1u64;
            if nat_lt(a589, a590) {
                let a591 = v0;
                let a592 = 0u64;
                if nat_eq(a591, a592) {
                    f28(1u64, f54(), 1065353216u64, f54())
                } else {
                    f2()
                }
            } else {
                let a593 = v0;
                let a594 = 1u64;
                if nat_eq(a593, a594) {
                    f28(2u64, 1056964608u64, f54(), f54())
                } else {
                    f2()
                }
            }
        } else {
            let a595 = v0;
            let a596 = 3u64;
            if nat_lt(a595, a596) {
                let a597 = v0;
                let a598 = 2u64;
                if nat_eq(a597, a598) {
                    f28(0u64, f54(), 1262485504u64, f54())
                } else {
                    f2()
                }
            } else {
                let a599 = v0;
                let a600 = 3u64;
                if nat_eq(a599, a600) {
                    f55(2u64, f54(), 15u64, 0u64, f45())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a601 = v0;
        let a602 = 6u64;
        if nat_lt(a601, a602) {
            let a603 = v0;
            let a604 = 5u64;
            if nat_lt(a603, a604) {
                let a605 = v0;
                let a606 = 4u64;
                if nat_eq(a605, a606) {
                    f48(8u64, f34(), 0u64, 0u64, f43(), f44())
                } else {
                    f2()
                }
            } else {
                let a607 = v0;
                let a608 = 5u64;
                if nat_eq(a607, a608) {
                    f47(f43(), f44(), f45())
                } else {
                    f2()
                }
            }
        } else {
            let a609 = v0;
            let a610 = 7u64;
            if nat_lt(a609, a610) {
                let a611 = v0;
                let a612 = 6u64;
                if nat_eq(a611, a612) {
                    f31(0u64, f34(), 1u64, f34())
                } else {
                    f2()
                }
            } else {
                let a613 = v0;
                let a614 = 7u64;
                if nat_eq(a613, a614) {
                    f25(1u64, f34(), 16u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a615 = v0;
    let a616 = 4u64;
    if nat_lt(a615, a616) {
        let a617 = v0;
        let a618 = 2u64;
        if nat_lt(a617, a618) {
            let a619 = v0;
            let a620 = 1u64;
            if nat_lt(a619, a620) {
                let a621 = v0;
                let a622 = 0u64;
                if nat_eq(a621, a622) {
                    Ok(f25(2u64, f35(), f26(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a623 = v0;
                let a624 = 1u64;
                if nat_eq(a623, a624) {
                    Ok(f27(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a625 = v0;
            let a626 = 3u64;
            if nat_lt(a625, a626) {
                let a627 = v0;
                let a628 = 2u64;
                if nat_eq(a627, a628) {
                    Ok(f29(f26(), f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a629 = v0;
                let a630 = 3u64;
                if nat_eq(a629, a630) {
                    Ok(f29(0u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a631 = v0;
        let a632 = 6u64;
        if nat_lt(a631, a632) {
            let a633 = v0;
            let a634 = 5u64;
            if nat_lt(a633, a634) {
                let a635 = v0;
                let a636 = 4u64;
                if nat_eq(a635, a636) {
                    Ok(f41(7u64, f34(), f39(), f42(f39())?, f43(), f44(), f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a637 = v0;
                let a638 = 5u64;
                if nat_eq(a637, a638) {
                    Ok(f29(f45(), f46(f43())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a639 = v0;
            let a640 = 7u64;
            if nat_lt(a639, a640) {
                let a641 = v0;
                let a642 = 6u64;
                if nat_eq(a641, a642) {
                    Ok(f48(9u64, f34(), f39(), 0u64, f43(), f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a643 = v0;
                let a644 = 7u64;
                if nat_eq(a643, a644) {
                    Ok(f31(4u64, f32(f39())?, 128u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a645 = v0;
    let a646 = 3u64;
    if nat_lt(a645, a646) {
        let a647 = v0;
        let a648 = 1u64;
        if nat_lt(a647, a648) {
            let a649 = v0;
            let a650 = 0u64;
            if nat_eq(a649, a650) {
                f49(f43(), f44(), f45())
            } else {
                f2()
            }
        } else {
            let a651 = v0;
            let a652 = 2u64;
            if nat_lt(a651, a652) {
                let a653 = v0;
                let a654 = 1u64;
                if nat_eq(a653, a654) {
                    f31(0u64, f39(), 1u64, f39())
                } else {
                    f2()
                }
            } else {
                let a655 = v0;
                let a656 = 2u64;
                if nat_eq(a655, a656) {
                    f25(1u64, f39(), 4u64, 3084u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a657 = v0;
        let a658 = 4u64;
        if nat_lt(a657, a658) {
            let a659 = v0;
            let a660 = 3u64;
            if nat_eq(a659, a660) {
                f31(0u64, f34(), 1u64, f34())
            } else {
                f2()
            }
        } else {
            let a661 = v0;
            let a662 = 5u64;
            if nat_lt(a661, a662) {
                let a663 = v0;
                let a664 = 4u64;
                if nat_eq(a663, a664) {
                    f25(1u64, f34(), 8u64, 512u64)
                } else {
                    f2()
                }
            } else {
                let a665 = v0;
                let a666 = 5u64;
                if nat_eq(a665, a666) {
                    f27(3584u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f26() -> u64 {
    4294967316u64
}

pub fn f27(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f29(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f30() -> u64 {
    4294967304u64
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f32(v0: u64) -> R<u64> {
    let a670 = 8589975552u64;
    let a671 = {
        let a667 = v0;
        let a668 = 4294967296u64;
        let a669 = 0u64;
        nat_rem(a667, a668, a669)
    };
    nat_add(a670, a671)
}

pub fn f33(v0: u64) -> R<u64> {
    let a675 = 8589984768u64;
    let a676 = {
        let a672 = v0;
        let a673 = 4294967296u64;
        let a674 = 0u64;
        nat_rem(a672, a673, a674)
    };
    nat_add(a675, a676)
}

pub fn f34() -> u64 {
    4294967301u64
}

pub fn f35() -> u64 {
    4294967347u64
}

pub fn f36() -> u64 {
    4294967302u64
}

pub fn f37() -> u64 {
    4294967346u64
}

pub fn f38() -> u64 {
    4294967297u64
}

pub fn f39() -> u64 {
    4294967323u64
}

pub fn f40() -> u64 {
    4294967341u64
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f42(v0: u64) -> R<u64> {
    let a680 = 8589980672u64;
    let a681 = {
        let a677 = v0;
        let a678 = 4294967296u64;
        let a679 = 0u64;
        nat_rem(a677, a678, a679)
    };
    nat_add(a680, a681)
}

pub fn f43() -> u64 {
    4294967344u64
}

pub fn f44() -> u64 {
    4294967345u64
}

pub fn f45() -> u64 {
    4294967331u64
}

pub fn f46(v0: u64) -> R<u64> {
    let a685 = 30064771072u64;
    let a686 = {
        let a682 = v0;
        let a683 = 4294967296u64;
        let a684 = 0u64;
        nat_rem(a682, a683, a684)
    };
    nat_add(a685, a686)
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (14u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (15u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f50() -> u64 {
    4294967357u64
}

pub fn f51() -> u64 {
    4294967337u64
}

pub fn f52(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (7u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f53() -> u64 {
    4294967319u64
}

pub fn f54() -> u64 {
    4294967338u64
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f56() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (1u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f57() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (2u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}
