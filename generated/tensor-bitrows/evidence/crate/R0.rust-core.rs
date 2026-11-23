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
    C8(bool, u64, u64),
    C9(u64),
    C10(u64, u64, u64, u64),
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
    F30,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F30 => f30(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F35,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F35 => f35(p0),
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
        Adt0::C7(v41, v42) => {
            let a69 = v41;
            let a70 = 4294967296u64;
            if if nat_lt(a69, a70) {
                let a71 = v42;
                let a72 = 2139095040u64;
                nat_lt(a71, a72)
            } else {
                false
            } {
                Ok(Adt1::C2(f10(f5(v41, v42)?)?))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C8(v33, v34, v35) => {
            f8(v33, v34, v35)
        }
        Adt0::C9(v36) => {
            let a57 = v36;
            let a58 = 256u64;
            if nat_lt(a57, a58) {
                Ok(Adt1::C4({
                    let a62 = 18584u64;
                    let a63 = {
                        let a59 = v36;
                        let a60 = 16u64;
                        let a61 = 0u64;
                        nat_rem(a59, a60, a61)
                    };
                    nat_add(a62, a63)?
                }, {
                    let a67 = 18584u64;
                    let a68 = {
                        let a64 = v36;
                        let a65 = 16u64;
                        let a66 = 0u64;
                        nat_quot(a64, a65, a66)
                    };
                    nat_add(a67, a68)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C10(v37, v38, v39, v40) => {
            f9(v37, v38, v39, v40)
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a73 = 0u64;
    let a74 = v2;
    if if nat_lt(a73, a74) {
        let a75 = 0u64;
        let a76 = v3;
        nat_lt(a75, a76)
    } else {
        false
    } {
        Ok(Some({
            let a82 = {
                let a79 = {
                    let a77 = v0;
                    let a78 = v1;
                    nat_mul(a77, a78)?
                };
                let a80 = v2;
                let a81 = 0u64;
                nat_quot(a79, a80, a81)
            };
            let a83 = v3;
            let a84 = 0u64;
            nat_rem(a82, a83, a84)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a85 = v0;
    let a86 = 0u64;
    if nat_eq(a85, a86) {
        1u64
    } else {
        let a87 = v0;
        let a88 = 1u64;
        if nat_eq(a87, a88) {
            2u64
        } else {
            let a89 = v0;
            let a90 = 2u64;
            if nat_eq(a89, a90) {
                4u64
            } else {
                let a91 = v0;
                let a92 = 3u64;
                if nat_eq(a91, a92) {
                    8u64
                } else {
                    let a93 = v0;
                    let a94 = 4u64;
                    if nat_eq(a93, a94) {
                        16u64
                    } else {
                        let a95 = v0;
                        let a96 = 5u64;
                        if nat_eq(a95, a96) {
                            32u64
                        } else {
                            let a97 = v0;
                            let a98 = 6u64;
                            if nat_eq(a97, a98) {
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
    let a99 = v1;
    let a100 = 16u64;
    if if if nat_lt(a99, a100) {
        let a101 = v2;
        let a102 = 256u64;
        nat_lt(a101, a102)
    } else {
        false
    } {
        let a103 = v3;
        let a104 = 4294967296u64;
        nat_lt(a103, a104)
    } else {
        false
    } {
        let v4: u64 = {
            let a105 = v2;
            let a106 = 16u64;
            let a107 = 0u64;
            nat_rem(a105, a106, a107)
        };
        let v5: u64 = {
            let a108 = v2;
            let a109 = 16u64;
            let a110 = 0u64;
            nat_quot(a108, a109, a110)
        };
        if v0 {
            Ok(Adt1::C4({
                let a111 = 18568u64;
                let a112 = v4;
                nat_add(a111, a112)?
            }, {
                let a113 = 18568u64;
                let a114 = v5;
                nat_add(a113, a114)?
            }))
        } else {
            let v6: u64 = {
                let a120 = f2({
                    let a115 = v1;
                    let a116 = 8u64;
                    let a117 = 0u64;
                    nat_rem(a115, a116, a117)
                });
                let a121 = {
                    let a118 = v1;
                    let a119 = 8u64;
                    if nat_lt(a118, a119) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a120, a121)?
            };
            Ok(Adt1::C3(f11(v4, v3, v6)?, f11(v5, v3, {
                let a122 = v6;
                let a123 = 65536u64;
                nat_mul(a122, a123)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a124 = v0;
    let a125 = 16u64;
    if if if nat_lt(a124, a125) {
        let a126 = v1;
        let a127 = 256u64;
        nat_lt(a126, a127)
    } else {
        false
    } {
        let a128 = v2;
        let a129 = 4294967296u64;
        nat_lt(a128, a129)
    } else {
        false
    } {
        let v3: u64 = {
            let a135 = f2({
                let a130 = v0;
                let a131 = 8u64;
                let a132 = 0u64;
                nat_rem(a130, a131, a132)
            });
            let a136 = {
                let a133 = v0;
                let a134 = 8u64;
                if nat_lt(a133, a134) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a135, a136)?
        };
        Ok(Adt1::C3(f12({
            let a137 = v1;
            let a138 = 16u64;
            let a139 = 0u64;
            nat_rem(a137, a138, a139)
        }, v2, v3)?, f12({
            let a140 = v1;
            let a141 = 16u64;
            let a142 = 0u64;
            nat_quot(a140, a141, a142)
        }, v2, {
            let a143 = v3;
            let a144 = 65536u64;
            nat_mul(a143, a144)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f13(v0)? {
        true
    } else {
        f13(v1)?
    } {
        true
    } else {
        if f14(v0)? {
            f15(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f14(v1)? {
            f15(v0)?
        } else {
            false
        }
    } {
        let a231 = {
            let a209 = {
                let a185 = {
                    let a183 = {
                        let a145 = 1u64;
                        let a146 = 1u64;
                        nat_add(a145, a146)?
                    };
                    let a184 = {
                        let a181 = {
                            let a179 = {
                                let a147 = 1u64;
                                let a148 = 1u64;
                                nat_add(a147, a148)?
                            };
                            let a180 = {
                                let a177 = {
                                    let a175 = {
                                        let a149 = 1u64;
                                        let a150 = 1u64;
                                        nat_add(a149, a150)?
                                    };
                                    let a176 = {
                                        let a173 = {
                                            let a171 = {
                                                let a151 = 1u64;
                                                let a152 = 1u64;
                                                nat_add(a151, a152)?
                                            };
                                            let a172 = {
                                                let a169 = {
                                                    let a167 = {
                                                        let a153 = 1u64;
                                                        let a154 = 1u64;
                                                        nat_add(a153, a154)?
                                                    };
                                                    let a168 = {
                                                        let a165 = {
                                                            let a163 = {
                                                                let a155 = 1u64;
                                                                let a156 = 1u64;
                                                                nat_add(a155, a156)?
                                                            };
                                                            let a164 = {
                                                                let a161 = {
                                                                    let a159 = {
                                                                        let a157 = 1u64;
                                                                        let a158 = 1u64;
                                                                        nat_add(a157, a158)?
                                                                    };
                                                                    let a160 = 1u64;
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
                                    nat_mul(a175, a176)?
                                };
                                let a178 = 1u64;
                                nat_add(a177, a178)?
                            };
                            nat_mul(a179, a180)?
                        };
                        let a182 = 1u64;
                        nat_add(a181, a182)?
                    };
                    nat_mul(a183, a184)?
                };
                let a186 = 1u64;
                nat_add(a185, a186)?
            };
            let a210 = f16({
                let a207 = {
                    let a205 = {
                        let a187 = 1u64;
                        let a188 = 1u64;
                        nat_add(a187, a188)?
                    };
                    let a206 = {
                        let a203 = {
                            let a201 = {
                                let a189 = 1u64;
                                let a190 = 1u64;
                                nat_add(a189, a190)?
                            };
                            let a202 = {
                                let a199 = {
                                    let a197 = {
                                        let a191 = 1u64;
                                        let a192 = 1u64;
                                        nat_add(a191, a192)?
                                    };
                                    let a198 = {
                                        let a195 = {
                                            let a193 = 1u64;
                                            let a194 = 1u64;
                                            nat_add(a193, a194)?
                                        };
                                        let a196 = 1u64;
                                        nat_mul(a195, a196)?
                                    };
                                    nat_mul(a197, a198)?
                                };
                                let a200 = 1u64;
                                nat_add(a199, a200)?
                            };
                            nat_mul(a201, a202)?
                        };
                        let a204 = 1u64;
                        nat_add(a203, a204)?
                    };
                    nat_mul(a205, a206)?
                };
                let a208 = 1u64;
                nat_add(a207, a208)?
            })?;
            nat_mul(a209, a210)?
        };
        let a232 = f16({
            let a229 = {
                let a211 = 1u64;
                let a212 = 1u64;
                nat_add(a211, a212)?
            };
            let a230 = {
                let a227 = {
                    let a225 = {
                        let a213 = 1u64;
                        let a214 = 1u64;
                        nat_add(a213, a214)?
                    };
                    let a226 = {
                        let a223 = {
                            let a221 = {
                                let a215 = 1u64;
                                let a216 = 1u64;
                                nat_add(a215, a216)?
                            };
                            let a222 = {
                                let a219 = {
                                    let a217 = 1u64;
                                    let a218 = 1u64;
                                    nat_add(a217, a218)?
                                };
                                let a220 = 1u64;
                                nat_mul(a219, a220)?
                            };
                            nat_mul(a221, a222)?
                        };
                        let a224 = 1u64;
                        nat_add(a223, a224)?
                    };
                    nat_mul(a225, a226)?
                };
                let a228 = 1u64;
                nat_add(a227, a228)?
            };
            nat_mul(a229, a230)?
        })?;
        nat_add(a231, a232)
    } else {
        if if f14(v0)? {
            true
        } else {
            f14(v1)?
        } {
            f17(f18(v0, v1)?)
        } else {
            if if f15(v0)? {
                true
            } else {
                f15(v1)?
            } {
                f19(f18(v0, v1)?)
            } else {
                f20(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a236 = {
            let a233 = v0;
            let a234 = 32768u64;
            let a235 = 0u64;
            nat_quot(a233, a234, a235)
        };
        let a237 = 2147483648u64;
        nat_mul(a236, a237)?
    };
    let v2: u64 = {
        let a241 = {
            let a238 = v0;
            let a239 = 32768u64;
            let a240 = 0u64;
            nat_rem(a238, a239, a240)
        };
        let a242 = 1024u64;
        let a243 = 0u64;
        nat_quot(a241, a242, a243)
    };
    let v3: u64 = {
        let a244 = v0;
        let a245 = 1024u64;
        let a246 = 0u64;
        nat_rem(a244, a245, a246)
    };
    let a281 = v1;
    let a282 = {
        let a247 = v2;
        let a248 = 31u64;
        if nat_eq(a247, a248) {
            let a257 = 2139095040u64;
            let a258 = {
                let a255 = {
                    let a249 = 0u64;
                    let a250 = v3;
                    if if nat_lt(a249, a250) {
                        let a251 = v3;
                        let a252 = 512u64;
                        nat_lt(a251, a252)
                    } else {
                        false
                    } {
                        let a253 = v3;
                        let a254 = 512u64;
                        nat_add(a253, a254)?
                    } else {
                        v3
                    }
                };
                let a256 = 8192u64;
                nat_mul(a255, a256)?
            };
            nat_add(a257, a258)?
        } else {
            let a259 = v2;
            let a260 = 0u64;
            if nat_eq(a259, a260) {
                let a261 = v3;
                let a262 = 0u64;
                if nat_eq(a261, a262) {
                    0u64
                } else {
                    let v4: Adt2 = f21(113u64, v3, 0u64, 1024u64)?;
                    let a271 = {
                        let a264 = match v4.clone() {
                            Adt2::C0(h263, _, _, _) => {
                                h263
                            }
                        };
                        let a265 = 8388608u64;
                        nat_mul(a264, a265)?
                    };
                    let a272 = {
                        let a269 = {
                            let a267 = match v4 {
                                Adt2::C0(_, h266, _, _) => {
                                    h266
                                }
                            };
                            let a268 = 1024u64;
                            nat_sub(a267, a268)
                        };
                        let a270 = 8192u64;
                        nat_mul(a269, a270)?
                    };
                    nat_add(a271, a272)?
                }
            } else {
                let a279 = {
                    let a275 = {
                        let a273 = v2;
                        let a274 = 112u64;
                        nat_add(a273, a274)?
                    };
                    let a276 = 8388608u64;
                    nat_mul(a275, a276)?
                };
                let a280 = {
                    let a277 = v3;
                    let a278 = 8192u64;
                    nat_mul(a277, a278)?
                };
                nat_add(a279, a280)?
            }
        }
    };
    nat_add(a281, a282)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f13(v0)? {
        true
    } else {
        f13(v1)?
    } {
        let a369 = {
            let a347 = {
                let a323 = {
                    let a321 = {
                        let a283 = 1u64;
                        let a284 = 1u64;
                        nat_add(a283, a284)?
                    };
                    let a322 = {
                        let a319 = {
                            let a317 = {
                                let a285 = 1u64;
                                let a286 = 1u64;
                                nat_add(a285, a286)?
                            };
                            let a318 = {
                                let a315 = {
                                    let a313 = {
                                        let a287 = 1u64;
                                        let a288 = 1u64;
                                        nat_add(a287, a288)?
                                    };
                                    let a314 = {
                                        let a311 = {
                                            let a309 = {
                                                let a289 = 1u64;
                                                let a290 = 1u64;
                                                nat_add(a289, a290)?
                                            };
                                            let a310 = {
                                                let a307 = {
                                                    let a305 = {
                                                        let a291 = 1u64;
                                                        let a292 = 1u64;
                                                        nat_add(a291, a292)?
                                                    };
                                                    let a306 = {
                                                        let a303 = {
                                                            let a301 = {
                                                                let a293 = 1u64;
                                                                let a294 = 1u64;
                                                                nat_add(a293, a294)?
                                                            };
                                                            let a302 = {
                                                                let a299 = {
                                                                    let a297 = {
                                                                        let a295 = 1u64;
                                                                        let a296 = 1u64;
                                                                        nat_add(a295, a296)?
                                                                    };
                                                                    let a298 = 1u64;
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
                                    nat_mul(a313, a314)?
                                };
                                let a316 = 1u64;
                                nat_add(a315, a316)?
                            };
                            nat_mul(a317, a318)?
                        };
                        let a320 = 1u64;
                        nat_add(a319, a320)?
                    };
                    nat_mul(a321, a322)?
                };
                let a324 = 1u64;
                nat_add(a323, a324)?
            };
            let a348 = f16({
                let a345 = {
                    let a343 = {
                        let a325 = 1u64;
                        let a326 = 1u64;
                        nat_add(a325, a326)?
                    };
                    let a344 = {
                        let a341 = {
                            let a339 = {
                                let a327 = 1u64;
                                let a328 = 1u64;
                                nat_add(a327, a328)?
                            };
                            let a340 = {
                                let a337 = {
                                    let a335 = {
                                        let a329 = 1u64;
                                        let a330 = 1u64;
                                        nat_add(a329, a330)?
                                    };
                                    let a336 = {
                                        let a333 = {
                                            let a331 = 1u64;
                                            let a332 = 1u64;
                                            nat_add(a331, a332)?
                                        };
                                        let a334 = 1u64;
                                        nat_mul(a333, a334)?
                                    };
                                    nat_mul(a335, a336)?
                                };
                                let a338 = 1u64;
                                nat_add(a337, a338)?
                            };
                            nat_mul(a339, a340)?
                        };
                        let a342 = 1u64;
                        nat_add(a341, a342)?
                    };
                    nat_mul(a343, a344)?
                };
                let a346 = 1u64;
                nat_add(a345, a346)?
            })?;
            nat_mul(a347, a348)?
        };
        let a370 = f16({
            let a367 = {
                let a349 = 1u64;
                let a350 = 1u64;
                nat_add(a349, a350)?
            };
            let a368 = {
                let a365 = {
                    let a363 = {
                        let a351 = 1u64;
                        let a352 = 1u64;
                        nat_add(a351, a352)?
                    };
                    let a364 = {
                        let a361 = {
                            let a359 = {
                                let a353 = 1u64;
                                let a354 = 1u64;
                                nat_add(a353, a354)?
                            };
                            let a360 = {
                                let a357 = {
                                    let a355 = 1u64;
                                    let a356 = 1u64;
                                    nat_add(a355, a356)?
                                };
                                let a358 = 1u64;
                                nat_mul(a357, a358)?
                            };
                            nat_mul(a359, a360)?
                        };
                        let a362 = 1u64;
                        nat_add(a361, a362)?
                    };
                    nat_mul(a363, a364)?
                };
                let a366 = 1u64;
                nat_add(a365, a366)?
            };
            nat_mul(a367, a368)?
        })?;
        nat_add(a369, a370)
    } else {
        if if f14(v0)? {
            f14(v1)?
        } else {
            false
        } {
            let a371 = f22(v0)?;
            let a372 = f22(v1)?;
            if nat_eq(a371, a372) {
                Ok(v0)
            } else {
                let a459 = {
                    let a437 = {
                        let a413 = {
                            let a411 = {
                                let a373 = 1u64;
                                let a374 = 1u64;
                                nat_add(a373, a374)?
                            };
                            let a412 = {
                                let a409 = {
                                    let a407 = {
                                        let a375 = 1u64;
                                        let a376 = 1u64;
                                        nat_add(a375, a376)?
                                    };
                                    let a408 = {
                                        let a405 = {
                                            let a403 = {
                                                let a377 = 1u64;
                                                let a378 = 1u64;
                                                nat_add(a377, a378)?
                                            };
                                            let a404 = {
                                                let a401 = {
                                                    let a399 = {
                                                        let a379 = 1u64;
                                                        let a380 = 1u64;
                                                        nat_add(a379, a380)?
                                                    };
                                                    let a400 = {
                                                        let a397 = {
                                                            let a395 = {
                                                                let a381 = 1u64;
                                                                let a382 = 1u64;
                                                                nat_add(a381, a382)?
                                                            };
                                                            let a396 = {
                                                                let a393 = {
                                                                    let a391 = {
                                                                        let a383 = 1u64;
                                                                        let a384 = 1u64;
                                                                        nat_add(a383, a384)?
                                                                    };
                                                                    let a392 = {
                                                                        let a389 = {
                                                                            let a387 = {
                                                                                let a385 = 1u64;
                                                                                let a386 = 1u64;
                                                                                nat_add(a385, a386)?
                                                                            };
                                                                            let a388 = 1u64;
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
                                            nat_mul(a403, a404)?
                                        };
                                        let a406 = 1u64;
                                        nat_add(a405, a406)?
                                    };
                                    nat_mul(a407, a408)?
                                };
                                let a410 = 1u64;
                                nat_add(a409, a410)?
                            };
                            nat_mul(a411, a412)?
                        };
                        let a414 = 1u64;
                        nat_add(a413, a414)?
                    };
                    let a438 = f16({
                        let a435 = {
                            let a433 = {
                                let a415 = 1u64;
                                let a416 = 1u64;
                                nat_add(a415, a416)?
                            };
                            let a434 = {
                                let a431 = {
                                    let a429 = {
                                        let a417 = 1u64;
                                        let a418 = 1u64;
                                        nat_add(a417, a418)?
                                    };
                                    let a430 = {
                                        let a427 = {
                                            let a425 = {
                                                let a419 = 1u64;
                                                let a420 = 1u64;
                                                nat_add(a419, a420)?
                                            };
                                            let a426 = {
                                                let a423 = {
                                                    let a421 = 1u64;
                                                    let a422 = 1u64;
                                                    nat_add(a421, a422)?
                                                };
                                                let a424 = 1u64;
                                                nat_mul(a423, a424)?
                                            };
                                            nat_mul(a425, a426)?
                                        };
                                        let a428 = 1u64;
                                        nat_add(a427, a428)?
                                    };
                                    nat_mul(a429, a430)?
                                };
                                let a432 = 1u64;
                                nat_add(a431, a432)?
                            };
                            nat_mul(a433, a434)?
                        };
                        let a436 = 1u64;
                        nat_add(a435, a436)?
                    })?;
                    nat_mul(a437, a438)?
                };
                let a460 = f16({
                    let a457 = {
                        let a439 = 1u64;
                        let a440 = 1u64;
                        nat_add(a439, a440)?
                    };
                    let a458 = {
                        let a455 = {
                            let a453 = {
                                let a441 = 1u64;
                                let a442 = 1u64;
                                nat_add(a441, a442)?
                            };
                            let a454 = {
                                let a451 = {
                                    let a449 = {
                                        let a443 = 1u64;
                                        let a444 = 1u64;
                                        nat_add(a443, a444)?
                                    };
                                    let a450 = {
                                        let a447 = {
                                            let a445 = 1u64;
                                            let a446 = 1u64;
                                            nat_add(a445, a446)?
                                        };
                                        let a448 = 1u64;
                                        nat_mul(a447, a448)?
                                    };
                                    nat_mul(a449, a450)?
                                };
                                let a452 = 1u64;
                                nat_add(a451, a452)?
                            };
                            nat_mul(a453, a454)?
                        };
                        let a456 = 1u64;
                        nat_add(a455, a456)?
                    };
                    nat_mul(a457, a458)?
                })?;
                nat_add(a459, a460)
            }
        } else {
            if f14(v0)? {
                Ok(v0)
            } else {
                if f14(v1)? {
                    Ok(v1)
                } else {
                    let a461 = f23(v1)?;
                    let a462 = f23(v0)?;
                    if nat_le(a461, a462) {
                        f24(v0, v1)
                    } else {
                        f24(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: bool, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = if v0 {
        36u64
    } else {
        17u64
    };
    let v4: u64 = if v0 {
        2u64
    } else {
        1u64
    };
    let a466 = {
        let a463 = v1;
        let a464 = v3;
        let a465 = 0u64;
        nat_rem(a463, a464, a465)
    };
    let a467 = 0u64;
    if if nat_eq(a466, a467) {
        let a473 = {
            let a471 = {
                let a468 = v1;
                let a469 = v3;
                let a470 = 0u64;
                nat_quot(a468, a469, a470)
            };
            let a472 = v4;
            nat_mul(a471, a472)?
        };
        let a474 = v2;
        nat_eq(a473, a474)
    } else {
        false
    } {
        Ok(Adt1::C1(v3, if v0 {
            4u64
        } else {
            1u64
        }, if v0 {
            16u64
        } else {
            32u64
        }, v4))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let a475 = v0;
    let a476 = 4294967296u64;
    if if if if nat_lt(a475, a476) {
        let a477 = v1;
        let a478 = 4294967296u64;
        nat_lt(a477, a478)
    } else {
        false
    } {
        let a479 = v2;
        let a480 = 4294967296u64;
        nat_lt(a479, a480)
    } else {
        false
    } {
        let a481 = v3;
        let a482 = 65536u64;
        nat_lt(a481, a482)
    } else {
        false
    } {
        let v4: u64 = f5(f6(v3)?, v2)?;
        Ok(Adt1::C2(f7(v0, f5(v4, v1)?)?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a483 = f22(v0)?;
        let a484 = 32768u64;
        nat_mul(a483, a484)?
    };
    let v2: u64 = f25(v0)?;
    let a503 = v1;
    let a504 = {
        let a485 = v2;
        let a486 = 255u64;
        if nat_eq(a485, a486) {
            let a487 = f26(v0)?;
            let a488 = 0u64;
            if nat_eq(a487, a488) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a489 = 143u64;
            let a490 = v2;
            if nat_le(a489, a490) {
                31744u64
            } else {
                let a491 = v2;
                let a492 = 102u64;
                if nat_lt(a491, a492) {
                    0u64
                } else {
                    let a493 = 113u64;
                    let a494 = v2;
                    if nat_le(a493, a494) {
                        let a499 = {
                            let a497 = {
                                let a495 = v2;
                                let a496 = 113u64;
                                nat_sub(a495, a496)
                            };
                            let a498 = 1024u64;
                            nat_mul(a497, a498)?
                        };
                        let a500 = f27(f28(v0)?, 8192u64)?;
                        nat_add(a499, a500)?
                    } else {
                        f27(f28(v0)?, f16({
                            let a501 = 126u64;
                            let a502 = v2;
                            nat_sub(a501, a502)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a503, a504)
}

pub fn f11(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a511 = {
        let a508 = {
            let a505 = v1;
            let a506 = v2;
            let a507 = 0u64;
            nat_quot(a505, a506, a507)
        };
        let a509 = 2u64;
        let a510 = 0u64;
        nat_rem(a508, a509, a510)
    };
    let a512 = 1u64;
    if nat_eq(a511, a512) {
        Ok(v0)
    } else {
        let a513 = 240u64;
        let a514 = v0;
        nat_add(a513, a514)
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a521 = {
        let a518 = {
            let a515 = v1;
            let a516 = v2;
            let a517 = 0u64;
            nat_quot(a515, a516, a517)
        };
        let a519 = 2u64;
        let a520 = 0u64;
        nat_rem(a518, a519, a520)
    };
    let a522 = 1u64;
    if nat_eq(a521, a522) {
        let a523 = 16u64;
        let a524 = v0;
        nat_add(a523, a524)
    } else {
        Ok(v0)
    }
}

pub fn f13(v0: u64) -> R<bool> {
    let a567 = f25(v0)?;
    let a568 = {
        let a565 = {
            let a563 = {
                let a525 = 1u64;
                let a526 = 1u64;
                nat_add(a525, a526)?
            };
            let a564 = {
                let a561 = {
                    let a559 = {
                        let a527 = 1u64;
                        let a528 = 1u64;
                        nat_add(a527, a528)?
                    };
                    let a560 = {
                        let a557 = {
                            let a555 = {
                                let a529 = 1u64;
                                let a530 = 1u64;
                                nat_add(a529, a530)?
                            };
                            let a556 = {
                                let a553 = {
                                    let a551 = {
                                        let a531 = 1u64;
                                        let a532 = 1u64;
                                        nat_add(a531, a532)?
                                    };
                                    let a552 = {
                                        let a549 = {
                                            let a547 = {
                                                let a533 = 1u64;
                                                let a534 = 1u64;
                                                nat_add(a533, a534)?
                                            };
                                            let a548 = {
                                                let a545 = {
                                                    let a543 = {
                                                        let a535 = 1u64;
                                                        let a536 = 1u64;
                                                        nat_add(a535, a536)?
                                                    };
                                                    let a544 = {
                                                        let a541 = {
                                                            let a539 = {
                                                                let a537 = 1u64;
                                                                let a538 = 1u64;
                                                                nat_add(a537, a538)?
                                                            };
                                                            let a540 = 1u64;
                                                            nat_mul(a539, a540)?
                                                        };
                                                        let a542 = 1u64;
                                                        nat_add(a541, a542)?
                                                    };
                                                    nat_mul(a543, a544)?
                                                };
                                                let a546 = 1u64;
                                                nat_add(a545, a546)?
                                            };
                                            nat_mul(a547, a548)?
                                        };
                                        let a550 = 1u64;
                                        nat_add(a549, a550)?
                                    };
                                    nat_mul(a551, a552)?
                                };
                                let a554 = 1u64;
                                nat_add(a553, a554)?
                            };
                            nat_mul(a555, a556)?
                        };
                        let a558 = 1u64;
                        nat_add(a557, a558)?
                    };
                    nat_mul(a559, a560)?
                };
                let a562 = 1u64;
                nat_add(a561, a562)?
            };
            nat_mul(a563, a564)?
        };
        let a566 = 1u64;
        nat_add(a565, a566)?
    };
    if nat_eq(a567, a568) {
        let a569 = 0u64;
        let a570 = f26(v0)?;
        Ok(nat_lt(a569, a570))
    } else {
        Ok(false)
    }
}

pub fn f14(v0: u64) -> R<bool> {
    let a613 = f25(v0)?;
    let a614 = {
        let a611 = {
            let a609 = {
                let a571 = 1u64;
                let a572 = 1u64;
                nat_add(a571, a572)?
            };
            let a610 = {
                let a607 = {
                    let a605 = {
                        let a573 = 1u64;
                        let a574 = 1u64;
                        nat_add(a573, a574)?
                    };
                    let a606 = {
                        let a603 = {
                            let a601 = {
                                let a575 = 1u64;
                                let a576 = 1u64;
                                nat_add(a575, a576)?
                            };
                            let a602 = {
                                let a599 = {
                                    let a597 = {
                                        let a577 = 1u64;
                                        let a578 = 1u64;
                                        nat_add(a577, a578)?
                                    };
                                    let a598 = {
                                        let a595 = {
                                            let a593 = {
                                                let a579 = 1u64;
                                                let a580 = 1u64;
                                                nat_add(a579, a580)?
                                            };
                                            let a594 = {
                                                let a591 = {
                                                    let a589 = {
                                                        let a581 = 1u64;
                                                        let a582 = 1u64;
                                                        nat_add(a581, a582)?
                                                    };
                                                    let a590 = {
                                                        let a587 = {
                                                            let a585 = {
                                                                let a583 = 1u64;
                                                                let a584 = 1u64;
                                                                nat_add(a583, a584)?
                                                            };
                                                            let a586 = 1u64;
                                                            nat_mul(a585, a586)?
                                                        };
                                                        let a588 = 1u64;
                                                        nat_add(a587, a588)?
                                                    };
                                                    nat_mul(a589, a590)?
                                                };
                                                let a592 = 1u64;
                                                nat_add(a591, a592)?
                                            };
                                            nat_mul(a593, a594)?
                                        };
                                        let a596 = 1u64;
                                        nat_add(a595, a596)?
                                    };
                                    nat_mul(a597, a598)?
                                };
                                let a600 = 1u64;
                                nat_add(a599, a600)?
                            };
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
    if nat_eq(a613, a614) {
        let a615 = f26(v0)?;
        let a616 = 0u64;
        Ok(nat_eq(a615, a616))
    } else {
        Ok(false)
    }
}

pub fn f15(v0: u64) -> R<bool> {
    let a617 = f23(v0)?;
    let a618 = 0u64;
    Ok(nat_eq(a617, a618))
}

pub fn f16(v0: u64) -> R<u64> {
    f29(Fn0::F30, v0, 1u64)
}

pub fn f17(v0: u64) -> R<u64> {
    let a685 = f19(v0)?;
    let a686 = {
        let a683 = {
            let a659 = {
                let a657 = {
                    let a619 = 1u64;
                    let a620 = 1u64;
                    nat_add(a619, a620)?
                };
                let a658 = {
                    let a655 = {
                        let a653 = {
                            let a621 = 1u64;
                            let a622 = 1u64;
                            nat_add(a621, a622)?
                        };
                        let a654 = {
                            let a651 = {
                                let a649 = {
                                    let a623 = 1u64;
                                    let a624 = 1u64;
                                    nat_add(a623, a624)?
                                };
                                let a650 = {
                                    let a647 = {
                                        let a645 = {
                                            let a625 = 1u64;
                                            let a626 = 1u64;
                                            nat_add(a625, a626)?
                                        };
                                        let a646 = {
                                            let a643 = {
                                                let a641 = {
                                                    let a627 = 1u64;
                                                    let a628 = 1u64;
                                                    nat_add(a627, a628)?
                                                };
                                                let a642 = {
                                                    let a639 = {
                                                        let a637 = {
                                                            let a629 = 1u64;
                                                            let a630 = 1u64;
                                                            nat_add(a629, a630)?
                                                        };
                                                        let a638 = {
                                                            let a635 = {
                                                                let a633 = {
                                                                    let a631 = 1u64;
                                                                    let a632 = 1u64;
                                                                    nat_add(a631, a632)?
                                                                };
                                                                let a634 = 1u64;
                                                                nat_mul(a633, a634)?
                                                            };
                                                            let a636 = 1u64;
                                                            nat_add(a635, a636)?
                                                        };
                                                        nat_mul(a637, a638)?
                                                    };
                                                    let a640 = 1u64;
                                                    nat_add(a639, a640)?
                                                };
                                                nat_mul(a641, a642)?
                                            };
                                            let a644 = 1u64;
                                            nat_add(a643, a644)?
                                        };
                                        nat_mul(a645, a646)?
                                    };
                                    let a648 = 1u64;
                                    nat_add(a647, a648)?
                                };
                                nat_mul(a649, a650)?
                            };
                            let a652 = 1u64;
                            nat_add(a651, a652)?
                        };
                        nat_mul(a653, a654)?
                    };
                    let a656 = 1u64;
                    nat_add(a655, a656)?
                };
                nat_mul(a657, a658)?
            };
            let a660 = 1u64;
            nat_add(a659, a660)?
        };
        let a684 = f16({
            let a681 = {
                let a679 = {
                    let a661 = 1u64;
                    let a662 = 1u64;
                    nat_add(a661, a662)?
                };
                let a680 = {
                    let a677 = {
                        let a675 = {
                            let a663 = 1u64;
                            let a664 = 1u64;
                            nat_add(a663, a664)?
                        };
                        let a676 = {
                            let a673 = {
                                let a671 = {
                                    let a665 = 1u64;
                                    let a666 = 1u64;
                                    nat_add(a665, a666)?
                                };
                                let a672 = {
                                    let a669 = {
                                        let a667 = 1u64;
                                        let a668 = 1u64;
                                        nat_add(a667, a668)?
                                    };
                                    let a670 = 1u64;
                                    nat_mul(a669, a670)?
                                };
                                nat_mul(a671, a672)?
                            };
                            let a674 = 1u64;
                            nat_add(a673, a674)?
                        };
                        nat_mul(a675, a676)?
                    };
                    let a678 = 1u64;
                    nat_add(a677, a678)?
                };
                nat_mul(a679, a680)?
            };
            let a682 = 1u64;
            nat_add(a681, a682)?
        })?;
        nat_mul(a683, a684)?
    };
    nat_add(a685, a686)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let a693 = {
        let a687 = f22(v0)?;
        let a688 = f22(v1)?;
        nat_add(a687, a688)?
    };
    let a694 = {
        let a691 = {
            let a689 = 1u64;
            let a690 = 1u64;
            nat_add(a689, a690)?
        };
        let a692 = 1u64;
        nat_mul(a691, a692)?
    };
    let a695 = 0u64;
    Ok(nat_rem(a693, a694, a695))
}

pub fn f19(v0: u64) -> R<u64> {
    let a720 = v0;
    let a721 = f16({
        let a718 = {
            let a716 = {
                let a696 = 1u64;
                let a697 = 1u64;
                nat_add(a696, a697)?
            };
            let a717 = {
                let a714 = {
                    let a712 = {
                        let a698 = 1u64;
                        let a699 = 1u64;
                        nat_add(a698, a699)?
                    };
                    let a713 = {
                        let a710 = {
                            let a708 = {
                                let a700 = 1u64;
                                let a701 = 1u64;
                                nat_add(a700, a701)?
                            };
                            let a709 = {
                                let a706 = {
                                    let a704 = {
                                        let a702 = 1u64;
                                        let a703 = 1u64;
                                        nat_add(a702, a703)?
                                    };
                                    let a705 = 1u64;
                                    nat_mul(a704, a705)?
                                };
                                let a707 = 1u64;
                                nat_add(a706, a707)?
                            };
                            nat_mul(a708, a709)?
                        };
                        let a711 = 1u64;
                        nat_add(a710, a711)?
                    };
                    nat_mul(a712, a713)?
                };
                let a715 = 1u64;
                nat_add(a714, a715)?
            };
            nat_mul(a716, a717)?
        };
        let a719 = 1u64;
        nat_add(a718, a719)?
    })?;
    nat_mul(a720, a721)
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f31(v0)?;
    let v3: Adt2 = f31(v1)?;
    f32(f18(v0, v1)?, {
        let a776 = {
            let a724 = match v2.clone() {
                Adt2::C0(h722, _, _, _) => {
                    h722
                }
            };
            let a725 = match v3.clone() {
                Adt2::C0(h723, _, _, _) => {
                    h723
                }
            };
            nat_add(a724, a725)?
        };
        let a777 = {
            let a774 = {
                let a772 = {
                    let a726 = 1u64;
                    let a727 = 1u64;
                    nat_add(a726, a727)?
                };
                let a773 = {
                    let a770 = {
                        let a768 = {
                            let a728 = 1u64;
                            let a729 = 1u64;
                            nat_add(a728, a729)?
                        };
                        let a769 = {
                            let a766 = {
                                let a764 = {
                                    let a730 = 1u64;
                                    let a731 = 1u64;
                                    nat_add(a730, a731)?
                                };
                                let a765 = {
                                    let a762 = {
                                        let a760 = {
                                            let a732 = 1u64;
                                            let a733 = 1u64;
                                            nat_add(a732, a733)?
                                        };
                                        let a761 = {
                                            let a758 = {
                                                let a756 = {
                                                    let a734 = 1u64;
                                                    let a735 = 1u64;
                                                    nat_add(a734, a735)?
                                                };
                                                let a757 = {
                                                    let a754 = {
                                                        let a752 = {
                                                            let a736 = 1u64;
                                                            let a737 = 1u64;
                                                            nat_add(a736, a737)?
                                                        };
                                                        let a753 = {
                                                            let a750 = {
                                                                let a748 = {
                                                                    let a738 = 1u64;
                                                                    let a739 = 1u64;
                                                                    nat_add(a738, a739)?
                                                                };
                                                                let a749 = {
                                                                    let a746 = {
                                                                        let a740 = 1u64;
                                                                        let a741 = 1u64;
                                                                        nat_add(a740, a741)?
                                                                    };
                                                                    let a747 = {
                                                                        let a744 = {
                                                                            let a742 = 1u64;
                                                                            let a743 = 1u64;
                                                                            nat_add(a742, a743)?
                                                                        };
                                                                        let a745 = 1u64;
                                                                        nat_mul(a744, a745)?
                                                                    };
                                                                    nat_mul(a746, a747)?
                                                                };
                                                                nat_mul(a748, a749)?
                                                            };
                                                            let a751 = 1u64;
                                                            nat_add(a750, a751)?
                                                        };
                                                        nat_mul(a752, a753)?
                                                    };
                                                    let a755 = 1u64;
                                                    nat_add(a754, a755)?
                                                };
                                                nat_mul(a756, a757)?
                                            };
                                            let a759 = 1u64;
                                            nat_add(a758, a759)?
                                        };
                                        nat_mul(a760, a761)?
                                    };
                                    let a763 = 1u64;
                                    nat_add(a762, a763)?
                                };
                                nat_mul(a764, a765)?
                            };
                            let a767 = 1u64;
                            nat_add(a766, a767)?
                        };
                        nat_mul(a768, a769)?
                    };
                    let a771 = 1u64;
                    nat_add(a770, a771)?
                };
                nat_mul(a772, a773)?
            };
            let a775 = 1u64;
            nat_add(a774, a775)?
        };
        nat_sub(a776, a777)
    }, f33({
        let a780 = match v2 {
            Adt2::C0(_, h778, _, _) => {
                h778
            }
        };
        let a781 = match v3 {
            Adt2::C0(_, h779, _, _) => {
                h779
            }
        };
        nat_mul(a780, a781)?
    }, {
        let a798 = {
            let a782 = 1u64;
            let a783 = 1u64;
            nat_add(a782, a783)?
        };
        let a799 = {
            let a796 = {
                let a784 = 1u64;
                let a785 = 1u64;
                nat_add(a784, a785)?
            };
            let a797 = {
                let a794 = {
                    let a792 = {
                        let a786 = 1u64;
                        let a787 = 1u64;
                        nat_add(a786, a787)?
                    };
                    let a793 = {
                        let a790 = {
                            let a788 = 1u64;
                            let a789 = 1u64;
                            nat_add(a788, a789)?
                        };
                        let a791 = 1u64;
                        nat_mul(a790, a791)?
                    };
                    nat_mul(a792, a793)?
                };
                let a795 = 1u64;
                nat_add(a794, a795)?
            };
            nat_mul(a796, a797)?
        };
        nat_mul(a798, a799)?
    })?)
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h800, _) = f34(Fn1::F35, {
        let a819 = {
            let a801 = 1u64;
            let a802 = 1u64;
            nat_add(a801, a802)?
        };
        let a820 = {
            let a817 = {
                let a803 = 1u64;
                let a804 = 1u64;
                nat_add(a803, a804)?
            };
            let a818 = {
                let a815 = {
                    let a805 = 1u64;
                    let a806 = 1u64;
                    nat_add(a805, a806)?
                };
                let a816 = {
                    let a813 = {
                        let a807 = 1u64;
                        let a808 = 1u64;
                        nat_add(a807, a808)?
                    };
                    let a814 = {
                        let a811 = {
                            let a809 = 1u64;
                            let a810 = 1u64;
                            nat_add(a809, a810)?
                        };
                        let a812 = 1u64;
                        nat_mul(a811, a812)?
                    };
                    nat_mul(a813, a814)?
                };
                nat_mul(a815, a816)?
            };
            nat_mul(a817, a818)?
        };
        nat_mul(a819, a820)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h800)
}

pub fn f22(v0: u64) -> R<u64> {
    let a845 = v0;
    let a846 = f16({
        let a843 = {
            let a841 = {
                let a821 = 1u64;
                let a822 = 1u64;
                nat_add(a821, a822)?
            };
            let a842 = {
                let a839 = {
                    let a837 = {
                        let a823 = 1u64;
                        let a824 = 1u64;
                        nat_add(a823, a824)?
                    };
                    let a838 = {
                        let a835 = {
                            let a833 = {
                                let a825 = 1u64;
                                let a826 = 1u64;
                                nat_add(a825, a826)?
                            };
                            let a834 = {
                                let a831 = {
                                    let a829 = {
                                        let a827 = 1u64;
                                        let a828 = 1u64;
                                        nat_add(a827, a828)?
                                    };
                                    let a830 = 1u64;
                                    nat_mul(a829, a830)?
                                };
                                let a832 = 1u64;
                                nat_add(a831, a832)?
                            };
                            nat_mul(a833, a834)?
                        };
                        let a836 = 1u64;
                        nat_add(a835, a836)?
                    };
                    nat_mul(a837, a838)?
                };
                let a840 = 1u64;
                nat_add(a839, a840)?
            };
            nat_mul(a841, a842)?
        };
        let a844 = 1u64;
        nat_add(a843, a844)?
    })?;
    let a847 = 0u64;
    Ok(nat_quot(a845, a846, a847))
}

pub fn f23(v0: u64) -> R<u64> {
    let a872 = v0;
    let a873 = f16({
        let a870 = {
            let a868 = {
                let a848 = 1u64;
                let a849 = 1u64;
                nat_add(a848, a849)?
            };
            let a869 = {
                let a866 = {
                    let a864 = {
                        let a850 = 1u64;
                        let a851 = 1u64;
                        nat_add(a850, a851)?
                    };
                    let a865 = {
                        let a862 = {
                            let a860 = {
                                let a852 = 1u64;
                                let a853 = 1u64;
                                nat_add(a852, a853)?
                            };
                            let a861 = {
                                let a858 = {
                                    let a856 = {
                                        let a854 = 1u64;
                                        let a855 = 1u64;
                                        nat_add(a854, a855)?
                                    };
                                    let a857 = 1u64;
                                    nat_mul(a856, a857)?
                                };
                                let a859 = 1u64;
                                nat_add(a858, a859)?
                            };
                            nat_mul(a860, a861)?
                        };
                        let a863 = 1u64;
                        nat_add(a862, a863)?
                    };
                    nat_mul(a864, a865)?
                };
                let a867 = 1u64;
                nat_add(a866, a867)?
            };
            nat_mul(a868, a869)?
        };
        let a871 = 1u64;
        nat_add(a870, a871)?
    })?;
    let a874 = 0u64;
    Ok(nat_rem(a872, a873, a874))
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f36(v0)?;
    let v3: u64 = {
        let a887 = f28(v0)?;
        let a888 = {
            let a885 = {
                let a875 = 1u64;
                let a876 = 1u64;
                nat_add(a875, a876)?
            };
            let a886 = {
                let a883 = {
                    let a877 = 1u64;
                    let a878 = 1u64;
                    nat_add(a877, a878)?
                };
                let a884 = {
                    let a881 = {
                        let a879 = 1u64;
                        let a880 = 1u64;
                        nat_add(a879, a880)?
                    };
                    let a882 = 1u64;
                    nat_mul(a881, a882)?
                };
                nat_mul(a883, a884)?
            };
            nat_mul(a885, a886)?
        };
        nat_mul(a887, a888)?
    };
    let v4: u64 = f33({
        let a901 = f28(v1)?;
        let a902 = {
            let a899 = {
                let a889 = 1u64;
                let a890 = 1u64;
                nat_add(a889, a890)?
            };
            let a900 = {
                let a897 = {
                    let a891 = 1u64;
                    let a892 = 1u64;
                    nat_add(a891, a892)?
                };
                let a898 = {
                    let a895 = {
                        let a893 = 1u64;
                        let a894 = 1u64;
                        nat_add(a893, a894)?
                    };
                    let a896 = 1u64;
                    nat_mul(a895, a896)?
                };
                nat_mul(a897, a898)?
            };
            nat_mul(a899, a900)?
        };
        nat_mul(a901, a902)?
    }, {
        let a903 = v2;
        let a904 = f36(v1)?;
        nat_sub(a903, a904)
    })?;
    let a905 = f22(v0)?;
    let a906 = f22(v1)?;
    if nat_eq(a905, a906) {
        f32(f22(v0)?, v2, {
            let a907 = v3;
            let a908 = v4;
            nat_add(a907, a908)?
        })
    } else {
        let a909 = v3;
        let a910 = v4;
        if nat_eq(a909, a910) {
            Ok(0u64)
        } else {
            f32(f22(v0)?, v2, {
                let a911 = v3;
                let a912 = v4;
                nat_sub(a911, a912)
            })
        }
    }
}

pub fn f25(v0: u64) -> R<u64> {
    let a935 = f23(v0)?;
    let a936 = f16({
        let a933 = {
            let a931 = {
                let a913 = 1u64;
                let a914 = 1u64;
                nat_add(a913, a914)?
            };
            let a932 = {
                let a929 = {
                    let a927 = {
                        let a915 = 1u64;
                        let a916 = 1u64;
                        nat_add(a915, a916)?
                    };
                    let a928 = {
                        let a925 = {
                            let a923 = {
                                let a917 = 1u64;
                                let a918 = 1u64;
                                nat_add(a917, a918)?
                            };
                            let a924 = {
                                let a921 = {
                                    let a919 = 1u64;
                                    let a920 = 1u64;
                                    nat_add(a919, a920)?
                                };
                                let a922 = 1u64;
                                nat_mul(a921, a922)?
                            };
                            nat_mul(a923, a924)?
                        };
                        let a926 = 1u64;
                        nat_add(a925, a926)?
                    };
                    nat_mul(a927, a928)?
                };
                let a930 = 1u64;
                nat_add(a929, a930)?
            };
            nat_mul(a931, a932)?
        };
        let a934 = 1u64;
        nat_add(a933, a934)?
    })?;
    let a937 = 0u64;
    Ok(nat_quot(a935, a936, a937))
}

pub fn f26(v0: u64) -> R<u64> {
    let a960 = v0;
    let a961 = f16({
        let a958 = {
            let a956 = {
                let a938 = 1u64;
                let a939 = 1u64;
                nat_add(a938, a939)?
            };
            let a957 = {
                let a954 = {
                    let a952 = {
                        let a940 = 1u64;
                        let a941 = 1u64;
                        nat_add(a940, a941)?
                    };
                    let a953 = {
                        let a950 = {
                            let a948 = {
                                let a942 = 1u64;
                                let a943 = 1u64;
                                nat_add(a942, a943)?
                            };
                            let a949 = {
                                let a946 = {
                                    let a944 = 1u64;
                                    let a945 = 1u64;
                                    nat_add(a944, a945)?
                                };
                                let a947 = 1u64;
                                nat_mul(a946, a947)?
                            };
                            nat_mul(a948, a949)?
                        };
                        let a951 = 1u64;
                        nat_add(a950, a951)?
                    };
                    nat_mul(a952, a953)?
                };
                let a955 = 1u64;
                nat_add(a954, a955)?
            };
            nat_mul(a956, a957)?
        };
        let a959 = 1u64;
        nat_add(a958, a959)?
    })?;
    let a962 = 0u64;
    Ok(nat_rem(a960, a961, a962))
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a963 = v0;
        let a964 = v1;
        let a965 = 0u64;
        nat_quot(a963, a964, a965)
    };
    let v3: u64 = {
        let a969 = {
            let a966 = v0;
            let a967 = v1;
            let a968 = 0u64;
            nat_rem(a966, a967, a968)
        };
        let a970 = 2u64;
        nat_mul(a969, a970)?
    };
    let a980 = v2;
    let a981 = {
        let a971 = v1;
        let a972 = v3;
        if if nat_lt(a971, a972) {
            true
        } else {
            let a973 = v3;
            let a974 = v1;
            if nat_eq(a973, a974) {
                let a978 = {
                    let a975 = v2;
                    let a976 = 2u64;
                    let a977 = 0u64;
                    nat_rem(a975, a976, a977)
                };
                let a979 = 1u64;
                nat_eq(a978, a979)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a980, a981)
}

pub fn f28(v0: u64) -> R<u64> {
    let a1006 = f26(v0)?;
    let a1007 = {
        let a982 = f25(v0)?;
        let a983 = 0u64;
        if nat_eq(a982, a983) {
            0u64
        } else {
            f16({
                let a1004 = {
                    let a1002 = {
                        let a984 = 1u64;
                        let a985 = 1u64;
                        nat_add(a984, a985)?
                    };
                    let a1003 = {
                        let a1000 = {
                            let a998 = {
                                let a986 = 1u64;
                                let a987 = 1u64;
                                nat_add(a986, a987)?
                            };
                            let a999 = {
                                let a996 = {
                                    let a994 = {
                                        let a988 = 1u64;
                                        let a989 = 1u64;
                                        nat_add(a988, a989)?
                                    };
                                    let a995 = {
                                        let a992 = {
                                            let a990 = 1u64;
                                            let a991 = 1u64;
                                            nat_add(a990, a991)?
                                        };
                                        let a993 = 1u64;
                                        nat_mul(a992, a993)?
                                    };
                                    nat_mul(a994, a995)?
                                };
                                let a997 = 1u64;
                                nat_add(a996, a997)?
                            };
                            nat_mul(a998, a999)?
                        };
                        let a1001 = 1u64;
                        nat_add(a1000, a1001)?
                    };
                    nat_mul(a1002, a1003)?
                };
                let a1005 = 1u64;
                nat_add(a1004, a1005)?
            })?
        }
    };
    nat_add(a1006, a1007)
}

pub fn f29(v0: Fn0, p1011: u64, p1012: u64) -> R<u64> {
    let m1010 = (p1011, p1012);
    {
        let mut m1010 = m1010;
        loop {
            let step = {
                let (v1, v2) = m1010;
                let m1008 = v1;
                if m1008 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1008 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1009: Fn0 = v0.clone();
                        c1009.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: u64) -> R<u64> {
    let a1013 = v0;
    let a1014 = v0;
    nat_add(a1013, a1014)
}

pub fn f31(v0: u64) -> R<Adt2> {
    f21(f36(v0)?, f28(v0)?, 0u64, f16({
        let a1035 = {
            let a1033 = {
                let a1015 = 1u64;
                let a1016 = 1u64;
                nat_add(a1015, a1016)?
            };
            let a1034 = {
                let a1031 = {
                    let a1029 = {
                        let a1017 = 1u64;
                        let a1018 = 1u64;
                        nat_add(a1017, a1018)?
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1025 = {
                                let a1019 = 1u64;
                                let a1020 = 1u64;
                                nat_add(a1019, a1020)?
                            };
                            let a1026 = {
                                let a1023 = {
                                    let a1021 = 1u64;
                                    let a1022 = 1u64;
                                    nat_add(a1021, a1022)?
                                };
                                let a1024 = 1u64;
                                nat_mul(a1023, a1024)?
                            };
                            nat_mul(a1025, a1026)?
                        };
                        let a1028 = 1u64;
                        nat_add(a1027, a1028)?
                    };
                    nat_mul(a1029, a1030)?
                };
                let a1032 = 1u64;
                nat_add(a1031, a1032)?
            };
            nat_mul(a1033, a1034)?
        };
        let a1036 = 1u64;
        nat_add(a1035, a1036)?
    })?)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1059 = f16({
        let a1057 = {
            let a1055 = {
                let a1037 = 1u64;
                let a1038 = 1u64;
                nat_add(a1037, a1038)?
            };
            let a1056 = {
                let a1053 = {
                    let a1051 = {
                        let a1039 = 1u64;
                        let a1040 = 1u64;
                        nat_add(a1039, a1040)?
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1041 = 1u64;
                            let a1042 = 1u64;
                            nat_add(a1041, a1042)?
                        };
                        let a1050 = {
                            let a1047 = {
                                let a1045 = {
                                    let a1043 = 1u64;
                                    let a1044 = 1u64;
                                    nat_add(a1043, a1044)?
                                };
                                let a1046 = 1u64;
                                nat_mul(a1045, a1046)?
                            };
                            let a1048 = 1u64;
                            nat_add(a1047, a1048)?
                        };
                        nat_mul(a1049, a1050)?
                    };
                    nat_mul(a1051, a1052)?
                };
                let a1054 = 1u64;
                nat_add(a1053, a1054)?
            };
            nat_mul(a1055, a1056)?
        };
        let a1058 = 1u64;
        nat_add(a1057, a1058)?
    })?;
    let a1060 = v2;
    if nat_le(a1059, a1060) {
        f37(v0, {
            let a1061 = v1;
            let a1062 = 1u64;
            nat_add(a1061, a1062)?
        }, f33(v2, 1u64)?)
    } else {
        let v3: Adt2 = f21(v1, v2, {
            let a1099 = {
                let a1097 = {
                    let a1063 = 1u64;
                    let a1064 = 1u64;
                    nat_add(a1063, a1064)?
                };
                let a1098 = {
                    let a1095 = {
                        let a1065 = 1u64;
                        let a1066 = 1u64;
                        nat_add(a1065, a1066)?
                    };
                    let a1096 = {
                        let a1093 = {
                            let a1067 = 1u64;
                            let a1068 = 1u64;
                            nat_add(a1067, a1068)?
                        };
                        let a1094 = {
                            let a1091 = {
                                let a1069 = 1u64;
                                let a1070 = 1u64;
                                nat_add(a1069, a1070)?
                            };
                            let a1092 = {
                                let a1089 = {
                                    let a1071 = 1u64;
                                    let a1072 = 1u64;
                                    nat_add(a1071, a1072)?
                                };
                                let a1090 = {
                                    let a1087 = {
                                        let a1073 = 1u64;
                                        let a1074 = 1u64;
                                        nat_add(a1073, a1074)?
                                    };
                                    let a1088 = {
                                        let a1085 = {
                                            let a1075 = 1u64;
                                            let a1076 = 1u64;
                                            nat_add(a1075, a1076)?
                                        };
                                        let a1086 = {
                                            let a1083 = {
                                                let a1077 = 1u64;
                                                let a1078 = 1u64;
                                                nat_add(a1077, a1078)?
                                            };
                                            let a1084 = {
                                                let a1081 = {
                                                    let a1079 = 1u64;
                                                    let a1080 = 1u64;
                                                    nat_add(a1079, a1080)?
                                                };
                                                let a1082 = 1u64;
                                                nat_mul(a1081, a1082)?
                                            };
                                            nat_mul(a1083, a1084)?
                                        };
                                        nat_mul(a1085, a1086)?
                                    };
                                    nat_mul(a1087, a1088)?
                                };
                                nat_mul(a1089, a1090)?
                            };
                            nat_mul(a1091, a1092)?
                        };
                        nat_mul(a1093, a1094)?
                    };
                    nat_mul(a1095, a1096)?
                };
                nat_mul(a1097, a1098)?
            };
            let a1100 = 1u64;
            nat_add(a1099, a1100)?
        }, f16({
            let a1119 = {
                let a1101 = 1u64;
                let a1102 = 1u64;
                nat_add(a1101, a1102)?
            };
            let a1120 = {
                let a1117 = {
                    let a1115 = {
                        let a1103 = 1u64;
                        let a1104 = 1u64;
                        nat_add(a1103, a1104)?
                    };
                    let a1116 = {
                        let a1113 = {
                            let a1105 = 1u64;
                            let a1106 = 1u64;
                            nat_add(a1105, a1106)?
                        };
                        let a1114 = {
                            let a1111 = {
                                let a1109 = {
                                    let a1107 = 1u64;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                };
                                let a1110 = 1u64;
                                nat_mul(a1109, a1110)?
                            };
                            let a1112 = 1u64;
                            nat_add(a1111, a1112)?
                        };
                        nat_mul(a1113, a1114)?
                    };
                    nat_mul(a1115, a1116)?
                };
                let a1118 = 1u64;
                nat_add(a1117, a1118)?
            };
            nat_mul(a1119, a1120)?
        })?)?;
        f37(v0, match v3.clone() {
            Adt2::C0(h1121, _, _, _) => {
                h1121
            }
        }, match v3 {
            Adt2::C0(_, h1122, _, _) => {
                h1122
            }
        })
    }
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let a1123 = v1;
    let a1124 = 0u64;
    if nat_eq(a1123, a1124) {
        Ok(v0)
    } else {
        let a1149 = {
            let a1147 = {
                let a1125 = 1u64;
                let a1126 = 1u64;
                nat_add(a1125, a1126)?
            };
            let a1148 = {
                let a1145 = {
                    let a1127 = 1u64;
                    let a1128 = 1u64;
                    nat_add(a1127, a1128)?
                };
                let a1146 = {
                    let a1143 = {
                        let a1129 = 1u64;
                        let a1130 = 1u64;
                        nat_add(a1129, a1130)?
                    };
                    let a1144 = {
                        let a1141 = {
                            let a1131 = 1u64;
                            let a1132 = 1u64;
                            nat_add(a1131, a1132)?
                        };
                        let a1142 = {
                            let a1139 = {
                                let a1133 = 1u64;
                                let a1134 = 1u64;
                                nat_add(a1133, a1134)?
                            };
                            let a1140 = {
                                let a1137 = {
                                    let a1135 = 1u64;
                                    let a1136 = 1u64;
                                    nat_add(a1135, a1136)?
                                };
                                let a1138 = 1u64;
                                nat_mul(a1137, a1138)?
                            };
                            nat_mul(a1139, a1140)?
                        };
                        nat_mul(a1141, a1142)?
                    };
                    nat_mul(a1143, a1144)?
                };
                nat_mul(a1145, a1146)?
            };
            nat_mul(a1147, a1148)?
        };
        let a1150 = v1;
        if nat_le(a1149, a1150) {
            let a1151 = v0;
            let a1152 = 0u64;
            if nat_eq(a1151, a1152) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f16(v1)?;
            f38({
                let a1153 = v0;
                let a1154 = v2;
                let a1155 = 0u64;
                nat_quot(a1153, a1154, a1155)
            }, {
                let a1159 = 0u64;
                let a1160 = {
                    let a1156 = v0;
                    let a1157 = v2;
                    let a1158 = 0u64;
                    nat_rem(a1156, a1157, a1158)
                };
                nat_lt(a1159, a1160)
            })
        }
    }
}

pub fn f34(v0: Fn1, p1165: u64, p1166: Adt2) -> R<(Adt2, bool)> {
    let m1164 = (p1165, p1166);
    {
        let mut m1164 = m1164;
        loop {
            let step = {
                let (v1, v2) = m1164;
                let m1161 = v1;
                if m1161 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1161 - 1;
                    let m1163 = {
                        let c1162: Fn1 = v0.clone();
                        c1162.apply(v2.clone())?
                    };
                    match m1163 {
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
                Ok(next) => { m1164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Adt2) -> R<Option<Adt2>> {
    f39(v0)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1205 = {
        let a1167 = f25(v0)?;
        let a1168 = 0u64;
        if nat_eq(a1167, a1168) {
            1u64
        } else {
            f25(v0)?
        }
    };
    let a1206 = {
        let a1203 = {
            let a1169 = 1u64;
            let a1170 = 1u64;
            nat_add(a1169, a1170)?
        };
        let a1204 = {
            let a1201 = {
                let a1171 = 1u64;
                let a1172 = 1u64;
                nat_add(a1171, a1172)?
            };
            let a1202 = {
                let a1199 = {
                    let a1173 = 1u64;
                    let a1174 = 1u64;
                    nat_add(a1173, a1174)?
                };
                let a1200 = {
                    let a1197 = {
                        let a1175 = 1u64;
                        let a1176 = 1u64;
                        nat_add(a1175, a1176)?
                    };
                    let a1198 = {
                        let a1195 = {
                            let a1177 = 1u64;
                            let a1178 = 1u64;
                            nat_add(a1177, a1178)?
                        };
                        let a1196 = {
                            let a1193 = {
                                let a1179 = 1u64;
                                let a1180 = 1u64;
                                nat_add(a1179, a1180)?
                            };
                            let a1194 = {
                                let a1191 = {
                                    let a1181 = 1u64;
                                    let a1182 = 1u64;
                                    nat_add(a1181, a1182)?
                                };
                                let a1192 = {
                                    let a1189 = {
                                        let a1183 = 1u64;
                                        let a1184 = 1u64;
                                        nat_add(a1183, a1184)?
                                    };
                                    let a1190 = {
                                        let a1187 = {
                                            let a1185 = 1u64;
                                            let a1186 = 1u64;
                                            nat_add(a1185, a1186)?
                                        };
                                        let a1188 = 1u64;
                                        nat_mul(a1187, a1188)?
                                    };
                                    nat_mul(a1189, a1190)?
                                };
                                nat_mul(a1191, a1192)?
                            };
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
    nat_add(a1205, a1206)
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1245 = v1;
    let a1246 = {
        let a1243 = {
            let a1241 = {
                let a1207 = 1u64;
                let a1208 = 1u64;
                nat_add(a1207, a1208)?
            };
            let a1242 = {
                let a1239 = {
                    let a1209 = 1u64;
                    let a1210 = 1u64;
                    nat_add(a1209, a1210)?
                };
                let a1240 = {
                    let a1237 = {
                        let a1211 = 1u64;
                        let a1212 = 1u64;
                        nat_add(a1211, a1212)?
                    };
                    let a1238 = {
                        let a1235 = {
                            let a1213 = 1u64;
                            let a1214 = 1u64;
                            nat_add(a1213, a1214)?
                        };
                        let a1236 = {
                            let a1233 = {
                                let a1215 = 1u64;
                                let a1216 = 1u64;
                                nat_add(a1215, a1216)?
                            };
                            let a1234 = {
                                let a1231 = {
                                    let a1217 = 1u64;
                                    let a1218 = 1u64;
                                    nat_add(a1217, a1218)?
                                };
                                let a1232 = {
                                    let a1229 = {
                                        let a1219 = 1u64;
                                        let a1220 = 1u64;
                                        nat_add(a1219, a1220)?
                                    };
                                    let a1230 = {
                                        let a1227 = {
                                            let a1221 = 1u64;
                                            let a1222 = 1u64;
                                            nat_add(a1221, a1222)?
                                        };
                                        let a1228 = {
                                            let a1225 = {
                                                let a1223 = 1u64;
                                                let a1224 = 1u64;
                                                nat_add(a1223, a1224)?
                                            };
                                            let a1226 = 1u64;
                                            nat_mul(a1225, a1226)?
                                        };
                                        nat_mul(a1227, a1228)?
                                    };
                                    nat_mul(a1229, a1230)?
                                };
                                nat_mul(a1231, a1232)?
                            };
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
        let a1244 = 1u64;
        nat_add(a1243, a1244)?
    };
    if nat_lt(a1245, a1246) {
        f40(v0, {
            let a1283 = {
                let a1281 = {
                    let a1247 = 1u64;
                    let a1248 = 1u64;
                    nat_add(a1247, a1248)?
                };
                let a1282 = {
                    let a1279 = {
                        let a1249 = 1u64;
                        let a1250 = 1u64;
                        nat_add(a1249, a1250)?
                    };
                    let a1280 = {
                        let a1277 = {
                            let a1251 = 1u64;
                            let a1252 = 1u64;
                            nat_add(a1251, a1252)?
                        };
                        let a1278 = {
                            let a1275 = {
                                let a1253 = 1u64;
                                let a1254 = 1u64;
                                nat_add(a1253, a1254)?
                            };
                            let a1276 = {
                                let a1273 = {
                                    let a1255 = 1u64;
                                    let a1256 = 1u64;
                                    nat_add(a1255, a1256)?
                                };
                                let a1274 = {
                                    let a1271 = {
                                        let a1257 = 1u64;
                                        let a1258 = 1u64;
                                        nat_add(a1257, a1258)?
                                    };
                                    let a1272 = {
                                        let a1269 = {
                                            let a1259 = 1u64;
                                            let a1260 = 1u64;
                                            nat_add(a1259, a1260)?
                                        };
                                        let a1270 = {
                                            let a1267 = {
                                                let a1261 = 1u64;
                                                let a1262 = 1u64;
                                                nat_add(a1261, a1262)?
                                            };
                                            let a1268 = {
                                                let a1265 = {
                                                    let a1263 = 1u64;
                                                    let a1264 = 1u64;
                                                    nat_add(a1263, a1264)?
                                                };
                                                let a1266 = 1u64;
                                                nat_mul(a1265, a1266)?
                                            };
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
            let a1284 = 1u64;
            nat_add(a1283, a1284)?
        }, f33(v2, {
            let a1323 = {
                let a1321 = {
                    let a1319 = {
                        let a1285 = 1u64;
                        let a1286 = 1u64;
                        nat_add(a1285, a1286)?
                    };
                    let a1320 = {
                        let a1317 = {
                            let a1287 = 1u64;
                            let a1288 = 1u64;
                            nat_add(a1287, a1288)?
                        };
                        let a1318 = {
                            let a1315 = {
                                let a1289 = 1u64;
                                let a1290 = 1u64;
                                nat_add(a1289, a1290)?
                            };
                            let a1316 = {
                                let a1313 = {
                                    let a1291 = 1u64;
                                    let a1292 = 1u64;
                                    nat_add(a1291, a1292)?
                                };
                                let a1314 = {
                                    let a1311 = {
                                        let a1293 = 1u64;
                                        let a1294 = 1u64;
                                        nat_add(a1293, a1294)?
                                    };
                                    let a1312 = {
                                        let a1309 = {
                                            let a1295 = 1u64;
                                            let a1296 = 1u64;
                                            nat_add(a1295, a1296)?
                                        };
                                        let a1310 = {
                                            let a1307 = {
                                                let a1297 = 1u64;
                                                let a1298 = 1u64;
                                                nat_add(a1297, a1298)?
                                            };
                                            let a1308 = {
                                                let a1305 = {
                                                    let a1299 = 1u64;
                                                    let a1300 = 1u64;
                                                    nat_add(a1299, a1300)?
                                                };
                                                let a1306 = {
                                                    let a1303 = {
                                                        let a1301 = 1u64;
                                                        let a1302 = 1u64;
                                                        nat_add(a1301, a1302)?
                                                    };
                                                    let a1304 = 1u64;
                                                    nat_mul(a1303, a1304)?
                                                };
                                                nat_mul(a1305, a1306)?
                                            };
                                            nat_mul(a1307, a1308)?
                                        };
                                        nat_mul(a1309, a1310)?
                                    };
                                    nat_mul(a1311, a1312)?
                                };
                                nat_mul(a1313, a1314)?
                            };
                            nat_mul(a1315, a1316)?
                        };
                        nat_mul(a1317, a1318)?
                    };
                    nat_mul(a1319, a1320)?
                };
                let a1322 = 1u64;
                nat_add(a1321, a1322)?
            };
            let a1324 = v1;
            nat_sub(a1323, a1324)
        })?)
    } else {
        f40(v0, v1, v2)
    }
}

pub fn f38(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1332 = {
            let a1329 = v0;
            let a1330 = {
                let a1327 = {
                    let a1325 = 1u64;
                    let a1326 = 1u64;
                    nat_add(a1325, a1326)?
                };
                let a1328 = 1u64;
                nat_mul(a1327, a1328)?
            };
            let a1331 = 0u64;
            nat_rem(a1329, a1330, a1331)
        };
        let a1333 = 0u64;
        nat_eq(a1332, a1333)
    } else {
        false
    } {
        let a1334 = v0;
        let a1335 = 1u64;
        nat_add(a1334, a1335)
    } else {
        Ok(v0)
    }
}

pub fn f39(v0: Adt2) -> R<Option<Adt2>> {
    let a1337 = 0u64;
    let a1338 = match v0.clone() {
        Adt2::C0(_, h1336, _, _) => {
            h1336
        }
    };
    if if if nat_lt(a1337, a1338) {
        let a1341 = match v0.clone() {
            Adt2::C0(_, h1339, _, _) => {
                h1339
            }
        };
        let a1342 = match v0.clone() {
            Adt2::C0(_, _, _, h1340) => {
                h1340
            }
        };
        nat_lt(a1341, a1342)
    } else {
        false
    } {
        let a1345 = match v0.clone() {
            Adt2::C0(_, _, h1343, _) => {
                h1343
            }
        };
        let a1346 = match v0.clone() {
            Adt2::C0(h1344, _, _, _) => {
                h1344
            }
        };
        nat_lt(a1345, a1346)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1348 = match v0.clone() {
                Adt2::C0(h1347, _, _, _) => {
                    h1347
                }
            };
            let a1349 = 1u64;
            nat_sub(a1348, a1349)
        }, {
            let a1352 = match v0.clone() {
                Adt2::C0(_, h1350, _, _) => {
                    h1350
                }
            };
            let a1353 = match v0.clone() {
                Adt2::C0(_, h1351, _, _) => {
                    h1351
                }
            };
            nat_add(a1352, a1353)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1354, _) => {
                h1354
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1355) => {
                h1355
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1368 = v2;
        let a1369 = {
            let a1366 = {
                let a1356 = 1u64;
                let a1357 = 1u64;
                nat_add(a1356, a1357)?
            };
            let a1367 = {
                let a1364 = {
                    let a1358 = 1u64;
                    let a1359 = 1u64;
                    nat_add(a1358, a1359)?
                };
                let a1365 = {
                    let a1362 = {
                        let a1360 = 1u64;
                        let a1361 = 1u64;
                        nat_add(a1360, a1361)?
                    };
                    let a1363 = 1u64;
                    nat_mul(a1362, a1363)?
                };
                nat_mul(a1364, a1365)?
            };
            nat_mul(a1366, a1367)?
        };
        let a1370 = 0u64;
        nat_rem(a1368, a1369, a1370)
    };
    let v4: u64 = {
        let a1383 = v2;
        let a1384 = {
            let a1381 = {
                let a1371 = 1u64;
                let a1372 = 1u64;
                nat_add(a1371, a1372)?
            };
            let a1382 = {
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
            nat_mul(a1381, a1382)?
        };
        let a1385 = 0u64;
        nat_quot(a1383, a1384, a1385)
    };
    let v5: bool = {
        let a1394 = {
            let a1392 = {
                let a1386 = 1u64;
                let a1387 = 1u64;
                nat_add(a1386, a1387)?
            };
            let a1393 = {
                let a1390 = {
                    let a1388 = 1u64;
                    let a1389 = 1u64;
                    nat_add(a1388, a1389)?
                };
                let a1391 = 1u64;
                nat_mul(a1390, a1391)?
            };
            nat_mul(a1392, a1393)?
        };
        let a1395 = v3;
        if nat_lt(a1394, a1395) {
            true
        } else {
            let a1404 = v3;
            let a1405 = {
                let a1402 = {
                    let a1396 = 1u64;
                    let a1397 = 1u64;
                    nat_add(a1396, a1397)?
                };
                let a1403 = {
                    let a1400 = {
                        let a1398 = 1u64;
                        let a1399 = 1u64;
                        nat_add(a1398, a1399)?
                    };
                    let a1401 = 1u64;
                    nat_mul(a1400, a1401)?
                };
                nat_mul(a1402, a1403)?
            };
            if nat_eq(a1404, a1405) {
                let a1413 = {
                    let a1410 = v4;
                    let a1411 = {
                        let a1408 = {
                            let a1406 = 1u64;
                            let a1407 = 1u64;
                            nat_add(a1406, a1407)?
                        };
                        let a1409 = 1u64;
                        nat_mul(a1408, a1409)?
                    };
                    let a1412 = 0u64;
                    nat_rem(a1410, a1411, a1412)
                };
                let a1414 = 1u64;
                nat_eq(a1413, a1414)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1415 = v4;
        let a1416 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1415, a1416)?
    };
    let v7: bool = {
        let a1445 = {
            let a1443 = {
                let a1419 = {
                    let a1417 = 1u64;
                    let a1418 = 1u64;
                    nat_add(a1417, a1418)?
                };
                let a1420 = 1u64;
                nat_mul(a1419, a1420)?
            };
            let a1444 = f16({
                let a1441 = {
                    let a1439 = {
                        let a1421 = 1u64;
                        let a1422 = 1u64;
                        nat_add(a1421, a1422)?
                    };
                    let a1440 = {
                        let a1437 = {
                            let a1435 = {
                                let a1423 = 1u64;
                                let a1424 = 1u64;
                                nat_add(a1423, a1424)?
                            };
                            let a1436 = {
                                let a1433 = {
                                    let a1431 = {
                                        let a1425 = 1u64;
                                        let a1426 = 1u64;
                                        nat_add(a1425, a1426)?
                                    };
                                    let a1432 = {
                                        let a1429 = {
                                            let a1427 = 1u64;
                                            let a1428 = 1u64;
                                            nat_add(a1427, a1428)?
                                        };
                                        let a1430 = 1u64;
                                        nat_mul(a1429, a1430)?
                                    };
                                    nat_mul(a1431, a1432)?
                                };
                                let a1434 = 1u64;
                                nat_add(a1433, a1434)?
                            };
                            nat_mul(a1435, a1436)?
                        };
                        let a1438 = 1u64;
                        nat_add(a1437, a1438)?
                    };
                    nat_mul(a1439, a1440)?
                };
                let a1442 = 1u64;
                nat_add(a1441, a1442)?
            })?;
            nat_mul(a1443, a1444)?
        };
        let a1446 = v6;
        nat_le(a1445, a1446)
    };
    let v8: u64 = if v7 {
        let a1451 = v6;
        let a1452 = {
            let a1449 = {
                let a1447 = 1u64;
                let a1448 = 1u64;
                nat_add(a1447, a1448)?
            };
            let a1450 = 1u64;
            nat_mul(a1449, a1450)?
        };
        let a1453 = 0u64;
        nat_quot(a1451, a1452, a1453)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1454 = v1;
        let a1455 = 1u64;
        nat_add(a1454, a1455)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1494 = v9;
        let a1495 = {
            let a1492 = {
                let a1490 = {
                    let a1456 = 1u64;
                    let a1457 = 1u64;
                    nat_add(a1456, a1457)?
                };
                let a1491 = {
                    let a1488 = {
                        let a1458 = 1u64;
                        let a1459 = 1u64;
                        nat_add(a1458, a1459)?
                    };
                    let a1489 = {
                        let a1486 = {
                            let a1460 = 1u64;
                            let a1461 = 1u64;
                            nat_add(a1460, a1461)?
                        };
                        let a1487 = {
                            let a1484 = {
                                let a1462 = 1u64;
                                let a1463 = 1u64;
                                nat_add(a1462, a1463)?
                            };
                            let a1485 = {
                                let a1482 = {
                                    let a1464 = 1u64;
                                    let a1465 = 1u64;
                                    nat_add(a1464, a1465)?
                                };
                                let a1483 = {
                                    let a1480 = {
                                        let a1466 = 1u64;
                                        let a1467 = 1u64;
                                        nat_add(a1466, a1467)?
                                    };
                                    let a1481 = {
                                        let a1478 = {
                                            let a1468 = 1u64;
                                            let a1469 = 1u64;
                                            nat_add(a1468, a1469)?
                                        };
                                        let a1479 = {
                                            let a1476 = {
                                                let a1470 = 1u64;
                                                let a1471 = 1u64;
                                                nat_add(a1470, a1471)?
                                            };
                                            let a1477 = {
                                                let a1474 = {
                                                    let a1472 = 1u64;
                                                    let a1473 = 1u64;
                                                    nat_add(a1472, a1473)?
                                                };
                                                let a1475 = 1u64;
                                                nat_mul(a1474, a1475)?
                                            };
                                            nat_mul(a1476, a1477)?
                                        };
                                        nat_mul(a1478, a1479)?
                                    };
                                    nat_mul(a1480, a1481)?
                                };
                                nat_mul(a1482, a1483)?
                            };
                            nat_mul(a1484, a1485)?
                        };
                        nat_mul(a1486, a1487)?
                    };
                    nat_mul(a1488, a1489)?
                };
                nat_mul(a1490, a1491)?
            };
            let a1493 = 1u64;
            nat_add(a1492, a1493)?
        };
        if if nat_eq(a1494, a1495) {
            let a1518 = v8;
            let a1519 = f16({
                let a1516 = {
                    let a1514 = {
                        let a1496 = 1u64;
                        let a1497 = 1u64;
                        nat_add(a1496, a1497)?
                    };
                    let a1515 = {
                        let a1512 = {
                            let a1510 = {
                                let a1498 = 1u64;
                                let a1499 = 1u64;
                                nat_add(a1498, a1499)?
                            };
                            let a1511 = {
                                let a1508 = {
                                    let a1506 = {
                                        let a1500 = 1u64;
                                        let a1501 = 1u64;
                                        nat_add(a1500, a1501)?
                                    };
                                    let a1507 = {
                                        let a1504 = {
                                            let a1502 = 1u64;
                                            let a1503 = 1u64;
                                            nat_add(a1502, a1503)?
                                        };
                                        let a1505 = 1u64;
                                        nat_mul(a1504, a1505)?
                                    };
                                    nat_mul(a1506, a1507)?
                                };
                                let a1509 = 1u64;
                                nat_add(a1508, a1509)?
                            };
                            nat_mul(a1510, a1511)?
                        };
                        let a1513 = 1u64;
                        nat_add(a1512, a1513)?
                    };
                    nat_mul(a1514, a1515)?
                };
                let a1517 = 1u64;
                nat_add(a1516, a1517)?
            })?;
            nat_lt(a1518, a1519)
        } else {
            false
        } {
            0u64
        } else {
            let a1556 = v9;
            let a1557 = {
                let a1554 = {
                    let a1520 = 1u64;
                    let a1521 = 1u64;
                    nat_add(a1520, a1521)?
                };
                let a1555 = {
                    let a1552 = {
                        let a1522 = 1u64;
                        let a1523 = 1u64;
                        nat_add(a1522, a1523)?
                    };
                    let a1553 = {
                        let a1550 = {
                            let a1524 = 1u64;
                            let a1525 = 1u64;
                            nat_add(a1524, a1525)?
                        };
                        let a1551 = {
                            let a1548 = {
                                let a1526 = 1u64;
                                let a1527 = 1u64;
                                nat_add(a1526, a1527)?
                            };
                            let a1549 = {
                                let a1546 = {
                                    let a1528 = 1u64;
                                    let a1529 = 1u64;
                                    nat_add(a1528, a1529)?
                                };
                                let a1547 = {
                                    let a1544 = {
                                        let a1530 = 1u64;
                                        let a1531 = 1u64;
                                        nat_add(a1530, a1531)?
                                    };
                                    let a1545 = {
                                        let a1542 = {
                                            let a1532 = 1u64;
                                            let a1533 = 1u64;
                                            nat_add(a1532, a1533)?
                                        };
                                        let a1543 = {
                                            let a1540 = {
                                                let a1534 = 1u64;
                                                let a1535 = 1u64;
                                                nat_add(a1534, a1535)?
                                            };
                                            let a1541 = {
                                                let a1538 = {
                                                    let a1536 = 1u64;
                                                    let a1537 = 1u64;
                                                    nat_add(a1536, a1537)?
                                                };
                                                let a1539 = 1u64;
                                                nat_mul(a1538, a1539)?
                                            };
                                            nat_mul(a1540, a1541)?
                                        };
                                        nat_mul(a1542, a1543)?
                                    };
                                    nat_mul(a1544, a1545)?
                                };
                                nat_mul(a1546, a1547)?
                            };
                            nat_mul(a1548, a1549)?
                        };
                        nat_mul(a1550, a1551)?
                    };
                    nat_mul(a1552, a1553)?
                };
                nat_mul(a1554, a1555)?
            };
            nat_sub(a1556, a1557)
        }
    };
    let a1558 = v8;
    let a1559 = 0u64;
    if nat_eq(a1558, a1559) {
        f19(v0)
    } else {
        let a1612 = {
            let a1610 = {
                let a1608 = {
                    let a1560 = 1u64;
                    let a1561 = 1u64;
                    nat_add(a1560, a1561)?
                };
                let a1609 = {
                    let a1606 = {
                        let a1604 = {
                            let a1562 = 1u64;
                            let a1563 = 1u64;
                            nat_add(a1562, a1563)?
                        };
                        let a1605 = {
                            let a1602 = {
                                let a1600 = {
                                    let a1564 = 1u64;
                                    let a1565 = 1u64;
                                    nat_add(a1564, a1565)?
                                };
                                let a1601 = {
                                    let a1598 = {
                                        let a1596 = {
                                            let a1566 = 1u64;
                                            let a1567 = 1u64;
                                            nat_add(a1566, a1567)?
                                        };
                                        let a1597 = {
                                            let a1594 = {
                                                let a1592 = {
                                                    let a1568 = 1u64;
                                                    let a1569 = 1u64;
                                                    nat_add(a1568, a1569)?
                                                };
                                                let a1593 = {
                                                    let a1590 = {
                                                        let a1588 = {
                                                            let a1570 = 1u64;
                                                            let a1571 = 1u64;
                                                            nat_add(a1570, a1571)?
                                                        };
                                                        let a1589 = {
                                                            let a1586 = {
                                                                let a1584 = {
                                                                    let a1572 = 1u64;
                                                                    let a1573 = 1u64;
                                                                    nat_add(a1572, a1573)?
                                                                };
                                                                let a1585 = {
                                                                    let a1582 = {
                                                                        let a1580 = {
                                                                            let a1574 = 1u64;
                                                                            let a1575 = 1u64;
                                                                            nat_add(a1574, a1575)?
                                                                        };
                                                                        let a1581 = {
                                                                            let a1578 = {
                                                                                let a1576 = 1u64;
                                                                                let a1577 = 1u64;
                                                                                nat_add(a1576, a1577)?
                                                                            };
                                                                            let a1579 = 1u64;
                                                                            nat_mul(a1578, a1579)?
                                                                        };
                                                                        nat_mul(a1580, a1581)?
                                                                    };
                                                                    let a1583 = 1u64;
                                                                    nat_add(a1582, a1583)?
                                                                };
                                                                nat_mul(a1584, a1585)?
                                                            };
                                                            let a1587 = 1u64;
                                                            nat_add(a1586, a1587)?
                                                        };
                                                        nat_mul(a1588, a1589)?
                                                    };
                                                    let a1591 = 1u64;
                                                    nat_add(a1590, a1591)?
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
                        };
                        nat_mul(a1604, a1605)?
                    };
                    let a1607 = 1u64;
                    nat_add(a1606, a1607)?
                };
                nat_mul(a1608, a1609)?
            };
            let a1611 = 1u64;
            nat_add(a1610, a1611)?
        };
        let a1613 = v9;
        if nat_le(a1612, a1613) {
            f17(v0)
        } else {
            let a1665 = {
                let a1638 = f19(v0)?;
                let a1639 = {
                    let a1636 = v10;
                    let a1637 = f16({
                        let a1634 = {
                            let a1632 = {
                                let a1614 = 1u64;
                                let a1615 = 1u64;
                                nat_add(a1614, a1615)?
                            };
                            let a1633 = {
                                let a1630 = {
                                    let a1628 = {
                                        let a1616 = 1u64;
                                        let a1617 = 1u64;
                                        nat_add(a1616, a1617)?
                                    };
                                    let a1629 = {
                                        let a1626 = {
                                            let a1624 = {
                                                let a1618 = 1u64;
                                                let a1619 = 1u64;
                                                nat_add(a1618, a1619)?
                                            };
                                            let a1625 = {
                                                let a1622 = {
                                                    let a1620 = 1u64;
                                                    let a1621 = 1u64;
                                                    nat_add(a1620, a1621)?
                                                };
                                                let a1623 = 1u64;
                                                nat_mul(a1622, a1623)?
                                            };
                                            nat_mul(a1624, a1625)?
                                        };
                                        let a1627 = 1u64;
                                        nat_add(a1626, a1627)?
                                    };
                                    nat_mul(a1628, a1629)?
                                };
                                let a1631 = 1u64;
                                nat_add(a1630, a1631)?
                            };
                            nat_mul(a1632, a1633)?
                        };
                        let a1635 = 1u64;
                        nat_add(a1634, a1635)?
                    })?;
                    nat_mul(a1636, a1637)?
                };
                nat_add(a1638, a1639)?
            };
            let a1666 = {
                let a1662 = v8;
                let a1663 = f16({
                    let a1660 = {
                        let a1658 = {
                            let a1640 = 1u64;
                            let a1641 = 1u64;
                            nat_add(a1640, a1641)?
                        };
                        let a1659 = {
                            let a1656 = {
                                let a1654 = {
                                    let a1642 = 1u64;
                                    let a1643 = 1u64;
                                    nat_add(a1642, a1643)?
                                };
                                let a1655 = {
                                    let a1652 = {
                                        let a1650 = {
                                            let a1644 = 1u64;
                                            let a1645 = 1u64;
                                            nat_add(a1644, a1645)?
                                        };
                                        let a1651 = {
                                            let a1648 = {
                                                let a1646 = 1u64;
                                                let a1647 = 1u64;
                                                nat_add(a1646, a1647)?
                                            };
                                            let a1649 = 1u64;
                                            nat_mul(a1648, a1649)?
                                        };
                                        nat_mul(a1650, a1651)?
                                    };
                                    let a1653 = 1u64;
                                    nat_add(a1652, a1653)?
                                };
                                nat_mul(a1654, a1655)?
                            };
                            let a1657 = 1u64;
                            nat_add(a1656, a1657)?
                        };
                        nat_mul(a1658, a1659)?
                    };
                    let a1661 = 1u64;
                    nat_add(a1660, a1661)?
                })?;
                let a1664 = 0u64;
                nat_rem(a1662, a1663, a1664)
            };
            nat_add(a1665, a1666)
        }
    }
}
