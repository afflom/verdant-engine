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
    C11(bool, u64, u64),
    C12(bool, u64, u64),
    C13(bool, u64),
    C14(bool, u64, u64, u64, u64),
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
    F34,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F34 => f34(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F39,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F39 => f39(p0),
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
        Adt0::C7(v54, v55) => {
            let a73 = v54;
            let a74 = 4294967296u64;
            if if nat_lt(a73, a74) {
                let a75 = v55;
                let a76 = 2139095040u64;
                nat_lt(a75, a76)
            } else {
                false
            } {
                Ok(Adt1::C2(f13(f5(v54, v55)?)?))
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
        Adt0::C11(v41, v42, v43) => {
            let a69 = v43;
            let a70 = 18u64;
            if if nat_eq(a69, a70) {
                let a71 = v42;
                let a72 = if v41 {
                    64u64
                } else {
                    128u64
                };
                nat_eq(a71, a72)
            } else {
                false
            } {
                Ok(Adt1::C2(if v41 {
                    4u64
                } else {
                    8u64
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C12(v44, v45, v46) => {
            f10(v44, v45, v46)
        }
        Adt0::C13(v47, v48) => {
            f11(v47, v48)
        }
        Adt0::C14(v49, v50, v51, v52, v53) => {
            f12(v49, v50, v51, v52, v53)
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a77 = 0u64;
    let a78 = v2;
    if if nat_lt(a77, a78) {
        let a79 = 0u64;
        let a80 = v3;
        nat_lt(a79, a80)
    } else {
        false
    } {
        Ok(Some({
            let a86 = {
                let a83 = {
                    let a81 = v0;
                    let a82 = v1;
                    nat_mul(a81, a82)?
                };
                let a84 = v2;
                let a85 = 0u64;
                nat_quot(a83, a84, a85)
            };
            let a87 = v3;
            let a88 = 0u64;
            nat_rem(a86, a87, a88)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a89 = v0;
    let a90 = 0u64;
    if nat_eq(a89, a90) {
        1u64
    } else {
        let a91 = v0;
        let a92 = 1u64;
        if nat_eq(a91, a92) {
            2u64
        } else {
            let a93 = v0;
            let a94 = 2u64;
            if nat_eq(a93, a94) {
                4u64
            } else {
                let a95 = v0;
                let a96 = 3u64;
                if nat_eq(a95, a96) {
                    8u64
                } else {
                    let a97 = v0;
                    let a98 = 4u64;
                    if nat_eq(a97, a98) {
                        16u64
                    } else {
                        let a99 = v0;
                        let a100 = 5u64;
                        if nat_eq(a99, a100) {
                            32u64
                        } else {
                            let a101 = v0;
                            let a102 = 6u64;
                            if nat_eq(a101, a102) {
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
    let a103 = v1;
    let a104 = 16u64;
    if if if nat_lt(a103, a104) {
        let a105 = v2;
        let a106 = 256u64;
        nat_lt(a105, a106)
    } else {
        false
    } {
        let a107 = v3;
        let a108 = 4294967296u64;
        nat_lt(a107, a108)
    } else {
        false
    } {
        let v4: u64 = {
            let a109 = v2;
            let a110 = 16u64;
            let a111 = 0u64;
            nat_rem(a109, a110, a111)
        };
        let v5: u64 = {
            let a112 = v2;
            let a113 = 16u64;
            let a114 = 0u64;
            nat_quot(a112, a113, a114)
        };
        if v0 {
            Ok(Adt1::C4({
                let a115 = 18568u64;
                let a116 = v4;
                nat_add(a115, a116)?
            }, {
                let a117 = 18568u64;
                let a118 = v5;
                nat_add(a117, a118)?
            }))
        } else {
            let v6: u64 = {
                let a124 = f2({
                    let a119 = v1;
                    let a120 = 8u64;
                    let a121 = 0u64;
                    nat_rem(a119, a120, a121)
                });
                let a125 = {
                    let a122 = v1;
                    let a123 = 8u64;
                    if nat_lt(a122, a123) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a124, a125)?
            };
            Ok(Adt1::C3(f14(v4, v3, v6)?, f14(v5, v3, {
                let a126 = v6;
                let a127 = 65536u64;
                nat_mul(a126, a127)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a128 = v0;
    let a129 = 16u64;
    if if if nat_lt(a128, a129) {
        let a130 = v1;
        let a131 = 256u64;
        nat_lt(a130, a131)
    } else {
        false
    } {
        let a132 = v2;
        let a133 = 4294967296u64;
        nat_lt(a132, a133)
    } else {
        false
    } {
        let v3: u64 = {
            let a139 = f2({
                let a134 = v0;
                let a135 = 8u64;
                let a136 = 0u64;
                nat_rem(a134, a135, a136)
            });
            let a140 = {
                let a137 = v0;
                let a138 = 8u64;
                if nat_lt(a137, a138) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a139, a140)?
        };
        Ok(Adt1::C3(f15({
            let a141 = v1;
            let a142 = 16u64;
            let a143 = 0u64;
            nat_rem(a141, a142, a143)
        }, v2, v3)?, f15({
            let a144 = v1;
            let a145 = 16u64;
            let a146 = 0u64;
            nat_quot(a144, a145, a146)
        }, v2, {
            let a147 = v3;
            let a148 = 65536u64;
            nat_mul(a147, a148)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f16(v0)? {
        true
    } else {
        f16(v1)?
    } {
        true
    } else {
        if f17(v0)? {
            f18(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f17(v1)? {
            f18(v0)?
        } else {
            false
        }
    } {
        let a235 = {
            let a213 = {
                let a189 = {
                    let a187 = {
                        let a149 = 1u64;
                        let a150 = 1u64;
                        nat_add(a149, a150)?
                    };
                    let a188 = {
                        let a185 = {
                            let a183 = {
                                let a151 = 1u64;
                                let a152 = 1u64;
                                nat_add(a151, a152)?
                            };
                            let a184 = {
                                let a181 = {
                                    let a179 = {
                                        let a153 = 1u64;
                                        let a154 = 1u64;
                                        nat_add(a153, a154)?
                                    };
                                    let a180 = {
                                        let a177 = {
                                            let a175 = {
                                                let a155 = 1u64;
                                                let a156 = 1u64;
                                                nat_add(a155, a156)?
                                            };
                                            let a176 = {
                                                let a173 = {
                                                    let a171 = {
                                                        let a157 = 1u64;
                                                        let a158 = 1u64;
                                                        nat_add(a157, a158)?
                                                    };
                                                    let a172 = {
                                                        let a169 = {
                                                            let a167 = {
                                                                let a159 = 1u64;
                                                                let a160 = 1u64;
                                                                nat_add(a159, a160)?
                                                            };
                                                            let a168 = {
                                                                let a165 = {
                                                                    let a163 = {
                                                                        let a161 = 1u64;
                                                                        let a162 = 1u64;
                                                                        nat_add(a161, a162)?
                                                                    };
                                                                    let a164 = 1u64;
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
                    nat_mul(a187, a188)?
                };
                let a190 = 1u64;
                nat_add(a189, a190)?
            };
            let a214 = f19({
                let a211 = {
                    let a209 = {
                        let a191 = 1u64;
                        let a192 = 1u64;
                        nat_add(a191, a192)?
                    };
                    let a210 = {
                        let a207 = {
                            let a205 = {
                                let a193 = 1u64;
                                let a194 = 1u64;
                                nat_add(a193, a194)?
                            };
                            let a206 = {
                                let a203 = {
                                    let a201 = {
                                        let a195 = 1u64;
                                        let a196 = 1u64;
                                        nat_add(a195, a196)?
                                    };
                                    let a202 = {
                                        let a199 = {
                                            let a197 = 1u64;
                                            let a198 = 1u64;
                                            nat_add(a197, a198)?
                                        };
                                        let a200 = 1u64;
                                        nat_mul(a199, a200)?
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
                    };
                    nat_mul(a209, a210)?
                };
                let a212 = 1u64;
                nat_add(a211, a212)?
            })?;
            nat_mul(a213, a214)?
        };
        let a236 = f19({
            let a233 = {
                let a215 = 1u64;
                let a216 = 1u64;
                nat_add(a215, a216)?
            };
            let a234 = {
                let a231 = {
                    let a229 = {
                        let a217 = 1u64;
                        let a218 = 1u64;
                        nat_add(a217, a218)?
                    };
                    let a230 = {
                        let a227 = {
                            let a225 = {
                                let a219 = 1u64;
                                let a220 = 1u64;
                                nat_add(a219, a220)?
                            };
                            let a226 = {
                                let a223 = {
                                    let a221 = 1u64;
                                    let a222 = 1u64;
                                    nat_add(a221, a222)?
                                };
                                let a224 = 1u64;
                                nat_mul(a223, a224)?
                            };
                            nat_mul(a225, a226)?
                        };
                        let a228 = 1u64;
                        nat_add(a227, a228)?
                    };
                    nat_mul(a229, a230)?
                };
                let a232 = 1u64;
                nat_add(a231, a232)?
            };
            nat_mul(a233, a234)?
        })?;
        nat_add(a235, a236)
    } else {
        if if f17(v0)? {
            true
        } else {
            f17(v1)?
        } {
            f20(f21(v0, v1)?)
        } else {
            if if f18(v0)? {
                true
            } else {
                f18(v1)?
            } {
                f22(f21(v0, v1)?)
            } else {
                f23(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a240 = {
            let a237 = v0;
            let a238 = 32768u64;
            let a239 = 0u64;
            nat_quot(a237, a238, a239)
        };
        let a241 = 2147483648u64;
        nat_mul(a240, a241)?
    };
    let v2: u64 = {
        let a245 = {
            let a242 = v0;
            let a243 = 32768u64;
            let a244 = 0u64;
            nat_rem(a242, a243, a244)
        };
        let a246 = 1024u64;
        let a247 = 0u64;
        nat_quot(a245, a246, a247)
    };
    let v3: u64 = {
        let a248 = v0;
        let a249 = 1024u64;
        let a250 = 0u64;
        nat_rem(a248, a249, a250)
    };
    let a285 = v1;
    let a286 = {
        let a251 = v2;
        let a252 = 31u64;
        if nat_eq(a251, a252) {
            let a261 = 2139095040u64;
            let a262 = {
                let a259 = {
                    let a253 = 0u64;
                    let a254 = v3;
                    if if nat_lt(a253, a254) {
                        let a255 = v3;
                        let a256 = 512u64;
                        nat_lt(a255, a256)
                    } else {
                        false
                    } {
                        let a257 = v3;
                        let a258 = 512u64;
                        nat_add(a257, a258)?
                    } else {
                        v3
                    }
                };
                let a260 = 8192u64;
                nat_mul(a259, a260)?
            };
            nat_add(a261, a262)?
        } else {
            let a263 = v2;
            let a264 = 0u64;
            if nat_eq(a263, a264) {
                let a265 = v3;
                let a266 = 0u64;
                if nat_eq(a265, a266) {
                    0u64
                } else {
                    let v4: Adt2 = f24(113u64, v3, 0u64, 1024u64)?;
                    let a275 = {
                        let a268 = match v4.clone() {
                            Adt2::C0(h267, _, _, _) => {
                                h267
                            }
                        };
                        let a269 = 8388608u64;
                        nat_mul(a268, a269)?
                    };
                    let a276 = {
                        let a273 = {
                            let a271 = match v4 {
                                Adt2::C0(_, h270, _, _) => {
                                    h270
                                }
                            };
                            let a272 = 1024u64;
                            nat_sub(a271, a272)
                        };
                        let a274 = 8192u64;
                        nat_mul(a273, a274)?
                    };
                    nat_add(a275, a276)?
                }
            } else {
                let a283 = {
                    let a279 = {
                        let a277 = v2;
                        let a278 = 112u64;
                        nat_add(a277, a278)?
                    };
                    let a280 = 8388608u64;
                    nat_mul(a279, a280)?
                };
                let a284 = {
                    let a281 = v3;
                    let a282 = 8192u64;
                    nat_mul(a281, a282)?
                };
                nat_add(a283, a284)?
            }
        }
    };
    nat_add(a285, a286)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f16(v0)? {
        true
    } else {
        f16(v1)?
    } {
        let a373 = {
            let a351 = {
                let a327 = {
                    let a325 = {
                        let a287 = 1u64;
                        let a288 = 1u64;
                        nat_add(a287, a288)?
                    };
                    let a326 = {
                        let a323 = {
                            let a321 = {
                                let a289 = 1u64;
                                let a290 = 1u64;
                                nat_add(a289, a290)?
                            };
                            let a322 = {
                                let a319 = {
                                    let a317 = {
                                        let a291 = 1u64;
                                        let a292 = 1u64;
                                        nat_add(a291, a292)?
                                    };
                                    let a318 = {
                                        let a315 = {
                                            let a313 = {
                                                let a293 = 1u64;
                                                let a294 = 1u64;
                                                nat_add(a293, a294)?
                                            };
                                            let a314 = {
                                                let a311 = {
                                                    let a309 = {
                                                        let a295 = 1u64;
                                                        let a296 = 1u64;
                                                        nat_add(a295, a296)?
                                                    };
                                                    let a310 = {
                                                        let a307 = {
                                                            let a305 = {
                                                                let a297 = 1u64;
                                                                let a298 = 1u64;
                                                                nat_add(a297, a298)?
                                                            };
                                                            let a306 = {
                                                                let a303 = {
                                                                    let a301 = {
                                                                        let a299 = 1u64;
                                                                        let a300 = 1u64;
                                                                        nat_add(a299, a300)?
                                                                    };
                                                                    let a302 = 1u64;
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
                    nat_mul(a325, a326)?
                };
                let a328 = 1u64;
                nat_add(a327, a328)?
            };
            let a352 = f19({
                let a349 = {
                    let a347 = {
                        let a329 = 1u64;
                        let a330 = 1u64;
                        nat_add(a329, a330)?
                    };
                    let a348 = {
                        let a345 = {
                            let a343 = {
                                let a331 = 1u64;
                                let a332 = 1u64;
                                nat_add(a331, a332)?
                            };
                            let a344 = {
                                let a341 = {
                                    let a339 = {
                                        let a333 = 1u64;
                                        let a334 = 1u64;
                                        nat_add(a333, a334)?
                                    };
                                    let a340 = {
                                        let a337 = {
                                            let a335 = 1u64;
                                            let a336 = 1u64;
                                            nat_add(a335, a336)?
                                        };
                                        let a338 = 1u64;
                                        nat_mul(a337, a338)?
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
                    };
                    nat_mul(a347, a348)?
                };
                let a350 = 1u64;
                nat_add(a349, a350)?
            })?;
            nat_mul(a351, a352)?
        };
        let a374 = f19({
            let a371 = {
                let a353 = 1u64;
                let a354 = 1u64;
                nat_add(a353, a354)?
            };
            let a372 = {
                let a369 = {
                    let a367 = {
                        let a355 = 1u64;
                        let a356 = 1u64;
                        nat_add(a355, a356)?
                    };
                    let a368 = {
                        let a365 = {
                            let a363 = {
                                let a357 = 1u64;
                                let a358 = 1u64;
                                nat_add(a357, a358)?
                            };
                            let a364 = {
                                let a361 = {
                                    let a359 = 1u64;
                                    let a360 = 1u64;
                                    nat_add(a359, a360)?
                                };
                                let a362 = 1u64;
                                nat_mul(a361, a362)?
                            };
                            nat_mul(a363, a364)?
                        };
                        let a366 = 1u64;
                        nat_add(a365, a366)?
                    };
                    nat_mul(a367, a368)?
                };
                let a370 = 1u64;
                nat_add(a369, a370)?
            };
            nat_mul(a371, a372)?
        })?;
        nat_add(a373, a374)
    } else {
        if if f17(v0)? {
            f17(v1)?
        } else {
            false
        } {
            let a375 = f25(v0)?;
            let a376 = f25(v1)?;
            if nat_eq(a375, a376) {
                Ok(v0)
            } else {
                let a463 = {
                    let a441 = {
                        let a417 = {
                            let a415 = {
                                let a377 = 1u64;
                                let a378 = 1u64;
                                nat_add(a377, a378)?
                            };
                            let a416 = {
                                let a413 = {
                                    let a411 = {
                                        let a379 = 1u64;
                                        let a380 = 1u64;
                                        nat_add(a379, a380)?
                                    };
                                    let a412 = {
                                        let a409 = {
                                            let a407 = {
                                                let a381 = 1u64;
                                                let a382 = 1u64;
                                                nat_add(a381, a382)?
                                            };
                                            let a408 = {
                                                let a405 = {
                                                    let a403 = {
                                                        let a383 = 1u64;
                                                        let a384 = 1u64;
                                                        nat_add(a383, a384)?
                                                    };
                                                    let a404 = {
                                                        let a401 = {
                                                            let a399 = {
                                                                let a385 = 1u64;
                                                                let a386 = 1u64;
                                                                nat_add(a385, a386)?
                                                            };
                                                            let a400 = {
                                                                let a397 = {
                                                                    let a395 = {
                                                                        let a387 = 1u64;
                                                                        let a388 = 1u64;
                                                                        nat_add(a387, a388)?
                                                                    };
                                                                    let a396 = {
                                                                        let a393 = {
                                                                            let a391 = {
                                                                                let a389 = 1u64;
                                                                                let a390 = 1u64;
                                                                                nat_add(a389, a390)?
                                                                            };
                                                                            let a392 = 1u64;
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
                            nat_mul(a415, a416)?
                        };
                        let a418 = 1u64;
                        nat_add(a417, a418)?
                    };
                    let a442 = f19({
                        let a439 = {
                            let a437 = {
                                let a419 = 1u64;
                                let a420 = 1u64;
                                nat_add(a419, a420)?
                            };
                            let a438 = {
                                let a435 = {
                                    let a433 = {
                                        let a421 = 1u64;
                                        let a422 = 1u64;
                                        nat_add(a421, a422)?
                                    };
                                    let a434 = {
                                        let a431 = {
                                            let a429 = {
                                                let a423 = 1u64;
                                                let a424 = 1u64;
                                                nat_add(a423, a424)?
                                            };
                                            let a430 = {
                                                let a427 = {
                                                    let a425 = 1u64;
                                                    let a426 = 1u64;
                                                    nat_add(a425, a426)?
                                                };
                                                let a428 = 1u64;
                                                nat_mul(a427, a428)?
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
                            };
                            nat_mul(a437, a438)?
                        };
                        let a440 = 1u64;
                        nat_add(a439, a440)?
                    })?;
                    nat_mul(a441, a442)?
                };
                let a464 = f19({
                    let a461 = {
                        let a443 = 1u64;
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    };
                    let a462 = {
                        let a459 = {
                            let a457 = {
                                let a445 = 1u64;
                                let a446 = 1u64;
                                nat_add(a445, a446)?
                            };
                            let a458 = {
                                let a455 = {
                                    let a453 = {
                                        let a447 = 1u64;
                                        let a448 = 1u64;
                                        nat_add(a447, a448)?
                                    };
                                    let a454 = {
                                        let a451 = {
                                            let a449 = 1u64;
                                            let a450 = 1u64;
                                            nat_add(a449, a450)?
                                        };
                                        let a452 = 1u64;
                                        nat_mul(a451, a452)?
                                    };
                                    nat_mul(a453, a454)?
                                };
                                let a456 = 1u64;
                                nat_add(a455, a456)?
                            };
                            nat_mul(a457, a458)?
                        };
                        let a460 = 1u64;
                        nat_add(a459, a460)?
                    };
                    nat_mul(a461, a462)?
                })?;
                nat_add(a463, a464)
            }
        } else {
            if f17(v0)? {
                Ok(v0)
            } else {
                if f17(v1)? {
                    Ok(v1)
                } else {
                    let a465 = f26(v1)?;
                    let a466 = f26(v0)?;
                    if nat_le(a465, a466) {
                        f27(v0, v1)
                    } else {
                        f27(v1, v0)
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
    let a470 = {
        let a467 = v1;
        let a468 = v3;
        let a469 = 0u64;
        nat_rem(a467, a468, a469)
    };
    let a471 = 0u64;
    if if nat_eq(a470, a471) {
        let a477 = {
            let a475 = {
                let a472 = v1;
                let a473 = v3;
                let a474 = 0u64;
                nat_quot(a472, a473, a474)
            };
            let a476 = v4;
            nat_mul(a475, a476)?
        };
        let a478 = v2;
        nat_eq(a477, a478)
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
    let a479 = v0;
    let a480 = 4294967296u64;
    if if if if nat_lt(a479, a480) {
        let a481 = v1;
        let a482 = 4294967296u64;
        nat_lt(a481, a482)
    } else {
        false
    } {
        let a483 = v2;
        let a484 = 4294967296u64;
        nat_lt(a483, a484)
    } else {
        false
    } {
        let a485 = v3;
        let a486 = 65536u64;
        nat_lt(a485, a486)
    } else {
        false
    } {
        let v4: u64 = f5(f6(v3)?, v2)?;
        Ok(Adt1::C2(f7(v0, f5(v4, v1)?)?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f10(v0: bool, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = f26(v2)?;
    let a487 = v2;
    let a488 = 4294967296u64;
    if if nat_lt(a487, a488) {
        let a489 = v3;
        let a490 = 2139095040u64;
        nat_lt(a489, a490)
    } else {
        false
    } {
        Ok(Adt1::C2(if v0 {
            let a491 = v1;
            let a492 = v3;
            if nat_lt(a491, a492) {
                v3
            } else {
                v1
            }
        } else {
            f7(v1, v3)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f11(v0: bool, v1: u64) -> R<Adt1> {
    let v2: u64 = if v0 {
        v1
    } else {
        f28(v1, 1124073472u64)?
    };
    let v3: u64 = if v0 {
        let a493 = v2;
        let a494 = 0u64;
        if nat_eq(a493, a494) {
            0u64
        } else {
            f28(1065353216u64, v2)?
        }
    } else {
        0u64
    };
    let a495 = v3;
    let a496 = 2139095040u64;
    if nat_lt(a495, a496) {
        Ok(Adt1::C3(f13(v2)?, v3))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f12(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a497 = v2;
    let a498 = 256u64;
    if if nat_lt(a497, a498) {
        let a499 = v1;
        let a500 = if v0 {
            4u64
        } else {
            8u64
        };
        nat_lt(a499, a500)
    } else {
        false
    } {
        let v5: u64 = if v0 {
            f5(v3, v4)?
        } else {
            v3
        };
        let v6: u64 = f26(v5)?;
        let v7: u64 = if v0 {
            let a501 = v6;
            let a502 = 1056964608u64;
            if nat_lt(a501, a502) {
                1u64
            } else {
                let a503 = v5;
                let a504 = 2147483648u64;
                if nat_lt(a503, a504) {
                    let a505 = v6;
                    let a506 = 1069547520u64;
                    if nat_lt(a505, a506) {
                        2u64
                    } else {
                        3u64
                    }
                } else {
                    0u64
                }
            }
        } else {
            let a507 = v3;
            let a508 = 2147483648u64;
            if if nat_lt(a507, a508) {
                true
            } else {
                let a509 = v6;
                let a510 = 0u64;
                nat_eq(a509, a510)
            } {
                1u64
            } else {
                0u64
            }
        };
        Ok(Adt1::C2({
            let a515 = v2;
            let a516 = {
                let a513 = v7;
                let a514 = f2(if v0 {
                    let a511 = v1;
                    let a512 = 2u64;
                    nat_mul(a511, a512)?
                } else {
                    v1
                });
                nat_mul(a513, a514)?
            };
            nat_add(a515, a516)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f13(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a517 = f25(v0)?;
        let a518 = 32768u64;
        nat_mul(a517, a518)?
    };
    let v2: u64 = f29(v0)?;
    let a537 = v1;
    let a538 = {
        let a519 = v2;
        let a520 = 255u64;
        if nat_eq(a519, a520) {
            let a521 = f30(v0)?;
            let a522 = 0u64;
            if nat_eq(a521, a522) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a523 = 143u64;
            let a524 = v2;
            if nat_le(a523, a524) {
                31744u64
            } else {
                let a525 = v2;
                let a526 = 102u64;
                if nat_lt(a525, a526) {
                    0u64
                } else {
                    let a527 = 113u64;
                    let a528 = v2;
                    if nat_le(a527, a528) {
                        let a533 = {
                            let a531 = {
                                let a529 = v2;
                                let a530 = 113u64;
                                nat_sub(a529, a530)
                            };
                            let a532 = 1024u64;
                            nat_mul(a531, a532)?
                        };
                        let a534 = f31(f32(v0)?, 8192u64)?;
                        nat_add(a533, a534)?
                    } else {
                        f31(f32(v0)?, f19({
                            let a535 = 126u64;
                            let a536 = v2;
                            nat_sub(a535, a536)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a537, a538)
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a545 = {
        let a542 = {
            let a539 = v1;
            let a540 = v2;
            let a541 = 0u64;
            nat_quot(a539, a540, a541)
        };
        let a543 = 2u64;
        let a544 = 0u64;
        nat_rem(a542, a543, a544)
    };
    let a546 = 1u64;
    if nat_eq(a545, a546) {
        Ok(v0)
    } else {
        let a547 = 240u64;
        let a548 = v0;
        nat_add(a547, a548)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a555 = {
        let a552 = {
            let a549 = v1;
            let a550 = v2;
            let a551 = 0u64;
            nat_quot(a549, a550, a551)
        };
        let a553 = 2u64;
        let a554 = 0u64;
        nat_rem(a552, a553, a554)
    };
    let a556 = 1u64;
    if nat_eq(a555, a556) {
        let a557 = 16u64;
        let a558 = v0;
        nat_add(a557, a558)
    } else {
        Ok(v0)
    }
}

pub fn f16(v0: u64) -> R<bool> {
    let a601 = f29(v0)?;
    let a602 = {
        let a599 = {
            let a597 = {
                let a559 = 1u64;
                let a560 = 1u64;
                nat_add(a559, a560)?
            };
            let a598 = {
                let a595 = {
                    let a593 = {
                        let a561 = 1u64;
                        let a562 = 1u64;
                        nat_add(a561, a562)?
                    };
                    let a594 = {
                        let a591 = {
                            let a589 = {
                                let a563 = 1u64;
                                let a564 = 1u64;
                                nat_add(a563, a564)?
                            };
                            let a590 = {
                                let a587 = {
                                    let a585 = {
                                        let a565 = 1u64;
                                        let a566 = 1u64;
                                        nat_add(a565, a566)?
                                    };
                                    let a586 = {
                                        let a583 = {
                                            let a581 = {
                                                let a567 = 1u64;
                                                let a568 = 1u64;
                                                nat_add(a567, a568)?
                                            };
                                            let a582 = {
                                                let a579 = {
                                                    let a577 = {
                                                        let a569 = 1u64;
                                                        let a570 = 1u64;
                                                        nat_add(a569, a570)?
                                                    };
                                                    let a578 = {
                                                        let a575 = {
                                                            let a573 = {
                                                                let a571 = 1u64;
                                                                let a572 = 1u64;
                                                                nat_add(a571, a572)?
                                                            };
                                                            let a574 = 1u64;
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
                                            nat_mul(a581, a582)?
                                        };
                                        let a584 = 1u64;
                                        nat_add(a583, a584)?
                                    };
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
    if nat_eq(a601, a602) {
        let a603 = 0u64;
        let a604 = f30(v0)?;
        Ok(nat_lt(a603, a604))
    } else {
        Ok(false)
    }
}

pub fn f17(v0: u64) -> R<bool> {
    let a647 = f29(v0)?;
    let a648 = {
        let a645 = {
            let a643 = {
                let a605 = 1u64;
                let a606 = 1u64;
                nat_add(a605, a606)?
            };
            let a644 = {
                let a641 = {
                    let a639 = {
                        let a607 = 1u64;
                        let a608 = 1u64;
                        nat_add(a607, a608)?
                    };
                    let a640 = {
                        let a637 = {
                            let a635 = {
                                let a609 = 1u64;
                                let a610 = 1u64;
                                nat_add(a609, a610)?
                            };
                            let a636 = {
                                let a633 = {
                                    let a631 = {
                                        let a611 = 1u64;
                                        let a612 = 1u64;
                                        nat_add(a611, a612)?
                                    };
                                    let a632 = {
                                        let a629 = {
                                            let a627 = {
                                                let a613 = 1u64;
                                                let a614 = 1u64;
                                                nat_add(a613, a614)?
                                            };
                                            let a628 = {
                                                let a625 = {
                                                    let a623 = {
                                                        let a615 = 1u64;
                                                        let a616 = 1u64;
                                                        nat_add(a615, a616)?
                                                    };
                                                    let a624 = {
                                                        let a621 = {
                                                            let a619 = {
                                                                let a617 = 1u64;
                                                                let a618 = 1u64;
                                                                nat_add(a617, a618)?
                                                            };
                                                            let a620 = 1u64;
                                                            nat_mul(a619, a620)?
                                                        };
                                                        let a622 = 1u64;
                                                        nat_add(a621, a622)?
                                                    };
                                                    nat_mul(a623, a624)?
                                                };
                                                let a626 = 1u64;
                                                nat_add(a625, a626)?
                                            };
                                            nat_mul(a627, a628)?
                                        };
                                        let a630 = 1u64;
                                        nat_add(a629, a630)?
                                    };
                                    nat_mul(a631, a632)?
                                };
                                let a634 = 1u64;
                                nat_add(a633, a634)?
                            };
                            nat_mul(a635, a636)?
                        };
                        let a638 = 1u64;
                        nat_add(a637, a638)?
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
    if nat_eq(a647, a648) {
        let a649 = f30(v0)?;
        let a650 = 0u64;
        Ok(nat_eq(a649, a650))
    } else {
        Ok(false)
    }
}

pub fn f18(v0: u64) -> R<bool> {
    let a651 = f26(v0)?;
    let a652 = 0u64;
    Ok(nat_eq(a651, a652))
}

pub fn f19(v0: u64) -> R<u64> {
    f33(Fn0::F34, v0, 1u64)
}

pub fn f20(v0: u64) -> R<u64> {
    let a719 = f22(v0)?;
    let a720 = {
        let a717 = {
            let a693 = {
                let a691 = {
                    let a653 = 1u64;
                    let a654 = 1u64;
                    nat_add(a653, a654)?
                };
                let a692 = {
                    let a689 = {
                        let a687 = {
                            let a655 = 1u64;
                            let a656 = 1u64;
                            nat_add(a655, a656)?
                        };
                        let a688 = {
                            let a685 = {
                                let a683 = {
                                    let a657 = 1u64;
                                    let a658 = 1u64;
                                    nat_add(a657, a658)?
                                };
                                let a684 = {
                                    let a681 = {
                                        let a679 = {
                                            let a659 = 1u64;
                                            let a660 = 1u64;
                                            nat_add(a659, a660)?
                                        };
                                        let a680 = {
                                            let a677 = {
                                                let a675 = {
                                                    let a661 = 1u64;
                                                    let a662 = 1u64;
                                                    nat_add(a661, a662)?
                                                };
                                                let a676 = {
                                                    let a673 = {
                                                        let a671 = {
                                                            let a663 = 1u64;
                                                            let a664 = 1u64;
                                                            nat_add(a663, a664)?
                                                        };
                                                        let a672 = {
                                                            let a669 = {
                                                                let a667 = {
                                                                    let a665 = 1u64;
                                                                    let a666 = 1u64;
                                                                    nat_add(a665, a666)?
                                                                };
                                                                let a668 = 1u64;
                                                                nat_mul(a667, a668)?
                                                            };
                                                            let a670 = 1u64;
                                                            nat_add(a669, a670)?
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
                                };
                                nat_mul(a683, a684)?
                            };
                            let a686 = 1u64;
                            nat_add(a685, a686)?
                        };
                        nat_mul(a687, a688)?
                    };
                    let a690 = 1u64;
                    nat_add(a689, a690)?
                };
                nat_mul(a691, a692)?
            };
            let a694 = 1u64;
            nat_add(a693, a694)?
        };
        let a718 = f19({
            let a715 = {
                let a713 = {
                    let a695 = 1u64;
                    let a696 = 1u64;
                    nat_add(a695, a696)?
                };
                let a714 = {
                    let a711 = {
                        let a709 = {
                            let a697 = 1u64;
                            let a698 = 1u64;
                            nat_add(a697, a698)?
                        };
                        let a710 = {
                            let a707 = {
                                let a705 = {
                                    let a699 = 1u64;
                                    let a700 = 1u64;
                                    nat_add(a699, a700)?
                                };
                                let a706 = {
                                    let a703 = {
                                        let a701 = 1u64;
                                        let a702 = 1u64;
                                        nat_add(a701, a702)?
                                    };
                                    let a704 = 1u64;
                                    nat_mul(a703, a704)?
                                };
                                nat_mul(a705, a706)?
                            };
                            let a708 = 1u64;
                            nat_add(a707, a708)?
                        };
                        nat_mul(a709, a710)?
                    };
                    let a712 = 1u64;
                    nat_add(a711, a712)?
                };
                nat_mul(a713, a714)?
            };
            let a716 = 1u64;
            nat_add(a715, a716)?
        })?;
        nat_mul(a717, a718)?
    };
    nat_add(a719, a720)
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a727 = {
        let a721 = f25(v0)?;
        let a722 = f25(v1)?;
        nat_add(a721, a722)?
    };
    let a728 = {
        let a725 = {
            let a723 = 1u64;
            let a724 = 1u64;
            nat_add(a723, a724)?
        };
        let a726 = 1u64;
        nat_mul(a725, a726)?
    };
    let a729 = 0u64;
    Ok(nat_rem(a727, a728, a729))
}

pub fn f22(v0: u64) -> R<u64> {
    let a754 = v0;
    let a755 = f19({
        let a752 = {
            let a750 = {
                let a730 = 1u64;
                let a731 = 1u64;
                nat_add(a730, a731)?
            };
            let a751 = {
                let a748 = {
                    let a746 = {
                        let a732 = 1u64;
                        let a733 = 1u64;
                        nat_add(a732, a733)?
                    };
                    let a747 = {
                        let a744 = {
                            let a742 = {
                                let a734 = 1u64;
                                let a735 = 1u64;
                                nat_add(a734, a735)?
                            };
                            let a743 = {
                                let a740 = {
                                    let a738 = {
                                        let a736 = 1u64;
                                        let a737 = 1u64;
                                        nat_add(a736, a737)?
                                    };
                                    let a739 = 1u64;
                                    nat_mul(a738, a739)?
                                };
                                let a741 = 1u64;
                                nat_add(a740, a741)?
                            };
                            nat_mul(a742, a743)?
                        };
                        let a745 = 1u64;
                        nat_add(a744, a745)?
                    };
                    nat_mul(a746, a747)?
                };
                let a749 = 1u64;
                nat_add(a748, a749)?
            };
            nat_mul(a750, a751)?
        };
        let a753 = 1u64;
        nat_add(a752, a753)?
    })?;
    nat_mul(a754, a755)
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f35(v0)?;
    let v3: Adt2 = f35(v1)?;
    f36(f21(v0, v1)?, {
        let a810 = {
            let a758 = match v2.clone() {
                Adt2::C0(h756, _, _, _) => {
                    h756
                }
            };
            let a759 = match v3.clone() {
                Adt2::C0(h757, _, _, _) => {
                    h757
                }
            };
            nat_add(a758, a759)?
        };
        let a811 = {
            let a808 = {
                let a806 = {
                    let a760 = 1u64;
                    let a761 = 1u64;
                    nat_add(a760, a761)?
                };
                let a807 = {
                    let a804 = {
                        let a802 = {
                            let a762 = 1u64;
                            let a763 = 1u64;
                            nat_add(a762, a763)?
                        };
                        let a803 = {
                            let a800 = {
                                let a798 = {
                                    let a764 = 1u64;
                                    let a765 = 1u64;
                                    nat_add(a764, a765)?
                                };
                                let a799 = {
                                    let a796 = {
                                        let a794 = {
                                            let a766 = 1u64;
                                            let a767 = 1u64;
                                            nat_add(a766, a767)?
                                        };
                                        let a795 = {
                                            let a792 = {
                                                let a790 = {
                                                    let a768 = 1u64;
                                                    let a769 = 1u64;
                                                    nat_add(a768, a769)?
                                                };
                                                let a791 = {
                                                    let a788 = {
                                                        let a786 = {
                                                            let a770 = 1u64;
                                                            let a771 = 1u64;
                                                            nat_add(a770, a771)?
                                                        };
                                                        let a787 = {
                                                            let a784 = {
                                                                let a782 = {
                                                                    let a772 = 1u64;
                                                                    let a773 = 1u64;
                                                                    nat_add(a772, a773)?
                                                                };
                                                                let a783 = {
                                                                    let a780 = {
                                                                        let a774 = 1u64;
                                                                        let a775 = 1u64;
                                                                        nat_add(a774, a775)?
                                                                    };
                                                                    let a781 = {
                                                                        let a778 = {
                                                                            let a776 = 1u64;
                                                                            let a777 = 1u64;
                                                                            nat_add(a776, a777)?
                                                                        };
                                                                        let a779 = 1u64;
                                                                        nat_mul(a778, a779)?
                                                                    };
                                                                    nat_mul(a780, a781)?
                                                                };
                                                                nat_mul(a782, a783)?
                                                            };
                                                            let a785 = 1u64;
                                                            nat_add(a784, a785)?
                                                        };
                                                        nat_mul(a786, a787)?
                                                    };
                                                    let a789 = 1u64;
                                                    nat_add(a788, a789)?
                                                };
                                                nat_mul(a790, a791)?
                                            };
                                            let a793 = 1u64;
                                            nat_add(a792, a793)?
                                        };
                                        nat_mul(a794, a795)?
                                    };
                                    let a797 = 1u64;
                                    nat_add(a796, a797)?
                                };
                                nat_mul(a798, a799)?
                            };
                            let a801 = 1u64;
                            nat_add(a800, a801)?
                        };
                        nat_mul(a802, a803)?
                    };
                    let a805 = 1u64;
                    nat_add(a804, a805)?
                };
                nat_mul(a806, a807)?
            };
            let a809 = 1u64;
            nat_add(a808, a809)?
        };
        nat_sub(a810, a811)
    }, f37({
        let a814 = match v2 {
            Adt2::C0(_, h812, _, _) => {
                h812
            }
        };
        let a815 = match v3 {
            Adt2::C0(_, h813, _, _) => {
                h813
            }
        };
        nat_mul(a814, a815)?
    }, {
        let a832 = {
            let a816 = 1u64;
            let a817 = 1u64;
            nat_add(a816, a817)?
        };
        let a833 = {
            let a830 = {
                let a818 = 1u64;
                let a819 = 1u64;
                nat_add(a818, a819)?
            };
            let a831 = {
                let a828 = {
                    let a826 = {
                        let a820 = 1u64;
                        let a821 = 1u64;
                        nat_add(a820, a821)?
                    };
                    let a827 = {
                        let a824 = {
                            let a822 = 1u64;
                            let a823 = 1u64;
                            nat_add(a822, a823)?
                        };
                        let a825 = 1u64;
                        nat_mul(a824, a825)?
                    };
                    nat_mul(a826, a827)?
                };
                let a829 = 1u64;
                nat_add(a828, a829)?
            };
            nat_mul(a830, a831)?
        };
        nat_mul(a832, a833)?
    })?)
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h834, _) = f38(Fn1::F39, {
        let a853 = {
            let a835 = 1u64;
            let a836 = 1u64;
            nat_add(a835, a836)?
        };
        let a854 = {
            let a851 = {
                let a837 = 1u64;
                let a838 = 1u64;
                nat_add(a837, a838)?
            };
            let a852 = {
                let a849 = {
                    let a839 = 1u64;
                    let a840 = 1u64;
                    nat_add(a839, a840)?
                };
                let a850 = {
                    let a847 = {
                        let a841 = 1u64;
                        let a842 = 1u64;
                        nat_add(a841, a842)?
                    };
                    let a848 = {
                        let a845 = {
                            let a843 = 1u64;
                            let a844 = 1u64;
                            nat_add(a843, a844)?
                        };
                        let a846 = 1u64;
                        nat_mul(a845, a846)?
                    };
                    nat_mul(a847, a848)?
                };
                nat_mul(a849, a850)?
            };
            nat_mul(a851, a852)?
        };
        nat_mul(a853, a854)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h834)
}

pub fn f25(v0: u64) -> R<u64> {
    let a879 = v0;
    let a880 = f19({
        let a877 = {
            let a875 = {
                let a855 = 1u64;
                let a856 = 1u64;
                nat_add(a855, a856)?
            };
            let a876 = {
                let a873 = {
                    let a871 = {
                        let a857 = 1u64;
                        let a858 = 1u64;
                        nat_add(a857, a858)?
                    };
                    let a872 = {
                        let a869 = {
                            let a867 = {
                                let a859 = 1u64;
                                let a860 = 1u64;
                                nat_add(a859, a860)?
                            };
                            let a868 = {
                                let a865 = {
                                    let a863 = {
                                        let a861 = 1u64;
                                        let a862 = 1u64;
                                        nat_add(a861, a862)?
                                    };
                                    let a864 = 1u64;
                                    nat_mul(a863, a864)?
                                };
                                let a866 = 1u64;
                                nat_add(a865, a866)?
                            };
                            nat_mul(a867, a868)?
                        };
                        let a870 = 1u64;
                        nat_add(a869, a870)?
                    };
                    nat_mul(a871, a872)?
                };
                let a874 = 1u64;
                nat_add(a873, a874)?
            };
            nat_mul(a875, a876)?
        };
        let a878 = 1u64;
        nat_add(a877, a878)?
    })?;
    let a881 = 0u64;
    Ok(nat_quot(a879, a880, a881))
}

pub fn f26(v0: u64) -> R<u64> {
    let a906 = v0;
    let a907 = f19({
        let a904 = {
            let a902 = {
                let a882 = 1u64;
                let a883 = 1u64;
                nat_add(a882, a883)?
            };
            let a903 = {
                let a900 = {
                    let a898 = {
                        let a884 = 1u64;
                        let a885 = 1u64;
                        nat_add(a884, a885)?
                    };
                    let a899 = {
                        let a896 = {
                            let a894 = {
                                let a886 = 1u64;
                                let a887 = 1u64;
                                nat_add(a886, a887)?
                            };
                            let a895 = {
                                let a892 = {
                                    let a890 = {
                                        let a888 = 1u64;
                                        let a889 = 1u64;
                                        nat_add(a888, a889)?
                                    };
                                    let a891 = 1u64;
                                    nat_mul(a890, a891)?
                                };
                                let a893 = 1u64;
                                nat_add(a892, a893)?
                            };
                            nat_mul(a894, a895)?
                        };
                        let a897 = 1u64;
                        nat_add(a896, a897)?
                    };
                    nat_mul(a898, a899)?
                };
                let a901 = 1u64;
                nat_add(a900, a901)?
            };
            nat_mul(a902, a903)?
        };
        let a905 = 1u64;
        nat_add(a904, a905)?
    })?;
    let a908 = 0u64;
    Ok(nat_rem(a906, a907, a908))
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f40(v0)?;
    let v3: u64 = {
        let a921 = f32(v0)?;
        let a922 = {
            let a919 = {
                let a909 = 1u64;
                let a910 = 1u64;
                nat_add(a909, a910)?
            };
            let a920 = {
                let a917 = {
                    let a911 = 1u64;
                    let a912 = 1u64;
                    nat_add(a911, a912)?
                };
                let a918 = {
                    let a915 = {
                        let a913 = 1u64;
                        let a914 = 1u64;
                        nat_add(a913, a914)?
                    };
                    let a916 = 1u64;
                    nat_mul(a915, a916)?
                };
                nat_mul(a917, a918)?
            };
            nat_mul(a919, a920)?
        };
        nat_mul(a921, a922)?
    };
    let v4: u64 = f37({
        let a935 = f32(v1)?;
        let a936 = {
            let a933 = {
                let a923 = 1u64;
                let a924 = 1u64;
                nat_add(a923, a924)?
            };
            let a934 = {
                let a931 = {
                    let a925 = 1u64;
                    let a926 = 1u64;
                    nat_add(a925, a926)?
                };
                let a932 = {
                    let a929 = {
                        let a927 = 1u64;
                        let a928 = 1u64;
                        nat_add(a927, a928)?
                    };
                    let a930 = 1u64;
                    nat_mul(a929, a930)?
                };
                nat_mul(a931, a932)?
            };
            nat_mul(a933, a934)?
        };
        nat_mul(a935, a936)?
    }, {
        let a937 = v2;
        let a938 = f40(v1)?;
        nat_sub(a937, a938)
    })?;
    let a939 = f25(v0)?;
    let a940 = f25(v1)?;
    if nat_eq(a939, a940) {
        f36(f25(v0)?, v2, {
            let a941 = v3;
            let a942 = v4;
            nat_add(a941, a942)?
        })
    } else {
        let a943 = v3;
        let a944 = v4;
        if nat_eq(a943, a944) {
            Ok(0u64)
        } else {
            f36(f25(v0)?, v2, {
                let a945 = v3;
                let a946 = v4;
                nat_sub(a945, a946)
            })
        }
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    if if if if f16(v0)? {
        true
    } else {
        f16(v1)?
    } {
        true
    } else {
        if f17(v0)? {
            f17(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f18(v0)? {
            f18(v1)?
        } else {
            false
        }
    } {
        let a1033 = {
            let a1011 = {
                let a987 = {
                    let a985 = {
                        let a947 = 1u64;
                        let a948 = 1u64;
                        nat_add(a947, a948)?
                    };
                    let a986 = {
                        let a983 = {
                            let a981 = {
                                let a949 = 1u64;
                                let a950 = 1u64;
                                nat_add(a949, a950)?
                            };
                            let a982 = {
                                let a979 = {
                                    let a977 = {
                                        let a951 = 1u64;
                                        let a952 = 1u64;
                                        nat_add(a951, a952)?
                                    };
                                    let a978 = {
                                        let a975 = {
                                            let a973 = {
                                                let a953 = 1u64;
                                                let a954 = 1u64;
                                                nat_add(a953, a954)?
                                            };
                                            let a974 = {
                                                let a971 = {
                                                    let a969 = {
                                                        let a955 = 1u64;
                                                        let a956 = 1u64;
                                                        nat_add(a955, a956)?
                                                    };
                                                    let a970 = {
                                                        let a967 = {
                                                            let a965 = {
                                                                let a957 = 1u64;
                                                                let a958 = 1u64;
                                                                nat_add(a957, a958)?
                                                            };
                                                            let a966 = {
                                                                let a963 = {
                                                                    let a961 = {
                                                                        let a959 = 1u64;
                                                                        let a960 = 1u64;
                                                                        nat_add(a959, a960)?
                                                                    };
                                                                    let a962 = 1u64;
                                                                    nat_mul(a961, a962)?
                                                                };
                                                                let a964 = 1u64;
                                                                nat_add(a963, a964)?
                                                            };
                                                            nat_mul(a965, a966)?
                                                        };
                                                        let a968 = 1u64;
                                                        nat_add(a967, a968)?
                                                    };
                                                    nat_mul(a969, a970)?
                                                };
                                                let a972 = 1u64;
                                                nat_add(a971, a972)?
                                            };
                                            nat_mul(a973, a974)?
                                        };
                                        let a976 = 1u64;
                                        nat_add(a975, a976)?
                                    };
                                    nat_mul(a977, a978)?
                                };
                                let a980 = 1u64;
                                nat_add(a979, a980)?
                            };
                            nat_mul(a981, a982)?
                        };
                        let a984 = 1u64;
                        nat_add(a983, a984)?
                    };
                    nat_mul(a985, a986)?
                };
                let a988 = 1u64;
                nat_add(a987, a988)?
            };
            let a1012 = f19({
                let a1009 = {
                    let a1007 = {
                        let a989 = 1u64;
                        let a990 = 1u64;
                        nat_add(a989, a990)?
                    };
                    let a1008 = {
                        let a1005 = {
                            let a1003 = {
                                let a991 = 1u64;
                                let a992 = 1u64;
                                nat_add(a991, a992)?
                            };
                            let a1004 = {
                                let a1001 = {
                                    let a999 = {
                                        let a993 = 1u64;
                                        let a994 = 1u64;
                                        nat_add(a993, a994)?
                                    };
                                    let a1000 = {
                                        let a997 = {
                                            let a995 = 1u64;
                                            let a996 = 1u64;
                                            nat_add(a995, a996)?
                                        };
                                        let a998 = 1u64;
                                        nat_mul(a997, a998)?
                                    };
                                    nat_mul(a999, a1000)?
                                };
                                let a1002 = 1u64;
                                nat_add(a1001, a1002)?
                            };
                            nat_mul(a1003, a1004)?
                        };
                        let a1006 = 1u64;
                        nat_add(a1005, a1006)?
                    };
                    nat_mul(a1007, a1008)?
                };
                let a1010 = 1u64;
                nat_add(a1009, a1010)?
            })?;
            nat_mul(a1011, a1012)?
        };
        let a1034 = f19({
            let a1031 = {
                let a1013 = 1u64;
                let a1014 = 1u64;
                nat_add(a1013, a1014)?
            };
            let a1032 = {
                let a1029 = {
                    let a1027 = {
                        let a1015 = 1u64;
                        let a1016 = 1u64;
                        nat_add(a1015, a1016)?
                    };
                    let a1028 = {
                        let a1025 = {
                            let a1023 = {
                                let a1017 = 1u64;
                                let a1018 = 1u64;
                                nat_add(a1017, a1018)?
                            };
                            let a1024 = {
                                let a1021 = {
                                    let a1019 = 1u64;
                                    let a1020 = 1u64;
                                    nat_add(a1019, a1020)?
                                };
                                let a1022 = 1u64;
                                nat_mul(a1021, a1022)?
                            };
                            nat_mul(a1023, a1024)?
                        };
                        let a1026 = 1u64;
                        nat_add(a1025, a1026)?
                    };
                    nat_mul(a1027, a1028)?
                };
                let a1030 = 1u64;
                nat_add(a1029, a1030)?
            };
            nat_mul(a1031, a1032)?
        })?;
        nat_add(a1033, a1034)
    } else {
        if if f17(v0)? {
            true
        } else {
            f18(v1)?
        } {
            f20(f21(v0, v1)?)
        } else {
            if if f17(v1)? {
                true
            } else {
                f18(v0)?
            } {
                f22(f21(v0, v1)?)
            } else {
                f41(v0, v1)
            }
        }
    }
}

pub fn f29(v0: u64) -> R<u64> {
    let a1057 = f26(v0)?;
    let a1058 = f19({
        let a1055 = {
            let a1053 = {
                let a1035 = 1u64;
                let a1036 = 1u64;
                nat_add(a1035, a1036)?
            };
            let a1054 = {
                let a1051 = {
                    let a1049 = {
                        let a1037 = 1u64;
                        let a1038 = 1u64;
                        nat_add(a1037, a1038)?
                    };
                    let a1050 = {
                        let a1047 = {
                            let a1045 = {
                                let a1039 = 1u64;
                                let a1040 = 1u64;
                                nat_add(a1039, a1040)?
                            };
                            let a1046 = {
                                let a1043 = {
                                    let a1041 = 1u64;
                                    let a1042 = 1u64;
                                    nat_add(a1041, a1042)?
                                };
                                let a1044 = 1u64;
                                nat_mul(a1043, a1044)?
                            };
                            nat_mul(a1045, a1046)?
                        };
                        let a1048 = 1u64;
                        nat_add(a1047, a1048)?
                    };
                    nat_mul(a1049, a1050)?
                };
                let a1052 = 1u64;
                nat_add(a1051, a1052)?
            };
            nat_mul(a1053, a1054)?
        };
        let a1056 = 1u64;
        nat_add(a1055, a1056)?
    })?;
    let a1059 = 0u64;
    Ok(nat_quot(a1057, a1058, a1059))
}

pub fn f30(v0: u64) -> R<u64> {
    let a1082 = v0;
    let a1083 = f19({
        let a1080 = {
            let a1078 = {
                let a1060 = 1u64;
                let a1061 = 1u64;
                nat_add(a1060, a1061)?
            };
            let a1079 = {
                let a1076 = {
                    let a1074 = {
                        let a1062 = 1u64;
                        let a1063 = 1u64;
                        nat_add(a1062, a1063)?
                    };
                    let a1075 = {
                        let a1072 = {
                            let a1070 = {
                                let a1064 = 1u64;
                                let a1065 = 1u64;
                                nat_add(a1064, a1065)?
                            };
                            let a1071 = {
                                let a1068 = {
                                    let a1066 = 1u64;
                                    let a1067 = 1u64;
                                    nat_add(a1066, a1067)?
                                };
                                let a1069 = 1u64;
                                nat_mul(a1068, a1069)?
                            };
                            nat_mul(a1070, a1071)?
                        };
                        let a1073 = 1u64;
                        nat_add(a1072, a1073)?
                    };
                    nat_mul(a1074, a1075)?
                };
                let a1077 = 1u64;
                nat_add(a1076, a1077)?
            };
            nat_mul(a1078, a1079)?
        };
        let a1081 = 1u64;
        nat_add(a1080, a1081)?
    })?;
    let a1084 = 0u64;
    Ok(nat_rem(a1082, a1083, a1084))
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1085 = v0;
        let a1086 = v1;
        let a1087 = 0u64;
        nat_quot(a1085, a1086, a1087)
    };
    let v3: u64 = {
        let a1091 = {
            let a1088 = v0;
            let a1089 = v1;
            let a1090 = 0u64;
            nat_rem(a1088, a1089, a1090)
        };
        let a1092 = 2u64;
        nat_mul(a1091, a1092)?
    };
    let a1102 = v2;
    let a1103 = {
        let a1093 = v1;
        let a1094 = v3;
        if if nat_lt(a1093, a1094) {
            true
        } else {
            let a1095 = v3;
            let a1096 = v1;
            if nat_eq(a1095, a1096) {
                let a1100 = {
                    let a1097 = v2;
                    let a1098 = 2u64;
                    let a1099 = 0u64;
                    nat_rem(a1097, a1098, a1099)
                };
                let a1101 = 1u64;
                nat_eq(a1100, a1101)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1102, a1103)
}

pub fn f32(v0: u64) -> R<u64> {
    let a1128 = f30(v0)?;
    let a1129 = {
        let a1104 = f29(v0)?;
        let a1105 = 0u64;
        if nat_eq(a1104, a1105) {
            0u64
        } else {
            f19({
                let a1126 = {
                    let a1124 = {
                        let a1106 = 1u64;
                        let a1107 = 1u64;
                        nat_add(a1106, a1107)?
                    };
                    let a1125 = {
                        let a1122 = {
                            let a1120 = {
                                let a1108 = 1u64;
                                let a1109 = 1u64;
                                nat_add(a1108, a1109)?
                            };
                            let a1121 = {
                                let a1118 = {
                                    let a1116 = {
                                        let a1110 = 1u64;
                                        let a1111 = 1u64;
                                        nat_add(a1110, a1111)?
                                    };
                                    let a1117 = {
                                        let a1114 = {
                                            let a1112 = 1u64;
                                            let a1113 = 1u64;
                                            nat_add(a1112, a1113)?
                                        };
                                        let a1115 = 1u64;
                                        nat_mul(a1114, a1115)?
                                    };
                                    nat_mul(a1116, a1117)?
                                };
                                let a1119 = 1u64;
                                nat_add(a1118, a1119)?
                            };
                            nat_mul(a1120, a1121)?
                        };
                        let a1123 = 1u64;
                        nat_add(a1122, a1123)?
                    };
                    nat_mul(a1124, a1125)?
                };
                let a1127 = 1u64;
                nat_add(a1126, a1127)?
            })?
        }
    };
    nat_add(a1128, a1129)
}

pub fn f33(v0: Fn0, p1133: u64, p1134: u64) -> R<u64> {
    let m1132 = (p1133, p1134);
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v1, v2) = m1132;
                let m1130 = v1;
                if m1130 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1130 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1131: Fn0 = v0.clone();
                        c1131.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: u64) -> R<u64> {
    let a1135 = v0;
    let a1136 = v0;
    nat_add(a1135, a1136)
}

pub fn f35(v0: u64) -> R<Adt2> {
    f24(f40(v0)?, f32(v0)?, 0u64, f19({
        let a1157 = {
            let a1155 = {
                let a1137 = 1u64;
                let a1138 = 1u64;
                nat_add(a1137, a1138)?
            };
            let a1156 = {
                let a1153 = {
                    let a1151 = {
                        let a1139 = 1u64;
                        let a1140 = 1u64;
                        nat_add(a1139, a1140)?
                    };
                    let a1152 = {
                        let a1149 = {
                            let a1147 = {
                                let a1141 = 1u64;
                                let a1142 = 1u64;
                                nat_add(a1141, a1142)?
                            };
                            let a1148 = {
                                let a1145 = {
                                    let a1143 = 1u64;
                                    let a1144 = 1u64;
                                    nat_add(a1143, a1144)?
                                };
                                let a1146 = 1u64;
                                nat_mul(a1145, a1146)?
                            };
                            nat_mul(a1147, a1148)?
                        };
                        let a1150 = 1u64;
                        nat_add(a1149, a1150)?
                    };
                    nat_mul(a1151, a1152)?
                };
                let a1154 = 1u64;
                nat_add(a1153, a1154)?
            };
            nat_mul(a1155, a1156)?
        };
        let a1158 = 1u64;
        nat_add(a1157, a1158)?
    })?)
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1181 = f19({
        let a1179 = {
            let a1177 = {
                let a1159 = 1u64;
                let a1160 = 1u64;
                nat_add(a1159, a1160)?
            };
            let a1178 = {
                let a1175 = {
                    let a1173 = {
                        let a1161 = 1u64;
                        let a1162 = 1u64;
                        nat_add(a1161, a1162)?
                    };
                    let a1174 = {
                        let a1171 = {
                            let a1163 = 1u64;
                            let a1164 = 1u64;
                            nat_add(a1163, a1164)?
                        };
                        let a1172 = {
                            let a1169 = {
                                let a1167 = {
                                    let a1165 = 1u64;
                                    let a1166 = 1u64;
                                    nat_add(a1165, a1166)?
                                };
                                let a1168 = 1u64;
                                nat_mul(a1167, a1168)?
                            };
                            let a1170 = 1u64;
                            nat_add(a1169, a1170)?
                        };
                        nat_mul(a1171, a1172)?
                    };
                    nat_mul(a1173, a1174)?
                };
                let a1176 = 1u64;
                nat_add(a1175, a1176)?
            };
            nat_mul(a1177, a1178)?
        };
        let a1180 = 1u64;
        nat_add(a1179, a1180)?
    })?;
    let a1182 = v2;
    if nat_le(a1181, a1182) {
        f42(v0, {
            let a1183 = v1;
            let a1184 = 1u64;
            nat_add(a1183, a1184)?
        }, f37(v2, 1u64)?)
    } else {
        let v3: Adt2 = f24(v1, v2, {
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
        }, f19({
            let a1241 = {
                let a1223 = 1u64;
                let a1224 = 1u64;
                nat_add(a1223, a1224)?
            };
            let a1242 = {
                let a1239 = {
                    let a1237 = {
                        let a1225 = 1u64;
                        let a1226 = 1u64;
                        nat_add(a1225, a1226)?
                    };
                    let a1238 = {
                        let a1235 = {
                            let a1227 = 1u64;
                            let a1228 = 1u64;
                            nat_add(a1227, a1228)?
                        };
                        let a1236 = {
                            let a1233 = {
                                let a1231 = {
                                    let a1229 = 1u64;
                                    let a1230 = 1u64;
                                    nat_add(a1229, a1230)?
                                };
                                let a1232 = 1u64;
                                nat_mul(a1231, a1232)?
                            };
                            let a1234 = 1u64;
                            nat_add(a1233, a1234)?
                        };
                        nat_mul(a1235, a1236)?
                    };
                    nat_mul(a1237, a1238)?
                };
                let a1240 = 1u64;
                nat_add(a1239, a1240)?
            };
            nat_mul(a1241, a1242)?
        })?)?;
        f42(v0, match v3.clone() {
            Adt2::C0(h1243, _, _, _) => {
                h1243
            }
        }, match v3 {
            Adt2::C0(_, h1244, _, _) => {
                h1244
            }
        })
    }
}

pub fn f37(v0: u64, v1: u64) -> R<u64> {
    let a1245 = v1;
    let a1246 = 0u64;
    if nat_eq(a1245, a1246) {
        Ok(v0)
    } else {
        let a1271 = {
            let a1269 = {
                let a1247 = 1u64;
                let a1248 = 1u64;
                nat_add(a1247, a1248)?
            };
            let a1270 = {
                let a1267 = {
                    let a1249 = 1u64;
                    let a1250 = 1u64;
                    nat_add(a1249, a1250)?
                };
                let a1268 = {
                    let a1265 = {
                        let a1251 = 1u64;
                        let a1252 = 1u64;
                        nat_add(a1251, a1252)?
                    };
                    let a1266 = {
                        let a1263 = {
                            let a1253 = 1u64;
                            let a1254 = 1u64;
                            nat_add(a1253, a1254)?
                        };
                        let a1264 = {
                            let a1261 = {
                                let a1255 = 1u64;
                                let a1256 = 1u64;
                                nat_add(a1255, a1256)?
                            };
                            let a1262 = {
                                let a1259 = {
                                    let a1257 = 1u64;
                                    let a1258 = 1u64;
                                    nat_add(a1257, a1258)?
                                };
                                let a1260 = 1u64;
                                nat_mul(a1259, a1260)?
                            };
                            nat_mul(a1261, a1262)?
                        };
                        nat_mul(a1263, a1264)?
                    };
                    nat_mul(a1265, a1266)?
                };
                nat_mul(a1267, a1268)?
            };
            nat_mul(a1269, a1270)?
        };
        let a1272 = v1;
        if nat_le(a1271, a1272) {
            let a1273 = v0;
            let a1274 = 0u64;
            if nat_eq(a1273, a1274) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f19(v1)?;
            f43({
                let a1275 = v0;
                let a1276 = v2;
                let a1277 = 0u64;
                nat_quot(a1275, a1276, a1277)
            }, {
                let a1281 = 0u64;
                let a1282 = {
                    let a1278 = v0;
                    let a1279 = v2;
                    let a1280 = 0u64;
                    nat_rem(a1278, a1279, a1280)
                };
                nat_lt(a1281, a1282)
            })
        }
    }
}

pub fn f38(v0: Fn1, p1287: u64, p1288: Adt2) -> R<(Adt2, bool)> {
    let m1286 = (p1287, p1288);
    {
        let mut m1286 = m1286;
        loop {
            let step = {
                let (v1, v2) = m1286;
                let m1283 = v1;
                if m1283 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1283 - 1;
                    let m1285 = {
                        let c1284: Fn1 = v0.clone();
                        c1284.apply(v2.clone())?
                    };
                    match m1285 {
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
                Ok(next) => { m1286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Adt2) -> R<Option<Adt2>> {
    f44(v0)
}

pub fn f40(v0: u64) -> R<u64> {
    let a1327 = {
        let a1289 = f29(v0)?;
        let a1290 = 0u64;
        if nat_eq(a1289, a1290) {
            1u64
        } else {
            f29(v0)?
        }
    };
    let a1328 = {
        let a1325 = {
            let a1291 = 1u64;
            let a1292 = 1u64;
            nat_add(a1291, a1292)?
        };
        let a1326 = {
            let a1323 = {
                let a1293 = 1u64;
                let a1294 = 1u64;
                nat_add(a1293, a1294)?
            };
            let a1324 = {
                let a1321 = {
                    let a1295 = 1u64;
                    let a1296 = 1u64;
                    nat_add(a1295, a1296)?
                };
                let a1322 = {
                    let a1319 = {
                        let a1297 = 1u64;
                        let a1298 = 1u64;
                        nat_add(a1297, a1298)?
                    };
                    let a1320 = {
                        let a1317 = {
                            let a1299 = 1u64;
                            let a1300 = 1u64;
                            nat_add(a1299, a1300)?
                        };
                        let a1318 = {
                            let a1315 = {
                                let a1301 = 1u64;
                                let a1302 = 1u64;
                                nat_add(a1301, a1302)?
                            };
                            let a1316 = {
                                let a1313 = {
                                    let a1303 = 1u64;
                                    let a1304 = 1u64;
                                    nat_add(a1303, a1304)?
                                };
                                let a1314 = {
                                    let a1311 = {
                                        let a1305 = 1u64;
                                        let a1306 = 1u64;
                                        nat_add(a1305, a1306)?
                                    };
                                    let a1312 = {
                                        let a1309 = {
                                            let a1307 = 1u64;
                                            let a1308 = 1u64;
                                            nat_add(a1307, a1308)?
                                        };
                                        let a1310 = 1u64;
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
                nat_mul(a1321, a1322)?
            };
            nat_mul(a1323, a1324)?
        };
        nat_mul(a1325, a1326)?
    };
    nat_add(a1327, a1328)
}

pub fn f41(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f35(v0)?;
    let v3: Adt2 = f35(v1)?;
    let v4: bool = {
        let a1331 = match v2.clone() {
            Adt2::C0(_, h1329, _, _) => {
                h1329
            }
        };
        let a1332 = match v3.clone() {
            Adt2::C0(_, h1330, _, _) => {
                h1330
            }
        };
        nat_lt(a1331, a1332)
    };
    let v5: u64 = {
        let a1376 = match v2.clone() {
            Adt2::C0(_, h1333, _, _) => {
                h1333
            }
        };
        let a1377 = f19(if v4 {
            let a1354 = {
                let a1352 = {
                    let a1334 = 1u64;
                    let a1335 = 1u64;
                    nat_add(a1334, a1335)?
                };
                let a1353 = {
                    let a1350 = {
                        let a1348 = {
                            let a1336 = 1u64;
                            let a1337 = 1u64;
                            nat_add(a1336, a1337)?
                        };
                        let a1349 = {
                            let a1346 = {
                                let a1338 = 1u64;
                                let a1339 = 1u64;
                                nat_add(a1338, a1339)?
                            };
                            let a1347 = {
                                let a1344 = {
                                    let a1342 = {
                                        let a1340 = 1u64;
                                        let a1341 = 1u64;
                                        nat_add(a1340, a1341)?
                                    };
                                    let a1343 = 1u64;
                                    nat_mul(a1342, a1343)?
                                };
                                let a1345 = 1u64;
                                nat_add(a1344, a1345)?
                            };
                            nat_mul(a1346, a1347)?
                        };
                        nat_mul(a1348, a1349)?
                    };
                    let a1351 = 1u64;
                    nat_add(a1350, a1351)?
                };
                nat_mul(a1352, a1353)?
            };
            let a1355 = 1u64;
            nat_add(a1354, a1355)?
        } else {
            let a1374 = {
                let a1356 = 1u64;
                let a1357 = 1u64;
                nat_add(a1356, a1357)?
            };
            let a1375 = {
                let a1372 = {
                    let a1370 = {
                        let a1358 = 1u64;
                        let a1359 = 1u64;
                        nat_add(a1358, a1359)?
                    };
                    let a1371 = {
                        let a1368 = {
                            let a1360 = 1u64;
                            let a1361 = 1u64;
                            nat_add(a1360, a1361)?
                        };
                        let a1369 = {
                            let a1366 = {
                                let a1364 = {
                                    let a1362 = 1u64;
                                    let a1363 = 1u64;
                                    nat_add(a1362, a1363)?
                                };
                                let a1365 = 1u64;
                                nat_mul(a1364, a1365)?
                            };
                            let a1367 = 1u64;
                            nat_add(a1366, a1367)?
                        };
                        nat_mul(a1368, a1369)?
                    };
                    nat_mul(a1370, a1371)?
                };
                let a1373 = 1u64;
                nat_add(a1372, a1373)?
            };
            nat_mul(a1374, a1375)?
        })?;
        nat_mul(a1376, a1377)?
    };
    let v6: u64 = {
        let a1434 = {
            let a1432 = {
                let a1429 = match v2 {
                    Adt2::C0(h1378, _, _, _) => {
                        h1378
                    }
                };
                let a1430 = {
                    let a1427 = {
                        let a1425 = {
                            let a1379 = 1u64;
                            let a1380 = 1u64;
                            nat_add(a1379, a1380)?
                        };
                        let a1426 = {
                            let a1423 = {
                                let a1421 = {
                                    let a1381 = 1u64;
                                    let a1382 = 1u64;
                                    nat_add(a1381, a1382)?
                                };
                                let a1422 = {
                                    let a1419 = {
                                        let a1417 = {
                                            let a1383 = 1u64;
                                            let a1384 = 1u64;
                                            nat_add(a1383, a1384)?
                                        };
                                        let a1418 = {
                                            let a1415 = {
                                                let a1413 = {
                                                    let a1385 = 1u64;
                                                    let a1386 = 1u64;
                                                    nat_add(a1385, a1386)?
                                                };
                                                let a1414 = {
                                                    let a1411 = {
                                                        let a1409 = {
                                                            let a1387 = 1u64;
                                                            let a1388 = 1u64;
                                                            nat_add(a1387, a1388)?
                                                        };
                                                        let a1410 = {
                                                            let a1407 = {
                                                                let a1405 = {
                                                                    let a1389 = 1u64;
                                                                    let a1390 = 1u64;
                                                                    nat_add(a1389, a1390)?
                                                                };
                                                                let a1406 = {
                                                                    let a1403 = {
                                                                        let a1401 = {
                                                                            let a1391 = 1u64;
                                                                            let a1392 = 1u64;
                                                                            nat_add(a1391, a1392)?
                                                                        };
                                                                        let a1402 = {
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
                                                                        nat_mul(a1401, a1402)?
                                                                    };
                                                                    let a1404 = 1u64;
                                                                    nat_add(a1403, a1404)?
                                                                };
                                                                nat_mul(a1405, a1406)?
                                                            };
                                                            let a1408 = 1u64;
                                                            nat_add(a1407, a1408)?
                                                        };
                                                        nat_mul(a1409, a1410)?
                                                    };
                                                    let a1412 = 1u64;
                                                    nat_add(a1411, a1412)?
                                                };
                                                nat_mul(a1413, a1414)?
                                            };
                                            let a1416 = 1u64;
                                            nat_add(a1415, a1416)?
                                        };
                                        nat_mul(a1417, a1418)?
                                    };
                                    let a1420 = 1u64;
                                    nat_add(a1419, a1420)?
                                };
                                nat_mul(a1421, a1422)?
                            };
                            let a1424 = 1u64;
                            nat_add(a1423, a1424)?
                        };
                        nat_mul(a1425, a1426)?
                    };
                    let a1428 = 1u64;
                    nat_add(a1427, a1428)?
                };
                nat_add(a1429, a1430)?
            };
            let a1433 = match v3.clone() {
                Adt2::C0(h1431, _, _, _) => {
                    h1431
                }
            };
            nat_sub(a1432, a1433)
        };
        let a1435 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a1434, a1435)
    };
    let v7: u64 = {
        let a1437 = v5;
        let a1438 = match v3.clone() {
            Adt2::C0(_, h1436, _, _) => {
                h1436
            }
        };
        let a1439 = 0u64;
        nat_quot(a1437, a1438, a1439)
    };
    let v8: bool = {
        let a1444 = 0u64;
        let a1445 = {
            let a1441 = v5;
            let a1442 = match v3 {
                Adt2::C0(_, h1440, _, _) => {
                    h1440
                }
            };
            let a1443 = 0u64;
            nat_rem(a1441, a1442, a1443)
        };
        nat_lt(a1444, a1445)
    };
    f36(f21(v0, v1)?, v6, f43(v7, v8)?)
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1484 = v1;
    let a1485 = {
        let a1482 = {
            let a1480 = {
                let a1446 = 1u64;
                let a1447 = 1u64;
                nat_add(a1446, a1447)?
            };
            let a1481 = {
                let a1478 = {
                    let a1448 = 1u64;
                    let a1449 = 1u64;
                    nat_add(a1448, a1449)?
                };
                let a1479 = {
                    let a1476 = {
                        let a1450 = 1u64;
                        let a1451 = 1u64;
                        nat_add(a1450, a1451)?
                    };
                    let a1477 = {
                        let a1474 = {
                            let a1452 = 1u64;
                            let a1453 = 1u64;
                            nat_add(a1452, a1453)?
                        };
                        let a1475 = {
                            let a1472 = {
                                let a1454 = 1u64;
                                let a1455 = 1u64;
                                nat_add(a1454, a1455)?
                            };
                            let a1473 = {
                                let a1470 = {
                                    let a1456 = 1u64;
                                    let a1457 = 1u64;
                                    nat_add(a1456, a1457)?
                                };
                                let a1471 = {
                                    let a1468 = {
                                        let a1458 = 1u64;
                                        let a1459 = 1u64;
                                        nat_add(a1458, a1459)?
                                    };
                                    let a1469 = {
                                        let a1466 = {
                                            let a1460 = 1u64;
                                            let a1461 = 1u64;
                                            nat_add(a1460, a1461)?
                                        };
                                        let a1467 = {
                                            let a1464 = {
                                                let a1462 = 1u64;
                                                let a1463 = 1u64;
                                                nat_add(a1462, a1463)?
                                            };
                                            let a1465 = 1u64;
                                            nat_mul(a1464, a1465)?
                                        };
                                        nat_mul(a1466, a1467)?
                                    };
                                    nat_mul(a1468, a1469)?
                                };
                                nat_mul(a1470, a1471)?
                            };
                            nat_mul(a1472, a1473)?
                        };
                        nat_mul(a1474, a1475)?
                    };
                    nat_mul(a1476, a1477)?
                };
                nat_mul(a1478, a1479)?
            };
            nat_mul(a1480, a1481)?
        };
        let a1483 = 1u64;
        nat_add(a1482, a1483)?
    };
    if nat_lt(a1484, a1485) {
        f45(v0, {
            let a1522 = {
                let a1520 = {
                    let a1486 = 1u64;
                    let a1487 = 1u64;
                    nat_add(a1486, a1487)?
                };
                let a1521 = {
                    let a1518 = {
                        let a1488 = 1u64;
                        let a1489 = 1u64;
                        nat_add(a1488, a1489)?
                    };
                    let a1519 = {
                        let a1516 = {
                            let a1490 = 1u64;
                            let a1491 = 1u64;
                            nat_add(a1490, a1491)?
                        };
                        let a1517 = {
                            let a1514 = {
                                let a1492 = 1u64;
                                let a1493 = 1u64;
                                nat_add(a1492, a1493)?
                            };
                            let a1515 = {
                                let a1512 = {
                                    let a1494 = 1u64;
                                    let a1495 = 1u64;
                                    nat_add(a1494, a1495)?
                                };
                                let a1513 = {
                                    let a1510 = {
                                        let a1496 = 1u64;
                                        let a1497 = 1u64;
                                        nat_add(a1496, a1497)?
                                    };
                                    let a1511 = {
                                        let a1508 = {
                                            let a1498 = 1u64;
                                            let a1499 = 1u64;
                                            nat_add(a1498, a1499)?
                                        };
                                        let a1509 = {
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
            let a1523 = 1u64;
            nat_add(a1522, a1523)?
        }, f37(v2, {
            let a1562 = {
                let a1560 = {
                    let a1558 = {
                        let a1524 = 1u64;
                        let a1525 = 1u64;
                        nat_add(a1524, a1525)?
                    };
                    let a1559 = {
                        let a1556 = {
                            let a1526 = 1u64;
                            let a1527 = 1u64;
                            nat_add(a1526, a1527)?
                        };
                        let a1557 = {
                            let a1554 = {
                                let a1528 = 1u64;
                                let a1529 = 1u64;
                                nat_add(a1528, a1529)?
                            };
                            let a1555 = {
                                let a1552 = {
                                    let a1530 = 1u64;
                                    let a1531 = 1u64;
                                    nat_add(a1530, a1531)?
                                };
                                let a1553 = {
                                    let a1550 = {
                                        let a1532 = 1u64;
                                        let a1533 = 1u64;
                                        nat_add(a1532, a1533)?
                                    };
                                    let a1551 = {
                                        let a1548 = {
                                            let a1534 = 1u64;
                                            let a1535 = 1u64;
                                            nat_add(a1534, a1535)?
                                        };
                                        let a1549 = {
                                            let a1546 = {
                                                let a1536 = 1u64;
                                                let a1537 = 1u64;
                                                nat_add(a1536, a1537)?
                                            };
                                            let a1547 = {
                                                let a1544 = {
                                                    let a1538 = 1u64;
                                                    let a1539 = 1u64;
                                                    nat_add(a1538, a1539)?
                                                };
                                                let a1545 = {
                                                    let a1542 = {
                                                        let a1540 = 1u64;
                                                        let a1541 = 1u64;
                                                        nat_add(a1540, a1541)?
                                                    };
                                                    let a1543 = 1u64;
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
                        nat_mul(a1556, a1557)?
                    };
                    nat_mul(a1558, a1559)?
                };
                let a1561 = 1u64;
                nat_add(a1560, a1561)?
            };
            let a1563 = v1;
            nat_sub(a1562, a1563)
        })?)
    } else {
        f45(v0, v1, v2)
    }
}

pub fn f43(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1571 = {
            let a1568 = v0;
            let a1569 = {
                let a1566 = {
                    let a1564 = 1u64;
                    let a1565 = 1u64;
                    nat_add(a1564, a1565)?
                };
                let a1567 = 1u64;
                nat_mul(a1566, a1567)?
            };
            let a1570 = 0u64;
            nat_rem(a1568, a1569, a1570)
        };
        let a1572 = 0u64;
        nat_eq(a1571, a1572)
    } else {
        false
    } {
        let a1573 = v0;
        let a1574 = 1u64;
        nat_add(a1573, a1574)
    } else {
        Ok(v0)
    }
}

pub fn f44(v0: Adt2) -> R<Option<Adt2>> {
    let a1576 = 0u64;
    let a1577 = match v0.clone() {
        Adt2::C0(_, h1575, _, _) => {
            h1575
        }
    };
    if if if nat_lt(a1576, a1577) {
        let a1580 = match v0.clone() {
            Adt2::C0(_, h1578, _, _) => {
                h1578
            }
        };
        let a1581 = match v0.clone() {
            Adt2::C0(_, _, _, h1579) => {
                h1579
            }
        };
        nat_lt(a1580, a1581)
    } else {
        false
    } {
        let a1584 = match v0.clone() {
            Adt2::C0(_, _, h1582, _) => {
                h1582
            }
        };
        let a1585 = match v0.clone() {
            Adt2::C0(h1583, _, _, _) => {
                h1583
            }
        };
        nat_lt(a1584, a1585)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1587 = match v0.clone() {
                Adt2::C0(h1586, _, _, _) => {
                    h1586
                }
            };
            let a1588 = 1u64;
            nat_sub(a1587, a1588)
        }, {
            let a1591 = match v0.clone() {
                Adt2::C0(_, h1589, _, _) => {
                    h1589
                }
            };
            let a1592 = match v0.clone() {
                Adt2::C0(_, h1590, _, _) => {
                    h1590
                }
            };
            nat_add(a1591, a1592)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1593, _) => {
                h1593
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1594) => {
                h1594
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1607 = v2;
        let a1608 = {
            let a1605 = {
                let a1595 = 1u64;
                let a1596 = 1u64;
                nat_add(a1595, a1596)?
            };
            let a1606 = {
                let a1603 = {
                    let a1597 = 1u64;
                    let a1598 = 1u64;
                    nat_add(a1597, a1598)?
                };
                let a1604 = {
                    let a1601 = {
                        let a1599 = 1u64;
                        let a1600 = 1u64;
                        nat_add(a1599, a1600)?
                    };
                    let a1602 = 1u64;
                    nat_mul(a1601, a1602)?
                };
                nat_mul(a1603, a1604)?
            };
            nat_mul(a1605, a1606)?
        };
        let a1609 = 0u64;
        nat_rem(a1607, a1608, a1609)
    };
    let v4: u64 = {
        let a1622 = v2;
        let a1623 = {
            let a1620 = {
                let a1610 = 1u64;
                let a1611 = 1u64;
                nat_add(a1610, a1611)?
            };
            let a1621 = {
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
            nat_mul(a1620, a1621)?
        };
        let a1624 = 0u64;
        nat_quot(a1622, a1623, a1624)
    };
    let v5: bool = {
        let a1633 = {
            let a1631 = {
                let a1625 = 1u64;
                let a1626 = 1u64;
                nat_add(a1625, a1626)?
            };
            let a1632 = {
                let a1629 = {
                    let a1627 = 1u64;
                    let a1628 = 1u64;
                    nat_add(a1627, a1628)?
                };
                let a1630 = 1u64;
                nat_mul(a1629, a1630)?
            };
            nat_mul(a1631, a1632)?
        };
        let a1634 = v3;
        if nat_lt(a1633, a1634) {
            true
        } else {
            let a1643 = v3;
            let a1644 = {
                let a1641 = {
                    let a1635 = 1u64;
                    let a1636 = 1u64;
                    nat_add(a1635, a1636)?
                };
                let a1642 = {
                    let a1639 = {
                        let a1637 = 1u64;
                        let a1638 = 1u64;
                        nat_add(a1637, a1638)?
                    };
                    let a1640 = 1u64;
                    nat_mul(a1639, a1640)?
                };
                nat_mul(a1641, a1642)?
            };
            if nat_eq(a1643, a1644) {
                let a1652 = {
                    let a1649 = v4;
                    let a1650 = {
                        let a1647 = {
                            let a1645 = 1u64;
                            let a1646 = 1u64;
                            nat_add(a1645, a1646)?
                        };
                        let a1648 = 1u64;
                        nat_mul(a1647, a1648)?
                    };
                    let a1651 = 0u64;
                    nat_rem(a1649, a1650, a1651)
                };
                let a1653 = 1u64;
                nat_eq(a1652, a1653)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1654 = v4;
        let a1655 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1654, a1655)?
    };
    let v7: bool = {
        let a1684 = {
            let a1682 = {
                let a1658 = {
                    let a1656 = 1u64;
                    let a1657 = 1u64;
                    nat_add(a1656, a1657)?
                };
                let a1659 = 1u64;
                nat_mul(a1658, a1659)?
            };
            let a1683 = f19({
                let a1680 = {
                    let a1678 = {
                        let a1660 = 1u64;
                        let a1661 = 1u64;
                        nat_add(a1660, a1661)?
                    };
                    let a1679 = {
                        let a1676 = {
                            let a1674 = {
                                let a1662 = 1u64;
                                let a1663 = 1u64;
                                nat_add(a1662, a1663)?
                            };
                            let a1675 = {
                                let a1672 = {
                                    let a1670 = {
                                        let a1664 = 1u64;
                                        let a1665 = 1u64;
                                        nat_add(a1664, a1665)?
                                    };
                                    let a1671 = {
                                        let a1668 = {
                                            let a1666 = 1u64;
                                            let a1667 = 1u64;
                                            nat_add(a1666, a1667)?
                                        };
                                        let a1669 = 1u64;
                                        nat_mul(a1668, a1669)?
                                    };
                                    nat_mul(a1670, a1671)?
                                };
                                let a1673 = 1u64;
                                nat_add(a1672, a1673)?
                            };
                            nat_mul(a1674, a1675)?
                        };
                        let a1677 = 1u64;
                        nat_add(a1676, a1677)?
                    };
                    nat_mul(a1678, a1679)?
                };
                let a1681 = 1u64;
                nat_add(a1680, a1681)?
            })?;
            nat_mul(a1682, a1683)?
        };
        let a1685 = v6;
        nat_le(a1684, a1685)
    };
    let v8: u64 = if v7 {
        let a1690 = v6;
        let a1691 = {
            let a1688 = {
                let a1686 = 1u64;
                let a1687 = 1u64;
                nat_add(a1686, a1687)?
            };
            let a1689 = 1u64;
            nat_mul(a1688, a1689)?
        };
        let a1692 = 0u64;
        nat_quot(a1690, a1691, a1692)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1693 = v1;
        let a1694 = 1u64;
        nat_add(a1693, a1694)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1733 = v9;
        let a1734 = {
            let a1731 = {
                let a1729 = {
                    let a1695 = 1u64;
                    let a1696 = 1u64;
                    nat_add(a1695, a1696)?
                };
                let a1730 = {
                    let a1727 = {
                        let a1697 = 1u64;
                        let a1698 = 1u64;
                        nat_add(a1697, a1698)?
                    };
                    let a1728 = {
                        let a1725 = {
                            let a1699 = 1u64;
                            let a1700 = 1u64;
                            nat_add(a1699, a1700)?
                        };
                        let a1726 = {
                            let a1723 = {
                                let a1701 = 1u64;
                                let a1702 = 1u64;
                                nat_add(a1701, a1702)?
                            };
                            let a1724 = {
                                let a1721 = {
                                    let a1703 = 1u64;
                                    let a1704 = 1u64;
                                    nat_add(a1703, a1704)?
                                };
                                let a1722 = {
                                    let a1719 = {
                                        let a1705 = 1u64;
                                        let a1706 = 1u64;
                                        nat_add(a1705, a1706)?
                                    };
                                    let a1720 = {
                                        let a1717 = {
                                            let a1707 = 1u64;
                                            let a1708 = 1u64;
                                            nat_add(a1707, a1708)?
                                        };
                                        let a1718 = {
                                            let a1715 = {
                                                let a1709 = 1u64;
                                                let a1710 = 1u64;
                                                nat_add(a1709, a1710)?
                                            };
                                            let a1716 = {
                                                let a1713 = {
                                                    let a1711 = 1u64;
                                                    let a1712 = 1u64;
                                                    nat_add(a1711, a1712)?
                                                };
                                                let a1714 = 1u64;
                                                nat_mul(a1713, a1714)?
                                            };
                                            nat_mul(a1715, a1716)?
                                        };
                                        nat_mul(a1717, a1718)?
                                    };
                                    nat_mul(a1719, a1720)?
                                };
                                nat_mul(a1721, a1722)?
                            };
                            nat_mul(a1723, a1724)?
                        };
                        nat_mul(a1725, a1726)?
                    };
                    nat_mul(a1727, a1728)?
                };
                nat_mul(a1729, a1730)?
            };
            let a1732 = 1u64;
            nat_add(a1731, a1732)?
        };
        if if nat_eq(a1733, a1734) {
            let a1757 = v8;
            let a1758 = f19({
                let a1755 = {
                    let a1753 = {
                        let a1735 = 1u64;
                        let a1736 = 1u64;
                        nat_add(a1735, a1736)?
                    };
                    let a1754 = {
                        let a1751 = {
                            let a1749 = {
                                let a1737 = 1u64;
                                let a1738 = 1u64;
                                nat_add(a1737, a1738)?
                            };
                            let a1750 = {
                                let a1747 = {
                                    let a1745 = {
                                        let a1739 = 1u64;
                                        let a1740 = 1u64;
                                        nat_add(a1739, a1740)?
                                    };
                                    let a1746 = {
                                        let a1743 = {
                                            let a1741 = 1u64;
                                            let a1742 = 1u64;
                                            nat_add(a1741, a1742)?
                                        };
                                        let a1744 = 1u64;
                                        nat_mul(a1743, a1744)?
                                    };
                                    nat_mul(a1745, a1746)?
                                };
                                let a1748 = 1u64;
                                nat_add(a1747, a1748)?
                            };
                            nat_mul(a1749, a1750)?
                        };
                        let a1752 = 1u64;
                        nat_add(a1751, a1752)?
                    };
                    nat_mul(a1753, a1754)?
                };
                let a1756 = 1u64;
                nat_add(a1755, a1756)?
            })?;
            nat_lt(a1757, a1758)
        } else {
            false
        } {
            0u64
        } else {
            let a1795 = v9;
            let a1796 = {
                let a1793 = {
                    let a1759 = 1u64;
                    let a1760 = 1u64;
                    nat_add(a1759, a1760)?
                };
                let a1794 = {
                    let a1791 = {
                        let a1761 = 1u64;
                        let a1762 = 1u64;
                        nat_add(a1761, a1762)?
                    };
                    let a1792 = {
                        let a1789 = {
                            let a1763 = 1u64;
                            let a1764 = 1u64;
                            nat_add(a1763, a1764)?
                        };
                        let a1790 = {
                            let a1787 = {
                                let a1765 = 1u64;
                                let a1766 = 1u64;
                                nat_add(a1765, a1766)?
                            };
                            let a1788 = {
                                let a1785 = {
                                    let a1767 = 1u64;
                                    let a1768 = 1u64;
                                    nat_add(a1767, a1768)?
                                };
                                let a1786 = {
                                    let a1783 = {
                                        let a1769 = 1u64;
                                        let a1770 = 1u64;
                                        nat_add(a1769, a1770)?
                                    };
                                    let a1784 = {
                                        let a1781 = {
                                            let a1771 = 1u64;
                                            let a1772 = 1u64;
                                            nat_add(a1771, a1772)?
                                        };
                                        let a1782 = {
                                            let a1779 = {
                                                let a1773 = 1u64;
                                                let a1774 = 1u64;
                                                nat_add(a1773, a1774)?
                                            };
                                            let a1780 = {
                                                let a1777 = {
                                                    let a1775 = 1u64;
                                                    let a1776 = 1u64;
                                                    nat_add(a1775, a1776)?
                                                };
                                                let a1778 = 1u64;
                                                nat_mul(a1777, a1778)?
                                            };
                                            nat_mul(a1779, a1780)?
                                        };
                                        nat_mul(a1781, a1782)?
                                    };
                                    nat_mul(a1783, a1784)?
                                };
                                nat_mul(a1785, a1786)?
                            };
                            nat_mul(a1787, a1788)?
                        };
                        nat_mul(a1789, a1790)?
                    };
                    nat_mul(a1791, a1792)?
                };
                nat_mul(a1793, a1794)?
            };
            nat_sub(a1795, a1796)
        }
    };
    let a1797 = v8;
    let a1798 = 0u64;
    if nat_eq(a1797, a1798) {
        f22(v0)
    } else {
        let a1851 = {
            let a1849 = {
                let a1847 = {
                    let a1799 = 1u64;
                    let a1800 = 1u64;
                    nat_add(a1799, a1800)?
                };
                let a1848 = {
                    let a1845 = {
                        let a1843 = {
                            let a1801 = 1u64;
                            let a1802 = 1u64;
                            nat_add(a1801, a1802)?
                        };
                        let a1844 = {
                            let a1841 = {
                                let a1839 = {
                                    let a1803 = 1u64;
                                    let a1804 = 1u64;
                                    nat_add(a1803, a1804)?
                                };
                                let a1840 = {
                                    let a1837 = {
                                        let a1835 = {
                                            let a1805 = 1u64;
                                            let a1806 = 1u64;
                                            nat_add(a1805, a1806)?
                                        };
                                        let a1836 = {
                                            let a1833 = {
                                                let a1831 = {
                                                    let a1807 = 1u64;
                                                    let a1808 = 1u64;
                                                    nat_add(a1807, a1808)?
                                                };
                                                let a1832 = {
                                                    let a1829 = {
                                                        let a1827 = {
                                                            let a1809 = 1u64;
                                                            let a1810 = 1u64;
                                                            nat_add(a1809, a1810)?
                                                        };
                                                        let a1828 = {
                                                            let a1825 = {
                                                                let a1823 = {
                                                                    let a1811 = 1u64;
                                                                    let a1812 = 1u64;
                                                                    nat_add(a1811, a1812)?
                                                                };
                                                                let a1824 = {
                                                                    let a1821 = {
                                                                        let a1819 = {
                                                                            let a1813 = 1u64;
                                                                            let a1814 = 1u64;
                                                                            nat_add(a1813, a1814)?
                                                                        };
                                                                        let a1820 = {
                                                                            let a1817 = {
                                                                                let a1815 = 1u64;
                                                                                let a1816 = 1u64;
                                                                                nat_add(a1815, a1816)?
                                                                            };
                                                                            let a1818 = 1u64;
                                                                            nat_mul(a1817, a1818)?
                                                                        };
                                                                        nat_mul(a1819, a1820)?
                                                                    };
                                                                    let a1822 = 1u64;
                                                                    nat_add(a1821, a1822)?
                                                                };
                                                                nat_mul(a1823, a1824)?
                                                            };
                                                            let a1826 = 1u64;
                                                            nat_add(a1825, a1826)?
                                                        };
                                                        nat_mul(a1827, a1828)?
                                                    };
                                                    let a1830 = 1u64;
                                                    nat_add(a1829, a1830)?
                                                };
                                                nat_mul(a1831, a1832)?
                                            };
                                            let a1834 = 1u64;
                                            nat_add(a1833, a1834)?
                                        };
                                        nat_mul(a1835, a1836)?
                                    };
                                    let a1838 = 1u64;
                                    nat_add(a1837, a1838)?
                                };
                                nat_mul(a1839, a1840)?
                            };
                            let a1842 = 1u64;
                            nat_add(a1841, a1842)?
                        };
                        nat_mul(a1843, a1844)?
                    };
                    let a1846 = 1u64;
                    nat_add(a1845, a1846)?
                };
                nat_mul(a1847, a1848)?
            };
            let a1850 = 1u64;
            nat_add(a1849, a1850)?
        };
        let a1852 = v9;
        if nat_le(a1851, a1852) {
            f20(v0)
        } else {
            let a1904 = {
                let a1877 = f22(v0)?;
                let a1878 = {
                    let a1875 = v10;
                    let a1876 = f19({
                        let a1873 = {
                            let a1871 = {
                                let a1853 = 1u64;
                                let a1854 = 1u64;
                                nat_add(a1853, a1854)?
                            };
                            let a1872 = {
                                let a1869 = {
                                    let a1867 = {
                                        let a1855 = 1u64;
                                        let a1856 = 1u64;
                                        nat_add(a1855, a1856)?
                                    };
                                    let a1868 = {
                                        let a1865 = {
                                            let a1863 = {
                                                let a1857 = 1u64;
                                                let a1858 = 1u64;
                                                nat_add(a1857, a1858)?
                                            };
                                            let a1864 = {
                                                let a1861 = {
                                                    let a1859 = 1u64;
                                                    let a1860 = 1u64;
                                                    nat_add(a1859, a1860)?
                                                };
                                                let a1862 = 1u64;
                                                nat_mul(a1861, a1862)?
                                            };
                                            nat_mul(a1863, a1864)?
                                        };
                                        let a1866 = 1u64;
                                        nat_add(a1865, a1866)?
                                    };
                                    nat_mul(a1867, a1868)?
                                };
                                let a1870 = 1u64;
                                nat_add(a1869, a1870)?
                            };
                            nat_mul(a1871, a1872)?
                        };
                        let a1874 = 1u64;
                        nat_add(a1873, a1874)?
                    })?;
                    nat_mul(a1875, a1876)?
                };
                nat_add(a1877, a1878)?
            };
            let a1905 = {
                let a1901 = v8;
                let a1902 = f19({
                    let a1899 = {
                        let a1897 = {
                            let a1879 = 1u64;
                            let a1880 = 1u64;
                            nat_add(a1879, a1880)?
                        };
                        let a1898 = {
                            let a1895 = {
                                let a1893 = {
                                    let a1881 = 1u64;
                                    let a1882 = 1u64;
                                    nat_add(a1881, a1882)?
                                };
                                let a1894 = {
                                    let a1891 = {
                                        let a1889 = {
                                            let a1883 = 1u64;
                                            let a1884 = 1u64;
                                            nat_add(a1883, a1884)?
                                        };
                                        let a1890 = {
                                            let a1887 = {
                                                let a1885 = 1u64;
                                                let a1886 = 1u64;
                                                nat_add(a1885, a1886)?
                                            };
                                            let a1888 = 1u64;
                                            nat_mul(a1887, a1888)?
                                        };
                                        nat_mul(a1889, a1890)?
                                    };
                                    let a1892 = 1u64;
                                    nat_add(a1891, a1892)?
                                };
                                nat_mul(a1893, a1894)?
                            };
                            let a1896 = 1u64;
                            nat_add(a1895, a1896)?
                        };
                        nat_mul(a1897, a1898)?
                    };
                    let a1900 = 1u64;
                    nat_add(a1899, a1900)?
                })?;
                let a1903 = 0u64;
                nat_rem(a1901, a1902, a1903)
            };
            nat_add(a1904, a1905)
        }
    }
}
