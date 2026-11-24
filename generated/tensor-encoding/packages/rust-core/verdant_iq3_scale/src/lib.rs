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
                Ok(f9(v1))
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
                Ok(f11({
                    let a45 = v0;
                    let a46 = 8u64;
                    let a47 = 0u64;
                    nat_rem(a45, a46, a47)
                }))
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
            let a56 = 4u64;
            if nat_lt(a55, a56) {
                let a57 = v1;
                let a58 = 3u64;
                if nat_eq(a57, a58) {
                    f13({
                        let a59 = v0;
                        let a60 = 8u64;
                        let a61 = 0u64;
                        nat_rem(a59, a60, a61)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a62 = v1;
                let a63 = 4u64;
                if nat_eq(a62, a63) {
                    f14({
                        let a64 = v0;
                        let a65 = 8u64;
                        let a66 = 0u64;
                        nat_rem(a64, a65, a66)
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
        let a67 = v0;
        let a68 = 8u64;
        let a69 = 0u64;
        nat_quot(a67, a68, a69)
    };
    let a70 = v1;
    let a71 = 2u64;
    if nat_lt(a70, a71) {
        let a72 = v1;
        let a73 = 1u64;
        if nat_lt(a72, a73) {
            let a74 = v1;
            let a75 = 0u64;
            if nat_eq(a74, a75) {
                Ok(f15({
                    let a76 = v0;
                    let a77 = 8u64;
                    let a78 = 0u64;
                    nat_rem(a76, a77, a78)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a79 = v1;
            let a80 = 1u64;
            if nat_eq(a79, a80) {
                f16({
                    let a81 = v0;
                    let a82 = 8u64;
                    let a83 = 0u64;
                    nat_rem(a81, a82, a83)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a84 = v1;
        let a85 = 3u64;
        if nat_lt(a84, a85) {
            let a86 = v1;
            let a87 = 2u64;
            if nat_eq(a86, a87) {
                Ok(f17({
                    let a88 = v0;
                    let a89 = 8u64;
                    let a90 = 0u64;
                    nat_rem(a88, a89, a90)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a91 = v1;
            let a92 = 3u64;
            if nat_eq(a91, a92) {
                Ok(f18({
                    let a93 = v0;
                    let a94 = 8u64;
                    let a95 = 0u64;
                    nat_rem(a93, a94, a95)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a96 = v0;
        let a97 = 8u64;
        let a98 = 0u64;
        nat_quot(a96, a97, a98)
    };
    let a99 = v1;
    let a100 = 2u64;
    if nat_lt(a99, a100) {
        let a101 = v1;
        let a102 = 1u64;
        if nat_lt(a101, a102) {
            let a103 = v1;
            let a104 = 0u64;
            if nat_eq(a103, a104) {
                f19({
                    let a105 = v0;
                    let a106 = 8u64;
                    let a107 = 0u64;
                    nat_rem(a105, a106, a107)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a108 = v1;
            let a109 = 1u64;
            if nat_eq(a108, a109) {
                f20({
                    let a110 = v0;
                    let a111 = 8u64;
                    let a112 = 0u64;
                    nat_rem(a110, a111, a112)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a113 = v1;
        let a114 = 3u64;
        if nat_lt(a113, a114) {
            let a115 = v1;
            let a116 = 2u64;
            if nat_eq(a115, a116) {
                f21({
                    let a117 = v0;
                    let a118 = 8u64;
                    let a119 = 0u64;
                    nat_rem(a117, a118, a119)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a120 = v1;
            let a121 = 3u64;
            if nat_eq(a120, a121) {
                f22({
                    let a122 = v0;
                    let a123 = 8u64;
                    let a124 = 0u64;
                    nat_rem(a122, a123, a124)
                })
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a125 = v0;
        let a126 = 8u64;
        let a127 = 0u64;
        nat_quot(a125, a126, a127)
    };
    let a128 = v1;
    let a129 = 2u64;
    if nat_lt(a128, a129) {
        let a130 = v1;
        let a131 = 1u64;
        if nat_lt(a130, a131) {
            let a132 = v1;
            let a133 = 0u64;
            if nat_eq(a132, a133) {
                Ok(f23({
                    let a134 = v0;
                    let a135 = 8u64;
                    let a136 = 0u64;
                    nat_rem(a134, a135, a136)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a137 = v1;
            let a138 = 1u64;
            if nat_eq(a137, a138) {
                f16({
                    let a139 = v0;
                    let a140 = 8u64;
                    let a141 = 0u64;
                    nat_rem(a139, a140, a141)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a142 = v1;
        let a143 = 3u64;
        if nat_lt(a142, a143) {
            let a144 = v1;
            let a145 = 2u64;
            if nat_eq(a144, a145) {
                Ok(f24({
                    let a146 = v0;
                    let a147 = 8u64;
                    let a148 = 0u64;
                    nat_rem(a146, a147, a148)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a149 = v1;
            let a150 = 3u64;
            if nat_eq(a149, a150) {
                Ok(f18({
                    let a151 = v0;
                    let a152 = 8u64;
                    let a153 = 0u64;
                    nat_rem(a151, a152, a153)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a154 = v0;
        let a155 = 8u64;
        let a156 = 0u64;
        nat_quot(a154, a155, a156)
    };
    let a157 = v1;
    let a158 = 1u64;
    if nat_lt(a157, a158) {
        let a159 = v1;
        let a160 = 0u64;
        if nat_eq(a159, a160) {
            f10({
                let a161 = v0;
                let a162 = 8u64;
                let a163 = 0u64;
                nat_rem(a161, a162, a163)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a164 = v1;
        let a165 = 2u64;
        if nat_lt(a164, a165) {
            let a166 = v1;
            let a167 = 1u64;
            if nat_eq(a166, a167) {
                f25({
                    let a168 = v0;
                    let a169 = 8u64;
                    let a170 = 0u64;
                    nat_rem(a168, a169, a170)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a171 = v1;
            let a172 = 2u64;
            if nat_eq(a171, a172) {
                Ok(f26({
                    let a173 = v0;
                    let a174 = 8u64;
                    let a175 = 0u64;
                    nat_rem(a173, a174, a175)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a176 = v0;
        let a177 = 8u64;
        let a178 = 0u64;
        nat_quot(a176, a177, a178)
    };
    let a179 = v1;
    let a180 = 2u64;
    if nat_lt(a179, a180) {
        let a181 = v1;
        let a182 = 1u64;
        if nat_lt(a181, a182) {
            let a183 = v1;
            let a184 = 0u64;
            if nat_eq(a183, a184) {
                Ok(f23({
                    let a185 = v0;
                    let a186 = 8u64;
                    let a187 = 0u64;
                    nat_rem(a185, a186, a187)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a188 = v1;
            let a189 = 1u64;
            if nat_eq(a188, a189) {
                f16({
                    let a190 = v0;
                    let a191 = 8u64;
                    let a192 = 0u64;
                    nat_rem(a190, a191, a192)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a193 = v1;
        let a194 = 3u64;
        if nat_lt(a193, a194) {
            let a195 = v1;
            let a196 = 2u64;
            if nat_eq(a195, a196) {
                Ok(f17({
                    let a197 = v0;
                    let a198 = 8u64;
                    let a199 = 0u64;
                    nat_rem(a197, a198, a199)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a200 = v1;
            let a201 = 3u64;
            if nat_eq(a200, a201) {
                Ok(f18({
                    let a202 = v0;
                    let a203 = 8u64;
                    let a204 = 0u64;
                    nat_rem(a202, a203, a204)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a205 = v0;
        let a206 = 8u64;
        let a207 = 0u64;
        nat_quot(a205, a206, a207)
    };
    let a208 = v1;
    let a209 = 5u64;
    if nat_lt(a208, a209) {
        let a210 = v1;
        let a211 = 2u64;
        if nat_lt(a210, a211) {
            let a212 = v1;
            let a213 = 1u64;
            if nat_lt(a212, a213) {
                let a214 = v1;
                let a215 = 0u64;
                if nat_eq(a214, a215) {
                    f27({
                        let a216 = v0;
                        let a217 = 8u64;
                        let a218 = 0u64;
                        nat_rem(a216, a217, a218)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a219 = v1;
                let a220 = 1u64;
                if nat_eq(a219, a220) {
                    Ok(f28({
                        let a221 = v0;
                        let a222 = 8u64;
                        let a223 = 0u64;
                        nat_rem(a221, a222, a223)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a224 = v1;
            let a225 = 3u64;
            if nat_lt(a224, a225) {
                let a226 = v1;
                let a227 = 2u64;
                if nat_eq(a226, a227) {
                    f29({
                        let a228 = v0;
                        let a229 = 8u64;
                        let a230 = 0u64;
                        nat_rem(a228, a229, a230)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a231 = v1;
                let a232 = 4u64;
                if nat_lt(a231, a232) {
                    let a233 = v1;
                    let a234 = 3u64;
                    if nat_eq(a233, a234) {
                        f30({
                            let a235 = v0;
                            let a236 = 8u64;
                            let a237 = 0u64;
                            nat_rem(a235, a236, a237)
                        })
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a238 = v1;
                    let a239 = 4u64;
                    if nat_eq(a238, a239) {
                        Ok(f31({
                            let a240 = v0;
                            let a241 = 8u64;
                            let a242 = 0u64;
                            nat_rem(a240, a241, a242)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    } else {
        let a243 = v1;
        let a244 = 8u64;
        if nat_lt(a243, a244) {
            let a245 = v1;
            let a246 = 6u64;
            if nat_lt(a245, a246) {
                let a247 = v1;
                let a248 = 5u64;
                if nat_eq(a247, a248) {
                    f32({
                        let a249 = v0;
                        let a250 = 8u64;
                        let a251 = 0u64;
                        nat_rem(a249, a250, a251)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a252 = v1;
                let a253 = 7u64;
                if nat_lt(a252, a253) {
                    let a254 = v1;
                    let a255 = 6u64;
                    if nat_eq(a254, a255) {
                        f33({
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
                    let a260 = 7u64;
                    if nat_eq(a259, a260) {
                        Ok(f34({
                            let a261 = v0;
                            let a262 = 8u64;
                            let a263 = 0u64;
                            nat_rem(a261, a262, a263)
                        }))
                    } else {
                        Ok(f2())
                    }
                }
            }
        } else {
            let a264 = v1;
            let a265 = 9u64;
            if nat_lt(a264, a265) {
                let a266 = v1;
                let a267 = 8u64;
                if nat_eq(a266, a267) {
                    Ok(f35({
                        let a268 = v0;
                        let a269 = 8u64;
                        let a270 = 0u64;
                        nat_rem(a268, a269, a270)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a271 = v1;
                let a272 = 10u64;
                if nat_lt(a271, a272) {
                    let a273 = v1;
                    let a274 = 9u64;
                    if nat_eq(a273, a274) {
                        Ok(f36({
                            let a275 = v0;
                            let a276 = 8u64;
                            let a277 = 0u64;
                            nat_rem(a275, a276, a277)
                        }))
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a278 = v1;
                    let a279 = 10u64;
                    if nat_eq(a278, a279) {
                        f37({
                            let a280 = v0;
                            let a281 = 8u64;
                            let a282 = 0u64;
                            nat_rem(a280, a281, a282)
                        })
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let v1: u64 = {
        let a283 = v0;
        let a284 = 8u64;
        let a285 = 0u64;
        nat_quot(a283, a284, a285)
    };
    let a286 = v1;
    let a287 = 0u64;
    if nat_eq(a286, a287) {
        f38({
            let a288 = v0;
            let a289 = 8u64;
            let a290 = 0u64;
            nat_rem(a288, a289, a290)
        })
    } else {
        f2()
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a291 = v0;
    let a292 = 4u64;
    if nat_lt(a291, a292) {
        let a293 = v0;
        let a294 = 2u64;
        if nat_lt(a293, a294) {
            let a295 = v0;
            let a296 = 1u64;
            if nat_lt(a295, a296) {
                let a297 = v0;
                let a298 = 0u64;
                if nat_eq(a297, a298) {
                    Ok(f39(2u64, f40(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a299 = v0;
                let a300 = 1u64;
                if nat_eq(a299, a300) {
                    Ok(f41(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a301 = v0;
            let a302 = 3u64;
            if nat_lt(a301, a302) {
                let a303 = v0;
                let a304 = 2u64;
                if nat_eq(a303, a304) {
                    Ok(f42(6u64, f40(), 0u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a305 = v0;
                let a306 = 3u64;
                if nat_eq(a305, a306) {
                    Ok(f43(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a307 = v0;
        let a308 = 6u64;
        if nat_lt(a307, a308) {
            let a309 = v0;
            let a310 = 5u64;
            if nat_lt(a309, a310) {
                let a311 = v0;
                let a312 = 4u64;
                if nat_eq(a311, a312) {
                    Ok(f45(1u64, 255u64, f46(f44())?, f46(f44())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a313 = v0;
                let a314 = 5u64;
                if nat_eq(a313, a314) {
                    Ok(f45(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a315 = v0;
            let a316 = 7u64;
            if nat_lt(a315, a316) {
                let a317 = v0;
                let a318 = 6u64;
                if nat_eq(a317, a318) {
                    Ok(f39(1u64, f44(), 4u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a319 = v0;
                let a320 = 7u64;
                if nat_eq(a319, a320) {
                    Ok(f43(f40(), f47(f48())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a321 = v0;
    let a322 = 4u64;
    if nat_lt(a321, a322) {
        let a323 = v0;
        let a324 = 2u64;
        if nat_lt(a323, a324) {
            let a325 = v0;
            let a326 = 1u64;
            if nat_lt(a325, a326) {
                let a327 = v0;
                let a328 = 0u64;
                if nat_eq(a327, a328) {
                    f39(2u64, f49(), f40(), 3082u64)
                } else {
                    f2()
                }
            } else {
                let a329 = v0;
                let a330 = 1u64;
                if nat_eq(a329, a330) {
                    f41(3083u64)
                } else {
                    f2()
                }
            }
        } else {
            let a331 = v0;
            let a332 = 3u64;
            if nat_lt(a331, a332) {
                let a333 = v0;
                let a334 = 2u64;
                if nat_eq(a333, a334) {
                    f43(f40(), f49())
                } else {
                    f2()
                }
            } else {
                let a335 = v0;
                let a336 = 3u64;
                if nat_eq(a335, a336) {
                    f43(f50(), f51())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a337 = v0;
        let a338 = 6u64;
        if nat_lt(a337, a338) {
            let a339 = v0;
            let a340 = 5u64;
            if nat_lt(a339, a340) {
                let a341 = v0;
                let a342 = 4u64;
                if nat_eq(a341, a342) {
                    f39(0u64, f52(), 0u64, 3086u64)
                } else {
                    f2()
                }
            } else {
                let a343 = v0;
                let a344 = 5u64;
                if nat_eq(a343, a344) {
                    f43(f48(), f51())
                } else {
                    f2()
                }
            }
        } else {
            let a345 = v0;
            let a346 = 7u64;
            if nat_lt(a345, a346) {
                let a347 = v0;
                let a348 = 6u64;
                if nat_eq(a347, a348) {
                    f45(0u64, f50(), 1u64, f50())
                } else {
                    f2()
                }
            } else {
                let a349 = v0;
                let a350 = 7u64;
                if nat_eq(a349, a350) {
                    f43(0u64, f53())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a351 = v0;
    let a352 = 4u64;
    if nat_lt(a351, a352) {
        let a353 = v0;
        let a354 = 2u64;
        if nat_lt(a353, a354) {
            let a355 = v0;
            let a356 = 1u64;
            if nat_lt(a355, a356) {
                let a357 = v0;
                let a358 = 0u64;
                if nat_eq(a357, a358) {
                    Ok(f45(2u64, f51(), 8u64, f54()))
                } else {
                    Ok(f2())
                }
            } else {
                let a359 = v0;
                let a360 = 1u64;
                if nat_eq(a359, a360) {
                    Ok(f45(0u64, f54(), f53(), f54()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a361 = v0;
            let a362 = 3u64;
            if nat_lt(a361, a362) {
                let a363 = v0;
                let a364 = 2u64;
                if nat_eq(a363, a364) {
                    Ok(f55(7u64, f54(), 0u64, f56(f53())?, f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a365 = v0;
                let a366 = 3u64;
                if nat_eq(a365, a366) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a367 = v0;
        let a368 = 6u64;
        if nat_lt(a367, a368) {
            let a369 = v0;
            let a370 = 5u64;
            if nat_lt(a369, a370) {
                let a371 = v0;
                let a372 = 4u64;
                if nat_eq(a371, a372) {
                    Ok(f39(0u64, f52(), 1u64, 3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a373 = v0;
                let a374 = 5u64;
                if nat_eq(a373, a374) {
                    Ok(f45(2u64, f48(), 8u64, f54()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a375 = v0;
            let a376 = 7u64;
            if nat_lt(a375, a376) {
                let a377 = v0;
                let a378 = 6u64;
                if nat_eq(a377, a378) {
                    Ok(f45(0u64, f54(), f53(), f54()))
                } else {
                    Ok(f2())
                }
            } else {
                let a379 = v0;
                let a380 = 7u64;
                if nat_eq(a379, a380) {
                    Ok(f55(7u64, f54(), 1u64, f56(f53())?, f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a381 = v0;
    let a382 = 4u64;
    if nat_lt(a381, a382) {
        let a383 = v0;
        let a384 = 2u64;
        if nat_lt(a383, a384) {
            let a385 = v0;
            let a386 = 1u64;
            if nat_lt(a385, a386) {
                let a387 = v0;
                let a388 = 0u64;
                if nat_eq(a387, a388) {
                    Ok(f61(f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a389 = v0;
                let a390 = 1u64;
                if nat_eq(a389, a390) {
                    Ok(f45(0u64, f53(), 1u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a391 = v0;
            let a392 = 3u64;
            if nat_lt(a391, a392) {
                let a393 = v0;
                let a394 = 2u64;
                if nat_eq(a393, a394) {
                    Ok(f39(1u64, f53(), 8u64, 3088u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a395 = v0;
                let a396 = 3u64;
                if nat_eq(a395, a396) {
                    Ok(f43(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a397 = v0;
        let a398 = 6u64;
        if nat_lt(a397, a398) {
            let a399 = v0;
            let a400 = 5u64;
            if nat_lt(a399, a400) {
                let a401 = v0;
                let a402 = 4u64;
                if nat_eq(a401, a402) {
                    Ok(f62(9u64, f51(), f44(), 0u64, f57(), f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a403 = v0;
                let a404 = 5u64;
                if nat_eq(a403, a404) {
                    Ok(f43(f46(f44())?, f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a405 = v0;
            let a406 = 7u64;
            if nat_lt(a405, a406) {
                let a407 = v0;
                let a408 = 6u64;
                if nat_eq(a407, a408) {
                    Ok(f39(0u64, f52(), 0u64, 3106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a409 = v0;
                let a410 = 7u64;
                if nat_eq(a409, a410) {
                    Ok(f45(4u64, f59(), 128u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a411 = v0;
    let a412 = 4u64;
    if nat_lt(a411, a412) {
        let a413 = v0;
        let a414 = 2u64;
        if nat_lt(a413, a414) {
            let a415 = v0;
            let a416 = 1u64;
            if nat_lt(a415, a416) {
                let a417 = v0;
                let a418 = 0u64;
                if nat_eq(a417, a418) {
                    Ok(f63(f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a419 = v0;
                let a420 = 1u64;
                if nat_eq(a419, a420) {
                    Ok(f41(3107u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a421 = v0;
            let a422 = 3u64;
            if nat_lt(a421, a422) {
                let a423 = v0;
                let a424 = 2u64;
                if nat_eq(a423, a424) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a425 = v0;
                let a426 = 3u64;
                if nat_eq(a425, a426) {
                    Ok(f45(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a427 = v0;
        let a428 = 6u64;
        if nat_lt(a427, a428) {
            let a429 = v0;
            let a430 = 5u64;
            if nat_lt(a429, a430) {
                let a431 = v0;
                let a432 = 4u64;
                if nat_eq(a431, a432) {
                    Ok(f39(1u64, f44(), 4u64, 3100u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a433 = v0;
                let a434 = 5u64;
                if nat_eq(a433, a434) {
                    Ok(f45(0u64, f48(), 1u64, f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a435 = v0;
            let a436 = 7u64;
            if nat_lt(a435, a436) {
                let a437 = v0;
                let a438 = 6u64;
                if nat_eq(a437, a438) {
                    Ok(f39(1u64, f48(), 8u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a439 = v0;
                let a440 = 7u64;
                if nat_eq(a439, a440) {
                    Ok(f41(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a441 = v0;
    let a442 = 4u64;
    if nat_lt(a441, a442) {
        let a443 = v0;
        let a444 = 2u64;
        if nat_lt(a443, a444) {
            let a445 = v0;
            let a446 = 1u64;
            if nat_lt(a445, a446) {
                let a447 = v0;
                let a448 = 0u64;
                if nat_eq(a447, a448) {
                    f39(0u64, f49(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a449 = v0;
                let a450 = 1u64;
                if nat_eq(a449, a450) {
                    f42(3u64, f49(), 1106771968u64, f40())
                } else {
                    f2()
                }
            }
        } else {
            let a451 = v0;
            let a452 = 3u64;
            if nat_lt(a451, a452) {
                let a453 = v0;
                let a454 = 2u64;
                if nat_eq(a453, a454) {
                    f43(1065458074u64, f64())
                } else {
                    f2()
                }
            } else {
                let a455 = v0;
                let a456 = 3u64;
                if nat_eq(a455, a456) {
                    f39(0u64, f52(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a457 = v0;
        let a458 = 6u64;
        if nat_lt(a457, a458) {
            let a459 = v0;
            let a460 = 5u64;
            if nat_lt(a459, a460) {
                let a461 = v0;
                let a462 = 4u64;
                if nat_eq(a461, a462) {
                    f43(1065630040u64, f64())
                } else {
                    f2()
                }
            } else {
                let a463 = v0;
                let a464 = 5u64;
                if nat_eq(a463, a464) {
                    f42(2u64, f40(), f64(), f65())
                } else {
                    f2()
                }
            }
        } else {
            let a465 = v0;
            let a466 = 7u64;
            if nat_lt(a465, a466) {
                let a467 = v0;
                let a468 = 6u64;
                if nat_eq(a467, a468) {
                    f66(f65(), f65())
                } else {
                    f2()
                }
            } else {
                let a469 = v0;
                let a470 = 7u64;
                if nat_eq(a469, a470) {
                    f45(4u64, f65(), 256u64, f59())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a471 = v0;
    let a472 = 4u64;
    if nat_lt(a471, a472) {
        let a473 = v0;
        let a474 = 2u64;
        if nat_lt(a473, a474) {
            let a475 = v0;
            let a476 = 1u64;
            if nat_lt(a475, a476) {
                let a477 = v0;
                let a478 = 0u64;
                if nat_eq(a477, a478) {
                    Ok(f43(0u64, f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a479 = v0;
                let a480 = 1u64;
                if nat_eq(a479, a480) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a481 = v0;
            let a482 = 3u64;
            if nat_lt(a481, a482) {
                let a483 = v0;
                let a484 = 2u64;
                if nat_eq(a483, a484) {
                    Ok(f45(3u64, f65(), 256u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a485 = v0;
                let a486 = 3u64;
                if nat_eq(a485, a486) {
                    Ok(f45(0u64, f57(), 1u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a487 = v0;
        let a488 = 6u64;
        if nat_lt(a487, a488) {
            let a489 = v0;
            let a490 = 5u64;
            if nat_lt(a489, a490) {
                let a491 = v0;
                let a492 = 4u64;
                if nat_eq(a491, a492) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a493 = v0;
                let a494 = 5u64;
                if nat_eq(a493, a494) {
                    Ok(f42(3u64, 1065353216u64, f40(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a495 = v0;
            let a496 = 7u64;
            if nat_lt(a495, a496) {
                let a497 = v0;
                let a498 = 6u64;
                if nat_eq(a497, a498) {
                    Ok(f43(0u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a499 = v0;
                let a500 = 7u64;
                if nat_eq(a499, a500) {
                    Ok(f42(2u64, f67(), f47(f48())?, f68()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a501 = v0;
    let a502 = 4u64;
    if nat_lt(a501, a502) {
        let a503 = v0;
        let a504 = 2u64;
        if nat_lt(a503, a504) {
            let a505 = v0;
            let a506 = 1u64;
            if nat_lt(a505, a506) {
                let a507 = v0;
                let a508 = 0u64;
                if nat_eq(a507, a508) {
                    f42(1u64, f68(), 1065353216u64, f68())
                } else {
                    f2()
                }
            } else {
                let a509 = v0;
                let a510 = 1u64;
                if nat_eq(a509, a510) {
                    f42(2u64, 1056964608u64, f68(), f68())
                } else {
                    f2()
                }
            }
        } else {
            let a511 = v0;
            let a512 = 3u64;
            if nat_lt(a511, a512) {
                let a513 = v0;
                let a514 = 2u64;
                if nat_eq(a513, a514) {
                    f42(0u64, f68(), 1262485504u64, f68())
                } else {
                    f2()
                }
            } else {
                let a515 = v0;
                let a516 = 3u64;
                if nat_eq(a515, a516) {
                    f69(2u64, f68(), 15u64, 0u64, f59())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a517 = v0;
        let a518 = 6u64;
        if nat_lt(a517, a518) {
            let a519 = v0;
            let a520 = 5u64;
            if nat_lt(a519, a520) {
                let a521 = v0;
                let a522 = 4u64;
                if nat_eq(a521, a522) {
                    f62(8u64, f48(), 0u64, 0u64, f57(), f58())
                } else {
                    f2()
                }
            } else {
                let a523 = v0;
                let a524 = 5u64;
                if nat_eq(a523, a524) {
                    f61(f57(), f58(), f59())
                } else {
                    f2()
                }
            }
        } else {
            let a525 = v0;
            let a526 = 7u64;
            if nat_lt(a525, a526) {
                let a527 = v0;
                let a528 = 6u64;
                if nat_eq(a527, a528) {
                    f45(0u64, f48(), 1u64, f48())
                } else {
                    f2()
                }
            } else {
                let a529 = v0;
                let a530 = 7u64;
                if nat_eq(a529, a530) {
                    f39(1u64, f48(), 8u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a531 = v0;
    let a532 = 1u64;
    if nat_lt(a531, a532) {
        let a533 = v0;
        let a534 = 0u64;
        if nat_eq(a533, a534) {
            f70()
        } else {
            f2()
        }
    } else {
        let a535 = v0;
        let a536 = 1u64;
        if nat_eq(a535, a536) {
            f71()
        } else {
            f2()
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a537 = v0;
    let a538 = 4u64;
    if nat_lt(a537, a538) {
        let a539 = v0;
        let a540 = 2u64;
        if nat_lt(a539, a540) {
            let a541 = v0;
            let a542 = 1u64;
            if nat_lt(a541, a542) {
                let a543 = v0;
                let a544 = 0u64;
                if nat_eq(a543, a544) {
                    Ok(f39(2u64, f40(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a545 = v0;
                let a546 = 1u64;
                if nat_eq(a545, a546) {
                    Ok(f41(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a547 = v0;
            let a548 = 3u64;
            if nat_lt(a547, a548) {
                let a549 = v0;
                let a550 = 2u64;
                if nat_eq(a549, a550) {
                    Ok(f42(6u64, f40(), 0u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a551 = v0;
                let a552 = 3u64;
                if nat_eq(a551, a552) {
                    Ok(f43(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a553 = v0;
        let a554 = 6u64;
        if nat_lt(a553, a554) {
            let a555 = v0;
            let a556 = 5u64;
            if nat_lt(a555, a556) {
                let a557 = v0;
                let a558 = 4u64;
                if nat_eq(a557, a558) {
                    Ok(f45(1u64, 255u64, f46(f44())?, f46(f44())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a559 = v0;
                let a560 = 5u64;
                if nat_eq(a559, a560) {
                    Ok(f45(0u64, f44(), 1u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a561 = v0;
            let a562 = 7u64;
            if nat_lt(a561, a562) {
                let a563 = v0;
                let a564 = 6u64;
                if nat_eq(a563, a564) {
                    Ok(f39(1u64, f44(), 2u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a565 = v0;
                let a566 = 7u64;
                if nat_eq(a565, a566) {
                    Ok(f43(f40(), f47(f48())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a567 = v0;
    let a568 = 4u64;
    if nat_lt(a567, a568) {
        let a569 = v0;
        let a570 = 2u64;
        if nat_lt(a569, a570) {
            let a571 = v0;
            let a572 = 1u64;
            if nat_lt(a571, a572) {
                let a573 = v0;
                let a574 = 0u64;
                if nat_eq(a573, a574) {
                    Ok(f39(2u64, f49(), f40(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a575 = v0;
                let a576 = 1u64;
                if nat_eq(a575, a576) {
                    Ok(f41(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a577 = v0;
            let a578 = 3u64;
            if nat_lt(a577, a578) {
                let a579 = v0;
                let a580 = 2u64;
                if nat_eq(a579, a580) {
                    Ok(f43(f40(), f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a581 = v0;
                let a582 = 3u64;
                if nat_eq(a581, a582) {
                    Ok(f43(0u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a583 = v0;
        let a584 = 6u64;
        if nat_lt(a583, a584) {
            let a585 = v0;
            let a586 = 5u64;
            if nat_lt(a585, a586) {
                let a587 = v0;
                let a588 = 4u64;
                if nat_eq(a587, a588) {
                    Ok(f45(2u64, f48(), 2u64, f54()))
                } else {
                    Ok(f2())
                }
            } else {
                let a589 = v0;
                let a590 = 5u64;
                if nat_eq(a589, a590) {
                    Ok(f45(0u64, f54(), f53(), f54()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a591 = v0;
            let a592 = 7u64;
            if nat_lt(a591, a592) {
                let a593 = v0;
                let a594 = 6u64;
                if nat_eq(a593, a594) {
                    Ok(f55(7u64, f54(), 0u64, f56(f53())?, f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a595 = v0;
                let a596 = 7u64;
                if nat_eq(a595, a596) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a597 = v0;
    let a598 = 4u64;
    if nat_lt(a597, a598) {
        let a599 = v0;
        let a600 = 2u64;
        if nat_lt(a599, a600) {
            let a601 = v0;
            let a602 = 1u64;
            if nat_lt(a601, a602) {
                let a603 = v0;
                let a604 = 0u64;
                if nat_eq(a603, a604) {
                    Ok(f55(7u64, f54(), 1u64, f56(f53())?, f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a605 = v0;
                let a606 = 1u64;
                if nat_eq(a605, a606) {
                    Ok(f61(f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a607 = v0;
            let a608 = 3u64;
            if nat_lt(a607, a608) {
                let a609 = v0;
                let a610 = 2u64;
                if nat_eq(a609, a610) {
                    Ok(f39(0u64, f52(), 0u64, 3095u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a611 = v0;
                let a612 = 3u64;
                if nat_eq(a611, a612) {
                    Ok(f45(4u64, f46(f53())?, 128u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a613 = v0;
        let a614 = 6u64;
        if nat_lt(a613, a614) {
            let a615 = v0;
            let a616 = 5u64;
            if nat_lt(a615, a616) {
                let a617 = v0;
                let a618 = 4u64;
                if nat_eq(a617, a618) {
                    Ok(f43(1u64, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a619 = v0;
                let a620 = 5u64;
                if nat_eq(a619, a620) {
                    Ok(f61(f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a621 = v0;
            let a622 = 7u64;
            if nat_lt(a621, a622) {
                let a623 = v0;
                let a624 = 6u64;
                if nat_eq(a623, a624) {
                    Ok(f41(3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a625 = v0;
                let a626 = 7u64;
                if nat_eq(a625, a626) {
                    Ok(f45(0u64, f54(), 34u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a627 = v0;
    let a628 = 3u64;
    if nat_lt(a627, a628) {
        let a629 = v0;
        let a630 = 1u64;
        if nat_lt(a629, a630) {
            let a631 = v0;
            let a632 = 0u64;
            if nat_eq(a631, a632) {
                Ok(f43(f46(f53())?, f60(f57())?))
            } else {
                Ok(f2())
            }
        } else {
            let a633 = v0;
            let a634 = 2u64;
            if nat_lt(a633, a634) {
                let a635 = v0;
                let a636 = 1u64;
                if nat_eq(a635, a636) {
                    Ok(f45(0u64, f53(), 1u64, f53()))
                } else {
                    Ok(f2())
                }
            } else {
                let a637 = v0;
                let a638 = 2u64;
                if nat_eq(a637, a638) {
                    Ok(f39(1u64, f53(), 2u64, 3084u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a639 = v0;
        let a640 = 4u64;
        if nat_lt(a639, a640) {
            let a641 = v0;
            let a642 = 3u64;
            if nat_eq(a641, a642) {
                Ok(f45(0u64, f48(), 1u64, f48()))
            } else {
                Ok(f2())
            }
        } else {
            let a643 = v0;
            let a644 = 5u64;
            if nat_lt(a643, a644) {
                let a645 = v0;
                let a646 = 4u64;
                if nat_eq(a645, a646) {
                    Ok(f39(1u64, f48(), 16u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a647 = v0;
                let a648 = 5u64;
                if nat_eq(a647, a648) {
                    Ok(f41(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a649 = v0;
    let a650 = 4u64;
    if nat_lt(a649, a650) {
        let a651 = v0;
        let a652 = 2u64;
        if nat_lt(a651, a652) {
            let a653 = v0;
            let a654 = 1u64;
            if nat_lt(a653, a654) {
                let a655 = v0;
                let a656 = 0u64;
                if nat_eq(a655, a656) {
                    f39(0u64, f49(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a657 = v0;
                let a658 = 1u64;
                if nat_eq(a657, a658) {
                    f42(3u64, f49(), 1106771968u64, f40())
                } else {
                    f2()
                }
            }
        } else {
            let a659 = v0;
            let a660 = 3u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 2u64;
                if nat_eq(a661, a662) {
                    f43(1065353216u64, f64())
                } else {
                    f2()
                }
            } else {
                let a663 = v0;
                let a664 = 3u64;
                if nat_eq(a663, a664) {
                    f39(0u64, f52(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a665 = v0;
        let a666 = 6u64;
        if nat_lt(a665, a666) {
            let a667 = v0;
            let a668 = 5u64;
            if nat_lt(a667, a668) {
                let a669 = v0;
                let a670 = 4u64;
                if nat_eq(a669, a670) {
                    f43(1065143501u64, f64())
                } else {
                    f2()
                }
            } else {
                let a671 = v0;
                let a672 = 5u64;
                if nat_eq(a671, a672) {
                    f42(2u64, f40(), f64(), f65())
                } else {
                    f2()
                }
            }
        } else {
            let a673 = v0;
            let a674 = 7u64;
            if nat_lt(a673, a674) {
                let a675 = v0;
                let a676 = 6u64;
                if nat_eq(a675, a676) {
                    f66(f65(), f65())
                } else {
                    f2()
                }
            } else {
                let a677 = v0;
                let a678 = 7u64;
                if nat_eq(a677, a678) {
                    f45(4u64, f65(), 256u64, f59())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a679 = v0;
    let a680 = 4u64;
    if nat_lt(a679, a680) {
        let a681 = v0;
        let a682 = 2u64;
        if nat_lt(a681, a682) {
            let a683 = v0;
            let a684 = 1u64;
            if nat_lt(a683, a684) {
                let a685 = v0;
                let a686 = 0u64;
                if nat_eq(a685, a686) {
                    f42(1u64, f68(), 1065353216u64, f68())
                } else {
                    f2()
                }
            } else {
                let a687 = v0;
                let a688 = 1u64;
                if nat_eq(a687, a688) {
                    f42(2u64, 1056964608u64, f68(), f68())
                } else {
                    f2()
                }
            }
        } else {
            let a689 = v0;
            let a690 = 3u64;
            if nat_lt(a689, a690) {
                let a691 = v0;
                let a692 = 2u64;
                if nat_eq(a691, a692) {
                    f42(0u64, f68(), 1262485504u64, f68())
                } else {
                    f2()
                }
            } else {
                let a693 = v0;
                let a694 = 3u64;
                if nat_eq(a693, a694) {
                    f69(2u64, f68(), 15u64, 0u64, f59())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a695 = v0;
        let a696 = 6u64;
        if nat_lt(a695, a696) {
            let a697 = v0;
            let a698 = 5u64;
            if nat_lt(a697, a698) {
                let a699 = v0;
                let a700 = 4u64;
                if nat_eq(a699, a700) {
                    f62(8u64, f48(), 0u64, 0u64, f57(), f58())
                } else {
                    f2()
                }
            } else {
                let a701 = v0;
                let a702 = 5u64;
                if nat_eq(a701, a702) {
                    f61(f57(), f58(), f59())
                } else {
                    f2()
                }
            }
        } else {
            let a703 = v0;
            let a704 = 7u64;
            if nat_lt(a703, a704) {
                let a705 = v0;
                let a706 = 6u64;
                if nat_eq(a705, a706) {
                    f45(0u64, f48(), 1u64, f48())
                } else {
                    f2()
                }
            } else {
                let a707 = v0;
                let a708 = 7u64;
                if nat_eq(a707, a708) {
                    f39(1u64, f48(), 16u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a709 = v0;
    let a710 = 4u64;
    if nat_lt(a709, a710) {
        let a711 = v0;
        let a712 = 2u64;
        if nat_lt(a711, a712) {
            let a713 = v0;
            let a714 = 1u64;
            if nat_lt(a713, a714) {
                let a715 = v0;
                let a716 = 0u64;
                if nat_eq(a715, a716) {
                    Ok(f39(2u64, f49(), f40(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a717 = v0;
                let a718 = 1u64;
                if nat_eq(a717, a718) {
                    Ok(f41(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a719 = v0;
            let a720 = 3u64;
            if nat_lt(a719, a720) {
                let a721 = v0;
                let a722 = 2u64;
                if nat_eq(a721, a722) {
                    Ok(f43(f40(), f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a723 = v0;
                let a724 = 3u64;
                if nat_eq(a723, a724) {
                    Ok(f43(0u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a725 = v0;
        let a726 = 6u64;
        if nat_lt(a725, a726) {
            let a727 = v0;
            let a728 = 5u64;
            if nat_lt(a727, a728) {
                let a729 = v0;
                let a730 = 4u64;
                if nat_eq(a729, a730) {
                    Ok(f55(7u64, f48(), f53(), f56(f53())?, f57(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a731 = v0;
                let a732 = 5u64;
                if nat_eq(a731, a732) {
                    Ok(f43(f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a733 = v0;
            let a734 = 7u64;
            if nat_lt(a733, a734) {
                let a735 = v0;
                let a736 = 6u64;
                if nat_eq(a735, a736) {
                    Ok(f62(9u64, f48(), f53(), 0u64, f57(), f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a737 = v0;
                let a738 = 7u64;
                if nat_eq(a737, a738) {
                    Ok(f45(4u64, f46(f53())?, 128u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a739 = v0;
    let a740 = 3u64;
    if nat_lt(a739, a740) {
        let a741 = v0;
        let a742 = 1u64;
        if nat_lt(a741, a742) {
            let a743 = v0;
            let a744 = 0u64;
            if nat_eq(a743, a744) {
                f63(f57(), f58(), f59())
            } else {
                f2()
            }
        } else {
            let a745 = v0;
            let a746 = 2u64;
            if nat_lt(a745, a746) {
                let a747 = v0;
                let a748 = 1u64;
                if nat_eq(a747, a748) {
                    f45(0u64, f53(), 1u64, f53())
                } else {
                    f2()
                }
            } else {
                let a749 = v0;
                let a750 = 2u64;
                if nat_eq(a749, a750) {
                    f39(1u64, f53(), 4u64, 3084u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a751 = v0;
        let a752 = 4u64;
        if nat_lt(a751, a752) {
            let a753 = v0;
            let a754 = 3u64;
            if nat_eq(a753, a754) {
                f45(0u64, f48(), 1u64, f48())
            } else {
                f2()
            }
        } else {
            let a755 = v0;
            let a756 = 5u64;
            if nat_lt(a755, a756) {
                let a757 = v0;
                let a758 = 4u64;
                if nat_eq(a757, a758) {
                    f39(1u64, f48(), 8u64, 512u64)
                } else {
                    f2()
                }
            } else {
                let a759 = v0;
                let a760 = 5u64;
                if nat_eq(a759, a760) {
                    f41(3584u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f43(0u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a769 = v0;
                let a770 = 1u64;
                if nat_eq(a769, a770) {
                    Ok(f43(0u64, f73()))
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
                    Ok(f43(0u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a775 = v0;
                let a776 = 3u64;
                if nat_eq(a775, a776) {
                    Ok(f42(2u64, f67(), f74(f48())?, f68()))
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
                    Ok(f42(1u64, f68(), 1065353216u64, f68()))
                } else {
                    Ok(f2())
                }
            } else {
                let a783 = v0;
                let a784 = 5u64;
                if nat_eq(a783, a784) {
                    Ok(f42(2u64, 1056964608u64, f68(), f68()))
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
                    Ok(f42(0u64, f68(), 1262485504u64, f68()))
                } else {
                    Ok(f2())
                }
            } else {
                let a789 = v0;
                let a790 = 7u64;
                if nat_eq(a789, a790) {
                    Ok(f69(2u64, f68(), 7u64, 0u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f39(0u64, f52(), 1u64, 3086u64)
                } else {
                    f2()
                }
            } else {
                let a799 = v0;
                let a800 = 1u64;
                if nat_eq(a799, a800) {
                    f45(2u64, f48(), 2u64, f57())
                } else {
                    f2()
                }
            }
        } else {
            let a801 = v0;
            let a802 = 3u64;
            if nat_lt(a801, a802) {
                let a803 = v0;
                let a804 = 2u64;
                if nat_eq(a803, a804) {
                    f45(0u64, f57(), 35u64, f57())
                } else {
                    f2()
                }
            } else {
                let a805 = v0;
                let a806 = 3u64;
                if nat_eq(a805, a806) {
                    f43(4u64, f58())
                } else {
                    f2()
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
                    f61(f57(), f58(), f59())
                } else {
                    f2()
                }
            } else {
                let a813 = v0;
                let a814 = 5u64;
                if nat_eq(a813, a814) {
                    f41(3134u64)
                } else {
                    f2()
                }
            }
        } else {
            let a815 = v0;
            let a816 = 7u64;
            if nat_lt(a815, a816) {
                let a817 = v0;
                let a818 = 6u64;
                if nat_eq(a817, a818) {
                    f43(f59(), f75())
                } else {
                    f2()
                }
            } else {
                let a819 = v0;
                let a820 = 7u64;
                if nat_eq(a819, a820) {
                    f62(8u64, f48(), 0u64, 0u64, f57(), f76())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f45(2u64, f59(), f76(), f68()))
                } else {
                    Ok(f2())
                }
            } else {
                let a829 = v0;
                let a830 = 1u64;
                if nat_eq(a829, a830) {
                    Ok(f45(4u64, f68(), 256u64, f59()))
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
                    Ok(f45(0u64, f60(f57())?, f59(), f60(f57())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a835 = v0;
                let a836 = 3u64;
                if nat_eq(a835, a836) {
                    Ok(f45(0u64, f57(), 1u64, f57()))
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
                    Ok(f45(3u64, f68(), 256u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a843 = v0;
                let a844 = 5u64;
                if nat_eq(a843, a844) {
                    Ok(f45(0u64, f60(f57())?, f59(), f60(f57())?))
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
                    Ok(f45(2u64, f75(), 2u64, f75()))
                } else {
                    Ok(f2())
                }
            } else {
                let a849 = v0;
                let a850 = 7u64;
                if nat_eq(a849, a850) {
                    Ok(f45(0u64, f75(), 1u64, f75()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a851 = v0;
    let a852 = 4u64;
    if nat_lt(a851, a852) {
        let a853 = v0;
        let a854 = 2u64;
        if nat_lt(a853, a854) {
            let a855 = v0;
            let a856 = 1u64;
            if nat_lt(a855, a856) {
                let a857 = v0;
                let a858 = 0u64;
                if nat_eq(a857, a858) {
                    Ok(f77(f75(), f75()))
                } else {
                    Ok(f2())
                }
            } else {
                let a859 = v0;
                let a860 = 1u64;
                if nat_eq(a859, a860) {
                    Ok(f45(2u64, f48(), f78(), f79()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a861 = v0;
            let a862 = 3u64;
            if nat_lt(a861, a862) {
                let a863 = v0;
                let a864 = 2u64;
                if nat_eq(a863, a864) {
                    Ok(f43(0u64, f53()))
                } else {
                    Ok(f2())
                }
            } else {
                let a865 = v0;
                let a866 = 3u64;
                if nat_eq(a865, a866) {
                    Ok(f45(2u64, f48(), 2u64, f80()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a867 = v0;
        let a868 = 6u64;
        if nat_lt(a867, a868) {
            let a869 = v0;
            let a870 = 5u64;
            if nat_lt(a869, a870) {
                let a871 = v0;
                let a872 = 4u64;
                if nat_eq(a871, a872) {
                    Ok(f45(0u64, f80(), f53(), f80()))
                } else {
                    Ok(f2())
                }
            } else {
                let a873 = v0;
                let a874 = 5u64;
                if nat_eq(a873, a874) {
                    Ok(f43(f60(f80())?, f81()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a875 = v0;
            let a876 = 7u64;
            if nat_lt(a875, a876) {
                let a877 = v0;
                let a878 = 6u64;
                if nat_eq(a877, a878) {
                    Ok(f45(0u64, f48(), 32u64, f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a879 = v0;
                let a880 = 7u64;
                if nat_eq(a879, a880) {
                    Ok(f43(f60(f57())?, f82()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a881 = v0;
    let a882 = 4u64;
    if nat_lt(a881, a882) {
        let a883 = v0;
        let a884 = 2u64;
        if nat_lt(a883, a884) {
            let a885 = v0;
            let a886 = 1u64;
            if nat_lt(a885, a886) {
                let a887 = v0;
                let a888 = 0u64;
                if nat_eq(a887, a888) {
                    f39(0u64, f53(), 0u64, 3106u64)
                } else {
                    f2()
                }
            } else {
                let a889 = v0;
                let a890 = 1u64;
                if nat_eq(a889, a890) {
                    f45(3u64, f82(), 16u64, f82())
                } else {
                    f2()
                }
            }
        } else {
            let a891 = v0;
            let a892 = 3u64;
            if nat_lt(a891, a892) {
                let a893 = v0;
                let a894 = 2u64;
                if nat_eq(a893, a894) {
                    f45(4u64, f82(), 8u64, f68())
                } else {
                    f2()
                }
            } else {
                let a895 = v0;
                let a896 = 3u64;
                if nat_eq(a895, a896) {
                    f45(2u64, f68(), 256u64, f68())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a897 = v0;
        let a898 = 6u64;
        if nat_lt(a897, a898) {
            let a899 = v0;
            let a900 = 5u64;
            if nat_lt(a899, a900) {
                let a901 = v0;
                let a902 = 4u64;
                if nat_eq(a901, a902) {
                    f45(0u64, f68(), f81(), f81())
                } else {
                    f2()
                }
            } else {
                let a903 = v0;
                let a904 = 5u64;
                if nat_eq(a903, a904) {
                    f45(3u64, f82(), 8u64, f68())
                } else {
                    f2()
                }
            }
        } else {
            let a905 = v0;
            let a906 = 7u64;
            if nat_lt(a905, a906) {
                let a907 = v0;
                let a908 = 6u64;
                if nat_eq(a907, a908) {
                    f45(4u64, f68(), 2u64, f83())
                } else {
                    f2()
                }
            } else {
                let a909 = v0;
                let a910 = 7u64;
                if nat_eq(a909, a910) {
                    f43(0u64, f84())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f32(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a911 = v0;
    let a912 = 4u64;
    if nat_lt(a911, a912) {
        let a913 = v0;
        let a914 = 2u64;
        if nat_lt(a913, a914) {
            let a915 = v0;
            let a916 = 1u64;
            if nat_lt(a915, a916) {
                let a917 = v0;
                let a918 = 0u64;
                if nat_eq(a917, a918) {
                    Ok(f45(2u64, f53(), 8u64, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a919 = v0;
                let a920 = 1u64;
                if nat_eq(a919, a920) {
                    Ok(f45(0u64, f85(), f84(), f86()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a921 = v0;
            let a922 = 3u64;
            if nat_lt(a921, a922) {
                let a923 = v0;
                let a924 = 2u64;
                if nat_eq(a923, a924) {
                    Ok(f45(0u64, f79(), f86(), f87()))
                } else {
                    Ok(f2())
                }
            } else {
                let a925 = v0;
                let a926 = 3u64;
                if nat_eq(a925, a926) {
                    Ok(f42(2u64, f88(f87())?, f88(f87())?, f89()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a927 = v0;
        let a928 = 6u64;
        if nat_lt(a927, a928) {
            let a929 = v0;
            let a930 = 5u64;
            if nat_lt(a929, a930) {
                let a931 = v0;
                let a932 = 4u64;
                if nat_eq(a931, a932) {
                    Ok(f39(0u64, f90(), 0u64, 3120u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a933 = v0;
                let a934 = 5u64;
                if nat_eq(a933, a934) {
                    Ok(f42(0u64, f91(), f89(), f89()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a935 = v0;
            let a936 = 7u64;
            if nat_lt(a935, a936) {
                let a937 = v0;
                let a938 = 6u64;
                if nat_eq(a937, a938) {
                    Ok(f42(4u64, f89(), 0u64, f89()))
                } else {
                    Ok(f2())
                }
            } else {
                let a939 = v0;
                let a940 = 7u64;
                if nat_eq(a939, a940) {
                    Ok(f42(2u64, f92(f87())?, f89(), f89()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a941 = v0;
    let a942 = 4u64;
    if nat_lt(a941, a942) {
        let a943 = v0;
        let a944 = 2u64;
        if nat_lt(a943, a944) {
            let a945 = v0;
            let a946 = 1u64;
            if nat_lt(a945, a946) {
                let a947 = v0;
                let a948 = 0u64;
                if nat_eq(a947, a948) {
                    Ok(f62(4u64, 0u64, f81(), 0u64, f68(), f93()))
                } else {
                    Ok(f2())
                }
            } else {
                let a949 = v0;
                let a950 = 1u64;
                if nat_eq(a949, a950) {
                    Ok(f45(0u64, f93(), f84(), f93()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a951 = v0;
            let a952 = 3u64;
            if nat_lt(a951, a952) {
                let a953 = v0;
                let a954 = 2u64;
                if nat_eq(a953, a954) {
                    Ok(f69(5u64, f94(f93())?, f83(), 0u64, f95()))
                } else {
                    Ok(f2())
                }
            } else {
                let a955 = v0;
                let a956 = 3u64;
                if nat_eq(a955, a956) {
                    Ok(f42(2u64, f95(), f75(), f95()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a957 = v0;
        let a958 = 6u64;
        if nat_lt(a957, a958) {
            let a959 = v0;
            let a960 = 5u64;
            if nat_lt(a959, a960) {
                let a961 = v0;
                let a962 = 4u64;
                if nat_eq(a961, a962) {
                    Ok(f42(2u64, f89(), f95(), f65()))
                } else {
                    Ok(f2())
                }
            } else {
                let a963 = v0;
                let a964 = 5u64;
                if nat_eq(a963, a964) {
                    Ok(f42(2u64, f65(), f88(f87())?, f65()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a965 = v0;
            let a966 = 7u64;
            if nat_lt(a965, a966) {
                let a967 = v0;
                let a968 = 6u64;
                if nat_eq(a967, a968) {
                    Ok(f42(0u64, f72(), f65(), f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a969 = v0;
                let a970 = 7u64;
                if nat_eq(a969, a970) {
                    Ok(f42(2u64, f89(), f95(), f65()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a971 = v0;
    let a972 = 4u64;
    if nat_lt(a971, a972) {
        let a973 = v0;
        let a974 = 2u64;
        if nat_lt(a973, a974) {
            let a975 = v0;
            let a976 = 1u64;
            if nat_lt(a975, a976) {
                let a977 = v0;
                let a978 = 0u64;
                if nat_eq(a977, a978) {
                    f42(2u64, f65(), f95(), f65())
                } else {
                    f2()
                }
            } else {
                let a979 = v0;
                let a980 = 1u64;
                if nat_eq(a979, a980) {
                    f42(0u64, f73(), f65(), f73())
                } else {
                    f2()
                }
            }
        } else {
            let a981 = v0;
            let a982 = 3u64;
            if nat_lt(a981, a982) {
                let a983 = v0;
                let a984 = 2u64;
                if nat_eq(a983, a984) {
                    f45(0u64, f84(), 1u64, f84())
                } else {
                    f2()
                }
            } else {
                let a985 = v0;
                let a986 = 3u64;
                if nat_eq(a985, a986) {
                    f39(1u64, f84(), 8u64, 3113u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a987 = v0;
        let a988 = 6u64;
        if nat_lt(a987, a988) {
            let a989 = v0;
            let a990 = 5u64;
            if nat_lt(a989, a990) {
                let a991 = v0;
                let a992 = 4u64;
                if nat_eq(a991, a992) {
                    f45(0u64, f53(), 1u64, f53())
                } else {
                    f2()
                }
            } else {
                let a993 = v0;
                let a994 = 5u64;
                if nat_eq(a993, a994) {
                    f39(1u64, f53(), f50(), 3099u64)
                } else {
                    f2()
                }
            }
        } else {
            let a995 = v0;
            let a996 = 7u64;
            if nat_lt(a995, a996) {
                let a997 = v0;
                let a998 = 6u64;
                if nat_eq(a997, a998) {
                    f45(0u64, f48(), 1u64, f48())
                } else {
                    f2()
                }
            } else {
                let a999 = v0;
                let a1000 = 7u64;
                if nat_eq(a999, a1000) {
                    f39(1u64, f48(), f96(), 3075u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f35(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1001 = v0;
    let a1002 = 4u64;
    if nat_lt(a1001, a1002) {
        let a1003 = v0;
        let a1004 = 2u64;
        if nat_lt(a1003, a1004) {
            let a1005 = v0;
            let a1006 = 1u64;
            if nat_lt(a1005, a1006) {
                let a1007 = v0;
                let a1008 = 0u64;
                if nat_eq(a1007, a1008) {
                    f39(0u64, f52(), 0u64, 3150u64)
                } else {
                    f2()
                }
            } else {
                let a1009 = v0;
                let a1010 = 1u64;
                if nat_eq(a1009, a1010) {
                    f39(2u64, 0u64, f73(), 3139u64)
                } else {
                    f2()
                }
            }
        } else {
            let a1011 = v0;
            let a1012 = 3u64;
            if nat_lt(a1011, a1012) {
                let a1013 = v0;
                let a1014 = 2u64;
                if nat_eq(a1013, a1014) {
                    f41(3140u64)
                } else {
                    f2()
                }
            } else {
                let a1015 = v0;
                let a1016 = 3u64;
                if nat_eq(a1015, a1016) {
                    f42(3u64, f72(), f73(), f40())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1017 = v0;
        let a1018 = 6u64;
        if nat_lt(a1017, a1018) {
            let a1019 = v0;
            let a1020 = 5u64;
            if nat_lt(a1019, a1020) {
                let a1021 = v0;
                let a1022 = 4u64;
                if nat_eq(a1021, a1022) {
                    f42(2u64, f40(), 1066296934u64, f68())
                } else {
                    f2()
                }
            } else {
                let a1023 = v0;
                let a1024 = 5u64;
                if nat_eq(a1023, a1024) {
                    f66(f68(), f68())
                } else {
                    f2()
                }
            }
        } else {
            let a1025 = v0;
            let a1026 = 7u64;
            if nat_lt(a1025, a1026) {
                let a1027 = v0;
                let a1028 = 6u64;
                if nat_eq(a1027, a1028) {
                    f43(49u64, f57())
                } else {
                    f2()
                }
            } else {
                let a1029 = v0;
                let a1030 = 7u64;
                if nat_eq(a1029, a1030) {
                    f45(4u64, f68(), 16u64, f59())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f36(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1031 = v0;
    let a1032 = 4u64;
    if nat_lt(a1031, a1032) {
        let a1033 = v0;
        let a1034 = 2u64;
        if nat_lt(a1033, a1034) {
            let a1035 = v0;
            let a1036 = 1u64;
            if nat_lt(a1035, a1036) {
                let a1037 = v0;
                let a1038 = 0u64;
                if nat_eq(a1037, a1038) {
                    f43(4u64, f58())
                } else {
                    f2()
                }
            } else {
                let a1039 = v0;
                let a1040 = 1u64;
                if nat_eq(a1039, a1040) {
                    f61(f57(), f58(), f59())
                } else {
                    f2()
                }
            }
        } else {
            let a1041 = v0;
            let a1042 = 3u64;
            if nat_lt(a1041, a1042) {
                let a1043 = v0;
                let a1044 = 2u64;
                if nat_eq(a1043, a1044) {
                    f45(3u64, f68(), 16u64, f68())
                } else {
                    f2()
                }
            } else {
                let a1045 = v0;
                let a1046 = 3u64;
                if nat_eq(a1045, a1046) {
                    f45(0u64, f57(), 2u64, f57())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a1047 = v0;
        let a1048 = 6u64;
        if nat_lt(a1047, a1048) {
            let a1049 = v0;
            let a1050 = 5u64;
            if nat_lt(a1049, a1050) {
                let a1051 = v0;
                let a1052 = 4u64;
                if nat_eq(a1051, a1052) {
                    f39(1u64, f57(), 56u64, 3143u64)
                } else {
                    f2()
                }
            } else {
                let a1053 = v0;
                let a1054 = 5u64;
                if nat_eq(a1053, a1054) {
                    f70()
                } else {
                    f2()
                }
            }
        } else {
            let a1055 = v0;
            let a1056 = 7u64;
            if nat_lt(a1055, a1056) {
                let a1057 = v0;
                let a1058 = 6u64;
                if nat_eq(a1057, a1058) {
                    f42(2u64, f40(), 1066401792u64, f68())
                } else {
                    f2()
                }
            } else {
                let a1059 = v0;
                let a1060 = 7u64;
                if nat_eq(a1059, a1060) {
                    f66(f68(), f68())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f37(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1061 = v0;
    let a1062 = 2u64;
    if nat_lt(a1061, a1062) {
        let a1063 = v0;
        let a1064 = 1u64;
        if nat_lt(a1063, a1064) {
            let a1065 = v0;
            let a1066 = 0u64;
            if nat_eq(a1065, a1066) {
                Ok(f43(0u64, f57()))
            } else {
                Ok(f2())
            }
        } else {
            let a1067 = v0;
            let a1068 = 1u64;
            if nat_eq(a1067, a1068) {
                Ok(f45(4u64, f68(), 256u64, f60(f57())?))
            } else {
                Ok(f2())
            }
        }
    } else {
        let a1069 = v0;
        let a1070 = 3u64;
        if nat_lt(a1069, a1070) {
            let a1071 = v0;
            let a1072 = 2u64;
            if nat_eq(a1071, a1072) {
                Ok(f45(0u64, f57(), 1u64, f57()))
            } else {
                Ok(f2())
            }
        } else {
            let a1073 = v0;
            let a1074 = 4u64;
            if nat_lt(a1073, a1074) {
                let a1075 = v0;
                let a1076 = 3u64;
                if nat_eq(a1075, a1076) {
                    Ok(f45(3u64, f68(), 256u64, f60(f57())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1077 = v0;
                let a1078 = 4u64;
                if nat_eq(a1077, a1078) {
                    Ok(f70())
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f38(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a1079 = v0;
    let a1080 = 2u64;
    if nat_lt(a1079, a1080) {
        let a1081 = v0;
        let a1082 = 1u64;
        if nat_lt(a1081, a1082) {
            let a1083 = v0;
            let a1084 = 0u64;
            if nat_eq(a1083, a1084) {
                f39(0u64, f49(), 0u64, 3588u64)
            } else {
                f2()
            }
        } else {
            let a1085 = v0;
            let a1086 = 1u64;
            if nat_eq(a1085, a1086) {
                f42(3u64, f49(), 1097859072u64, f40())
            } else {
                f2()
            }
        }
    } else {
        let a1087 = v0;
        let a1088 = 3u64;
        if nat_lt(a1087, a1088) {
            let a1089 = v0;
            let a1090 = 2u64;
            if nat_eq(a1089, a1090) {
                f42(3u64, 1065353216u64, f40(), f67())
            } else {
                f2()
            }
        } else {
            let a1091 = v0;
            let a1092 = 4u64;
            if nat_lt(a1091, a1092) {
                let a1093 = v0;
                let a1094 = 3u64;
                if nat_eq(a1093, a1094) {
                    f41(3072u64)
                } else {
                    f2()
                }
            } else {
                let a1095 = v0;
                let a1096 = 4u64;
                if nat_eq(a1095, a1096) {
                    f71()
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f40() -> u64 {
    4294967316u64
}

pub fn f41(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f43(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f44() -> u64 {
    4294967304u64
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f46(v0: u64) -> R<u64> {
    let a1100 = 8589975552u64;
    let a1101 = {
        let a1097 = v0;
        let a1098 = 4294967296u64;
        let a1099 = 0u64;
        nat_rem(a1097, a1098, a1099)
    };
    nat_add(a1100, a1101)
}

pub fn f47(v0: u64) -> R<u64> {
    let a1105 = 8589984768u64;
    let a1106 = {
        let a1102 = v0;
        let a1103 = 4294967296u64;
        let a1104 = 0u64;
        nat_rem(a1102, a1103, a1104)
    };
    nat_add(a1105, a1106)
}

pub fn f48() -> u64 {
    4294967301u64
}

pub fn f49() -> u64 {
    4294967347u64
}

pub fn f50() -> u64 {
    4294967302u64
}

pub fn f51() -> u64 {
    4294967346u64
}

pub fn f52() -> u64 {
    4294967297u64
}

pub fn f53() -> u64 {
    4294967323u64
}

pub fn f54() -> u64 {
    4294967341u64
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f56(v0: u64) -> R<u64> {
    let a1110 = 8589980672u64;
    let a1111 = {
        let a1107 = v0;
        let a1108 = 4294967296u64;
        let a1109 = 0u64;
        nat_rem(a1107, a1108, a1109)
    };
    nat_add(a1110, a1111)
}

pub fn f57() -> u64 {
    4294967344u64
}

pub fn f58() -> u64 {
    4294967345u64
}

pub fn f59() -> u64 {
    4294967331u64
}

pub fn f60(v0: u64) -> R<u64> {
    let a1115 = 30064771072u64;
    let a1116 = {
        let a1112 = v0;
        let a1113 = 4294967296u64;
        let a1114 = 0u64;
        nat_rem(a1112, a1113, a1114)
    };
    nat_add(a1115, a1116)
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (14u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (15u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f64() -> u64 {
    4294967357u64
}

pub fn f65() -> u64 {
    4294967337u64
}

pub fn f66(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (7u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f67() -> u64 {
    4294967319u64
}

pub fn f68() -> u64 {
    4294967338u64
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f70() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (1u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f71() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (2u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f72() -> u64 {
    4294967335u64
}

pub fn f73() -> u64 {
    4294967336u64
}

pub fn f74(v0: u64) -> R<u64> {
    let a1120 = 8589992960u64;
    let a1121 = {
        let a1117 = v0;
        let a1118 = 4294967296u64;
        let a1119 = 0u64;
        nat_rem(a1117, a1118, a1119)
    };
    nat_add(a1120, a1121)
}

pub fn f75() -> u64 {
    4294967356u64
}

pub fn f76() -> u64 {
    4294967306u64
}

pub fn f77(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f78() -> u64 {
    4294967299u64
}

pub fn f79() -> u64 {
    4294967354u64
}

pub fn f80() -> u64 {
    4294967343u64
}

pub fn f81() -> u64 {
    4294967321u64
}

pub fn f82() -> u64 {
    4294967342u64
}

pub fn f83() -> u64 {
    4294967314u64
}

pub fn f84() -> u64 {
    4294967305u64
}

pub fn f85() -> u64 {
    4294967325u64
}

pub fn f86() -> u64 {
    4294967303u64
}

pub fn f87() -> u64 {
    4294967307u64
}

pub fn f88(v0: u64) -> R<u64> {
    let a1125 = 12884901888u64;
    let a1126 = {
        let a1122 = v0;
        let a1123 = 4294967296u64;
        let a1124 = 0u64;
        nat_rem(a1122, a1123, a1124)
    };
    nat_add(a1125, a1126)
}

pub fn f89() -> u64 {
    4294967309u64
}

pub fn f90() -> u64 {
    4294967298u64
}

pub fn f91() -> u64 {
    4294967348u64
}

pub fn f92(v0: u64) -> R<u64> {
    let a1130 = 17179869184u64;
    let a1131 = {
        let a1127 = v0;
        let a1128 = 4294967296u64;
        let a1129 = 0u64;
        nat_rem(a1127, a1128, a1129)
    };
    nat_add(a1130, a1131)
}

pub fn f93() -> u64 {
    4294967330u64
}

pub fn f94(v0: u64) -> R<u64> {
    let a1135 = 25769803776u64;
    let a1136 = {
        let a1132 = v0;
        let a1133 = 4294967296u64;
        let a1134 = 0u64;
        nat_rem(a1132, a1133, a1134)
    };
    nat_add(a1135, a1136)
}

pub fn f95() -> u64 {
    4294967332u64
}

pub fn f96() -> u64 {
    4294967358u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
