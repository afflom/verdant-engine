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
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F26,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F26 => f26(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F31,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F31 => f31(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Adt0> {
    if f1(0u64, v0) {
        let v2: u64 = f2(v1)?;
        let v3: u64 = f3(f4(v0, v2)?)?;
        Ok(Adt0::C0(f4(v2, v0)?, v3, f5(v3)?))
    } else {
        Ok(Adt0::C0(0u64, 0u64, 0u64))
    }
}

pub fn f1(v0: u64, v1: u64) -> bool {
    let a1 = f6(v0, v1);
    let a2 = 0u64;
    nat_eq(a1, a2)
}

pub fn f2(v0: u64) -> R<u64> {
    let a3 = v0;
    let a4 = 0u64;
    if nat_eq(a3, a4) {
        Ok(0u64)
    } else {
        let a5 = v0;
        let a6 = 2u64;
        if nat_lt(a5, a6) {
            let a11 = 1065353216u64;
            let a12 = {
                let a9 = {
                    let a7 = v0;
                    let a8 = 1u64;
                    nat_sub(a7, a8)
                };
                let a10 = 8388608u64;
                nat_mul(a9, a10)?
            };
            nat_add(a11, a12)
        } else {
            let a13 = v0;
            let a14 = 4u64;
            if nat_lt(a13, a14) {
                let a19 = 1073741824u64;
                let a20 = {
                    let a17 = {
                        let a15 = v0;
                        let a16 = 2u64;
                        nat_sub(a15, a16)
                    };
                    let a18 = 4194304u64;
                    nat_mul(a17, a18)?
                };
                nat_add(a19, a20)
            } else {
                let a21 = v0;
                let a22 = 8u64;
                if nat_lt(a21, a22) {
                    let a27 = 1082130432u64;
                    let a28 = {
                        let a25 = {
                            let a23 = v0;
                            let a24 = 4u64;
                            nat_sub(a23, a24)
                        };
                        let a26 = 2097152u64;
                        nat_mul(a25, a26)?
                    };
                    nat_add(a27, a28)
                } else {
                    let a29 = v0;
                    let a30 = 16u64;
                    if nat_lt(a29, a30) {
                        let a35 = 1090519040u64;
                        let a36 = {
                            let a33 = {
                                let a31 = v0;
                                let a32 = 8u64;
                                nat_sub(a31, a32)
                            };
                            let a34 = 1048576u64;
                            nat_mul(a33, a34)?
                        };
                        nat_add(a35, a36)
                    } else {
                        let a37 = v0;
                        let a38 = 32u64;
                        if nat_lt(a37, a38) {
                            let a43 = 1098907648u64;
                            let a44 = {
                                let a41 = {
                                    let a39 = v0;
                                    let a40 = 16u64;
                                    nat_sub(a39, a40)
                                };
                                let a42 = 524288u64;
                                nat_mul(a41, a42)?
                            };
                            nat_add(a43, a44)
                        } else {
                            let a45 = v0;
                            let a46 = 64u64;
                            if nat_lt(a45, a46) {
                                let a51 = 1107296256u64;
                                let a52 = {
                                    let a49 = {
                                        let a47 = v0;
                                        let a48 = 32u64;
                                        nat_sub(a47, a48)
                                    };
                                    let a50 = 262144u64;
                                    nat_mul(a49, a50)?
                                };
                                nat_add(a51, a52)
                            } else {
                                let a53 = v0;
                                let a54 = 128u64;
                                if nat_lt(a53, a54) {
                                    let a59 = 1115684864u64;
                                    let a60 = {
                                        let a57 = {
                                            let a55 = v0;
                                            let a56 = 64u64;
                                            nat_sub(a55, a56)
                                        };
                                        let a58 = 131072u64;
                                        nat_mul(a57, a58)?
                                    };
                                    nat_add(a59, a60)
                                } else {
                                    let a61 = v0;
                                    let a62 = 256u64;
                                    if nat_lt(a61, a62) {
                                        let a67 = 1124073472u64;
                                        let a68 = {
                                            let a65 = {
                                                let a63 = v0;
                                                let a64 = 128u64;
                                                nat_sub(a63, a64)
                                            };
                                            let a66 = 65536u64;
                                            nat_mul(a65, a66)?
                                        };
                                        nat_add(a67, a68)
                                    } else {
                                        Ok(1132462080u64)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a69 = f7(v0)?;
        let a70 = 32768u64;
        nat_mul(a69, a70)?
    };
    let v2: u64 = f8(v0)?;
    let a89 = v1;
    let a90 = {
        let a71 = v2;
        let a72 = 255u64;
        if nat_eq(a71, a72) {
            let a73 = f9(v0)?;
            let a74 = 0u64;
            if nat_eq(a73, a74) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a75 = 143u64;
            let a76 = v2;
            if nat_le(a75, a76) {
                31744u64
            } else {
                let a77 = v2;
                let a78 = 102u64;
                if nat_lt(a77, a78) {
                    0u64
                } else {
                    let a79 = 113u64;
                    let a80 = v2;
                    if nat_le(a79, a80) {
                        let a85 = {
                            let a83 = {
                                let a81 = v2;
                                let a82 = 113u64;
                                nat_sub(a81, a82)
                            };
                            let a84 = 1024u64;
                            nat_mul(a83, a84)?
                        };
                        let a86 = f10(f11(v0)?, 8192u64)?;
                        nat_add(a85, a86)?
                    } else {
                        f10(f11(v0)?, f12({
                            let a87 = 126u64;
                            let a88 = v2;
                            nat_sub(a87, a88)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a89, a90)
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if if if f13(v0)? {
        true
    } else {
        f13(v1)?
    } {
        true
    } else {
        if f14(v0)? {
            f14(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f15(v0)? {
            f15(v1)?
        } else {
            false
        }
    } {
        let a177 = {
            let a155 = {
                let a131 = {
                    let a129 = {
                        let a91 = 1u64;
                        let a92 = 1u64;
                        nat_add(a91, a92)?
                    };
                    let a130 = {
                        let a127 = {
                            let a125 = {
                                let a93 = 1u64;
                                let a94 = 1u64;
                                nat_add(a93, a94)?
                            };
                            let a126 = {
                                let a123 = {
                                    let a121 = {
                                        let a95 = 1u64;
                                        let a96 = 1u64;
                                        nat_add(a95, a96)?
                                    };
                                    let a122 = {
                                        let a119 = {
                                            let a117 = {
                                                let a97 = 1u64;
                                                let a98 = 1u64;
                                                nat_add(a97, a98)?
                                            };
                                            let a118 = {
                                                let a115 = {
                                                    let a113 = {
                                                        let a99 = 1u64;
                                                        let a100 = 1u64;
                                                        nat_add(a99, a100)?
                                                    };
                                                    let a114 = {
                                                        let a111 = {
                                                            let a109 = {
                                                                let a101 = 1u64;
                                                                let a102 = 1u64;
                                                                nat_add(a101, a102)?
                                                            };
                                                            let a110 = {
                                                                let a107 = {
                                                                    let a105 = {
                                                                        let a103 = 1u64;
                                                                        let a104 = 1u64;
                                                                        nat_add(a103, a104)?
                                                                    };
                                                                    let a106 = 1u64;
                                                                    nat_mul(a105, a106)?
                                                                };
                                                                let a108 = 1u64;
                                                                nat_add(a107, a108)?
                                                            };
                                                            nat_mul(a109, a110)?
                                                        };
                                                        let a112 = 1u64;
                                                        nat_add(a111, a112)?
                                                    };
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
                    };
                    nat_mul(a129, a130)?
                };
                let a132 = 1u64;
                nat_add(a131, a132)?
            };
            let a156 = f12({
                let a153 = {
                    let a151 = {
                        let a133 = 1u64;
                        let a134 = 1u64;
                        nat_add(a133, a134)?
                    };
                    let a152 = {
                        let a149 = {
                            let a147 = {
                                let a135 = 1u64;
                                let a136 = 1u64;
                                nat_add(a135, a136)?
                            };
                            let a148 = {
                                let a145 = {
                                    let a143 = {
                                        let a137 = 1u64;
                                        let a138 = 1u64;
                                        nat_add(a137, a138)?
                                    };
                                    let a144 = {
                                        let a141 = {
                                            let a139 = 1u64;
                                            let a140 = 1u64;
                                            nat_add(a139, a140)?
                                        };
                                        let a142 = 1u64;
                                        nat_mul(a141, a142)?
                                    };
                                    nat_mul(a143, a144)?
                                };
                                let a146 = 1u64;
                                nat_add(a145, a146)?
                            };
                            nat_mul(a147, a148)?
                        };
                        let a150 = 1u64;
                        nat_add(a149, a150)?
                    };
                    nat_mul(a151, a152)?
                };
                let a154 = 1u64;
                nat_add(a153, a154)?
            })?;
            nat_mul(a155, a156)?
        };
        let a178 = f12({
            let a175 = {
                let a157 = 1u64;
                let a158 = 1u64;
                nat_add(a157, a158)?
            };
            let a176 = {
                let a173 = {
                    let a171 = {
                        let a159 = 1u64;
                        let a160 = 1u64;
                        nat_add(a159, a160)?
                    };
                    let a172 = {
                        let a169 = {
                            let a167 = {
                                let a161 = 1u64;
                                let a162 = 1u64;
                                nat_add(a161, a162)?
                            };
                            let a168 = {
                                let a165 = {
                                    let a163 = 1u64;
                                    let a164 = 1u64;
                                    nat_add(a163, a164)?
                                };
                                let a166 = 1u64;
                                nat_mul(a165, a166)?
                            };
                            nat_mul(a167, a168)?
                        };
                        let a170 = 1u64;
                        nat_add(a169, a170)?
                    };
                    nat_mul(a171, a172)?
                };
                let a174 = 1u64;
                nat_add(a173, a174)?
            };
            nat_mul(a175, a176)?
        })?;
        nat_add(a177, a178)
    } else {
        if if f14(v0)? {
            true
        } else {
            f15(v1)?
        } {
            f16(f17(v0, v1)?)
        } else {
            if if f14(v1)? {
                true
            } else {
                f15(v0)?
            } {
                f18(f17(v0, v1)?)
            } else {
                f19(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a182 = {
            let a179 = v0;
            let a180 = 32768u64;
            let a181 = 0u64;
            nat_quot(a179, a180, a181)
        };
        let a183 = 2147483648u64;
        nat_mul(a182, a183)?
    };
    let v2: u64 = {
        let a187 = {
            let a184 = v0;
            let a185 = 32768u64;
            let a186 = 0u64;
            nat_rem(a184, a185, a186)
        };
        let a188 = 1024u64;
        let a189 = 0u64;
        nat_quot(a187, a188, a189)
    };
    let v3: u64 = {
        let a190 = v0;
        let a191 = 1024u64;
        let a192 = 0u64;
        nat_rem(a190, a191, a192)
    };
    let a227 = v1;
    let a228 = {
        let a193 = v2;
        let a194 = 31u64;
        if nat_eq(a193, a194) {
            let a203 = 2139095040u64;
            let a204 = {
                let a201 = {
                    let a195 = 0u64;
                    let a196 = v3;
                    if if nat_lt(a195, a196) {
                        let a197 = v3;
                        let a198 = 512u64;
                        nat_lt(a197, a198)
                    } else {
                        false
                    } {
                        let a199 = v3;
                        let a200 = 512u64;
                        nat_add(a199, a200)?
                    } else {
                        v3
                    }
                };
                let a202 = 8192u64;
                nat_mul(a201, a202)?
            };
            nat_add(a203, a204)?
        } else {
            let a205 = v2;
            let a206 = 0u64;
            if nat_eq(a205, a206) {
                let a207 = v3;
                let a208 = 0u64;
                if nat_eq(a207, a208) {
                    0u64
                } else {
                    let v4: Adt1 = f20(113u64, v3, 0u64, 1024u64)?;
                    let a217 = {
                        let a210 = match v4.clone() {
                            Adt1::C0(h209, _, _, _) => {
                                h209
                            }
                        };
                        let a211 = 8388608u64;
                        nat_mul(a210, a211)?
                    };
                    let a218 = {
                        let a215 = {
                            let a213 = match v4 {
                                Adt1::C0(_, h212, _, _) => {
                                    h212
                                }
                            };
                            let a214 = 1024u64;
                            nat_sub(a213, a214)
                        };
                        let a216 = 8192u64;
                        nat_mul(a215, a216)?
                    };
                    nat_add(a217, a218)?
                }
            } else {
                let a225 = {
                    let a221 = {
                        let a219 = v2;
                        let a220 = 112u64;
                        nat_add(a219, a220)?
                    };
                    let a222 = 8388608u64;
                    nat_mul(a221, a222)?
                };
                let a226 = {
                    let a223 = v3;
                    let a224 = 8192u64;
                    nat_mul(a223, a224)?
                };
                nat_add(a225, a226)?
            }
        }
    };
    nat_add(a227, a228)
}

pub fn f6(v0: u64, v1: u64) -> u64 {
    if f21(v1) {
        3u64
    } else {
        if f21(v0) {
            4u64
        } else {
            let a229 = v0;
            let a230 = v1;
            if if nat_eq(a229, a230) {
                true
            } else {
                let a231 = f22(v0);
                let a232 = 0u64;
                if nat_eq(a231, a232) {
                    let a233 = f22(v1);
                    let a234 = 0u64;
                    nat_eq(a233, a234)
                } else {
                    false
                }
            } {
                1u64
            } else {
                let a235 = f23(v0);
                let a236 = f23(v1);
                if nat_eq(a235, a236) {
                    let a237 = f23(v0);
                    let a238 = 0u64;
                    if nat_eq(a237, a238) {
                        let a239 = f22(v0);
                        let a240 = f22(v1);
                        if nat_lt(a239, a240) {
                            0u64
                        } else {
                            2u64
                        }
                    } else {
                        let a241 = f22(v1);
                        let a242 = f22(v0);
                        if nat_lt(a241, a242) {
                            0u64
                        } else {
                            2u64
                        }
                    }
                } else {
                    let a243 = f23(v0);
                    let a244 = 1u64;
                    if nat_eq(a243, a244) {
                        0u64
                    } else {
                        2u64
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<u64> {
    let a269 = v0;
    let a270 = f12({
        let a267 = {
            let a265 = {
                let a245 = 1u64;
                let a246 = 1u64;
                nat_add(a245, a246)?
            };
            let a266 = {
                let a263 = {
                    let a261 = {
                        let a247 = 1u64;
                        let a248 = 1u64;
                        nat_add(a247, a248)?
                    };
                    let a262 = {
                        let a259 = {
                            let a257 = {
                                let a249 = 1u64;
                                let a250 = 1u64;
                                nat_add(a249, a250)?
                            };
                            let a258 = {
                                let a255 = {
                                    let a253 = {
                                        let a251 = 1u64;
                                        let a252 = 1u64;
                                        nat_add(a251, a252)?
                                    };
                                    let a254 = 1u64;
                                    nat_mul(a253, a254)?
                                };
                                let a256 = 1u64;
                                nat_add(a255, a256)?
                            };
                            nat_mul(a257, a258)?
                        };
                        let a260 = 1u64;
                        nat_add(a259, a260)?
                    };
                    nat_mul(a261, a262)?
                };
                let a264 = 1u64;
                nat_add(a263, a264)?
            };
            nat_mul(a265, a266)?
        };
        let a268 = 1u64;
        nat_add(a267, a268)?
    })?;
    let a271 = 0u64;
    Ok(nat_quot(a269, a270, a271))
}

pub fn f8(v0: u64) -> R<u64> {
    let a294 = f24(v0)?;
    let a295 = f12({
        let a292 = {
            let a290 = {
                let a272 = 1u64;
                let a273 = 1u64;
                nat_add(a272, a273)?
            };
            let a291 = {
                let a288 = {
                    let a286 = {
                        let a274 = 1u64;
                        let a275 = 1u64;
                        nat_add(a274, a275)?
                    };
                    let a287 = {
                        let a284 = {
                            let a282 = {
                                let a276 = 1u64;
                                let a277 = 1u64;
                                nat_add(a276, a277)?
                            };
                            let a283 = {
                                let a280 = {
                                    let a278 = 1u64;
                                    let a279 = 1u64;
                                    nat_add(a278, a279)?
                                };
                                let a281 = 1u64;
                                nat_mul(a280, a281)?
                            };
                            nat_mul(a282, a283)?
                        };
                        let a285 = 1u64;
                        nat_add(a284, a285)?
                    };
                    nat_mul(a286, a287)?
                };
                let a289 = 1u64;
                nat_add(a288, a289)?
            };
            nat_mul(a290, a291)?
        };
        let a293 = 1u64;
        nat_add(a292, a293)?
    })?;
    let a296 = 0u64;
    Ok(nat_quot(a294, a295, a296))
}

pub fn f9(v0: u64) -> R<u64> {
    let a319 = v0;
    let a320 = f12({
        let a317 = {
            let a315 = {
                let a297 = 1u64;
                let a298 = 1u64;
                nat_add(a297, a298)?
            };
            let a316 = {
                let a313 = {
                    let a311 = {
                        let a299 = 1u64;
                        let a300 = 1u64;
                        nat_add(a299, a300)?
                    };
                    let a312 = {
                        let a309 = {
                            let a307 = {
                                let a301 = 1u64;
                                let a302 = 1u64;
                                nat_add(a301, a302)?
                            };
                            let a308 = {
                                let a305 = {
                                    let a303 = 1u64;
                                    let a304 = 1u64;
                                    nat_add(a303, a304)?
                                };
                                let a306 = 1u64;
                                nat_mul(a305, a306)?
                            };
                            nat_mul(a307, a308)?
                        };
                        let a310 = 1u64;
                        nat_add(a309, a310)?
                    };
                    nat_mul(a311, a312)?
                };
                let a314 = 1u64;
                nat_add(a313, a314)?
            };
            nat_mul(a315, a316)?
        };
        let a318 = 1u64;
        nat_add(a317, a318)?
    })?;
    let a321 = 0u64;
    Ok(nat_rem(a319, a320, a321))
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a322 = v0;
        let a323 = v1;
        let a324 = 0u64;
        nat_quot(a322, a323, a324)
    };
    let v3: u64 = {
        let a328 = {
            let a325 = v0;
            let a326 = v1;
            let a327 = 0u64;
            nat_rem(a325, a326, a327)
        };
        let a329 = 2u64;
        nat_mul(a328, a329)?
    };
    let a339 = v2;
    let a340 = {
        let a330 = v1;
        let a331 = v3;
        if if nat_lt(a330, a331) {
            true
        } else {
            let a332 = v3;
            let a333 = v1;
            if nat_eq(a332, a333) {
                let a337 = {
                    let a334 = v2;
                    let a335 = 2u64;
                    let a336 = 0u64;
                    nat_rem(a334, a335, a336)
                };
                let a338 = 1u64;
                nat_eq(a337, a338)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a339, a340)
}

pub fn f11(v0: u64) -> R<u64> {
    let a365 = f9(v0)?;
    let a366 = {
        let a341 = f8(v0)?;
        let a342 = 0u64;
        if nat_eq(a341, a342) {
            0u64
        } else {
            f12({
                let a363 = {
                    let a361 = {
                        let a343 = 1u64;
                        let a344 = 1u64;
                        nat_add(a343, a344)?
                    };
                    let a362 = {
                        let a359 = {
                            let a357 = {
                                let a345 = 1u64;
                                let a346 = 1u64;
                                nat_add(a345, a346)?
                            };
                            let a358 = {
                                let a355 = {
                                    let a353 = {
                                        let a347 = 1u64;
                                        let a348 = 1u64;
                                        nat_add(a347, a348)?
                                    };
                                    let a354 = {
                                        let a351 = {
                                            let a349 = 1u64;
                                            let a350 = 1u64;
                                            nat_add(a349, a350)?
                                        };
                                        let a352 = 1u64;
                                        nat_mul(a351, a352)?
                                    };
                                    nat_mul(a353, a354)?
                                };
                                let a356 = 1u64;
                                nat_add(a355, a356)?
                            };
                            nat_mul(a357, a358)?
                        };
                        let a360 = 1u64;
                        nat_add(a359, a360)?
                    };
                    nat_mul(a361, a362)?
                };
                let a364 = 1u64;
                nat_add(a363, a364)?
            })?
        }
    };
    nat_add(a365, a366)
}

pub fn f12(v0: u64) -> R<u64> {
    f25(Fn0::F26, v0, 1u64)
}

pub fn f13(v0: u64) -> R<bool> {
    let a409 = f8(v0)?;
    let a410 = {
        let a407 = {
            let a405 = {
                let a367 = 1u64;
                let a368 = 1u64;
                nat_add(a367, a368)?
            };
            let a406 = {
                let a403 = {
                    let a401 = {
                        let a369 = 1u64;
                        let a370 = 1u64;
                        nat_add(a369, a370)?
                    };
                    let a402 = {
                        let a399 = {
                            let a397 = {
                                let a371 = 1u64;
                                let a372 = 1u64;
                                nat_add(a371, a372)?
                            };
                            let a398 = {
                                let a395 = {
                                    let a393 = {
                                        let a373 = 1u64;
                                        let a374 = 1u64;
                                        nat_add(a373, a374)?
                                    };
                                    let a394 = {
                                        let a391 = {
                                            let a389 = {
                                                let a375 = 1u64;
                                                let a376 = 1u64;
                                                nat_add(a375, a376)?
                                            };
                                            let a390 = {
                                                let a387 = {
                                                    let a385 = {
                                                        let a377 = 1u64;
                                                        let a378 = 1u64;
                                                        nat_add(a377, a378)?
                                                    };
                                                    let a386 = {
                                                        let a383 = {
                                                            let a381 = {
                                                                let a379 = 1u64;
                                                                let a380 = 1u64;
                                                                nat_add(a379, a380)?
                                                            };
                                                            let a382 = 1u64;
                                                            nat_mul(a381, a382)?
                                                        };
                                                        let a384 = 1u64;
                                                        nat_add(a383, a384)?
                                                    };
                                                    nat_mul(a385, a386)?
                                                };
                                                let a388 = 1u64;
                                                nat_add(a387, a388)?
                                            };
                                            nat_mul(a389, a390)?
                                        };
                                        let a392 = 1u64;
                                        nat_add(a391, a392)?
                                    };
                                    nat_mul(a393, a394)?
                                };
                                let a396 = 1u64;
                                nat_add(a395, a396)?
                            };
                            nat_mul(a397, a398)?
                        };
                        let a400 = 1u64;
                        nat_add(a399, a400)?
                    };
                    nat_mul(a401, a402)?
                };
                let a404 = 1u64;
                nat_add(a403, a404)?
            };
            nat_mul(a405, a406)?
        };
        let a408 = 1u64;
        nat_add(a407, a408)?
    };
    if nat_eq(a409, a410) {
        let a411 = 0u64;
        let a412 = f9(v0)?;
        Ok(nat_lt(a411, a412))
    } else {
        Ok(false)
    }
}

pub fn f14(v0: u64) -> R<bool> {
    let a455 = f8(v0)?;
    let a456 = {
        let a453 = {
            let a451 = {
                let a413 = 1u64;
                let a414 = 1u64;
                nat_add(a413, a414)?
            };
            let a452 = {
                let a449 = {
                    let a447 = {
                        let a415 = 1u64;
                        let a416 = 1u64;
                        nat_add(a415, a416)?
                    };
                    let a448 = {
                        let a445 = {
                            let a443 = {
                                let a417 = 1u64;
                                let a418 = 1u64;
                                nat_add(a417, a418)?
                            };
                            let a444 = {
                                let a441 = {
                                    let a439 = {
                                        let a419 = 1u64;
                                        let a420 = 1u64;
                                        nat_add(a419, a420)?
                                    };
                                    let a440 = {
                                        let a437 = {
                                            let a435 = {
                                                let a421 = 1u64;
                                                let a422 = 1u64;
                                                nat_add(a421, a422)?
                                            };
                                            let a436 = {
                                                let a433 = {
                                                    let a431 = {
                                                        let a423 = 1u64;
                                                        let a424 = 1u64;
                                                        nat_add(a423, a424)?
                                                    };
                                                    let a432 = {
                                                        let a429 = {
                                                            let a427 = {
                                                                let a425 = 1u64;
                                                                let a426 = 1u64;
                                                                nat_add(a425, a426)?
                                                            };
                                                            let a428 = 1u64;
                                                            nat_mul(a427, a428)?
                                                        };
                                                        let a430 = 1u64;
                                                        nat_add(a429, a430)?
                                                    };
                                                    nat_mul(a431, a432)?
                                                };
                                                let a434 = 1u64;
                                                nat_add(a433, a434)?
                                            };
                                            nat_mul(a435, a436)?
                                        };
                                        let a438 = 1u64;
                                        nat_add(a437, a438)?
                                    };
                                    nat_mul(a439, a440)?
                                };
                                let a442 = 1u64;
                                nat_add(a441, a442)?
                            };
                            nat_mul(a443, a444)?
                        };
                        let a446 = 1u64;
                        nat_add(a445, a446)?
                    };
                    nat_mul(a447, a448)?
                };
                let a450 = 1u64;
                nat_add(a449, a450)?
            };
            nat_mul(a451, a452)?
        };
        let a454 = 1u64;
        nat_add(a453, a454)?
    };
    if nat_eq(a455, a456) {
        let a457 = f9(v0)?;
        let a458 = 0u64;
        Ok(nat_eq(a457, a458))
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u64) -> R<bool> {
    let a459 = f24(v0)?;
    let a460 = 0u64;
    Ok(nat_eq(a459, a460))
}

pub fn f16(v0: u64) -> R<u64> {
    let a527 = f18(v0)?;
    let a528 = {
        let a525 = {
            let a501 = {
                let a499 = {
                    let a461 = 1u64;
                    let a462 = 1u64;
                    nat_add(a461, a462)?
                };
                let a500 = {
                    let a497 = {
                        let a495 = {
                            let a463 = 1u64;
                            let a464 = 1u64;
                            nat_add(a463, a464)?
                        };
                        let a496 = {
                            let a493 = {
                                let a491 = {
                                    let a465 = 1u64;
                                    let a466 = 1u64;
                                    nat_add(a465, a466)?
                                };
                                let a492 = {
                                    let a489 = {
                                        let a487 = {
                                            let a467 = 1u64;
                                            let a468 = 1u64;
                                            nat_add(a467, a468)?
                                        };
                                        let a488 = {
                                            let a485 = {
                                                let a483 = {
                                                    let a469 = 1u64;
                                                    let a470 = 1u64;
                                                    nat_add(a469, a470)?
                                                };
                                                let a484 = {
                                                    let a481 = {
                                                        let a479 = {
                                                            let a471 = 1u64;
                                                            let a472 = 1u64;
                                                            nat_add(a471, a472)?
                                                        };
                                                        let a480 = {
                                                            let a477 = {
                                                                let a475 = {
                                                                    let a473 = 1u64;
                                                                    let a474 = 1u64;
                                                                    nat_add(a473, a474)?
                                                                };
                                                                let a476 = 1u64;
                                                                nat_mul(a475, a476)?
                                                            };
                                                            let a478 = 1u64;
                                                            nat_add(a477, a478)?
                                                        };
                                                        nat_mul(a479, a480)?
                                                    };
                                                    let a482 = 1u64;
                                                    nat_add(a481, a482)?
                                                };
                                                nat_mul(a483, a484)?
                                            };
                                            let a486 = 1u64;
                                            nat_add(a485, a486)?
                                        };
                                        nat_mul(a487, a488)?
                                    };
                                    let a490 = 1u64;
                                    nat_add(a489, a490)?
                                };
                                nat_mul(a491, a492)?
                            };
                            let a494 = 1u64;
                            nat_add(a493, a494)?
                        };
                        nat_mul(a495, a496)?
                    };
                    let a498 = 1u64;
                    nat_add(a497, a498)?
                };
                nat_mul(a499, a500)?
            };
            let a502 = 1u64;
            nat_add(a501, a502)?
        };
        let a526 = f12({
            let a523 = {
                let a521 = {
                    let a503 = 1u64;
                    let a504 = 1u64;
                    nat_add(a503, a504)?
                };
                let a522 = {
                    let a519 = {
                        let a517 = {
                            let a505 = 1u64;
                            let a506 = 1u64;
                            nat_add(a505, a506)?
                        };
                        let a518 = {
                            let a515 = {
                                let a513 = {
                                    let a507 = 1u64;
                                    let a508 = 1u64;
                                    nat_add(a507, a508)?
                                };
                                let a514 = {
                                    let a511 = {
                                        let a509 = 1u64;
                                        let a510 = 1u64;
                                        nat_add(a509, a510)?
                                    };
                                    let a512 = 1u64;
                                    nat_mul(a511, a512)?
                                };
                                nat_mul(a513, a514)?
                            };
                            let a516 = 1u64;
                            nat_add(a515, a516)?
                        };
                        nat_mul(a517, a518)?
                    };
                    let a520 = 1u64;
                    nat_add(a519, a520)?
                };
                nat_mul(a521, a522)?
            };
            let a524 = 1u64;
            nat_add(a523, a524)?
        })?;
        nat_mul(a525, a526)?
    };
    nat_add(a527, a528)
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let a535 = {
        let a529 = f7(v0)?;
        let a530 = f7(v1)?;
        nat_add(a529, a530)?
    };
    let a536 = {
        let a533 = {
            let a531 = 1u64;
            let a532 = 1u64;
            nat_add(a531, a532)?
        };
        let a534 = 1u64;
        nat_mul(a533, a534)?
    };
    let a537 = 0u64;
    Ok(nat_rem(a535, a536, a537))
}

pub fn f18(v0: u64) -> R<u64> {
    let a562 = v0;
    let a563 = f12({
        let a560 = {
            let a558 = {
                let a538 = 1u64;
                let a539 = 1u64;
                nat_add(a538, a539)?
            };
            let a559 = {
                let a556 = {
                    let a554 = {
                        let a540 = 1u64;
                        let a541 = 1u64;
                        nat_add(a540, a541)?
                    };
                    let a555 = {
                        let a552 = {
                            let a550 = {
                                let a542 = 1u64;
                                let a543 = 1u64;
                                nat_add(a542, a543)?
                            };
                            let a551 = {
                                let a548 = {
                                    let a546 = {
                                        let a544 = 1u64;
                                        let a545 = 1u64;
                                        nat_add(a544, a545)?
                                    };
                                    let a547 = 1u64;
                                    nat_mul(a546, a547)?
                                };
                                let a549 = 1u64;
                                nat_add(a548, a549)?
                            };
                            nat_mul(a550, a551)?
                        };
                        let a553 = 1u64;
                        nat_add(a552, a553)?
                    };
                    nat_mul(a554, a555)?
                };
                let a557 = 1u64;
                nat_add(a556, a557)?
            };
            nat_mul(a558, a559)?
        };
        let a561 = 1u64;
        nat_add(a560, a561)?
    })?;
    nat_mul(a562, a563)
}

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f27(v0)?;
    let v3: Adt1 = f27(v1)?;
    let v4: bool = {
        let a566 = match v2.clone() {
            Adt1::C0(_, h564, _, _) => {
                h564
            }
        };
        let a567 = match v3.clone() {
            Adt1::C0(_, h565, _, _) => {
                h565
            }
        };
        nat_lt(a566, a567)
    };
    let v5: u64 = {
        let a611 = match v2.clone() {
            Adt1::C0(_, h568, _, _) => {
                h568
            }
        };
        let a612 = f12(if v4 {
            let a589 = {
                let a587 = {
                    let a569 = 1u64;
                    let a570 = 1u64;
                    nat_add(a569, a570)?
                };
                let a588 = {
                    let a585 = {
                        let a583 = {
                            let a571 = 1u64;
                            let a572 = 1u64;
                            nat_add(a571, a572)?
                        };
                        let a584 = {
                            let a581 = {
                                let a573 = 1u64;
                                let a574 = 1u64;
                                nat_add(a573, a574)?
                            };
                            let a582 = {
                                let a579 = {
                                    let a577 = {
                                        let a575 = 1u64;
                                        let a576 = 1u64;
                                        nat_add(a575, a576)?
                                    };
                                    let a578 = 1u64;
                                    nat_mul(a577, a578)?
                                };
                                let a580 = 1u64;
                                nat_add(a579, a580)?
                            };
                            nat_mul(a581, a582)?
                        };
                        nat_mul(a583, a584)?
                    };
                    let a586 = 1u64;
                    nat_add(a585, a586)?
                };
                nat_mul(a587, a588)?
            };
            let a590 = 1u64;
            nat_add(a589, a590)?
        } else {
            let a609 = {
                let a591 = 1u64;
                let a592 = 1u64;
                nat_add(a591, a592)?
            };
            let a610 = {
                let a607 = {
                    let a605 = {
                        let a593 = 1u64;
                        let a594 = 1u64;
                        nat_add(a593, a594)?
                    };
                    let a606 = {
                        let a603 = {
                            let a595 = 1u64;
                            let a596 = 1u64;
                            nat_add(a595, a596)?
                        };
                        let a604 = {
                            let a601 = {
                                let a599 = {
                                    let a597 = 1u64;
                                    let a598 = 1u64;
                                    nat_add(a597, a598)?
                                };
                                let a600 = 1u64;
                                nat_mul(a599, a600)?
                            };
                            let a602 = 1u64;
                            nat_add(a601, a602)?
                        };
                        nat_mul(a603, a604)?
                    };
                    nat_mul(a605, a606)?
                };
                let a608 = 1u64;
                nat_add(a607, a608)?
            };
            nat_mul(a609, a610)?
        })?;
        nat_mul(a611, a612)?
    };
    let v6: u64 = {
        let a669 = {
            let a667 = {
                let a664 = match v2 {
                    Adt1::C0(h613, _, _, _) => {
                        h613
                    }
                };
                let a665 = {
                    let a662 = {
                        let a660 = {
                            let a614 = 1u64;
                            let a615 = 1u64;
                            nat_add(a614, a615)?
                        };
                        let a661 = {
                            let a658 = {
                                let a656 = {
                                    let a616 = 1u64;
                                    let a617 = 1u64;
                                    nat_add(a616, a617)?
                                };
                                let a657 = {
                                    let a654 = {
                                        let a652 = {
                                            let a618 = 1u64;
                                            let a619 = 1u64;
                                            nat_add(a618, a619)?
                                        };
                                        let a653 = {
                                            let a650 = {
                                                let a648 = {
                                                    let a620 = 1u64;
                                                    let a621 = 1u64;
                                                    nat_add(a620, a621)?
                                                };
                                                let a649 = {
                                                    let a646 = {
                                                        let a644 = {
                                                            let a622 = 1u64;
                                                            let a623 = 1u64;
                                                            nat_add(a622, a623)?
                                                        };
                                                        let a645 = {
                                                            let a642 = {
                                                                let a640 = {
                                                                    let a624 = 1u64;
                                                                    let a625 = 1u64;
                                                                    nat_add(a624, a625)?
                                                                };
                                                                let a641 = {
                                                                    let a638 = {
                                                                        let a636 = {
                                                                            let a626 = 1u64;
                                                                            let a627 = 1u64;
                                                                            nat_add(a626, a627)?
                                                                        };
                                                                        let a637 = {
                                                                            let a634 = {
                                                                                let a628 = 1u64;
                                                                                let a629 = 1u64;
                                                                                nat_add(a628, a629)?
                                                                            };
                                                                            let a635 = {
                                                                                let a632 = {
                                                                                    let a630 = 1u64;
                                                                                    let a631 = 1u64;
                                                                                    nat_add(a630, a631)?
                                                                                };
                                                                                let a633 = 1u64;
                                                                                nat_mul(a632, a633)?
                                                                            };
                                                                            nat_mul(a634, a635)?
                                                                        };
                                                                        nat_mul(a636, a637)?
                                                                    };
                                                                    let a639 = 1u64;
                                                                    nat_add(a638, a639)?
                                                                };
                                                                nat_mul(a640, a641)?
                                                            };
                                                            let a643 = 1u64;
                                                            nat_add(a642, a643)?
                                                        };
                                                        nat_mul(a644, a645)?
                                                    };
                                                    let a647 = 1u64;
                                                    nat_add(a646, a647)?
                                                };
                                                nat_mul(a648, a649)?
                                            };
                                            let a651 = 1u64;
                                            nat_add(a650, a651)?
                                        };
                                        nat_mul(a652, a653)?
                                    };
                                    let a655 = 1u64;
                                    nat_add(a654, a655)?
                                };
                                nat_mul(a656, a657)?
                            };
                            let a659 = 1u64;
                            nat_add(a658, a659)?
                        };
                        nat_mul(a660, a661)?
                    };
                    let a663 = 1u64;
                    nat_add(a662, a663)?
                };
                nat_add(a664, a665)?
            };
            let a668 = match v3.clone() {
                Adt1::C0(h666, _, _, _) => {
                    h666
                }
            };
            nat_sub(a667, a668)
        };
        let a670 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a669, a670)
    };
    let v7: u64 = {
        let a672 = v5;
        let a673 = match v3.clone() {
            Adt1::C0(_, h671, _, _) => {
                h671
            }
        };
        let a674 = 0u64;
        nat_quot(a672, a673, a674)
    };
    let v8: bool = {
        let a679 = 0u64;
        let a680 = {
            let a676 = v5;
            let a677 = match v3 {
                Adt1::C0(_, h675, _, _) => {
                    h675
                }
            };
            let a678 = 0u64;
            nat_rem(a676, a677, a678)
        };
        nat_lt(a679, a680)
    };
    f28(f17(v0, v1)?, v6, f29(v7, v8)?)
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h681, _) = f30(Fn1::F31, {
        let a700 = {
            let a682 = 1u64;
            let a683 = 1u64;
            nat_add(a682, a683)?
        };
        let a701 = {
            let a698 = {
                let a684 = 1u64;
                let a685 = 1u64;
                nat_add(a684, a685)?
            };
            let a699 = {
                let a696 = {
                    let a686 = 1u64;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                };
                let a697 = {
                    let a694 = {
                        let a688 = 1u64;
                        let a689 = 1u64;
                        nat_add(a688, a689)?
                    };
                    let a695 = {
                        let a692 = {
                            let a690 = 1u64;
                            let a691 = 1u64;
                            nat_add(a690, a691)?
                        };
                        let a693 = 1u64;
                        nat_mul(a692, a693)?
                    };
                    nat_mul(a694, a695)?
                };
                nat_mul(a696, a697)?
            };
            nat_mul(a698, a699)?
        };
        nat_mul(a700, a701)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h681)
}

pub fn f21(v0: u64) -> bool {
    let a702 = 2139095040u64;
    let a703 = f22(v0);
    nat_lt(a702, a703)
}

pub fn f22(v0: u64) -> u64 {
    let a704 = v0;
    let a705 = 2147483648u64;
    let a706 = 0u64;
    nat_rem(a704, a705, a706)
}

pub fn f23(v0: u64) -> u64 {
    let a707 = v0;
    let a708 = 2147483648u64;
    let a709 = 0u64;
    nat_quot(a707, a708, a709)
}

pub fn f24(v0: u64) -> R<u64> {
    let a734 = v0;
    let a735 = f12({
        let a732 = {
            let a730 = {
                let a710 = 1u64;
                let a711 = 1u64;
                nat_add(a710, a711)?
            };
            let a731 = {
                let a728 = {
                    let a726 = {
                        let a712 = 1u64;
                        let a713 = 1u64;
                        nat_add(a712, a713)?
                    };
                    let a727 = {
                        let a724 = {
                            let a722 = {
                                let a714 = 1u64;
                                let a715 = 1u64;
                                nat_add(a714, a715)?
                            };
                            let a723 = {
                                let a720 = {
                                    let a718 = {
                                        let a716 = 1u64;
                                        let a717 = 1u64;
                                        nat_add(a716, a717)?
                                    };
                                    let a719 = 1u64;
                                    nat_mul(a718, a719)?
                                };
                                let a721 = 1u64;
                                nat_add(a720, a721)?
                            };
                            nat_mul(a722, a723)?
                        };
                        let a725 = 1u64;
                        nat_add(a724, a725)?
                    };
                    nat_mul(a726, a727)?
                };
                let a729 = 1u64;
                nat_add(a728, a729)?
            };
            nat_mul(a730, a731)?
        };
        let a733 = 1u64;
        nat_add(a732, a733)?
    })?;
    let a736 = 0u64;
    Ok(nat_rem(a734, a735, a736))
}

pub fn f25(v0: Fn0, p740: u64, p741: u64) -> R<u64> {
    let m739 = (p740, p741);
    {
        let mut m739 = m739;
        loop {
            let step = {
                let (v1, v2) = m739;
                let m737 = v1;
                if m737 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m737 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c738: Fn0 = v0.clone();
                        c738.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f26(v0: u64) -> R<u64> {
    let a742 = v0;
    let a743 = v0;
    nat_add(a742, a743)
}

pub fn f27(v0: u64) -> R<Adt1> {
    f20(f32(v0)?, f11(v0)?, 0u64, f12({
        let a764 = {
            let a762 = {
                let a744 = 1u64;
                let a745 = 1u64;
                nat_add(a744, a745)?
            };
            let a763 = {
                let a760 = {
                    let a758 = {
                        let a746 = 1u64;
                        let a747 = 1u64;
                        nat_add(a746, a747)?
                    };
                    let a759 = {
                        let a756 = {
                            let a754 = {
                                let a748 = 1u64;
                                let a749 = 1u64;
                                nat_add(a748, a749)?
                            };
                            let a755 = {
                                let a752 = {
                                    let a750 = 1u64;
                                    let a751 = 1u64;
                                    nat_add(a750, a751)?
                                };
                                let a753 = 1u64;
                                nat_mul(a752, a753)?
                            };
                            nat_mul(a754, a755)?
                        };
                        let a757 = 1u64;
                        nat_add(a756, a757)?
                    };
                    nat_mul(a758, a759)?
                };
                let a761 = 1u64;
                nat_add(a760, a761)?
            };
            nat_mul(a762, a763)?
        };
        let a765 = 1u64;
        nat_add(a764, a765)?
    })?)
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a788 = f12({
        let a786 = {
            let a784 = {
                let a766 = 1u64;
                let a767 = 1u64;
                nat_add(a766, a767)?
            };
            let a785 = {
                let a782 = {
                    let a780 = {
                        let a768 = 1u64;
                        let a769 = 1u64;
                        nat_add(a768, a769)?
                    };
                    let a781 = {
                        let a778 = {
                            let a770 = 1u64;
                            let a771 = 1u64;
                            nat_add(a770, a771)?
                        };
                        let a779 = {
                            let a776 = {
                                let a774 = {
                                    let a772 = 1u64;
                                    let a773 = 1u64;
                                    nat_add(a772, a773)?
                                };
                                let a775 = 1u64;
                                nat_mul(a774, a775)?
                            };
                            let a777 = 1u64;
                            nat_add(a776, a777)?
                        };
                        nat_mul(a778, a779)?
                    };
                    nat_mul(a780, a781)?
                };
                let a783 = 1u64;
                nat_add(a782, a783)?
            };
            nat_mul(a784, a785)?
        };
        let a787 = 1u64;
        nat_add(a786, a787)?
    })?;
    let a789 = v2;
    if nat_le(a788, a789) {
        f33(v0, {
            let a790 = v1;
            let a791 = 1u64;
            nat_add(a790, a791)?
        }, f34(v2, 1u64)?)
    } else {
        let v3: Adt1 = f20(v1, v2, {
            let a828 = {
                let a826 = {
                    let a792 = 1u64;
                    let a793 = 1u64;
                    nat_add(a792, a793)?
                };
                let a827 = {
                    let a824 = {
                        let a794 = 1u64;
                        let a795 = 1u64;
                        nat_add(a794, a795)?
                    };
                    let a825 = {
                        let a822 = {
                            let a796 = 1u64;
                            let a797 = 1u64;
                            nat_add(a796, a797)?
                        };
                        let a823 = {
                            let a820 = {
                                let a798 = 1u64;
                                let a799 = 1u64;
                                nat_add(a798, a799)?
                            };
                            let a821 = {
                                let a818 = {
                                    let a800 = 1u64;
                                    let a801 = 1u64;
                                    nat_add(a800, a801)?
                                };
                                let a819 = {
                                    let a816 = {
                                        let a802 = 1u64;
                                        let a803 = 1u64;
                                        nat_add(a802, a803)?
                                    };
                                    let a817 = {
                                        let a814 = {
                                            let a804 = 1u64;
                                            let a805 = 1u64;
                                            nat_add(a804, a805)?
                                        };
                                        let a815 = {
                                            let a812 = {
                                                let a806 = 1u64;
                                                let a807 = 1u64;
                                                nat_add(a806, a807)?
                                            };
                                            let a813 = {
                                                let a810 = {
                                                    let a808 = 1u64;
                                                    let a809 = 1u64;
                                                    nat_add(a808, a809)?
                                                };
                                                let a811 = 1u64;
                                                nat_mul(a810, a811)?
                                            };
                                            nat_mul(a812, a813)?
                                        };
                                        nat_mul(a814, a815)?
                                    };
                                    nat_mul(a816, a817)?
                                };
                                nat_mul(a818, a819)?
                            };
                            nat_mul(a820, a821)?
                        };
                        nat_mul(a822, a823)?
                    };
                    nat_mul(a824, a825)?
                };
                nat_mul(a826, a827)?
            };
            let a829 = 1u64;
            nat_add(a828, a829)?
        }, f12({
            let a848 = {
                let a830 = 1u64;
                let a831 = 1u64;
                nat_add(a830, a831)?
            };
            let a849 = {
                let a846 = {
                    let a844 = {
                        let a832 = 1u64;
                        let a833 = 1u64;
                        nat_add(a832, a833)?
                    };
                    let a845 = {
                        let a842 = {
                            let a834 = 1u64;
                            let a835 = 1u64;
                            nat_add(a834, a835)?
                        };
                        let a843 = {
                            let a840 = {
                                let a838 = {
                                    let a836 = 1u64;
                                    let a837 = 1u64;
                                    nat_add(a836, a837)?
                                };
                                let a839 = 1u64;
                                nat_mul(a838, a839)?
                            };
                            let a841 = 1u64;
                            nat_add(a840, a841)?
                        };
                        nat_mul(a842, a843)?
                    };
                    nat_mul(a844, a845)?
                };
                let a847 = 1u64;
                nat_add(a846, a847)?
            };
            nat_mul(a848, a849)?
        })?)?;
        f33(v0, match v3.clone() {
            Adt1::C0(h850, _, _, _) => {
                h850
            }
        }, match v3 {
            Adt1::C0(_, h851, _, _) => {
                h851
            }
        })
    }
}

pub fn f29(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a859 = {
            let a856 = v0;
            let a857 = {
                let a854 = {
                    let a852 = 1u64;
                    let a853 = 1u64;
                    nat_add(a852, a853)?
                };
                let a855 = 1u64;
                nat_mul(a854, a855)?
            };
            let a858 = 0u64;
            nat_rem(a856, a857, a858)
        };
        let a860 = 0u64;
        nat_eq(a859, a860)
    } else {
        false
    } {
        let a861 = v0;
        let a862 = 1u64;
        nat_add(a861, a862)
    } else {
        Ok(v0)
    }
}

pub fn f30(v0: Fn1, p867: u64, p868: Adt1) -> R<(Adt1, bool)> {
    let m866 = (p867, p868);
    {
        let mut m866 = m866;
        loop {
            let step = {
                let (v1, v2) = m866;
                let m863 = v1;
                if m863 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m863 - 1;
                    let m865 = {
                        let c864: Fn1 = v0.clone();
                        c864.apply(v2.clone())?
                    };
                    match m865 {
                        None => {
                            Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt1), R<(Adt1, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt1) -> R<Option<Adt1>> {
    f35(v0)
}

pub fn f32(v0: u64) -> R<u64> {
    let a907 = {
        let a869 = f8(v0)?;
        let a870 = 0u64;
        if nat_eq(a869, a870) {
            1u64
        } else {
            f8(v0)?
        }
    };
    let a908 = {
        let a905 = {
            let a871 = 1u64;
            let a872 = 1u64;
            nat_add(a871, a872)?
        };
        let a906 = {
            let a903 = {
                let a873 = 1u64;
                let a874 = 1u64;
                nat_add(a873, a874)?
            };
            let a904 = {
                let a901 = {
                    let a875 = 1u64;
                    let a876 = 1u64;
                    nat_add(a875, a876)?
                };
                let a902 = {
                    let a899 = {
                        let a877 = 1u64;
                        let a878 = 1u64;
                        nat_add(a877, a878)?
                    };
                    let a900 = {
                        let a897 = {
                            let a879 = 1u64;
                            let a880 = 1u64;
                            nat_add(a879, a880)?
                        };
                        let a898 = {
                            let a895 = {
                                let a881 = 1u64;
                                let a882 = 1u64;
                                nat_add(a881, a882)?
                            };
                            let a896 = {
                                let a893 = {
                                    let a883 = 1u64;
                                    let a884 = 1u64;
                                    nat_add(a883, a884)?
                                };
                                let a894 = {
                                    let a891 = {
                                        let a885 = 1u64;
                                        let a886 = 1u64;
                                        nat_add(a885, a886)?
                                    };
                                    let a892 = {
                                        let a889 = {
                                            let a887 = 1u64;
                                            let a888 = 1u64;
                                            nat_add(a887, a888)?
                                        };
                                        let a890 = 1u64;
                                        nat_mul(a889, a890)?
                                    };
                                    nat_mul(a891, a892)?
                                };
                                nat_mul(a893, a894)?
                            };
                            nat_mul(a895, a896)?
                        };
                        nat_mul(a897, a898)?
                    };
                    nat_mul(a899, a900)?
                };
                nat_mul(a901, a902)?
            };
            nat_mul(a903, a904)?
        };
        nat_mul(a905, a906)?
    };
    nat_add(a907, a908)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a947 = v1;
    let a948 = {
        let a945 = {
            let a943 = {
                let a909 = 1u64;
                let a910 = 1u64;
                nat_add(a909, a910)?
            };
            let a944 = {
                let a941 = {
                    let a911 = 1u64;
                    let a912 = 1u64;
                    nat_add(a911, a912)?
                };
                let a942 = {
                    let a939 = {
                        let a913 = 1u64;
                        let a914 = 1u64;
                        nat_add(a913, a914)?
                    };
                    let a940 = {
                        let a937 = {
                            let a915 = 1u64;
                            let a916 = 1u64;
                            nat_add(a915, a916)?
                        };
                        let a938 = {
                            let a935 = {
                                let a917 = 1u64;
                                let a918 = 1u64;
                                nat_add(a917, a918)?
                            };
                            let a936 = {
                                let a933 = {
                                    let a919 = 1u64;
                                    let a920 = 1u64;
                                    nat_add(a919, a920)?
                                };
                                let a934 = {
                                    let a931 = {
                                        let a921 = 1u64;
                                        let a922 = 1u64;
                                        nat_add(a921, a922)?
                                    };
                                    let a932 = {
                                        let a929 = {
                                            let a923 = 1u64;
                                            let a924 = 1u64;
                                            nat_add(a923, a924)?
                                        };
                                        let a930 = {
                                            let a927 = {
                                                let a925 = 1u64;
                                                let a926 = 1u64;
                                                nat_add(a925, a926)?
                                            };
                                            let a928 = 1u64;
                                            nat_mul(a927, a928)?
                                        };
                                        nat_mul(a929, a930)?
                                    };
                                    nat_mul(a931, a932)?
                                };
                                nat_mul(a933, a934)?
                            };
                            nat_mul(a935, a936)?
                        };
                        nat_mul(a937, a938)?
                    };
                    nat_mul(a939, a940)?
                };
                nat_mul(a941, a942)?
            };
            nat_mul(a943, a944)?
        };
        let a946 = 1u64;
        nat_add(a945, a946)?
    };
    if nat_lt(a947, a948) {
        f36(v0, {
            let a985 = {
                let a983 = {
                    let a949 = 1u64;
                    let a950 = 1u64;
                    nat_add(a949, a950)?
                };
                let a984 = {
                    let a981 = {
                        let a951 = 1u64;
                        let a952 = 1u64;
                        nat_add(a951, a952)?
                    };
                    let a982 = {
                        let a979 = {
                            let a953 = 1u64;
                            let a954 = 1u64;
                            nat_add(a953, a954)?
                        };
                        let a980 = {
                            let a977 = {
                                let a955 = 1u64;
                                let a956 = 1u64;
                                nat_add(a955, a956)?
                            };
                            let a978 = {
                                let a975 = {
                                    let a957 = 1u64;
                                    let a958 = 1u64;
                                    nat_add(a957, a958)?
                                };
                                let a976 = {
                                    let a973 = {
                                        let a959 = 1u64;
                                        let a960 = 1u64;
                                        nat_add(a959, a960)?
                                    };
                                    let a974 = {
                                        let a971 = {
                                            let a961 = 1u64;
                                            let a962 = 1u64;
                                            nat_add(a961, a962)?
                                        };
                                        let a972 = {
                                            let a969 = {
                                                let a963 = 1u64;
                                                let a964 = 1u64;
                                                nat_add(a963, a964)?
                                            };
                                            let a970 = {
                                                let a967 = {
                                                    let a965 = 1u64;
                                                    let a966 = 1u64;
                                                    nat_add(a965, a966)?
                                                };
                                                let a968 = 1u64;
                                                nat_mul(a967, a968)?
                                            };
                                            nat_mul(a969, a970)?
                                        };
                                        nat_mul(a971, a972)?
                                    };
                                    nat_mul(a973, a974)?
                                };
                                nat_mul(a975, a976)?
                            };
                            nat_mul(a977, a978)?
                        };
                        nat_mul(a979, a980)?
                    };
                    nat_mul(a981, a982)?
                };
                nat_mul(a983, a984)?
            };
            let a986 = 1u64;
            nat_add(a985, a986)?
        }, f34(v2, {
            let a1025 = {
                let a1023 = {
                    let a1021 = {
                        let a987 = 1u64;
                        let a988 = 1u64;
                        nat_add(a987, a988)?
                    };
                    let a1022 = {
                        let a1019 = {
                            let a989 = 1u64;
                            let a990 = 1u64;
                            nat_add(a989, a990)?
                        };
                        let a1020 = {
                            let a1017 = {
                                let a991 = 1u64;
                                let a992 = 1u64;
                                nat_add(a991, a992)?
                            };
                            let a1018 = {
                                let a1015 = {
                                    let a993 = 1u64;
                                    let a994 = 1u64;
                                    nat_add(a993, a994)?
                                };
                                let a1016 = {
                                    let a1013 = {
                                        let a995 = 1u64;
                                        let a996 = 1u64;
                                        nat_add(a995, a996)?
                                    };
                                    let a1014 = {
                                        let a1011 = {
                                            let a997 = 1u64;
                                            let a998 = 1u64;
                                            nat_add(a997, a998)?
                                        };
                                        let a1012 = {
                                            let a1009 = {
                                                let a999 = 1u64;
                                                let a1000 = 1u64;
                                                nat_add(a999, a1000)?
                                            };
                                            let a1010 = {
                                                let a1007 = {
                                                    let a1001 = 1u64;
                                                    let a1002 = 1u64;
                                                    nat_add(a1001, a1002)?
                                                };
                                                let a1008 = {
                                                    let a1005 = {
                                                        let a1003 = 1u64;
                                                        let a1004 = 1u64;
                                                        nat_add(a1003, a1004)?
                                                    };
                                                    let a1006 = 1u64;
                                                    nat_mul(a1005, a1006)?
                                                };
                                                nat_mul(a1007, a1008)?
                                            };
                                            nat_mul(a1009, a1010)?
                                        };
                                        nat_mul(a1011, a1012)?
                                    };
                                    nat_mul(a1013, a1014)?
                                };
                                nat_mul(a1015, a1016)?
                            };
                            nat_mul(a1017, a1018)?
                        };
                        nat_mul(a1019, a1020)?
                    };
                    nat_mul(a1021, a1022)?
                };
                let a1024 = 1u64;
                nat_add(a1023, a1024)?
            };
            let a1026 = v1;
            nat_sub(a1025, a1026)
        })?)
    } else {
        f36(v0, v1, v2)
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a1027 = v1;
    let a1028 = 0u64;
    if nat_eq(a1027, a1028) {
        Ok(v0)
    } else {
        let a1053 = {
            let a1051 = {
                let a1029 = 1u64;
                let a1030 = 1u64;
                nat_add(a1029, a1030)?
            };
            let a1052 = {
                let a1049 = {
                    let a1031 = 1u64;
                    let a1032 = 1u64;
                    nat_add(a1031, a1032)?
                };
                let a1050 = {
                    let a1047 = {
                        let a1033 = 1u64;
                        let a1034 = 1u64;
                        nat_add(a1033, a1034)?
                    };
                    let a1048 = {
                        let a1045 = {
                            let a1035 = 1u64;
                            let a1036 = 1u64;
                            nat_add(a1035, a1036)?
                        };
                        let a1046 = {
                            let a1043 = {
                                let a1037 = 1u64;
                                let a1038 = 1u64;
                                nat_add(a1037, a1038)?
                            };
                            let a1044 = {
                                let a1041 = {
                                    let a1039 = 1u64;
                                    let a1040 = 1u64;
                                    nat_add(a1039, a1040)?
                                };
                                let a1042 = 1u64;
                                nat_mul(a1041, a1042)?
                            };
                            nat_mul(a1043, a1044)?
                        };
                        nat_mul(a1045, a1046)?
                    };
                    nat_mul(a1047, a1048)?
                };
                nat_mul(a1049, a1050)?
            };
            nat_mul(a1051, a1052)?
        };
        let a1054 = v1;
        if nat_le(a1053, a1054) {
            let a1055 = v0;
            let a1056 = 0u64;
            if nat_eq(a1055, a1056) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f12(v1)?;
            f29({
                let a1057 = v0;
                let a1058 = v2;
                let a1059 = 0u64;
                nat_quot(a1057, a1058, a1059)
            }, {
                let a1063 = 0u64;
                let a1064 = {
                    let a1060 = v0;
                    let a1061 = v2;
                    let a1062 = 0u64;
                    nat_rem(a1060, a1061, a1062)
                };
                nat_lt(a1063, a1064)
            })
        }
    }
}

pub fn f35(v0: Adt1) -> R<Option<Adt1>> {
    let a1066 = 0u64;
    let a1067 = match v0.clone() {
        Adt1::C0(_, h1065, _, _) => {
            h1065
        }
    };
    if if if nat_lt(a1066, a1067) {
        let a1070 = match v0.clone() {
            Adt1::C0(_, h1068, _, _) => {
                h1068
            }
        };
        let a1071 = match v0.clone() {
            Adt1::C0(_, _, _, h1069) => {
                h1069
            }
        };
        nat_lt(a1070, a1071)
    } else {
        false
    } {
        let a1074 = match v0.clone() {
            Adt1::C0(_, _, h1072, _) => {
                h1072
            }
        };
        let a1075 = match v0.clone() {
            Adt1::C0(h1073, _, _, _) => {
                h1073
            }
        };
        nat_lt(a1074, a1075)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1077 = match v0.clone() {
                Adt1::C0(h1076, _, _, _) => {
                    h1076
                }
            };
            let a1078 = 1u64;
            nat_sub(a1077, a1078)
        }, {
            let a1081 = match v0.clone() {
                Adt1::C0(_, h1079, _, _) => {
                    h1079
                }
            };
            let a1082 = match v0.clone() {
                Adt1::C0(_, h1080, _, _) => {
                    h1080
                }
            };
            nat_add(a1081, a1082)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1083, _) => {
                h1083
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1084) => {
                h1084
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1097 = v2;
        let a1098 = {
            let a1095 = {
                let a1085 = 1u64;
                let a1086 = 1u64;
                nat_add(a1085, a1086)?
            };
            let a1096 = {
                let a1093 = {
                    let a1087 = 1u64;
                    let a1088 = 1u64;
                    nat_add(a1087, a1088)?
                };
                let a1094 = {
                    let a1091 = {
                        let a1089 = 1u64;
                        let a1090 = 1u64;
                        nat_add(a1089, a1090)?
                    };
                    let a1092 = 1u64;
                    nat_mul(a1091, a1092)?
                };
                nat_mul(a1093, a1094)?
            };
            nat_mul(a1095, a1096)?
        };
        let a1099 = 0u64;
        nat_rem(a1097, a1098, a1099)
    };
    let v4: u64 = {
        let a1112 = v2;
        let a1113 = {
            let a1110 = {
                let a1100 = 1u64;
                let a1101 = 1u64;
                nat_add(a1100, a1101)?
            };
            let a1111 = {
                let a1108 = {
                    let a1102 = 1u64;
                    let a1103 = 1u64;
                    nat_add(a1102, a1103)?
                };
                let a1109 = {
                    let a1106 = {
                        let a1104 = 1u64;
                        let a1105 = 1u64;
                        nat_add(a1104, a1105)?
                    };
                    let a1107 = 1u64;
                    nat_mul(a1106, a1107)?
                };
                nat_mul(a1108, a1109)?
            };
            nat_mul(a1110, a1111)?
        };
        let a1114 = 0u64;
        nat_quot(a1112, a1113, a1114)
    };
    let v5: bool = {
        let a1123 = {
            let a1121 = {
                let a1115 = 1u64;
                let a1116 = 1u64;
                nat_add(a1115, a1116)?
            };
            let a1122 = {
                let a1119 = {
                    let a1117 = 1u64;
                    let a1118 = 1u64;
                    nat_add(a1117, a1118)?
                };
                let a1120 = 1u64;
                nat_mul(a1119, a1120)?
            };
            nat_mul(a1121, a1122)?
        };
        let a1124 = v3;
        if nat_lt(a1123, a1124) {
            true
        } else {
            let a1133 = v3;
            let a1134 = {
                let a1131 = {
                    let a1125 = 1u64;
                    let a1126 = 1u64;
                    nat_add(a1125, a1126)?
                };
                let a1132 = {
                    let a1129 = {
                        let a1127 = 1u64;
                        let a1128 = 1u64;
                        nat_add(a1127, a1128)?
                    };
                    let a1130 = 1u64;
                    nat_mul(a1129, a1130)?
                };
                nat_mul(a1131, a1132)?
            };
            if nat_eq(a1133, a1134) {
                let a1142 = {
                    let a1139 = v4;
                    let a1140 = {
                        let a1137 = {
                            let a1135 = 1u64;
                            let a1136 = 1u64;
                            nat_add(a1135, a1136)?
                        };
                        let a1138 = 1u64;
                        nat_mul(a1137, a1138)?
                    };
                    let a1141 = 0u64;
                    nat_rem(a1139, a1140, a1141)
                };
                let a1143 = 1u64;
                nat_eq(a1142, a1143)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1144 = v4;
        let a1145 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1144, a1145)?
    };
    let v7: bool = {
        let a1174 = {
            let a1172 = {
                let a1148 = {
                    let a1146 = 1u64;
                    let a1147 = 1u64;
                    nat_add(a1146, a1147)?
                };
                let a1149 = 1u64;
                nat_mul(a1148, a1149)?
            };
            let a1173 = f12({
                let a1170 = {
                    let a1168 = {
                        let a1150 = 1u64;
                        let a1151 = 1u64;
                        nat_add(a1150, a1151)?
                    };
                    let a1169 = {
                        let a1166 = {
                            let a1164 = {
                                let a1152 = 1u64;
                                let a1153 = 1u64;
                                nat_add(a1152, a1153)?
                            };
                            let a1165 = {
                                let a1162 = {
                                    let a1160 = {
                                        let a1154 = 1u64;
                                        let a1155 = 1u64;
                                        nat_add(a1154, a1155)?
                                    };
                                    let a1161 = {
                                        let a1158 = {
                                            let a1156 = 1u64;
                                            let a1157 = 1u64;
                                            nat_add(a1156, a1157)?
                                        };
                                        let a1159 = 1u64;
                                        nat_mul(a1158, a1159)?
                                    };
                                    nat_mul(a1160, a1161)?
                                };
                                let a1163 = 1u64;
                                nat_add(a1162, a1163)?
                            };
                            nat_mul(a1164, a1165)?
                        };
                        let a1167 = 1u64;
                        nat_add(a1166, a1167)?
                    };
                    nat_mul(a1168, a1169)?
                };
                let a1171 = 1u64;
                nat_add(a1170, a1171)?
            })?;
            nat_mul(a1172, a1173)?
        };
        let a1175 = v6;
        nat_le(a1174, a1175)
    };
    let v8: u64 = if v7 {
        let a1180 = v6;
        let a1181 = {
            let a1178 = {
                let a1176 = 1u64;
                let a1177 = 1u64;
                nat_add(a1176, a1177)?
            };
            let a1179 = 1u64;
            nat_mul(a1178, a1179)?
        };
        let a1182 = 0u64;
        nat_quot(a1180, a1181, a1182)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1183 = v1;
        let a1184 = 1u64;
        nat_add(a1183, a1184)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1223 = v9;
        let a1224 = {
            let a1221 = {
                let a1219 = {
                    let a1185 = 1u64;
                    let a1186 = 1u64;
                    nat_add(a1185, a1186)?
                };
                let a1220 = {
                    let a1217 = {
                        let a1187 = 1u64;
                        let a1188 = 1u64;
                        nat_add(a1187, a1188)?
                    };
                    let a1218 = {
                        let a1215 = {
                            let a1189 = 1u64;
                            let a1190 = 1u64;
                            nat_add(a1189, a1190)?
                        };
                        let a1216 = {
                            let a1213 = {
                                let a1191 = 1u64;
                                let a1192 = 1u64;
                                nat_add(a1191, a1192)?
                            };
                            let a1214 = {
                                let a1211 = {
                                    let a1193 = 1u64;
                                    let a1194 = 1u64;
                                    nat_add(a1193, a1194)?
                                };
                                let a1212 = {
                                    let a1209 = {
                                        let a1195 = 1u64;
                                        let a1196 = 1u64;
                                        nat_add(a1195, a1196)?
                                    };
                                    let a1210 = {
                                        let a1207 = {
                                            let a1197 = 1u64;
                                            let a1198 = 1u64;
                                            nat_add(a1197, a1198)?
                                        };
                                        let a1208 = {
                                            let a1205 = {
                                                let a1199 = 1u64;
                                                let a1200 = 1u64;
                                                nat_add(a1199, a1200)?
                                            };
                                            let a1206 = {
                                                let a1203 = {
                                                    let a1201 = 1u64;
                                                    let a1202 = 1u64;
                                                    nat_add(a1201, a1202)?
                                                };
                                                let a1204 = 1u64;
                                                nat_mul(a1203, a1204)?
                                            };
                                            nat_mul(a1205, a1206)?
                                        };
                                        nat_mul(a1207, a1208)?
                                    };
                                    nat_mul(a1209, a1210)?
                                };
                                nat_mul(a1211, a1212)?
                            };
                            nat_mul(a1213, a1214)?
                        };
                        nat_mul(a1215, a1216)?
                    };
                    nat_mul(a1217, a1218)?
                };
                nat_mul(a1219, a1220)?
            };
            let a1222 = 1u64;
            nat_add(a1221, a1222)?
        };
        if if nat_eq(a1223, a1224) {
            let a1247 = v8;
            let a1248 = f12({
                let a1245 = {
                    let a1243 = {
                        let a1225 = 1u64;
                        let a1226 = 1u64;
                        nat_add(a1225, a1226)?
                    };
                    let a1244 = {
                        let a1241 = {
                            let a1239 = {
                                let a1227 = 1u64;
                                let a1228 = 1u64;
                                nat_add(a1227, a1228)?
                            };
                            let a1240 = {
                                let a1237 = {
                                    let a1235 = {
                                        let a1229 = 1u64;
                                        let a1230 = 1u64;
                                        nat_add(a1229, a1230)?
                                    };
                                    let a1236 = {
                                        let a1233 = {
                                            let a1231 = 1u64;
                                            let a1232 = 1u64;
                                            nat_add(a1231, a1232)?
                                        };
                                        let a1234 = 1u64;
                                        nat_mul(a1233, a1234)?
                                    };
                                    nat_mul(a1235, a1236)?
                                };
                                let a1238 = 1u64;
                                nat_add(a1237, a1238)?
                            };
                            nat_mul(a1239, a1240)?
                        };
                        let a1242 = 1u64;
                        nat_add(a1241, a1242)?
                    };
                    nat_mul(a1243, a1244)?
                };
                let a1246 = 1u64;
                nat_add(a1245, a1246)?
            })?;
            nat_lt(a1247, a1248)
        } else {
            false
        } {
            0u64
        } else {
            let a1285 = v9;
            let a1286 = {
                let a1283 = {
                    let a1249 = 1u64;
                    let a1250 = 1u64;
                    nat_add(a1249, a1250)?
                };
                let a1284 = {
                    let a1281 = {
                        let a1251 = 1u64;
                        let a1252 = 1u64;
                        nat_add(a1251, a1252)?
                    };
                    let a1282 = {
                        let a1279 = {
                            let a1253 = 1u64;
                            let a1254 = 1u64;
                            nat_add(a1253, a1254)?
                        };
                        let a1280 = {
                            let a1277 = {
                                let a1255 = 1u64;
                                let a1256 = 1u64;
                                nat_add(a1255, a1256)?
                            };
                            let a1278 = {
                                let a1275 = {
                                    let a1257 = 1u64;
                                    let a1258 = 1u64;
                                    nat_add(a1257, a1258)?
                                };
                                let a1276 = {
                                    let a1273 = {
                                        let a1259 = 1u64;
                                        let a1260 = 1u64;
                                        nat_add(a1259, a1260)?
                                    };
                                    let a1274 = {
                                        let a1271 = {
                                            let a1261 = 1u64;
                                            let a1262 = 1u64;
                                            nat_add(a1261, a1262)?
                                        };
                                        let a1272 = {
                                            let a1269 = {
                                                let a1263 = 1u64;
                                                let a1264 = 1u64;
                                                nat_add(a1263, a1264)?
                                            };
                                            let a1270 = {
                                                let a1267 = {
                                                    let a1265 = 1u64;
                                                    let a1266 = 1u64;
                                                    nat_add(a1265, a1266)?
                                                };
                                                let a1268 = 1u64;
                                                nat_mul(a1267, a1268)?
                                            };
                                            nat_mul(a1269, a1270)?
                                        };
                                        nat_mul(a1271, a1272)?
                                    };
                                    nat_mul(a1273, a1274)?
                                };
                                nat_mul(a1275, a1276)?
                            };
                            nat_mul(a1277, a1278)?
                        };
                        nat_mul(a1279, a1280)?
                    };
                    nat_mul(a1281, a1282)?
                };
                nat_mul(a1283, a1284)?
            };
            nat_sub(a1285, a1286)
        }
    };
    let a1287 = v8;
    let a1288 = 0u64;
    if nat_eq(a1287, a1288) {
        f18(v0)
    } else {
        let a1341 = {
            let a1339 = {
                let a1337 = {
                    let a1289 = 1u64;
                    let a1290 = 1u64;
                    nat_add(a1289, a1290)?
                };
                let a1338 = {
                    let a1335 = {
                        let a1333 = {
                            let a1291 = 1u64;
                            let a1292 = 1u64;
                            nat_add(a1291, a1292)?
                        };
                        let a1334 = {
                            let a1331 = {
                                let a1329 = {
                                    let a1293 = 1u64;
                                    let a1294 = 1u64;
                                    nat_add(a1293, a1294)?
                                };
                                let a1330 = {
                                    let a1327 = {
                                        let a1325 = {
                                            let a1295 = 1u64;
                                            let a1296 = 1u64;
                                            nat_add(a1295, a1296)?
                                        };
                                        let a1326 = {
                                            let a1323 = {
                                                let a1321 = {
                                                    let a1297 = 1u64;
                                                    let a1298 = 1u64;
                                                    nat_add(a1297, a1298)?
                                                };
                                                let a1322 = {
                                                    let a1319 = {
                                                        let a1317 = {
                                                            let a1299 = 1u64;
                                                            let a1300 = 1u64;
                                                            nat_add(a1299, a1300)?
                                                        };
                                                        let a1318 = {
                                                            let a1315 = {
                                                                let a1313 = {
                                                                    let a1301 = 1u64;
                                                                    let a1302 = 1u64;
                                                                    nat_add(a1301, a1302)?
                                                                };
                                                                let a1314 = {
                                                                    let a1311 = {
                                                                        let a1309 = {
                                                                            let a1303 = 1u64;
                                                                            let a1304 = 1u64;
                                                                            nat_add(a1303, a1304)?
                                                                        };
                                                                        let a1310 = {
                                                                            let a1307 = {
                                                                                let a1305 = 1u64;
                                                                                let a1306 = 1u64;
                                                                                nat_add(a1305, a1306)?
                                                                            };
                                                                            let a1308 = 1u64;
                                                                            nat_mul(a1307, a1308)?
                                                                        };
                                                                        nat_mul(a1309, a1310)?
                                                                    };
                                                                    let a1312 = 1u64;
                                                                    nat_add(a1311, a1312)?
                                                                };
                                                                nat_mul(a1313, a1314)?
                                                            };
                                                            let a1316 = 1u64;
                                                            nat_add(a1315, a1316)?
                                                        };
                                                        nat_mul(a1317, a1318)?
                                                    };
                                                    let a1320 = 1u64;
                                                    nat_add(a1319, a1320)?
                                                };
                                                nat_mul(a1321, a1322)?
                                            };
                                            let a1324 = 1u64;
                                            nat_add(a1323, a1324)?
                                        };
                                        nat_mul(a1325, a1326)?
                                    };
                                    let a1328 = 1u64;
                                    nat_add(a1327, a1328)?
                                };
                                nat_mul(a1329, a1330)?
                            };
                            let a1332 = 1u64;
                            nat_add(a1331, a1332)?
                        };
                        nat_mul(a1333, a1334)?
                    };
                    let a1336 = 1u64;
                    nat_add(a1335, a1336)?
                };
                nat_mul(a1337, a1338)?
            };
            let a1340 = 1u64;
            nat_add(a1339, a1340)?
        };
        let a1342 = v9;
        if nat_le(a1341, a1342) {
            f16(v0)
        } else {
            let a1394 = {
                let a1367 = f18(v0)?;
                let a1368 = {
                    let a1365 = v10;
                    let a1366 = f12({
                        let a1363 = {
                            let a1361 = {
                                let a1343 = 1u64;
                                let a1344 = 1u64;
                                nat_add(a1343, a1344)?
                            };
                            let a1362 = {
                                let a1359 = {
                                    let a1357 = {
                                        let a1345 = 1u64;
                                        let a1346 = 1u64;
                                        nat_add(a1345, a1346)?
                                    };
                                    let a1358 = {
                                        let a1355 = {
                                            let a1353 = {
                                                let a1347 = 1u64;
                                                let a1348 = 1u64;
                                                nat_add(a1347, a1348)?
                                            };
                                            let a1354 = {
                                                let a1351 = {
                                                    let a1349 = 1u64;
                                                    let a1350 = 1u64;
                                                    nat_add(a1349, a1350)?
                                                };
                                                let a1352 = 1u64;
                                                nat_mul(a1351, a1352)?
                                            };
                                            nat_mul(a1353, a1354)?
                                        };
                                        let a1356 = 1u64;
                                        nat_add(a1355, a1356)?
                                    };
                                    nat_mul(a1357, a1358)?
                                };
                                let a1360 = 1u64;
                                nat_add(a1359, a1360)?
                            };
                            nat_mul(a1361, a1362)?
                        };
                        let a1364 = 1u64;
                        nat_add(a1363, a1364)?
                    })?;
                    nat_mul(a1365, a1366)?
                };
                nat_add(a1367, a1368)?
            };
            let a1395 = {
                let a1391 = v8;
                let a1392 = f12({
                    let a1389 = {
                        let a1387 = {
                            let a1369 = 1u64;
                            let a1370 = 1u64;
                            nat_add(a1369, a1370)?
                        };
                        let a1388 = {
                            let a1385 = {
                                let a1383 = {
                                    let a1371 = 1u64;
                                    let a1372 = 1u64;
                                    nat_add(a1371, a1372)?
                                };
                                let a1384 = {
                                    let a1381 = {
                                        let a1379 = {
                                            let a1373 = 1u64;
                                            let a1374 = 1u64;
                                            nat_add(a1373, a1374)?
                                        };
                                        let a1380 = {
                                            let a1377 = {
                                                let a1375 = 1u64;
                                                let a1376 = 1u64;
                                                nat_add(a1375, a1376)?
                                            };
                                            let a1378 = 1u64;
                                            nat_mul(a1377, a1378)?
                                        };
                                        nat_mul(a1379, a1380)?
                                    };
                                    let a1382 = 1u64;
                                    nat_add(a1381, a1382)?
                                };
                                nat_mul(a1383, a1384)?
                            };
                            let a1386 = 1u64;
                            nat_add(a1385, a1386)?
                        };
                        nat_mul(a1387, a1388)?
                    };
                    let a1390 = 1u64;
                    nat_add(a1389, a1390)?
                })?;
                let a1393 = 0u64;
                nat_rem(a1391, a1392, a1393)
            };
            nat_add(a1394, a1395)
        }
    }
}

pub fn run(p0: u64, p1: u64) -> R<Adt0> {
    f0(p0, p1)
}
