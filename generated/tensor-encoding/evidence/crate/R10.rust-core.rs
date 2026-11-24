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
    let a35 = 2u64;
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
            let a44 = 1u64;
            if nat_eq(a43, a44) {
                f11({
                    let a45 = v0;
                    let a46 = 8u64;
                    let a47 = 0u64;
                    nat_rem(a45, a46, a47)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a48 = v1;
        let a49 = 3u64;
        if nat_lt(a48, a49) {
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
        } else {
            let a55 = v1;
            let a56 = 3u64;
            if nat_eq(a55, a56) {
                Ok(f13({
                    let a57 = v0;
                    let a58 = 8u64;
                    let a59 = 0u64;
                    nat_rem(a57, a58, a59)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a60 = v0;
        let a61 = 8u64;
        let a62 = 0u64;
        nat_quot(a60, a61, a62)
    };
    let a63 = v1;
    let a64 = 1u64;
    if nat_lt(a63, a64) {
        let a65 = v1;
        let a66 = 0u64;
        if nat_eq(a65, a66) {
            f14({
                let a67 = v0;
                let a68 = 8u64;
                let a69 = 0u64;
                nat_rem(a67, a68, a69)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a70 = v1;
        let a71 = 2u64;
        if nat_lt(a70, a71) {
            let a72 = v1;
            let a73 = 1u64;
            if nat_eq(a72, a73) {
                f15({
                    let a74 = v0;
                    let a75 = 8u64;
                    let a76 = 0u64;
                    nat_rem(a74, a75, a76)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a77 = v1;
            let a78 = 2u64;
            if nat_eq(a77, a78) {
                Ok(f16({
                    let a79 = v0;
                    let a80 = 8u64;
                    let a81 = 0u64;
                    nat_rem(a79, a80, a81)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a82 = v0;
        let a83 = 8u64;
        let a84 = 0u64;
        nat_quot(a82, a83, a84)
    };
    let a85 = v1;
    let a86 = 2u64;
    if nat_lt(a85, a86) {
        let a87 = v1;
        let a88 = 1u64;
        if nat_lt(a87, a88) {
            let a89 = v1;
            let a90 = 0u64;
            if nat_eq(a89, a90) {
                f10({
                    let a91 = v0;
                    let a92 = 8u64;
                    let a93 = 0u64;
                    nat_rem(a91, a92, a93)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a94 = v1;
            let a95 = 1u64;
            if nat_eq(a94, a95) {
                f11({
                    let a96 = v0;
                    let a97 = 8u64;
                    let a98 = 0u64;
                    nat_rem(a96, a97, a98)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a99 = v1;
        let a100 = 3u64;
        if nat_lt(a99, a100) {
            let a101 = v1;
            let a102 = 2u64;
            if nat_eq(a101, a102) {
                f17({
                    let a103 = v0;
                    let a104 = 8u64;
                    let a105 = 0u64;
                    nat_rem(a103, a104, a105)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a106 = v1;
            let a107 = 3u64;
            if nat_eq(a106, a107) {
                Ok(f13({
                    let a108 = v0;
                    let a109 = 8u64;
                    let a110 = 0u64;
                    nat_rem(a108, a109, a110)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a111 = v0;
        let a112 = 8u64;
        let a113 = 0u64;
        nat_quot(a111, a112, a113)
    };
    let a114 = v1;
    let a115 = 1u64;
    if nat_lt(a114, a115) {
        let a116 = v1;
        let a117 = 0u64;
        if nat_eq(a116, a117) {
            f18({
                let a118 = v0;
                let a119 = 8u64;
                let a120 = 0u64;
                nat_rem(a118, a119, a120)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a121 = v1;
        let a122 = 2u64;
        if nat_lt(a121, a122) {
            let a123 = v1;
            let a124 = 1u64;
            if nat_eq(a123, a124) {
                f19({
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
            if nat_eq(a128, a129) {
                Ok(f20({
                    let a130 = v0;
                    let a131 = 8u64;
                    let a132 = 0u64;
                    nat_rem(a130, a131, a132)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a133 = v0;
        let a134 = 8u64;
        let a135 = 0u64;
        nat_quot(a133, a134, a135)
    };
    let a136 = v1;
    let a137 = 2u64;
    if nat_lt(a136, a137) {
        let a138 = v1;
        let a139 = 1u64;
        if nat_lt(a138, a139) {
            let a140 = v1;
            let a141 = 0u64;
            if nat_eq(a140, a141) {
                f10({
                    let a142 = v0;
                    let a143 = 8u64;
                    let a144 = 0u64;
                    nat_rem(a142, a143, a144)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a145 = v1;
            let a146 = 1u64;
            if nat_eq(a145, a146) {
                f11({
                    let a147 = v0;
                    let a148 = 8u64;
                    let a149 = 0u64;
                    nat_rem(a147, a148, a149)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a150 = v1;
        let a151 = 3u64;
        if nat_lt(a150, a151) {
            let a152 = v1;
            let a153 = 2u64;
            if nat_eq(a152, a153) {
                f21({
                    let a154 = v0;
                    let a155 = 8u64;
                    let a156 = 0u64;
                    nat_rem(a154, a155, a156)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a157 = v1;
            let a158 = 3u64;
            if nat_eq(a157, a158) {
                Ok(f13({
                    let a159 = v0;
                    let a160 = 8u64;
                    let a161 = 0u64;
                    nat_rem(a159, a160, a161)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a162 = v0;
        let a163 = 8u64;
        let a164 = 0u64;
        nat_quot(a162, a163, a164)
    };
    let a165 = v1;
    let a166 = 1u64;
    if nat_lt(a165, a166) {
        let a167 = v1;
        let a168 = 0u64;
        if nat_eq(a167, a168) {
            f22({
                let a169 = v0;
                let a170 = 8u64;
                let a171 = 0u64;
                nat_rem(a169, a170, a171)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a172 = v1;
        let a173 = 2u64;
        if nat_lt(a172, a173) {
            let a174 = v1;
            let a175 = 1u64;
            if nat_eq(a174, a175) {
                Ok(f23({
                    let a176 = v0;
                    let a177 = 8u64;
                    let a178 = 0u64;
                    nat_rem(a176, a177, a178)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a179 = v1;
            let a180 = 2u64;
            if nat_eq(a179, a180) {
                Ok(f24({
                    let a181 = v0;
                    let a182 = 8u64;
                    let a183 = 0u64;
                    nat_rem(a181, a182, a183)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a184 = v0;
        let a185 = 8u64;
        let a186 = 0u64;
        nat_quot(a184, a185, a186)
    };
    let a187 = v1;
    let a188 = 2u64;
    if nat_lt(a187, a188) {
        let a189 = v1;
        let a190 = 1u64;
        if nat_lt(a189, a190) {
            let a191 = v1;
            let a192 = 0u64;
            if nat_eq(a191, a192) {
                Ok(f25({
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
                f26({
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
                f27({
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
            let a209 = 4u64;
            if nat_lt(a208, a209) {
                let a210 = v1;
                let a211 = 3u64;
                if nat_eq(a210, a211) {
                    Ok(f28({
                        let a212 = v0;
                        let a213 = 8u64;
                        let a214 = 0u64;
                        nat_rem(a212, a213, a214)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a215 = v1;
                let a216 = 4u64;
                if nat_eq(a215, a216) {
                    Ok(f29({
                        let a217 = v0;
                        let a218 = 8u64;
                        let a219 = 0u64;
                        nat_rem(a217, a218, a219)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a220 = v0;
        let a221 = 8u64;
        let a222 = 0u64;
        nat_quot(a220, a221, a222)
    };
    let a223 = v1;
    let a224 = 2u64;
    if nat_lt(a223, a224) {
        let a225 = v1;
        let a226 = 1u64;
        if nat_lt(a225, a226) {
            let a227 = v1;
            let a228 = 0u64;
            if nat_eq(a227, a228) {
                f30({
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
                f31({
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
                f32({
                    let a241 = v0;
                    let a242 = 8u64;
                    let a243 = 0u64;
                    nat_rem(a241, a242, a243)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a244 = v1;
            let a245 = 3u64;
            if nat_eq(a244, a245) {
                f33({
                    let a246 = v0;
                    let a247 = 8u64;
                    let a248 = 0u64;
                    nat_rem(a246, a247, a248)
                })
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a249 = v0;
    let a250 = 4u64;
    if nat_lt(a249, a250) {
        let a251 = v0;
        let a252 = 2u64;
        if nat_lt(a251, a252) {
            let a253 = v0;
            let a254 = 1u64;
            if nat_lt(a253, a254) {
                let a255 = v0;
                let a256 = 0u64;
                if nat_eq(a255, a256) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a257 = v0;
                let a258 = 1u64;
                if nat_eq(a257, a258) {
                    Ok(f34(0u64, f36()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a259 = v0;
            let a260 = 3u64;
            if nat_lt(a259, a260) {
                let a261 = v0;
                let a262 = 2u64;
                if nat_eq(a261, a262) {
                    Ok(f34(0u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a263 = v0;
                let a264 = 3u64;
                if nat_eq(a263, a264) {
                    Ok(f34(0u64, f38()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a265 = v0;
        let a266 = 6u64;
        if nat_lt(a265, a266) {
            let a267 = v0;
            let a268 = 5u64;
            if nat_lt(a267, a268) {
                let a269 = v0;
                let a270 = 4u64;
                if nat_eq(a269, a270) {
                    Ok(f39(0u64, f40(), 1u64, 2055u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a271 = v0;
                let a272 = 5u64;
                if nat_eq(a271, a272) {
                    Ok(f34(f41(f35())?, f42()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a273 = v0;
            let a274 = 7u64;
            if nat_lt(a273, a274) {
                let a275 = v0;
                let a276 = 6u64;
                if nat_eq(a275, a276) {
                    Ok(f43(2056u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a277 = v0;
                let a278 = 7u64;
                if nat_eq(a277, a278) {
                    Ok(f34(f44(f35())?, f42()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a279 = v0;
    let a280 = 4u64;
    if nat_lt(a279, a280) {
        let a281 = v0;
        let a282 = 2u64;
        if nat_lt(a281, a282) {
            let a283 = v0;
            let a284 = 1u64;
            if nat_lt(a283, a284) {
                let a285 = v0;
                let a286 = 0u64;
                if nat_eq(a285, a286) {
                    Ok(f34(0u64, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a287 = v0;
                let a288 = 1u64;
                if nat_eq(a287, a288) {
                    Ok(f46(4u64, 0u64, f42(), 0u64, f47(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a289 = v0;
            let a290 = 3u64;
            if nat_lt(a289, a290) {
                let a291 = v0;
                let a292 = 2u64;
                if nat_eq(a291, a292) {
                    Ok(f49(0u64, f48(), f45(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a293 = v0;
                let a294 = 3u64;
                if nat_eq(a293, a294) {
                    Ok(f50(5u64, f51(f48())?, 0u64, 0u64, f52()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a295 = v0;
        let a296 = 6u64;
        if nat_lt(a295, a296) {
            let a297 = v0;
            let a298 = 5u64;
            if nat_lt(a297, a298) {
                let a299 = v0;
                let a300 = 4u64;
                if nat_eq(a299, a300) {
                    Ok(f53(f52(), f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a301 = v0;
                let a302 = 5u64;
                if nat_eq(a301, a302) {
                    Ok(f54(2u64, f55(f38())?, f56(f38())?, f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a303 = v0;
            let a304 = 7u64;
            if nat_lt(a303, a304) {
                let a305 = v0;
                let a306 = 6u64;
                if nat_eq(a305, a306) {
                    Ok(f54(2u64, f57(), f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a307 = v0;
                let a308 = 7u64;
                if nat_eq(a307, a308) {
                    Ok(f54(0u64, f36(), f57(), f36()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a309 = v0;
    let a310 = 4u64;
    if nat_lt(a309, a310) {
        let a311 = v0;
        let a312 = 2u64;
        if nat_lt(a311, a312) {
            let a313 = v0;
            let a314 = 1u64;
            if nat_lt(a313, a314) {
                let a315 = v0;
                let a316 = 0u64;
                if nat_eq(a315, a316) {
                    Ok(f54(2u64, f55(f38())?, f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a317 = v0;
                let a318 = 1u64;
                if nat_eq(a317, a318) {
                    Ok(f54(2u64, f57(), f52(), f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a319 = v0;
            let a320 = 3u64;
            if nat_lt(a319, a320) {
                let a321 = v0;
                let a322 = 2u64;
                if nat_eq(a321, a322) {
                    Ok(f54(0u64, f37(), f57(), f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a323 = v0;
                let a324 = 3u64;
                if nat_eq(a323, a324) {
                    Ok(f49(0u64, f45(), 1u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a325 = v0;
        let a326 = 6u64;
        if nat_lt(a325, a326) {
            let a327 = v0;
            let a328 = 5u64;
            if nat_lt(a327, a328) {
                let a329 = v0;
                let a330 = 4u64;
                if nat_eq(a329, a330) {
                    Ok(f49(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            } else {
                let a331 = v0;
                let a332 = 5u64;
                if nat_eq(a331, a332) {
                    Ok(f39(1u64, f45(), 4u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a333 = v0;
            let a334 = 7u64;
            if nat_lt(a333, a334) {
                let a335 = v0;
                let a336 = 6u64;
                if nat_eq(a335, a336) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a337 = v0;
                let a338 = 7u64;
                if nat_eq(a337, a338) {
                    Ok(f39(1u64, f35(), 8u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a339 = v0;
    let a340 = 0u64;
    if nat_eq(a339, a340) {
        f58(f59())
    } else {
        f2()
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a341 = v0;
    let a342 = 4u64;
    if nat_lt(a341, a342) {
        let a343 = v0;
        let a344 = 2u64;
        if nat_lt(a343, a344) {
            let a345 = v0;
            let a346 = 1u64;
            if nat_lt(a345, a346) {
                let a347 = v0;
                let a348 = 0u64;
                if nat_eq(a347, a348) {
                    Ok(f39(2u64, 0u64, f60(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a349 = v0;
                let a350 = 1u64;
                if nat_eq(a349, a350) {
                    Ok(f43(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a351 = v0;
            let a352 = 3u64;
            if nat_lt(a351, a352) {
                let a353 = v0;
                let a354 = 2u64;
                if nat_eq(a353, a354) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a355 = v0;
                let a356 = 3u64;
                if nat_eq(a355, a356) {
                    Ok(f39(0u64, f61(f35())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a357 = v0;
        let a358 = 6u64;
        if nat_lt(a357, a358) {
            let a359 = v0;
            let a360 = 5u64;
            if nat_lt(a359, a360) {
                let a361 = v0;
                let a362 = 4u64;
                if nat_eq(a361, a362) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a363 = v0;
                let a364 = 5u64;
                if nat_eq(a363, a364) {
                    Ok(f39(1u64, f35(), 8u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a365 = v0;
            let a366 = 7u64;
            if nat_lt(a365, a366) {
                let a367 = v0;
                let a368 = 6u64;
                if nat_eq(a367, a368) {
                    Ok(f43(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a369 = v0;
                let a370 = 7u64;
                if nat_eq(a369, a370) {
                    Ok(f54(3u64, 1065353216u64, f60(), f62()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f34(f60(), f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a379 = v0;
                let a380 = 1u64;
                if nat_eq(a379, a380) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a381 = v0;
            let a382 = 3u64;
            if nat_lt(a381, a382) {
                let a383 = v0;
                let a384 = 2u64;
                if nat_eq(a383, a384) {
                    Ok(f39(0u64, f64(), 0u64, 2572u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a385 = v0;
                let a386 = 3u64;
                if nat_eq(a385, a386) {
                    Ok(f39(0u64, f61(f35())?, 1u64, 2575u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a387 = v0;
        let a388 = 6u64;
        if nat_lt(a387, a388) {
            let a389 = v0;
            let a390 = 5u64;
            if nat_lt(a389, a390) {
                let a391 = v0;
                let a392 = 4u64;
                if nat_eq(a391, a392) {
                    Ok(f34(2574u64, f65()))
                } else {
                    Ok(f2())
                }
            } else {
                let a393 = v0;
                let a394 = 5u64;
                if nat_eq(a393, a394) {
                    Ok(f43(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a395 = v0;
            let a396 = 7u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 6u64;
                if nat_eq(a397, a398) {
                    Ok(f34(f42(), f44(f35())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a399 = v0;
                let a400 = 7u64;
                if nat_eq(a399, a400) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a401 = v0;
    let a402 = 4u64;
    if nat_lt(a401, a402) {
        let a403 = v0;
        let a404 = 2u64;
        if nat_lt(a403, a404) {
            let a405 = v0;
            let a406 = 1u64;
            if nat_lt(a405, a406) {
                let a407 = v0;
                let a408 = 0u64;
                if nat_eq(a407, a408) {
                    f39(1u64, f35(), 8u64, 2570u64)
                } else {
                    f2()
                }
            } else {
                let a409 = v0;
                let a410 = 1u64;
                if nat_eq(a409, a410) {
                    f34(1u64, f40())
                } else {
                    f2()
                }
            }
        } else {
            let a411 = v0;
            let a412 = 3u64;
            if nat_lt(a411, a412) {
                let a413 = v0;
                let a414 = 2u64;
                if nat_eq(a413, a414) {
                    f34(2580u64, f59())
                } else {
                    f2()
                }
            } else {
                let a415 = v0;
                let a416 = 3u64;
                if nat_eq(a415, a416) {
                    f43(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a417 = v0;
        let a418 = 6u64;
        if nat_lt(a417, a418) {
            let a419 = v0;
            let a420 = 5u64;
            if nat_lt(a419, a420) {
                let a421 = v0;
                let a422 = 4u64;
                if nat_eq(a421, a422) {
                    f39(2u64, 0u64, f37(), 2582u64)
                } else {
                    f2()
                }
            } else {
                let a423 = v0;
                let a424 = 5u64;
                if nat_eq(a423, a424) {
                    f43(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a425 = v0;
            let a426 = 7u64;
            if nat_lt(a425, a426) {
                let a427 = v0;
                let a428 = 6u64;
                if nat_eq(a427, a428) {
                    f54(3u64, f36(), f37(), f60())
                } else {
                    f2()
                }
            } else {
                let a429 = v0;
                let a430 = 7u64;
                if nat_eq(a429, a430) {
                    f43(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a431 = v0;
    let a432 = 4u64;
    if nat_lt(a431, a432) {
        let a433 = v0;
        let a434 = 2u64;
        if nat_lt(a433, a434) {
            let a435 = v0;
            let a436 = 1u64;
            if nat_lt(a435, a436) {
                let a437 = v0;
                let a438 = 0u64;
                if nat_eq(a437, a438) {
                    Ok(f54(2u64, f55(f38())?, f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a439 = v0;
                let a440 = 1u64;
                if nat_eq(a439, a440) {
                    Ok(f54(2u64, f57(), f52(), f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a441 = v0;
            let a442 = 3u64;
            if nat_lt(a441, a442) {
                let a443 = v0;
                let a444 = 2u64;
                if nat_eq(a443, a444) {
                    Ok(f54(0u64, f37(), f57(), f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a445 = v0;
                let a446 = 3u64;
                if nat_eq(a445, a446) {
                    Ok(f49(0u64, f45(), 1u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a447 = v0;
        let a448 = 6u64;
        if nat_lt(a447, a448) {
            let a449 = v0;
            let a450 = 5u64;
            if nat_lt(a449, a450) {
                let a451 = v0;
                let a452 = 4u64;
                if nat_eq(a451, a452) {
                    Ok(f49(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            } else {
                let a453 = v0;
                let a454 = 5u64;
                if nat_eq(a453, a454) {
                    Ok(f39(1u64, f45(), 8u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a455 = v0;
            let a456 = 7u64;
            if nat_lt(a455, a456) {
                let a457 = v0;
                let a458 = 6u64;
                if nat_eq(a457, a458) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a459 = v0;
                let a460 = 7u64;
                if nat_eq(a459, a460) {
                    Ok(f39(1u64, f35(), 2u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a461 = v0;
    let a462 = 4u64;
    if nat_lt(a461, a462) {
        let a463 = v0;
        let a464 = 2u64;
        if nat_lt(a463, a464) {
            let a465 = v0;
            let a466 = 1u64;
            if nat_lt(a465, a466) {
                let a467 = v0;
                let a468 = 0u64;
                if nat_eq(a467, a468) {
                    Ok(f39(2u64, 0u64, f60(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a469 = v0;
                let a470 = 1u64;
                if nat_eq(a469, a470) {
                    Ok(f43(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a471 = v0;
            let a472 = 3u64;
            if nat_lt(a471, a472) {
                let a473 = v0;
                let a474 = 2u64;
                if nat_eq(a473, a474) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a475 = v0;
                let a476 = 3u64;
                if nat_eq(a475, a476) {
                    Ok(f39(0u64, f61(f35())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a477 = v0;
        let a478 = 6u64;
        if nat_lt(a477, a478) {
            let a479 = v0;
            let a480 = 5u64;
            if nat_lt(a479, a480) {
                let a481 = v0;
                let a482 = 4u64;
                if nat_eq(a481, a482) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a483 = v0;
                let a484 = 5u64;
                if nat_eq(a483, a484) {
                    Ok(f39(1u64, f35(), 2u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a485 = v0;
            let a486 = 7u64;
            if nat_lt(a485, a486) {
                let a487 = v0;
                let a488 = 6u64;
                if nat_eq(a487, a488) {
                    Ok(f43(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a489 = v0;
                let a490 = 7u64;
                if nat_eq(a489, a490) {
                    Ok(f54(3u64, 1065353216u64, f60(), f62()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a491 = v0;
    let a492 = 4u64;
    if nat_lt(a491, a492) {
        let a493 = v0;
        let a494 = 2u64;
        if nat_lt(a493, a494) {
            let a495 = v0;
            let a496 = 1u64;
            if nat_lt(a495, a496) {
                let a497 = v0;
                let a498 = 0u64;
                if nat_eq(a497, a498) {
                    Ok(f34(f60(), f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a499 = v0;
                let a500 = 1u64;
                if nat_eq(a499, a500) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a501 = v0;
            let a502 = 3u64;
            if nat_lt(a501, a502) {
                let a503 = v0;
                let a504 = 2u64;
                if nat_eq(a503, a504) {
                    Ok(f39(0u64, f61(f35())?, 1u64, 2574u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a505 = v0;
                let a506 = 3u64;
                if nat_eq(a505, a506) {
                    Ok(f34(2573u64, f65()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a507 = v0;
        let a508 = 6u64;
        if nat_lt(a507, a508) {
            let a509 = v0;
            let a510 = 5u64;
            if nat_lt(a509, a510) {
                let a511 = v0;
                let a512 = 4u64;
                if nat_eq(a511, a512) {
                    Ok(f43(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a513 = v0;
                let a514 = 5u64;
                if nat_eq(a513, a514) {
                    Ok(f34(f42(), f44(f35())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a515 = v0;
            let a516 = 7u64;
            if nat_lt(a515, a516) {
                let a517 = v0;
                let a518 = 6u64;
                if nat_eq(a517, a518) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a519 = v0;
                let a520 = 7u64;
                if nat_eq(a519, a520) {
                    Ok(f39(1u64, f35(), 2u64, 2570u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a521 = v0;
    let a522 = 3u64;
    if nat_lt(a521, a522) {
        let a523 = v0;
        let a524 = 1u64;
        if nat_lt(a523, a524) {
            let a525 = v0;
            let a526 = 0u64;
            if nat_eq(a525, a526) {
                f34(1u64, f40())
            } else {
                f2()
            }
        } else {
            let a527 = v0;
            let a528 = 2u64;
            if nat_lt(a527, a528) {
                let a529 = v0;
                let a530 = 1u64;
                if nat_eq(a529, a530) {
                    f34(2579u64, f59())
                } else {
                    f2()
                }
            } else {
                let a531 = v0;
                let a532 = 2u64;
                if nat_eq(a531, a532) {
                    f43(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a533 = v0;
        let a534 = 5u64;
        if nat_lt(a533, a534) {
            let a535 = v0;
            let a536 = 4u64;
            if nat_lt(a535, a536) {
                let a537 = v0;
                let a538 = 3u64;
                if nat_eq(a537, a538) {
                    f39(2u64, 0u64, f37(), 2581u64)
                } else {
                    f2()
                }
            } else {
                let a539 = v0;
                let a540 = 4u64;
                if nat_eq(a539, a540) {
                    f43(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a541 = v0;
            let a542 = 6u64;
            if nat_lt(a541, a542) {
                let a543 = v0;
                let a544 = 5u64;
                if nat_eq(a543, a544) {
                    f54(3u64, f36(), f37(), f60())
                } else {
                    f2()
                }
            } else {
                let a545 = v0;
                let a546 = 6u64;
                if nat_eq(a545, a546) {
                    f43(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a547 = v0;
    let a548 = 4u64;
    if nat_lt(a547, a548) {
        let a549 = v0;
        let a550 = 2u64;
        if nat_lt(a549, a550) {
            let a551 = v0;
            let a552 = 1u64;
            if nat_lt(a551, a552) {
                let a553 = v0;
                let a554 = 0u64;
                if nat_eq(a553, a554) {
                    Ok(f54(2u64, f55(f38())?, f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a555 = v0;
                let a556 = 1u64;
                if nat_eq(a555, a556) {
                    Ok(f54(2u64, f57(), f52(), f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a557 = v0;
            let a558 = 3u64;
            if nat_lt(a557, a558) {
                let a559 = v0;
                let a560 = 2u64;
                if nat_eq(a559, a560) {
                    Ok(f54(0u64, f37(), f57(), f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a561 = v0;
                let a562 = 3u64;
                if nat_eq(a561, a562) {
                    Ok(f49(0u64, f45(), 1u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a563 = v0;
        let a564 = 6u64;
        if nat_lt(a563, a564) {
            let a565 = v0;
            let a566 = 5u64;
            if nat_lt(a565, a566) {
                let a567 = v0;
                let a568 = 4u64;
                if nat_eq(a567, a568) {
                    Ok(f49(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            } else {
                let a569 = v0;
                let a570 = 5u64;
                if nat_eq(a569, a570) {
                    Ok(f39(1u64, f45(), 8u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a571 = v0;
            let a572 = 7u64;
            if nat_lt(a571, a572) {
                let a573 = v0;
                let a574 = 6u64;
                if nat_eq(a573, a574) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a575 = v0;
                let a576 = 7u64;
                if nat_eq(a575, a576) {
                    Ok(f39(1u64, f35(), 4u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a577 = v0;
    let a578 = 4u64;
    if nat_lt(a577, a578) {
        let a579 = v0;
        let a580 = 2u64;
        if nat_lt(a579, a580) {
            let a581 = v0;
            let a582 = 1u64;
            if nat_lt(a581, a582) {
                let a583 = v0;
                let a584 = 0u64;
                if nat_eq(a583, a584) {
                    Ok(f39(2u64, 0u64, f60(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a585 = v0;
                let a586 = 1u64;
                if nat_eq(a585, a586) {
                    Ok(f43(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a587 = v0;
            let a588 = 3u64;
            if nat_lt(a587, a588) {
                let a589 = v0;
                let a590 = 2u64;
                if nat_eq(a589, a590) {
                    Ok(f54(3u64, 1065353216u64, f60(), f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a591 = v0;
                let a592 = 3u64;
                if nat_eq(a591, a592) {
                    Ok(f34(f60(), f63()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a593 = v0;
        let a594 = 6u64;
        if nat_lt(a593, a594) {
            let a595 = v0;
            let a596 = 5u64;
            if nat_lt(a595, a596) {
                let a597 = v0;
                let a598 = 4u64;
                if nat_eq(a597, a598) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a599 = v0;
                let a600 = 5u64;
                if nat_eq(a599, a600) {
                    Ok(f34(2567u64, f65()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a601 = v0;
            let a602 = 7u64;
            if nat_lt(a601, a602) {
                let a603 = v0;
                let a604 = 6u64;
                if nat_eq(a603, a604) {
                    Ok(f43(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a605 = v0;
                let a606 = 7u64;
                if nat_eq(a605, a606) {
                    Ok(f34(f42(), f44(f35())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a607 = v0;
    let a608 = 4u64;
    if nat_lt(a607, a608) {
        let a609 = v0;
        let a610 = 2u64;
        if nat_lt(a609, a610) {
            let a611 = v0;
            let a612 = 1u64;
            if nat_lt(a611, a612) {
                let a613 = v0;
                let a614 = 0u64;
                if nat_eq(a613, a614) {
                    f49(0u64, f35(), 1u64, f35())
                } else {
                    f2()
                }
            } else {
                let a615 = v0;
                let a616 = 1u64;
                if nat_eq(a615, a616) {
                    f39(1u64, f35(), 4u64, 2565u64)
                } else {
                    f2()
                }
            }
        } else {
            let a617 = v0;
            let a618 = 3u64;
            if nat_lt(a617, a618) {
                let a619 = v0;
                let a620 = 2u64;
                if nat_eq(a619, a620) {
                    f34(1u64, f40())
                } else {
                    f2()
                }
            } else {
                let a621 = v0;
                let a622 = 3u64;
                if nat_eq(a621, a622) {
                    f34(2573u64, f59())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a623 = v0;
        let a624 = 6u64;
        if nat_lt(a623, a624) {
            let a625 = v0;
            let a626 = 5u64;
            if nat_lt(a625, a626) {
                let a627 = v0;
                let a628 = 4u64;
                if nat_eq(a627, a628) {
                    f43(2048u64)
                } else {
                    f2()
                }
            } else {
                let a629 = v0;
                let a630 = 5u64;
                if nat_eq(a629, a630) {
                    f39(2u64, 0u64, f37(), 2575u64)
                } else {
                    f2()
                }
            }
        } else {
            let a631 = v0;
            let a632 = 7u64;
            if nat_lt(a631, a632) {
                let a633 = v0;
                let a634 = 6u64;
                if nat_eq(a633, a634) {
                    f43(3072u64)
                } else {
                    f2()
                }
            } else {
                let a635 = v0;
                let a636 = 7u64;
                if nat_eq(a635, a636) {
                    f54(3u64, f36(), f37(), f60())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a637 = v0;
    let a638 = 0u64;
    if nat_eq(a637, a638) {
        f43(3072u64)
    } else {
        f2()
    }
}

pub fn f25(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a639 = v0;
    let a640 = 4u64;
    if nat_lt(a639, a640) {
        let a641 = v0;
        let a642 = 2u64;
        if nat_lt(a641, a642) {
            let a643 = v0;
            let a644 = 1u64;
            if nat_lt(a643, a644) {
                let a645 = v0;
                let a646 = 0u64;
                if nat_eq(a645, a646) {
                    f34(0u64, f35())
                } else {
                    f2()
                }
            } else {
                let a647 = v0;
                let a648 = 1u64;
                if nat_eq(a647, a648) {
                    f34(0u64, f36())
                } else {
                    f2()
                }
            }
        } else {
            let a649 = v0;
            let a650 = 3u64;
            if nat_lt(a649, a650) {
                let a651 = v0;
                let a652 = 2u64;
                if nat_eq(a651, a652) {
                    f34(0u64, f37())
                } else {
                    f2()
                }
            } else {
                let a653 = v0;
                let a654 = 3u64;
                if nat_eq(a653, a654) {
                    f34(0u64, f38())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a655 = v0;
        let a656 = 6u64;
        if nat_lt(a655, a656) {
            let a657 = v0;
            let a658 = 5u64;
            if nat_lt(a657, a658) {
                let a659 = v0;
                let a660 = 4u64;
                if nat_eq(a659, a660) {
                    f39(0u64, f64(), 0u64, 2058u64)
                } else {
                    f2()
                }
            } else {
                let a661 = v0;
                let a662 = 5u64;
                if nat_eq(a661, a662) {
                    f39(0u64, f35(), 0u64, 2056u64)
                } else {
                    f2()
                }
            }
        } else {
            let a663 = v0;
            let a664 = 7u64;
            if nat_lt(a663, a664) {
                let a665 = v0;
                let a666 = 6u64;
                if nat_eq(a665, a666) {
                    f49(4u64, f66(), 2u64, f67())
                } else {
                    f2()
                }
            } else {
                let a667 = v0;
                let a668 = 7u64;
                if nat_eq(a667, a668) {
                    f43(2059u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a669 = v0;
    let a670 = 4u64;
    if nat_lt(a669, a670) {
        let a671 = v0;
        let a672 = 2u64;
        if nat_lt(a671, a672) {
            let a673 = v0;
            let a674 = 1u64;
            if nat_lt(a673, a674) {
                let a675 = v0;
                let a676 = 0u64;
                if nat_eq(a675, a676) {
                    Ok(f49(3u64, f66(), 2u64, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a677 = v0;
                let a678 = 1u64;
                if nat_eq(a677, a678) {
                    Ok(f43(2059u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a679 = v0;
            let a680 = 3u64;
            if nat_lt(a679, a680) {
                let a681 = v0;
                let a682 = 2u64;
                if nat_eq(a681, a682) {
                    Ok(f34(f66(), f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a683 = v0;
                let a684 = 3u64;
                if nat_eq(a683, a684) {
                    Ok(f34(0u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a685 = v0;
        let a686 = 6u64;
        if nat_lt(a685, a686) {
            let a687 = v0;
            let a688 = 5u64;
            if nat_lt(a687, a688) {
                let a689 = v0;
                let a690 = 4u64;
                if nat_eq(a689, a690) {
                    Ok(f46(4u64, 0u64, f68(f35())?, 0u64, f47(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a691 = v0;
                let a692 = 5u64;
                if nat_eq(a691, a692) {
                    Ok(f49(0u64, f48(), f45(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a693 = v0;
            let a694 = 7u64;
            if nat_lt(a693, a694) {
                let a695 = v0;
                let a696 = 6u64;
                if nat_eq(a695, a696) {
                    Ok(f50(5u64, f51(f48())?, f67(), 0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a697 = v0;
                let a698 = 7u64;
                if nat_eq(a697, a698) {
                    Ok(f49(0u64, f69(), f38(), f70()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a699 = v0;
    let a700 = 4u64;
    if nat_lt(a699, a700) {
        let a701 = v0;
        let a702 = 2u64;
        if nat_lt(a701, a702) {
            let a703 = v0;
            let a704 = 1u64;
            if nat_lt(a703, a704) {
                let a705 = v0;
                let a706 = 0u64;
                if nat_eq(a705, a706) {
                    Ok(f54(2u64, f71(f38())?, f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a707 = v0;
                let a708 = 1u64;
                if nat_eq(a707, a708) {
                    Ok(f54(2u64, f57(), f72(f70())?, f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a709 = v0;
            let a710 = 3u64;
            if nat_lt(a709, a710) {
                let a711 = v0;
                let a712 = 2u64;
                if nat_eq(a711, a712) {
                    Ok(f54(0u64, f36(), f57(), f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a713 = v0;
                let a714 = 3u64;
                if nat_eq(a713, a714) {
                    Ok(f54(2u64, f71(f38())?, f52(), f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a715 = v0;
        let a716 = 6u64;
        if nat_lt(a715, a716) {
            let a717 = v0;
            let a718 = 5u64;
            if nat_lt(a717, a718) {
                let a719 = v0;
                let a720 = 4u64;
                if nat_eq(a719, a720) {
                    Ok(f54(2u64, f57(), f52(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a721 = v0;
                let a722 = 5u64;
                if nat_eq(a721, a722) {
                    Ok(f54(0u64, f37(), f57(), f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a723 = v0;
            let a724 = 7u64;
            if nat_lt(a723, a724) {
                let a725 = v0;
                let a726 = 6u64;
                if nat_eq(a725, a726) {
                    Ok(f49(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            } else {
                let a727 = v0;
                let a728 = 7u64;
                if nat_eq(a727, a728) {
                    Ok(f49(0u64, f45(), 1u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a729 = v0;
    let a730 = 4u64;
    if nat_lt(a729, a730) {
        let a731 = v0;
        let a732 = 2u64;
        if nat_lt(a731, a732) {
            let a733 = v0;
            let a734 = 1u64;
            if nat_lt(a733, a734) {
                let a735 = v0;
                let a736 = 0u64;
                if nat_eq(a735, a736) {
                    f39(1u64, f45(), 8u64, 2060u64)
                } else {
                    f2()
                }
            } else {
                let a737 = v0;
                let a738 = 1u64;
                if nat_eq(a737, a738) {
                    f49(0u64, f35(), 1u64, f35())
                } else {
                    f2()
                }
            }
        } else {
            let a739 = v0;
            let a740 = 3u64;
            if nat_lt(a739, a740) {
                let a741 = v0;
                let a742 = 2u64;
                if nat_eq(a741, a742) {
                    f39(1u64, f35(), f73(), 2052u64)
                } else {
                    f2()
                }
            } else {
                let a743 = v0;
                let a744 = 3u64;
                if nat_eq(a743, a744) {
                    f39(2u64, 0u64, f36(), 2077u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a745 = v0;
        let a746 = 6u64;
        if nat_lt(a745, a746) {
            let a747 = v0;
            let a748 = 5u64;
            if nat_lt(a747, a748) {
                let a749 = v0;
                let a750 = 4u64;
                if nat_eq(a749, a750) {
                    f43(2560u64)
                } else {
                    f2()
                }
            } else {
                let a751 = v0;
                let a752 = 5u64;
                if nat_eq(a751, a752) {
                    f39(2u64, 0u64, f37(), 2079u64)
                } else {
                    f2()
                }
            }
        } else {
            let a753 = v0;
            let a754 = 7u64;
            if nat_lt(a753, a754) {
                let a755 = v0;
                let a756 = 6u64;
                if nat_eq(a755, a756) {
                    f43(2560u64)
                } else {
                    f2()
                }
            } else {
                let a757 = v0;
                let a758 = 7u64;
                if nat_eq(a757, a758) {
                    f54(3u64, f36(), f37(), f60())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a759 = v0;
    let a760 = 0u64;
    if nat_eq(a759, a760) {
        f43(2560u64)
    } else {
        f2()
    }
}

pub fn f30(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a761 = v0;
    let a762 = 4u64;
    if nat_lt(a761, a762) {
        let a763 = v0;
        let a764 = 2u64;
        if nat_lt(a763, a764) {
            let a765 = v0;
            let a766 = 1u64;
            if nat_lt(a765, a766) {
                let a767 = v0;
                let a768 = 0u64;
                if nat_eq(a767, a768) {
                    Ok(f34(f60(), f74(f75())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a769 = v0;
                let a770 = 1u64;
                if nat_eq(a769, a770) {
                    Ok(f39(2u64, f76(), f60(), 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a771 = v0;
            let a772 = 3u64;
            if nat_lt(a771, a772) {
                let a773 = v0;
                let a774 = 2u64;
                if nat_eq(a773, a774) {
                    Ok(f43(2564u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a775 = v0;
                let a776 = 3u64;
                if nat_eq(a775, a776) {
                    Ok(f34(f60(), f76()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a777 = v0;
        let a778 = 6u64;
        if nat_lt(a777, a778) {
            let a779 = v0;
            let a780 = 5u64;
            if nat_lt(a779, a780) {
                let a781 = v0;
                let a782 = 4u64;
                if nat_eq(a781, a782) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a783 = v0;
                let a784 = 5u64;
                if nat_eq(a783, a784) {
                    Ok(f34(0u64, f77()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a785 = v0;
            let a786 = 7u64;
            if nat_lt(a785, a786) {
                let a787 = v0;
                let a788 = 6u64;
                if nat_eq(a787, a788) {
                    Ok(f78(7u64, f75(), f35(), f68(f35())?, f79(), f80(), f81()))
                } else {
                    Ok(f2())
                }
            } else {
                let a789 = v0;
                let a790 = 7u64;
                if nat_eq(a789, a790) {
                    Ok(f34(f81(), f82(f79())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a791 = v0;
    let a792 = 4u64;
    if nat_lt(a791, a792) {
        let a793 = v0;
        let a794 = 2u64;
        if nat_lt(a793, a794) {
            let a795 = v0;
            let a796 = 1u64;
            if nat_lt(a795, a796) {
                let a797 = v0;
                let a798 = 0u64;
                if nat_eq(a797, a798) {
                    Ok(f49(3u64, f68(f35())?, 256u64, f81()))
                } else {
                    Ok(f2())
                }
            } else {
                let a799 = v0;
                let a800 = 1u64;
                if nat_eq(a799, a800) {
                    Ok(f49(2u64, f81(), f80(), f81()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a801 = v0;
            let a802 = 3u64;
            if nat_lt(a801, a802) {
                let a803 = v0;
                let a804 = 2u64;
                if nat_eq(a803, a804) {
                    Ok(f49(0u64, f77(), f81(), f77()))
                } else {
                    Ok(f2())
                }
            } else {
                let a805 = v0;
                let a806 = 3u64;
                if nat_eq(a805, a806) {
                    Ok(f49(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a807 = v0;
        let a808 = 6u64;
        if nat_lt(a807, a808) {
            let a809 = v0;
            let a810 = 5u64;
            if nat_lt(a809, a810) {
                let a811 = v0;
                let a812 = 4u64;
                if nat_eq(a811, a812) {
                    Ok(f39(1u64, f35(), f73(), 2566u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a813 = v0;
                let a814 = 5u64;
                if nat_eq(a813, a814) {
                    Ok(f39(0u64, f64(), 1u64, 2582u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a815 = v0;
            let a816 = 7u64;
            if nat_lt(a815, a816) {
                let a817 = v0;
                let a818 = 6u64;
                if nat_eq(a817, a818) {
                    Ok(f49(2u64, f66(), 32768u64, f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a819 = v0;
                let a820 = 7u64;
                if nat_eq(a819, a820) {
                    Ok(f49(0u64, f77(), f47(), f77()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f32(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a821 = v0;
    let a822 = 4u64;
    if nat_lt(a821, a822) {
        let a823 = v0;
        let a824 = 2u64;
        if nat_lt(a823, a824) {
            let a825 = v0;
            let a826 = 1u64;
            if nat_lt(a825, a826) {
                let a827 = v0;
                let a828 = 0u64;
                if nat_eq(a827, a828) {
                    Ok(f49(2u64, f75(), 2u64, f79()))
                } else {
                    Ok(f2())
                }
            } else {
                let a829 = v0;
                let a830 = 1u64;
                if nat_eq(a829, a830) {
                    Ok(f49(0u64, f79(), 34u64, f79()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a831 = v0;
            let a832 = 3u64;
            if nat_lt(a831, a832) {
                let a833 = v0;
                let a834 = 2u64;
                if nat_eq(a833, a834) {
                    Ok(f49(4u64, f77(), 256u64, f82(f79())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a835 = v0;
                let a836 = 3u64;
                if nat_eq(a835, a836) {
                    Ok(f49(0u64, f79(), 1u64, f79()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a837 = v0;
        let a838 = 6u64;
        if nat_lt(a837, a838) {
            let a839 = v0;
            let a840 = 5u64;
            if nat_lt(a839, a840) {
                let a841 = v0;
                let a842 = 4u64;
                if nat_eq(a841, a842) {
                    Ok(f49(3u64, f77(), 256u64, f82(f79())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a843 = v0;
                let a844 = 5u64;
                if nat_eq(a843, a844) {
                    Ok(f43(2586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a845 = v0;
            let a846 = 7u64;
            if nat_lt(a845, a846) {
                let a847 = v0;
                let a848 = 6u64;
                if nat_eq(a847, a848) {
                    Ok(f50(6u64, f66(), 0u64, 0u64, f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a849 = v0;
                let a850 = 7u64;
                if nat_eq(a849, a850) {
                    Ok(f49(0u64, f77(), f47(), f77()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a851 = v0;
    let a852 = 2u64;
    if nat_lt(a851, a852) {
        let a853 = v0;
        let a854 = 1u64;
        if nat_lt(a853, a854) {
            let a855 = v0;
            let a856 = 0u64;
            if nat_eq(a855, a856) {
                Ok(f49(0u64, f75(), 32u64, f79()))
            } else {
                Ok(f2())
            }
        } else {
            let a857 = v0;
            let a858 = 1u64;
            if nat_eq(a857, a858) {
                Ok(f34(f77(), f82(f79())?))
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
                Ok(f49(0u64, f75(), 1u64, f75()))
            } else {
                Ok(f2())
            }
        } else {
            let a863 = v0;
            let a864 = 4u64;
            if nat_lt(a863, a864) {
                let a865 = v0;
                let a866 = 3u64;
                if nat_eq(a865, a866) {
                    Ok(f39(1u64, f75(), f83(), 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a867 = v0;
                let a868 = 4u64;
                if nat_eq(a867, a868) {
                    Ok(f43(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f34(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f35() -> u64 {
    4294967323u64
}

pub fn f36() -> u64 {
    4294967335u64
}

pub fn f37() -> u64 {
    4294967336u64
}

pub fn f38() -> u64 {
    4294967303u64
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f40() -> u64 {
    4294967324u64
}

pub fn f41(v0: u64) -> R<u64> {
    let a872 = 8589976576u64;
    let a873 = {
        let a869 = v0;
        let a870 = 4294967296u64;
        let a871 = 0u64;
        nat_rem(a869, a870, a871)
    };
    nat_add(a872, a873)
}

pub fn f42() -> u64 {
    4294967321u64
}

pub fn f43(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f44(v0: u64) -> R<u64> {
    let a877 = 8589980672u64;
    let a878 = {
        let a874 = v0;
        let a875 = 4294967296u64;
        let a876 = 0u64;
        nat_rem(a874, a875, a876)
    };
    nat_add(a877, a878)
}

pub fn f45() -> u64 {
    4294967305u64
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f47() -> u64 {
    4294967338u64
}

pub fn f48() -> u64 {
    4294967330u64
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f51(v0: u64) -> R<u64> {
    let a882 = 25769803776u64;
    let a883 = {
        let a879 = v0;
        let a880 = 4294967296u64;
        let a881 = 0u64;
        nat_rem(a879, a880, a881)
    };
    nat_add(a882, a883)
}

pub fn f52() -> u64 {
    4294967332u64
}

pub fn f53(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f55(v0: u64) -> R<u64> {
    let a887 = 8589959168u64;
    let a888 = {
        let a884 = v0;
        let a885 = 4294967296u64;
        let a886 = 0u64;
        nat_rem(a884, a885, a886)
    };
    nat_add(a887, a888)
}

pub fn f56(v0: u64) -> R<u64> {
    let a892 = 8589950976u64;
    let a893 = {
        let a889 = v0;
        let a890 = 4294967296u64;
        let a891 = 0u64;
        nat_rem(a889, a890, a891)
    };
    nat_add(a892, a893)
}

pub fn f57() -> u64 {
    4294967337u64
}

pub fn f58(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f59() -> u64 {
    4294967351u64
}

pub fn f60() -> u64 {
    4294967316u64
}

pub fn f61(v0: u64) -> R<u64> {
    let a897 = 8589982720u64;
    let a898 = {
        let a894 = v0;
        let a895 = 4294967296u64;
        let a896 = 0u64;
        nat_rem(a894, a895, a896)
    };
    nat_add(a897, a898)
}

pub fn f62() -> u64 {
    4294967319u64
}

pub fn f63() -> u64 {
    4294967320u64
}

pub fn f64() -> u64 {
    4294967297u64
}

pub fn f65() -> u64 {
    4294967350u64
}

pub fn f66() -> u64 {
    4294967346u64
}

pub fn f67() -> u64 {
    4294967314u64
}

pub fn f68(v0: u64) -> R<u64> {
    let a902 = 8589959168u64;
    let a903 = {
        let a899 = v0;
        let a900 = 4294967296u64;
        let a901 = 0u64;
        nat_rem(a899, a900, a901)
    };
    nat_add(a902, a903)
}

pub fn f69() -> u64 {
    4294967354u64
}

pub fn f70() -> u64 {
    4294967307u64
}

pub fn f71(v0: u64) -> R<u64> {
    let a907 = 8589950976u64;
    let a908 = {
        let a904 = v0;
        let a905 = 4294967296u64;
        let a906 = 0u64;
        nat_rem(a904, a905, a906)
    };
    nat_add(a907, a908)
}

pub fn f72(v0: u64) -> R<u64> {
    let a912 = 12884901888u64;
    let a913 = {
        let a909 = v0;
        let a910 = 4294967296u64;
        let a911 = 0u64;
        nat_rem(a909, a910, a911)
    };
    nat_add(a912, a913)
}

pub fn f73() -> u64 {
    4294967302u64
}

pub fn f74(v0: u64) -> R<u64> {
    let a917 = 8589992960u64;
    let a918 = {
        let a914 = v0;
        let a915 = 4294967296u64;
        let a916 = 0u64;
        nat_rem(a914, a915, a916)
    };
    nat_add(a917, a918)
}

pub fn f75() -> u64 {
    4294967301u64
}

pub fn f76() -> u64 {
    4294967347u64
}

pub fn f77() -> u64 {
    4294967342u64
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f79() -> u64 {
    4294967344u64
}

pub fn f80() -> u64 {
    4294967345u64
}

pub fn f81() -> u64 {
    4294967331u64
}

pub fn f82(v0: u64) -> R<u64> {
    let a922 = 30064771072u64;
    let a923 = {
        let a919 = v0;
        let a920 = 4294967296u64;
        let a921 = 0u64;
        nat_rem(a919, a920, a921)
    };
    nat_add(a922, a923)
}

pub fn f83() -> u64 {
    4294967358u64
}
