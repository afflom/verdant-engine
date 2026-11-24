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
    let a27 = 3u64;
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
            let a36 = 2u64;
            if nat_lt(a35, a36) {
                let a37 = v1;
                let a38 = 1u64;
                if nat_eq(a37, a38) {
                    Ok(f9({
                        let a39 = v0;
                        let a40 = 8u64;
                        let a41 = 0u64;
                        nat_rem(a39, a40, a41)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
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
            }
        }
    } else {
        let a47 = v1;
        let a48 = 4u64;
        if nat_lt(a47, a48) {
            let a49 = v1;
            let a50 = 3u64;
            if nat_eq(a49, a50) {
                Ok(f11({
                    let a51 = v0;
                    let a52 = 8u64;
                    let a53 = 0u64;
                    nat_rem(a51, a52, a53)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a54 = v1;
            let a55 = 5u64;
            if nat_lt(a54, a55) {
                let a56 = v1;
                let a57 = 4u64;
                if nat_eq(a56, a57) {
                    f12({
                        let a58 = v0;
                        let a59 = 8u64;
                        let a60 = 0u64;
                        nat_rem(a58, a59, a60)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a61 = v1;
                let a62 = 5u64;
                if nat_eq(a61, a62) {
                    Ok(f13({
                        let a63 = v0;
                        let a64 = 8u64;
                        let a65 = 0u64;
                        nat_rem(a63, a64, a65)
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
        let a66 = v0;
        let a67 = 8u64;
        let a68 = 0u64;
        nat_quot(a66, a67, a68)
    };
    let a69 = v1;
    let a70 = 3u64;
    if nat_lt(a69, a70) {
        let a71 = v1;
        let a72 = 1u64;
        if nat_lt(a71, a72) {
            let a73 = v1;
            let a74 = 0u64;
            if nat_eq(a73, a74) {
                f14({
                    let a75 = v0;
                    let a76 = 8u64;
                    let a77 = 0u64;
                    nat_rem(a75, a76, a77)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a78 = v1;
            let a79 = 2u64;
            if nat_lt(a78, a79) {
                let a80 = v1;
                let a81 = 1u64;
                if nat_eq(a80, a81) {
                    f15({
                        let a82 = v0;
                        let a83 = 8u64;
                        let a84 = 0u64;
                        nat_rem(a82, a83, a84)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a85 = v1;
                let a86 = 2u64;
                if nat_eq(a85, a86) {
                    f16({
                        let a87 = v0;
                        let a88 = 8u64;
                        let a89 = 0u64;
                        nat_rem(a87, a88, a89)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a90 = v1;
        let a91 = 5u64;
        if nat_lt(a90, a91) {
            let a92 = v1;
            let a93 = 4u64;
            if nat_lt(a92, a93) {
                let a94 = v1;
                let a95 = 3u64;
                if nat_eq(a94, a95) {
                    f17({
                        let a96 = v0;
                        let a97 = 8u64;
                        let a98 = 0u64;
                        nat_rem(a96, a97, a98)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a99 = v1;
                let a100 = 4u64;
                if nat_eq(a99, a100) {
                    f18({
                        let a101 = v0;
                        let a102 = 8u64;
                        let a103 = 0u64;
                        nat_rem(a101, a102, a103)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a104 = v1;
            let a105 = 6u64;
            if nat_lt(a104, a105) {
                let a106 = v1;
                let a107 = 5u64;
                if nat_eq(a106, a107) {
                    Ok(f19({
                        let a108 = v0;
                        let a109 = 8u64;
                        let a110 = 0u64;
                        nat_rem(a108, a109, a110)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a111 = v1;
                let a112 = 6u64;
                if nat_eq(a111, a112) {
                    Ok(f20({
                        let a113 = v0;
                        let a114 = 8u64;
                        let a115 = 0u64;
                        nat_rem(a113, a114, a115)
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
        let a116 = v0;
        let a117 = 8u64;
        let a118 = 0u64;
        nat_quot(a116, a117, a118)
    };
    let a119 = v1;
    let a120 = 3u64;
    if nat_lt(a119, a120) {
        let a121 = v1;
        let a122 = 1u64;
        if nat_lt(a121, a122) {
            let a123 = v1;
            let a124 = 0u64;
            if nat_eq(a123, a124) {
                f21({
                    let a125 = v0;
                    let a126 = 8u64;
                    let a127 = 0u64;
                    nat_rem(a125, a126, a127)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a128 = v1;
            let a129 = 2u64;
            if nat_lt(a128, a129) {
                let a130 = v1;
                let a131 = 1u64;
                if nat_eq(a130, a131) {
                    Ok(f22({
                        let a132 = v0;
                        let a133 = 8u64;
                        let a134 = 0u64;
                        nat_rem(a132, a133, a134)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a135 = v1;
                let a136 = 2u64;
                if nat_eq(a135, a136) {
                    f23({
                        let a137 = v0;
                        let a138 = 8u64;
                        let a139 = 0u64;
                        nat_rem(a137, a138, a139)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a140 = v1;
        let a141 = 4u64;
        if nat_lt(a140, a141) {
            let a142 = v1;
            let a143 = 3u64;
            if nat_eq(a142, a143) {
                Ok(f11({
                    let a144 = v0;
                    let a145 = 8u64;
                    let a146 = 0u64;
                    nat_rem(a144, a145, a146)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a147 = v1;
            let a148 = 5u64;
            if nat_lt(a147, a148) {
                let a149 = v1;
                let a150 = 4u64;
                if nat_eq(a149, a150) {
                    f24({
                        let a151 = v0;
                        let a152 = 8u64;
                        let a153 = 0u64;
                        nat_rem(a151, a152, a153)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a154 = v1;
                let a155 = 5u64;
                if nat_eq(a154, a155) {
                    Ok(f13({
                        let a156 = v0;
                        let a157 = 8u64;
                        let a158 = 0u64;
                        nat_rem(a156, a157, a158)
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
        let a159 = v0;
        let a160 = 8u64;
        let a161 = 0u64;
        nat_quot(a159, a160, a161)
    };
    let a162 = v1;
    let a163 = 4u64;
    if nat_lt(a162, a163) {
        let a164 = v1;
        let a165 = 2u64;
        if nat_lt(a164, a165) {
            let a166 = v1;
            let a167 = 1u64;
            if nat_lt(a166, a167) {
                let a168 = v1;
                let a169 = 0u64;
                if nat_eq(a168, a169) {
                    f25({
                        let a170 = v0;
                        let a171 = 8u64;
                        let a172 = 0u64;
                        nat_rem(a170, a171, a172)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a173 = v1;
                let a174 = 1u64;
                if nat_eq(a173, a174) {
                    f26({
                        let a175 = v0;
                        let a176 = 8u64;
                        let a177 = 0u64;
                        nat_rem(a175, a176, a177)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a178 = v1;
            let a179 = 3u64;
            if nat_lt(a178, a179) {
                let a180 = v1;
                let a181 = 2u64;
                if nat_eq(a180, a181) {
                    f27({
                        let a182 = v0;
                        let a183 = 8u64;
                        let a184 = 0u64;
                        nat_rem(a182, a183, a184)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a185 = v1;
                let a186 = 3u64;
                if nat_eq(a185, a186) {
                    f28({
                        let a187 = v0;
                        let a188 = 8u64;
                        let a189 = 0u64;
                        nat_rem(a187, a188, a189)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a190 = v1;
        let a191 = 6u64;
        if nat_lt(a190, a191) {
            let a192 = v1;
            let a193 = 5u64;
            if nat_lt(a192, a193) {
                let a194 = v1;
                let a195 = 4u64;
                if nat_eq(a194, a195) {
                    f29({
                        let a196 = v0;
                        let a197 = 8u64;
                        let a198 = 0u64;
                        nat_rem(a196, a197, a198)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a199 = v1;
                let a200 = 5u64;
                if nat_eq(a199, a200) {
                    f30({
                        let a201 = v0;
                        let a202 = 8u64;
                        let a203 = 0u64;
                        nat_rem(a201, a202, a203)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a204 = v1;
            let a205 = 7u64;
            if nat_lt(a204, a205) {
                let a206 = v1;
                let a207 = 6u64;
                if nat_eq(a206, a207) {
                    Ok(f31({
                        let a208 = v0;
                        let a209 = 8u64;
                        let a210 = 0u64;
                        nat_rem(a208, a209, a210)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a211 = v1;
                let a212 = 7u64;
                if nat_eq(a211, a212) {
                    Ok(f32({
                        let a213 = v0;
                        let a214 = 8u64;
                        let a215 = 0u64;
                        nat_rem(a213, a214, a215)
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
        let a216 = v0;
        let a217 = 8u64;
        let a218 = 0u64;
        nat_quot(a216, a217, a218)
    };
    let a219 = v1;
    let a220 = 10u64;
    if nat_lt(a219, a220) {
        let a221 = v1;
        let a222 = 5u64;
        if nat_lt(a221, a222) {
            let a223 = v1;
            let a224 = 2u64;
            if nat_lt(a223, a224) {
                let a225 = v1;
                let a226 = 1u64;
                if nat_lt(a225, a226) {
                    let a227 = v1;
                    let a228 = 0u64;
                    if nat_eq(a227, a228) {
                        f33({
                            let a229 = v0;
                            let a230 = 8u64;
                            let a231 = 0u64;
                            nat_rem(a229, a230, a231)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a232 = v1;
                    let a233 = 1u64;
                    if nat_eq(a232, a233) {
                        f34({
                            let a234 = v0;
                            let a235 = 8u64;
                            let a236 = 0u64;
                            nat_rem(a234, a235, a236)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a237 = v1;
                let a238 = 3u64;
                if nat_lt(a237, a238) {
                    let a239 = v1;
                    let a240 = 2u64;
                    if nat_eq(a239, a240) {
                        Ok(f35({
                            let a241 = v0;
                            let a242 = 8u64;
                            let a243 = 0u64;
                            nat_rem(a241, a242, a243)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a244 = v1;
                    let a245 = 4u64;
                    if nat_lt(a244, a245) {
                        let a246 = v1;
                        let a247 = 3u64;
                        if nat_eq(a246, a247) {
                            f36({
                                let a248 = v0;
                                let a249 = 8u64;
                                let a250 = 0u64;
                                nat_rem(a248, a249, a250)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a251 = v1;
                        let a252 = 4u64;
                        if nat_eq(a251, a252) {
                            f37({
                                let a253 = v0;
                                let a254 = 8u64;
                                let a255 = 0u64;
                                nat_rem(a253, a254, a255)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        } else {
            let a256 = v1;
            let a257 = 7u64;
            if nat_lt(a256, a257) {
                let a258 = v1;
                let a259 = 6u64;
                if nat_lt(a258, a259) {
                    let a260 = v1;
                    let a261 = 5u64;
                    if nat_eq(a260, a261) {
                        Ok(f38({
                            let a262 = v0;
                            let a263 = 8u64;
                            let a264 = 0u64;
                            nat_rem(a262, a263, a264)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a265 = v1;
                    let a266 = 6u64;
                    if nat_eq(a265, a266) {
                        f39({
                            let a267 = v0;
                            let a268 = 8u64;
                            let a269 = 0u64;
                            nat_rem(a267, a268, a269)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a270 = v1;
                let a271 = 8u64;
                if nat_lt(a270, a271) {
                    let a272 = v1;
                    let a273 = 7u64;
                    if nat_eq(a272, a273) {
                        f40({
                            let a274 = v0;
                            let a275 = 8u64;
                            let a276 = 0u64;
                            nat_rem(a274, a275, a276)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a277 = v1;
                    let a278 = 9u64;
                    if nat_lt(a277, a278) {
                        let a279 = v1;
                        let a280 = 8u64;
                        if nat_eq(a279, a280) {
                            f41({
                                let a281 = v0;
                                let a282 = 8u64;
                                let a283 = 0u64;
                                nat_rem(a281, a282, a283)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a284 = v1;
                        let a285 = 9u64;
                        if nat_eq(a284, a285) {
                            f42({
                                let a286 = v0;
                                let a287 = 8u64;
                                let a288 = 0u64;
                                nat_rem(a286, a287, a288)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        }
    } else {
        let a289 = v1;
        let a290 = 15u64;
        if nat_lt(a289, a290) {
            let a291 = v1;
            let a292 = 12u64;
            if nat_lt(a291, a292) {
                let a293 = v1;
                let a294 = 11u64;
                if nat_lt(a293, a294) {
                    let a295 = v1;
                    let a296 = 10u64;
                    if nat_eq(a295, a296) {
                        f43({
                            let a297 = v0;
                            let a298 = 8u64;
                            let a299 = 0u64;
                            nat_rem(a297, a298, a299)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a300 = v1;
                    let a301 = 11u64;
                    if nat_eq(a300, a301) {
                        f44({
                            let a302 = v0;
                            let a303 = 8u64;
                            let a304 = 0u64;
                            nat_rem(a302, a303, a304)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            } else {
                let a305 = v1;
                let a306 = 13u64;
                if nat_lt(a305, a306) {
                    let a307 = v1;
                    let a308 = 12u64;
                    if nat_eq(a307, a308) {
                        f45({
                            let a309 = v0;
                            let a310 = 8u64;
                            let a311 = 0u64;
                            nat_rem(a309, a310, a311)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a312 = v1;
                    let a313 = 14u64;
                    if nat_lt(a312, a313) {
                        let a314 = v1;
                        let a315 = 13u64;
                        if nat_eq(a314, a315) {
                            f46({
                                let a316 = v0;
                                let a317 = 8u64;
                                let a318 = 0u64;
                                nat_rem(a316, a317, a318)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a319 = v1;
                        let a320 = 14u64;
                        if nat_eq(a319, a320) {
                            Ok(f47({
                                let a321 = v0;
                                let a322 = 8u64;
                                let a323 = 0u64;
                                nat_rem(a321, a322, a323)
                            }))
                        } else {
                            Ok(f2())
                        }
                    }
                }
            }
        } else {
            let a324 = v1;
            let a325 = 18u64;
            if nat_lt(a324, a325) {
                let a326 = v1;
                let a327 = 16u64;
                if nat_lt(a326, a327) {
                    let a328 = v1;
                    let a329 = 15u64;
                    if nat_eq(a328, a329) {
                        f48({
                            let a330 = v0;
                            let a331 = 8u64;
                            let a332 = 0u64;
                            nat_rem(a330, a331, a332)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a333 = v1;
                    let a334 = 17u64;
                    if nat_lt(a333, a334) {
                        let a335 = v1;
                        let a336 = 16u64;
                        if nat_eq(a335, a336) {
                            Ok(f49({
                                let a337 = v0;
                                let a338 = 8u64;
                                let a339 = 0u64;
                                nat_rem(a337, a338, a339)
                            }))
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a340 = v1;
                        let a341 = 17u64;
                        if nat_eq(a340, a341) {
                            f50({
                                let a342 = v0;
                                let a343 = 8u64;
                                let a344 = 0u64;
                                nat_rem(a342, a343, a344)
                            })
                        } else {
                            Ok(f2())
                        }
                    }
                }
            } else {
                let a345 = v1;
                let a346 = 19u64;
                if nat_lt(a345, a346) {
                    let a347 = v1;
                    let a348 = 18u64;
                    if nat_eq(a347, a348) {
                        Ok(f51({
                            let a349 = v0;
                            let a350 = 8u64;
                            let a351 = 0u64;
                            nat_rem(a349, a350, a351)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a352 = v1;
                    let a353 = 20u64;
                    if nat_lt(a352, a353) {
                        let a354 = v1;
                        let a355 = 19u64;
                        if nat_eq(a354, a355) {
                            f52({
                                let a356 = v0;
                                let a357 = 8u64;
                                let a358 = 0u64;
                                nat_rem(a356, a357, a358)
                            })
                        } else {
                            Ok(f2())
                        }
                    } else {
                        let a359 = v1;
                        let a360 = 20u64;
                        if nat_eq(a359, a360) {
                            Ok(f53({
                                let a361 = v0;
                                let a362 = 8u64;
                                let a363 = 0u64;
                                nat_rem(a361, a362, a363)
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
        let a364 = v0;
        let a365 = 8u64;
        let a366 = 0u64;
        nat_quot(a364, a365, a366)
    };
    let a367 = v1;
    let a368 = 4u64;
    if nat_lt(a367, a368) {
        let a369 = v1;
        let a370 = 2u64;
        if nat_lt(a369, a370) {
            let a371 = v1;
            let a372 = 1u64;
            if nat_lt(a371, a372) {
                let a373 = v1;
                let a374 = 0u64;
                if nat_eq(a373, a374) {
                    f25({
                        let a375 = v0;
                        let a376 = 8u64;
                        let a377 = 0u64;
                        nat_rem(a375, a376, a377)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a378 = v1;
                let a379 = 1u64;
                if nat_eq(a378, a379) {
                    f26({
                        let a380 = v0;
                        let a381 = 8u64;
                        let a382 = 0u64;
                        nat_rem(a380, a381, a382)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a383 = v1;
            let a384 = 3u64;
            if nat_lt(a383, a384) {
                let a385 = v1;
                let a386 = 2u64;
                if nat_eq(a385, a386) {
                    f27({
                        let a387 = v0;
                        let a388 = 8u64;
                        let a389 = 0u64;
                        nat_rem(a387, a388, a389)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a390 = v1;
                let a391 = 3u64;
                if nat_eq(a390, a391) {
                    f28({
                        let a392 = v0;
                        let a393 = 8u64;
                        let a394 = 0u64;
                        nat_rem(a392, a393, a394)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a395 = v1;
        let a396 = 6u64;
        if nat_lt(a395, a396) {
            let a397 = v1;
            let a398 = 5u64;
            if nat_lt(a397, a398) {
                let a399 = v1;
                let a400 = 4u64;
                if nat_eq(a399, a400) {
                    f29({
                        let a401 = v0;
                        let a402 = 8u64;
                        let a403 = 0u64;
                        nat_rem(a401, a402, a403)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a404 = v1;
                let a405 = 5u64;
                if nat_eq(a404, a405) {
                    f30({
                        let a406 = v0;
                        let a407 = 8u64;
                        let a408 = 0u64;
                        nat_rem(a406, a407, a408)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a409 = v1;
            let a410 = 7u64;
            if nat_lt(a409, a410) {
                let a411 = v1;
                let a412 = 6u64;
                if nat_eq(a411, a412) {
                    Ok(f31({
                        let a413 = v0;
                        let a414 = 8u64;
                        let a415 = 0u64;
                        nat_rem(a413, a414, a415)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a416 = v1;
                let a417 = 7u64;
                if nat_eq(a416, a417) {
                    Ok(f32({
                        let a418 = v0;
                        let a419 = 8u64;
                        let a420 = 0u64;
                        nat_rem(a418, a419, a420)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a421 = v0;
    let a422 = 4u64;
    if nat_lt(a421, a422) {
        let a423 = v0;
        let a424 = 2u64;
        if nat_lt(a423, a424) {
            let a425 = v0;
            let a426 = 1u64;
            if nat_lt(a425, a426) {
                let a427 = v0;
                let a428 = 0u64;
                if nat_eq(a427, a428) {
                    Ok(f54(3u64, f55(), 1097859072u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a429 = v0;
                let a430 = 1u64;
                if nat_eq(a429, a430) {
                    Ok(f57(0u64, f58()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a431 = v0;
            let a432 = 3u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 2u64;
                if nat_eq(a433, a434) {
                    Ok(f57(0u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 3u64;
                if nat_eq(a435, a436) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a437 = v0;
        let a438 = 6u64;
        if nat_lt(a437, a438) {
            let a439 = v0;
            let a440 = 5u64;
            if nat_lt(a439, a440) {
                let a441 = v0;
                let a442 = 4u64;
                if nat_eq(a441, a442) {
                    Ok(f57(0u64, f61(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a443 = v0;
                let a444 = 5u64;
                if nat_eq(a443, a444) {
                    Ok(f57(f62(), f63(f60())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a445 = v0;
            let a446 = 7u64;
            if nat_lt(a445, a446) {
                let a447 = v0;
                let a448 = 6u64;
                if nat_eq(a447, a448) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a449 = v0;
                let a450 = 7u64;
                if nat_eq(a449, a450) {
                    Ok(f65(1u64, f60(), 8u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a451 = v0;
    let a452 = 4u64;
    if nat_lt(a451, a452) {
        let a453 = v0;
        let a454 = 2u64;
        if nat_lt(a453, a454) {
            let a455 = v0;
            let a456 = 1u64;
            if nat_lt(a455, a456) {
                let a457 = v0;
                let a458 = 0u64;
                if nat_eq(a457, a458) {
                    f66(f59(), f67())
                } else {
                    f2()
                }
            } else {
                let a459 = v0;
                let a460 = 1u64;
                if nat_eq(a459, a460) {
                    f66(f68(), f69())
                } else {
                    f2()
                }
            }
        } else {
            let a461 = v0;
            let a462 = 3u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 2u64;
                if nat_eq(a463, a464) {
                    f54(1u64, f67(), f69(), f67())
                } else {
                    f2()
                }
            } else {
                let a465 = v0;
                let a466 = 3u64;
                if nat_eq(a465, a466) {
                    f54(2u64, f67(), 1045220557u64, f67())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a467 = v0;
        let a468 = 6u64;
        if nat_lt(a467, a468) {
            let a469 = v0;
            let a470 = 5u64;
            if nat_lt(a469, a470) {
                let a471 = v0;
                let a472 = 4u64;
                if nat_eq(a471, a472) {
                    f54(0u64, 1097859072u64, f67(), f67())
                } else {
                    f2()
                }
            } else {
                let a473 = v0;
                let a474 = 5u64;
                if nat_eq(a473, a474) {
                    f54(3u64, f67(), f55(), f70())
                } else {
                    f2()
                }
            }
        } else {
            let a475 = v0;
            let a476 = 7u64;
            if nat_lt(a475, a476) {
                let a477 = v0;
                let a478 = 6u64;
                if nat_eq(a477, a478) {
                    f54(3u64, 1065353216u64, f70(), f71())
                } else {
                    f2()
                }
            } else {
                let a479 = v0;
                let a480 = 7u64;
                if nat_eq(a479, a480) {
                    f57(0u64, f60())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a481 = v0;
    let a482 = 4u64;
    if nat_lt(a481, a482) {
        let a483 = v0;
        let a484 = 2u64;
        if nat_lt(a483, a484) {
            let a485 = v0;
            let a486 = 1u64;
            if nat_lt(a485, a486) {
                let a487 = v0;
                let a488 = 0u64;
                if nat_eq(a487, a488) {
                    Ok(f57(1042u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a489 = v0;
                let a490 = 1u64;
                if nat_eq(a489, a490) {
                    Ok(f73(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a491 = v0;
            let a492 = 3u64;
            if nat_lt(a491, a492) {
                let a493 = v0;
                let a494 = 2u64;
                if nat_eq(a493, a494) {
                    Ok(f57(f74(), f75(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a495 = v0;
                let a496 = 3u64;
                if nat_eq(a495, a496) {
                    Ok(f57(f76(), f77(f60())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a497 = v0;
        let a498 = 6u64;
        if nat_lt(a497, a498) {
            let a499 = v0;
            let a500 = 5u64;
            if nat_lt(a499, a500) {
                let a501 = v0;
                let a502 = 4u64;
                if nat_eq(a501, a502) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a503 = v0;
                let a504 = 5u64;
                if nat_eq(a503, a504) {
                    Ok(f65(1u64, f60(), 8u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a505 = v0;
            let a506 = 7u64;
            if nat_lt(a505, a506) {
                let a507 = v0;
                let a508 = 6u64;
                if nat_eq(a507, a508) {
                    Ok(f57(0u64, f78()))
                } else {
                    Ok(f2())
                }
            } else {
                let a509 = v0;
                let a510 = 7u64;
                if nat_eq(a509, a510) {
                    Ok(f57(1049u64, f79()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a511 = v0;
    let a512 = 4u64;
    if nat_lt(a511, a512) {
        let a513 = v0;
        let a514 = 2u64;
        if nat_lt(a513, a514) {
            let a515 = v0;
            let a516 = 1u64;
            if nat_lt(a515, a516) {
                let a517 = v0;
                let a518 = 0u64;
                if nat_eq(a517, a518) {
                    f73(2048u64)
                } else {
                    f2()
                }
            } else {
                let a519 = v0;
                let a520 = 1u64;
                if nat_eq(a519, a520) {
                    f65(2u64, 0u64, f80(), 1051u64)
                } else {
                    f2()
                }
            }
        } else {
            let a521 = v0;
            let a522 = 3u64;
            if nat_lt(a521, a522) {
                let a523 = v0;
                let a524 = 2u64;
                if nat_eq(a523, a524) {
                    f73(1062u64)
                } else {
                    f2()
                }
            } else {
                let a525 = v0;
                let a526 = 3u64;
                if nat_eq(a525, a526) {
                    f54(2u64, f81(), f81(), f69())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a527 = v0;
        let a528 = 6u64;
        if nat_lt(a527, a528) {
            let a529 = v0;
            let a530 = 5u64;
            if nat_lt(a529, a530) {
                let a531 = v0;
                let a532 = 4u64;
                if nat_eq(a531, a532) {
                    f54(2u64, f58(), f80(), f67())
                } else {
                    f2()
                }
            } else {
                let a533 = v0;
                let a534 = 5u64;
                if nat_eq(a533, a534) {
                    f65(2u64, f67(), f69(), 1055u64)
                } else {
                    f2()
                }
            }
        } else {
            let a535 = v0;
            let a536 = 7u64;
            if nat_lt(a535, a536) {
                let a537 = v0;
                let a538 = 6u64;
                if nat_eq(a537, a538) {
                    f73(1062u64)
                } else {
                    f2()
                }
            } else {
                let a539 = v0;
                let a540 = 7u64;
                if nat_eq(a539, a540) {
                    f54(3u64, f81(), f80(), f56())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a541 = v0;
    let a542 = 4u64;
    if nat_lt(a541, a542) {
        let a543 = v0;
        let a544 = 2u64;
        if nat_lt(a543, a544) {
            let a545 = v0;
            let a546 = 1u64;
            if nat_lt(a545, a546) {
                let a547 = v0;
                let a548 = 0u64;
                if nat_eq(a547, a548) {
                    Ok(f54(2u64, f56(), f81(), f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a549 = v0;
                let a550 = 1u64;
                if nat_eq(a549, a550) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a551 = v0;
            let a552 = 3u64;
            if nat_lt(a551, a552) {
                let a553 = v0;
                let a554 = 2u64;
                if nat_eq(a553, a554) {
                    Ok(f57(f75(f60())?, f61(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a555 = v0;
                let a556 = 3u64;
                if nat_eq(a555, a556) {
                    Ok(f57(f77(f60())?, f63(f60())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a557 = v0;
        let a558 = 6u64;
        if nat_lt(a557, a558) {
            let a559 = v0;
            let a560 = 5u64;
            if nat_lt(a559, a560) {
                let a561 = v0;
                let a562 = 4u64;
                if nat_eq(a561, a562) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a563 = v0;
                let a564 = 5u64;
                if nat_eq(a563, a564) {
                    Ok(f65(1u64, f60(), 8u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a565 = v0;
            let a566 = 7u64;
            if nat_lt(a565, a566) {
                let a567 = v0;
                let a568 = 6u64;
                if nat_eq(a567, a568) {
                    Ok(f64(0u64, f59(), 1u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a569 = v0;
                let a570 = 7u64;
                if nat_eq(a569, a570) {
                    Ok(f64(2u64, f68(), 2u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a571 = v0;
    let a572 = 1u64;
    if nat_lt(a571, a572) {
        let a573 = v0;
        let a574 = 0u64;
        if nat_eq(a573, a574) {
            f64(0u64, f69(), 1u64, f69())
        } else {
            f2()
        }
    } else {
        let a575 = v0;
        let a576 = 2u64;
        if nat_lt(a575, a576) {
            let a577 = v0;
            let a578 = 1u64;
            if nat_eq(a577, a578) {
                f65(1u64, f59(), f69(), 1032u64)
            } else {
                f2()
            }
        } else {
            let a579 = v0;
            let a580 = 2u64;
            if nat_eq(a579, a580) {
                f73(2560u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a581 = v0;
    let a582 = 4u64;
    if nat_lt(a581, a582) {
        let a583 = v0;
        let a584 = 2u64;
        if nat_lt(a583, a584) {
            let a585 = v0;
            let a586 = 1u64;
            if nat_lt(a585, a586) {
                let a587 = v0;
                let a588 = 0u64;
                if nat_eq(a587, a588) {
                    Ok(f57(0u64, f82()))
                } else {
                    Ok(f2())
                }
            } else {
                let a589 = v0;
                let a590 = 1u64;
                if nat_eq(a589, a590) {
                    Ok(f57(0u64, f83()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a591 = v0;
            let a592 = 3u64;
            if nat_lt(a591, a592) {
                let a593 = v0;
                let a594 = 2u64;
                if nat_eq(a593, a594) {
                    Ok(f64(2u64, f60(), 4u64, f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a595 = v0;
                let a596 = 3u64;
                if nat_eq(a595, a596) {
                    Ok(f64(0u64, f84(), f83(), f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a597 = v0;
        let a598 = 6u64;
        if nat_lt(a597, a598) {
            let a599 = v0;
            let a600 = 5u64;
            if nat_lt(a599, a600) {
                let a601 = v0;
                let a602 = 4u64;
                if nat_eq(a601, a602) {
                    Ok(f54(2u64, f70(), f86(f85())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a603 = v0;
                let a604 = 5u64;
                if nat_eq(a603, a604) {
                    Ok(f54(1u64, f69(), 1065353216u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a605 = v0;
            let a606 = 7u64;
            if nat_lt(a605, a606) {
                let a607 = v0;
                let a608 = 6u64;
                if nat_eq(a607, a608) {
                    Ok(f54(2u64, 1056964608u64, f69(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a609 = v0;
                let a610 = 7u64;
                if nat_eq(a609, a610) {
                    Ok(f54(0u64, f69(), 1262485504u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a611 = v0;
    let a612 = 4u64;
    if nat_lt(a611, a612) {
        let a613 = v0;
        let a614 = 2u64;
        if nat_lt(a613, a614) {
            let a615 = v0;
            let a616 = 1u64;
            if nat_lt(a615, a616) {
                let a617 = v0;
                let a618 = 0u64;
                if nat_eq(a617, a618) {
                    Ok(f87(2u64, f69(), 7u64, 0u64, f88()))
                } else {
                    Ok(f2())
                }
            } else {
                let a619 = v0;
                let a620 = 1u64;
                if nat_eq(a619, a620) {
                    Ok(f87(3u64, f82(), f83(), f88(), f82()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a621 = v0;
            let a622 = 3u64;
            if nat_lt(a621, a622) {
                let a623 = v0;
                let a624 = 2u64;
                if nat_eq(a623, a624) {
                    Ok(f64(0u64, f83(), 1u64, f83()))
                } else {
                    Ok(f2())
                }
            } else {
                let a625 = v0;
                let a626 = 3u64;
                if nat_eq(a625, a626) {
                    Ok(f65(1u64, f83(), 4u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a627 = v0;
        let a628 = 6u64;
        if nat_lt(a627, a628) {
            let a629 = v0;
            let a630 = 5u64;
            if nat_lt(a629, a630) {
                let a631 = v0;
                let a632 = 4u64;
                if nat_eq(a631, a632) {
                    Ok(f89(4u64, f82(), 0u64, 0u64, f90(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a633 = v0;
                let a634 = 5u64;
                if nat_eq(a633, a634) {
                    Ok(f64(0u64, f90(), 1u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a635 = v0;
            let a636 = 7u64;
            if nat_lt(a635, a636) {
                let a637 = v0;
                let a638 = 6u64;
                if nat_eq(a637, a638) {
                    Ok(f57(f91(f90())?, f92()))
                } else {
                    Ok(f2())
                }
            } else {
                let a639 = v0;
                let a640 = 7u64;
                if nat_eq(a639, a640) {
                    Ok(f64(0u64, f90(), 1u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a641 = v0;
    let a642 = 4u64;
    if nat_lt(a641, a642) {
        let a643 = v0;
        let a644 = 2u64;
        if nat_lt(a643, a644) {
            let a645 = v0;
            let a646 = 1u64;
            if nat_lt(a645, a646) {
                let a647 = v0;
                let a648 = 0u64;
                if nat_eq(a647, a648) {
                    Ok(f57(f91(f90())?, f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a649 = v0;
                let a650 = 1u64;
                if nat_eq(a649, a650) {
                    Ok(f57(0u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a651 = v0;
            let a652 = 3u64;
            if nat_lt(a651, a652) {
                let a653 = v0;
                let a654 = 2u64;
                if nat_eq(a653, a654) {
                    Ok(f57(f91(f90())?, f94()))
                } else {
                    Ok(f2())
                }
            } else {
                let a655 = v0;
                let a656 = 3u64;
                if nat_eq(a655, a656) {
                    Ok(f64(0u64, f94(), 1u64, f94()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a657 = v0;
        let a658 = 6u64;
        if nat_lt(a657, a658) {
            let a659 = v0;
            let a660 = 5u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 4u64;
                if nat_eq(a661, a662) {
                    Ok(f64(0u64, f92(), f94(), f92()))
                } else {
                    Ok(f2())
                }
            } else {
                let a663 = v0;
                let a664 = 5u64;
                if nat_eq(a663, a664) {
                    Ok(f64(0u64, f93(), f94(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a665 = v0;
            let a666 = 7u64;
            if nat_lt(a665, a666) {
                let a667 = v0;
                let a668 = 6u64;
                if nat_eq(a667, a668) {
                    Ok(f57(f91(f92())?, f74()))
                } else {
                    Ok(f2())
                }
            } else {
                let a669 = v0;
                let a670 = 7u64;
                if nat_eq(a669, a670) {
                    Ok(f64(0u64, f92(), 1u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a671 = v0;
    let a672 = 4u64;
    if nat_lt(a671, a672) {
        let a673 = v0;
        let a674 = 2u64;
        if nat_lt(a673, a674) {
            let a675 = v0;
            let a676 = 1u64;
            if nat_lt(a675, a676) {
                let a677 = v0;
                let a678 = 0u64;
                if nat_eq(a677, a678) {
                    Ok(f57(1u64, f76()))
                } else {
                    Ok(f2())
                }
            } else {
                let a679 = v0;
                let a680 = 1u64;
                if nat_eq(a679, a680) {
                    Ok(f65(0u64, f93(), f69(), 1586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a681 = v0;
            let a682 = 3u64;
            if nat_lt(a681, a682) {
                let a683 = v0;
                let a684 = 2u64;
                if nat_eq(a683, a684) {
                    Ok(f57(0u64, f76()))
                } else {
                    Ok(f2())
                }
            } else {
                let a685 = v0;
                let a686 = 3u64;
                if nat_eq(a685, a686) {
                    Ok(f57(2139095039u64, f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a687 = v0;
        let a688 = 6u64;
        if nat_lt(a687, a688) {
            let a689 = v0;
            let a690 = 5u64;
            if nat_lt(a689, a690) {
                let a691 = v0;
                let a692 = 4u64;
                if nat_eq(a691, a692) {
                    Ok(f57(f92(), f96()))
                } else {
                    Ok(f2())
                }
            } else {
                let a693 = v0;
                let a694 = 5u64;
                if nat_eq(a693, a694) {
                    Ok(f57(f91(f96())?, f97()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a695 = v0;
            let a696 = 7u64;
            if nat_lt(a695, a696) {
                let a697 = v0;
                let a698 = 6u64;
                if nat_eq(a697, a698) {
                    Ok(f57(0u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a699 = v0;
                let a700 = 7u64;
                if nat_eq(a699, a700) {
                    Ok(f57(0u64, f83()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a701 = v0;
    let a702 = 4u64;
    if nat_lt(a701, a702) {
        let a703 = v0;
        let a704 = 2u64;
        if nat_lt(a703, a704) {
            let a705 = v0;
            let a706 = 1u64;
            if nat_lt(a705, a706) {
                let a707 = v0;
                let a708 = 0u64;
                if nat_eq(a707, a708) {
                    Ok(f89(4u64, 0u64, f97(), 0u64, f69(), f99()))
                } else {
                    Ok(f2())
                }
            } else {
                let a709 = v0;
                let a710 = 1u64;
                if nat_eq(a709, a710) {
                    Ok(f64(0u64, f99(), f83(), f99()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a711 = v0;
            let a712 = 3u64;
            if nat_lt(a711, a712) {
                let a713 = v0;
                let a714 = 2u64;
                if nat_eq(a713, a714) {
                    Ok(f87(5u64, f100(f99())?, 0u64, 0u64, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a715 = v0;
                let a716 = 3u64;
                if nat_eq(a715, a716) {
                    Ok(f66(f101(), f101()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a717 = v0;
        let a718 = 6u64;
        if nat_lt(a717, a718) {
            let a719 = v0;
            let a720 = 5u64;
            if nat_lt(a719, a720) {
                let a721 = v0;
                let a722 = 4u64;
                if nat_eq(a721, a722) {
                    Ok(f64(0u64, f84(), f83(), f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a723 = v0;
                let a724 = 5u64;
                if nat_eq(a723, a724) {
                    Ok(f54(2u64, f71(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a725 = v0;
            let a726 = 7u64;
            if nat_lt(a725, a726) {
                let a727 = v0;
                let a728 = 6u64;
                if nat_eq(a727, a728) {
                    Ok(f54(1u64, f67(), f86(f85())?, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a729 = v0;
                let a730 = 7u64;
                if nat_eq(a729, a730) {
                    Ok(f54(2u64, f102(f85())?, f67(), f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a731 = v0;
    let a732 = 4u64;
    if nat_lt(a731, a732) {
        let a733 = v0;
        let a734 = 2u64;
        if nat_lt(a733, a734) {
            let a735 = v0;
            let a736 = 1u64;
            if nat_lt(a735, a736) {
                let a737 = v0;
                let a738 = 0u64;
                if nat_eq(a737, a738) {
                    f54(2u64, f69(), f67(), f69())
                } else {
                    f2()
                }
            } else {
                let a739 = v0;
                let a740 = 1u64;
                if nat_eq(a739, a740) {
                    f54(0u64, f98(), f69(), f98())
                } else {
                    f2()
                }
            }
        } else {
            let a741 = v0;
            let a742 = 3u64;
            if nat_lt(a741, a742) {
                let a743 = v0;
                let a744 = 2u64;
                if nat_eq(a743, a744) {
                    f64(0u64, f83(), 1u64, f83())
                } else {
                    f2()
                }
            } else {
                let a745 = v0;
                let a746 = 3u64;
                if nat_eq(a745, a746) {
                    f65(1u64, f83(), 4u64, 1568u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a747 = v0;
        let a748 = 6u64;
        if nat_lt(a747, a748) {
            let a749 = v0;
            let a750 = 5u64;
            if nat_lt(a749, a750) {
                let a751 = v0;
                let a752 = 4u64;
                if nat_eq(a751, a752) {
                    f65(2u64, f98(), f95(), 1582u64)
                } else {
                    f2()
                }
            } else {
                let a753 = v0;
                let a754 = 5u64;
                if nat_eq(a753, a754) {
                    f73(1584u64)
                } else {
                    f2()
                }
            }
        } else {
            let a755 = v0;
            let a756 = 7u64;
            if nat_lt(a755, a756) {
                let a757 = v0;
                let a758 = 6u64;
                if nat_eq(a757, a758) {
                    f57(f98(), f95())
                } else {
                    f2()
                }
            } else {
                let a759 = v0;
                let a760 = 7u64;
                if nat_eq(a759, a760) {
                    f57(f97(), f74())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a761 = v0;
    let a762 = 1u64;
    if nat_lt(a761, a762) {
        let a763 = v0;
        let a764 = 0u64;
        if nat_eq(a763, a764) {
            f64(0u64, f96(), 1u64, f96())
        } else {
            f2()
        }
    } else {
        let a765 = v0;
        let a766 = 2u64;
        if nat_lt(a765, a766) {
            let a767 = v0;
            let a768 = 1u64;
            if nat_eq(a767, a768) {
                f65(1u64, f96(), f93(), 1565u64)
            } else {
                f2()
            }
        } else {
            let a769 = v0;
            let a770 = 2u64;
            if nat_eq(a769, a770) {
                f103(f72())
            } else {
                f2()
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a771 = v0;
    let a772 = 4u64;
    if nat_lt(a771, a772) {
        let a773 = v0;
        let a774 = 2u64;
        if nat_lt(a773, a774) {
            let a775 = v0;
            let a776 = 1u64;
            if nat_lt(a775, a776) {
                let a777 = v0;
                let a778 = 0u64;
                if nat_eq(a777, a778) {
                    Ok(f54(3u64, f55(), 1084227584u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a779 = v0;
                let a780 = 1u64;
                if nat_eq(a779, a780) {
                    Ok(f57(0u64, f58()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a781 = v0;
            let a782 = 3u64;
            if nat_lt(a781, a782) {
                let a783 = v0;
                let a784 = 2u64;
                if nat_eq(a783, a784) {
                    Ok(f57(0u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a785 = v0;
                let a786 = 3u64;
                if nat_eq(a785, a786) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a787 = v0;
        let a788 = 6u64;
        if nat_lt(a787, a788) {
            let a789 = v0;
            let a790 = 5u64;
            if nat_lt(a789, a790) {
                let a791 = v0;
                let a792 = 4u64;
                if nat_eq(a791, a792) {
                    Ok(f57(0u64, f61(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a793 = v0;
                let a794 = 5u64;
                if nat_eq(a793, a794) {
                    Ok(f57(1u64, f63(f60())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a795 = v0;
            let a796 = 7u64;
            if nat_lt(a795, a796) {
                let a797 = v0;
                let a798 = 6u64;
                if nat_eq(a797, a798) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a799 = v0;
                let a800 = 7u64;
                if nat_eq(a799, a800) {
                    Ok(f65(1u64, f60(), 2u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a801 = v0;
    let a802 = 4u64;
    if nat_lt(a801, a802) {
        let a803 = v0;
        let a804 = 2u64;
        if nat_lt(a803, a804) {
            let a805 = v0;
            let a806 = 1u64;
            if nat_lt(a805, a806) {
                let a807 = v0;
                let a808 = 0u64;
                if nat_eq(a807, a808) {
                    f66(f59(), f67())
                } else {
                    f2()
                }
            } else {
                let a809 = v0;
                let a810 = 1u64;
                if nat_eq(a809, a810) {
                    f66(f68(), f69())
                } else {
                    f2()
                }
            }
        } else {
            let a811 = v0;
            let a812 = 3u64;
            if nat_lt(a811, a812) {
                let a813 = v0;
                let a814 = 2u64;
                if nat_eq(a813, a814) {
                    f54(1u64, f67(), f69(), f67())
                } else {
                    f2()
                }
            } else {
                let a815 = v0;
                let a816 = 3u64;
                if nat_eq(a815, a816) {
                    f54(2u64, f67(), 1036831949u64, f67())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a817 = v0;
        let a818 = 6u64;
        if nat_lt(a817, a818) {
            let a819 = v0;
            let a820 = 5u64;
            if nat_lt(a819, a820) {
                let a821 = v0;
                let a822 = 4u64;
                if nat_eq(a821, a822) {
                    f54(0u64, 1084227584u64, f67(), f67())
                } else {
                    f2()
                }
            } else {
                let a823 = v0;
                let a824 = 5u64;
                if nat_eq(a823, a824) {
                    f54(3u64, f67(), f55(), f70())
                } else {
                    f2()
                }
            }
        } else {
            let a825 = v0;
            let a826 = 7u64;
            if nat_lt(a825, a826) {
                let a827 = v0;
                let a828 = 6u64;
                if nat_eq(a827, a828) {
                    f54(3u64, 1065353216u64, f70(), f71())
                } else {
                    f2()
                }
            } else {
                let a829 = v0;
                let a830 = 7u64;
                if nat_eq(a829, a830) {
                    f57(0u64, f60())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a831 = v0;
    let a832 = 4u64;
    if nat_lt(a831, a832) {
        let a833 = v0;
        let a834 = 2u64;
        if nat_lt(a833, a834) {
            let a835 = v0;
            let a836 = 1u64;
            if nat_lt(a835, a836) {
                let a837 = v0;
                let a838 = 0u64;
                if nat_eq(a837, a838) {
                    Ok(f57(1042u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a839 = v0;
                let a840 = 1u64;
                if nat_eq(a839, a840) {
                    Ok(f73(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a841 = v0;
            let a842 = 3u64;
            if nat_lt(a841, a842) {
                let a843 = v0;
                let a844 = 2u64;
                if nat_eq(a843, a844) {
                    Ok(f57(f74(), f75(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a845 = v0;
                let a846 = 3u64;
                if nat_eq(a845, a846) {
                    Ok(f57(f76(), f77(f60())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a847 = v0;
        let a848 = 6u64;
        if nat_lt(a847, a848) {
            let a849 = v0;
            let a850 = 5u64;
            if nat_lt(a849, a850) {
                let a851 = v0;
                let a852 = 4u64;
                if nat_eq(a851, a852) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a853 = v0;
                let a854 = 5u64;
                if nat_eq(a853, a854) {
                    Ok(f65(1u64, f60(), 2u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a855 = v0;
            let a856 = 7u64;
            if nat_lt(a855, a856) {
                let a857 = v0;
                let a858 = 6u64;
                if nat_eq(a857, a858) {
                    Ok(f57(0u64, f78()))
                } else {
                    Ok(f2())
                }
            } else {
                let a859 = v0;
                let a860 = 7u64;
                if nat_eq(a859, a860) {
                    Ok(f57(1049u64, f79()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a861 = v0;
    let a862 = 4u64;
    if nat_lt(a861, a862) {
        let a863 = v0;
        let a864 = 2u64;
        if nat_lt(a863, a864) {
            let a865 = v0;
            let a866 = 1u64;
            if nat_lt(a865, a866) {
                let a867 = v0;
                let a868 = 0u64;
                if nat_eq(a867, a868) {
                    Ok(f54(2u64, f56(), f81(), f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a869 = v0;
                let a870 = 1u64;
                if nat_eq(a869, a870) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a871 = v0;
            let a872 = 3u64;
            if nat_lt(a871, a872) {
                let a873 = v0;
                let a874 = 2u64;
                if nat_eq(a873, a874) {
                    Ok(f57(f75(f60())?, f61(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a875 = v0;
                let a876 = 3u64;
                if nat_eq(a875, a876) {
                    Ok(f57(f77(f60())?, f63(f60())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a877 = v0;
        let a878 = 6u64;
        if nat_lt(a877, a878) {
            let a879 = v0;
            let a880 = 5u64;
            if nat_lt(a879, a880) {
                let a881 = v0;
                let a882 = 4u64;
                if nat_eq(a881, a882) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a883 = v0;
                let a884 = 5u64;
                if nat_eq(a883, a884) {
                    Ok(f65(1u64, f60(), 2u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a885 = v0;
            let a886 = 7u64;
            if nat_lt(a885, a886) {
                let a887 = v0;
                let a888 = 6u64;
                if nat_eq(a887, a888) {
                    Ok(f64(0u64, f59(), 1u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a889 = v0;
                let a890 = 7u64;
                if nat_eq(a889, a890) {
                    Ok(f64(2u64, f68(), 2u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a891 = v0;
    let a892 = 4u64;
    if nat_lt(a891, a892) {
        let a893 = v0;
        let a894 = 2u64;
        if nat_lt(a893, a894) {
            let a895 = v0;
            let a896 = 1u64;
            if nat_lt(a895, a896) {
                let a897 = v0;
                let a898 = 0u64;
                if nat_eq(a897, a898) {
                    Ok(f57(0u64, f82()))
                } else {
                    Ok(f2())
                }
            } else {
                let a899 = v0;
                let a900 = 1u64;
                if nat_eq(a899, a900) {
                    Ok(f57(0u64, f83()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a901 = v0;
            let a902 = 3u64;
            if nat_lt(a901, a902) {
                let a903 = v0;
                let a904 = 2u64;
                if nat_eq(a903, a904) {
                    Ok(f64(2u64, f60(), 8u64, f84()))
                } else {
                    Ok(f2())
                }
            } else {
                let a905 = v0;
                let a906 = 3u64;
                if nat_eq(a905, a906) {
                    Ok(f64(0u64, f84(), f83(), f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a907 = v0;
        let a908 = 6u64;
        if nat_lt(a907, a908) {
            let a909 = v0;
            let a910 = 5u64;
            if nat_lt(a909, a910) {
                let a911 = v0;
                let a912 = 4u64;
                if nat_eq(a911, a912) {
                    Ok(f54(2u64, f70(), f86(f85())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a913 = v0;
                let a914 = 5u64;
                if nat_eq(a913, a914) {
                    Ok(f54(1u64, f69(), 1065353216u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a915 = v0;
            let a916 = 7u64;
            if nat_lt(a915, a916) {
                let a917 = v0;
                let a918 = 6u64;
                if nat_eq(a917, a918) {
                    Ok(f54(2u64, 1056964608u64, f69(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a919 = v0;
                let a920 = 7u64;
                if nat_eq(a919, a920) {
                    Ok(f54(0u64, f69(), 1262485504u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a921 = v0;
    let a922 = 4u64;
    if nat_lt(a921, a922) {
        let a923 = v0;
        let a924 = 2u64;
        if nat_lt(a923, a924) {
            let a925 = v0;
            let a926 = 1u64;
            if nat_lt(a925, a926) {
                let a927 = v0;
                let a928 = 0u64;
                if nat_eq(a927, a928) {
                    Ok(f87(2u64, f69(), 2u64, 0u64, f88()))
                } else {
                    Ok(f2())
                }
            } else {
                let a929 = v0;
                let a930 = 1u64;
                if nat_eq(a929, a930) {
                    Ok(f87(3u64, f82(), f83(), f88(), f82()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a931 = v0;
            let a932 = 3u64;
            if nat_lt(a931, a932) {
                let a933 = v0;
                let a934 = 2u64;
                if nat_eq(a933, a934) {
                    Ok(f64(0u64, f83(), 1u64, f83()))
                } else {
                    Ok(f2())
                }
            } else {
                let a935 = v0;
                let a936 = 3u64;
                if nat_eq(a935, a936) {
                    Ok(f65(1u64, f83(), 8u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a937 = v0;
        let a938 = 6u64;
        if nat_lt(a937, a938) {
            let a939 = v0;
            let a940 = 5u64;
            if nat_lt(a939, a940) {
                let a941 = v0;
                let a942 = 4u64;
                if nat_eq(a941, a942) {
                    Ok(f89(4u64, f82(), 0u64, 0u64, f90(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a943 = v0;
                let a944 = 5u64;
                if nat_eq(a943, a944) {
                    Ok(f64(0u64, f90(), 1u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a945 = v0;
            let a946 = 7u64;
            if nat_lt(a945, a946) {
                let a947 = v0;
                let a948 = 6u64;
                if nat_eq(a947, a948) {
                    Ok(f57(f91(f90())?, f92()))
                } else {
                    Ok(f2())
                }
            } else {
                let a949 = v0;
                let a950 = 7u64;
                if nat_eq(a949, a950) {
                    Ok(f64(0u64, f90(), 1u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a951 = v0;
    let a952 = 4u64;
    if nat_lt(a951, a952) {
        let a953 = v0;
        let a954 = 2u64;
        if nat_lt(a953, a954) {
            let a955 = v0;
            let a956 = 1u64;
            if nat_lt(a955, a956) {
                let a957 = v0;
                let a958 = 0u64;
                if nat_eq(a957, a958) {
                    Ok(f57(f91(f90())?, f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a959 = v0;
                let a960 = 1u64;
                if nat_eq(a959, a960) {
                    Ok(f65(0u64, f62(), 0u64, 1560u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a961 = v0;
            let a962 = 3u64;
            if nat_lt(a961, a962) {
                let a963 = v0;
                let a964 = 2u64;
                if nat_eq(a963, a964) {
                    Ok(f64(0u64, f90(), 6561u64, f90()))
                } else {
                    Ok(f2())
                }
            } else {
                let a965 = v0;
                let a966 = 3u64;
                if nat_eq(a965, a966) {
                    Ok(f64(2u64, f91(f90())?, 65536u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a967 = v0;
        let a968 = 6u64;
        if nat_lt(a967, a968) {
            let a969 = v0;
            let a970 = 5u64;
            if nat_lt(a969, a970) {
                let a971 = v0;
                let a972 = 4u64;
                if nat_eq(a971, a972) {
                    Ok(f64(0u64, f92(), f69(), f92()))
                } else {
                    Ok(f2())
                }
            } else {
                let a973 = v0;
                let a974 = 5u64;
                if nat_eq(a973, a974) {
                    Ok(f64(0u64, f90(), 1u64, f90()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a975 = v0;
            let a976 = 7u64;
            if nat_lt(a975, a976) {
                let a977 = v0;
                let a978 = 6u64;
                if nat_eq(a977, a978) {
                    Ok(f64(2u64, f91(f90())?, 65536u64, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a979 = v0;
                let a980 = 7u64;
                if nat_eq(a979, a980) {
                    Ok(f64(0u64, f93(), f69(), f93()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a981 = v0;
    let a982 = 4u64;
    if nat_lt(a981, a982) {
        let a983 = v0;
        let a984 = 2u64;
        if nat_lt(a983, a984) {
            let a985 = v0;
            let a986 = 1u64;
            if nat_lt(a985, a986) {
                let a987 = v0;
                let a988 = 0u64;
                if nat_eq(a987, a988) {
                    Ok(f57(0u64, f90()))
                } else {
                    Ok(f2())
                }
            } else {
                let a989 = v0;
                let a990 = 1u64;
                if nat_eq(a989, a990) {
                    Ok(f57(f91(f90())?, f94()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a991 = v0;
            let a992 = 3u64;
            if nat_lt(a991, a992) {
                let a993 = v0;
                let a994 = 2u64;
                if nat_eq(a993, a994) {
                    Ok(f64(0u64, f94(), 1u64, f94()))
                } else {
                    Ok(f2())
                }
            } else {
                let a995 = v0;
                let a996 = 3u64;
                if nat_eq(a995, a996) {
                    Ok(f64(0u64, f92(), f94(), f92()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a997 = v0;
        let a998 = 6u64;
        if nat_lt(a997, a998) {
            let a999 = v0;
            let a1000 = 5u64;
            if nat_lt(a999, a1000) {
                let a1001 = v0;
                let a1002 = 4u64;
                if nat_eq(a1001, a1002) {
                    Ok(f64(0u64, f93(), f94(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1003 = v0;
                let a1004 = 5u64;
                if nat_eq(a1003, a1004) {
                    Ok(f57(f91(f92())?, f74()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1005 = v0;
            let a1006 = 7u64;
            if nat_lt(a1005, a1006) {
                let a1007 = v0;
                let a1008 = 6u64;
                if nat_eq(a1007, a1008) {
                    Ok(f64(3u64, f74(), 32768u64, f76()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1009 = v0;
                let a1010 = 7u64;
                if nat_eq(a1009, a1010) {
                    Ok(f64(4u64, f74(), 32768u64, f74()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1011 = v0;
    let a1012 = 4u64;
    if nat_lt(a1011, a1012) {
        let a1013 = v0;
        let a1014 = 2u64;
        if nat_lt(a1013, a1014) {
            let a1015 = v0;
            let a1016 = 1u64;
            if nat_lt(a1015, a1016) {
                let a1017 = v0;
                let a1018 = 0u64;
                if nat_eq(a1017, a1018) {
                    Ok(f65(0u64, f76(), 1u64, 1593u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1019 = v0;
                let a1020 = 1u64;
                if nat_eq(a1019, a1020) {
                    Ok(f57(0u64, f76()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1021 = v0;
            let a1022 = 3u64;
            if nat_lt(a1021, a1022) {
                let a1023 = v0;
                let a1024 = 2u64;
                if nat_eq(a1023, a1024) {
                    Ok(f57(2139095039u64, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1025 = v0;
                let a1026 = 3u64;
                if nat_eq(a1025, a1026) {
                    Ok(f57(f92(), f96()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1027 = v0;
        let a1028 = 6u64;
        if nat_lt(a1027, a1028) {
            let a1029 = v0;
            let a1030 = 5u64;
            if nat_lt(a1029, a1030) {
                let a1031 = v0;
                let a1032 = 4u64;
                if nat_eq(a1031, a1032) {
                    Ok(f57(f91(f96())?, f97()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1033 = v0;
                let a1034 = 5u64;
                if nat_eq(a1033, a1034) {
                    Ok(f57(0u64, f98()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1035 = v0;
            let a1036 = 7u64;
            if nat_lt(a1035, a1036) {
                let a1037 = v0;
                let a1038 = 6u64;
                if nat_eq(a1037, a1038) {
                    Ok(f57(0u64, f83()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1039 = v0;
                let a1040 = 7u64;
                if nat_eq(a1039, a1040) {
                    Ok(f89(4u64, 0u64, f97(), 0u64, f69(), f99()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1041 = v0;
    let a1042 = 4u64;
    if nat_lt(a1041, a1042) {
        let a1043 = v0;
        let a1044 = 2u64;
        if nat_lt(a1043, a1044) {
            let a1045 = v0;
            let a1046 = 1u64;
            if nat_lt(a1045, a1046) {
                let a1047 = v0;
                let a1048 = 0u64;
                if nat_eq(a1047, a1048) {
                    Ok(f64(0u64, f99(), f83(), f99()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1049 = v0;
                let a1050 = 1u64;
                if nat_eq(a1049, a1050) {
                    Ok(f87(5u64, f100(f99())?, 0u64, 0u64, f101()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1051 = v0;
            let a1052 = 3u64;
            if nat_lt(a1051, a1052) {
                let a1053 = v0;
                let a1054 = 2u64;
                if nat_eq(a1053, a1054) {
                    Ok(f66(f101(), f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1055 = v0;
                let a1056 = 3u64;
                if nat_eq(a1055, a1056) {
                    Ok(f64(0u64, f84(), f83(), f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1057 = v0;
        let a1058 = 6u64;
        if nat_lt(a1057, a1058) {
            let a1059 = v0;
            let a1060 = 5u64;
            if nat_lt(a1059, a1060) {
                let a1061 = v0;
                let a1062 = 4u64;
                if nat_eq(a1061, a1062) {
                    Ok(f54(2u64, f71(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1063 = v0;
                let a1064 = 5u64;
                if nat_eq(a1063, a1064) {
                    Ok(f54(1u64, f67(), f86(f85())?, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1065 = v0;
            let a1066 = 7u64;
            if nat_lt(a1065, a1066) {
                let a1067 = v0;
                let a1068 = 6u64;
                if nat_eq(a1067, a1068) {
                    Ok(f54(2u64, f102(f85())?, f67(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1069 = v0;
                let a1070 = 7u64;
                if nat_eq(a1069, a1070) {
                    Ok(f54(2u64, f69(), f67(), f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1071 = v0;
    let a1072 = 4u64;
    if nat_lt(a1071, a1072) {
        let a1073 = v0;
        let a1074 = 2u64;
        if nat_lt(a1073, a1074) {
            let a1075 = v0;
            let a1076 = 1u64;
            if nat_lt(a1075, a1076) {
                let a1077 = v0;
                let a1078 = 0u64;
                if nat_eq(a1077, a1078) {
                    f54(0u64, f98(), f69(), f98())
                } else {
                    f2()
                }
            } else {
                let a1079 = v0;
                let a1080 = 1u64;
                if nat_eq(a1079, a1080) {
                    f64(0u64, f83(), 1u64, f83())
                } else {
                    f2()
                }
            }
        } else {
            let a1081 = v0;
            let a1082 = 3u64;
            if nat_lt(a1081, a1082) {
                let a1083 = v0;
                let a1084 = 2u64;
                if nat_eq(a1083, a1084) {
                    f65(1u64, f83(), 8u64, 1575u64)
                } else {
                    f2()
                }
            } else {
                let a1085 = v0;
                let a1086 = 3u64;
                if nat_eq(a1085, a1086) {
                    f65(2u64, f98(), f95(), 1589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1087 = v0;
        let a1088 = 6u64;
        if nat_lt(a1087, a1088) {
            let a1089 = v0;
            let a1090 = 5u64;
            if nat_lt(a1089, a1090) {
                let a1091 = v0;
                let a1092 = 4u64;
                if nat_eq(a1091, a1092) {
                    f73(1591u64)
                } else {
                    f2()
                }
            } else {
                let a1093 = v0;
                let a1094 = 5u64;
                if nat_eq(a1093, a1094) {
                    f57(f98(), f95())
                } else {
                    f2()
                }
            }
        } else {
            let a1095 = v0;
            let a1096 = 7u64;
            if nat_lt(a1095, a1096) {
                let a1097 = v0;
                let a1098 = 6u64;
                if nat_eq(a1097, a1098) {
                    f57(f97(), f74())
                } else {
                    f2()
                }
            } else {
                let a1099 = v0;
                let a1100 = 7u64;
                if nat_eq(a1099, a1100) {
                    f64(0u64, f96(), 1u64, f96())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f32(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1101 = v0;
    let a1102 = 1u64;
    if nat_lt(a1101, a1102) {
        let a1103 = v0;
        let a1104 = 0u64;
        if nat_eq(a1103, a1104) {
            f65(1u64, f96(), f93(), 1572u64)
        } else {
            f2()
        }
    } else {
        let a1105 = v0;
        let a1106 = 1u64;
        if nat_eq(a1105, a1106) {
            f103(f72())
        } else {
            f2()
        }
    }
}

pub fn f33(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1107 = v0;
    let a1108 = 4u64;
    if nat_lt(a1107, a1108) {
        let a1109 = v0;
        let a1110 = 2u64;
        if nat_lt(a1109, a1110) {
            let a1111 = v0;
            let a1112 = 1u64;
            if nat_lt(a1111, a1112) {
                let a1113 = v0;
                let a1114 = 0u64;
                if nat_eq(a1113, a1114) {
                    Ok(f54(3u64, 1082130432u64, f55(), f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1115 = v0;
                let a1116 = 1u64;
                if nat_eq(a1115, a1116) {
                    Ok(f54(3u64, 1065353216u64, f70(), f71()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1117 = v0;
            let a1118 = 3u64;
            if nat_lt(a1117, a1118) {
                let a1119 = v0;
                let a1120 = 2u64;
                if nat_eq(a1119, a1120) {
                    Ok(f57(0u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1121 = v0;
                let a1122 = 3u64;
                if nat_eq(a1121, a1122) {
                    Ok(f57(0u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1123 = v0;
        let a1124 = 6u64;
        if nat_lt(a1123, a1124) {
            let a1125 = v0;
            let a1126 = 5u64;
            if nat_lt(a1125, a1126) {
                let a1127 = v0;
                let a1128 = 4u64;
                if nat_eq(a1127, a1128) {
                    Ok(f54(2u64, f70(), f86(f85())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1129 = v0;
                let a1130 = 5u64;
                if nat_eq(a1129, a1130) {
                    Ok(f54(0u64, f69(), 1262485504u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1131 = v0;
            let a1132 = 7u64;
            if nat_lt(a1131, a1132) {
                let a1133 = v0;
                let a1134 = 6u64;
                if nat_eq(a1133, a1134) {
                    Ok(f54(1u64, f104(), 1262485504u64, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1135 = v0;
                let a1136 = 7u64;
                if nat_eq(a1135, a1136) {
                    Ok(f64(4u64, f104(), 256u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1137 = v0;
    let a1138 = 4u64;
    if nat_lt(a1137, a1138) {
        let a1139 = v0;
        let a1140 = 2u64;
        if nat_lt(a1139, a1140) {
            let a1141 = v0;
            let a1142 = 1u64;
            if nat_lt(a1141, a1142) {
                let a1143 = v0;
                let a1144 = 0u64;
                if nat_eq(a1143, a1144) {
                    Ok(f66(f69(), f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1145 = v0;
                let a1146 = 1u64;
                if nat_eq(a1145, a1146) {
                    Ok(f54(2u64, f71(), f101(), f69()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1147 = v0;
            let a1148 = 3u64;
            if nat_lt(a1147, a1148) {
                let a1149 = v0;
                let a1150 = 2u64;
                if nat_eq(a1149, a1150) {
                    Ok(f54(1u64, f86(f85())?, f69(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1151 = v0;
                let a1152 = 3u64;
                if nat_eq(a1151, a1152) {
                    Ok(f54(2u64, f105(f85())?, f69(), f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1153 = v0;
        let a1154 = 6u64;
        if nat_lt(a1153, a1154) {
            let a1155 = v0;
            let a1156 = 5u64;
            if nat_lt(a1155, a1156) {
                let a1157 = v0;
                let a1158 = 4u64;
                if nat_eq(a1157, a1158) {
                    Ok(f54(2u64, f67(), f69(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1159 = v0;
                let a1160 = 5u64;
                if nat_eq(a1159, a1160) {
                    Ok(f54(0u64, f98(), f67(), f98()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1161 = v0;
            let a1162 = 7u64;
            if nat_lt(a1161, a1162) {
                let a1163 = v0;
                let a1164 = 6u64;
                if nat_eq(a1163, a1164) {
                    Ok(f64(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1165 = v0;
                let a1166 = 7u64;
                if nat_eq(a1165, a1166) {
                    Ok(f65(1u64, f85(), 32u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f35(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1167 = v0;
    let a1168 = 4u64;
    if nat_lt(a1167, a1168) {
        let a1169 = v0;
        let a1170 = 2u64;
        if nat_lt(a1169, a1170) {
            let a1171 = v0;
            let a1172 = 1u64;
            if nat_lt(a1171, a1172) {
                let a1173 = v0;
                let a1174 = 0u64;
                if nat_eq(a1173, a1174) {
                    f57(f98(), f95())
                } else {
                    f2()
                }
            } else {
                let a1175 = v0;
                let a1176 = 1u64;
                if nat_eq(a1175, a1176) {
                    f57(f70(), f106())
                } else {
                    f2()
                }
            }
        } else {
            let a1177 = v0;
            let a1178 = 3u64;
            if nat_lt(a1177, a1178) {
                let a1179 = v0;
                let a1180 = 2u64;
                if nat_eq(a1179, a1180) {
                    f57(0u64, f59())
                } else {
                    f2()
                }
            } else {
                let a1181 = v0;
                let a1182 = 3u64;
                if nat_eq(a1181, a1182) {
                    f65(0u64, f59(), 4u64, 1070u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1183 = v0;
        let a1184 = 6u64;
        if nat_lt(a1183, a1184) {
            let a1185 = v0;
            let a1186 = 5u64;
            if nat_lt(a1185, a1186) {
                let a1187 = v0;
                let a1188 = 4u64;
                if nat_eq(a1187, a1188) {
                    f66(f59(), f67())
                } else {
                    f2()
                }
            } else {
                let a1189 = v0;
                let a1190 = 5u64;
                if nat_eq(a1189, a1190) {
                    f54(1u64, f67(), 1082130432u64, f67())
                } else {
                    f2()
                }
            }
        } else {
            let a1191 = v0;
            let a1192 = 7u64;
            if nat_lt(a1191, a1192) {
                let a1193 = v0;
                let a1194 = 6u64;
                if nat_eq(a1193, a1194) {
                    f54(2u64, f67(), 1036831949u64, f67())
                } else {
                    f2()
                }
            } else {
                let a1195 = v0;
                let a1196 = 7u64;
                if nat_eq(a1195, a1196) {
                    f54(0u64, f67(), 1082130432u64, f67())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f36(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1197 = v0;
    let a1198 = 4u64;
    if nat_lt(a1197, a1198) {
        let a1199 = v0;
        let a1200 = 2u64;
        if nat_lt(a1199, a1200) {
            let a1201 = v0;
            let a1202 = 1u64;
            if nat_lt(a1201, a1202) {
                let a1203 = v0;
                let a1204 = 0u64;
                if nat_eq(a1203, a1204) {
                    Ok(f54(3u64, f67(), f55(), f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1205 = v0;
                let a1206 = 1u64;
                if nat_eq(a1205, a1206) {
                    Ok(f54(3u64, 1065353216u64, f70(), f71()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1207 = v0;
            let a1208 = 3u64;
            if nat_lt(a1207, a1208) {
                let a1209 = v0;
                let a1210 = 2u64;
                if nat_eq(a1209, a1210) {
                    Ok(f57(0u64, f98()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1211 = v0;
                let a1212 = 3u64;
                if nat_eq(a1211, a1212) {
                    Ok(f57(0u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1213 = v0;
        let a1214 = 6u64;
        if nat_lt(a1213, a1214) {
            let a1215 = v0;
            let a1216 = 5u64;
            if nat_lt(a1215, a1216) {
                let a1217 = v0;
                let a1218 = 4u64;
                if nat_eq(a1217, a1218) {
                    Ok(f54(2u64, f70(), f86(f85())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1219 = v0;
                let a1220 = 5u64;
                if nat_eq(a1219, a1220) {
                    Ok(f54(0u64, f69(), 1262485504u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1221 = v0;
            let a1222 = 7u64;
            if nat_lt(a1221, a1222) {
                let a1223 = v0;
                let a1224 = 6u64;
                if nat_eq(a1223, a1224) {
                    Ok(f54(1u64, f104(), 1262485504u64, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1225 = v0;
                let a1226 = 7u64;
                if nat_eq(a1225, a1226) {
                    Ok(f65(2u64, 1082130432u64, f101(), 1057u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f37(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1227 = v0;
    let a1228 = 4u64;
    if nat_lt(a1227, a1228) {
        let a1229 = v0;
        let a1230 = 2u64;
        if nat_lt(a1229, a1230) {
            let a1231 = v0;
            let a1232 = 1u64;
            if nat_lt(a1231, a1232) {
                let a1233 = v0;
                let a1234 = 0u64;
                if nat_eq(a1233, a1234) {
                    Ok(f73(1059u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1235 = v0;
                let a1236 = 1u64;
                if nat_eq(a1235, a1236) {
                    Ok(f57(1082130432u64, f101()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1237 = v0;
            let a1238 = 3u64;
            if nat_lt(a1237, a1238) {
                let a1239 = v0;
                let a1240 = 2u64;
                if nat_eq(a1239, a1240) {
                    Ok(f57(4u64, f104()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1241 = v0;
                let a1242 = 3u64;
                if nat_eq(a1241, a1242) {
                    Ok(f54(2u64, f71(), f101(), f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1243 = v0;
        let a1244 = 6u64;
        if nat_lt(a1243, a1244) {
            let a1245 = v0;
            let a1246 = 5u64;
            if nat_lt(a1245, a1246) {
                let a1247 = v0;
                let a1248 = 4u64;
                if nat_eq(a1247, a1248) {
                    Ok(f54(1u64, f86(f85())?, f69(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1249 = v0;
                let a1250 = 5u64;
                if nat_eq(a1249, a1250) {
                    Ok(f54(2u64, f105(f85())?, f69(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1251 = v0;
            let a1252 = 7u64;
            if nat_lt(a1251, a1252) {
                let a1253 = v0;
                let a1254 = 6u64;
                if nat_eq(a1253, a1254) {
                    Ok(f54(2u64, f67(), f69(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1255 = v0;
                let a1256 = 7u64;
                if nat_eq(a1255, a1256) {
                    Ok(f54(0u64, f98(), f67(), f98()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f38(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1257 = v0;
    let a1258 = 4u64;
    if nat_lt(a1257, a1258) {
        let a1259 = v0;
        let a1260 = 2u64;
        if nat_lt(a1259, a1260) {
            let a1261 = v0;
            let a1262 = 1u64;
            if nat_lt(a1261, a1262) {
                let a1263 = v0;
                let a1264 = 0u64;
                if nat_eq(a1263, a1264) {
                    f64(0u64, f85(), 1u64, f85())
                } else {
                    f2()
                }
            } else {
                let a1265 = v0;
                let a1266 = 1u64;
                if nat_eq(a1265, a1266) {
                    f65(1u64, f85(), 32u64, 1052u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1267 = v0;
            let a1268 = 3u64;
            if nat_lt(a1267, a1268) {
                let a1269 = v0;
                let a1270 = 2u64;
                if nat_eq(a1269, a1270) {
                    f65(2u64, f98(), f95(), 1068u64)
                } else {
                    f2()
                }
            } else {
                let a1271 = v0;
                let a1272 = 3u64;
                if nat_eq(a1271, a1272) {
                    f73(1070u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1273 = v0;
        let a1274 = 6u64;
        if nat_lt(a1273, a1274) {
            let a1275 = v0;
            let a1276 = 5u64;
            if nat_lt(a1275, a1276) {
                let a1277 = v0;
                let a1278 = 4u64;
                if nat_eq(a1277, a1278) {
                    f57(f98(), f95())
                } else {
                    f2()
                }
            } else {
                let a1279 = v0;
                let a1280 = 5u64;
                if nat_eq(a1279, a1280) {
                    f57(f70(), f106())
                } else {
                    f2()
                }
            }
        } else {
            let a1281 = v0;
            let a1282 = 7u64;
            if nat_lt(a1281, a1282) {
                let a1283 = v0;
                let a1284 = 6u64;
                if nat_eq(a1283, a1284) {
                    f64(0u64, f59(), 1u64, f59())
                } else {
                    f2()
                }
            } else {
                let a1285 = v0;
                let a1286 = 7u64;
                if nat_eq(a1285, a1286) {
                    f65(1u64, f59(), 9u64, 1043u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f39(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1287 = v0;
    let a1288 = 4u64;
    if nat_lt(a1287, a1288) {
        let a1289 = v0;
        let a1290 = 2u64;
        if nat_lt(a1289, a1290) {
            let a1291 = v0;
            let a1292 = 1u64;
            if nat_lt(a1291, a1292) {
                let a1293 = v0;
                let a1294 = 0u64;
                if nat_eq(a1293, a1294) {
                    Ok(f57(0u64, f81()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1295 = v0;
                let a1296 = 1u64;
                if nat_eq(a1295, a1296) {
                    Ok(f57(0u64, f80()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1297 = v0;
            let a1298 = 3u64;
            if nat_lt(a1297, a1298) {
                let a1299 = v0;
                let a1300 = 2u64;
                if nat_eq(a1299, a1300) {
                    Ok(f57(0u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1301 = v0;
                let a1302 = 3u64;
                if nat_eq(a1301, a1302) {
                    Ok(f54(2u64, f106(), f86(f85())?, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1303 = v0;
        let a1304 = 6u64;
        if nat_lt(a1303, a1304) {
            let a1305 = v0;
            let a1306 = 5u64;
            if nat_lt(a1305, a1306) {
                let a1307 = v0;
                let a1308 = 4u64;
                if nat_eq(a1307, a1308) {
                    Ok(f54(0u64, f69(), 1262485504u64, f104()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1309 = v0;
                let a1310 = 5u64;
                if nat_eq(a1309, a1310) {
                    Ok(f54(1u64, f104(), 1262485504u64, f101()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1311 = v0;
            let a1312 = 7u64;
            if nat_lt(a1311, a1312) {
                let a1313 = v0;
                let a1314 = 6u64;
                if nat_eq(a1313, a1314) {
                    Ok(f65(2u64, 1082130432u64, f101(), 1080u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1315 = v0;
                let a1316 = 7u64;
                if nat_eq(a1315, a1316) {
                    Ok(f73(1082u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f40(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1317 = v0;
    let a1318 = 4u64;
    if nat_lt(a1317, a1318) {
        let a1319 = v0;
        let a1320 = 2u64;
        if nat_lt(a1319, a1320) {
            let a1321 = v0;
            let a1322 = 1u64;
            if nat_lt(a1321, a1322) {
                let a1323 = v0;
                let a1324 = 0u64;
                if nat_eq(a1323, a1324) {
                    Ok(f57(1082130432u64, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1325 = v0;
                let a1326 = 1u64;
                if nat_eq(a1325, a1326) {
                    Ok(f57(4u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1327 = v0;
            let a1328 = 3u64;
            if nat_lt(a1327, a1328) {
                let a1329 = v0;
                let a1330 = 2u64;
                if nat_eq(a1329, a1330) {
                    Ok(f64(4u64, f104(), 256u64, f107(f85())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1331 = v0;
                let a1332 = 3u64;
                if nat_eq(a1331, a1332) {
                    Ok(f54(2u64, f105(f85())?, f86(f85())?, f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1333 = v0;
        let a1334 = 6u64;
        if nat_lt(a1333, a1334) {
            let a1335 = v0;
            let a1336 = 5u64;
            if nat_lt(a1335, a1336) {
                let a1337 = v0;
                let a1338 = 4u64;
                if nat_eq(a1337, a1338) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1339 = v0;
                let a1340 = 5u64;
                if nat_eq(a1339, a1340) {
                    Ok(f54(0u64, f81(), f67(), f81()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1341 = v0;
            let a1342 = 7u64;
            if nat_lt(a1341, a1342) {
                let a1343 = v0;
                let a1344 = 6u64;
                if nat_eq(a1343, a1344) {
                    Ok(f54(2u64, f105(f85())?, f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1345 = v0;
                let a1346 = 7u64;
                if nat_eq(a1345, a1346) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f41(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1347 = v0;
    let a1348 = 4u64;
    if nat_lt(a1347, a1348) {
        let a1349 = v0;
        let a1350 = 2u64;
        if nat_lt(a1349, a1350) {
            let a1351 = v0;
            let a1352 = 1u64;
            if nat_lt(a1351, a1352) {
                let a1353 = v0;
                let a1354 = 0u64;
                if nat_eq(a1353, a1354) {
                    Ok(f54(0u64, f80(), f67(), f80()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1355 = v0;
                let a1356 = 1u64;
                if nat_eq(a1355, a1356) {
                    Ok(f64(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1357 = v0;
            let a1358 = 3u64;
            if nat_lt(a1357, a1358) {
                let a1359 = v0;
                let a1360 = 2u64;
                if nat_eq(a1359, a1360) {
                    Ok(f65(1u64, f85(), 32u64, 1075u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1361 = v0;
                let a1362 = 3u64;
                if nat_eq(a1361, a1362) {
                    Ok(f57(0u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1363 = v0;
        let a1364 = 6u64;
        if nat_lt(a1363, a1364) {
            let a1365 = v0;
            let a1366 = 5u64;
            if nat_lt(a1365, a1366) {
                let a1367 = v0;
                let a1368 = 4u64;
                if nat_eq(a1367, a1368) {
                    Ok(f57(0u64, f108()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1369 = v0;
                let a1370 = 5u64;
                if nat_eq(a1369, a1370) {
                    Ok(f57(0u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1371 = v0;
            let a1372 = 7u64;
            if nat_lt(a1371, a1372) {
                let a1373 = v0;
                let a1374 = 6u64;
                if nat_eq(a1373, a1374) {
                    Ok(f66(f107(f85())?, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1375 = v0;
                let a1376 = 7u64;
                if nat_eq(a1375, a1376) {
                    Ok(f57(f101(), f109()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f42(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1377 = v0;
    let a1378 = 4u64;
    if nat_lt(a1377, a1378) {
        let a1379 = v0;
        let a1380 = 2u64;
        if nat_lt(a1379, a1380) {
            let a1381 = v0;
            let a1382 = 1u64;
            if nat_lt(a1381, a1382) {
                let a1383 = v0;
                let a1384 = 0u64;
                if nat_eq(a1383, a1384) {
                    Ok(f57(f81(), f110()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1385 = v0;
                let a1386 = 1u64;
                if nat_eq(a1385, a1386) {
                    Ok(f57(f80(), f111()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1387 = v0;
            let a1388 = 3u64;
            if nat_lt(a1387, a1388) {
                let a1389 = v0;
                let a1390 = 2u64;
                if nat_eq(a1389, a1390) {
                    Ok(f54(2u64, f105(f85())?, f86(f85())?, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1391 = v0;
                let a1392 = 3u64;
                if nat_eq(a1391, a1392) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1393 = v0;
        let a1394 = 6u64;
        if nat_lt(a1393, a1394) {
            let a1395 = v0;
            let a1396 = 5u64;
            if nat_lt(a1395, a1396) {
                let a1397 = v0;
                let a1398 = 4u64;
                if nat_eq(a1397, a1398) {
                    Ok(f54(1u64, f110(), f67(), f110()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1399 = v0;
                let a1400 = 5u64;
                if nat_eq(a1399, a1400) {
                    Ok(f54(2u64, f105(f85())?, f101(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1401 = v0;
            let a1402 = 7u64;
            if nat_lt(a1401, a1402) {
                let a1403 = v0;
                let a1404 = 6u64;
                if nat_eq(a1403, a1404) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1405 = v0;
                let a1406 = 7u64;
                if nat_eq(a1405, a1406) {
                    Ok(f54(1u64, f111(), f67(), f111()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f43(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1407 = v0;
    let a1408 = 4u64;
    if nat_lt(a1407, a1408) {
        let a1409 = v0;
        let a1410 = 2u64;
        if nat_lt(a1409, a1410) {
            let a1411 = v0;
            let a1412 = 1u64;
            if nat_lt(a1411, a1412) {
                let a1413 = v0;
                let a1414 = 0u64;
                if nat_eq(a1413, a1414) {
                    Ok(f65(2u64, 0u64, f110(), 1106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1415 = v0;
                let a1416 = 1u64;
                if nat_eq(a1415, a1416) {
                    Ok(f73(1133u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1417 = v0;
            let a1418 = 3u64;
            if nat_lt(a1417, a1418) {
                let a1419 = v0;
                let a1420 = 2u64;
                if nat_eq(a1419, a1420) {
                    Ok(f65(2u64, 0u64, f111(), 1108u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1421 = v0;
                let a1422 = 3u64;
                if nat_eq(a1421, a1422) {
                    Ok(f73(1133u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1423 = v0;
        let a1424 = 6u64;
        if nat_lt(a1423, a1424) {
            let a1425 = v0;
            let a1426 = 5u64;
            if nat_lt(a1425, a1426) {
                let a1427 = v0;
                let a1428 = 4u64;
                if nat_eq(a1427, a1428) {
                    Ok(f54(2u64, f86(f85())?, f111(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1429 = v0;
                let a1430 = 5u64;
                if nat_eq(a1429, a1430) {
                    Ok(f54(3u64, f69(), f110(), f69()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1431 = v0;
            let a1432 = 7u64;
            if nat_lt(a1431, a1432) {
                let a1433 = v0;
                let a1434 = 6u64;
                if nat_eq(a1433, a1434) {
                    Ok(f54(0u64, f69(), 1262485504u64, f104()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1435 = v0;
                let a1436 = 7u64;
                if nat_eq(a1435, a1436) {
                    Ok(f54(1u64, f104(), 1262485504u64, f101()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f44(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1437 = v0;
    let a1438 = 4u64;
    if nat_lt(a1437, a1438) {
        let a1439 = v0;
        let a1440 = 2u64;
        if nat_lt(a1439, a1440) {
            let a1441 = v0;
            let a1442 = 1u64;
            if nat_lt(a1441, a1442) {
                let a1443 = v0;
                let a1444 = 0u64;
                if nat_eq(a1443, a1444) {
                    Ok(f65(2u64, 1082130432u64, f101(), 1114u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1445 = v0;
                let a1446 = 1u64;
                if nat_eq(a1445, a1446) {
                    Ok(f73(1116u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1447 = v0;
            let a1448 = 3u64;
            if nat_lt(a1447, a1448) {
                let a1449 = v0;
                let a1450 = 2u64;
                if nat_eq(a1449, a1450) {
                    Ok(f57(1082130432u64, f101()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1451 = v0;
                let a1452 = 3u64;
                if nat_eq(a1451, a1452) {
                    Ok(f57(4u64, f104()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1453 = v0;
        let a1454 = 6u64;
        if nat_lt(a1453, a1454) {
            let a1455 = v0;
            let a1456 = 5u64;
            if nat_lt(a1455, a1456) {
                let a1457 = v0;
                let a1458 = 4u64;
                if nat_eq(a1457, a1458) {
                    Ok(f65(0u64, f101(), f109(), 1133u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1459 = v0;
                let a1460 = 5u64;
                if nat_eq(a1459, a1460) {
                    Ok(f54(2u64, f105(f85())?, f86(f85())?, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1461 = v0;
            let a1462 = 7u64;
            if nat_lt(a1461, a1462) {
                let a1463 = v0;
                let a1464 = 6u64;
                if nat_eq(a1463, a1464) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1465 = v0;
                let a1466 = 7u64;
                if nat_eq(a1465, a1466) {
                    Ok(f54(0u64, f110(), f67(), f110()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f45(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1467 = v0;
    let a1468 = 4u64;
    if nat_lt(a1467, a1468) {
        let a1469 = v0;
        let a1470 = 2u64;
        if nat_lt(a1469, a1470) {
            let a1471 = v0;
            let a1472 = 1u64;
            if nat_lt(a1471, a1472) {
                let a1473 = v0;
                let a1474 = 0u64;
                if nat_eq(a1473, a1474) {
                    Ok(f54(2u64, f105(f85())?, f101(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1475 = v0;
                let a1476 = 1u64;
                if nat_eq(a1475, a1476) {
                    Ok(f54(2u64, f67(), f101(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1477 = v0;
            let a1478 = 3u64;
            if nat_lt(a1477, a1478) {
                let a1479 = v0;
                let a1480 = 2u64;
                if nat_eq(a1479, a1480) {
                    Ok(f54(0u64, f111(), f67(), f111()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1481 = v0;
                let a1482 = 3u64;
                if nat_eq(a1481, a1482) {
                    Ok(f54(2u64, f110(), f110(), f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1483 = v0;
        let a1484 = 6u64;
        if nat_lt(a1483, a1484) {
            let a1485 = v0;
            let a1486 = 5u64;
            if nat_lt(a1485, a1486) {
                let a1487 = v0;
                let a1488 = 4u64;
                if nat_eq(a1487, a1488) {
                    Ok(f54(2u64, f67(), f80(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1489 = v0;
                let a1490 = 5u64;
                if nat_eq(a1489, a1490) {
                    Ok(f54(2u64, f81(), f81(), f69()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1491 = v0;
            let a1492 = 7u64;
            if nat_lt(a1491, a1492) {
                let a1493 = v0;
                let a1494 = 6u64;
                if nat_eq(a1493, a1494) {
                    Ok(f54(2u64, f69(), f111(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1495 = v0;
                let a1496 = 7u64;
                if nat_eq(a1495, a1496) {
                    Ok(f65(2u64, f69(), f67(), 1129u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f46(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1497 = v0;
    let a1498 = 4u64;
    if nat_lt(a1497, a1498) {
        let a1499 = v0;
        let a1500 = 2u64;
        if nat_lt(a1499, a1500) {
            let a1501 = v0;
            let a1502 = 1u64;
            if nat_lt(a1501, a1502) {
                let a1503 = v0;
                let a1504 = 0u64;
                if nat_eq(a1503, a1504) {
                    Ok(f73(1133u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1505 = v0;
                let a1506 = 1u64;
                if nat_eq(a1505, a1506) {
                    Ok(f64(4u64, f104(), 256u64, f107(f85())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1507 = v0;
            let a1508 = 3u64;
            if nat_lt(a1507, a1508) {
                let a1509 = v0;
                let a1510 = 2u64;
                if nat_eq(a1509, a1510) {
                    Ok(f57(f110(), f81()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1511 = v0;
                let a1512 = 3u64;
                if nat_eq(a1511, a1512) {
                    Ok(f57(f111(), f80()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1513 = v0;
        let a1514 = 6u64;
        if nat_lt(a1513, a1514) {
            let a1515 = v0;
            let a1516 = 5u64;
            if nat_lt(a1515, a1516) {
                let a1517 = v0;
                let a1518 = 4u64;
                if nat_eq(a1517, a1518) {
                    Ok(f57(1u64, f108()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1519 = v0;
                let a1520 = 5u64;
                if nat_eq(a1519, a1520) {
                    Ok(f64(0u64, f85(), 1u64, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1521 = v0;
            let a1522 = 7u64;
            if nat_lt(a1521, a1522) {
                let a1523 = v0;
                let a1524 = 6u64;
                if nat_eq(a1523, a1524) {
                    Ok(f65(1u64, f85(), 32u64, 1094u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1525 = v0;
                let a1526 = 7u64;
                if nat_eq(a1525, a1526) {
                    Ok(f65(0u64, f108(), 0u64, 1138u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f47(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1527 = v0;
    let a1528 = 4u64;
    if nat_lt(a1527, a1528) {
        let a1529 = v0;
        let a1530 = 2u64;
        if nat_lt(a1529, a1530) {
            let a1531 = v0;
            let a1532 = 1u64;
            if nat_lt(a1531, a1532) {
                let a1533 = v0;
                let a1534 = 0u64;
                if nat_eq(a1533, a1534) {
                    f64(0u64, f59(), 1u64, f59())
                } else {
                    f2()
                }
            } else {
                let a1535 = v0;
                let a1536 = 1u64;
                if nat_eq(a1535, a1536) {
                    f65(1u64, f59(), 5u64, 1092u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1537 = v0;
            let a1538 = 3u64;
            if nat_lt(a1537, a1538) {
                let a1539 = v0;
                let a1540 = 2u64;
                if nat_eq(a1539, a1540) {
                    f65(2u64, 0u64, f80(), 1140u64)
                } else {
                    f2()
                }
            } else {
                let a1541 = v0;
                let a1542 = 3u64;
                if nat_eq(a1541, a1542) {
                    f73(3091u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1543 = v0;
        let a1544 = 6u64;
        if nat_lt(a1543, a1544) {
            let a1545 = v0;
            let a1546 = 5u64;
            if nat_lt(a1545, a1546) {
                let a1547 = v0;
                let a1548 = 4u64;
                if nat_eq(a1547, a1548) {
                    f54(3u64, f81(), f80(), f56())
                } else {
                    f2()
                }
            } else {
                let a1549 = v0;
                let a1550 = 5u64;
                if nat_eq(a1549, a1550) {
                    f54(2u64, f56(), 1077936128u64, f55())
                } else {
                    f2()
                }
            }
        } else {
            let a1551 = v0;
            let a1552 = 7u64;
            if nat_lt(a1551, a1552) {
                let a1553 = v0;
                let a1554 = 6u64;
                if nat_eq(a1553, a1554) {
                    f65(2u64, 0u64, f55(), 1144u64)
                } else {
                    f2()
                }
            } else {
                let a1555 = v0;
                let a1556 = 7u64;
                if nat_eq(a1555, a1556) {
                    f73(3091u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f48(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1557 = v0;
    let a1558 = 4u64;
    if nat_lt(a1557, a1558) {
        let a1559 = v0;
        let a1560 = 2u64;
        if nat_lt(a1559, a1560) {
            let a1561 = v0;
            let a1562 = 1u64;
            if nat_lt(a1561, a1562) {
                let a1563 = v0;
                let a1564 = 0u64;
                if nat_eq(a1563, a1564) {
                    Ok(f57(0u64, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1565 = v0;
                let a1566 = 1u64;
                if nat_eq(a1565, a1566) {
                    Ok(f57(0u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1567 = v0;
            let a1568 = 3u64;
            if nat_lt(a1567, a1568) {
                let a1569 = v0;
                let a1570 = 2u64;
                if nat_eq(a1569, a1570) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1571 = v0;
                let a1572 = 3u64;
                if nat_eq(a1571, a1572) {
                    Ok(f57(0u64, f61(f60())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1573 = v0;
        let a1574 = 6u64;
        if nat_lt(a1573, a1574) {
            let a1575 = v0;
            let a1576 = 5u64;
            if nat_lt(a1575, a1576) {
                let a1577 = v0;
                let a1578 = 4u64;
                if nat_eq(a1577, a1578) {
                    Ok(f57(1u64, f63(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1579 = v0;
                let a1580 = 5u64;
                if nat_eq(a1579, a1580) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1581 = v0;
            let a1582 = 7u64;
            if nat_lt(a1581, a1582) {
                let a1583 = v0;
                let a1584 = 6u64;
                if nat_eq(a1583, a1584) {
                    Ok(f65(1u64, f60(), 4u64, 1147u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1585 = v0;
                let a1586 = 7u64;
                if nat_eq(a1585, a1586) {
                    Ok(f66(f59(), f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f49(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1587 = v0;
    let a1588 = 4u64;
    if nat_lt(a1587, a1588) {
        let a1589 = v0;
        let a1590 = 2u64;
        if nat_lt(a1589, a1590) {
            let a1591 = v0;
            let a1592 = 1u64;
            if nat_lt(a1591, a1592) {
                let a1593 = v0;
                let a1594 = 0u64;
                if nat_eq(a1593, a1594) {
                    f66(f68(), f69())
                } else {
                    f2()
                }
            } else {
                let a1595 = v0;
                let a1596 = 1u64;
                if nat_eq(a1595, a1596) {
                    f54(1u64, f67(), f69(), f67())
                } else {
                    f2()
                }
            }
        } else {
            let a1597 = v0;
            let a1598 = 3u64;
            if nat_lt(a1597, a1598) {
                let a1599 = v0;
                let a1600 = 2u64;
                if nat_eq(a1599, a1600) {
                    f54(2u64, f67(), 1036831949u64, f67())
                } else {
                    f2()
                }
            } else {
                let a1601 = v0;
                let a1602 = 3u64;
                if nat_eq(a1601, a1602) {
                    f54(0u64, 1084227584u64, f67(), f67())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1603 = v0;
        let a1604 = 6u64;
        if nat_lt(a1603, a1604) {
            let a1605 = v0;
            let a1606 = 5u64;
            if nat_lt(a1605, a1606) {
                let a1607 = v0;
                let a1608 = 4u64;
                if nat_eq(a1607, a1608) {
                    f54(3u64, f67(), f55(), f70())
                } else {
                    f2()
                }
            } else {
                let a1609 = v0;
                let a1610 = 5u64;
                if nat_eq(a1609, a1610) {
                    f54(3u64, 1065353216u64, f70(), f71())
                } else {
                    f2()
                }
            }
        } else {
            let a1611 = v0;
            let a1612 = 7u64;
            if nat_lt(a1611, a1612) {
                let a1613 = v0;
                let a1614 = 6u64;
                if nat_eq(a1613, a1614) {
                    f57(0u64, f60())
                } else {
                    f2()
                }
            } else {
                let a1615 = v0;
                let a1616 = 7u64;
                if nat_eq(a1615, a1616) {
                    f57(1161u64, f72())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f50(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1617 = v0;
    let a1618 = 4u64;
    if nat_lt(a1617, a1618) {
        let a1619 = v0;
        let a1620 = 2u64;
        if nat_lt(a1619, a1620) {
            let a1621 = v0;
            let a1622 = 1u64;
            if nat_lt(a1621, a1622) {
                let a1623 = v0;
                let a1624 = 0u64;
                if nat_eq(a1623, a1624) {
                    Ok(f73(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1625 = v0;
                let a1626 = 1u64;
                if nat_eq(a1625, a1626) {
                    Ok(f57(f74(), f75(f60())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1627 = v0;
            let a1628 = 3u64;
            if nat_lt(a1627, a1628) {
                let a1629 = v0;
                let a1630 = 2u64;
                if nat_eq(a1629, a1630) {
                    Ok(f57(f76(), f77(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1631 = v0;
                let a1632 = 3u64;
                if nat_eq(a1631, a1632) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1633 = v0;
        let a1634 = 6u64;
        if nat_lt(a1633, a1634) {
            let a1635 = v0;
            let a1636 = 5u64;
            if nat_lt(a1635, a1636) {
                let a1637 = v0;
                let a1638 = 4u64;
                if nat_eq(a1637, a1638) {
                    Ok(f65(1u64, f60(), 4u64, 1159u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1639 = v0;
                let a1640 = 5u64;
                if nat_eq(a1639, a1640) {
                    Ok(f57(0u64, f78()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1641 = v0;
            let a1642 = 7u64;
            if nat_lt(a1641, a1642) {
                let a1643 = v0;
                let a1644 = 6u64;
                if nat_eq(a1643, a1644) {
                    Ok(f57(1168u64, f79()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1645 = v0;
                let a1646 = 7u64;
                if nat_eq(a1645, a1646) {
                    Ok(f73(2048u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f51(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1647 = v0;
    let a1648 = 4u64;
    if nat_lt(a1647, a1648) {
        let a1649 = v0;
        let a1650 = 2u64;
        if nat_lt(a1649, a1650) {
            let a1651 = v0;
            let a1652 = 1u64;
            if nat_lt(a1651, a1652) {
                let a1653 = v0;
                let a1654 = 0u64;
                if nat_eq(a1653, a1654) {
                    f65(2u64, 0u64, f80(), 1170u64)
                } else {
                    f2()
                }
            } else {
                let a1655 = v0;
                let a1656 = 1u64;
                if nat_eq(a1655, a1656) {
                    f73(1181u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1657 = v0;
            let a1658 = 3u64;
            if nat_lt(a1657, a1658) {
                let a1659 = v0;
                let a1660 = 2u64;
                if nat_eq(a1659, a1660) {
                    f54(2u64, f81(), f81(), f69())
                } else {
                    f2()
                }
            } else {
                let a1661 = v0;
                let a1662 = 3u64;
                if nat_eq(a1661, a1662) {
                    f54(2u64, f58(), f80(), f67())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1663 = v0;
        let a1664 = 6u64;
        if nat_lt(a1663, a1664) {
            let a1665 = v0;
            let a1666 = 5u64;
            if nat_lt(a1665, a1666) {
                let a1667 = v0;
                let a1668 = 4u64;
                if nat_eq(a1667, a1668) {
                    f65(2u64, f67(), f69(), 1174u64)
                } else {
                    f2()
                }
            } else {
                let a1669 = v0;
                let a1670 = 5u64;
                if nat_eq(a1669, a1670) {
                    f73(1181u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1671 = v0;
            let a1672 = 7u64;
            if nat_lt(a1671, a1672) {
                let a1673 = v0;
                let a1674 = 6u64;
                if nat_eq(a1673, a1674) {
                    f54(3u64, f81(), f80(), f56())
                } else {
                    f2()
                }
            } else {
                let a1675 = v0;
                let a1676 = 7u64;
                if nat_eq(a1675, a1676) {
                    f54(2u64, f56(), f81(), f58())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f52(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1677 = v0;
    let a1678 = 4u64;
    if nat_lt(a1677, a1678) {
        let a1679 = v0;
        let a1680 = 2u64;
        if nat_lt(a1679, a1680) {
            let a1681 = v0;
            let a1682 = 1u64;
            if nat_lt(a1681, a1682) {
                let a1683 = v0;
                let a1684 = 0u64;
                if nat_eq(a1683, a1684) {
                    Ok(f57(0u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1685 = v0;
                let a1686 = 1u64;
                if nat_eq(a1685, a1686) {
                    Ok(f57(f75(f60())?, f61(f60())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1687 = v0;
            let a1688 = 3u64;
            if nat_lt(a1687, a1688) {
                let a1689 = v0;
                let a1690 = 2u64;
                if nat_eq(a1689, a1690) {
                    Ok(f57(f77(f60())?, f63(f60())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1691 = v0;
                let a1692 = 3u64;
                if nat_eq(a1691, a1692) {
                    Ok(f64(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1693 = v0;
        let a1694 = 6u64;
        if nat_lt(a1693, a1694) {
            let a1695 = v0;
            let a1696 = 5u64;
            if nat_lt(a1695, a1696) {
                let a1697 = v0;
                let a1698 = 4u64;
                if nat_eq(a1697, a1698) {
                    Ok(f65(1u64, f60(), 4u64, 1177u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1699 = v0;
                let a1700 = 5u64;
                if nat_eq(a1699, a1700) {
                    Ok(f64(0u64, f59(), 1u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1701 = v0;
            let a1702 = 7u64;
            if nat_lt(a1701, a1702) {
                let a1703 = v0;
                let a1704 = 6u64;
                if nat_eq(a1703, a1704) {
                    Ok(f64(2u64, f68(), 2u64, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1705 = v0;
                let a1706 = 7u64;
                if nat_eq(a1705, a1706) {
                    Ok(f64(0u64, f69(), 1u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f53(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1707 = v0;
    let a1708 = 1u64;
    if nat_lt(a1707, a1708) {
        let a1709 = v0;
        let a1710 = 0u64;
        if nat_eq(a1709, a1710) {
            f65(1u64, f59(), f69(), 1151u64)
        } else {
            f2()
        }
    } else {
        let a1711 = v0;
        let a1712 = 1u64;
        if nat_eq(a1711, a1712) {
            f73(2560u64)
        } else {
            f2()
        }
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f55() -> u64 {
    4294967315u64
}

pub fn f56() -> u64 {
    4294967316u64
}

pub fn f57(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f58() -> u64 {
    4294967317u64
}

pub fn f59() -> u64 {
    4294967318u64
}

pub fn f60() -> u64 {
    4294967323u64
}

pub fn f61(v0: u64) -> R<u64> {
    let a1716 = 8589980672u64;
    let a1717 = {
        let a1713 = v0;
        let a1714 = 4294967296u64;
        let a1715 = 0u64;
        nat_rem(a1713, a1714, a1715)
    };
    nat_add(a1716, a1717)
}

pub fn f62() -> u64 {
    4294967297u64
}

pub fn f63(v0: u64) -> R<u64> {
    let a1721 = 8589982720u64;
    let a1722 = {
        let a1718 = v0;
        let a1719 = 4294967296u64;
        let a1720 = 0u64;
        nat_rem(a1718, a1719, a1720)
    };
    nat_add(a1721, a1722)
}

pub fn f64(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f66(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f67() -> u64 {
    4294967337u64
}

pub fn f68() -> u64 {
    4294967299u64
}

pub fn f69() -> u64 {
    4294967338u64
}

pub fn f70() -> u64 {
    4294967319u64
}

pub fn f71() -> u64 {
    4294967320u64
}

pub fn f72() -> u64 {
    4294967350u64
}

pub fn f73(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f74() -> u64 {
    4294967321u64
}

pub fn f75(v0: u64) -> R<u64> {
    let a1726 = 8589976576u64;
    let a1727 = {
        let a1723 = v0;
        let a1724 = 4294967296u64;
        let a1725 = 0u64;
        nat_rem(a1723, a1724, a1725)
    };
    nat_add(a1726, a1727)
}

pub fn f76() -> u64 {
    4294967322u64
}

pub fn f77(v0: u64) -> R<u64> {
    let a1731 = 8589978624u64;
    let a1732 = {
        let a1728 = v0;
        let a1729 = 4294967296u64;
        let a1730 = 0u64;
        nat_rem(a1728, a1729, a1730)
    };
    nat_add(a1731, a1732)
}

pub fn f78() -> u64 {
    4294967324u64
}

pub fn f79() -> u64 {
    4294967351u64
}

pub fn f80() -> u64 {
    4294967336u64
}

pub fn f81() -> u64 {
    4294967335u64
}

pub fn f82() -> u64 {
    4294967326u64
}

pub fn f83() -> u64 {
    4294967305u64
}

pub fn f84() -> u64 {
    4294967325u64
}

pub fn f85() -> u64 {
    4294967303u64
}

pub fn f86(v0: u64) -> R<u64> {
    let a1736 = 8589950976u64;
    let a1737 = {
        let a1733 = v0;
        let a1734 = 4294967296u64;
        let a1735 = 0u64;
        nat_rem(a1733, a1734, a1735)
    };
    nat_add(a1736, a1737)
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f88() -> u64 {
    4294967331u64
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f90() -> u64 {
    4294967343u64
}

pub fn f91(v0: u64) -> R<u64> {
    let a1741 = 21474836480u64;
    let a1742 = {
        let a1738 = v0;
        let a1739 = 4294967296u64;
        let a1740 = 0u64;
        nat_rem(a1738, a1739, a1740)
    };
    nat_add(a1741, a1742)
}

pub fn f92() -> u64 {
    4294967327u64
}

pub fn f93() -> u64 {
    4294967328u64
}

pub fn f94() -> u64 {
    4294967339u64
}

pub fn f95() -> u64 {
    4294967334u64
}

pub fn f96() -> u64 {
    4294967329u64
}

pub fn f97() -> u64 {
    4294967353u64
}

pub fn f98() -> u64 {
    4294967333u64
}

pub fn f99() -> u64 {
    4294967330u64
}

pub fn f100(v0: u64) -> R<u64> {
    let a1746 = 25769803776u64;
    let a1747 = {
        let a1743 = v0;
        let a1744 = 4294967296u64;
        let a1745 = 0u64;
        nat_rem(a1743, a1744, a1745)
    };
    nat_add(a1746, a1747)
}

pub fn f101() -> u64 {
    4294967332u64
}

pub fn f102(v0: u64) -> R<u64> {
    let a1751 = 8589967360u64;
    let a1752 = {
        let a1748 = v0;
        let a1749 = 4294967296u64;
        let a1750 = 0u64;
        nat_rem(a1748, a1749, a1750)
    };
    nat_add(a1751, a1752)
}

pub fn f103(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f104() -> u64 {
    4294967308u64
}

pub fn f105(v0: u64) -> R<u64> {
    let a1756 = 8589959168u64;
    let a1757 = {
        let a1753 = v0;
        let a1754 = 4294967296u64;
        let a1755 = 0u64;
        nat_rem(a1753, a1754, a1755)
    };
    nat_add(a1756, a1757)
}

pub fn f106() -> u64 {
    4294967357u64
}

pub fn f107(v0: u64) -> R<u64> {
    let a1761 = 8589988864u64;
    let a1762 = {
        let a1758 = v0;
        let a1759 = 4294967296u64;
        let a1760 = 0u64;
        nat_rem(a1758, a1759, a1760)
    };
    nat_add(a1761, a1762)
}

pub fn f108() -> u64 {
    4294967355u64
}

pub fn f109() -> u64 {
    4294967356u64
}

pub fn f110() -> u64 {
    4294967310u64
}

pub fn f111() -> u64 {
    4294967311u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
