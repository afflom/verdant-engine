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

pub fn f0(v0: bool, v1: u64, v2: u64) -> R<u64> {
    if if v0 {
        f1(v2, v1)?
    } else {
        f1(v1, v2)?
    } {
        Ok(v1)
    } else {
        Ok(v2)
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
            let a1 = f4(v0)?;
            let a2 = f4(v1)?;
            if nat_eq(a1, a2) {
                let a3 = f4(v0)?;
                let a4 = 0u64;
                if nat_eq(a3, a4) {
                    let a5 = f5(v0)?;
                    let a6 = f5(v1)?;
                    Ok(nat_lt(a5, a6))
                } else {
                    let a7 = f5(v1)?;
                    let a8 = f5(v0)?;
                    Ok(nat_lt(a7, a8))
                }
            } else {
                let a9 = f4(v0)?;
                let a10 = 1u64;
                Ok(nat_eq(a9, a10))
            }
        }
    }
}

pub fn f2(v0: u64) -> R<bool> {
    let a53 = f6(v0)?;
    let a54 = {
        let a51 = {
            let a49 = {
                let a11 = 1u64;
                let a12 = 1u64;
                nat_add(a11, a12)?
            };
            let a50 = {
                let a47 = {
                    let a45 = {
                        let a13 = 1u64;
                        let a14 = 1u64;
                        nat_add(a13, a14)?
                    };
                    let a46 = {
                        let a43 = {
                            let a41 = {
                                let a15 = 1u64;
                                let a16 = 1u64;
                                nat_add(a15, a16)?
                            };
                            let a42 = {
                                let a39 = {
                                    let a37 = {
                                        let a17 = 1u64;
                                        let a18 = 1u64;
                                        nat_add(a17, a18)?
                                    };
                                    let a38 = {
                                        let a35 = {
                                            let a33 = {
                                                let a19 = 1u64;
                                                let a20 = 1u64;
                                                nat_add(a19, a20)?
                                            };
                                            let a34 = {
                                                let a31 = {
                                                    let a29 = {
                                                        let a21 = 1u64;
                                                        let a22 = 1u64;
                                                        nat_add(a21, a22)?
                                                    };
                                                    let a30 = {
                                                        let a27 = {
                                                            let a25 = {
                                                                let a23 = 1u64;
                                                                let a24 = 1u64;
                                                                nat_add(a23, a24)?
                                                            };
                                                            let a26 = 1u64;
                                                            nat_mul(a25, a26)?
                                                        };
                                                        let a28 = 1u64;
                                                        nat_add(a27, a28)?
                                                    };
                                                    nat_mul(a29, a30)?
                                                };
                                                let a32 = 1u64;
                                                nat_add(a31, a32)?
                                            };
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
    if nat_eq(a53, a54) {
        let a55 = 0u64;
        let a56 = f7(v0)?;
        Ok(nat_lt(a55, a56))
    } else {
        Ok(false)
    }
}

pub fn f3(v0: u64) -> R<bool> {
    let a57 = f5(v0)?;
    let a58 = 0u64;
    Ok(nat_eq(a57, a58))
}

pub fn f4(v0: u64) -> R<u64> {
    let a83 = v0;
    let a84 = f8({
        let a81 = {
            let a79 = {
                let a59 = 1u64;
                let a60 = 1u64;
                nat_add(a59, a60)?
            };
            let a80 = {
                let a77 = {
                    let a75 = {
                        let a61 = 1u64;
                        let a62 = 1u64;
                        nat_add(a61, a62)?
                    };
                    let a76 = {
                        let a73 = {
                            let a71 = {
                                let a63 = 1u64;
                                let a64 = 1u64;
                                nat_add(a63, a64)?
                            };
                            let a72 = {
                                let a69 = {
                                    let a67 = {
                                        let a65 = 1u64;
                                        let a66 = 1u64;
                                        nat_add(a65, a66)?
                                    };
                                    let a68 = 1u64;
                                    nat_mul(a67, a68)?
                                };
                                let a70 = 1u64;
                                nat_add(a69, a70)?
                            };
                            nat_mul(a71, a72)?
                        };
                        let a74 = 1u64;
                        nat_add(a73, a74)?
                    };
                    nat_mul(a75, a76)?
                };
                let a78 = 1u64;
                nat_add(a77, a78)?
            };
            nat_mul(a79, a80)?
        };
        let a82 = 1u64;
        nat_add(a81, a82)?
    })?;
    let a85 = 0u64;
    Ok(nat_quot(a83, a84, a85))
}

pub fn f5(v0: u64) -> R<u64> {
    let a110 = v0;
    let a111 = f8({
        let a108 = {
            let a106 = {
                let a86 = 1u64;
                let a87 = 1u64;
                nat_add(a86, a87)?
            };
            let a107 = {
                let a104 = {
                    let a102 = {
                        let a88 = 1u64;
                        let a89 = 1u64;
                        nat_add(a88, a89)?
                    };
                    let a103 = {
                        let a100 = {
                            let a98 = {
                                let a90 = 1u64;
                                let a91 = 1u64;
                                nat_add(a90, a91)?
                            };
                            let a99 = {
                                let a96 = {
                                    let a94 = {
                                        let a92 = 1u64;
                                        let a93 = 1u64;
                                        nat_add(a92, a93)?
                                    };
                                    let a95 = 1u64;
                                    nat_mul(a94, a95)?
                                };
                                let a97 = 1u64;
                                nat_add(a96, a97)?
                            };
                            nat_mul(a98, a99)?
                        };
                        let a101 = 1u64;
                        nat_add(a100, a101)?
                    };
                    nat_mul(a102, a103)?
                };
                let a105 = 1u64;
                nat_add(a104, a105)?
            };
            nat_mul(a106, a107)?
        };
        let a109 = 1u64;
        nat_add(a108, a109)?
    })?;
    let a112 = 0u64;
    Ok(nat_rem(a110, a111, a112))
}

pub fn f6(v0: u64) -> R<u64> {
    let a135 = f5(v0)?;
    let a136 = f8({
        let a133 = {
            let a131 = {
                let a113 = 1u64;
                let a114 = 1u64;
                nat_add(a113, a114)?
            };
            let a132 = {
                let a129 = {
                    let a127 = {
                        let a115 = 1u64;
                        let a116 = 1u64;
                        nat_add(a115, a116)?
                    };
                    let a128 = {
                        let a125 = {
                            let a123 = {
                                let a117 = 1u64;
                                let a118 = 1u64;
                                nat_add(a117, a118)?
                            };
                            let a124 = {
                                let a121 = {
                                    let a119 = 1u64;
                                    let a120 = 1u64;
                                    nat_add(a119, a120)?
                                };
                                let a122 = 1u64;
                                nat_mul(a121, a122)?
                            };
                            nat_mul(a123, a124)?
                        };
                        let a126 = 1u64;
                        nat_add(a125, a126)?
                    };
                    nat_mul(a127, a128)?
                };
                let a130 = 1u64;
                nat_add(a129, a130)?
            };
            nat_mul(a131, a132)?
        };
        let a134 = 1u64;
        nat_add(a133, a134)?
    })?;
    let a137 = 0u64;
    Ok(nat_quot(a135, a136, a137))
}

pub fn f7(v0: u64) -> R<u64> {
    let a160 = v0;
    let a161 = f8({
        let a158 = {
            let a156 = {
                let a138 = 1u64;
                let a139 = 1u64;
                nat_add(a138, a139)?
            };
            let a157 = {
                let a154 = {
                    let a152 = {
                        let a140 = 1u64;
                        let a141 = 1u64;
                        nat_add(a140, a141)?
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = {
                                let a142 = 1u64;
                                let a143 = 1u64;
                                nat_add(a142, a143)?
                            };
                            let a149 = {
                                let a146 = {
                                    let a144 = 1u64;
                                    let a145 = 1u64;
                                    nat_add(a144, a145)?
                                };
                                let a147 = 1u64;
                                nat_mul(a146, a147)?
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
            };
            nat_mul(a156, a157)?
        };
        let a159 = 1u64;
        nat_add(a158, a159)?
    })?;
    let a162 = 0u64;
    Ok(nat_rem(a160, a161, a162))
}

pub fn f8(v0: u64) -> R<u64> {
    f9(Fn0::F10, v0, 1u64)
}

pub fn f9(v0: Fn0, p166: u64, p167: u64) -> R<u64> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v1, v2) = m165;
                let m163 = v1;
                if m163 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m163 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c164: Fn0 = v0.clone();
                        c164.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let a168 = v0;
    let a169 = v0;
    nat_add(a168, a169)
}

pub fn run(p0: bool, p1: u64, p2: u64) -> R<u64> {
    f0(p0, p1, p2)
}
