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
    let a27 = 1u64;
    if nat_lt(a26, a27) {
        let a28 = v1;
        let a29 = 0u64;
        if nat_eq(a28, a29) {
            f8({
                let a30 = v0;
                let a31 = 8u64;
                let a32 = 0u64;
                nat_rem(a30, a31, a32)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a33 = v1;
        let a34 = 1u64;
        if nat_eq(a33, a34) {
            Ok(f9({
                let a35 = v0;
                let a36 = 8u64;
                let a37 = 0u64;
                nat_rem(a35, a36, a37)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a38 = v0;
        let a39 = 8u64;
        let a40 = 0u64;
        nat_quot(a38, a39, a40)
    };
    let a41 = v1;
    let a42 = 4u64;
    if nat_lt(a41, a42) {
        let a43 = v1;
        let a44 = 2u64;
        if nat_lt(a43, a44) {
            let a45 = v1;
            let a46 = 1u64;
            if nat_lt(a45, a46) {
                let a47 = v1;
                let a48 = 0u64;
                if nat_eq(a47, a48) {
                    Ok(f10({
                        let a49 = v0;
                        let a50 = 8u64;
                        let a51 = 0u64;
                        nat_rem(a49, a50, a51)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a52 = v1;
                let a53 = 1u64;
                if nat_eq(a52, a53) {
                    f11({
                        let a54 = v0;
                        let a55 = 8u64;
                        let a56 = 0u64;
                        nat_rem(a54, a55, a56)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a57 = v1;
            let a58 = 3u64;
            if nat_lt(a57, a58) {
                let a59 = v1;
                let a60 = 2u64;
                if nat_eq(a59, a60) {
                    f12({
                        let a61 = v0;
                        let a62 = 8u64;
                        let a63 = 0u64;
                        nat_rem(a61, a62, a63)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a64 = v1;
                let a65 = 3u64;
                if nat_eq(a64, a65) {
                    Ok(f13({
                        let a66 = v0;
                        let a67 = 8u64;
                        let a68 = 0u64;
                        nat_rem(a66, a67, a68)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a69 = v1;
        let a70 = 6u64;
        if nat_lt(a69, a70) {
            let a71 = v1;
            let a72 = 5u64;
            if nat_lt(a71, a72) {
                let a73 = v1;
                let a74 = 4u64;
                if nat_eq(a73, a74) {
                    Ok(f14({
                        let a75 = v0;
                        let a76 = 8u64;
                        let a77 = 0u64;
                        nat_rem(a75, a76, a77)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a78 = v1;
                let a79 = 5u64;
                if nat_eq(a78, a79) {
                    f15({
                        let a80 = v0;
                        let a81 = 8u64;
                        let a82 = 0u64;
                        nat_rem(a80, a81, a82)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a83 = v1;
            let a84 = 7u64;
            if nat_lt(a83, a84) {
                let a85 = v1;
                let a86 = 6u64;
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
            } else {
                let a90 = v1;
                let a91 = 7u64;
                if nat_eq(a90, a91) {
                    Ok(f17({
                        let a92 = v0;
                        let a93 = 8u64;
                        let a94 = 0u64;
                        nat_rem(a92, a93, a94)
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
        let a95 = v0;
        let a96 = 8u64;
        let a97 = 0u64;
        nat_quot(a95, a96, a97)
    };
    let a98 = v1;
    let a99 = 1u64;
    if nat_lt(a98, a99) {
        let a100 = v1;
        let a101 = 0u64;
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
    } else {
        let a105 = v1;
        let a106 = 1u64;
        if nat_eq(a105, a106) {
            Ok(f19({
                let a107 = v0;
                let a108 = 8u64;
                let a109 = 0u64;
                nat_rem(a107, a108, a109)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a110 = v0;
        let a111 = 8u64;
        let a112 = 0u64;
        nat_quot(a110, a111, a112)
    };
    let a113 = v1;
    let a114 = 4u64;
    if nat_lt(a113, a114) {
        let a115 = v1;
        let a116 = 2u64;
        if nat_lt(a115, a116) {
            let a117 = v1;
            let a118 = 1u64;
            if nat_lt(a117, a118) {
                let a119 = v1;
                let a120 = 0u64;
                if nat_eq(a119, a120) {
                    Ok(f20({
                        let a121 = v0;
                        let a122 = 8u64;
                        let a123 = 0u64;
                        nat_rem(a121, a122, a123)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a124 = v1;
                let a125 = 1u64;
                if nat_eq(a124, a125) {
                    f21({
                        let a126 = v0;
                        let a127 = 8u64;
                        let a128 = 0u64;
                        nat_rem(a126, a127, a128)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a129 = v1;
            let a130 = 3u64;
            if nat_lt(a129, a130) {
                let a131 = v1;
                let a132 = 2u64;
                if nat_eq(a131, a132) {
                    f22({
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
                let a137 = 3u64;
                if nat_eq(a136, a137) {
                    Ok(f23({
                        let a138 = v0;
                        let a139 = 8u64;
                        let a140 = 0u64;
                        nat_rem(a138, a139, a140)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a141 = v1;
        let a142 = 6u64;
        if nat_lt(a141, a142) {
            let a143 = v1;
            let a144 = 5u64;
            if nat_lt(a143, a144) {
                let a145 = v1;
                let a146 = 4u64;
                if nat_eq(a145, a146) {
                    Ok(f24({
                        let a147 = v0;
                        let a148 = 8u64;
                        let a149 = 0u64;
                        nat_rem(a147, a148, a149)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a150 = v1;
                let a151 = 5u64;
                if nat_eq(a150, a151) {
                    Ok(f25({
                        let a152 = v0;
                        let a153 = 8u64;
                        let a154 = 0u64;
                        nat_rem(a152, a153, a154)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a155 = v1;
            let a156 = 7u64;
            if nat_lt(a155, a156) {
                let a157 = v1;
                let a158 = 6u64;
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
                let a163 = 7u64;
                if nat_eq(a162, a163) {
                    f27({
                        let a164 = v0;
                        let a165 = 8u64;
                        let a166 = 0u64;
                        nat_rem(a164, a165, a166)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a167 = v0;
        let a168 = 8u64;
        let a169 = 0u64;
        nat_quot(a167, a168, a169)
    };
    let a170 = v1;
    let a171 = 1u64;
    if nat_lt(a170, a171) {
        let a172 = v1;
        let a173 = 0u64;
        if nat_eq(a172, a173) {
            f18({
                let a174 = v0;
                let a175 = 8u64;
                let a176 = 0u64;
                nat_rem(a174, a175, a176)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a177 = v1;
        let a178 = 1u64;
        if nat_eq(a177, a178) {
            Ok(f19({
                let a179 = v0;
                let a180 = 8u64;
                let a181 = 0u64;
                nat_rem(a179, a180, a181)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a182 = v0;
        let a183 = 8u64;
        let a184 = 0u64;
        nat_quot(a182, a183, a184)
    };
    let a185 = v1;
    let a186 = 4u64;
    if nat_lt(a185, a186) {
        let a187 = v1;
        let a188 = 2u64;
        if nat_lt(a187, a188) {
            let a189 = v1;
            let a190 = 1u64;
            if nat_lt(a189, a190) {
                let a191 = v1;
                let a192 = 0u64;
                if nat_eq(a191, a192) {
                    Ok(f10({
                        let a193 = v0;
                        let a194 = 8u64;
                        let a195 = 0u64;
                        nat_rem(a193, a194, a195)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a196 = v1;
                let a197 = 1u64;
                if nat_eq(a196, a197) {
                    f21({
                        let a198 = v0;
                        let a199 = 8u64;
                        let a200 = 0u64;
                        nat_rem(a198, a199, a200)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a201 = v1;
            let a202 = 3u64;
            if nat_lt(a201, a202) {
                let a203 = v1;
                let a204 = 2u64;
                if nat_eq(a203, a204) {
                    f22({
                        let a205 = v0;
                        let a206 = 8u64;
                        let a207 = 0u64;
                        nat_rem(a205, a206, a207)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a208 = v1;
                let a209 = 3u64;
                if nat_eq(a208, a209) {
                    Ok(f23({
                        let a210 = v0;
                        let a211 = 8u64;
                        let a212 = 0u64;
                        nat_rem(a210, a211, a212)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a213 = v1;
        let a214 = 6u64;
        if nat_lt(a213, a214) {
            let a215 = v1;
            let a216 = 5u64;
            if nat_lt(a215, a216) {
                let a217 = v1;
                let a218 = 4u64;
                if nat_eq(a217, a218) {
                    Ok(f24({
                        let a219 = v0;
                        let a220 = 8u64;
                        let a221 = 0u64;
                        nat_rem(a219, a220, a221)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a222 = v1;
                let a223 = 5u64;
                if nat_eq(a222, a223) {
                    Ok(f25({
                        let a224 = v0;
                        let a225 = 8u64;
                        let a226 = 0u64;
                        nat_rem(a224, a225, a226)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a227 = v1;
            let a228 = 7u64;
            if nat_lt(a227, a228) {
                let a229 = v1;
                let a230 = 6u64;
                if nat_eq(a229, a230) {
                    f28({
                        let a231 = v0;
                        let a232 = 8u64;
                        let a233 = 0u64;
                        nat_rem(a231, a232, a233)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a234 = v1;
                let a235 = 7u64;
                if nat_eq(a234, a235) {
                    f29({
                        let a236 = v0;
                        let a237 = 8u64;
                        let a238 = 0u64;
                        nat_rem(a236, a237, a238)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a239 = v0;
    let a240 = 4u64;
    if nat_lt(a239, a240) {
        let a241 = v0;
        let a242 = 2u64;
        if nat_lt(a241, a242) {
            let a243 = v0;
            let a244 = 1u64;
            if nat_lt(a243, a244) {
                let a245 = v0;
                let a246 = 0u64;
                if nat_eq(a245, a246) {
                    Ok(f30(0u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a247 = v0;
                let a248 = 1u64;
                if nat_eq(a247, a248) {
                    Ok(f30(0u64, f32()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a249 = v0;
            let a250 = 3u64;
            if nat_lt(a249, a250) {
                let a251 = v0;
                let a252 = 2u64;
                if nat_eq(a251, a252) {
                    Ok(f30(0u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a253 = v0;
                let a254 = 3u64;
                if nat_eq(a253, a254) {
                    Ok(f30(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a255 = v0;
        let a256 = 6u64;
        if nat_lt(a255, a256) {
            let a257 = v0;
            let a258 = 5u64;
            if nat_lt(a257, a258) {
                let a259 = v0;
                let a260 = 4u64;
                if nat_eq(a259, a260) {
                    Ok(f35(0u64, f36(), 0u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a261 = v0;
                let a262 = 5u64;
                if nat_eq(a261, a262) {
                    Ok(f30(0u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a263 = v0;
            let a264 = 7u64;
            if nat_lt(a263, a264) {
                let a265 = v0;
                let a266 = 6u64;
                if nat_eq(a265, a266) {
                    Ok(f38(2u64, f39(f37())?, f39(f37())?, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a267 = v0;
                let a268 = 7u64;
                if nat_eq(a267, a268) {
                    Ok(f38(0u64, f31(), f40(), f31()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a269 = v0;
    let a270 = 2u64;
    if nat_lt(a269, a270) {
        let a271 = v0;
        let a272 = 1u64;
        if nat_lt(a271, a272) {
            let a273 = v0;
            let a274 = 0u64;
            if nat_eq(a273, a274) {
                f41(0u64, f37(), 1u64, f37())
            } else {
                f2()
            }
        } else {
            let a275 = v0;
            let a276 = 1u64;
            if nat_eq(a275, a276) {
                f35(1u64, f37(), 256u64, 6u64)
            } else {
                f2()
            }
        }
    } else {
        let a277 = v0;
        let a278 = 3u64;
        if nat_lt(a277, a278) {
            let a279 = v0;
            let a280 = 2u64;
            if nat_eq(a279, a280) {
                f38(2u64, 1073741824u64, f31(), f31())
            } else {
                f2()
            }
        } else {
            let a281 = v0;
            let a282 = 4u64;
            if nat_lt(a281, a282) {
                let a283 = v0;
                let a284 = 3u64;
                if nat_eq(a283, a284) {
                    f38(3u64, f31(), 1132462080u64, f31())
                } else {
                    f2()
                }
            } else {
                let a285 = v0;
                let a286 = 4u64;
                if nat_eq(a285, a286) {
                    f42(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f30(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a295 = v0;
                let a296 = 1u64;
                if nat_eq(a295, a296) {
                    f30(0u64, f44())
                } else {
                    f2()
                }
            }
        } else {
            let a297 = v0;
            let a298 = 3u64;
            if nat_lt(a297, a298) {
                let a299 = v0;
                let a300 = 2u64;
                if nat_eq(a299, a300) {
                    f41(2u64, f32(), 32u64, f45())
                } else {
                    f2()
                }
            } else {
                let a301 = v0;
                let a302 = 3u64;
                if nat_eq(a301, a302) {
                    f30(0u64, f46())
                } else {
                    f2()
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
                    f30(0u64, f47())
                } else {
                    f2()
                }
            } else {
                let a309 = v0;
                let a310 = 5u64;
                if nat_eq(a309, a310) {
                    f30(0u64, f48())
                } else {
                    f2()
                }
            }
        } else {
            let a311 = v0;
            let a312 = 7u64;
            if nat_lt(a311, a312) {
                let a313 = v0;
                let a314 = 6u64;
                if nat_eq(a313, a314) {
                    f30(0u64, f49())
                } else {
                    f2()
                }
            } else {
                let a315 = v0;
                let a316 = 7u64;
                if nat_eq(a315, a316) {
                    f41(0u64, f45(), f43(), f50())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f30(f39(f50())?, f51()))
                } else {
                    Ok(f2())
                }
            } else {
                let a325 = v0;
                let a326 = 1u64;
                if nat_eq(a325, a326) {
                    Ok(f38(2u64, f51(), f51(), f52()))
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
                    Ok(f35(0u64, f36(), 0u64, 526u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a331 = v0;
                let a332 = 3u64;
                if nat_eq(a331, a332) {
                    Ok(f38(0u64, f31(), f52(), f52()))
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
                    Ok(f38(4u64, f52(), 0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a339 = v0;
                let a340 = 5u64;
                if nat_eq(a339, a340) {
                    Ok(f38(2u64, f53(f50())?, f52(), f52()))
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
                    Ok(f30(f52(), f54(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a345 = v0;
                let a346 = 7u64;
                if nat_eq(a345, a346) {
                    Ok(f38(4u64, f52(), 0u64, f55(f43())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f30(f51(), f56(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a355 = v0;
                let a356 = 1u64;
                if nat_eq(a355, a356) {
                    Ok(f35(2u64, f51(), 0u64, 531u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a357 = v0;
            let a358 = 3u64;
            if nat_lt(a357, a358) {
                let a359 = v0;
                let a360 = 2u64;
                if nat_eq(a359, a360) {
                    Ok(f42(535u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a361 = v0;
                let a362 = 3u64;
                if nat_eq(a361, a362) {
                    Ok(f38(6u64, f51(), 0u64, f56(f43())?))
                } else {
                    Ok(f2())
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
                    Ok(f57(6u64, f49(), 0u64, 0u64, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a369 = v0;
                let a370 = 5u64;
                if nat_eq(a369, a370) {
                    Ok(f41(0u64, f48(), f58(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a371 = v0;
            let a372 = 7u64;
            if nat_lt(a371, a372) {
                let a373 = v0;
                let a374 = 6u64;
                if nat_eq(a373, a374) {
                    Ok(f41(1u64, 1u64, f47(), f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a375 = v0;
                let a376 = 7u64;
                if nat_eq(a375, a376) {
                    Ok(f35(0u64, f59(), 0u64, 543u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f38(2u64, f52(), f51(), f60())
                } else {
                    f2()
                }
            } else {
                let a385 = v0;
                let a386 = 1u64;
                if nat_eq(a385, a386) {
                    f38(2u64, f60(), f51(), f60())
                } else {
                    f2()
                }
            }
        } else {
            let a387 = v0;
            let a388 = 3u64;
            if nat_lt(a387, a388) {
                let a389 = v0;
                let a390 = 2u64;
                if nat_eq(a389, a390) {
                    f35(0u64, f49(), 0u64, 541u64)
                } else {
                    f2()
                }
            } else {
                let a391 = v0;
                let a392 = 3u64;
                if nat_eq(a391, a392) {
                    f35(2u64, f60(), f61(), 541u64)
                } else {
                    f2()
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
                    f42(543u64)
                } else {
                    f2()
                }
            } else {
                let a399 = v0;
                let a400 = 5u64;
                if nat_eq(a399, a400) {
                    f30(f60(), f61())
                } else {
                    f2()
                }
            }
        } else {
            let a401 = v0;
            let a402 = 7u64;
            if nat_lt(a401, a402) {
                let a403 = v0;
                let a404 = 6u64;
                if nat_eq(a403, a404) {
                    f30(f49(), f62())
                } else {
                    f2()
                }
            } else {
                let a405 = v0;
                let a406 = 7u64;
                if nat_eq(a405, a406) {
                    f41(0u64, f49(), 1u64, f49())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f41(0u64, f43(), 1u64, f43())
                } else {
                    f2()
                }
            } else {
                let a415 = v0;
                let a416 = 1u64;
                if nat_eq(a415, a416) {
                    f35(1u64, f49(), 8u64, 519u64)
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
                    f35(0u64, f59(), 0u64, 558u64)
                } else {
                    f2()
                }
            } else {
                let a421 = v0;
                let a422 = 3u64;
                if nat_eq(a421, a422) {
                    f35(0u64, f47(), 0u64, 558u64)
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
                    f57(6u64, f62(), 0u64, 0u64, f58())
                } else {
                    f2()
                }
            } else {
                let a429 = v0;
                let a430 = 5u64;
                if nat_eq(a429, a430) {
                    f41(3u64, f48(), f58(), f63())
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
                    f41(4u64, f63(), 2u64, f63())
                } else {
                    f2()
                }
            } else {
                let a435 = v0;
                let a436 = 7u64;
                if nat_eq(a435, a436) {
                    f35(0u64, f63(), 0u64, 554u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a437 = v0;
    let a438 = 4u64;
    if nat_lt(a437, a438) {
        let a439 = v0;
        let a440 = 2u64;
        if nat_lt(a439, a440) {
            let a441 = v0;
            let a442 = 1u64;
            if nat_lt(a441, a442) {
                let a443 = v0;
                let a444 = 0u64;
                if nat_eq(a443, a444) {
                    Ok(f41(1u64, f48(), f58(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a445 = v0;
                let a446 = 1u64;
                if nat_eq(a445, a446) {
                    Ok(f42(555u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a447 = v0;
            let a448 = 3u64;
            if nat_lt(a447, a448) {
                let a449 = v0;
                let a450 = 2u64;
                if nat_eq(a449, a450) {
                    Ok(f41(0u64, f48(), f58(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a451 = v0;
                let a452 = 3u64;
                if nat_eq(a451, a452) {
                    Ok(f41(2u64, f44(), 8u64, f63()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a453 = v0;
        let a454 = 6u64;
        if nat_lt(a453, a454) {
            let a455 = v0;
            let a456 = 5u64;
            if nat_lt(a455, a456) {
                let a457 = v0;
                let a458 = 4u64;
                if nat_eq(a457, a458) {
                    Ok(f41(0u64, f63(), f62(), f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a459 = v0;
                let a460 = 5u64;
                if nat_eq(a459, a460) {
                    Ok(f38(6u64, f56(f63())?, 0u64, f56(f63())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a461 = v0;
            let a462 = 7u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 6u64;
                if nat_eq(a463, a464) {
                    Ok(f30(f48(), f64(f44())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a465 = v0;
                let a466 = 7u64;
                if nat_eq(a465, a466) {
                    Ok(f41(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a467 = v0;
    let a468 = 4u64;
    if nat_lt(a467, a468) {
        let a469 = v0;
        let a470 = 2u64;
        if nat_lt(a469, a470) {
            let a471 = v0;
            let a472 = 1u64;
            if nat_lt(a471, a472) {
                let a473 = v0;
                let a474 = 0u64;
                if nat_eq(a473, a474) {
                    Ok(f35(1u64, f44(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a475 = v0;
                let a476 = 1u64;
                if nat_eq(a475, a476) {
                    Ok(f30(0u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a477 = v0;
            let a478 = 3u64;
            if nat_lt(a477, a478) {
                let a479 = v0;
                let a480 = 2u64;
                if nat_eq(a479, a480) {
                    Ok(f35(2u64, f46(), f56(f43())?, 564u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a481 = v0;
                let a482 = 3u64;
                if nat_eq(a481, a482) {
                    Ok(f42(565u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a483 = v0;
        let a484 = 6u64;
        if nat_lt(a483, a484) {
            let a485 = v0;
            let a486 = 5u64;
            if nat_lt(a485, a486) {
                let a487 = v0;
                let a488 = 4u64;
                if nat_eq(a487, a488) {
                    Ok(f30(f56(f43())?, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a489 = v0;
                let a490 = 5u64;
                if nat_eq(a489, a490) {
                    Ok(f41(0u64, f43(), 1u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a491 = v0;
            let a492 = 7u64;
            if nat_lt(a491, a492) {
                let a493 = v0;
                let a494 = 6u64;
                if nat_eq(a493, a494) {
                    Ok(f35(1u64, f43(), 32u64, 562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a495 = v0;
                let a496 = 7u64;
                if nat_eq(a495, a496) {
                    Ok(f30(0u64, f65(f32())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a497 = v0;
    let a498 = 1u64;
    if nat_lt(a497, a498) {
        let a499 = v0;
        let a500 = 0u64;
        if nat_eq(a499, a500) {
            f35(0u64, f46(), 0u64, 3109u64)
        } else {
            f2()
        }
    } else {
        let a501 = v0;
        let a502 = 2u64;
        if nat_lt(a501, a502) {
            let a503 = v0;
            let a504 = 1u64;
            if nat_eq(a503, a504) {
                f35(2u64, f46(), f66(), 3109u64)
            } else {
                f2()
            }
        } else {
            let a505 = v0;
            let a506 = 2u64;
            if nat_eq(a505, a506) {
                f42(1024u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a507 = v0;
    let a508 = 4u64;
    if nat_lt(a507, a508) {
        let a509 = v0;
        let a510 = 2u64;
        if nat_lt(a509, a510) {
            let a511 = v0;
            let a512 = 1u64;
            if nat_lt(a511, a512) {
                let a513 = v0;
                let a514 = 0u64;
                if nat_eq(a513, a514) {
                    Ok(f30(0u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a515 = v0;
                let a516 = 1u64;
                if nat_eq(a515, a516) {
                    Ok(f30(0u64, f32()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a517 = v0;
            let a518 = 3u64;
            if nat_lt(a517, a518) {
                let a519 = v0;
                let a520 = 2u64;
                if nat_eq(a519, a520) {
                    Ok(f30(0u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a521 = v0;
                let a522 = 3u64;
                if nat_eq(a521, a522) {
                    Ok(f30(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a523 = v0;
        let a524 = 6u64;
        if nat_lt(a523, a524) {
            let a525 = v0;
            let a526 = 5u64;
            if nat_lt(a525, a526) {
                let a527 = v0;
                let a528 = 4u64;
                if nat_eq(a527, a528) {
                    Ok(f30(0u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a529 = v0;
                let a530 = 5u64;
                if nat_eq(a529, a530) {
                    Ok(f38(2u64, f39(f37())?, f39(f37())?, f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a531 = v0;
            let a532 = 7u64;
            if nat_lt(a531, a532) {
                let a533 = v0;
                let a534 = 6u64;
                if nat_eq(a533, a534) {
                    Ok(f38(0u64, f31(), f40(), f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a535 = v0;
                let a536 = 7u64;
                if nat_eq(a535, a536) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a537 = v0;
    let a538 = 2u64;
    if nat_lt(a537, a538) {
        let a539 = v0;
        let a540 = 1u64;
        if nat_lt(a539, a540) {
            let a541 = v0;
            let a542 = 0u64;
            if nat_eq(a541, a542) {
                f35(1u64, f37(), 256u64, 5u64)
            } else {
                f2()
            }
        } else {
            let a543 = v0;
            let a544 = 1u64;
            if nat_eq(a543, a544) {
                f35(0u64, f59(), 1u64, 11u64)
            } else {
                f2()
            }
        }
    } else {
        let a545 = v0;
        let a546 = 3u64;
        if nat_lt(a545, a546) {
            let a547 = v0;
            let a548 = 2u64;
            if nat_eq(a547, a548) {
                f38(2u64, 1073741824u64, f31(), f31())
            } else {
                f2()
            }
        } else {
            let a549 = v0;
            let a550 = 4u64;
            if nat_lt(a549, a550) {
                let a551 = v0;
                let a552 = 3u64;
                if nat_eq(a551, a552) {
                    f38(3u64, f31(), 1132462080u64, f31())
                } else {
                    f2()
                }
            } else {
                let a553 = v0;
                let a554 = 4u64;
                if nat_eq(a553, a554) {
                    f42(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f30(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a563 = v0;
                let a564 = 1u64;
                if nat_eq(a563, a564) {
                    f30(0u64, f44())
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
                    f41(2u64, f32(), 16u64, f45())
                } else {
                    f2()
                }
            } else {
                let a569 = v0;
                let a570 = 3u64;
                if nat_eq(a569, a570) {
                    f30(0u64, f46())
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
                    f30(0u64, f47())
                } else {
                    f2()
                }
            } else {
                let a577 = v0;
                let a578 = 5u64;
                if nat_eq(a577, a578) {
                    f30(0u64, f48())
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
                    f30(0u64, f49())
                } else {
                    f2()
                }
            } else {
                let a583 = v0;
                let a584 = 7u64;
                if nat_eq(a583, a584) {
                    f41(0u64, f45(), f43(), f50())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f30(f39(f50())?, f51()))
                } else {
                    Ok(f2())
                }
            } else {
                let a593 = v0;
                let a594 = 1u64;
                if nat_eq(a593, a594) {
                    Ok(f38(2u64, f51(), f51(), f52()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a595 = v0;
            let a596 = 3u64;
            if nat_lt(a595, a596) {
                let a597 = v0;
                let a598 = 2u64;
                if nat_eq(a597, a598) {
                    Ok(f35(0u64, f36(), 0u64, 527u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a599 = v0;
                let a600 = 3u64;
                if nat_eq(a599, a600) {
                    Ok(f38(0u64, f31(), f52(), f52()))
                } else {
                    Ok(f2())
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
                    Ok(f38(4u64, f52(), 0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a607 = v0;
                let a608 = 5u64;
                if nat_eq(a607, a608) {
                    Ok(f38(2u64, f53(f50())?, f52(), f52()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a609 = v0;
            let a610 = 7u64;
            if nat_lt(a609, a610) {
                let a611 = v0;
                let a612 = 6u64;
                if nat_eq(a611, a612) {
                    Ok(f42(529u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a613 = v0;
                let a614 = 7u64;
                if nat_eq(a613, a614) {
                    Ok(f38(2u64, 1048576000u64, f31(), f63()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f38(0u64, f63(), f52(), f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a623 = v0;
                let a624 = 1u64;
                if nat_eq(a623, a624) {
                    Ok(f30(f52(), f54(f43())?))
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
                    Ok(f38(4u64, f52(), 0u64, f55(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a629 = v0;
                let a630 = 3u64;
                if nat_eq(a629, a630) {
                    Ok(f30(f51(), f56(f43())?))
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
                    Ok(f35(2u64, f51(), 0u64, 534u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a637 = v0;
                let a638 = 5u64;
                if nat_eq(a637, a638) {
                    Ok(f42(538u64))
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
                    Ok(f38(6u64, f51(), 0u64, f56(f43())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a643 = v0;
                let a644 = 7u64;
                if nat_eq(a643, a644) {
                    Ok(f57(6u64, f49(), 0u64, 0u64, f58()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a645 = v0;
    let a646 = 4u64;
    if nat_lt(a645, a646) {
        let a647 = v0;
        let a648 = 2u64;
        if nat_lt(a647, a648) {
            let a649 = v0;
            let a650 = 1u64;
            if nat_lt(a649, a650) {
                let a651 = v0;
                let a652 = 0u64;
                if nat_eq(a651, a652) {
                    f41(0u64, f48(), f58(), f48())
                } else {
                    f2()
                }
            } else {
                let a653 = v0;
                let a654 = 1u64;
                if nat_eq(a653, a654) {
                    f41(1u64, 1u64, f47(), f47())
                } else {
                    f2()
                }
            }
        } else {
            let a655 = v0;
            let a656 = 3u64;
            if nat_lt(a655, a656) {
                let a657 = v0;
                let a658 = 2u64;
                if nat_eq(a657, a658) {
                    f35(0u64, f59(), 0u64, 546u64)
                } else {
                    f2()
                }
            } else {
                let a659 = v0;
                let a660 = 3u64;
                if nat_eq(a659, a660) {
                    f38(2u64, f52(), f51(), f60())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a661 = v0;
        let a662 = 6u64;
        if nat_lt(a661, a662) {
            let a663 = v0;
            let a664 = 5u64;
            if nat_lt(a663, a664) {
                let a665 = v0;
                let a666 = 4u64;
                if nat_eq(a665, a666) {
                    f38(2u64, f60(), f51(), f60())
                } else {
                    f2()
                }
            } else {
                let a667 = v0;
                let a668 = 5u64;
                if nat_eq(a667, a668) {
                    f35(0u64, f49(), 0u64, 544u64)
                } else {
                    f2()
                }
            }
        } else {
            let a669 = v0;
            let a670 = 7u64;
            if nat_lt(a669, a670) {
                let a671 = v0;
                let a672 = 6u64;
                if nat_eq(a671, a672) {
                    f35(2u64, f60(), f61(), 544u64)
                } else {
                    f2()
                }
            } else {
                let a673 = v0;
                let a674 = 7u64;
                if nat_eq(a673, a674) {
                    f42(546u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a675 = v0;
    let a676 = 4u64;
    if nat_lt(a675, a676) {
        let a677 = v0;
        let a678 = 2u64;
        if nat_lt(a677, a678) {
            let a679 = v0;
            let a680 = 1u64;
            if nat_lt(a679, a680) {
                let a681 = v0;
                let a682 = 0u64;
                if nat_eq(a681, a682) {
                    f30(f60(), f61())
                } else {
                    f2()
                }
            } else {
                let a683 = v0;
                let a684 = 1u64;
                if nat_eq(a683, a684) {
                    f30(f49(), f62())
                } else {
                    f2()
                }
            }
        } else {
            let a685 = v0;
            let a686 = 3u64;
            if nat_lt(a685, a686) {
                let a687 = v0;
                let a688 = 2u64;
                if nat_eq(a687, a688) {
                    f41(0u64, f49(), 1u64, f49())
                } else {
                    f2()
                }
            } else {
                let a689 = v0;
                let a690 = 3u64;
                if nat_eq(a689, a690) {
                    f41(0u64, f43(), 1u64, f43())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a691 = v0;
        let a692 = 6u64;
        if nat_lt(a691, a692) {
            let a693 = v0;
            let a694 = 5u64;
            if nat_lt(a693, a694) {
                let a695 = v0;
                let a696 = 4u64;
                if nat_eq(a695, a696) {
                    f35(1u64, f49(), 8u64, 519u64)
                } else {
                    f2()
                }
            } else {
                let a697 = v0;
                let a698 = 5u64;
                if nat_eq(a697, a698) {
                    f35(0u64, f59(), 0u64, 561u64)
                } else {
                    f2()
                }
            }
        } else {
            let a699 = v0;
            let a700 = 7u64;
            if nat_lt(a699, a700) {
                let a701 = v0;
                let a702 = 6u64;
                if nat_eq(a701, a702) {
                    f35(0u64, f47(), 0u64, 561u64)
                } else {
                    f2()
                }
            } else {
                let a703 = v0;
                let a704 = 7u64;
                if nat_eq(a703, a704) {
                    f57(6u64, f62(), 0u64, 0u64, f58())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a705 = v0;
    let a706 = 4u64;
    if nat_lt(a705, a706) {
        let a707 = v0;
        let a708 = 2u64;
        if nat_lt(a707, a708) {
            let a709 = v0;
            let a710 = 1u64;
            if nat_lt(a709, a710) {
                let a711 = v0;
                let a712 = 0u64;
                if nat_eq(a711, a712) {
                    f41(3u64, f48(), f58(), f63())
                } else {
                    f2()
                }
            } else {
                let a713 = v0;
                let a714 = 1u64;
                if nat_eq(a713, a714) {
                    f41(4u64, f63(), 2u64, f63())
                } else {
                    f2()
                }
            }
        } else {
            let a715 = v0;
            let a716 = 3u64;
            if nat_lt(a715, a716) {
                let a717 = v0;
                let a718 = 2u64;
                if nat_eq(a717, a718) {
                    f35(0u64, f63(), 0u64, 557u64)
                } else {
                    f2()
                }
            } else {
                let a719 = v0;
                let a720 = 3u64;
                if nat_eq(a719, a720) {
                    f41(1u64, f48(), f58(), f48())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a721 = v0;
        let a722 = 6u64;
        if nat_lt(a721, a722) {
            let a723 = v0;
            let a724 = 5u64;
            if nat_lt(a723, a724) {
                let a725 = v0;
                let a726 = 4u64;
                if nat_eq(a725, a726) {
                    f42(558u64)
                } else {
                    f2()
                }
            } else {
                let a727 = v0;
                let a728 = 5u64;
                if nat_eq(a727, a728) {
                    f41(0u64, f48(), f58(), f48())
                } else {
                    f2()
                }
            }
        } else {
            let a729 = v0;
            let a730 = 7u64;
            if nat_lt(a729, a730) {
                let a731 = v0;
                let a732 = 6u64;
                if nat_eq(a731, a732) {
                    f41(2u64, f44(), 8u64, f63())
                } else {
                    f2()
                }
            } else {
                let a733 = v0;
                let a734 = 7u64;
                if nat_eq(a733, a734) {
                    f41(0u64, f63(), f62(), f63())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a735 = v0;
    let a736 = 4u64;
    if nat_lt(a735, a736) {
        let a737 = v0;
        let a738 = 2u64;
        if nat_lt(a737, a738) {
            let a739 = v0;
            let a740 = 1u64;
            if nat_lt(a739, a740) {
                let a741 = v0;
                let a742 = 0u64;
                if nat_eq(a741, a742) {
                    Ok(f38(6u64, f56(f63())?, 0u64, f56(f63())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a743 = v0;
                let a744 = 1u64;
                if nat_eq(a743, a744) {
                    Ok(f30(f48(), f64(f44())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a745 = v0;
            let a746 = 3u64;
            if nat_lt(a745, a746) {
                let a747 = v0;
                let a748 = 2u64;
                if nat_eq(a747, a748) {
                    Ok(f41(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a749 = v0;
                let a750 = 3u64;
                if nat_eq(a749, a750) {
                    Ok(f35(1u64, f44(), 2u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a751 = v0;
        let a752 = 6u64;
        if nat_lt(a751, a752) {
            let a753 = v0;
            let a754 = 5u64;
            if nat_lt(a753, a754) {
                let a755 = v0;
                let a756 = 4u64;
                if nat_eq(a755, a756) {
                    Ok(f30(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a757 = v0;
                let a758 = 5u64;
                if nat_eq(a757, a758) {
                    Ok(f35(2u64, f46(), f56(f43())?, 567u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a759 = v0;
            let a760 = 7u64;
            if nat_lt(a759, a760) {
                let a761 = v0;
                let a762 = 6u64;
                if nat_eq(a761, a762) {
                    Ok(f42(568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a763 = v0;
                let a764 = 7u64;
                if nat_eq(a763, a764) {
                    Ok(f30(f56(f43())?, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a765 = v0;
    let a766 = 3u64;
    if nat_lt(a765, a766) {
        let a767 = v0;
        let a768 = 1u64;
        if nat_lt(a767, a768) {
            let a769 = v0;
            let a770 = 0u64;
            if nat_eq(a769, a770) {
                Ok(f41(0u64, f43(), 1u64, f43()))
            } else {
                Ok(f2())
            }
        } else {
            let a771 = v0;
            let a772 = 2u64;
            if nat_lt(a771, a772) {
                let a773 = v0;
                let a774 = 1u64;
                if nat_eq(a773, a774) {
                    Ok(f35(1u64, f43(), 16u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a775 = v0;
                let a776 = 2u64;
                if nat_eq(a775, a776) {
                    Ok(f30(0u64, f65(f32())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a777 = v0;
        let a778 = 4u64;
        if nat_lt(a777, a778) {
            let a779 = v0;
            let a780 = 3u64;
            if nat_eq(a779, a780) {
                Ok(f35(0u64, f46(), 0u64, 3099u64))
            } else {
                Ok(f2())
            }
        } else {
            let a781 = v0;
            let a782 = 5u64;
            if nat_lt(a781, a782) {
                let a783 = v0;
                let a784 = 4u64;
                if nat_eq(a783, a784) {
                    Ok(f35(2u64, f46(), f66(), 3099u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a785 = v0;
                let a786 = 5u64;
                if nat_eq(a785, a786) {
                    Ok(f42(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a787 = v0;
    let a788 = 4u64;
    if nat_lt(a787, a788) {
        let a789 = v0;
        let a790 = 2u64;
        if nat_lt(a789, a790) {
            let a791 = v0;
            let a792 = 1u64;
            if nat_lt(a791, a792) {
                let a793 = v0;
                let a794 = 0u64;
                if nat_eq(a793, a794) {
                    Ok(f38(6u64, f56(f63())?, 0u64, f56(f63())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a795 = v0;
                let a796 = 1u64;
                if nat_eq(a795, a796) {
                    Ok(f30(f48(), f64(f44())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a797 = v0;
            let a798 = 3u64;
            if nat_lt(a797, a798) {
                let a799 = v0;
                let a800 = 2u64;
                if nat_eq(a799, a800) {
                    Ok(f41(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a801 = v0;
                let a802 = 3u64;
                if nat_eq(a801, a802) {
                    Ok(f35(1u64, f44(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a803 = v0;
        let a804 = 6u64;
        if nat_lt(a803, a804) {
            let a805 = v0;
            let a806 = 5u64;
            if nat_lt(a805, a806) {
                let a807 = v0;
                let a808 = 4u64;
                if nat_eq(a807, a808) {
                    Ok(f30(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a809 = v0;
                let a810 = 5u64;
                if nat_eq(a809, a810) {
                    Ok(f35(2u64, f46(), f56(f43())?, 567u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a811 = v0;
            let a812 = 7u64;
            if nat_lt(a811, a812) {
                let a813 = v0;
                let a814 = 6u64;
                if nat_eq(a813, a814) {
                    Ok(f42(568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a815 = v0;
                let a816 = 7u64;
                if nat_eq(a815, a816) {
                    Ok(f30(f56(f43())?, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a817 = v0;
    let a818 = 3u64;
    if nat_lt(a817, a818) {
        let a819 = v0;
        let a820 = 1u64;
        if nat_lt(a819, a820) {
            let a821 = v0;
            let a822 = 0u64;
            if nat_eq(a821, a822) {
                Ok(f41(0u64, f43(), 1u64, f43()))
            } else {
                Ok(f2())
            }
        } else {
            let a823 = v0;
            let a824 = 2u64;
            if nat_lt(a823, a824) {
                let a825 = v0;
                let a826 = 1u64;
                if nat_eq(a825, a826) {
                    Ok(f35(1u64, f43(), 32u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a827 = v0;
                let a828 = 2u64;
                if nat_eq(a827, a828) {
                    Ok(f30(0u64, f65(f32())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a829 = v0;
        let a830 = 4u64;
        if nat_lt(a829, a830) {
            let a831 = v0;
            let a832 = 3u64;
            if nat_eq(a831, a832) {
                Ok(f35(0u64, f46(), 0u64, 3091u64))
            } else {
                Ok(f2())
            }
        } else {
            let a833 = v0;
            let a834 = 5u64;
            if nat_lt(a833, a834) {
                let a835 = v0;
                let a836 = 4u64;
                if nat_eq(a835, a836) {
                    Ok(f35(2u64, f46(), f66(), 3091u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a837 = v0;
                let a838 = 5u64;
                if nat_eq(a837, a838) {
                    Ok(f42(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f30(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f31() -> u64 {
    4294967348u64
}

pub fn f32() -> u64 {
    4294967301u64
}

pub fn f33() -> u64 {
    4294967302u64
}

pub fn f34() -> u64 {
    4294967347u64
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f36() -> u64 {
    4294967298u64
}

pub fn f37() -> u64 {
    4294967343u64
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f39(v0: u64) -> R<u64> {
    let a842 = 12884901888u64;
    let a843 = {
        let a839 = v0;
        let a840 = 4294967296u64;
        let a841 = 0u64;
        nat_rem(a839, a840, a841)
    };
    nat_add(a842, a843)
}

pub fn f40() -> u64 {
    4294967337u64
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f42(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f43() -> u64 {
    4294967303u64
}

pub fn f44() -> u64 {
    4294967304u64
}

pub fn f45() -> u64 {
    4294967354u64
}

pub fn f46() -> u64 {
    4294967315u64
}

pub fn f47() -> u64 {
    4294967313u64
}

pub fn f48() -> u64 {
    4294967314u64
}

pub fn f49() -> u64 {
    4294967305u64
}

pub fn f50() -> u64 {
    4294967307u64
}

pub fn f51() -> u64 {
    4294967308u64
}

pub fn f52() -> u64 {
    4294967309u64
}

pub fn f53(v0: u64) -> R<u64> {
    let a847 = 17179869184u64;
    let a848 = {
        let a844 = v0;
        let a845 = 4294967296u64;
        let a846 = 0u64;
        nat_rem(a844, a845, a846)
    };
    nat_add(a847, a848)
}

pub fn f54(v0: u64) -> R<u64> {
    let a852 = 8589959168u64;
    let a853 = {
        let a849 = v0;
        let a850 = 4294967296u64;
        let a851 = 0u64;
        nat_rem(a849, a850, a851)
    };
    nat_add(a852, a853)
}

pub fn f55(v0: u64) -> R<u64> {
    let a857 = 8589967360u64;
    let a858 = {
        let a854 = v0;
        let a855 = 4294967296u64;
        let a856 = 0u64;
        nat_rem(a854, a855, a856)
    };
    nat_add(a857, a858)
}

pub fn f56(v0: u64) -> R<u64> {
    let a862 = 8589950976u64;
    let a863 = {
        let a859 = v0;
        let a860 = 4294967296u64;
        let a861 = 0u64;
        nat_rem(a859, a860, a861)
    };
    nat_add(a862, a863)
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f58() -> u64 {
    4294967342u64
}

pub fn f59() -> u64 {
    4294967297u64
}

pub fn f60() -> u64 {
    4294967310u64
}

pub fn f61() -> u64 {
    4294967311u64
}

pub fn f62() -> u64 {
    4294967312u64
}

pub fn f63() -> u64 {
    4294967338u64
}

pub fn f64(v0: u64) -> R<u64> {
    let a867 = 8589975552u64;
    let a868 = {
        let a864 = v0;
        let a865 = 4294967296u64;
        let a866 = 0u64;
        nat_rem(a864, a865, a866)
    };
    nat_add(a867, a868)
}

pub fn f65(v0: u64) -> R<u64> {
    let a872 = 8589984768u64;
    let a873 = {
        let a869 = v0;
        let a870 = 4294967296u64;
        let a871 = 0u64;
        nat_rem(a869, a870, a871)
    };
    nat_add(a872, a873)
}

pub fn f66() -> u64 {
    4294967300u64
}
