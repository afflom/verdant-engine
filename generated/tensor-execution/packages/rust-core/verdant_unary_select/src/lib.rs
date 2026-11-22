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
    F10,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F10 => f10(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 3u64;
    if if nat_lt(a1, a2) {
        let a3 = v1;
        let a4 = 4294967296u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        Ok(Some({
            let a5 = v0;
            let a6 = 0u64;
            if nat_eq(a5, a6) {
                if f1(0u64, v1)? {
                    1065353216u64
                } else {
                    if f1(v1, 0u64)? {
                        3212836864u64
                    } else {
                        0u64
                    }
                }
            } else {
                let a7 = v0;
                let a8 = 1u64;
                if nat_eq(a7, a8) {
                    if f1(0u64, v1)? {
                        1065353216u64
                    } else {
                        0u64
                    }
                } else {
                    if f1(0u64, v1)? {
                        v1
                    } else {
                        0u64
                    }
                }
            }
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64, v1: u64) -> R<bool> {
    if if f2(v0)? {
        true
    } else {
        f2(v1)?
    } {
        Ok(false)
    } else {
        if if f3(v0)? {
            f3(v1)?
        } else {
            false
        } {
            Ok(false)
        } else {
            let a9 = f4(v0)?;
            let a10 = f4(v1)?;
            if nat_eq(a9, a10) {
                let a11 = f4(v0)?;
                let a12 = 0u64;
                if nat_eq(a11, a12) {
                    let a13 = f5(v0)?;
                    let a14 = f5(v1)?;
                    Ok(nat_lt(a13, a14))
                } else {
                    let a15 = f5(v1)?;
                    let a16 = f5(v0)?;
                    Ok(nat_lt(a15, a16))
                }
            } else {
                let a17 = f4(v0)?;
                let a18 = 1u64;
                Ok(nat_eq(a17, a18))
            }
        }
    }
}

pub fn f2(v0: u64) -> R<bool> {
    let a61 = f6(v0)?;
    let a62 = {
        let a59 = {
            let a57 = {
                let a19 = 1u64;
                let a20 = 1u64;
                nat_add(a19, a20)?
            };
            let a58 = {
                let a55 = {
                    let a53 = {
                        let a21 = 1u64;
                        let a22 = 1u64;
                        nat_add(a21, a22)?
                    };
                    let a54 = {
                        let a51 = {
                            let a49 = {
                                let a23 = 1u64;
                                let a24 = 1u64;
                                nat_add(a23, a24)?
                            };
                            let a50 = {
                                let a47 = {
                                    let a45 = {
                                        let a25 = 1u64;
                                        let a26 = 1u64;
                                        nat_add(a25, a26)?
                                    };
                                    let a46 = {
                                        let a43 = {
                                            let a41 = {
                                                let a27 = 1u64;
                                                let a28 = 1u64;
                                                nat_add(a27, a28)?
                                            };
                                            let a42 = {
                                                let a39 = {
                                                    let a37 = {
                                                        let a29 = 1u64;
                                                        let a30 = 1u64;
                                                        nat_add(a29, a30)?
                                                    };
                                                    let a38 = {
                                                        let a35 = {
                                                            let a33 = {
                                                                let a31 = 1u64;
                                                                let a32 = 1u64;
                                                                nat_add(a31, a32)?
                                                            };
                                                            let a34 = 1u64;
                                                            nat_mul(a33, a34)?
                                                        };
                                                        let a36 = 1u64;
                                                        nat_add(a35, a36)?
                                                    };
                                                    nat_mul(a37, a38)?
                                                };
                                                let a40 = 1u64;
                                                nat_add(a39, a40)?
                                            };
                                            nat_mul(a41, a42)?
                                        };
                                        let a44 = 1u64;
                                        nat_add(a43, a44)?
                                    };
                                    nat_mul(a45, a46)?
                                };
                                let a48 = 1u64;
                                nat_add(a47, a48)?
                            };
                            nat_mul(a49, a50)?
                        };
                        let a52 = 1u64;
                        nat_add(a51, a52)?
                    };
                    nat_mul(a53, a54)?
                };
                let a56 = 1u64;
                nat_add(a55, a56)?
            };
            nat_mul(a57, a58)?
        };
        let a60 = 1u64;
        nat_add(a59, a60)?
    };
    if nat_eq(a61, a62) {
        let a63 = 0u64;
        let a64 = f7(v0)?;
        Ok(nat_lt(a63, a64))
    } else {
        Ok(false)
    }
}

pub fn f3(v0: u64) -> R<bool> {
    let a65 = f5(v0)?;
    let a66 = 0u64;
    Ok(nat_eq(a65, a66))
}

pub fn f4(v0: u64) -> R<u64> {
    let a91 = v0;
    let a92 = f8({
        let a89 = {
            let a87 = {
                let a67 = 1u64;
                let a68 = 1u64;
                nat_add(a67, a68)?
            };
            let a88 = {
                let a85 = {
                    let a83 = {
                        let a69 = 1u64;
                        let a70 = 1u64;
                        nat_add(a69, a70)?
                    };
                    let a84 = {
                        let a81 = {
                            let a79 = {
                                let a71 = 1u64;
                                let a72 = 1u64;
                                nat_add(a71, a72)?
                            };
                            let a80 = {
                                let a77 = {
                                    let a75 = {
                                        let a73 = 1u64;
                                        let a74 = 1u64;
                                        nat_add(a73, a74)?
                                    };
                                    let a76 = 1u64;
                                    nat_mul(a75, a76)?
                                };
                                let a78 = 1u64;
                                nat_add(a77, a78)?
                            };
                            nat_mul(a79, a80)?
                        };
                        let a82 = 1u64;
                        nat_add(a81, a82)?
                    };
                    nat_mul(a83, a84)?
                };
                let a86 = 1u64;
                nat_add(a85, a86)?
            };
            nat_mul(a87, a88)?
        };
        let a90 = 1u64;
        nat_add(a89, a90)?
    })?;
    let a93 = 0u64;
    Ok(nat_quot(a91, a92, a93))
}

pub fn f5(v0: u64) -> R<u64> {
    let a118 = v0;
    let a119 = f8({
        let a116 = {
            let a114 = {
                let a94 = 1u64;
                let a95 = 1u64;
                nat_add(a94, a95)?
            };
            let a115 = {
                let a112 = {
                    let a110 = {
                        let a96 = 1u64;
                        let a97 = 1u64;
                        nat_add(a96, a97)?
                    };
                    let a111 = {
                        let a108 = {
                            let a106 = {
                                let a98 = 1u64;
                                let a99 = 1u64;
                                nat_add(a98, a99)?
                            };
                            let a107 = {
                                let a104 = {
                                    let a102 = {
                                        let a100 = 1u64;
                                        let a101 = 1u64;
                                        nat_add(a100, a101)?
                                    };
                                    let a103 = 1u64;
                                    nat_mul(a102, a103)?
                                };
                                let a105 = 1u64;
                                nat_add(a104, a105)?
                            };
                            nat_mul(a106, a107)?
                        };
                        let a109 = 1u64;
                        nat_add(a108, a109)?
                    };
                    nat_mul(a110, a111)?
                };
                let a113 = 1u64;
                nat_add(a112, a113)?
            };
            nat_mul(a114, a115)?
        };
        let a117 = 1u64;
        nat_add(a116, a117)?
    })?;
    let a120 = 0u64;
    Ok(nat_rem(a118, a119, a120))
}

pub fn f6(v0: u64) -> R<u64> {
    let a143 = f5(v0)?;
    let a144 = f8({
        let a141 = {
            let a139 = {
                let a121 = 1u64;
                let a122 = 1u64;
                nat_add(a121, a122)?
            };
            let a140 = {
                let a137 = {
                    let a135 = {
                        let a123 = 1u64;
                        let a124 = 1u64;
                        nat_add(a123, a124)?
                    };
                    let a136 = {
                        let a133 = {
                            let a131 = {
                                let a125 = 1u64;
                                let a126 = 1u64;
                                nat_add(a125, a126)?
                            };
                            let a132 = {
                                let a129 = {
                                    let a127 = 1u64;
                                    let a128 = 1u64;
                                    nat_add(a127, a128)?
                                };
                                let a130 = 1u64;
                                nat_mul(a129, a130)?
                            };
                            nat_mul(a131, a132)?
                        };
                        let a134 = 1u64;
                        nat_add(a133, a134)?
                    };
                    nat_mul(a135, a136)?
                };
                let a138 = 1u64;
                nat_add(a137, a138)?
            };
            nat_mul(a139, a140)?
        };
        let a142 = 1u64;
        nat_add(a141, a142)?
    })?;
    let a145 = 0u64;
    Ok(nat_quot(a143, a144, a145))
}

pub fn f7(v0: u64) -> R<u64> {
    let a168 = v0;
    let a169 = f8({
        let a166 = {
            let a164 = {
                let a146 = 1u64;
                let a147 = 1u64;
                nat_add(a146, a147)?
            };
            let a165 = {
                let a162 = {
                    let a160 = {
                        let a148 = 1u64;
                        let a149 = 1u64;
                        nat_add(a148, a149)?
                    };
                    let a161 = {
                        let a158 = {
                            let a156 = {
                                let a150 = 1u64;
                                let a151 = 1u64;
                                nat_add(a150, a151)?
                            };
                            let a157 = {
                                let a154 = {
                                    let a152 = 1u64;
                                    let a153 = 1u64;
                                    nat_add(a152, a153)?
                                };
                                let a155 = 1u64;
                                nat_mul(a154, a155)?
                            };
                            nat_mul(a156, a157)?
                        };
                        let a159 = 1u64;
                        nat_add(a158, a159)?
                    };
                    nat_mul(a160, a161)?
                };
                let a163 = 1u64;
                nat_add(a162, a163)?
            };
            nat_mul(a164, a165)?
        };
        let a167 = 1u64;
        nat_add(a166, a167)?
    })?;
    let a170 = 0u64;
    Ok(nat_rem(a168, a169, a170))
}

pub fn f8(v0: u64) -> R<u64> {
    f9(Fn0::F10, v0, 1u64)
}

pub fn f9(v0: Fn0, p174: u64, p175: u64) -> R<u64> {
    let m173 = (p174, p175);
    {
        let mut m173 = m173;
        loop {
            let step = {
                let (v1, v2) = m173;
                let m171 = v1;
                if m171 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m171 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c172: Fn0 = v0.clone();
                        c172.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let a176 = v0;
    let a177 = v0;
    nat_add(a176, a177)
}

pub fn run(p0: u64, p1: u64) -> R<Option<u64>> {
    f0(p0, p1)
}
