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
pub enum Adt0 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 2u64;
    if nat_eq(a1, a2) {
        f1(v1, v2)
    } else {
        let a3 = v0;
        let a4 = 4u64;
        if if nat_eq(a3, a4) {
            true
        } else {
            let a5 = v0;
            let a6 = 5u64;
            nat_eq(a5, a6)
        } {
            f2(v0, v1, v2)
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<Adt0> {
    let a7 = v0;
    let a8 = 16u64;
    if nat_lt(a7, a8) {
        let a9 = v1;
        let a10 = 1u64;
        if nat_lt(a9, a10) {
            Ok(Adt0::C2(1u64, v0, 1u64, 16u64, 1u64))
        } else {
            let a11 = v1;
            let a12 = 2u64;
            if nat_lt(a11, a12) {
                Ok(Adt0::C2(2u64, v0, 1u64, 16u64, 16u64))
            } else {
                Ok(Adt0::C1)
            }
        }
    } else {
        let a13 = v0;
        let a14 = 80u64;
        if nat_lt(a13, a14) {
            let a15 = v1;
            let a16 = 4u64;
            if nat_lt(a15, a16) {
                Ok(Adt0::C2(0u64, {
                    let a33 = {
                        let a29 = {
                            let a22 = {
                                let a19 = {
                                    let a17 = v0;
                                    let a18 = 16u64;
                                    nat_sub(a17, a18)
                                };
                                let a20 = 32u64;
                                let a21 = 0u64;
                                nat_quot(a19, a20, a21)
                            };
                            let a23 = 128u64;
                            nat_mul(a22, a23)?
                        };
                        let a30 = {
                            let a26 = {
                                let a24 = v0;
                                let a25 = 16u64;
                                nat_sub(a24, a25)
                            };
                            let a27 = 32u64;
                            let a28 = 0u64;
                            nat_rem(a26, a27, a28)
                        };
                        nat_add(a29, a30)?
                    };
                    let a34 = {
                        let a31 = v1;
                        let a32 = 32u64;
                        nat_mul(a31, a32)?
                    };
                    nat_add(a33, a34)?
                }, 1u64, 4u64, {
                    let a35 = v1;
                    let a36 = 2u64;
                    if nat_lt(a35, a36) {
                        let a37 = v1;
                        let a38 = 1u64;
                        if nat_lt(a37, a38) {
                            1u64
                        } else {
                            4u64
                        }
                    } else {
                        let a39 = v1;
                        let a40 = 3u64;
                        if nat_lt(a39, a40) {
                            16u64
                        } else {
                            64u64
                        }
                    }
                }))
            } else {
                Ok(Adt0::C1)
            }
        } else {
            let a41 = v0;
            let a42 = 82u64;
            if nat_lt(a41, a42) {
                let a43 = v1;
                let a44 = 0u64;
                if nat_eq(a43, a44) {
                    Ok(Adt0::C2(3u64, 0u64, {
                        let a45 = v0;
                        let a46 = 80u64;
                        if nat_eq(a45, a46) {
                            1u64
                        } else {
                            256u64
                        }
                    }, 256u64, 1u64))
                } else {
                    Ok(Adt0::C1)
                }
            } else {
                let a47 = v0;
                let a48 = 84u64;
                if nat_lt(a47, a48) {
                    let a49 = v1;
                    let a50 = 0u64;
                    if nat_eq(a49, a50) {
                        Ok(Adt0::C2(4u64, 0u64, {
                            let a51 = v0;
                            let a52 = 82u64;
                            if nat_eq(a51, a52) {
                                1u64
                            } else {
                                256u64
                            }
                        }, 256u64, 1u64))
                    } else {
                        Ok(Adt0::C1)
                    }
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let a53 = v1;
    let a54 = 2u64;
    if nat_lt(a53, a54) {
        let a55 = v2;
        let a56 = 0u64;
        if nat_eq(a55, a56) {
            Ok(Adt0::C2(3u64, 0u64, {
                let a57 = v1;
                let a58 = 0u64;
                if nat_eq(a57, a58) {
                    1u64
                } else {
                    256u64
                }
            }, 256u64, 1u64))
        } else {
            Ok(Adt0::C1)
        }
    } else {
        let a59 = v1;
        let a60 = 4u64;
        if nat_lt(a59, a60) {
            let a61 = v2;
            let a62 = 0u64;
            if nat_eq(a61, a62) {
                Ok(Adt0::C2(4u64, 0u64, {
                    let a63 = v1;
                    let a64 = 2u64;
                    if nat_eq(a63, a64) {
                        1u64
                    } else {
                        256u64
                    }
                }, 256u64, 1u64))
            } else {
                Ok(Adt0::C1)
            }
        } else {
            let a65 = v1;
            let a66 = 16u64;
            if nat_lt(a65, a66) {
                let a69 = {
                    let a67 = v1;
                    let a68 = 4u64;
                    nat_sub(a67, a68)
                };
                let a70 = 4u64;
                if nat_lt(a69, a70) {
                    let a71 = v2;
                    let a72 = 1u64;
                    if nat_lt(a71, a72) {
                        Ok(Adt0::C2(1u64, {
                            let a75 = {
                                let a73 = v1;
                                let a74 = 4u64;
                                nat_sub(a73, a74)
                            };
                            let a76 = 4u64;
                            let a77 = 0u64;
                            nat_rem(a75, a76, a77)
                        }, 1u64, 64u64, 1u64))
                    } else {
                        let a78 = v2;
                        let a79 = 2u64;
                        if nat_lt(a78, a79) {
                            Ok(Adt0::C2(1u64, {
                                let a85 = {
                                    let a82 = {
                                        let a80 = v1;
                                        let a81 = 4u64;
                                        nat_sub(a80, a81)
                                    };
                                    let a83 = 4u64;
                                    let a84 = 0u64;
                                    nat_rem(a82, a83, a84)
                                };
                                let a86 = 4u64;
                                nat_add(a85, a86)?
                            }, 16u64, 4u64, 64u64))
                        } else {
                            Ok(Adt0::C1)
                        }
                    }
                } else {
                    let a89 = {
                        let a87 = v1;
                        let a88 = 4u64;
                        nat_sub(a87, a88)
                    };
                    let a90 = 8u64;
                    if nat_lt(a89, a90) {
                        let a91 = v2;
                        let a92 = 1u64;
                        if nat_lt(a91, a92) {
                            Ok(Adt0::C2(2u64, {
                                let a95 = {
                                    let a93 = v1;
                                    let a94 = 4u64;
                                    nat_sub(a93, a94)
                                };
                                let a96 = 4u64;
                                let a97 = 0u64;
                                nat_rem(a95, a96, a97)
                            }, 1u64, 64u64, 1u64))
                        } else {
                            let a98 = v2;
                            let a99 = 2u64;
                            if nat_lt(a98, a99) {
                                Ok(Adt0::C2(2u64, {
                                    let a105 = {
                                        let a102 = {
                                            let a100 = v1;
                                            let a101 = 4u64;
                                            nat_sub(a100, a101)
                                        };
                                        let a103 = 4u64;
                                        let a104 = 0u64;
                                        nat_rem(a102, a103, a104)
                                    };
                                    let a106 = 4u64;
                                    nat_add(a105, a106)?
                                }, 16u64, 4u64, 64u64))
                            } else {
                                Ok(Adt0::C1)
                            }
                        }
                    } else {
                        let a107 = v2;
                        let a108 = 1u64;
                        if nat_lt(a107, a108) {
                            Ok(Adt0::C2(1u64, {
                                let a114 = {
                                    let a111 = {
                                        let a109 = v1;
                                        let a110 = 4u64;
                                        nat_sub(a109, a110)
                                    };
                                    let a112 = 4u64;
                                    let a113 = 0u64;
                                    nat_rem(a111, a112, a113)
                                };
                                let a115 = 4u64;
                                nat_add(a114, a115)?
                            }, 1u64, 16u64, 1u64))
                        } else {
                            let a116 = v2;
                            let a117 = 2u64;
                            if nat_lt(a116, a117) {
                                Ok(Adt0::C2(2u64, {
                                    let a123 = {
                                        let a120 = {
                                            let a118 = v1;
                                            let a119 = 4u64;
                                            nat_sub(a118, a119)
                                        };
                                        let a121 = 4u64;
                                        let a122 = 0u64;
                                        nat_rem(a120, a121, a122)
                                    };
                                    let a124 = 4u64;
                                    nat_add(a123, a124)?
                                }, 1u64, 16u64, 16u64))
                            } else {
                                Ok(Adt0::C1)
                            }
                        }
                    }
                }
            } else {
                let a125 = v0;
                let a126 = 5u64;
                if if nat_eq(a125, a126) {
                    let a127 = v1;
                    let a128 = 48u64;
                    nat_lt(a127, a128)
                } else {
                    false
                } {
                    let a129 = v2;
                    let a130 = 8u64;
                    if nat_lt(a129, a130) {
                        Ok(Adt0::C2(0u64, {
                            let a135 = {
                                let a131 = v1;
                                let a132 = 16u64;
                                nat_sub(a131, a132)
                            };
                            let a136 = {
                                let a133 = v2;
                                let a134 = 32u64;
                                nat_mul(a133, a134)?
                            };
                            nat_add(a135, a136)?
                        }, 16u64, 2u64, {
                            let a137 = v2;
                            let a138 = 4u64;
                            if nat_lt(a137, a138) {
                                let a139 = v2;
                                let a140 = 2u64;
                                if nat_lt(a139, a140) {
                                    let a141 = v2;
                                    let a142 = 1u64;
                                    if nat_lt(a141, a142) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                } else {
                                    let a143 = v2;
                                    let a144 = 3u64;
                                    if nat_lt(a143, a144) {
                                        4u64
                                    } else {
                                        8u64
                                    }
                                }
                            } else {
                                let a145 = v2;
                                let a146 = 6u64;
                                if nat_lt(a145, a146) {
                                    let a147 = v2;
                                    let a148 = 5u64;
                                    if nat_lt(a147, a148) {
                                        16u64
                                    } else {
                                        32u64
                                    }
                                } else {
                                    let a149 = v2;
                                    let a150 = 7u64;
                                    if nat_lt(a149, a150) {
                                        64u64
                                    } else {
                                        128u64
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(Adt0::C1)
                    }
                } else {
                    let a153 = v1;
                    let a154 = {
                        let a151 = v0;
                        let a152 = 4u64;
                        if nat_eq(a151, a152) {
                            144u64
                        } else {
                            176u64
                        }
                    };
                    if nat_lt(a153, a154) {
                        let a155 = v2;
                        let a156 = 2u64;
                        if nat_lt(a155, a156) {
                            Ok(Adt0::C2(0u64, {
                                let a177 = {
                                    let a173 = {
                                        let a164 = {
                                            let a161 = {
                                                let a159 = v1;
                                                let a160 = {
                                                    let a157 = v0;
                                                    let a158 = 4u64;
                                                    if nat_eq(a157, a158) {
                                                        16u64
                                                    } else {
                                                        48u64
                                                    }
                                                };
                                                nat_sub(a159, a160)
                                            };
                                            let a162 = 32u64;
                                            let a163 = 0u64;
                                            nat_quot(a161, a162, a163)
                                        };
                                        let a165 = 64u64;
                                        nat_mul(a164, a165)?
                                    };
                                    let a174 = {
                                        let a170 = {
                                            let a168 = v1;
                                            let a169 = {
                                                let a166 = v0;
                                                let a167 = 4u64;
                                                if nat_eq(a166, a167) {
                                                    16u64
                                                } else {
                                                    48u64
                                                }
                                            };
                                            nat_sub(a168, a169)
                                        };
                                        let a171 = 32u64;
                                        let a172 = 0u64;
                                        nat_rem(a170, a171, a172)
                                    };
                                    nat_add(a173, a174)?
                                };
                                let a178 = {
                                    let a175 = v2;
                                    let a176 = 32u64;
                                    nat_mul(a175, a176)?
                                };
                                nat_add(a177, a178)?
                            }, 1u64, 16u64, {
                                let a179 = v2;
                                let a180 = 0u64;
                                if nat_eq(a179, a180) {
                                    1u64
                                } else {
                                    16u64
                                }
                            }))
                        } else {
                            Ok(Adt0::C1)
                        }
                    } else {
                        Ok(Adt0::C0)
                    }
                }
            }
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64) -> R<Adt0> {
    f0(p0, p1, p2)
}
