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
                f9({
                    let a37 = v0;
                    let a38 = 8u64;
                    let a39 = 0u64;
                    nat_rem(a37, a38, a39)
                })
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
            let a48 = 3u64;
            if nat_eq(a47, a48) {
                Ok(f11({
                    let a49 = v0;
                    let a50 = 8u64;
                    let a51 = 0u64;
                    nat_rem(a49, a50, a51)
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
        let a52 = v0;
        let a53 = 8u64;
        let a54 = 0u64;
        nat_quot(a52, a53, a54)
    };
    let a55 = v1;
    let a56 = 1u64;
    if nat_lt(a55, a56) {
        let a57 = v1;
        let a58 = 0u64;
        if nat_eq(a57, a58) {
            f12({
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
        let a63 = 2u64;
        if nat_lt(a62, a63) {
            let a64 = v1;
            let a65 = 1u64;
            if nat_eq(a64, a65) {
                f13({
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
            let a70 = 2u64;
            if nat_eq(a69, a70) {
                Ok(f14({
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

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a74 = v0;
        let a75 = 8u64;
        let a76 = 0u64;
        nat_quot(a74, a75, a76)
    };
    let a77 = v1;
    let a78 = 2u64;
    if nat_lt(a77, a78) {
        let a79 = v1;
        let a80 = 1u64;
        if nat_lt(a79, a80) {
            let a81 = v1;
            let a82 = 0u64;
            if nat_eq(a81, a82) {
                f8({
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
            let a87 = 1u64;
            if nat_eq(a86, a87) {
                f9({
                    let a88 = v0;
                    let a89 = 8u64;
                    let a90 = 0u64;
                    nat_rem(a88, a89, a90)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a91 = v1;
        let a92 = 3u64;
        if nat_lt(a91, a92) {
            let a93 = v1;
            let a94 = 2u64;
            if nat_eq(a93, a94) {
                f15({
                    let a95 = v0;
                    let a96 = 8u64;
                    let a97 = 0u64;
                    nat_rem(a95, a96, a97)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a98 = v1;
            let a99 = 3u64;
            if nat_eq(a98, a99) {
                Ok(f11({
                    let a100 = v0;
                    let a101 = 8u64;
                    let a102 = 0u64;
                    nat_rem(a100, a101, a102)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a103 = v0;
        let a104 = 8u64;
        let a105 = 0u64;
        nat_quot(a103, a104, a105)
    };
    let a106 = v1;
    let a107 = 1u64;
    if nat_lt(a106, a107) {
        let a108 = v1;
        let a109 = 0u64;
        if nat_eq(a108, a109) {
            f16({
                let a110 = v0;
                let a111 = 8u64;
                let a112 = 0u64;
                nat_rem(a110, a111, a112)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a113 = v1;
        let a114 = 2u64;
        if nat_lt(a113, a114) {
            let a115 = v1;
            let a116 = 1u64;
            if nat_eq(a115, a116) {
                f17({
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
            let a121 = 2u64;
            if nat_eq(a120, a121) {
                Ok(f18({
                    let a122 = v0;
                    let a123 = 8u64;
                    let a124 = 0u64;
                    nat_rem(a122, a123, a124)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                f8({
                    let a134 = v0;
                    let a135 = 8u64;
                    let a136 = 0u64;
                    nat_rem(a134, a135, a136)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a137 = v1;
            let a138 = 1u64;
            if nat_eq(a137, a138) {
                f9({
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
                f19({
                    let a146 = v0;
                    let a147 = 8u64;
                    let a148 = 0u64;
                    nat_rem(a146, a147, a148)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a149 = v1;
            let a150 = 3u64;
            if nat_eq(a149, a150) {
                Ok(f11({
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

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
            f20({
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
                Ok(f21({
                    let a168 = v0;
                    let a169 = 8u64;
                    let a170 = 0u64;
                    nat_rem(a168, a169, a170)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a171 = v1;
            let a172 = 2u64;
            if nat_eq(a171, a172) {
                Ok(f22({
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

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a176 = v0;
    let a177 = 4u64;
    if nat_lt(a176, a177) {
        let a178 = v0;
        let a179 = 2u64;
        if nat_lt(a178, a179) {
            let a180 = v0;
            let a181 = 1u64;
            if nat_lt(a180, a181) {
                let a182 = v0;
                let a183 = 0u64;
                if nat_eq(a182, a183) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a184 = v0;
                let a185 = 1u64;
                if nat_eq(a184, a185) {
                    Ok(f23(0u64, f25()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a186 = v0;
            let a187 = 3u64;
            if nat_lt(a186, a187) {
                let a188 = v0;
                let a189 = 2u64;
                if nat_eq(a188, a189) {
                    Ok(f23(0u64, f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a190 = v0;
                let a191 = 3u64;
                if nat_eq(a190, a191) {
                    Ok(f23(0u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a192 = v0;
        let a193 = 6u64;
        if nat_lt(a192, a193) {
            let a194 = v0;
            let a195 = 5u64;
            if nat_lt(a194, a195) {
                let a196 = v0;
                let a197 = 4u64;
                if nat_eq(a196, a197) {
                    Ok(f28(0u64, f29(), 1u64, 2055u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a198 = v0;
                let a199 = 5u64;
                if nat_eq(a198, a199) {
                    Ok(f23(f30(f24())?, f31()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a200 = v0;
            let a201 = 7u64;
            if nat_lt(a200, a201) {
                let a202 = v0;
                let a203 = 6u64;
                if nat_eq(a202, a203) {
                    Ok(f32(2056u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a204 = v0;
                let a205 = 7u64;
                if nat_eq(a204, a205) {
                    Ok(f23(f33(f24())?, f31()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a206 = v0;
    let a207 = 4u64;
    if nat_lt(a206, a207) {
        let a208 = v0;
        let a209 = 2u64;
        if nat_lt(a208, a209) {
            let a210 = v0;
            let a211 = 1u64;
            if nat_lt(a210, a211) {
                let a212 = v0;
                let a213 = 0u64;
                if nat_eq(a212, a213) {
                    Ok(f23(0u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a214 = v0;
                let a215 = 1u64;
                if nat_eq(a214, a215) {
                    Ok(f35(4u64, 0u64, f31(), 0u64, f36(), f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a216 = v0;
            let a217 = 3u64;
            if nat_lt(a216, a217) {
                let a218 = v0;
                let a219 = 2u64;
                if nat_eq(a218, a219) {
                    Ok(f38(0u64, f37(), f34(), f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a220 = v0;
                let a221 = 3u64;
                if nat_eq(a220, a221) {
                    Ok(f39(5u64, f40(f37())?, 0u64, 0u64, f41()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a222 = v0;
        let a223 = 6u64;
        if nat_lt(a222, a223) {
            let a224 = v0;
            let a225 = 5u64;
            if nat_lt(a224, a225) {
                let a226 = v0;
                let a227 = 4u64;
                if nat_eq(a226, a227) {
                    Ok(f42(f41(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a228 = v0;
                let a229 = 5u64;
                if nat_eq(a228, a229) {
                    Ok(f43(2u64, f44(f27())?, f45(f27())?, f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a230 = v0;
            let a231 = 7u64;
            if nat_lt(a230, a231) {
                let a232 = v0;
                let a233 = 6u64;
                if nat_eq(a232, a233) {
                    Ok(f43(2u64, f46(), f41(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a234 = v0;
                let a235 = 7u64;
                if nat_eq(a234, a235) {
                    Ok(f43(0u64, f25(), f46(), f25()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a236 = v0;
    let a237 = 4u64;
    if nat_lt(a236, a237) {
        let a238 = v0;
        let a239 = 2u64;
        if nat_lt(a238, a239) {
            let a240 = v0;
            let a241 = 1u64;
            if nat_lt(a240, a241) {
                let a242 = v0;
                let a243 = 0u64;
                if nat_eq(a242, a243) {
                    Ok(f43(2u64, f44(f27())?, f41(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a244 = v0;
                let a245 = 1u64;
                if nat_eq(a244, a245) {
                    Ok(f43(2u64, f46(), f41(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a246 = v0;
            let a247 = 3u64;
            if nat_lt(a246, a247) {
                let a248 = v0;
                let a249 = 2u64;
                if nat_eq(a248, a249) {
                    Ok(f43(0u64, f26(), f46(), f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a250 = v0;
                let a251 = 3u64;
                if nat_eq(a250, a251) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a252 = v0;
        let a253 = 6u64;
        if nat_lt(a252, a253) {
            let a254 = v0;
            let a255 = 5u64;
            if nat_lt(a254, a255) {
                let a256 = v0;
                let a257 = 4u64;
                if nat_eq(a256, a257) {
                    Ok(f38(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a258 = v0;
                let a259 = 5u64;
                if nat_eq(a258, a259) {
                    Ok(f28(1u64, f34(), 4u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a260 = v0;
            let a261 = 7u64;
            if nat_lt(a260, a261) {
                let a262 = v0;
                let a263 = 6u64;
                if nat_eq(a262, a263) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a264 = v0;
                let a265 = 7u64;
                if nat_eq(a264, a265) {
                    Ok(f28(1u64, f24(), 8u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a266 = v0;
    let a267 = 0u64;
    if nat_eq(a266, a267) {
        f47(f48())
    } else {
        f2()
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a268 = v0;
    let a269 = 4u64;
    if nat_lt(a268, a269) {
        let a270 = v0;
        let a271 = 2u64;
        if nat_lt(a270, a271) {
            let a272 = v0;
            let a273 = 1u64;
            if nat_lt(a272, a273) {
                let a274 = v0;
                let a275 = 0u64;
                if nat_eq(a274, a275) {
                    Ok(f28(2u64, 0u64, f49(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a276 = v0;
                let a277 = 1u64;
                if nat_eq(a276, a277) {
                    Ok(f32(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a278 = v0;
            let a279 = 3u64;
            if nat_lt(a278, a279) {
                let a280 = v0;
                let a281 = 2u64;
                if nat_eq(a280, a281) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a282 = v0;
                let a283 = 3u64;
                if nat_eq(a282, a283) {
                    Ok(f28(0u64, f50(f24())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a284 = v0;
        let a285 = 6u64;
        if nat_lt(a284, a285) {
            let a286 = v0;
            let a287 = 5u64;
            if nat_lt(a286, a287) {
                let a288 = v0;
                let a289 = 4u64;
                if nat_eq(a288, a289) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a290 = v0;
                let a291 = 5u64;
                if nat_eq(a290, a291) {
                    Ok(f28(1u64, f24(), 8u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a292 = v0;
            let a293 = 7u64;
            if nat_lt(a292, a293) {
                let a294 = v0;
                let a295 = 6u64;
                if nat_eq(a294, a295) {
                    Ok(f32(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a296 = v0;
                let a297 = 7u64;
                if nat_eq(a296, a297) {
                    Ok(f43(3u64, 1065353216u64, f49(), f51()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a298 = v0;
    let a299 = 4u64;
    if nat_lt(a298, a299) {
        let a300 = v0;
        let a301 = 2u64;
        if nat_lt(a300, a301) {
            let a302 = v0;
            let a303 = 1u64;
            if nat_lt(a302, a303) {
                let a304 = v0;
                let a305 = 0u64;
                if nat_eq(a304, a305) {
                    Ok(f23(f49(), f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a306 = v0;
                let a307 = 1u64;
                if nat_eq(a306, a307) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a308 = v0;
            let a309 = 3u64;
            if nat_lt(a308, a309) {
                let a310 = v0;
                let a311 = 2u64;
                if nat_eq(a310, a311) {
                    Ok(f28(0u64, f53(), 0u64, 2572u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a312 = v0;
                let a313 = 3u64;
                if nat_eq(a312, a313) {
                    Ok(f28(0u64, f50(f24())?, 1u64, 2575u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a314 = v0;
        let a315 = 6u64;
        if nat_lt(a314, a315) {
            let a316 = v0;
            let a317 = 5u64;
            if nat_lt(a316, a317) {
                let a318 = v0;
                let a319 = 4u64;
                if nat_eq(a318, a319) {
                    Ok(f23(2574u64, f54()))
                } else {
                    Ok(f2())
                }
            } else {
                let a320 = v0;
                let a321 = 5u64;
                if nat_eq(a320, a321) {
                    Ok(f32(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a322 = v0;
            let a323 = 7u64;
            if nat_lt(a322, a323) {
                let a324 = v0;
                let a325 = 6u64;
                if nat_eq(a324, a325) {
                    Ok(f23(f31(), f33(f24())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a326 = v0;
                let a327 = 7u64;
                if nat_eq(a326, a327) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a328 = v0;
    let a329 = 4u64;
    if nat_lt(a328, a329) {
        let a330 = v0;
        let a331 = 2u64;
        if nat_lt(a330, a331) {
            let a332 = v0;
            let a333 = 1u64;
            if nat_lt(a332, a333) {
                let a334 = v0;
                let a335 = 0u64;
                if nat_eq(a334, a335) {
                    f28(1u64, f24(), 8u64, 2570u64)
                } else {
                    f2()
                }
            } else {
                let a336 = v0;
                let a337 = 1u64;
                if nat_eq(a336, a337) {
                    f23(1u64, f29())
                } else {
                    f2()
                }
            }
        } else {
            let a338 = v0;
            let a339 = 3u64;
            if nat_lt(a338, a339) {
                let a340 = v0;
                let a341 = 2u64;
                if nat_eq(a340, a341) {
                    f23(2580u64, f48())
                } else {
                    f2()
                }
            } else {
                let a342 = v0;
                let a343 = 3u64;
                if nat_eq(a342, a343) {
                    f32(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a344 = v0;
        let a345 = 6u64;
        if nat_lt(a344, a345) {
            let a346 = v0;
            let a347 = 5u64;
            if nat_lt(a346, a347) {
                let a348 = v0;
                let a349 = 4u64;
                if nat_eq(a348, a349) {
                    f28(2u64, 0u64, f26(), 2582u64)
                } else {
                    f2()
                }
            } else {
                let a350 = v0;
                let a351 = 5u64;
                if nat_eq(a350, a351) {
                    f32(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a352 = v0;
            let a353 = 7u64;
            if nat_lt(a352, a353) {
                let a354 = v0;
                let a355 = 6u64;
                if nat_eq(a354, a355) {
                    f43(3u64, f25(), f26(), f49())
                } else {
                    f2()
                }
            } else {
                let a356 = v0;
                let a357 = 7u64;
                if nat_eq(a356, a357) {
                    f32(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a358 = v0;
    let a359 = 4u64;
    if nat_lt(a358, a359) {
        let a360 = v0;
        let a361 = 2u64;
        if nat_lt(a360, a361) {
            let a362 = v0;
            let a363 = 1u64;
            if nat_lt(a362, a363) {
                let a364 = v0;
                let a365 = 0u64;
                if nat_eq(a364, a365) {
                    Ok(f43(2u64, f44(f27())?, f41(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a366 = v0;
                let a367 = 1u64;
                if nat_eq(a366, a367) {
                    Ok(f43(2u64, f46(), f41(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a368 = v0;
            let a369 = 3u64;
            if nat_lt(a368, a369) {
                let a370 = v0;
                let a371 = 2u64;
                if nat_eq(a370, a371) {
                    Ok(f43(0u64, f26(), f46(), f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a372 = v0;
                let a373 = 3u64;
                if nat_eq(a372, a373) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a374 = v0;
        let a375 = 6u64;
        if nat_lt(a374, a375) {
            let a376 = v0;
            let a377 = 5u64;
            if nat_lt(a376, a377) {
                let a378 = v0;
                let a379 = 4u64;
                if nat_eq(a378, a379) {
                    Ok(f38(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a380 = v0;
                let a381 = 5u64;
                if nat_eq(a380, a381) {
                    Ok(f28(1u64, f34(), 8u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a382 = v0;
            let a383 = 7u64;
            if nat_lt(a382, a383) {
                let a384 = v0;
                let a385 = 6u64;
                if nat_eq(a384, a385) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a386 = v0;
                let a387 = 7u64;
                if nat_eq(a386, a387) {
                    Ok(f28(1u64, f24(), 2u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a388 = v0;
    let a389 = 4u64;
    if nat_lt(a388, a389) {
        let a390 = v0;
        let a391 = 2u64;
        if nat_lt(a390, a391) {
            let a392 = v0;
            let a393 = 1u64;
            if nat_lt(a392, a393) {
                let a394 = v0;
                let a395 = 0u64;
                if nat_eq(a394, a395) {
                    Ok(f28(2u64, 0u64, f49(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a396 = v0;
                let a397 = 1u64;
                if nat_eq(a396, a397) {
                    Ok(f32(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a398 = v0;
            let a399 = 3u64;
            if nat_lt(a398, a399) {
                let a400 = v0;
                let a401 = 2u64;
                if nat_eq(a400, a401) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a402 = v0;
                let a403 = 3u64;
                if nat_eq(a402, a403) {
                    Ok(f28(0u64, f50(f24())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a404 = v0;
        let a405 = 6u64;
        if nat_lt(a404, a405) {
            let a406 = v0;
            let a407 = 5u64;
            if nat_lt(a406, a407) {
                let a408 = v0;
                let a409 = 4u64;
                if nat_eq(a408, a409) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a410 = v0;
                let a411 = 5u64;
                if nat_eq(a410, a411) {
                    Ok(f28(1u64, f24(), 2u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a412 = v0;
            let a413 = 7u64;
            if nat_lt(a412, a413) {
                let a414 = v0;
                let a415 = 6u64;
                if nat_eq(a414, a415) {
                    Ok(f32(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a416 = v0;
                let a417 = 7u64;
                if nat_eq(a416, a417) {
                    Ok(f43(3u64, 1065353216u64, f49(), f51()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a418 = v0;
    let a419 = 4u64;
    if nat_lt(a418, a419) {
        let a420 = v0;
        let a421 = 2u64;
        if nat_lt(a420, a421) {
            let a422 = v0;
            let a423 = 1u64;
            if nat_lt(a422, a423) {
                let a424 = v0;
                let a425 = 0u64;
                if nat_eq(a424, a425) {
                    Ok(f23(f49(), f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a426 = v0;
                let a427 = 1u64;
                if nat_eq(a426, a427) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a428 = v0;
            let a429 = 3u64;
            if nat_lt(a428, a429) {
                let a430 = v0;
                let a431 = 2u64;
                if nat_eq(a430, a431) {
                    Ok(f28(0u64, f50(f24())?, 1u64, 2574u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a432 = v0;
                let a433 = 3u64;
                if nat_eq(a432, a433) {
                    Ok(f23(2573u64, f54()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a434 = v0;
        let a435 = 6u64;
        if nat_lt(a434, a435) {
            let a436 = v0;
            let a437 = 5u64;
            if nat_lt(a436, a437) {
                let a438 = v0;
                let a439 = 4u64;
                if nat_eq(a438, a439) {
                    Ok(f32(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a440 = v0;
                let a441 = 5u64;
                if nat_eq(a440, a441) {
                    Ok(f23(f31(), f33(f24())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a442 = v0;
            let a443 = 7u64;
            if nat_lt(a442, a443) {
                let a444 = v0;
                let a445 = 6u64;
                if nat_eq(a444, a445) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a446 = v0;
                let a447 = 7u64;
                if nat_eq(a446, a447) {
                    Ok(f28(1u64, f24(), 2u64, 2570u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a448 = v0;
    let a449 = 3u64;
    if nat_lt(a448, a449) {
        let a450 = v0;
        let a451 = 1u64;
        if nat_lt(a450, a451) {
            let a452 = v0;
            let a453 = 0u64;
            if nat_eq(a452, a453) {
                f23(1u64, f29())
            } else {
                f2()
            }
        } else {
            let a454 = v0;
            let a455 = 2u64;
            if nat_lt(a454, a455) {
                let a456 = v0;
                let a457 = 1u64;
                if nat_eq(a456, a457) {
                    f23(2579u64, f48())
                } else {
                    f2()
                }
            } else {
                let a458 = v0;
                let a459 = 2u64;
                if nat_eq(a458, a459) {
                    f32(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a460 = v0;
        let a461 = 5u64;
        if nat_lt(a460, a461) {
            let a462 = v0;
            let a463 = 4u64;
            if nat_lt(a462, a463) {
                let a464 = v0;
                let a465 = 3u64;
                if nat_eq(a464, a465) {
                    f28(2u64, 0u64, f26(), 2581u64)
                } else {
                    f2()
                }
            } else {
                let a466 = v0;
                let a467 = 4u64;
                if nat_eq(a466, a467) {
                    f32(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a468 = v0;
            let a469 = 6u64;
            if nat_lt(a468, a469) {
                let a470 = v0;
                let a471 = 5u64;
                if nat_eq(a470, a471) {
                    f43(3u64, f25(), f26(), f49())
                } else {
                    f2()
                }
            } else {
                let a472 = v0;
                let a473 = 6u64;
                if nat_eq(a472, a473) {
                    f32(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a474 = v0;
    let a475 = 4u64;
    if nat_lt(a474, a475) {
        let a476 = v0;
        let a477 = 2u64;
        if nat_lt(a476, a477) {
            let a478 = v0;
            let a479 = 1u64;
            if nat_lt(a478, a479) {
                let a480 = v0;
                let a481 = 0u64;
                if nat_eq(a480, a481) {
                    Ok(f43(2u64, f44(f27())?, f41(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a482 = v0;
                let a483 = 1u64;
                if nat_eq(a482, a483) {
                    Ok(f43(2u64, f46(), f41(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a484 = v0;
            let a485 = 3u64;
            if nat_lt(a484, a485) {
                let a486 = v0;
                let a487 = 2u64;
                if nat_eq(a486, a487) {
                    Ok(f43(0u64, f26(), f46(), f26()))
                } else {
                    Ok(f2())
                }
            } else {
                let a488 = v0;
                let a489 = 3u64;
                if nat_eq(a488, a489) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a490 = v0;
        let a491 = 6u64;
        if nat_lt(a490, a491) {
            let a492 = v0;
            let a493 = 5u64;
            if nat_lt(a492, a493) {
                let a494 = v0;
                let a495 = 4u64;
                if nat_eq(a494, a495) {
                    Ok(f38(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a496 = v0;
                let a497 = 5u64;
                if nat_eq(a496, a497) {
                    Ok(f28(1u64, f34(), 8u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a498 = v0;
            let a499 = 7u64;
            if nat_lt(a498, a499) {
                let a500 = v0;
                let a501 = 6u64;
                if nat_eq(a500, a501) {
                    Ok(f38(0u64, f24(), 1u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a502 = v0;
                let a503 = 7u64;
                if nat_eq(a502, a503) {
                    Ok(f28(1u64, f24(), 4u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a504 = v0;
    let a505 = 4u64;
    if nat_lt(a504, a505) {
        let a506 = v0;
        let a507 = 2u64;
        if nat_lt(a506, a507) {
            let a508 = v0;
            let a509 = 1u64;
            if nat_lt(a508, a509) {
                let a510 = v0;
                let a511 = 0u64;
                if nat_eq(a510, a511) {
                    Ok(f28(2u64, 0u64, f49(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a512 = v0;
                let a513 = 1u64;
                if nat_eq(a512, a513) {
                    Ok(f32(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a514 = v0;
            let a515 = 3u64;
            if nat_lt(a514, a515) {
                let a516 = v0;
                let a517 = 2u64;
                if nat_eq(a516, a517) {
                    Ok(f43(3u64, 1065353216u64, f49(), f51()))
                } else {
                    Ok(f2())
                }
            } else {
                let a518 = v0;
                let a519 = 3u64;
                if nat_eq(a518, a519) {
                    Ok(f23(f49(), f52()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a520 = v0;
        let a521 = 6u64;
        if nat_lt(a520, a521) {
            let a522 = v0;
            let a523 = 5u64;
            if nat_lt(a522, a523) {
                let a524 = v0;
                let a525 = 4u64;
                if nat_eq(a524, a525) {
                    Ok(f23(0u64, f24()))
                } else {
                    Ok(f2())
                }
            } else {
                let a526 = v0;
                let a527 = 5u64;
                if nat_eq(a526, a527) {
                    Ok(f23(2567u64, f54()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a528 = v0;
            let a529 = 7u64;
            if nat_lt(a528, a529) {
                let a530 = v0;
                let a531 = 6u64;
                if nat_eq(a530, a531) {
                    Ok(f32(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a532 = v0;
                let a533 = 7u64;
                if nat_eq(a532, a533) {
                    Ok(f23(f31(), f33(f24())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a534 = v0;
    let a535 = 4u64;
    if nat_lt(a534, a535) {
        let a536 = v0;
        let a537 = 2u64;
        if nat_lt(a536, a537) {
            let a538 = v0;
            let a539 = 1u64;
            if nat_lt(a538, a539) {
                let a540 = v0;
                let a541 = 0u64;
                if nat_eq(a540, a541) {
                    f38(0u64, f24(), 1u64, f24())
                } else {
                    f2()
                }
            } else {
                let a542 = v0;
                let a543 = 1u64;
                if nat_eq(a542, a543) {
                    f28(1u64, f24(), 4u64, 2565u64)
                } else {
                    f2()
                }
            }
        } else {
            let a544 = v0;
            let a545 = 3u64;
            if nat_lt(a544, a545) {
                let a546 = v0;
                let a547 = 2u64;
                if nat_eq(a546, a547) {
                    f23(1u64, f29())
                } else {
                    f2()
                }
            } else {
                let a548 = v0;
                let a549 = 3u64;
                if nat_eq(a548, a549) {
                    f23(2573u64, f48())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a550 = v0;
        let a551 = 6u64;
        if nat_lt(a550, a551) {
            let a552 = v0;
            let a553 = 5u64;
            if nat_lt(a552, a553) {
                let a554 = v0;
                let a555 = 4u64;
                if nat_eq(a554, a555) {
                    f32(2048u64)
                } else {
                    f2()
                }
            } else {
                let a556 = v0;
                let a557 = 5u64;
                if nat_eq(a556, a557) {
                    f28(2u64, 0u64, f26(), 2575u64)
                } else {
                    f2()
                }
            }
        } else {
            let a558 = v0;
            let a559 = 7u64;
            if nat_lt(a558, a559) {
                let a560 = v0;
                let a561 = 6u64;
                if nat_eq(a560, a561) {
                    f32(3072u64)
                } else {
                    f2()
                }
            } else {
                let a562 = v0;
                let a563 = 7u64;
                if nat_eq(a562, a563) {
                    f43(3u64, f25(), f26(), f49())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a564 = v0;
    let a565 = 0u64;
    if nat_eq(a564, a565) {
        f32(3072u64)
    } else {
        f2()
    }
}

pub fn f23(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f24() -> u64 {
    4294967323u64
}

pub fn f25() -> u64 {
    4294967335u64
}

pub fn f26() -> u64 {
    4294967336u64
}

pub fn f27() -> u64 {
    4294967303u64
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f29() -> u64 {
    4294967324u64
}

pub fn f30(v0: u64) -> R<u64> {
    let a569 = 8589976576u64;
    let a570 = {
        let a566 = v0;
        let a567 = 4294967296u64;
        let a568 = 0u64;
        nat_rem(a566, a567, a568)
    };
    nat_add(a569, a570)
}

pub fn f31() -> u64 {
    4294967321u64
}

pub fn f32(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f33(v0: u64) -> R<u64> {
    let a574 = 8589980672u64;
    let a575 = {
        let a571 = v0;
        let a572 = 4294967296u64;
        let a573 = 0u64;
        nat_rem(a571, a572, a573)
    };
    nat_add(a574, a575)
}

pub fn f34() -> u64 {
    4294967305u64
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f36() -> u64 {
    4294967338u64
}

pub fn f37() -> u64 {
    4294967330u64
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f40(v0: u64) -> R<u64> {
    let a579 = 25769803776u64;
    let a580 = {
        let a576 = v0;
        let a577 = 4294967296u64;
        let a578 = 0u64;
        nat_rem(a576, a577, a578)
    };
    nat_add(a579, a580)
}

pub fn f41() -> u64 {
    4294967332u64
}

pub fn f42(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f44(v0: u64) -> R<u64> {
    let a584 = 8589959168u64;
    let a585 = {
        let a581 = v0;
        let a582 = 4294967296u64;
        let a583 = 0u64;
        nat_rem(a581, a582, a583)
    };
    nat_add(a584, a585)
}

pub fn f45(v0: u64) -> R<u64> {
    let a589 = 8589950976u64;
    let a590 = {
        let a586 = v0;
        let a587 = 4294967296u64;
        let a588 = 0u64;
        nat_rem(a586, a587, a588)
    };
    nat_add(a589, a590)
}

pub fn f46() -> u64 {
    4294967337u64
}

pub fn f47(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f48() -> u64 {
    4294967351u64
}

pub fn f49() -> u64 {
    4294967316u64
}

pub fn f50(v0: u64) -> R<u64> {
    let a594 = 8589982720u64;
    let a595 = {
        let a591 = v0;
        let a592 = 4294967296u64;
        let a593 = 0u64;
        nat_rem(a591, a592, a593)
    };
    nat_add(a594, a595)
}

pub fn f51() -> u64 {
    4294967319u64
}

pub fn f52() -> u64 {
    4294967320u64
}

pub fn f53() -> u64 {
    4294967297u64
}

pub fn f54() -> u64 {
    4294967350u64
}
