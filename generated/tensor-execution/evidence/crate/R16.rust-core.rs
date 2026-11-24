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

#[derive(Clone)]
pub enum Fn0 {
    F9,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F9 => f9(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 4u64;
    if if nat_lt(a1, a2) {
        let a3 = v1;
        let a4 = 4294967296u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        Ok(Some(f1(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f2(v1)?;
    let a5 = 150u64;
    let a6 = v2;
    if nat_le(a5, a6) {
        Ok(v1)
    } else {
        let a7 = v2;
        let a8 = 127u64;
        if nat_lt(a7, a8) {
            f3(v0, v1)
        } else {
            let v3: u64 = f4({
                let a9 = 150u64;
                let a10 = v2;
                nat_sub(a9, a10)
            })?;
            let v4: u64 = {
                let a11 = f5(v1)?;
                let a12 = v3;
                let a13 = 0u64;
                nat_rem(a11, a12, a13)
            };
            let v5: bool = {
                let a14 = 0u64;
                let a15 = v4;
                if nat_lt(a14, a15) {
                    let a16 = v0;
                    let a17 = 0u64;
                    if if if nat_eq(a16, a17) {
                        let a18 = f6(v1)?;
                        let a19 = 1u64;
                        nat_eq(a18, a19)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a20 = v0;
                        let a21 = 1u64;
                        if nat_eq(a20, a21) {
                            let a22 = f6(v1)?;
                            let a23 = 0u64;
                            nat_eq(a22, a23)
                        } else {
                            false
                        }
                    } {
                        true
                    } else {
                        let a24 = v0;
                        let a25 = 2u64;
                        if nat_eq(a24, a25) {
                            let a29 = {
                                let a26 = v3;
                                let a27 = 2u64;
                                let a28 = 0u64;
                                nat_quot(a26, a27, a28)
                            };
                            let a30 = v4;
                            nat_le(a29, a30)
                        } else {
                            false
                        }
                    }
                } else {
                    false
                }
            };
            let a35 = {
                let a33 = f7(f6(v1)?)?;
                let a34 = {
                    let a31 = f5(v1)?;
                    let a32 = v4;
                    nat_sub(a31, a32)
                };
                nat_add(a33, a34)?
            };
            let a36 = if v5 {
                v3
            } else {
                0u64
            };
            nat_add(a35, a36)
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let a59 = f5(v0)?;
    let a60 = f4({
        let a57 = {
            let a55 = {
                let a37 = 1u64;
                let a38 = 1u64;
                nat_add(a37, a38)?
            };
            let a56 = {
                let a53 = {
                    let a51 = {
                        let a39 = 1u64;
                        let a40 = 1u64;
                        nat_add(a39, a40)?
                    };
                    let a52 = {
                        let a49 = {
                            let a47 = {
                                let a41 = 1u64;
                                let a42 = 1u64;
                                nat_add(a41, a42)?
                            };
                            let a48 = {
                                let a45 = {
                                    let a43 = 1u64;
                                    let a44 = 1u64;
                                    nat_add(a43, a44)?
                                };
                                let a46 = 1u64;
                                nat_mul(a45, a46)?
                            };
                            nat_mul(a47, a48)?
                        };
                        let a50 = 1u64;
                        nat_add(a49, a50)?
                    };
                    nat_mul(a51, a52)?
                };
                let a54 = 1u64;
                nat_add(a53, a54)?
            };
            nat_mul(a55, a56)?
        };
        let a58 = 1u64;
        nat_add(a57, a58)?
    })?;
    let a61 = 0u64;
    Ok(nat_quot(a59, a60, a61))
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f5(v1)?;
    let v3: bool = {
        let a62 = f6(v1)?;
        let a63 = 1u64;
        nat_eq(a62, a63)
    };
    let a76 = f7(f6(v1)?)?;
    let a77 = {
        let a64 = v0;
        let a65 = 0u64;
        if if if if nat_eq(a64, a65) {
            v3
        } else {
            false
        } {
            true
        } else {
            let a66 = v0;
            let a67 = 1u64;
            if nat_eq(a66, a67) {
                let a68 = f6(v1)?;
                let a69 = 0u64;
                nat_eq(a68, a69)
            } else {
                false
            }
        } {
            let a70 = 0u64;
            let a71 = v2;
            nat_lt(a70, a71)
        } else {
            false
        } {
            1065353216u64
        } else {
            let a72 = v0;
            let a73 = 2u64;
            if if nat_eq(a72, a73) {
                let a74 = 1056964608u64;
                let a75 = v2;
                nat_le(a74, a75)
            } else {
                false
            } {
                1065353216u64
            } else {
                0u64
            }
        }
    };
    nat_add(a76, a77)
}

pub fn f4(v0: u64) -> R<u64> {
    f8(Fn0::F9, v0, 1u64)
}

pub fn f5(v0: u64) -> R<u64> {
    let a102 = v0;
    let a103 = f4({
        let a100 = {
            let a98 = {
                let a78 = 1u64;
                let a79 = 1u64;
                nat_add(a78, a79)?
            };
            let a99 = {
                let a96 = {
                    let a94 = {
                        let a80 = 1u64;
                        let a81 = 1u64;
                        nat_add(a80, a81)?
                    };
                    let a95 = {
                        let a92 = {
                            let a90 = {
                                let a82 = 1u64;
                                let a83 = 1u64;
                                nat_add(a82, a83)?
                            };
                            let a91 = {
                                let a88 = {
                                    let a86 = {
                                        let a84 = 1u64;
                                        let a85 = 1u64;
                                        nat_add(a84, a85)?
                                    };
                                    let a87 = 1u64;
                                    nat_mul(a86, a87)?
                                };
                                let a89 = 1u64;
                                nat_add(a88, a89)?
                            };
                            nat_mul(a90, a91)?
                        };
                        let a93 = 1u64;
                        nat_add(a92, a93)?
                    };
                    nat_mul(a94, a95)?
                };
                let a97 = 1u64;
                nat_add(a96, a97)?
            };
            nat_mul(a98, a99)?
        };
        let a101 = 1u64;
        nat_add(a100, a101)?
    })?;
    let a104 = 0u64;
    Ok(nat_rem(a102, a103, a104))
}

pub fn f6(v0: u64) -> R<u64> {
    let a129 = v0;
    let a130 = f4({
        let a127 = {
            let a125 = {
                let a105 = 1u64;
                let a106 = 1u64;
                nat_add(a105, a106)?
            };
            let a126 = {
                let a123 = {
                    let a121 = {
                        let a107 = 1u64;
                        let a108 = 1u64;
                        nat_add(a107, a108)?
                    };
                    let a122 = {
                        let a119 = {
                            let a117 = {
                                let a109 = 1u64;
                                let a110 = 1u64;
                                nat_add(a109, a110)?
                            };
                            let a118 = {
                                let a115 = {
                                    let a113 = {
                                        let a111 = 1u64;
                                        let a112 = 1u64;
                                        nat_add(a111, a112)?
                                    };
                                    let a114 = 1u64;
                                    nat_mul(a113, a114)?
                                };
                                let a116 = 1u64;
                                nat_add(a115, a116)?
                            };
                            nat_mul(a117, a118)?
                        };
                        let a120 = 1u64;
                        nat_add(a119, a120)?
                    };
                    nat_mul(a121, a122)?
                };
                let a124 = 1u64;
                nat_add(a123, a124)?
            };
            nat_mul(a125, a126)?
        };
        let a128 = 1u64;
        nat_add(a127, a128)?
    })?;
    let a131 = 0u64;
    Ok(nat_quot(a129, a130, a131))
}

pub fn f7(v0: u64) -> R<u64> {
    let a156 = v0;
    let a157 = f4({
        let a154 = {
            let a152 = {
                let a132 = 1u64;
                let a133 = 1u64;
                nat_add(a132, a133)?
            };
            let a153 = {
                let a150 = {
                    let a148 = {
                        let a134 = 1u64;
                        let a135 = 1u64;
                        nat_add(a134, a135)?
                    };
                    let a149 = {
                        let a146 = {
                            let a144 = {
                                let a136 = 1u64;
                                let a137 = 1u64;
                                nat_add(a136, a137)?
                            };
                            let a145 = {
                                let a142 = {
                                    let a140 = {
                                        let a138 = 1u64;
                                        let a139 = 1u64;
                                        nat_add(a138, a139)?
                                    };
                                    let a141 = 1u64;
                                    nat_mul(a140, a141)?
                                };
                                let a143 = 1u64;
                                nat_add(a142, a143)?
                            };
                            nat_mul(a144, a145)?
                        };
                        let a147 = 1u64;
                        nat_add(a146, a147)?
                    };
                    nat_mul(a148, a149)?
                };
                let a151 = 1u64;
                nat_add(a150, a151)?
            };
            nat_mul(a152, a153)?
        };
        let a155 = 1u64;
        nat_add(a154, a155)?
    })?;
    nat_mul(a156, a157)
}

pub fn f8(v0: Fn0, p161: u64, p162: u64) -> R<u64> {
    let m160 = (p161, p162);
    {
        let mut m160 = m160;
        loop {
            let step = {
                let (v1, v2) = m160;
                let m158 = v1;
                if m158 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m158 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c159: Fn0 = v0.clone();
                        c159.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f9(v0: u64) -> R<u64> {
    let a163 = v0;
    let a164 = v0;
    nat_add(a163, a164)
}
