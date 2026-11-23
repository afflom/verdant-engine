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
    C0(bool, u64, u64),
    C1(bool, u64, u64),
    C2(bool, u64, u64, u64),
    C3(bool, u64, u64, u64),
    C4(bool, u64, u64),
    C5(u64, u64, u64),
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64, u64, u64, u64),
    C2(u64),
    C3(u64, u64),
    C4(u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F28,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F28 => f28(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F33,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F33 => f33(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3) => {
            let v4: u64 = if v1 {
                2u64
            } else {
                4u64
            };
            let a5 = {
                let a2 = v2;
                let a3 = 18u64;
                let a4 = 0u64;
                nat_rem(a2, a3, a4)
            };
            let a6 = 0u64;
            if if nat_eq(a5, a6) {
                let a12 = {
                    let a10 = {
                        let a7 = v2;
                        let a8 = 18u64;
                        let a9 = 0u64;
                        nat_quot(a7, a8, a9)
                    };
                    let a11 = v4;
                    nat_mul(a10, a11)?
                };
                let a13 = v3;
                nat_eq(a12, a13)
            } else {
                false
            } {
                Ok(Adt1::C1(18u64, v4, if v1 {
                    8u64
                } else {
                    4u64
                }, if v1 {
                    4u64
                } else {
                    8u64
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C1(v5, v6, v7) => {
            let a14 = v6;
            let a15 = 256u64;
            if if nat_lt(a14, a15) {
                let a16 = v7;
                let a17 = if v5 {
                    4u64
                } else {
                    8u64
                };
                nat_lt(a16, a17)
            } else {
                false
            } {
                let m20 = f1(v6, 1u64, f2(if v5 {
                    let a18 = v7;
                    let a19 = 2u64;
                    nat_mul(a18, a19)?
                } else {
                    v7
                }), if v5 {
                    4u64
                } else {
                    2u64
                })?;
                match m20 {
                    None => {
                        Ok(Adt1::C0)
                    }
                    Some(v8) => {
                        Ok(Adt1::C2(if v5 {
                            v8
                        } else {
                            let a21 = v8;
                            let a22 = 2u64;
                            nat_mul(a21, a22)?
                        }))
                    }
                }
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C2(v9, v10, v11, v12) => {
            let v13: u64 = if v9 {
                18u64
            } else {
                22u64
            };
            let a23 = v10;
            let a24 = 32u64;
            if if if nat_eq(a23, a24) {
                let a28 = {
                    let a25 = v11;
                    let a26 = v13;
                    let a27 = 0u64;
                    nat_rem(a25, a26, a27)
                };
                let a29 = 0u64;
                nat_eq(a28, a29)
            } else {
                false
            } {
                let a33 = {
                    let a30 = v11;
                    let a31 = v13;
                    let a32 = 0u64;
                    nat_quot(a30, a31, a32)
                };
                let a34 = v12;
                nat_eq(a33, a34)
            } else {
                false
            } {
                Ok(Adt1::C2(v13))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C3(v14, v15, v16, v17) => {
            f3(v14, v15, v16, v17)
        }
        Adt0::C4(v18, v19, v20) => {
            let v21: u64 = if v18 {
                24u64
            } else {
                20u64
            };
            let a38 = {
                let a35 = v19;
                let a36 = v21;
                let a37 = 0u64;
                nat_rem(a35, a36, a37)
            };
            let a39 = 0u64;
            if if nat_eq(a38, a39) {
                let a43 = {
                    let a40 = v19;
                    let a41 = v21;
                    let a42 = 0u64;
                    nat_quot(a40, a41, a42)
                };
                let a44 = v20;
                nat_eq(a43, a44)
            } else {
                false
            } {
                Ok(Adt1::C2(v21))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C5(v22, v23, v24) => {
            f4(v22, v23, v24)
        }
        Adt0::C6(v25, v26, v27, v28, v29, v30) => {
            let a45 = v25;
            let a46 = 4294967296u64;
            if if if if if if nat_lt(a45, a46) {
                let a47 = v26;
                let a48 = 4294967296u64;
                nat_lt(a47, a48)
            } else {
                false
            } {
                let a49 = v27;
                let a50 = 65536u64;
                nat_lt(a49, a50)
            } else {
                false
            } {
                let a51 = v28;
                let a52 = 65536u64;
                nat_lt(a51, a52)
            } else {
                false
            } {
                let a53 = v29;
                let a54 = 65536u64;
                nat_lt(a53, a54)
            } else {
                false
            } {
                let a55 = v30;
                let a56 = 65536u64;
                nat_lt(a55, a56)
            } else {
                false
            } {
                let v31: u64 = f5(f6(v27)?, f6(v28)?)?;
                let v32: u64 = f5(f6(v29)?, f6(v30)?)?;
                Ok(Adt1::C2(f7(v25, f7(f5(v31, v26)?, v32)?)?))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C7(v33, v34) => {
            let a57 = v33;
            let a58 = 4294967296u64;
            if if nat_lt(a57, a58) {
                let a59 = v34;
                let a60 = 2139095040u64;
                nat_lt(a59, a60)
            } else {
                false
            } {
                Ok(Adt1::C2(f8(f5(v33, v34)?)?))
            } else {
                Ok(Adt1::C0)
            }
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a61 = 0u64;
    let a62 = v2;
    if if nat_lt(a61, a62) {
        let a63 = 0u64;
        let a64 = v3;
        nat_lt(a63, a64)
    } else {
        false
    } {
        Ok(Some({
            let a70 = {
                let a67 = {
                    let a65 = v0;
                    let a66 = v1;
                    nat_mul(a65, a66)?
                };
                let a68 = v2;
                let a69 = 0u64;
                nat_quot(a67, a68, a69)
            };
            let a71 = v3;
            let a72 = 0u64;
            nat_rem(a70, a71, a72)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a73 = v0;
    let a74 = 0u64;
    if nat_eq(a73, a74) {
        1u64
    } else {
        let a75 = v0;
        let a76 = 1u64;
        if nat_eq(a75, a76) {
            2u64
        } else {
            let a77 = v0;
            let a78 = 2u64;
            if nat_eq(a77, a78) {
                4u64
            } else {
                let a79 = v0;
                let a80 = 3u64;
                if nat_eq(a79, a80) {
                    8u64
                } else {
                    let a81 = v0;
                    let a82 = 4u64;
                    if nat_eq(a81, a82) {
                        16u64
                    } else {
                        let a83 = v0;
                        let a84 = 5u64;
                        if nat_eq(a83, a84) {
                            32u64
                        } else {
                            let a85 = v0;
                            let a86 = 6u64;
                            if nat_eq(a85, a86) {
                                64u64
                            } else {
                                128u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f3(v0: bool, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let a87 = v1;
    let a88 = 16u64;
    if if if nat_lt(a87, a88) {
        let a89 = v2;
        let a90 = 256u64;
        nat_lt(a89, a90)
    } else {
        false
    } {
        let a91 = v3;
        let a92 = 4294967296u64;
        nat_lt(a91, a92)
    } else {
        false
    } {
        let v4: u64 = {
            let a93 = v2;
            let a94 = 16u64;
            let a95 = 0u64;
            nat_rem(a93, a94, a95)
        };
        let v5: u64 = {
            let a96 = v2;
            let a97 = 16u64;
            let a98 = 0u64;
            nat_quot(a96, a97, a98)
        };
        if v0 {
            Ok(Adt1::C4({
                let a99 = 18568u64;
                let a100 = v4;
                nat_add(a99, a100)?
            }, {
                let a101 = 18568u64;
                let a102 = v5;
                nat_add(a101, a102)?
            }))
        } else {
            let v6: u64 = {
                let a108 = f2({
                    let a103 = v1;
                    let a104 = 8u64;
                    let a105 = 0u64;
                    nat_rem(a103, a104, a105)
                });
                let a109 = {
                    let a106 = v1;
                    let a107 = 8u64;
                    if nat_lt(a106, a107) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a108, a109)?
            };
            Ok(Adt1::C3(f9(v4, v3, v6)?, f9(v5, v3, {
                let a110 = v6;
                let a111 = 65536u64;
                nat_mul(a110, a111)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a112 = v0;
    let a113 = 16u64;
    if if if nat_lt(a112, a113) {
        let a114 = v1;
        let a115 = 256u64;
        nat_lt(a114, a115)
    } else {
        false
    } {
        let a116 = v2;
        let a117 = 4294967296u64;
        nat_lt(a116, a117)
    } else {
        false
    } {
        let v3: u64 = {
            let a123 = f2({
                let a118 = v0;
                let a119 = 8u64;
                let a120 = 0u64;
                nat_rem(a118, a119, a120)
            });
            let a124 = {
                let a121 = v0;
                let a122 = 8u64;
                if nat_lt(a121, a122) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a123, a124)?
        };
        Ok(Adt1::C3(f10({
            let a125 = v1;
            let a126 = 16u64;
            let a127 = 0u64;
            nat_rem(a125, a126, a127)
        }, v2, v3)?, f10({
            let a128 = v1;
            let a129 = 16u64;
            let a130 = 0u64;
            nat_quot(a128, a129, a130)
        }, v2, {
            let a131 = v3;
            let a132 = 65536u64;
            nat_mul(a131, a132)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f11(v0)? {
        true
    } else {
        f11(v1)?
    } {
        true
    } else {
        if f12(v0)? {
            f13(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f12(v1)? {
            f13(v0)?
        } else {
            false
        }
    } {
        let a219 = {
            let a197 = {
                let a173 = {
                    let a171 = {
                        let a133 = 1u64;
                        let a134 = 1u64;
                        nat_add(a133, a134)?
                    };
                    let a172 = {
                        let a169 = {
                            let a167 = {
                                let a135 = 1u64;
                                let a136 = 1u64;
                                nat_add(a135, a136)?
                            };
                            let a168 = {
                                let a165 = {
                                    let a163 = {
                                        let a137 = 1u64;
                                        let a138 = 1u64;
                                        nat_add(a137, a138)?
                                    };
                                    let a164 = {
                                        let a161 = {
                                            let a159 = {
                                                let a139 = 1u64;
                                                let a140 = 1u64;
                                                nat_add(a139, a140)?
                                            };
                                            let a160 = {
                                                let a157 = {
                                                    let a155 = {
                                                        let a141 = 1u64;
                                                        let a142 = 1u64;
                                                        nat_add(a141, a142)?
                                                    };
                                                    let a156 = {
                                                        let a153 = {
                                                            let a151 = {
                                                                let a143 = 1u64;
                                                                let a144 = 1u64;
                                                                nat_add(a143, a144)?
                                                            };
                                                            let a152 = {
                                                                let a149 = {
                                                                    let a147 = {
                                                                        let a145 = 1u64;
                                                                        let a146 = 1u64;
                                                                        nat_add(a145, a146)?
                                                                    };
                                                                    let a148 = 1u64;
                                                                    nat_mul(a147, a148)?
                                                                };
                                                                let a150 = 1u64;
                                                                nat_add(a149, a150)?
                                                            };
                                                            nat_mul(a151, a152)?
                                                        };
                                                        let a154 = 1u64;
                                                        nat_add(a153, a154)?
                                                    };
                                                    nat_mul(a155, a156)?
                                                };
                                                let a158 = 1u64;
                                                nat_add(a157, a158)?
                                            };
                                            nat_mul(a159, a160)?
                                        };
                                        let a162 = 1u64;
                                        nat_add(a161, a162)?
                                    };
                                    nat_mul(a163, a164)?
                                };
                                let a166 = 1u64;
                                nat_add(a165, a166)?
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
            let a198 = f14({
                let a195 = {
                    let a193 = {
                        let a175 = 1u64;
                        let a176 = 1u64;
                        nat_add(a175, a176)?
                    };
                    let a194 = {
                        let a191 = {
                            let a189 = {
                                let a177 = 1u64;
                                let a178 = 1u64;
                                nat_add(a177, a178)?
                            };
                            let a190 = {
                                let a187 = {
                                    let a185 = {
                                        let a179 = 1u64;
                                        let a180 = 1u64;
                                        nat_add(a179, a180)?
                                    };
                                    let a186 = {
                                        let a183 = {
                                            let a181 = 1u64;
                                            let a182 = 1u64;
                                            nat_add(a181, a182)?
                                        };
                                        let a184 = 1u64;
                                        nat_mul(a183, a184)?
                                    };
                                    nat_mul(a185, a186)?
                                };
                                let a188 = 1u64;
                                nat_add(a187, a188)?
                            };
                            nat_mul(a189, a190)?
                        };
                        let a192 = 1u64;
                        nat_add(a191, a192)?
                    };
                    nat_mul(a193, a194)?
                };
                let a196 = 1u64;
                nat_add(a195, a196)?
            })?;
            nat_mul(a197, a198)?
        };
        let a220 = f14({
            let a217 = {
                let a199 = 1u64;
                let a200 = 1u64;
                nat_add(a199, a200)?
            };
            let a218 = {
                let a215 = {
                    let a213 = {
                        let a201 = 1u64;
                        let a202 = 1u64;
                        nat_add(a201, a202)?
                    };
                    let a214 = {
                        let a211 = {
                            let a209 = {
                                let a203 = 1u64;
                                let a204 = 1u64;
                                nat_add(a203, a204)?
                            };
                            let a210 = {
                                let a207 = {
                                    let a205 = 1u64;
                                    let a206 = 1u64;
                                    nat_add(a205, a206)?
                                };
                                let a208 = 1u64;
                                nat_mul(a207, a208)?
                            };
                            nat_mul(a209, a210)?
                        };
                        let a212 = 1u64;
                        nat_add(a211, a212)?
                    };
                    nat_mul(a213, a214)?
                };
                let a216 = 1u64;
                nat_add(a215, a216)?
            };
            nat_mul(a217, a218)?
        })?;
        nat_add(a219, a220)
    } else {
        if if f12(v0)? {
            true
        } else {
            f12(v1)?
        } {
            f15(f16(v0, v1)?)
        } else {
            if if f13(v0)? {
                true
            } else {
                f13(v1)?
            } {
                f17(f16(v0, v1)?)
            } else {
                f18(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a224 = {
            let a221 = v0;
            let a222 = 32768u64;
            let a223 = 0u64;
            nat_quot(a221, a222, a223)
        };
        let a225 = 2147483648u64;
        nat_mul(a224, a225)?
    };
    let v2: u64 = {
        let a229 = {
            let a226 = v0;
            let a227 = 32768u64;
            let a228 = 0u64;
            nat_rem(a226, a227, a228)
        };
        let a230 = 1024u64;
        let a231 = 0u64;
        nat_quot(a229, a230, a231)
    };
    let v3: u64 = {
        let a232 = v0;
        let a233 = 1024u64;
        let a234 = 0u64;
        nat_rem(a232, a233, a234)
    };
    let a269 = v1;
    let a270 = {
        let a235 = v2;
        let a236 = 31u64;
        if nat_eq(a235, a236) {
            let a245 = 2139095040u64;
            let a246 = {
                let a243 = {
                    let a237 = 0u64;
                    let a238 = v3;
                    if if nat_lt(a237, a238) {
                        let a239 = v3;
                        let a240 = 512u64;
                        nat_lt(a239, a240)
                    } else {
                        false
                    } {
                        let a241 = v3;
                        let a242 = 512u64;
                        nat_add(a241, a242)?
                    } else {
                        v3
                    }
                };
                let a244 = 8192u64;
                nat_mul(a243, a244)?
            };
            nat_add(a245, a246)?
        } else {
            let a247 = v2;
            let a248 = 0u64;
            if nat_eq(a247, a248) {
                let a249 = v3;
                let a250 = 0u64;
                if nat_eq(a249, a250) {
                    0u64
                } else {
                    let v4: Adt2 = f19(113u64, v3, 0u64, 1024u64)?;
                    let a259 = {
                        let a252 = match v4.clone() {
                            Adt2::C0(h251, _, _, _) => {
                                h251
                            }
                        };
                        let a253 = 8388608u64;
                        nat_mul(a252, a253)?
                    };
                    let a260 = {
                        let a257 = {
                            let a255 = match v4 {
                                Adt2::C0(_, h254, _, _) => {
                                    h254
                                }
                            };
                            let a256 = 1024u64;
                            nat_sub(a255, a256)
                        };
                        let a258 = 8192u64;
                        nat_mul(a257, a258)?
                    };
                    nat_add(a259, a260)?
                }
            } else {
                let a267 = {
                    let a263 = {
                        let a261 = v2;
                        let a262 = 112u64;
                        nat_add(a261, a262)?
                    };
                    let a264 = 8388608u64;
                    nat_mul(a263, a264)?
                };
                let a268 = {
                    let a265 = v3;
                    let a266 = 8192u64;
                    nat_mul(a265, a266)?
                };
                nat_add(a267, a268)?
            }
        }
    };
    nat_add(a269, a270)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f11(v0)? {
        true
    } else {
        f11(v1)?
    } {
        let a357 = {
            let a335 = {
                let a311 = {
                    let a309 = {
                        let a271 = 1u64;
                        let a272 = 1u64;
                        nat_add(a271, a272)?
                    };
                    let a310 = {
                        let a307 = {
                            let a305 = {
                                let a273 = 1u64;
                                let a274 = 1u64;
                                nat_add(a273, a274)?
                            };
                            let a306 = {
                                let a303 = {
                                    let a301 = {
                                        let a275 = 1u64;
                                        let a276 = 1u64;
                                        nat_add(a275, a276)?
                                    };
                                    let a302 = {
                                        let a299 = {
                                            let a297 = {
                                                let a277 = 1u64;
                                                let a278 = 1u64;
                                                nat_add(a277, a278)?
                                            };
                                            let a298 = {
                                                let a295 = {
                                                    let a293 = {
                                                        let a279 = 1u64;
                                                        let a280 = 1u64;
                                                        nat_add(a279, a280)?
                                                    };
                                                    let a294 = {
                                                        let a291 = {
                                                            let a289 = {
                                                                let a281 = 1u64;
                                                                let a282 = 1u64;
                                                                nat_add(a281, a282)?
                                                            };
                                                            let a290 = {
                                                                let a287 = {
                                                                    let a285 = {
                                                                        let a283 = 1u64;
                                                                        let a284 = 1u64;
                                                                        nat_add(a283, a284)?
                                                                    };
                                                                    let a286 = 1u64;
                                                                    nat_mul(a285, a286)?
                                                                };
                                                                let a288 = 1u64;
                                                                nat_add(a287, a288)?
                                                            };
                                                            nat_mul(a289, a290)?
                                                        };
                                                        let a292 = 1u64;
                                                        nat_add(a291, a292)?
                                                    };
                                                    nat_mul(a293, a294)?
                                                };
                                                let a296 = 1u64;
                                                nat_add(a295, a296)?
                                            };
                                            nat_mul(a297, a298)?
                                        };
                                        let a300 = 1u64;
                                        nat_add(a299, a300)?
                                    };
                                    nat_mul(a301, a302)?
                                };
                                let a304 = 1u64;
                                nat_add(a303, a304)?
                            };
                            nat_mul(a305, a306)?
                        };
                        let a308 = 1u64;
                        nat_add(a307, a308)?
                    };
                    nat_mul(a309, a310)?
                };
                let a312 = 1u64;
                nat_add(a311, a312)?
            };
            let a336 = f14({
                let a333 = {
                    let a331 = {
                        let a313 = 1u64;
                        let a314 = 1u64;
                        nat_add(a313, a314)?
                    };
                    let a332 = {
                        let a329 = {
                            let a327 = {
                                let a315 = 1u64;
                                let a316 = 1u64;
                                nat_add(a315, a316)?
                            };
                            let a328 = {
                                let a325 = {
                                    let a323 = {
                                        let a317 = 1u64;
                                        let a318 = 1u64;
                                        nat_add(a317, a318)?
                                    };
                                    let a324 = {
                                        let a321 = {
                                            let a319 = 1u64;
                                            let a320 = 1u64;
                                            nat_add(a319, a320)?
                                        };
                                        let a322 = 1u64;
                                        nat_mul(a321, a322)?
                                    };
                                    nat_mul(a323, a324)?
                                };
                                let a326 = 1u64;
                                nat_add(a325, a326)?
                            };
                            nat_mul(a327, a328)?
                        };
                        let a330 = 1u64;
                        nat_add(a329, a330)?
                    };
                    nat_mul(a331, a332)?
                };
                let a334 = 1u64;
                nat_add(a333, a334)?
            })?;
            nat_mul(a335, a336)?
        };
        let a358 = f14({
            let a355 = {
                let a337 = 1u64;
                let a338 = 1u64;
                nat_add(a337, a338)?
            };
            let a356 = {
                let a353 = {
                    let a351 = {
                        let a339 = 1u64;
                        let a340 = 1u64;
                        nat_add(a339, a340)?
                    };
                    let a352 = {
                        let a349 = {
                            let a347 = {
                                let a341 = 1u64;
                                let a342 = 1u64;
                                nat_add(a341, a342)?
                            };
                            let a348 = {
                                let a345 = {
                                    let a343 = 1u64;
                                    let a344 = 1u64;
                                    nat_add(a343, a344)?
                                };
                                let a346 = 1u64;
                                nat_mul(a345, a346)?
                            };
                            nat_mul(a347, a348)?
                        };
                        let a350 = 1u64;
                        nat_add(a349, a350)?
                    };
                    nat_mul(a351, a352)?
                };
                let a354 = 1u64;
                nat_add(a353, a354)?
            };
            nat_mul(a355, a356)?
        })?;
        nat_add(a357, a358)
    } else {
        if if f12(v0)? {
            f12(v1)?
        } else {
            false
        } {
            let a359 = f20(v0)?;
            let a360 = f20(v1)?;
            if nat_eq(a359, a360) {
                Ok(v0)
            } else {
                let a447 = {
                    let a425 = {
                        let a401 = {
                            let a399 = {
                                let a361 = 1u64;
                                let a362 = 1u64;
                                nat_add(a361, a362)?
                            };
                            let a400 = {
                                let a397 = {
                                    let a395 = {
                                        let a363 = 1u64;
                                        let a364 = 1u64;
                                        nat_add(a363, a364)?
                                    };
                                    let a396 = {
                                        let a393 = {
                                            let a391 = {
                                                let a365 = 1u64;
                                                let a366 = 1u64;
                                                nat_add(a365, a366)?
                                            };
                                            let a392 = {
                                                let a389 = {
                                                    let a387 = {
                                                        let a367 = 1u64;
                                                        let a368 = 1u64;
                                                        nat_add(a367, a368)?
                                                    };
                                                    let a388 = {
                                                        let a385 = {
                                                            let a383 = {
                                                                let a369 = 1u64;
                                                                let a370 = 1u64;
                                                                nat_add(a369, a370)?
                                                            };
                                                            let a384 = {
                                                                let a381 = {
                                                                    let a379 = {
                                                                        let a371 = 1u64;
                                                                        let a372 = 1u64;
                                                                        nat_add(a371, a372)?
                                                                    };
                                                                    let a380 = {
                                                                        let a377 = {
                                                                            let a375 = {
                                                                                let a373 = 1u64;
                                                                                let a374 = 1u64;
                                                                                nat_add(a373, a374)?
                                                                            };
                                                                            let a376 = 1u64;
                                                                            nat_mul(a375, a376)?
                                                                        };
                                                                        let a378 = 1u64;
                                                                        nat_add(a377, a378)?
                                                                    };
                                                                    nat_mul(a379, a380)?
                                                                };
                                                                let a382 = 1u64;
                                                                nat_add(a381, a382)?
                                                            };
                                                            nat_mul(a383, a384)?
                                                        };
                                                        let a386 = 1u64;
                                                        nat_add(a385, a386)?
                                                    };
                                                    nat_mul(a387, a388)?
                                                };
                                                let a390 = 1u64;
                                                nat_add(a389, a390)?
                                            };
                                            nat_mul(a391, a392)?
                                        };
                                        let a394 = 1u64;
                                        nat_add(a393, a394)?
                                    };
                                    nat_mul(a395, a396)?
                                };
                                let a398 = 1u64;
                                nat_add(a397, a398)?
                            };
                            nat_mul(a399, a400)?
                        };
                        let a402 = 1u64;
                        nat_add(a401, a402)?
                    };
                    let a426 = f14({
                        let a423 = {
                            let a421 = {
                                let a403 = 1u64;
                                let a404 = 1u64;
                                nat_add(a403, a404)?
                            };
                            let a422 = {
                                let a419 = {
                                    let a417 = {
                                        let a405 = 1u64;
                                        let a406 = 1u64;
                                        nat_add(a405, a406)?
                                    };
                                    let a418 = {
                                        let a415 = {
                                            let a413 = {
                                                let a407 = 1u64;
                                                let a408 = 1u64;
                                                nat_add(a407, a408)?
                                            };
                                            let a414 = {
                                                let a411 = {
                                                    let a409 = 1u64;
                                                    let a410 = 1u64;
                                                    nat_add(a409, a410)?
                                                };
                                                let a412 = 1u64;
                                                nat_mul(a411, a412)?
                                            };
                                            nat_mul(a413, a414)?
                                        };
                                        let a416 = 1u64;
                                        nat_add(a415, a416)?
                                    };
                                    nat_mul(a417, a418)?
                                };
                                let a420 = 1u64;
                                nat_add(a419, a420)?
                            };
                            nat_mul(a421, a422)?
                        };
                        let a424 = 1u64;
                        nat_add(a423, a424)?
                    })?;
                    nat_mul(a425, a426)?
                };
                let a448 = f14({
                    let a445 = {
                        let a427 = 1u64;
                        let a428 = 1u64;
                        nat_add(a427, a428)?
                    };
                    let a446 = {
                        let a443 = {
                            let a441 = {
                                let a429 = 1u64;
                                let a430 = 1u64;
                                nat_add(a429, a430)?
                            };
                            let a442 = {
                                let a439 = {
                                    let a437 = {
                                        let a431 = 1u64;
                                        let a432 = 1u64;
                                        nat_add(a431, a432)?
                                    };
                                    let a438 = {
                                        let a435 = {
                                            let a433 = 1u64;
                                            let a434 = 1u64;
                                            nat_add(a433, a434)?
                                        };
                                        let a436 = 1u64;
                                        nat_mul(a435, a436)?
                                    };
                                    nat_mul(a437, a438)?
                                };
                                let a440 = 1u64;
                                nat_add(a439, a440)?
                            };
                            nat_mul(a441, a442)?
                        };
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    };
                    nat_mul(a445, a446)?
                })?;
                nat_add(a447, a448)
            }
        } else {
            if f12(v0)? {
                Ok(v0)
            } else {
                if f12(v1)? {
                    Ok(v1)
                } else {
                    let a449 = f21(v1)?;
                    let a450 = f21(v0)?;
                    if nat_le(a449, a450) {
                        f22(v0, v1)
                    } else {
                        f22(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a451 = f20(v0)?;
        let a452 = 32768u64;
        nat_mul(a451, a452)?
    };
    let v2: u64 = f23(v0)?;
    let a471 = v1;
    let a472 = {
        let a453 = v2;
        let a454 = 255u64;
        if nat_eq(a453, a454) {
            let a455 = f24(v0)?;
            let a456 = 0u64;
            if nat_eq(a455, a456) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a457 = 143u64;
            let a458 = v2;
            if nat_le(a457, a458) {
                31744u64
            } else {
                let a459 = v2;
                let a460 = 102u64;
                if nat_lt(a459, a460) {
                    0u64
                } else {
                    let a461 = 113u64;
                    let a462 = v2;
                    if nat_le(a461, a462) {
                        let a467 = {
                            let a465 = {
                                let a463 = v2;
                                let a464 = 113u64;
                                nat_sub(a463, a464)
                            };
                            let a466 = 1024u64;
                            nat_mul(a465, a466)?
                        };
                        let a468 = f25(f26(v0)?, 8192u64)?;
                        nat_add(a467, a468)?
                    } else {
                        f25(f26(v0)?, f14({
                            let a469 = 126u64;
                            let a470 = v2;
                            nat_sub(a469, a470)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a471, a472)
}

pub fn f9(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a479 = {
        let a476 = {
            let a473 = v1;
            let a474 = v2;
            let a475 = 0u64;
            nat_quot(a473, a474, a475)
        };
        let a477 = 2u64;
        let a478 = 0u64;
        nat_rem(a476, a477, a478)
    };
    let a480 = 1u64;
    if nat_eq(a479, a480) {
        Ok(v0)
    } else {
        let a481 = 240u64;
        let a482 = v0;
        nat_add(a481, a482)
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a489 = {
        let a486 = {
            let a483 = v1;
            let a484 = v2;
            let a485 = 0u64;
            nat_quot(a483, a484, a485)
        };
        let a487 = 2u64;
        let a488 = 0u64;
        nat_rem(a486, a487, a488)
    };
    let a490 = 1u64;
    if nat_eq(a489, a490) {
        let a491 = 16u64;
        let a492 = v0;
        nat_add(a491, a492)
    } else {
        Ok(v0)
    }
}

pub fn f11(v0: u64) -> R<bool> {
    let a535 = f23(v0)?;
    let a536 = {
        let a533 = {
            let a531 = {
                let a493 = 1u64;
                let a494 = 1u64;
                nat_add(a493, a494)?
            };
            let a532 = {
                let a529 = {
                    let a527 = {
                        let a495 = 1u64;
                        let a496 = 1u64;
                        nat_add(a495, a496)?
                    };
                    let a528 = {
                        let a525 = {
                            let a523 = {
                                let a497 = 1u64;
                                let a498 = 1u64;
                                nat_add(a497, a498)?
                            };
                            let a524 = {
                                let a521 = {
                                    let a519 = {
                                        let a499 = 1u64;
                                        let a500 = 1u64;
                                        nat_add(a499, a500)?
                                    };
                                    let a520 = {
                                        let a517 = {
                                            let a515 = {
                                                let a501 = 1u64;
                                                let a502 = 1u64;
                                                nat_add(a501, a502)?
                                            };
                                            let a516 = {
                                                let a513 = {
                                                    let a511 = {
                                                        let a503 = 1u64;
                                                        let a504 = 1u64;
                                                        nat_add(a503, a504)?
                                                    };
                                                    let a512 = {
                                                        let a509 = {
                                                            let a507 = {
                                                                let a505 = 1u64;
                                                                let a506 = 1u64;
                                                                nat_add(a505, a506)?
                                                            };
                                                            let a508 = 1u64;
                                                            nat_mul(a507, a508)?
                                                        };
                                                        let a510 = 1u64;
                                                        nat_add(a509, a510)?
                                                    };
                                                    nat_mul(a511, a512)?
                                                };
                                                let a514 = 1u64;
                                                nat_add(a513, a514)?
                                            };
                                            nat_mul(a515, a516)?
                                        };
                                        let a518 = 1u64;
                                        nat_add(a517, a518)?
                                    };
                                    nat_mul(a519, a520)?
                                };
                                let a522 = 1u64;
                                nat_add(a521, a522)?
                            };
                            nat_mul(a523, a524)?
                        };
                        let a526 = 1u64;
                        nat_add(a525, a526)?
                    };
                    nat_mul(a527, a528)?
                };
                let a530 = 1u64;
                nat_add(a529, a530)?
            };
            nat_mul(a531, a532)?
        };
        let a534 = 1u64;
        nat_add(a533, a534)?
    };
    if nat_eq(a535, a536) {
        let a537 = 0u64;
        let a538 = f24(v0)?;
        Ok(nat_lt(a537, a538))
    } else {
        Ok(false)
    }
}

pub fn f12(v0: u64) -> R<bool> {
    let a581 = f23(v0)?;
    let a582 = {
        let a579 = {
            let a577 = {
                let a539 = 1u64;
                let a540 = 1u64;
                nat_add(a539, a540)?
            };
            let a578 = {
                let a575 = {
                    let a573 = {
                        let a541 = 1u64;
                        let a542 = 1u64;
                        nat_add(a541, a542)?
                    };
                    let a574 = {
                        let a571 = {
                            let a569 = {
                                let a543 = 1u64;
                                let a544 = 1u64;
                                nat_add(a543, a544)?
                            };
                            let a570 = {
                                let a567 = {
                                    let a565 = {
                                        let a545 = 1u64;
                                        let a546 = 1u64;
                                        nat_add(a545, a546)?
                                    };
                                    let a566 = {
                                        let a563 = {
                                            let a561 = {
                                                let a547 = 1u64;
                                                let a548 = 1u64;
                                                nat_add(a547, a548)?
                                            };
                                            let a562 = {
                                                let a559 = {
                                                    let a557 = {
                                                        let a549 = 1u64;
                                                        let a550 = 1u64;
                                                        nat_add(a549, a550)?
                                                    };
                                                    let a558 = {
                                                        let a555 = {
                                                            let a553 = {
                                                                let a551 = 1u64;
                                                                let a552 = 1u64;
                                                                nat_add(a551, a552)?
                                                            };
                                                            let a554 = 1u64;
                                                            nat_mul(a553, a554)?
                                                        };
                                                        let a556 = 1u64;
                                                        nat_add(a555, a556)?
                                                    };
                                                    nat_mul(a557, a558)?
                                                };
                                                let a560 = 1u64;
                                                nat_add(a559, a560)?
                                            };
                                            nat_mul(a561, a562)?
                                        };
                                        let a564 = 1u64;
                                        nat_add(a563, a564)?
                                    };
                                    nat_mul(a565, a566)?
                                };
                                let a568 = 1u64;
                                nat_add(a567, a568)?
                            };
                            nat_mul(a569, a570)?
                        };
                        let a572 = 1u64;
                        nat_add(a571, a572)?
                    };
                    nat_mul(a573, a574)?
                };
                let a576 = 1u64;
                nat_add(a575, a576)?
            };
            nat_mul(a577, a578)?
        };
        let a580 = 1u64;
        nat_add(a579, a580)?
    };
    if nat_eq(a581, a582) {
        let a583 = f24(v0)?;
        let a584 = 0u64;
        Ok(nat_eq(a583, a584))
    } else {
        Ok(false)
    }
}

pub fn f13(v0: u64) -> R<bool> {
    let a585 = f21(v0)?;
    let a586 = 0u64;
    Ok(nat_eq(a585, a586))
}

pub fn f14(v0: u64) -> R<u64> {
    f27(Fn0::F28, v0, 1u64)
}

pub fn f15(v0: u64) -> R<u64> {
    let a653 = f17(v0)?;
    let a654 = {
        let a651 = {
            let a627 = {
                let a625 = {
                    let a587 = 1u64;
                    let a588 = 1u64;
                    nat_add(a587, a588)?
                };
                let a626 = {
                    let a623 = {
                        let a621 = {
                            let a589 = 1u64;
                            let a590 = 1u64;
                            nat_add(a589, a590)?
                        };
                        let a622 = {
                            let a619 = {
                                let a617 = {
                                    let a591 = 1u64;
                                    let a592 = 1u64;
                                    nat_add(a591, a592)?
                                };
                                let a618 = {
                                    let a615 = {
                                        let a613 = {
                                            let a593 = 1u64;
                                            let a594 = 1u64;
                                            nat_add(a593, a594)?
                                        };
                                        let a614 = {
                                            let a611 = {
                                                let a609 = {
                                                    let a595 = 1u64;
                                                    let a596 = 1u64;
                                                    nat_add(a595, a596)?
                                                };
                                                let a610 = {
                                                    let a607 = {
                                                        let a605 = {
                                                            let a597 = 1u64;
                                                            let a598 = 1u64;
                                                            nat_add(a597, a598)?
                                                        };
                                                        let a606 = {
                                                            let a603 = {
                                                                let a601 = {
                                                                    let a599 = 1u64;
                                                                    let a600 = 1u64;
                                                                    nat_add(a599, a600)?
                                                                };
                                                                let a602 = 1u64;
                                                                nat_mul(a601, a602)?
                                                            };
                                                            let a604 = 1u64;
                                                            nat_add(a603, a604)?
                                                        };
                                                        nat_mul(a605, a606)?
                                                    };
                                                    let a608 = 1u64;
                                                    nat_add(a607, a608)?
                                                };
                                                nat_mul(a609, a610)?
                                            };
                                            let a612 = 1u64;
                                            nat_add(a611, a612)?
                                        };
                                        nat_mul(a613, a614)?
                                    };
                                    let a616 = 1u64;
                                    nat_add(a615, a616)?
                                };
                                nat_mul(a617, a618)?
                            };
                            let a620 = 1u64;
                            nat_add(a619, a620)?
                        };
                        nat_mul(a621, a622)?
                    };
                    let a624 = 1u64;
                    nat_add(a623, a624)?
                };
                nat_mul(a625, a626)?
            };
            let a628 = 1u64;
            nat_add(a627, a628)?
        };
        let a652 = f14({
            let a649 = {
                let a647 = {
                    let a629 = 1u64;
                    let a630 = 1u64;
                    nat_add(a629, a630)?
                };
                let a648 = {
                    let a645 = {
                        let a643 = {
                            let a631 = 1u64;
                            let a632 = 1u64;
                            nat_add(a631, a632)?
                        };
                        let a644 = {
                            let a641 = {
                                let a639 = {
                                    let a633 = 1u64;
                                    let a634 = 1u64;
                                    nat_add(a633, a634)?
                                };
                                let a640 = {
                                    let a637 = {
                                        let a635 = 1u64;
                                        let a636 = 1u64;
                                        nat_add(a635, a636)?
                                    };
                                    let a638 = 1u64;
                                    nat_mul(a637, a638)?
                                };
                                nat_mul(a639, a640)?
                            };
                            let a642 = 1u64;
                            nat_add(a641, a642)?
                        };
                        nat_mul(a643, a644)?
                    };
                    let a646 = 1u64;
                    nat_add(a645, a646)?
                };
                nat_mul(a647, a648)?
            };
            let a650 = 1u64;
            nat_add(a649, a650)?
        })?;
        nat_mul(a651, a652)?
    };
    nat_add(a653, a654)
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let a661 = {
        let a655 = f20(v0)?;
        let a656 = f20(v1)?;
        nat_add(a655, a656)?
    };
    let a662 = {
        let a659 = {
            let a657 = 1u64;
            let a658 = 1u64;
            nat_add(a657, a658)?
        };
        let a660 = 1u64;
        nat_mul(a659, a660)?
    };
    let a663 = 0u64;
    Ok(nat_rem(a661, a662, a663))
}

pub fn f17(v0: u64) -> R<u64> {
    let a688 = v0;
    let a689 = f14({
        let a686 = {
            let a684 = {
                let a664 = 1u64;
                let a665 = 1u64;
                nat_add(a664, a665)?
            };
            let a685 = {
                let a682 = {
                    let a680 = {
                        let a666 = 1u64;
                        let a667 = 1u64;
                        nat_add(a666, a667)?
                    };
                    let a681 = {
                        let a678 = {
                            let a676 = {
                                let a668 = 1u64;
                                let a669 = 1u64;
                                nat_add(a668, a669)?
                            };
                            let a677 = {
                                let a674 = {
                                    let a672 = {
                                        let a670 = 1u64;
                                        let a671 = 1u64;
                                        nat_add(a670, a671)?
                                    };
                                    let a673 = 1u64;
                                    nat_mul(a672, a673)?
                                };
                                let a675 = 1u64;
                                nat_add(a674, a675)?
                            };
                            nat_mul(a676, a677)?
                        };
                        let a679 = 1u64;
                        nat_add(a678, a679)?
                    };
                    nat_mul(a680, a681)?
                };
                let a683 = 1u64;
                nat_add(a682, a683)?
            };
            nat_mul(a684, a685)?
        };
        let a687 = 1u64;
        nat_add(a686, a687)?
    })?;
    nat_mul(a688, a689)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f29(v0)?;
    let v3: Adt2 = f29(v1)?;
    f30(f16(v0, v1)?, {
        let a744 = {
            let a692 = match v2.clone() {
                Adt2::C0(h690, _, _, _) => {
                    h690
                }
            };
            let a693 = match v3.clone() {
                Adt2::C0(h691, _, _, _) => {
                    h691
                }
            };
            nat_add(a692, a693)?
        };
        let a745 = {
            let a742 = {
                let a740 = {
                    let a694 = 1u64;
                    let a695 = 1u64;
                    nat_add(a694, a695)?
                };
                let a741 = {
                    let a738 = {
                        let a736 = {
                            let a696 = 1u64;
                            let a697 = 1u64;
                            nat_add(a696, a697)?
                        };
                        let a737 = {
                            let a734 = {
                                let a732 = {
                                    let a698 = 1u64;
                                    let a699 = 1u64;
                                    nat_add(a698, a699)?
                                };
                                let a733 = {
                                    let a730 = {
                                        let a728 = {
                                            let a700 = 1u64;
                                            let a701 = 1u64;
                                            nat_add(a700, a701)?
                                        };
                                        let a729 = {
                                            let a726 = {
                                                let a724 = {
                                                    let a702 = 1u64;
                                                    let a703 = 1u64;
                                                    nat_add(a702, a703)?
                                                };
                                                let a725 = {
                                                    let a722 = {
                                                        let a720 = {
                                                            let a704 = 1u64;
                                                            let a705 = 1u64;
                                                            nat_add(a704, a705)?
                                                        };
                                                        let a721 = {
                                                            let a718 = {
                                                                let a716 = {
                                                                    let a706 = 1u64;
                                                                    let a707 = 1u64;
                                                                    nat_add(a706, a707)?
                                                                };
                                                                let a717 = {
                                                                    let a714 = {
                                                                        let a708 = 1u64;
                                                                        let a709 = 1u64;
                                                                        nat_add(a708, a709)?
                                                                    };
                                                                    let a715 = {
                                                                        let a712 = {
                                                                            let a710 = 1u64;
                                                                            let a711 = 1u64;
                                                                            nat_add(a710, a711)?
                                                                        };
                                                                        let a713 = 1u64;
                                                                        nat_mul(a712, a713)?
                                                                    };
                                                                    nat_mul(a714, a715)?
                                                                };
                                                                nat_mul(a716, a717)?
                                                            };
                                                            let a719 = 1u64;
                                                            nat_add(a718, a719)?
                                                        };
                                                        nat_mul(a720, a721)?
                                                    };
                                                    let a723 = 1u64;
                                                    nat_add(a722, a723)?
                                                };
                                                nat_mul(a724, a725)?
                                            };
                                            let a727 = 1u64;
                                            nat_add(a726, a727)?
                                        };
                                        nat_mul(a728, a729)?
                                    };
                                    let a731 = 1u64;
                                    nat_add(a730, a731)?
                                };
                                nat_mul(a732, a733)?
                            };
                            let a735 = 1u64;
                            nat_add(a734, a735)?
                        };
                        nat_mul(a736, a737)?
                    };
                    let a739 = 1u64;
                    nat_add(a738, a739)?
                };
                nat_mul(a740, a741)?
            };
            let a743 = 1u64;
            nat_add(a742, a743)?
        };
        nat_sub(a744, a745)
    }, f31({
        let a748 = match v2 {
            Adt2::C0(_, h746, _, _) => {
                h746
            }
        };
        let a749 = match v3 {
            Adt2::C0(_, h747, _, _) => {
                h747
            }
        };
        nat_mul(a748, a749)?
    }, {
        let a766 = {
            let a750 = 1u64;
            let a751 = 1u64;
            nat_add(a750, a751)?
        };
        let a767 = {
            let a764 = {
                let a752 = 1u64;
                let a753 = 1u64;
                nat_add(a752, a753)?
            };
            let a765 = {
                let a762 = {
                    let a760 = {
                        let a754 = 1u64;
                        let a755 = 1u64;
                        nat_add(a754, a755)?
                    };
                    let a761 = {
                        let a758 = {
                            let a756 = 1u64;
                            let a757 = 1u64;
                            nat_add(a756, a757)?
                        };
                        let a759 = 1u64;
                        nat_mul(a758, a759)?
                    };
                    nat_mul(a760, a761)?
                };
                let a763 = 1u64;
                nat_add(a762, a763)?
            };
            nat_mul(a764, a765)?
        };
        nat_mul(a766, a767)?
    })?)
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h768, _) = f32(Fn1::F33, {
        let a787 = {
            let a769 = 1u64;
            let a770 = 1u64;
            nat_add(a769, a770)?
        };
        let a788 = {
            let a785 = {
                let a771 = 1u64;
                let a772 = 1u64;
                nat_add(a771, a772)?
            };
            let a786 = {
                let a783 = {
                    let a773 = 1u64;
                    let a774 = 1u64;
                    nat_add(a773, a774)?
                };
                let a784 = {
                    let a781 = {
                        let a775 = 1u64;
                        let a776 = 1u64;
                        nat_add(a775, a776)?
                    };
                    let a782 = {
                        let a779 = {
                            let a777 = 1u64;
                            let a778 = 1u64;
                            nat_add(a777, a778)?
                        };
                        let a780 = 1u64;
                        nat_mul(a779, a780)?
                    };
                    nat_mul(a781, a782)?
                };
                nat_mul(a783, a784)?
            };
            nat_mul(a785, a786)?
        };
        nat_mul(a787, a788)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h768)
}

pub fn f20(v0: u64) -> R<u64> {
    let a813 = v0;
    let a814 = f14({
        let a811 = {
            let a809 = {
                let a789 = 1u64;
                let a790 = 1u64;
                nat_add(a789, a790)?
            };
            let a810 = {
                let a807 = {
                    let a805 = {
                        let a791 = 1u64;
                        let a792 = 1u64;
                        nat_add(a791, a792)?
                    };
                    let a806 = {
                        let a803 = {
                            let a801 = {
                                let a793 = 1u64;
                                let a794 = 1u64;
                                nat_add(a793, a794)?
                            };
                            let a802 = {
                                let a799 = {
                                    let a797 = {
                                        let a795 = 1u64;
                                        let a796 = 1u64;
                                        nat_add(a795, a796)?
                                    };
                                    let a798 = 1u64;
                                    nat_mul(a797, a798)?
                                };
                                let a800 = 1u64;
                                nat_add(a799, a800)?
                            };
                            nat_mul(a801, a802)?
                        };
                        let a804 = 1u64;
                        nat_add(a803, a804)?
                    };
                    nat_mul(a805, a806)?
                };
                let a808 = 1u64;
                nat_add(a807, a808)?
            };
            nat_mul(a809, a810)?
        };
        let a812 = 1u64;
        nat_add(a811, a812)?
    })?;
    let a815 = 0u64;
    Ok(nat_quot(a813, a814, a815))
}

pub fn f21(v0: u64) -> R<u64> {
    let a840 = v0;
    let a841 = f14({
        let a838 = {
            let a836 = {
                let a816 = 1u64;
                let a817 = 1u64;
                nat_add(a816, a817)?
            };
            let a837 = {
                let a834 = {
                    let a832 = {
                        let a818 = 1u64;
                        let a819 = 1u64;
                        nat_add(a818, a819)?
                    };
                    let a833 = {
                        let a830 = {
                            let a828 = {
                                let a820 = 1u64;
                                let a821 = 1u64;
                                nat_add(a820, a821)?
                            };
                            let a829 = {
                                let a826 = {
                                    let a824 = {
                                        let a822 = 1u64;
                                        let a823 = 1u64;
                                        nat_add(a822, a823)?
                                    };
                                    let a825 = 1u64;
                                    nat_mul(a824, a825)?
                                };
                                let a827 = 1u64;
                                nat_add(a826, a827)?
                            };
                            nat_mul(a828, a829)?
                        };
                        let a831 = 1u64;
                        nat_add(a830, a831)?
                    };
                    nat_mul(a832, a833)?
                };
                let a835 = 1u64;
                nat_add(a834, a835)?
            };
            nat_mul(a836, a837)?
        };
        let a839 = 1u64;
        nat_add(a838, a839)?
    })?;
    let a842 = 0u64;
    Ok(nat_rem(a840, a841, a842))
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f34(v0)?;
    let v3: u64 = {
        let a855 = f26(v0)?;
        let a856 = {
            let a853 = {
                let a843 = 1u64;
                let a844 = 1u64;
                nat_add(a843, a844)?
            };
            let a854 = {
                let a851 = {
                    let a845 = 1u64;
                    let a846 = 1u64;
                    nat_add(a845, a846)?
                };
                let a852 = {
                    let a849 = {
                        let a847 = 1u64;
                        let a848 = 1u64;
                        nat_add(a847, a848)?
                    };
                    let a850 = 1u64;
                    nat_mul(a849, a850)?
                };
                nat_mul(a851, a852)?
            };
            nat_mul(a853, a854)?
        };
        nat_mul(a855, a856)?
    };
    let v4: u64 = f31({
        let a869 = f26(v1)?;
        let a870 = {
            let a867 = {
                let a857 = 1u64;
                let a858 = 1u64;
                nat_add(a857, a858)?
            };
            let a868 = {
                let a865 = {
                    let a859 = 1u64;
                    let a860 = 1u64;
                    nat_add(a859, a860)?
                };
                let a866 = {
                    let a863 = {
                        let a861 = 1u64;
                        let a862 = 1u64;
                        nat_add(a861, a862)?
                    };
                    let a864 = 1u64;
                    nat_mul(a863, a864)?
                };
                nat_mul(a865, a866)?
            };
            nat_mul(a867, a868)?
        };
        nat_mul(a869, a870)?
    }, {
        let a871 = v2;
        let a872 = f34(v1)?;
        nat_sub(a871, a872)
    })?;
    let a873 = f20(v0)?;
    let a874 = f20(v1)?;
    if nat_eq(a873, a874) {
        f30(f20(v0)?, v2, {
            let a875 = v3;
            let a876 = v4;
            nat_add(a875, a876)?
        })
    } else {
        let a877 = v3;
        let a878 = v4;
        if nat_eq(a877, a878) {
            Ok(0u64)
        } else {
            f30(f20(v0)?, v2, {
                let a879 = v3;
                let a880 = v4;
                nat_sub(a879, a880)
            })
        }
    }
}

pub fn f23(v0: u64) -> R<u64> {
    let a903 = f21(v0)?;
    let a904 = f14({
        let a901 = {
            let a899 = {
                let a881 = 1u64;
                let a882 = 1u64;
                nat_add(a881, a882)?
            };
            let a900 = {
                let a897 = {
                    let a895 = {
                        let a883 = 1u64;
                        let a884 = 1u64;
                        nat_add(a883, a884)?
                    };
                    let a896 = {
                        let a893 = {
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
                        let a894 = 1u64;
                        nat_add(a893, a894)?
                    };
                    nat_mul(a895, a896)?
                };
                let a898 = 1u64;
                nat_add(a897, a898)?
            };
            nat_mul(a899, a900)?
        };
        let a902 = 1u64;
        nat_add(a901, a902)?
    })?;
    let a905 = 0u64;
    Ok(nat_quot(a903, a904, a905))
}

pub fn f24(v0: u64) -> R<u64> {
    let a928 = v0;
    let a929 = f14({
        let a926 = {
            let a924 = {
                let a906 = 1u64;
                let a907 = 1u64;
                nat_add(a906, a907)?
            };
            let a925 = {
                let a922 = {
                    let a920 = {
                        let a908 = 1u64;
                        let a909 = 1u64;
                        nat_add(a908, a909)?
                    };
                    let a921 = {
                        let a918 = {
                            let a916 = {
                                let a910 = 1u64;
                                let a911 = 1u64;
                                nat_add(a910, a911)?
                            };
                            let a917 = {
                                let a914 = {
                                    let a912 = 1u64;
                                    let a913 = 1u64;
                                    nat_add(a912, a913)?
                                };
                                let a915 = 1u64;
                                nat_mul(a914, a915)?
                            };
                            nat_mul(a916, a917)?
                        };
                        let a919 = 1u64;
                        nat_add(a918, a919)?
                    };
                    nat_mul(a920, a921)?
                };
                let a923 = 1u64;
                nat_add(a922, a923)?
            };
            nat_mul(a924, a925)?
        };
        let a927 = 1u64;
        nat_add(a926, a927)?
    })?;
    let a930 = 0u64;
    Ok(nat_rem(a928, a929, a930))
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a931 = v0;
        let a932 = v1;
        let a933 = 0u64;
        nat_quot(a931, a932, a933)
    };
    let v3: u64 = {
        let a937 = {
            let a934 = v0;
            let a935 = v1;
            let a936 = 0u64;
            nat_rem(a934, a935, a936)
        };
        let a938 = 2u64;
        nat_mul(a937, a938)?
    };
    let a948 = v2;
    let a949 = {
        let a939 = v1;
        let a940 = v3;
        if if nat_lt(a939, a940) {
            true
        } else {
            let a941 = v3;
            let a942 = v1;
            if nat_eq(a941, a942) {
                let a946 = {
                    let a943 = v2;
                    let a944 = 2u64;
                    let a945 = 0u64;
                    nat_rem(a943, a944, a945)
                };
                let a947 = 1u64;
                nat_eq(a946, a947)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a948, a949)
}

pub fn f26(v0: u64) -> R<u64> {
    let a974 = f24(v0)?;
    let a975 = {
        let a950 = f23(v0)?;
        let a951 = 0u64;
        if nat_eq(a950, a951) {
            0u64
        } else {
            f14({
                let a972 = {
                    let a970 = {
                        let a952 = 1u64;
                        let a953 = 1u64;
                        nat_add(a952, a953)?
                    };
                    let a971 = {
                        let a968 = {
                            let a966 = {
                                let a954 = 1u64;
                                let a955 = 1u64;
                                nat_add(a954, a955)?
                            };
                            let a967 = {
                                let a964 = {
                                    let a962 = {
                                        let a956 = 1u64;
                                        let a957 = 1u64;
                                        nat_add(a956, a957)?
                                    };
                                    let a963 = {
                                        let a960 = {
                                            let a958 = 1u64;
                                            let a959 = 1u64;
                                            nat_add(a958, a959)?
                                        };
                                        let a961 = 1u64;
                                        nat_mul(a960, a961)?
                                    };
                                    nat_mul(a962, a963)?
                                };
                                let a965 = 1u64;
                                nat_add(a964, a965)?
                            };
                            nat_mul(a966, a967)?
                        };
                        let a969 = 1u64;
                        nat_add(a968, a969)?
                    };
                    nat_mul(a970, a971)?
                };
                let a973 = 1u64;
                nat_add(a972, a973)?
            })?
        }
    };
    nat_add(a974, a975)
}

pub fn f27(v0: Fn0, p979: u64, p980: u64) -> R<u64> {
    let m978 = (p979, p980);
    {
        let mut m978 = m978;
        loop {
            let step = {
                let (v1, v2) = m978;
                let m976 = v1;
                if m976 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m976 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c977: Fn0 = v0.clone();
                        c977.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: u64) -> R<u64> {
    let a981 = v0;
    let a982 = v0;
    nat_add(a981, a982)
}

pub fn f29(v0: u64) -> R<Adt2> {
    f19(f34(v0)?, f26(v0)?, 0u64, f14({
        let a1003 = {
            let a1001 = {
                let a983 = 1u64;
                let a984 = 1u64;
                nat_add(a983, a984)?
            };
            let a1002 = {
                let a999 = {
                    let a997 = {
                        let a985 = 1u64;
                        let a986 = 1u64;
                        nat_add(a985, a986)?
                    };
                    let a998 = {
                        let a995 = {
                            let a993 = {
                                let a987 = 1u64;
                                let a988 = 1u64;
                                nat_add(a987, a988)?
                            };
                            let a994 = {
                                let a991 = {
                                    let a989 = 1u64;
                                    let a990 = 1u64;
                                    nat_add(a989, a990)?
                                };
                                let a992 = 1u64;
                                nat_mul(a991, a992)?
                            };
                            nat_mul(a993, a994)?
                        };
                        let a996 = 1u64;
                        nat_add(a995, a996)?
                    };
                    nat_mul(a997, a998)?
                };
                let a1000 = 1u64;
                nat_add(a999, a1000)?
            };
            nat_mul(a1001, a1002)?
        };
        let a1004 = 1u64;
        nat_add(a1003, a1004)?
    })?)
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1027 = f14({
        let a1025 = {
            let a1023 = {
                let a1005 = 1u64;
                let a1006 = 1u64;
                nat_add(a1005, a1006)?
            };
            let a1024 = {
                let a1021 = {
                    let a1019 = {
                        let a1007 = 1u64;
                        let a1008 = 1u64;
                        nat_add(a1007, a1008)?
                    };
                    let a1020 = {
                        let a1017 = {
                            let a1009 = 1u64;
                            let a1010 = 1u64;
                            nat_add(a1009, a1010)?
                        };
                        let a1018 = {
                            let a1015 = {
                                let a1013 = {
                                    let a1011 = 1u64;
                                    let a1012 = 1u64;
                                    nat_add(a1011, a1012)?
                                };
                                let a1014 = 1u64;
                                nat_mul(a1013, a1014)?
                            };
                            let a1016 = 1u64;
                            nat_add(a1015, a1016)?
                        };
                        nat_mul(a1017, a1018)?
                    };
                    nat_mul(a1019, a1020)?
                };
                let a1022 = 1u64;
                nat_add(a1021, a1022)?
            };
            nat_mul(a1023, a1024)?
        };
        let a1026 = 1u64;
        nat_add(a1025, a1026)?
    })?;
    let a1028 = v2;
    if nat_le(a1027, a1028) {
        f35(v0, {
            let a1029 = v1;
            let a1030 = 1u64;
            nat_add(a1029, a1030)?
        }, f31(v2, 1u64)?)
    } else {
        let v3: Adt2 = f19(v1, v2, {
            let a1067 = {
                let a1065 = {
                    let a1031 = 1u64;
                    let a1032 = 1u64;
                    nat_add(a1031, a1032)?
                };
                let a1066 = {
                    let a1063 = {
                        let a1033 = 1u64;
                        let a1034 = 1u64;
                        nat_add(a1033, a1034)?
                    };
                    let a1064 = {
                        let a1061 = {
                            let a1035 = 1u64;
                            let a1036 = 1u64;
                            nat_add(a1035, a1036)?
                        };
                        let a1062 = {
                            let a1059 = {
                                let a1037 = 1u64;
                                let a1038 = 1u64;
                                nat_add(a1037, a1038)?
                            };
                            let a1060 = {
                                let a1057 = {
                                    let a1039 = 1u64;
                                    let a1040 = 1u64;
                                    nat_add(a1039, a1040)?
                                };
                                let a1058 = {
                                    let a1055 = {
                                        let a1041 = 1u64;
                                        let a1042 = 1u64;
                                        nat_add(a1041, a1042)?
                                    };
                                    let a1056 = {
                                        let a1053 = {
                                            let a1043 = 1u64;
                                            let a1044 = 1u64;
                                            nat_add(a1043, a1044)?
                                        };
                                        let a1054 = {
                                            let a1051 = {
                                                let a1045 = 1u64;
                                                let a1046 = 1u64;
                                                nat_add(a1045, a1046)?
                                            };
                                            let a1052 = {
                                                let a1049 = {
                                                    let a1047 = 1u64;
                                                    let a1048 = 1u64;
                                                    nat_add(a1047, a1048)?
                                                };
                                                let a1050 = 1u64;
                                                nat_mul(a1049, a1050)?
                                            };
                                            nat_mul(a1051, a1052)?
                                        };
                                        nat_mul(a1053, a1054)?
                                    };
                                    nat_mul(a1055, a1056)?
                                };
                                nat_mul(a1057, a1058)?
                            };
                            nat_mul(a1059, a1060)?
                        };
                        nat_mul(a1061, a1062)?
                    };
                    nat_mul(a1063, a1064)?
                };
                nat_mul(a1065, a1066)?
            };
            let a1068 = 1u64;
            nat_add(a1067, a1068)?
        }, f14({
            let a1087 = {
                let a1069 = 1u64;
                let a1070 = 1u64;
                nat_add(a1069, a1070)?
            };
            let a1088 = {
                let a1085 = {
                    let a1083 = {
                        let a1071 = 1u64;
                        let a1072 = 1u64;
                        nat_add(a1071, a1072)?
                    };
                    let a1084 = {
                        let a1081 = {
                            let a1073 = 1u64;
                            let a1074 = 1u64;
                            nat_add(a1073, a1074)?
                        };
                        let a1082 = {
                            let a1079 = {
                                let a1077 = {
                                    let a1075 = 1u64;
                                    let a1076 = 1u64;
                                    nat_add(a1075, a1076)?
                                };
                                let a1078 = 1u64;
                                nat_mul(a1077, a1078)?
                            };
                            let a1080 = 1u64;
                            nat_add(a1079, a1080)?
                        };
                        nat_mul(a1081, a1082)?
                    };
                    nat_mul(a1083, a1084)?
                };
                let a1086 = 1u64;
                nat_add(a1085, a1086)?
            };
            nat_mul(a1087, a1088)?
        })?)?;
        f35(v0, match v3.clone() {
            Adt2::C0(h1089, _, _, _) => {
                h1089
            }
        }, match v3 {
            Adt2::C0(_, h1090, _, _) => {
                h1090
            }
        })
    }
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let a1091 = v1;
    let a1092 = 0u64;
    if nat_eq(a1091, a1092) {
        Ok(v0)
    } else {
        let a1117 = {
            let a1115 = {
                let a1093 = 1u64;
                let a1094 = 1u64;
                nat_add(a1093, a1094)?
            };
            let a1116 = {
                let a1113 = {
                    let a1095 = 1u64;
                    let a1096 = 1u64;
                    nat_add(a1095, a1096)?
                };
                let a1114 = {
                    let a1111 = {
                        let a1097 = 1u64;
                        let a1098 = 1u64;
                        nat_add(a1097, a1098)?
                    };
                    let a1112 = {
                        let a1109 = {
                            let a1099 = 1u64;
                            let a1100 = 1u64;
                            nat_add(a1099, a1100)?
                        };
                        let a1110 = {
                            let a1107 = {
                                let a1101 = 1u64;
                                let a1102 = 1u64;
                                nat_add(a1101, a1102)?
                            };
                            let a1108 = {
                                let a1105 = {
                                    let a1103 = 1u64;
                                    let a1104 = 1u64;
                                    nat_add(a1103, a1104)?
                                };
                                let a1106 = 1u64;
                                nat_mul(a1105, a1106)?
                            };
                            nat_mul(a1107, a1108)?
                        };
                        nat_mul(a1109, a1110)?
                    };
                    nat_mul(a1111, a1112)?
                };
                nat_mul(a1113, a1114)?
            };
            nat_mul(a1115, a1116)?
        };
        let a1118 = v1;
        if nat_le(a1117, a1118) {
            let a1119 = v0;
            let a1120 = 0u64;
            if nat_eq(a1119, a1120) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f14(v1)?;
            f36({
                let a1121 = v0;
                let a1122 = v2;
                let a1123 = 0u64;
                nat_quot(a1121, a1122, a1123)
            }, {
                let a1127 = 0u64;
                let a1128 = {
                    let a1124 = v0;
                    let a1125 = v2;
                    let a1126 = 0u64;
                    nat_rem(a1124, a1125, a1126)
                };
                nat_lt(a1127, a1128)
            })
        }
    }
}

pub fn f32(v0: Fn1, p1133: u64, p1134: Adt2) -> R<(Adt2, bool)> {
    let m1132 = (p1133, p1134);
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v1, v2) = m1132;
                let m1129 = v1;
                if m1129 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1129 - 1;
                    let m1131 = {
                        let c1130: Fn1 = v0.clone();
                        c1130.apply(v2.clone())?
                    };
                    match m1131 {
                        None => {
                            Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt2), R<(Adt2, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: Adt2) -> R<Option<Adt2>> {
    f37(v0)
}

pub fn f34(v0: u64) -> R<u64> {
    let a1173 = {
        let a1135 = f23(v0)?;
        let a1136 = 0u64;
        if nat_eq(a1135, a1136) {
            1u64
        } else {
            f23(v0)?
        }
    };
    let a1174 = {
        let a1171 = {
            let a1137 = 1u64;
            let a1138 = 1u64;
            nat_add(a1137, a1138)?
        };
        let a1172 = {
            let a1169 = {
                let a1139 = 1u64;
                let a1140 = 1u64;
                nat_add(a1139, a1140)?
            };
            let a1170 = {
                let a1167 = {
                    let a1141 = 1u64;
                    let a1142 = 1u64;
                    nat_add(a1141, a1142)?
                };
                let a1168 = {
                    let a1165 = {
                        let a1143 = 1u64;
                        let a1144 = 1u64;
                        nat_add(a1143, a1144)?
                    };
                    let a1166 = {
                        let a1163 = {
                            let a1145 = 1u64;
                            let a1146 = 1u64;
                            nat_add(a1145, a1146)?
                        };
                        let a1164 = {
                            let a1161 = {
                                let a1147 = 1u64;
                                let a1148 = 1u64;
                                nat_add(a1147, a1148)?
                            };
                            let a1162 = {
                                let a1159 = {
                                    let a1149 = 1u64;
                                    let a1150 = 1u64;
                                    nat_add(a1149, a1150)?
                                };
                                let a1160 = {
                                    let a1157 = {
                                        let a1151 = 1u64;
                                        let a1152 = 1u64;
                                        nat_add(a1151, a1152)?
                                    };
                                    let a1158 = {
                                        let a1155 = {
                                            let a1153 = 1u64;
                                            let a1154 = 1u64;
                                            nat_add(a1153, a1154)?
                                        };
                                        let a1156 = 1u64;
                                        nat_mul(a1155, a1156)?
                                    };
                                    nat_mul(a1157, a1158)?
                                };
                                nat_mul(a1159, a1160)?
                            };
                            nat_mul(a1161, a1162)?
                        };
                        nat_mul(a1163, a1164)?
                    };
                    nat_mul(a1165, a1166)?
                };
                nat_mul(a1167, a1168)?
            };
            nat_mul(a1169, a1170)?
        };
        nat_mul(a1171, a1172)?
    };
    nat_add(a1173, a1174)
}

pub fn f35(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1213 = v1;
    let a1214 = {
        let a1211 = {
            let a1209 = {
                let a1175 = 1u64;
                let a1176 = 1u64;
                nat_add(a1175, a1176)?
            };
            let a1210 = {
                let a1207 = {
                    let a1177 = 1u64;
                    let a1178 = 1u64;
                    nat_add(a1177, a1178)?
                };
                let a1208 = {
                    let a1205 = {
                        let a1179 = 1u64;
                        let a1180 = 1u64;
                        nat_add(a1179, a1180)?
                    };
                    let a1206 = {
                        let a1203 = {
                            let a1181 = 1u64;
                            let a1182 = 1u64;
                            nat_add(a1181, a1182)?
                        };
                        let a1204 = {
                            let a1201 = {
                                let a1183 = 1u64;
                                let a1184 = 1u64;
                                nat_add(a1183, a1184)?
                            };
                            let a1202 = {
                                let a1199 = {
                                    let a1185 = 1u64;
                                    let a1186 = 1u64;
                                    nat_add(a1185, a1186)?
                                };
                                let a1200 = {
                                    let a1197 = {
                                        let a1187 = 1u64;
                                        let a1188 = 1u64;
                                        nat_add(a1187, a1188)?
                                    };
                                    let a1198 = {
                                        let a1195 = {
                                            let a1189 = 1u64;
                                            let a1190 = 1u64;
                                            nat_add(a1189, a1190)?
                                        };
                                        let a1196 = {
                                            let a1193 = {
                                                let a1191 = 1u64;
                                                let a1192 = 1u64;
                                                nat_add(a1191, a1192)?
                                            };
                                            let a1194 = 1u64;
                                            nat_mul(a1193, a1194)?
                                        };
                                        nat_mul(a1195, a1196)?
                                    };
                                    nat_mul(a1197, a1198)?
                                };
                                nat_mul(a1199, a1200)?
                            };
                            nat_mul(a1201, a1202)?
                        };
                        nat_mul(a1203, a1204)?
                    };
                    nat_mul(a1205, a1206)?
                };
                nat_mul(a1207, a1208)?
            };
            nat_mul(a1209, a1210)?
        };
        let a1212 = 1u64;
        nat_add(a1211, a1212)?
    };
    if nat_lt(a1213, a1214) {
        f38(v0, {
            let a1251 = {
                let a1249 = {
                    let a1215 = 1u64;
                    let a1216 = 1u64;
                    nat_add(a1215, a1216)?
                };
                let a1250 = {
                    let a1247 = {
                        let a1217 = 1u64;
                        let a1218 = 1u64;
                        nat_add(a1217, a1218)?
                    };
                    let a1248 = {
                        let a1245 = {
                            let a1219 = 1u64;
                            let a1220 = 1u64;
                            nat_add(a1219, a1220)?
                        };
                        let a1246 = {
                            let a1243 = {
                                let a1221 = 1u64;
                                let a1222 = 1u64;
                                nat_add(a1221, a1222)?
                            };
                            let a1244 = {
                                let a1241 = {
                                    let a1223 = 1u64;
                                    let a1224 = 1u64;
                                    nat_add(a1223, a1224)?
                                };
                                let a1242 = {
                                    let a1239 = {
                                        let a1225 = 1u64;
                                        let a1226 = 1u64;
                                        nat_add(a1225, a1226)?
                                    };
                                    let a1240 = {
                                        let a1237 = {
                                            let a1227 = 1u64;
                                            let a1228 = 1u64;
                                            nat_add(a1227, a1228)?
                                        };
                                        let a1238 = {
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
                                        nat_mul(a1237, a1238)?
                                    };
                                    nat_mul(a1239, a1240)?
                                };
                                nat_mul(a1241, a1242)?
                            };
                            nat_mul(a1243, a1244)?
                        };
                        nat_mul(a1245, a1246)?
                    };
                    nat_mul(a1247, a1248)?
                };
                nat_mul(a1249, a1250)?
            };
            let a1252 = 1u64;
            nat_add(a1251, a1252)?
        }, f31(v2, {
            let a1291 = {
                let a1289 = {
                    let a1287 = {
                        let a1253 = 1u64;
                        let a1254 = 1u64;
                        nat_add(a1253, a1254)?
                    };
                    let a1288 = {
                        let a1285 = {
                            let a1255 = 1u64;
                            let a1256 = 1u64;
                            nat_add(a1255, a1256)?
                        };
                        let a1286 = {
                            let a1283 = {
                                let a1257 = 1u64;
                                let a1258 = 1u64;
                                nat_add(a1257, a1258)?
                            };
                            let a1284 = {
                                let a1281 = {
                                    let a1259 = 1u64;
                                    let a1260 = 1u64;
                                    nat_add(a1259, a1260)?
                                };
                                let a1282 = {
                                    let a1279 = {
                                        let a1261 = 1u64;
                                        let a1262 = 1u64;
                                        nat_add(a1261, a1262)?
                                    };
                                    let a1280 = {
                                        let a1277 = {
                                            let a1263 = 1u64;
                                            let a1264 = 1u64;
                                            nat_add(a1263, a1264)?
                                        };
                                        let a1278 = {
                                            let a1275 = {
                                                let a1265 = 1u64;
                                                let a1266 = 1u64;
                                                nat_add(a1265, a1266)?
                                            };
                                            let a1276 = {
                                                let a1273 = {
                                                    let a1267 = 1u64;
                                                    let a1268 = 1u64;
                                                    nat_add(a1267, a1268)?
                                                };
                                                let a1274 = {
                                                    let a1271 = {
                                                        let a1269 = 1u64;
                                                        let a1270 = 1u64;
                                                        nat_add(a1269, a1270)?
                                                    };
                                                    let a1272 = 1u64;
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
                        nat_mul(a1285, a1286)?
                    };
                    nat_mul(a1287, a1288)?
                };
                let a1290 = 1u64;
                nat_add(a1289, a1290)?
            };
            let a1292 = v1;
            nat_sub(a1291, a1292)
        })?)
    } else {
        f38(v0, v1, v2)
    }
}

pub fn f36(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1300 = {
            let a1297 = v0;
            let a1298 = {
                let a1295 = {
                    let a1293 = 1u64;
                    let a1294 = 1u64;
                    nat_add(a1293, a1294)?
                };
                let a1296 = 1u64;
                nat_mul(a1295, a1296)?
            };
            let a1299 = 0u64;
            nat_rem(a1297, a1298, a1299)
        };
        let a1301 = 0u64;
        nat_eq(a1300, a1301)
    } else {
        false
    } {
        let a1302 = v0;
        let a1303 = 1u64;
        nat_add(a1302, a1303)
    } else {
        Ok(v0)
    }
}

pub fn f37(v0: Adt2) -> R<Option<Adt2>> {
    let a1305 = 0u64;
    let a1306 = match v0.clone() {
        Adt2::C0(_, h1304, _, _) => {
            h1304
        }
    };
    if if if nat_lt(a1305, a1306) {
        let a1309 = match v0.clone() {
            Adt2::C0(_, h1307, _, _) => {
                h1307
            }
        };
        let a1310 = match v0.clone() {
            Adt2::C0(_, _, _, h1308) => {
                h1308
            }
        };
        nat_lt(a1309, a1310)
    } else {
        false
    } {
        let a1313 = match v0.clone() {
            Adt2::C0(_, _, h1311, _) => {
                h1311
            }
        };
        let a1314 = match v0.clone() {
            Adt2::C0(h1312, _, _, _) => {
                h1312
            }
        };
        nat_lt(a1313, a1314)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1316 = match v0.clone() {
                Adt2::C0(h1315, _, _, _) => {
                    h1315
                }
            };
            let a1317 = 1u64;
            nat_sub(a1316, a1317)
        }, {
            let a1320 = match v0.clone() {
                Adt2::C0(_, h1318, _, _) => {
                    h1318
                }
            };
            let a1321 = match v0.clone() {
                Adt2::C0(_, h1319, _, _) => {
                    h1319
                }
            };
            nat_add(a1320, a1321)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1322, _) => {
                h1322
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1323) => {
                h1323
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1336 = v2;
        let a1337 = {
            let a1334 = {
                let a1324 = 1u64;
                let a1325 = 1u64;
                nat_add(a1324, a1325)?
            };
            let a1335 = {
                let a1332 = {
                    let a1326 = 1u64;
                    let a1327 = 1u64;
                    nat_add(a1326, a1327)?
                };
                let a1333 = {
                    let a1330 = {
                        let a1328 = 1u64;
                        let a1329 = 1u64;
                        nat_add(a1328, a1329)?
                    };
                    let a1331 = 1u64;
                    nat_mul(a1330, a1331)?
                };
                nat_mul(a1332, a1333)?
            };
            nat_mul(a1334, a1335)?
        };
        let a1338 = 0u64;
        nat_rem(a1336, a1337, a1338)
    };
    let v4: u64 = {
        let a1351 = v2;
        let a1352 = {
            let a1349 = {
                let a1339 = 1u64;
                let a1340 = 1u64;
                nat_add(a1339, a1340)?
            };
            let a1350 = {
                let a1347 = {
                    let a1341 = 1u64;
                    let a1342 = 1u64;
                    nat_add(a1341, a1342)?
                };
                let a1348 = {
                    let a1345 = {
                        let a1343 = 1u64;
                        let a1344 = 1u64;
                        nat_add(a1343, a1344)?
                    };
                    let a1346 = 1u64;
                    nat_mul(a1345, a1346)?
                };
                nat_mul(a1347, a1348)?
            };
            nat_mul(a1349, a1350)?
        };
        let a1353 = 0u64;
        nat_quot(a1351, a1352, a1353)
    };
    let v5: bool = {
        let a1362 = {
            let a1360 = {
                let a1354 = 1u64;
                let a1355 = 1u64;
                nat_add(a1354, a1355)?
            };
            let a1361 = {
                let a1358 = {
                    let a1356 = 1u64;
                    let a1357 = 1u64;
                    nat_add(a1356, a1357)?
                };
                let a1359 = 1u64;
                nat_mul(a1358, a1359)?
            };
            nat_mul(a1360, a1361)?
        };
        let a1363 = v3;
        if nat_lt(a1362, a1363) {
            true
        } else {
            let a1372 = v3;
            let a1373 = {
                let a1370 = {
                    let a1364 = 1u64;
                    let a1365 = 1u64;
                    nat_add(a1364, a1365)?
                };
                let a1371 = {
                    let a1368 = {
                        let a1366 = 1u64;
                        let a1367 = 1u64;
                        nat_add(a1366, a1367)?
                    };
                    let a1369 = 1u64;
                    nat_mul(a1368, a1369)?
                };
                nat_mul(a1370, a1371)?
            };
            if nat_eq(a1372, a1373) {
                let a1381 = {
                    let a1378 = v4;
                    let a1379 = {
                        let a1376 = {
                            let a1374 = 1u64;
                            let a1375 = 1u64;
                            nat_add(a1374, a1375)?
                        };
                        let a1377 = 1u64;
                        nat_mul(a1376, a1377)?
                    };
                    let a1380 = 0u64;
                    nat_rem(a1378, a1379, a1380)
                };
                let a1382 = 1u64;
                nat_eq(a1381, a1382)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1383 = v4;
        let a1384 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1383, a1384)?
    };
    let v7: bool = {
        let a1413 = {
            let a1411 = {
                let a1387 = {
                    let a1385 = 1u64;
                    let a1386 = 1u64;
                    nat_add(a1385, a1386)?
                };
                let a1388 = 1u64;
                nat_mul(a1387, a1388)?
            };
            let a1412 = f14({
                let a1409 = {
                    let a1407 = {
                        let a1389 = 1u64;
                        let a1390 = 1u64;
                        nat_add(a1389, a1390)?
                    };
                    let a1408 = {
                        let a1405 = {
                            let a1403 = {
                                let a1391 = 1u64;
                                let a1392 = 1u64;
                                nat_add(a1391, a1392)?
                            };
                            let a1404 = {
                                let a1401 = {
                                    let a1399 = {
                                        let a1393 = 1u64;
                                        let a1394 = 1u64;
                                        nat_add(a1393, a1394)?
                                    };
                                    let a1400 = {
                                        let a1397 = {
                                            let a1395 = 1u64;
                                            let a1396 = 1u64;
                                            nat_add(a1395, a1396)?
                                        };
                                        let a1398 = 1u64;
                                        nat_mul(a1397, a1398)?
                                    };
                                    nat_mul(a1399, a1400)?
                                };
                                let a1402 = 1u64;
                                nat_add(a1401, a1402)?
                            };
                            nat_mul(a1403, a1404)?
                        };
                        let a1406 = 1u64;
                        nat_add(a1405, a1406)?
                    };
                    nat_mul(a1407, a1408)?
                };
                let a1410 = 1u64;
                nat_add(a1409, a1410)?
            })?;
            nat_mul(a1411, a1412)?
        };
        let a1414 = v6;
        nat_le(a1413, a1414)
    };
    let v8: u64 = if v7 {
        let a1419 = v6;
        let a1420 = {
            let a1417 = {
                let a1415 = 1u64;
                let a1416 = 1u64;
                nat_add(a1415, a1416)?
            };
            let a1418 = 1u64;
            nat_mul(a1417, a1418)?
        };
        let a1421 = 0u64;
        nat_quot(a1419, a1420, a1421)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1422 = v1;
        let a1423 = 1u64;
        nat_add(a1422, a1423)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1462 = v9;
        let a1463 = {
            let a1460 = {
                let a1458 = {
                    let a1424 = 1u64;
                    let a1425 = 1u64;
                    nat_add(a1424, a1425)?
                };
                let a1459 = {
                    let a1456 = {
                        let a1426 = 1u64;
                        let a1427 = 1u64;
                        nat_add(a1426, a1427)?
                    };
                    let a1457 = {
                        let a1454 = {
                            let a1428 = 1u64;
                            let a1429 = 1u64;
                            nat_add(a1428, a1429)?
                        };
                        let a1455 = {
                            let a1452 = {
                                let a1430 = 1u64;
                                let a1431 = 1u64;
                                nat_add(a1430, a1431)?
                            };
                            let a1453 = {
                                let a1450 = {
                                    let a1432 = 1u64;
                                    let a1433 = 1u64;
                                    nat_add(a1432, a1433)?
                                };
                                let a1451 = {
                                    let a1448 = {
                                        let a1434 = 1u64;
                                        let a1435 = 1u64;
                                        nat_add(a1434, a1435)?
                                    };
                                    let a1449 = {
                                        let a1446 = {
                                            let a1436 = 1u64;
                                            let a1437 = 1u64;
                                            nat_add(a1436, a1437)?
                                        };
                                        let a1447 = {
                                            let a1444 = {
                                                let a1438 = 1u64;
                                                let a1439 = 1u64;
                                                nat_add(a1438, a1439)?
                                            };
                                            let a1445 = {
                                                let a1442 = {
                                                    let a1440 = 1u64;
                                                    let a1441 = 1u64;
                                                    nat_add(a1440, a1441)?
                                                };
                                                let a1443 = 1u64;
                                                nat_mul(a1442, a1443)?
                                            };
                                            nat_mul(a1444, a1445)?
                                        };
                                        nat_mul(a1446, a1447)?
                                    };
                                    nat_mul(a1448, a1449)?
                                };
                                nat_mul(a1450, a1451)?
                            };
                            nat_mul(a1452, a1453)?
                        };
                        nat_mul(a1454, a1455)?
                    };
                    nat_mul(a1456, a1457)?
                };
                nat_mul(a1458, a1459)?
            };
            let a1461 = 1u64;
            nat_add(a1460, a1461)?
        };
        if if nat_eq(a1462, a1463) {
            let a1486 = v8;
            let a1487 = f14({
                let a1484 = {
                    let a1482 = {
                        let a1464 = 1u64;
                        let a1465 = 1u64;
                        nat_add(a1464, a1465)?
                    };
                    let a1483 = {
                        let a1480 = {
                            let a1478 = {
                                let a1466 = 1u64;
                                let a1467 = 1u64;
                                nat_add(a1466, a1467)?
                            };
                            let a1479 = {
                                let a1476 = {
                                    let a1474 = {
                                        let a1468 = 1u64;
                                        let a1469 = 1u64;
                                        nat_add(a1468, a1469)?
                                    };
                                    let a1475 = {
                                        let a1472 = {
                                            let a1470 = 1u64;
                                            let a1471 = 1u64;
                                            nat_add(a1470, a1471)?
                                        };
                                        let a1473 = 1u64;
                                        nat_mul(a1472, a1473)?
                                    };
                                    nat_mul(a1474, a1475)?
                                };
                                let a1477 = 1u64;
                                nat_add(a1476, a1477)?
                            };
                            nat_mul(a1478, a1479)?
                        };
                        let a1481 = 1u64;
                        nat_add(a1480, a1481)?
                    };
                    nat_mul(a1482, a1483)?
                };
                let a1485 = 1u64;
                nat_add(a1484, a1485)?
            })?;
            nat_lt(a1486, a1487)
        } else {
            false
        } {
            0u64
        } else {
            let a1524 = v9;
            let a1525 = {
                let a1522 = {
                    let a1488 = 1u64;
                    let a1489 = 1u64;
                    nat_add(a1488, a1489)?
                };
                let a1523 = {
                    let a1520 = {
                        let a1490 = 1u64;
                        let a1491 = 1u64;
                        nat_add(a1490, a1491)?
                    };
                    let a1521 = {
                        let a1518 = {
                            let a1492 = 1u64;
                            let a1493 = 1u64;
                            nat_add(a1492, a1493)?
                        };
                        let a1519 = {
                            let a1516 = {
                                let a1494 = 1u64;
                                let a1495 = 1u64;
                                nat_add(a1494, a1495)?
                            };
                            let a1517 = {
                                let a1514 = {
                                    let a1496 = 1u64;
                                    let a1497 = 1u64;
                                    nat_add(a1496, a1497)?
                                };
                                let a1515 = {
                                    let a1512 = {
                                        let a1498 = 1u64;
                                        let a1499 = 1u64;
                                        nat_add(a1498, a1499)?
                                    };
                                    let a1513 = {
                                        let a1510 = {
                                            let a1500 = 1u64;
                                            let a1501 = 1u64;
                                            nat_add(a1500, a1501)?
                                        };
                                        let a1511 = {
                                            let a1508 = {
                                                let a1502 = 1u64;
                                                let a1503 = 1u64;
                                                nat_add(a1502, a1503)?
                                            };
                                            let a1509 = {
                                                let a1506 = {
                                                    let a1504 = 1u64;
                                                    let a1505 = 1u64;
                                                    nat_add(a1504, a1505)?
                                                };
                                                let a1507 = 1u64;
                                                nat_mul(a1506, a1507)?
                                            };
                                            nat_mul(a1508, a1509)?
                                        };
                                        nat_mul(a1510, a1511)?
                                    };
                                    nat_mul(a1512, a1513)?
                                };
                                nat_mul(a1514, a1515)?
                            };
                            nat_mul(a1516, a1517)?
                        };
                        nat_mul(a1518, a1519)?
                    };
                    nat_mul(a1520, a1521)?
                };
                nat_mul(a1522, a1523)?
            };
            nat_sub(a1524, a1525)
        }
    };
    let a1526 = v8;
    let a1527 = 0u64;
    if nat_eq(a1526, a1527) {
        f17(v0)
    } else {
        let a1580 = {
            let a1578 = {
                let a1576 = {
                    let a1528 = 1u64;
                    let a1529 = 1u64;
                    nat_add(a1528, a1529)?
                };
                let a1577 = {
                    let a1574 = {
                        let a1572 = {
                            let a1530 = 1u64;
                            let a1531 = 1u64;
                            nat_add(a1530, a1531)?
                        };
                        let a1573 = {
                            let a1570 = {
                                let a1568 = {
                                    let a1532 = 1u64;
                                    let a1533 = 1u64;
                                    nat_add(a1532, a1533)?
                                };
                                let a1569 = {
                                    let a1566 = {
                                        let a1564 = {
                                            let a1534 = 1u64;
                                            let a1535 = 1u64;
                                            nat_add(a1534, a1535)?
                                        };
                                        let a1565 = {
                                            let a1562 = {
                                                let a1560 = {
                                                    let a1536 = 1u64;
                                                    let a1537 = 1u64;
                                                    nat_add(a1536, a1537)?
                                                };
                                                let a1561 = {
                                                    let a1558 = {
                                                        let a1556 = {
                                                            let a1538 = 1u64;
                                                            let a1539 = 1u64;
                                                            nat_add(a1538, a1539)?
                                                        };
                                                        let a1557 = {
                                                            let a1554 = {
                                                                let a1552 = {
                                                                    let a1540 = 1u64;
                                                                    let a1541 = 1u64;
                                                                    nat_add(a1540, a1541)?
                                                                };
                                                                let a1553 = {
                                                                    let a1550 = {
                                                                        let a1548 = {
                                                                            let a1542 = 1u64;
                                                                            let a1543 = 1u64;
                                                                            nat_add(a1542, a1543)?
                                                                        };
                                                                        let a1549 = {
                                                                            let a1546 = {
                                                                                let a1544 = 1u64;
                                                                                let a1545 = 1u64;
                                                                                nat_add(a1544, a1545)?
                                                                            };
                                                                            let a1547 = 1u64;
                                                                            nat_mul(a1546, a1547)?
                                                                        };
                                                                        nat_mul(a1548, a1549)?
                                                                    };
                                                                    let a1551 = 1u64;
                                                                    nat_add(a1550, a1551)?
                                                                };
                                                                nat_mul(a1552, a1553)?
                                                            };
                                                            let a1555 = 1u64;
                                                            nat_add(a1554, a1555)?
                                                        };
                                                        nat_mul(a1556, a1557)?
                                                    };
                                                    let a1559 = 1u64;
                                                    nat_add(a1558, a1559)?
                                                };
                                                nat_mul(a1560, a1561)?
                                            };
                                            let a1563 = 1u64;
                                            nat_add(a1562, a1563)?
                                        };
                                        nat_mul(a1564, a1565)?
                                    };
                                    let a1567 = 1u64;
                                    nat_add(a1566, a1567)?
                                };
                                nat_mul(a1568, a1569)?
                            };
                            let a1571 = 1u64;
                            nat_add(a1570, a1571)?
                        };
                        nat_mul(a1572, a1573)?
                    };
                    let a1575 = 1u64;
                    nat_add(a1574, a1575)?
                };
                nat_mul(a1576, a1577)?
            };
            let a1579 = 1u64;
            nat_add(a1578, a1579)?
        };
        let a1581 = v9;
        if nat_le(a1580, a1581) {
            f15(v0)
        } else {
            let a1633 = {
                let a1606 = f17(v0)?;
                let a1607 = {
                    let a1604 = v10;
                    let a1605 = f14({
                        let a1602 = {
                            let a1600 = {
                                let a1582 = 1u64;
                                let a1583 = 1u64;
                                nat_add(a1582, a1583)?
                            };
                            let a1601 = {
                                let a1598 = {
                                    let a1596 = {
                                        let a1584 = 1u64;
                                        let a1585 = 1u64;
                                        nat_add(a1584, a1585)?
                                    };
                                    let a1597 = {
                                        let a1594 = {
                                            let a1592 = {
                                                let a1586 = 1u64;
                                                let a1587 = 1u64;
                                                nat_add(a1586, a1587)?
                                            };
                                            let a1593 = {
                                                let a1590 = {
                                                    let a1588 = 1u64;
                                                    let a1589 = 1u64;
                                                    nat_add(a1588, a1589)?
                                                };
                                                let a1591 = 1u64;
                                                nat_mul(a1590, a1591)?
                                            };
                                            nat_mul(a1592, a1593)?
                                        };
                                        let a1595 = 1u64;
                                        nat_add(a1594, a1595)?
                                    };
                                    nat_mul(a1596, a1597)?
                                };
                                let a1599 = 1u64;
                                nat_add(a1598, a1599)?
                            };
                            nat_mul(a1600, a1601)?
                        };
                        let a1603 = 1u64;
                        nat_add(a1602, a1603)?
                    })?;
                    nat_mul(a1604, a1605)?
                };
                nat_add(a1606, a1607)?
            };
            let a1634 = {
                let a1630 = v8;
                let a1631 = f14({
                    let a1628 = {
                        let a1626 = {
                            let a1608 = 1u64;
                            let a1609 = 1u64;
                            nat_add(a1608, a1609)?
                        };
                        let a1627 = {
                            let a1624 = {
                                let a1622 = {
                                    let a1610 = 1u64;
                                    let a1611 = 1u64;
                                    nat_add(a1610, a1611)?
                                };
                                let a1623 = {
                                    let a1620 = {
                                        let a1618 = {
                                            let a1612 = 1u64;
                                            let a1613 = 1u64;
                                            nat_add(a1612, a1613)?
                                        };
                                        let a1619 = {
                                            let a1616 = {
                                                let a1614 = 1u64;
                                                let a1615 = 1u64;
                                                nat_add(a1614, a1615)?
                                            };
                                            let a1617 = 1u64;
                                            nat_mul(a1616, a1617)?
                                        };
                                        nat_mul(a1618, a1619)?
                                    };
                                    let a1621 = 1u64;
                                    nat_add(a1620, a1621)?
                                };
                                nat_mul(a1622, a1623)?
                            };
                            let a1625 = 1u64;
                            nat_add(a1624, a1625)?
                        };
                        nat_mul(a1626, a1627)?
                    };
                    let a1629 = 1u64;
                    nat_add(a1628, a1629)?
                })?;
                let a1632 = 0u64;
                nat_rem(a1630, a1631, a1632)
            };
            nat_add(a1633, a1634)
        }
    }
}
