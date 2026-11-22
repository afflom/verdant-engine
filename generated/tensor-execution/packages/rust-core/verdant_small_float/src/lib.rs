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

pub fn f0(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 7u64;
    if if nat_lt(a1, a2) {
        let a7 = v1;
        let a8 = {
            let a3 = v0;
            let a4 = 4u64;
            if nat_lt(a3, a4) {
                256u64
            } else {
                let a5 = v0;
                let a6 = 6u64;
                if nat_lt(a5, a6) {
                    4294967296u64
                } else {
                    16u64
                }
            }
        };
        nat_lt(a7, a8)
    } else {
        false
    } {
        Ok(Some({
            let a9 = v0;
            let a10 = 4u64;
            if nat_lt(a9, a10) {
                f1(v0, v1)?
            } else {
                let a11 = v0;
                let a12 = 4u64;
                if nat_eq(a11, a12) {
                    f2(v1)?
                } else {
                    let a13 = v0;
                    let a14 = 5u64;
                    if nat_eq(a13, a14) {
                        f3(v1)?
                    } else {
                        f4(v1)?
                    }
                }
            }
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    let a15 = v0;
    let a16 = 0u64;
    if nat_eq(a15, a16) {
        let a17 = v1;
        let a18 = 0u64;
        if nat_eq(a17, a18) {
            Ok(4194304u64)
        } else {
            let a19 = v1;
            let a20 = 8388608u64;
            nat_mul(a19, a20)
        }
    } else {
        let a21 = v0;
        let a22 = 1u64;
        if nat_eq(a21, a22) {
            let a23 = v1;
            let a24 = 2u64;
            if nat_lt(a23, a24) {
                let a25 = v1;
                let a26 = 0u64;
                if nat_eq(a25, a26) {
                    Ok(2097152u64)
                } else {
                    Ok(4194304u64)
                }
            } else {
                let a29 = {
                    let a27 = v1;
                    let a28 = 1u64;
                    nat_sub(a27, a28)
                };
                let a30 = 8388608u64;
                nat_mul(a29, a30)
            }
        } else {
            let a31 = v0;
            let a32 = 2u64;
            if nat_eq(a31, a32) {
                let a33 = v1;
                let a34 = 0u64;
                if if nat_eq(a33, a34) {
                    true
                } else {
                    let a35 = v1;
                    let a36 = 127u64;
                    nat_eq(a35, a36)
                } {
                    Ok(0u64)
                } else {
                    f5(v1, 120u64)
                }
            } else {
                let a40 = {
                    let a37 = v1;
                    let a38 = 128u64;
                    let a39 = 0u64;
                    nat_rem(a37, a38, a39)
                };
                let a41 = 127u64;
                if nat_eq(a40, a41) {
                    Ok(2143289344u64)
                } else {
                    let a47 = {
                        let a45 = {
                            let a42 = v1;
                            let a43 = 128u64;
                            let a44 = 0u64;
                            nat_quot(a42, a43, a44)
                        };
                        let a46 = 2147483648u64;
                        nat_mul(a45, a46)?
                    };
                    let a48 = f5(v1, 121u64)?;
                    nat_add(a47, a48)
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a49 = f6(v0);
        let a50 = 128u64;
        nat_mul(a49, a50)?
    };
    let v2: u64 = f7(v0);
    if f8(v0) {
        Ok(127u64)
    } else {
        let a51 = 1138753536u64;
        let a52 = f9(v0);
        if nat_le(a51, a52) {
            let a53 = v1;
            let a54 = 126u64;
            nat_add(a53, a54)
        } else {
            let a65 = v1;
            let a66 = {
                let a55 = v2;
                let a56 = 121u64;
                if nat_lt(a55, a56) {
                    let a57 = v2;
                    let a58 = 117u64;
                    if nat_lt(a57, a58) {
                        0u64
                    } else {
                        f10(f11(v0)?, f12(v2))?
                    }
                } else {
                    let a63 = {
                        let a61 = {
                            let a59 = v2;
                            let a60 = 120u64;
                            nat_sub(a59, a60)
                        };
                        let a62 = 8u64;
                        nat_mul(a61, a62)?
                    };
                    let a64 = f10(f13(v0), 1048576u64)?;
                    nat_add(a63, a64)?
                }
            };
            nat_add(a65, a66)
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let a67 = f6(v0);
    let a68 = 1u64;
    if if if nat_eq(a67, a68) {
        true
    } else {
        let a69 = v0;
        let a70 = 0u64;
        nat_eq(a69, a70)
    } {
        true
    } else {
        f8(v0)
    } {
        Ok(0u64)
    } else {
        let v1: u64 = f7(v0);
        let a71 = 135u64;
        let a72 = v1;
        if nat_le(a71, a72) {
            Ok(126u64)
        } else {
            let a73 = v1;
            let a74 = 120u64;
            if nat_le(a73, a74) {
                let a75 = v1;
                let a76 = 117u64;
                if nat_lt(a75, a76) {
                    let a77 = 981467135u64;
                    let a78 = v0;
                    if nat_le(a77, a78) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    let v2: u64 = f12(v1);
                    let v3: u64 = {
                        let a84 = {
                            let a82 = f11(v0)?;
                            let a83 = {
                                let a79 = v2;
                                let a80 = 2u64;
                                let a81 = 0u64;
                                nat_quot(a79, a80, a81)
                            };
                            nat_add(a82, a83)?
                        };
                        let a85 = v2;
                        let a86 = 0u64;
                        nat_quot(a84, a85, a86)
                    };
                    let a87 = 7u64;
                    let a88 = v3;
                    if nat_lt(a87, a88) {
                        Ok(7u64)
                    } else {
                        Ok(v3)
                    }
                }
            } else {
                let v4: u64 = {
                    let a104 = {
                        let a96 = {
                            let a91 = {
                                let a89 = v1;
                                let a90 = 120u64;
                                nat_sub(a89, a90)
                            };
                            let a92 = 8u64;
                            nat_mul(a91, a92)?
                        };
                        let a97 = {
                            let a93 = f13(v0);
                            let a94 = 1048576u64;
                            let a95 = 0u64;
                            nat_quot(a93, a94, a95)
                        };
                        nat_add(a96, a97)?
                    };
                    let a105 = {
                        let a101 = {
                            let a98 = f13(v0);
                            let a99 = 524288u64;
                            let a100 = 0u64;
                            nat_quot(a98, a99, a100)
                        };
                        let a102 = 2u64;
                        let a103 = 0u64;
                        nat_rem(a101, a102, a103)
                    };
                    nat_add(a104, a105)?
                };
                let a106 = 120u64;
                let a107 = v4;
                if nat_le(a106, a107) {
                    Ok(126u64)
                } else {
                    Ok(v4)
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a108 = v0;
        let a109 = 8u64;
        let a110 = 0u64;
        nat_rem(a108, a109, a110)
    };
    let v2: u64 = {
        let a111 = v1;
        let a112 = 4u64;
        if nat_lt(a111, a112) {
            v1
        } else {
            let a120 = {
                let a116 = 2u64;
                let a117 = {
                    let a113 = v1;
                    let a114 = 2u64;
                    let a115 = 0u64;
                    nat_rem(a113, a114, a115)
                };
                nat_add(a116, a117)?
            };
            let a121 = {
                let a118 = v1;
                let a119 = 6u64;
                if nat_lt(a118, a119) {
                    2u64
                } else {
                    4u64
                }
            };
            nat_mul(a120, a121)?
        }
    };
    let a122 = v2;
    let a123 = 0u64;
    if nat_eq(a122, a123) {
        Ok(0u64)
    } else {
        let a169 = {
            let a127 = {
                let a124 = v0;
                let a125 = 8u64;
                let a126 = 0u64;
                nat_quot(a124, a125, a126)
            };
            let a128 = 2147483648u64;
            nat_mul(a127, a128)?
        };
        let a170 = {
            let a129 = v2;
            let a130 = 2u64;
            if nat_lt(a129, a130) {
                let a135 = {
                    let a131 = 127u64;
                    let a132 = 8388608u64;
                    nat_mul(a131, a132)?
                };
                let a136 = {
                    let a133 = 0u64;
                    let a134 = 1048576u64;
                    nat_mul(a133, a134)?
                };
                nat_add(a135, a136)?
            } else {
                let a137 = v2;
                let a138 = 4u64;
                if nat_lt(a137, a138) {
                    let a147 = {
                        let a139 = 128u64;
                        let a140 = 8388608u64;
                        nat_mul(a139, a140)?
                    };
                    let a148 = {
                        let a145 = {
                            let a143 = {
                                let a141 = v2;
                                let a142 = 4u64;
                                nat_mul(a141, a142)?
                            };
                            let a144 = 8u64;
                            nat_sub(a143, a144)
                        };
                        let a146 = 1048576u64;
                        nat_mul(a145, a146)?
                    };
                    nat_add(a147, a148)?
                } else {
                    let a149 = v2;
                    let a150 = 8u64;
                    if nat_lt(a149, a150) {
                        let a159 = {
                            let a151 = 129u64;
                            let a152 = 8388608u64;
                            nat_mul(a151, a152)?
                        };
                        let a160 = {
                            let a157 = {
                                let a155 = {
                                    let a153 = v2;
                                    let a154 = 2u64;
                                    nat_mul(a153, a154)?
                                };
                                let a156 = 8u64;
                                nat_sub(a155, a156)
                            };
                            let a158 = 1048576u64;
                            nat_mul(a157, a158)?
                        };
                        nat_add(a159, a160)?
                    } else {
                        let a167 = {
                            let a161 = 130u64;
                            let a162 = 8388608u64;
                            nat_mul(a161, a162)?
                        };
                        let a168 = {
                            let a165 = {
                                let a163 = v2;
                                let a164 = 8u64;
                                nat_sub(a163, a164)
                            };
                            let a166 = 1048576u64;
                            nat_mul(a165, a166)?
                        };
                        nat_add(a167, a168)?
                    }
                }
            }
        };
        nat_add(a169, a170)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a174 = {
            let a171 = v0;
            let a172 = 128u64;
            let a173 = 0u64;
            nat_rem(a171, a172, a173)
        };
        let a175 = 8u64;
        let a176 = 0u64;
        nat_quot(a174, a175, a176)
    };
    let v3: u64 = {
        let a177 = v0;
        let a178 = 8u64;
        let a179 = 0u64;
        nat_rem(a177, a178, a179)
    };
    let a180 = v2;
    let a181 = 0u64;
    if nat_eq(a180, a181) {
        let a182 = v3;
        let a183 = 0u64;
        if nat_eq(a182, a183) {
            Ok(0u64)
        } else {
            let a184 = v3;
            let a185 = 2u64;
            if nat_lt(a184, a185) {
                let a192 = {
                    let a188 = {
                        let a186 = v1;
                        let a187 = 3u64;
                        nat_sub(a186, a187)
                    };
                    let a189 = 8388608u64;
                    nat_mul(a188, a189)?
                };
                let a193 = {
                    let a190 = 0u64;
                    let a191 = 1048576u64;
                    nat_mul(a190, a191)?
                };
                nat_add(a192, a193)
            } else {
                let a194 = v3;
                let a195 = 4u64;
                if nat_lt(a194, a195) {
                    let a206 = {
                        let a198 = {
                            let a196 = v1;
                            let a197 = 2u64;
                            nat_sub(a196, a197)
                        };
                        let a199 = 8388608u64;
                        nat_mul(a198, a199)?
                    };
                    let a207 = {
                        let a204 = {
                            let a202 = {
                                let a200 = v3;
                                let a201 = 4u64;
                                nat_mul(a200, a201)?
                            };
                            let a203 = 8u64;
                            nat_sub(a202, a203)
                        };
                        let a205 = 1048576u64;
                        nat_mul(a204, a205)?
                    };
                    nat_add(a206, a207)
                } else {
                    let a218 = {
                        let a210 = {
                            let a208 = v1;
                            let a209 = 1u64;
                            nat_sub(a208, a209)
                        };
                        let a211 = 8388608u64;
                        nat_mul(a210, a211)?
                    };
                    let a219 = {
                        let a216 = {
                            let a214 = {
                                let a212 = v3;
                                let a213 = 2u64;
                                nat_mul(a212, a213)?
                            };
                            let a215 = 8u64;
                            nat_sub(a214, a215)
                        };
                        let a217 = 1048576u64;
                        nat_mul(a216, a217)?
                    };
                    nat_add(a218, a219)
                }
            }
        }
    } else {
        let a228 = {
            let a224 = {
                let a222 = v2;
                let a223 = {
                    let a220 = v1;
                    let a221 = 1u64;
                    nat_sub(a220, a221)
                };
                nat_add(a222, a223)?
            };
            let a225 = 8388608u64;
            nat_mul(a224, a225)?
        };
        let a229 = {
            let a226 = v3;
            let a227 = 1048576u64;
            nat_mul(a226, a227)?
        };
        nat_add(a228, a229)
    }
}

pub fn f6(v0: u64) -> u64 {
    let a230 = v0;
    let a231 = 2147483648u64;
    let a232 = 0u64;
    nat_quot(a230, a231, a232)
}

pub fn f7(v0: u64) -> u64 {
    let a233 = f9(v0);
    let a234 = 8388608u64;
    let a235 = 0u64;
    nat_quot(a233, a234, a235)
}

pub fn f8(v0: u64) -> bool {
    let a236 = 2139095040u64;
    let a237 = f9(v0);
    nat_lt(a236, a237)
}

pub fn f9(v0: u64) -> u64 {
    let a238 = v0;
    let a239 = 2147483648u64;
    let a240 = 0u64;
    nat_rem(a238, a239, a240)
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a241 = v0;
        let a242 = v1;
        let a243 = 0u64;
        nat_quot(a241, a242, a243)
    };
    let v3: u64 = {
        let a247 = {
            let a244 = v0;
            let a245 = v1;
            let a246 = 0u64;
            nat_rem(a244, a245, a246)
        };
        let a248 = 2u64;
        nat_mul(a247, a248)?
    };
    let a258 = v2;
    let a259 = {
        let a249 = v1;
        let a250 = v3;
        if if nat_lt(a249, a250) {
            true
        } else {
            let a251 = v3;
            let a252 = v1;
            if nat_eq(a251, a252) {
                let a256 = {
                    let a253 = v2;
                    let a254 = 2u64;
                    let a255 = 0u64;
                    nat_rem(a253, a254, a255)
                };
                let a257 = 1u64;
                nat_eq(a256, a257)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a258, a259)
}

pub fn f11(v0: u64) -> R<u64> {
    let a262 = f13(v0);
    let a263 = {
        let a260 = f7(v0);
        let a261 = 0u64;
        if nat_eq(a260, a261) {
            0u64
        } else {
            8388608u64
        }
    };
    nat_add(a262, a263)
}

pub fn f12(v0: u64) -> u64 {
    let a264 = v0;
    let a265 = 117u64;
    if nat_eq(a264, a265) {
        16777216u64
    } else {
        let a266 = v0;
        let a267 = 118u64;
        if nat_eq(a266, a267) {
            8388608u64
        } else {
            let a268 = v0;
            let a269 = 119u64;
            if nat_eq(a268, a269) {
                4194304u64
            } else {
                2097152u64
            }
        }
    }
}

pub fn f13(v0: u64) -> u64 {
    let a270 = v0;
    let a271 = 8388608u64;
    let a272 = 0u64;
    nat_rem(a270, a271, a272)
}

pub fn run(p0: u64, p1: u64) -> R<Option<u64>> {
    f0(p0, p1)
}
