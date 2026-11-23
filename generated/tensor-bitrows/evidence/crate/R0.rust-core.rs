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
    C15(bool, u64, u64),
    C16(bool, u64),
    C17(bool, u64, u64, u64, u64),
    C18(bool, u64, u64),
    C19(bool, u64, u64),
    C20(bool, u64),
    C21(bool, u64, u64),
    C22(bool, u64),
    C23(u64, u64, u64, u64, u64),
    C24(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64, u64, u64, u64),
    C2(u64),
    C3(u64, u64),
    C4(u64, u64),
    C5(u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F42,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F42 => f42(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F47,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F47 => f47(p0),
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
        Adt0::C7(v88, v89) => {
            let a128 = v88;
            let a129 = 4294967296u64;
            if if nat_lt(a128, a129) {
                let a130 = v89;
                let a131 = 2139095040u64;
                nat_lt(a130, a131)
            } else {
                false
            } {
                Ok(Adt1::C2(f21(f5(v88, v89)?)?))
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
        Adt0::C11(v75, v76, v77) => {
            let a124 = v77;
            let a125 = 18u64;
            if if nat_eq(a124, a125) {
                let a126 = v76;
                let a127 = if v75 {
                    64u64
                } else {
                    128u64
                };
                nat_eq(a126, a127)
            } else {
                false
            } {
                Ok(Adt1::C2(if v75 {
                    4u64
                } else {
                    8u64
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C12(v78, v79, v80) => {
            f18(v78, v79, v80)
        }
        Adt0::C13(v81, v82) => {
            f19(v81, v82)
        }
        Adt0::C14(v83, v84, v85, v86, v87) => {
            f20(v83, v84, v85, v86, v87)
        }
        Adt0::C15(v60, v61, v62) => {
            let v63: u64 = if v60 {
                52u64
            } else {
                64u64
            };
            let a103 = v61;
            let a104 = 256u64;
            if if nat_eq(a103, a104) {
                let a107 = v62;
                let a108 = {
                    let a105 = v63;
                    let a106 = 2u64;
                    nat_add(a105, a106)?
                };
                nat_eq(a107, a108)
            } else {
                false
            } {
                Ok(Adt1::C2(v63))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C16(v64, v65) => {
            f16(v64, v65)
        }
        Adt0::C17(v66, v67, v68, v69, v70) => {
            f17(v66, v67, v68, v69, v70)
        }
        Adt0::C18(v71, v72, v73) => {
            let v74: u64 = {
                let a109 = v72;
                let a110 = v73;
                nat_mul(a109, a110)?
            };
            if v71 {
                let a111 = v74;
                let a112 = 243u64;
                if nat_lt(a111, a112) {
                    Ok(Adt1::C2({
                        let a117 = {
                            let a115 = {
                                let a113 = v74;
                                let a114 = 256u64;
                                nat_mul(a113, a114)?
                            };
                            let a116 = 242u64;
                            nat_add(a115, a116)?
                        };
                        let a118 = 243u64;
                        let a119 = 0u64;
                        nat_quot(a117, a118, a119)
                    }))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                let a120 = v72;
                let a121 = 1u64;
                if if nat_eq(a120, a121) {
                    let a122 = v73;
                    let a123 = 256u64;
                    nat_lt(a122, a123)
                } else {
                    false
                } {
                    Ok(Adt1::C2(v73))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
        Adt0::C19(v54, v55, v56) => {
            let v57: u64 = if v54 {
                54u64
            } else {
                66u64
            };
            let a96 = {
                let a93 = v55;
                let a94 = v57;
                let a95 = 0u64;
                nat_rem(a93, a94, a95)
            };
            let a97 = 0u64;
            if if nat_eq(a96, a97) {
                let a101 = {
                    let a98 = v55;
                    let a99 = v57;
                    let a100 = 0u64;
                    nat_quot(a98, a99, a100)
                };
                let a102 = v56;
                nat_eq(a101, a102)
            } else {
                false
            } {
                Ok(Adt1::C2(v57))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C20(v58, v59) => {
            f15(v58, v59)
        }
        Adt0::C21(v41, v42, v43) => {
            let a69 = v42;
            let a70 = if v41 {
                64u64
            } else {
                32u64
            };
            if if nat_eq(a69, a70) {
                let a71 = v43;
                let a72 = if v41 {
                    36u64
                } else {
                    17u64
                };
                nat_eq(a71, a72)
            } else {
                false
            } {
                Ok(Adt1::C1(if v41 {
                    4u64
                } else {
                    1u64
                }, if v41 {
                    16u64
                } else {
                    32u64
                }, if v41 {
                    2u64
                } else {
                    1u64
                }, 18584u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C22(v44, v45) => {
            let a73 = v45;
            let a74 = 2139095040u64;
            if nat_lt(a73, a74) {
                Ok(Adt1::C2(if v44 {
                    f10(f11(v45, 1086324736u64)?)?
                } else {
                    f12(v45)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C23(v46, v47, v48, v49, v50) => {
            let a75 = v46;
            let a76 = 4294967296u64;
            if if if nat_lt(a75, a76) {
                let a77 = v47;
                let a78 = 4294967296u64;
                nat_lt(a77, a78)
            } else {
                false
            } {
                let a79 = v48;
                let a80 = 16u64;
                if nat_lt(a79, a80) {
                    let a81 = v49;
                    let a82 = 16u64;
                    nat_lt(a81, a82)
                } else {
                    false
                }
            } else {
                false
            } {
                let v51: u64 = f13(f7(v47, f14(v46)?)?)?;
                let a83 = v51;
                let a84 = v50;
                if nat_lt(a83, a84) {
                    Ok(Adt1::C5(v48, v51))
                } else {
                    Ok(Adt1::C5(v49, v50))
                }
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C24(v52, v53) => {
            let a85 = v52;
            let a86 = 16u64;
            if if nat_lt(a85, a86) {
                let a87 = v53;
                let a88 = 16u64;
                nat_lt(a87, a88)
            } else {
                false
            } {
                Ok(Adt1::C2({
                    let a91 = v52;
                    let a92 = {
                        let a89 = 16u64;
                        let a90 = v53;
                        nat_mul(a89, a90)?
                    };
                    nat_add(a91, a92)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a132 = 0u64;
    let a133 = v2;
    if if nat_lt(a132, a133) {
        let a134 = 0u64;
        let a135 = v3;
        nat_lt(a134, a135)
    } else {
        false
    } {
        Ok(Some({
            let a141 = {
                let a138 = {
                    let a136 = v0;
                    let a137 = v1;
                    nat_mul(a136, a137)?
                };
                let a139 = v2;
                let a140 = 0u64;
                nat_quot(a138, a139, a140)
            };
            let a142 = v3;
            let a143 = 0u64;
            nat_rem(a141, a142, a143)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a144 = v0;
    let a145 = 0u64;
    if nat_eq(a144, a145) {
        1u64
    } else {
        let a146 = v0;
        let a147 = 1u64;
        if nat_eq(a146, a147) {
            2u64
        } else {
            let a148 = v0;
            let a149 = 2u64;
            if nat_eq(a148, a149) {
                4u64
            } else {
                let a150 = v0;
                let a151 = 3u64;
                if nat_eq(a150, a151) {
                    8u64
                } else {
                    let a152 = v0;
                    let a153 = 4u64;
                    if nat_eq(a152, a153) {
                        16u64
                    } else {
                        let a154 = v0;
                        let a155 = 5u64;
                        if nat_eq(a154, a155) {
                            32u64
                        } else {
                            let a156 = v0;
                            let a157 = 6u64;
                            if nat_eq(a156, a157) {
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
    let a158 = v1;
    let a159 = 16u64;
    if if if nat_lt(a158, a159) {
        let a160 = v2;
        let a161 = 256u64;
        nat_lt(a160, a161)
    } else {
        false
    } {
        let a162 = v3;
        let a163 = 4294967296u64;
        nat_lt(a162, a163)
    } else {
        false
    } {
        let v4: u64 = {
            let a164 = v2;
            let a165 = 16u64;
            let a166 = 0u64;
            nat_rem(a164, a165, a166)
        };
        let v5: u64 = {
            let a167 = v2;
            let a168 = 16u64;
            let a169 = 0u64;
            nat_quot(a167, a168, a169)
        };
        if v0 {
            Ok(Adt1::C4({
                let a170 = 18568u64;
                let a171 = v4;
                nat_add(a170, a171)?
            }, {
                let a172 = 18568u64;
                let a173 = v5;
                nat_add(a172, a173)?
            }))
        } else {
            let v6: u64 = {
                let a179 = f2({
                    let a174 = v1;
                    let a175 = 8u64;
                    let a176 = 0u64;
                    nat_rem(a174, a175, a176)
                });
                let a180 = {
                    let a177 = v1;
                    let a178 = 8u64;
                    if nat_lt(a177, a178) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a179, a180)?
            };
            Ok(Adt1::C3(f22(v4, v3, v6)?, f22(v5, v3, {
                let a181 = v6;
                let a182 = 65536u64;
                nat_mul(a181, a182)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a183 = v0;
    let a184 = 16u64;
    if if if nat_lt(a183, a184) {
        let a185 = v1;
        let a186 = 256u64;
        nat_lt(a185, a186)
    } else {
        false
    } {
        let a187 = v2;
        let a188 = 4294967296u64;
        nat_lt(a187, a188)
    } else {
        false
    } {
        let v3: u64 = {
            let a194 = f2({
                let a189 = v0;
                let a190 = 8u64;
                let a191 = 0u64;
                nat_rem(a189, a190, a191)
            });
            let a195 = {
                let a192 = v0;
                let a193 = 8u64;
                if nat_lt(a192, a193) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a194, a195)?
        };
        Ok(Adt1::C3(f23({
            let a196 = v1;
            let a197 = 16u64;
            let a198 = 0u64;
            nat_rem(a196, a197, a198)
        }, v2, v3)?, f23({
            let a199 = v1;
            let a200 = 16u64;
            let a201 = 0u64;
            nat_quot(a199, a200, a201)
        }, v2, {
            let a202 = v3;
            let a203 = 65536u64;
            nat_mul(a202, a203)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f24(v0)? {
        true
    } else {
        f24(v1)?
    } {
        true
    } else {
        if f25(v0)? {
            f26(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f25(v1)? {
            f26(v0)?
        } else {
            false
        }
    } {
        let a290 = {
            let a268 = {
                let a244 = {
                    let a242 = {
                        let a204 = 1u64;
                        let a205 = 1u64;
                        nat_add(a204, a205)?
                    };
                    let a243 = {
                        let a240 = {
                            let a238 = {
                                let a206 = 1u64;
                                let a207 = 1u64;
                                nat_add(a206, a207)?
                            };
                            let a239 = {
                                let a236 = {
                                    let a234 = {
                                        let a208 = 1u64;
                                        let a209 = 1u64;
                                        nat_add(a208, a209)?
                                    };
                                    let a235 = {
                                        let a232 = {
                                            let a230 = {
                                                let a210 = 1u64;
                                                let a211 = 1u64;
                                                nat_add(a210, a211)?
                                            };
                                            let a231 = {
                                                let a228 = {
                                                    let a226 = {
                                                        let a212 = 1u64;
                                                        let a213 = 1u64;
                                                        nat_add(a212, a213)?
                                                    };
                                                    let a227 = {
                                                        let a224 = {
                                                            let a222 = {
                                                                let a214 = 1u64;
                                                                let a215 = 1u64;
                                                                nat_add(a214, a215)?
                                                            };
                                                            let a223 = {
                                                                let a220 = {
                                                                    let a218 = {
                                                                        let a216 = 1u64;
                                                                        let a217 = 1u64;
                                                                        nat_add(a216, a217)?
                                                                    };
                                                                    let a219 = 1u64;
                                                                    nat_mul(a218, a219)?
                                                                };
                                                                let a221 = 1u64;
                                                                nat_add(a220, a221)?
                                                            };
                                                            nat_mul(a222, a223)?
                                                        };
                                                        let a225 = 1u64;
                                                        nat_add(a224, a225)?
                                                    };
                                                    nat_mul(a226, a227)?
                                                };
                                                let a229 = 1u64;
                                                nat_add(a228, a229)?
                                            };
                                            nat_mul(a230, a231)?
                                        };
                                        let a233 = 1u64;
                                        nat_add(a232, a233)?
                                    };
                                    nat_mul(a234, a235)?
                                };
                                let a237 = 1u64;
                                nat_add(a236, a237)?
                            };
                            nat_mul(a238, a239)?
                        };
                        let a241 = 1u64;
                        nat_add(a240, a241)?
                    };
                    nat_mul(a242, a243)?
                };
                let a245 = 1u64;
                nat_add(a244, a245)?
            };
            let a269 = f27({
                let a266 = {
                    let a264 = {
                        let a246 = 1u64;
                        let a247 = 1u64;
                        nat_add(a246, a247)?
                    };
                    let a265 = {
                        let a262 = {
                            let a260 = {
                                let a248 = 1u64;
                                let a249 = 1u64;
                                nat_add(a248, a249)?
                            };
                            let a261 = {
                                let a258 = {
                                    let a256 = {
                                        let a250 = 1u64;
                                        let a251 = 1u64;
                                        nat_add(a250, a251)?
                                    };
                                    let a257 = {
                                        let a254 = {
                                            let a252 = 1u64;
                                            let a253 = 1u64;
                                            nat_add(a252, a253)?
                                        };
                                        let a255 = 1u64;
                                        nat_mul(a254, a255)?
                                    };
                                    nat_mul(a256, a257)?
                                };
                                let a259 = 1u64;
                                nat_add(a258, a259)?
                            };
                            nat_mul(a260, a261)?
                        };
                        let a263 = 1u64;
                        nat_add(a262, a263)?
                    };
                    nat_mul(a264, a265)?
                };
                let a267 = 1u64;
                nat_add(a266, a267)?
            })?;
            nat_mul(a268, a269)?
        };
        let a291 = f27({
            let a288 = {
                let a270 = 1u64;
                let a271 = 1u64;
                nat_add(a270, a271)?
            };
            let a289 = {
                let a286 = {
                    let a284 = {
                        let a272 = 1u64;
                        let a273 = 1u64;
                        nat_add(a272, a273)?
                    };
                    let a285 = {
                        let a282 = {
                            let a280 = {
                                let a274 = 1u64;
                                let a275 = 1u64;
                                nat_add(a274, a275)?
                            };
                            let a281 = {
                                let a278 = {
                                    let a276 = 1u64;
                                    let a277 = 1u64;
                                    nat_add(a276, a277)?
                                };
                                let a279 = 1u64;
                                nat_mul(a278, a279)?
                            };
                            nat_mul(a280, a281)?
                        };
                        let a283 = 1u64;
                        nat_add(a282, a283)?
                    };
                    nat_mul(a284, a285)?
                };
                let a287 = 1u64;
                nat_add(a286, a287)?
            };
            nat_mul(a288, a289)?
        })?;
        nat_add(a290, a291)
    } else {
        if if f25(v0)? {
            true
        } else {
            f25(v1)?
        } {
            f28(f29(v0, v1)?)
        } else {
            if if f26(v0)? {
                true
            } else {
                f26(v1)?
            } {
                f30(f29(v0, v1)?)
            } else {
                f31(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a295 = {
            let a292 = v0;
            let a293 = 32768u64;
            let a294 = 0u64;
            nat_quot(a292, a293, a294)
        };
        let a296 = 2147483648u64;
        nat_mul(a295, a296)?
    };
    let v2: u64 = {
        let a300 = {
            let a297 = v0;
            let a298 = 32768u64;
            let a299 = 0u64;
            nat_rem(a297, a298, a299)
        };
        let a301 = 1024u64;
        let a302 = 0u64;
        nat_quot(a300, a301, a302)
    };
    let v3: u64 = {
        let a303 = v0;
        let a304 = 1024u64;
        let a305 = 0u64;
        nat_rem(a303, a304, a305)
    };
    let a340 = v1;
    let a341 = {
        let a306 = v2;
        let a307 = 31u64;
        if nat_eq(a306, a307) {
            let a316 = 2139095040u64;
            let a317 = {
                let a314 = {
                    let a308 = 0u64;
                    let a309 = v3;
                    if if nat_lt(a308, a309) {
                        let a310 = v3;
                        let a311 = 512u64;
                        nat_lt(a310, a311)
                    } else {
                        false
                    } {
                        let a312 = v3;
                        let a313 = 512u64;
                        nat_add(a312, a313)?
                    } else {
                        v3
                    }
                };
                let a315 = 8192u64;
                nat_mul(a314, a315)?
            };
            nat_add(a316, a317)?
        } else {
            let a318 = v2;
            let a319 = 0u64;
            if nat_eq(a318, a319) {
                let a320 = v3;
                let a321 = 0u64;
                if nat_eq(a320, a321) {
                    0u64
                } else {
                    let v4: Adt2 = f32(113u64, v3, 0u64, 1024u64)?;
                    let a330 = {
                        let a323 = match v4.clone() {
                            Adt2::C0(h322, _, _, _) => {
                                h322
                            }
                        };
                        let a324 = 8388608u64;
                        nat_mul(a323, a324)?
                    };
                    let a331 = {
                        let a328 = {
                            let a326 = match v4 {
                                Adt2::C0(_, h325, _, _) => {
                                    h325
                                }
                            };
                            let a327 = 1024u64;
                            nat_sub(a326, a327)
                        };
                        let a329 = 8192u64;
                        nat_mul(a328, a329)?
                    };
                    nat_add(a330, a331)?
                }
            } else {
                let a338 = {
                    let a334 = {
                        let a332 = v2;
                        let a333 = 112u64;
                        nat_add(a332, a333)?
                    };
                    let a335 = 8388608u64;
                    nat_mul(a334, a335)?
                };
                let a339 = {
                    let a336 = v3;
                    let a337 = 8192u64;
                    nat_mul(a336, a337)?
                };
                nat_add(a338, a339)?
            }
        }
    };
    nat_add(a340, a341)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f24(v0)? {
        true
    } else {
        f24(v1)?
    } {
        let a428 = {
            let a406 = {
                let a382 = {
                    let a380 = {
                        let a342 = 1u64;
                        let a343 = 1u64;
                        nat_add(a342, a343)?
                    };
                    let a381 = {
                        let a378 = {
                            let a376 = {
                                let a344 = 1u64;
                                let a345 = 1u64;
                                nat_add(a344, a345)?
                            };
                            let a377 = {
                                let a374 = {
                                    let a372 = {
                                        let a346 = 1u64;
                                        let a347 = 1u64;
                                        nat_add(a346, a347)?
                                    };
                                    let a373 = {
                                        let a370 = {
                                            let a368 = {
                                                let a348 = 1u64;
                                                let a349 = 1u64;
                                                nat_add(a348, a349)?
                                            };
                                            let a369 = {
                                                let a366 = {
                                                    let a364 = {
                                                        let a350 = 1u64;
                                                        let a351 = 1u64;
                                                        nat_add(a350, a351)?
                                                    };
                                                    let a365 = {
                                                        let a362 = {
                                                            let a360 = {
                                                                let a352 = 1u64;
                                                                let a353 = 1u64;
                                                                nat_add(a352, a353)?
                                                            };
                                                            let a361 = {
                                                                let a358 = {
                                                                    let a356 = {
                                                                        let a354 = 1u64;
                                                                        let a355 = 1u64;
                                                                        nat_add(a354, a355)?
                                                                    };
                                                                    let a357 = 1u64;
                                                                    nat_mul(a356, a357)?
                                                                };
                                                                let a359 = 1u64;
                                                                nat_add(a358, a359)?
                                                            };
                                                            nat_mul(a360, a361)?
                                                        };
                                                        let a363 = 1u64;
                                                        nat_add(a362, a363)?
                                                    };
                                                    nat_mul(a364, a365)?
                                                };
                                                let a367 = 1u64;
                                                nat_add(a366, a367)?
                                            };
                                            nat_mul(a368, a369)?
                                        };
                                        let a371 = 1u64;
                                        nat_add(a370, a371)?
                                    };
                                    nat_mul(a372, a373)?
                                };
                                let a375 = 1u64;
                                nat_add(a374, a375)?
                            };
                            nat_mul(a376, a377)?
                        };
                        let a379 = 1u64;
                        nat_add(a378, a379)?
                    };
                    nat_mul(a380, a381)?
                };
                let a383 = 1u64;
                nat_add(a382, a383)?
            };
            let a407 = f27({
                let a404 = {
                    let a402 = {
                        let a384 = 1u64;
                        let a385 = 1u64;
                        nat_add(a384, a385)?
                    };
                    let a403 = {
                        let a400 = {
                            let a398 = {
                                let a386 = 1u64;
                                let a387 = 1u64;
                                nat_add(a386, a387)?
                            };
                            let a399 = {
                                let a396 = {
                                    let a394 = {
                                        let a388 = 1u64;
                                        let a389 = 1u64;
                                        nat_add(a388, a389)?
                                    };
                                    let a395 = {
                                        let a392 = {
                                            let a390 = 1u64;
                                            let a391 = 1u64;
                                            nat_add(a390, a391)?
                                        };
                                        let a393 = 1u64;
                                        nat_mul(a392, a393)?
                                    };
                                    nat_mul(a394, a395)?
                                };
                                let a397 = 1u64;
                                nat_add(a396, a397)?
                            };
                            nat_mul(a398, a399)?
                        };
                        let a401 = 1u64;
                        nat_add(a400, a401)?
                    };
                    nat_mul(a402, a403)?
                };
                let a405 = 1u64;
                nat_add(a404, a405)?
            })?;
            nat_mul(a406, a407)?
        };
        let a429 = f27({
            let a426 = {
                let a408 = 1u64;
                let a409 = 1u64;
                nat_add(a408, a409)?
            };
            let a427 = {
                let a424 = {
                    let a422 = {
                        let a410 = 1u64;
                        let a411 = 1u64;
                        nat_add(a410, a411)?
                    };
                    let a423 = {
                        let a420 = {
                            let a418 = {
                                let a412 = 1u64;
                                let a413 = 1u64;
                                nat_add(a412, a413)?
                            };
                            let a419 = {
                                let a416 = {
                                    let a414 = 1u64;
                                    let a415 = 1u64;
                                    nat_add(a414, a415)?
                                };
                                let a417 = 1u64;
                                nat_mul(a416, a417)?
                            };
                            nat_mul(a418, a419)?
                        };
                        let a421 = 1u64;
                        nat_add(a420, a421)?
                    };
                    nat_mul(a422, a423)?
                };
                let a425 = 1u64;
                nat_add(a424, a425)?
            };
            nat_mul(a426, a427)?
        })?;
        nat_add(a428, a429)
    } else {
        if if f25(v0)? {
            f25(v1)?
        } else {
            false
        } {
            let a430 = f33(v0)?;
            let a431 = f33(v1)?;
            if nat_eq(a430, a431) {
                Ok(v0)
            } else {
                let a518 = {
                    let a496 = {
                        let a472 = {
                            let a470 = {
                                let a432 = 1u64;
                                let a433 = 1u64;
                                nat_add(a432, a433)?
                            };
                            let a471 = {
                                let a468 = {
                                    let a466 = {
                                        let a434 = 1u64;
                                        let a435 = 1u64;
                                        nat_add(a434, a435)?
                                    };
                                    let a467 = {
                                        let a464 = {
                                            let a462 = {
                                                let a436 = 1u64;
                                                let a437 = 1u64;
                                                nat_add(a436, a437)?
                                            };
                                            let a463 = {
                                                let a460 = {
                                                    let a458 = {
                                                        let a438 = 1u64;
                                                        let a439 = 1u64;
                                                        nat_add(a438, a439)?
                                                    };
                                                    let a459 = {
                                                        let a456 = {
                                                            let a454 = {
                                                                let a440 = 1u64;
                                                                let a441 = 1u64;
                                                                nat_add(a440, a441)?
                                                            };
                                                            let a455 = {
                                                                let a452 = {
                                                                    let a450 = {
                                                                        let a442 = 1u64;
                                                                        let a443 = 1u64;
                                                                        nat_add(a442, a443)?
                                                                    };
                                                                    let a451 = {
                                                                        let a448 = {
                                                                            let a446 = {
                                                                                let a444 = 1u64;
                                                                                let a445 = 1u64;
                                                                                nat_add(a444, a445)?
                                                                            };
                                                                            let a447 = 1u64;
                                                                            nat_mul(a446, a447)?
                                                                        };
                                                                        let a449 = 1u64;
                                                                        nat_add(a448, a449)?
                                                                    };
                                                                    nat_mul(a450, a451)?
                                                                };
                                                                let a453 = 1u64;
                                                                nat_add(a452, a453)?
                                                            };
                                                            nat_mul(a454, a455)?
                                                        };
                                                        let a457 = 1u64;
                                                        nat_add(a456, a457)?
                                                    };
                                                    nat_mul(a458, a459)?
                                                };
                                                let a461 = 1u64;
                                                nat_add(a460, a461)?
                                            };
                                            nat_mul(a462, a463)?
                                        };
                                        let a465 = 1u64;
                                        nat_add(a464, a465)?
                                    };
                                    nat_mul(a466, a467)?
                                };
                                let a469 = 1u64;
                                nat_add(a468, a469)?
                            };
                            nat_mul(a470, a471)?
                        };
                        let a473 = 1u64;
                        nat_add(a472, a473)?
                    };
                    let a497 = f27({
                        let a494 = {
                            let a492 = {
                                let a474 = 1u64;
                                let a475 = 1u64;
                                nat_add(a474, a475)?
                            };
                            let a493 = {
                                let a490 = {
                                    let a488 = {
                                        let a476 = 1u64;
                                        let a477 = 1u64;
                                        nat_add(a476, a477)?
                                    };
                                    let a489 = {
                                        let a486 = {
                                            let a484 = {
                                                let a478 = 1u64;
                                                let a479 = 1u64;
                                                nat_add(a478, a479)?
                                            };
                                            let a485 = {
                                                let a482 = {
                                                    let a480 = 1u64;
                                                    let a481 = 1u64;
                                                    nat_add(a480, a481)?
                                                };
                                                let a483 = 1u64;
                                                nat_mul(a482, a483)?
                                            };
                                            nat_mul(a484, a485)?
                                        };
                                        let a487 = 1u64;
                                        nat_add(a486, a487)?
                                    };
                                    nat_mul(a488, a489)?
                                };
                                let a491 = 1u64;
                                nat_add(a490, a491)?
                            };
                            nat_mul(a492, a493)?
                        };
                        let a495 = 1u64;
                        nat_add(a494, a495)?
                    })?;
                    nat_mul(a496, a497)?
                };
                let a519 = f27({
                    let a516 = {
                        let a498 = 1u64;
                        let a499 = 1u64;
                        nat_add(a498, a499)?
                    };
                    let a517 = {
                        let a514 = {
                            let a512 = {
                                let a500 = 1u64;
                                let a501 = 1u64;
                                nat_add(a500, a501)?
                            };
                            let a513 = {
                                let a510 = {
                                    let a508 = {
                                        let a502 = 1u64;
                                        let a503 = 1u64;
                                        nat_add(a502, a503)?
                                    };
                                    let a509 = {
                                        let a506 = {
                                            let a504 = 1u64;
                                            let a505 = 1u64;
                                            nat_add(a504, a505)?
                                        };
                                        let a507 = 1u64;
                                        nat_mul(a506, a507)?
                                    };
                                    nat_mul(a508, a509)?
                                };
                                let a511 = 1u64;
                                nat_add(a510, a511)?
                            };
                            nat_mul(a512, a513)?
                        };
                        let a515 = 1u64;
                        nat_add(a514, a515)?
                    };
                    nat_mul(a516, a517)?
                })?;
                nat_add(a518, a519)
            }
        } else {
            if f25(v0)? {
                Ok(v0)
            } else {
                if f25(v1)? {
                    Ok(v1)
                } else {
                    let a520 = f13(v1)?;
                    let a521 = f13(v0)?;
                    if nat_le(a520, a521) {
                        f34(v0, v1)
                    } else {
                        f34(v1, v0)
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
    let a525 = {
        let a522 = v1;
        let a523 = v3;
        let a524 = 0u64;
        nat_rem(a522, a523, a524)
    };
    let a526 = 0u64;
    if if nat_eq(a525, a526) {
        let a532 = {
            let a530 = {
                let a527 = v1;
                let a528 = v3;
                let a529 = 0u64;
                nat_quot(a527, a528, a529)
            };
            let a531 = v4;
            nat_mul(a530, a531)?
        };
        let a533 = v2;
        nat_eq(a532, a533)
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
    let a534 = v0;
    let a535 = 4294967296u64;
    if if if if nat_lt(a534, a535) {
        let a536 = v1;
        let a537 = 4294967296u64;
        nat_lt(a536, a537)
    } else {
        false
    } {
        let a538 = v2;
        let a539 = 4294967296u64;
        nat_lt(a538, a539)
    } else {
        false
    } {
        let a540 = v3;
        let a541 = 65536u64;
        nat_lt(a540, a541)
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
    let v1: u64 = f35(v0)?;
    let a542 = v1;
    let a543 = 120u64;
    if nat_le(a542, a543) {
        let v2: u64 = f7(f5(v0, 1140850688u64)?, 1056964608u64)?;
        let v3: u64 = {
            let a546 = f36(v2)?;
            let a547 = f27({
                let a544 = 150u64;
                let a545 = f35(v2)?;
                nat_sub(a544, a545)
            })?;
            let a548 = 0u64;
            nat_quot(a546, a547, a548)
        };
        let a549 = v3;
        let a550 = 8u64;
        if nat_lt(a549, a550) {
            Ok(v3)
        } else {
            Ok(7u64)
        }
    } else {
        let a551 = 134u64;
        let a552 = v1;
        if nat_lt(a551, a552) {
            Ok(126u64)
        } else {
            let v4: u64 = {
                let a562 = {
                    let a555 = 8u64;
                    let a556 = {
                        let a553 = v1;
                        let a554 = 120u64;
                        nat_sub(a553, a554)
                    };
                    nat_mul(a555, a556)?
                };
                let a563 = {
                    let a559 = {
                        let a557 = f37(v0)?;
                        let a558 = 524288u64;
                        nat_add(a557, a558)?
                    };
                    let a560 = 1048576u64;
                    let a561 = 0u64;
                    nat_quot(a559, a560, a561)
                };
                nat_add(a562, a563)?
            };
            let a564 = v4;
            let a565 = 120u64;
            if nat_lt(a564, a565) {
                Ok(v4)
            } else {
                Ok(126u64)
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    if if if if f24(v0)? {
        true
    } else {
        f24(v1)?
    } {
        true
    } else {
        if f25(v0)? {
            f25(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f26(v0)? {
            f26(v1)?
        } else {
            false
        }
    } {
        let a652 = {
            let a630 = {
                let a606 = {
                    let a604 = {
                        let a566 = 1u64;
                        let a567 = 1u64;
                        nat_add(a566, a567)?
                    };
                    let a605 = {
                        let a602 = {
                            let a600 = {
                                let a568 = 1u64;
                                let a569 = 1u64;
                                nat_add(a568, a569)?
                            };
                            let a601 = {
                                let a598 = {
                                    let a596 = {
                                        let a570 = 1u64;
                                        let a571 = 1u64;
                                        nat_add(a570, a571)?
                                    };
                                    let a597 = {
                                        let a594 = {
                                            let a592 = {
                                                let a572 = 1u64;
                                                let a573 = 1u64;
                                                nat_add(a572, a573)?
                                            };
                                            let a593 = {
                                                let a590 = {
                                                    let a588 = {
                                                        let a574 = 1u64;
                                                        let a575 = 1u64;
                                                        nat_add(a574, a575)?
                                                    };
                                                    let a589 = {
                                                        let a586 = {
                                                            let a584 = {
                                                                let a576 = 1u64;
                                                                let a577 = 1u64;
                                                                nat_add(a576, a577)?
                                                            };
                                                            let a585 = {
                                                                let a582 = {
                                                                    let a580 = {
                                                                        let a578 = 1u64;
                                                                        let a579 = 1u64;
                                                                        nat_add(a578, a579)?
                                                                    };
                                                                    let a581 = 1u64;
                                                                    nat_mul(a580, a581)?
                                                                };
                                                                let a583 = 1u64;
                                                                nat_add(a582, a583)?
                                                            };
                                                            nat_mul(a584, a585)?
                                                        };
                                                        let a587 = 1u64;
                                                        nat_add(a586, a587)?
                                                    };
                                                    nat_mul(a588, a589)?
                                                };
                                                let a591 = 1u64;
                                                nat_add(a590, a591)?
                                            };
                                            nat_mul(a592, a593)?
                                        };
                                        let a595 = 1u64;
                                        nat_add(a594, a595)?
                                    };
                                    nat_mul(a596, a597)?
                                };
                                let a599 = 1u64;
                                nat_add(a598, a599)?
                            };
                            nat_mul(a600, a601)?
                        };
                        let a603 = 1u64;
                        nat_add(a602, a603)?
                    };
                    nat_mul(a604, a605)?
                };
                let a607 = 1u64;
                nat_add(a606, a607)?
            };
            let a631 = f27({
                let a628 = {
                    let a626 = {
                        let a608 = 1u64;
                        let a609 = 1u64;
                        nat_add(a608, a609)?
                    };
                    let a627 = {
                        let a624 = {
                            let a622 = {
                                let a610 = 1u64;
                                let a611 = 1u64;
                                nat_add(a610, a611)?
                            };
                            let a623 = {
                                let a620 = {
                                    let a618 = {
                                        let a612 = 1u64;
                                        let a613 = 1u64;
                                        nat_add(a612, a613)?
                                    };
                                    let a619 = {
                                        let a616 = {
                                            let a614 = 1u64;
                                            let a615 = 1u64;
                                            nat_add(a614, a615)?
                                        };
                                        let a617 = 1u64;
                                        nat_mul(a616, a617)?
                                    };
                                    nat_mul(a618, a619)?
                                };
                                let a621 = 1u64;
                                nat_add(a620, a621)?
                            };
                            nat_mul(a622, a623)?
                        };
                        let a625 = 1u64;
                        nat_add(a624, a625)?
                    };
                    nat_mul(a626, a627)?
                };
                let a629 = 1u64;
                nat_add(a628, a629)?
            })?;
            nat_mul(a630, a631)?
        };
        let a653 = f27({
            let a650 = {
                let a632 = 1u64;
                let a633 = 1u64;
                nat_add(a632, a633)?
            };
            let a651 = {
                let a648 = {
                    let a646 = {
                        let a634 = 1u64;
                        let a635 = 1u64;
                        nat_add(a634, a635)?
                    };
                    let a647 = {
                        let a644 = {
                            let a642 = {
                                let a636 = 1u64;
                                let a637 = 1u64;
                                nat_add(a636, a637)?
                            };
                            let a643 = {
                                let a640 = {
                                    let a638 = 1u64;
                                    let a639 = 1u64;
                                    nat_add(a638, a639)?
                                };
                                let a641 = 1u64;
                                nat_mul(a640, a641)?
                            };
                            nat_mul(a642, a643)?
                        };
                        let a645 = 1u64;
                        nat_add(a644, a645)?
                    };
                    nat_mul(a646, a647)?
                };
                let a649 = 1u64;
                nat_add(a648, a649)?
            };
            nat_mul(a650, a651)?
        })?;
        nat_add(a652, a653)
    } else {
        if if f25(v0)? {
            true
        } else {
            f26(v1)?
        } {
            f28(f29(v0, v1)?)
        } else {
            if if f25(v1)? {
                true
            } else {
                f26(v0)?
            } {
                f30(f29(v0, v1)?)
            } else {
                f38(v0, v1)
            }
        }
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a654 = v0;
    let a655 = 0u64;
    if nat_eq(a654, a655) {
        Ok(0u64)
    } else {
        let v1: bool = {
            let a656 = v0;
            let a657 = 8388608u64;
            nat_lt(a656, a657)
        };
        let v2: u64 = if v1 {
            f5(v0, 1266679808u64)?
        } else {
            v0
        };
        let v3: u64 = f35(v2)?;
        let v4: u64 = if v1 {
            150u64
        } else {
            126u64
        };
        let v5: u64 = {
            let a658 = v4;
            let a659 = v3;
            if nat_lt(a658, a659) {
                let a662 = v3;
                let a663 = {
                    let a660 = v4;
                    let a661 = 1u64;
                    nat_add(a660, a661)?
                };
                nat_sub(a662, a663)
            } else {
                let a664 = v4;
                let a665 = v3;
                nat_sub(a664, a665)
            }
        };
        let v6: u64 = {
            let a666 = v5;
            let a667 = 2u64;
            if nat_lt(a666, a667) {
                0u64
            } else {
                let a668 = v5;
                let a669 = 4u64;
                if nat_lt(a668, a669) {
                    1u64
                } else {
                    let a670 = v5;
                    let a671 = 8u64;
                    if nat_lt(a670, a671) {
                        2u64
                    } else {
                        let a672 = v5;
                        let a673 = 16u64;
                        if nat_lt(a672, a673) {
                            5u64
                        } else {
                            let a674 = v5;
                            let a675 = 32u64;
                            if nat_lt(a674, a675) {
                                11u64
                            } else {
                                let a676 = v5;
                                let a677 = 64u64;
                                if nat_lt(a676, a677) {
                                    22u64
                                } else {
                                    let a678 = v5;
                                    let a679 = 128u64;
                                    if nat_lt(a678, a679) {
                                        44u64
                                    } else {
                                        88u64
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v7: u64 = {
            let a682 = {
                let a680 = 8388608u64;
                let a681 = f37(v2)?;
                nat_sub(a680, a681)
            };
            let a683 = v6;
            if nat_le(a682, a683) {
                1u64
            } else {
                0u64
            }
        };
        let a690 = {
            let a688 = {
                let a686 = {
                    let a684 = v3;
                    let a685 = 256u64;
                    nat_add(a684, a685)?
                };
                let a687 = v7;
                nat_add(a686, a687)?
            };
            let a689 = if v1 {
                26u64
            } else {
                2u64
            };
            nat_sub(a688, a689)
        };
        let a691 = 256u64;
        let a692 = 0u64;
        Ok(nat_rem(a690, a691, a692))
    }
}

pub fn f13(v0: u64) -> R<u64> {
    let a717 = v0;
    let a718 = f27({
        let a715 = {
            let a713 = {
                let a693 = 1u64;
                let a694 = 1u64;
                nat_add(a693, a694)?
            };
            let a714 = {
                let a711 = {
                    let a709 = {
                        let a695 = 1u64;
                        let a696 = 1u64;
                        nat_add(a695, a696)?
                    };
                    let a710 = {
                        let a707 = {
                            let a705 = {
                                let a697 = 1u64;
                                let a698 = 1u64;
                                nat_add(a697, a698)?
                            };
                            let a706 = {
                                let a703 = {
                                    let a701 = {
                                        let a699 = 1u64;
                                        let a700 = 1u64;
                                        nat_add(a699, a700)?
                                    };
                                    let a702 = 1u64;
                                    nat_mul(a701, a702)?
                                };
                                let a704 = 1u64;
                                nat_add(a703, a704)?
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
    let a719 = 0u64;
    Ok(nat_rem(a717, a718, a719))
}

pub fn f14(v0: u64) -> R<u64> {
    let a766 = {
        let a744 = v0;
        let a745 = f27({
            let a742 = {
                let a740 = {
                    let a720 = 1u64;
                    let a721 = 1u64;
                    nat_add(a720, a721)?
                };
                let a741 = {
                    let a738 = {
                        let a736 = {
                            let a722 = 1u64;
                            let a723 = 1u64;
                            nat_add(a722, a723)?
                        };
                        let a737 = {
                            let a734 = {
                                let a732 = {
                                    let a724 = 1u64;
                                    let a725 = 1u64;
                                    nat_add(a724, a725)?
                                };
                                let a733 = {
                                    let a730 = {
                                        let a728 = {
                                            let a726 = 1u64;
                                            let a727 = 1u64;
                                            nat_add(a726, a727)?
                                        };
                                        let a729 = 1u64;
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
        })?;
        nat_add(a744, a745)?
    };
    let a767 = f27({
        let a764 = {
            let a746 = 1u64;
            let a747 = 1u64;
            nat_add(a746, a747)?
        };
        let a765 = {
            let a762 = {
                let a748 = 1u64;
                let a749 = 1u64;
                nat_add(a748, a749)?
            };
            let a763 = {
                let a760 = {
                    let a750 = 1u64;
                    let a751 = 1u64;
                    nat_add(a750, a751)?
                };
                let a761 = {
                    let a758 = {
                        let a752 = 1u64;
                        let a753 = 1u64;
                        nat_add(a752, a753)?
                    };
                    let a759 = {
                        let a756 = {
                            let a754 = 1u64;
                            let a755 = 1u64;
                            nat_add(a754, a755)?
                        };
                        let a757 = 1u64;
                        nat_mul(a756, a757)?
                    };
                    nat_mul(a758, a759)?
                };
                nat_mul(a760, a761)?
            };
            nat_mul(a762, a763)?
        };
        nat_mul(a764, a765)?
    })?;
    let a768 = 0u64;
    Ok(nat_rem(a766, a767, a768))
}

pub fn f15(v0: bool, v1: u64) -> R<Adt1> {
    let a769 = v1;
    let a770 = 256u64;
    if nat_lt(a769, a770) {
        let v2: u64 = f39(v0, v1, 1u64)?;
        let v3: u64 = f39(v0, v1, if v0 {
            3u64
        } else {
            4u64
        })?;
        let v4: u64 = f39(v0, v1, if v0 {
            9u64
        } else {
            16u64
        })?;
        let v5: u64 = f39(v0, v1, if v0 {
            27u64
        } else {
            64u64
        })?;
        let v6: u64 = if v0 {
            f39(true, v1, 81u64)?
        } else {
            0u64
        };
        Ok(Adt1::C2({
            let a785 = v2;
            let a786 = {
                let a783 = 256u64;
                let a784 = {
                    let a781 = v3;
                    let a782 = {
                        let a779 = 256u64;
                        let a780 = {
                            let a777 = v4;
                            let a778 = {
                                let a775 = 256u64;
                                let a776 = {
                                    let a773 = v5;
                                    let a774 = {
                                        let a771 = 256u64;
                                        let a772 = v6;
                                        nat_mul(a771, a772)?
                                    };
                                    nat_add(a773, a774)?
                                };
                                nat_mul(a775, a776)?
                            };
                            nat_add(a777, a778)?
                        };
                        nat_mul(a779, a780)?
                    };
                    nat_add(a781, a782)?
                };
                nat_mul(a783, a784)?
            };
            nat_add(a785, a786)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f16(v0: bool, v1: u64) -> R<Adt1> {
    if v0 {
        let a787 = v1;
        let a788 = 32u64;
        if nat_lt(a787, a788) {
            Ok(Adt1::C1(v1, 32u64, 5u64, 1u64))
        } else {
            let a789 = v1;
            let a790 = 48u64;
            if nat_lt(a789, a790) {
                Ok(Adt1::C1({
                    let a793 = 160u64;
                    let a794 = {
                        let a791 = v1;
                        let a792 = 32u64;
                        nat_sub(a791, a792)
                    };
                    nat_add(a793, a794)?
                }, 16u64, 5u64, 1u64))
            } else {
                let a795 = v1;
                let a796 = 52u64;
                if nat_lt(a795, a796) {
                    Ok(Adt1::C1({
                        let a799 = 240u64;
                        let a800 = {
                            let a797 = v1;
                            let a798 = 48u64;
                            nat_sub(a797, a798)
                        };
                        nat_add(a799, a800)?
                    }, 4u64, 4u64, 3u64))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    } else {
        let a801 = v1;
        let a802 = 64u64;
        if nat_lt(a801, a802) {
            Ok(Adt1::C1({
                let a811 = {
                    let a806 = 128u64;
                    let a807 = {
                        let a803 = v1;
                        let a804 = 32u64;
                        let a805 = 0u64;
                        nat_quot(a803, a804, a805)
                    };
                    nat_mul(a806, a807)?
                };
                let a812 = {
                    let a808 = v1;
                    let a809 = 32u64;
                    let a810 = 0u64;
                    nat_rem(a808, a809, a810)
                };
                nat_add(a811, a812)?
            }, 32u64, 4u64, 1u64))
        } else {
            Ok(Adt1::C0)
        }
    }
}

pub fn f17(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a813 = v2;
    let a814 = 256u64;
    if if nat_lt(a813, a814) {
        let a815 = v1;
        let a816 = if v0 {
            5u64
        } else {
            4u64
        };
        nat_lt(a815, a816)
    } else {
        false
    } {
        let m817 = f20(true, 0u64, 0u64, v3, v4)?;
        match m817 {
            Adt1::C0 => {
                Ok(Adt1::C0)
            }
            Adt1::C1(_, _, _, _) => {
                Ok(Adt1::C0)
            }
            Adt1::C2(v5) => {
                let a818 = v5;
                let a819 = 3u64;
                if nat_lt(a818, a819) {
                    Ok(Adt1::C2(if v0 {
                        let a822 = {
                            let a820 = v2;
                            let a821 = 3u64;
                            nat_mul(a820, a821)?
                        };
                        let a823 = v5;
                        nat_add(a822, a823)?
                    } else {
                        let a828 = v2;
                        let a829 = {
                            let a826 = v5;
                            let a827 = f2({
                                let a824 = v1;
                                let a825 = 2u64;
                                nat_mul(a824, a825)?
                            });
                            nat_mul(a826, a827)?
                        };
                        nat_add(a828, a829)?
                    }))
                } else {
                    Ok(Adt1::C0)
                }
            }
            Adt1::C3(_, _) => {
                Ok(Adt1::C0)
            }
            Adt1::C4(_, _) => {
                Ok(Adt1::C0)
            }
            Adt1::C5(_, _) => {
                Ok(Adt1::C0)
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f18(v0: bool, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = f13(v2)?;
    let a830 = v2;
    let a831 = 4294967296u64;
    if if nat_lt(a830, a831) {
        let a832 = v3;
        let a833 = 2139095040u64;
        nat_lt(a832, a833)
    } else {
        false
    } {
        Ok(Adt1::C2(if v0 {
            let a834 = v1;
            let a835 = v3;
            if nat_lt(a834, a835) {
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

pub fn f19(v0: bool, v1: u64) -> R<Adt1> {
    let v2: u64 = if v0 {
        v1
    } else {
        f11(v1, 1124073472u64)?
    };
    let v3: u64 = if v0 {
        let a836 = v2;
        let a837 = 0u64;
        if nat_eq(a836, a837) {
            0u64
        } else {
            f11(1065353216u64, v2)?
        }
    } else {
        0u64
    };
    let a838 = v3;
    let a839 = 2139095040u64;
    if nat_lt(a838, a839) {
        Ok(Adt1::C3(f21(v2)?, v3))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a840 = v2;
    let a841 = 256u64;
    if if nat_lt(a840, a841) {
        let a842 = v1;
        let a843 = if v0 {
            4u64
        } else {
            8u64
        };
        nat_lt(a842, a843)
    } else {
        false
    } {
        let v5: u64 = if v0 {
            f5(v3, v4)?
        } else {
            v3
        };
        let v6: u64 = f13(v5)?;
        let v7: u64 = if v0 {
            let a844 = v6;
            let a845 = 1056964608u64;
            if nat_lt(a844, a845) {
                1u64
            } else {
                let a846 = v5;
                let a847 = 2147483648u64;
                if nat_lt(a846, a847) {
                    let a848 = v6;
                    let a849 = 1069547520u64;
                    if nat_lt(a848, a849) {
                        2u64
                    } else {
                        3u64
                    }
                } else {
                    0u64
                }
            }
        } else {
            let a850 = v3;
            let a851 = 2147483648u64;
            if if nat_lt(a850, a851) {
                true
            } else {
                let a852 = v6;
                let a853 = 0u64;
                nat_eq(a852, a853)
            } {
                1u64
            } else {
                0u64
            }
        };
        Ok(Adt1::C2({
            let a858 = v2;
            let a859 = {
                let a856 = v7;
                let a857 = f2(if v0 {
                    let a854 = v1;
                    let a855 = 2u64;
                    nat_mul(a854, a855)?
                } else {
                    v1
                });
                nat_mul(a856, a857)?
            };
            nat_add(a858, a859)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f21(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a860 = f33(v0)?;
        let a861 = 32768u64;
        nat_mul(a860, a861)?
    };
    let v2: u64 = f35(v0)?;
    let a880 = v1;
    let a881 = {
        let a862 = v2;
        let a863 = 255u64;
        if nat_eq(a862, a863) {
            let a864 = f37(v0)?;
            let a865 = 0u64;
            if nat_eq(a864, a865) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a866 = 143u64;
            let a867 = v2;
            if nat_le(a866, a867) {
                31744u64
            } else {
                let a868 = v2;
                let a869 = 102u64;
                if nat_lt(a868, a869) {
                    0u64
                } else {
                    let a870 = 113u64;
                    let a871 = v2;
                    if nat_le(a870, a871) {
                        let a876 = {
                            let a874 = {
                                let a872 = v2;
                                let a873 = 113u64;
                                nat_sub(a872, a873)
                            };
                            let a875 = 1024u64;
                            nat_mul(a874, a875)?
                        };
                        let a877 = f40(f36(v0)?, 8192u64)?;
                        nat_add(a876, a877)?
                    } else {
                        f40(f36(v0)?, f27({
                            let a878 = 126u64;
                            let a879 = v2;
                            nat_sub(a878, a879)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a880, a881)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a888 = {
        let a885 = {
            let a882 = v1;
            let a883 = v2;
            let a884 = 0u64;
            nat_quot(a882, a883, a884)
        };
        let a886 = 2u64;
        let a887 = 0u64;
        nat_rem(a885, a886, a887)
    };
    let a889 = 1u64;
    if nat_eq(a888, a889) {
        Ok(v0)
    } else {
        let a890 = 240u64;
        let a891 = v0;
        nat_add(a890, a891)
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a898 = {
        let a895 = {
            let a892 = v1;
            let a893 = v2;
            let a894 = 0u64;
            nat_quot(a892, a893, a894)
        };
        let a896 = 2u64;
        let a897 = 0u64;
        nat_rem(a895, a896, a897)
    };
    let a899 = 1u64;
    if nat_eq(a898, a899) {
        let a900 = 16u64;
        let a901 = v0;
        nat_add(a900, a901)
    } else {
        Ok(v0)
    }
}

pub fn f24(v0: u64) -> R<bool> {
    let a944 = f35(v0)?;
    let a945 = {
        let a942 = {
            let a940 = {
                let a902 = 1u64;
                let a903 = 1u64;
                nat_add(a902, a903)?
            };
            let a941 = {
                let a938 = {
                    let a936 = {
                        let a904 = 1u64;
                        let a905 = 1u64;
                        nat_add(a904, a905)?
                    };
                    let a937 = {
                        let a934 = {
                            let a932 = {
                                let a906 = 1u64;
                                let a907 = 1u64;
                                nat_add(a906, a907)?
                            };
                            let a933 = {
                                let a930 = {
                                    let a928 = {
                                        let a908 = 1u64;
                                        let a909 = 1u64;
                                        nat_add(a908, a909)?
                                    };
                                    let a929 = {
                                        let a926 = {
                                            let a924 = {
                                                let a910 = 1u64;
                                                let a911 = 1u64;
                                                nat_add(a910, a911)?
                                            };
                                            let a925 = {
                                                let a922 = {
                                                    let a920 = {
                                                        let a912 = 1u64;
                                                        let a913 = 1u64;
                                                        nat_add(a912, a913)?
                                                    };
                                                    let a921 = {
                                                        let a918 = {
                                                            let a916 = {
                                                                let a914 = 1u64;
                                                                let a915 = 1u64;
                                                                nat_add(a914, a915)?
                                                            };
                                                            let a917 = 1u64;
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
                                    };
                                    nat_mul(a928, a929)?
                                };
                                let a931 = 1u64;
                                nat_add(a930, a931)?
                            };
                            nat_mul(a932, a933)?
                        };
                        let a935 = 1u64;
                        nat_add(a934, a935)?
                    };
                    nat_mul(a936, a937)?
                };
                let a939 = 1u64;
                nat_add(a938, a939)?
            };
            nat_mul(a940, a941)?
        };
        let a943 = 1u64;
        nat_add(a942, a943)?
    };
    if nat_eq(a944, a945) {
        let a946 = 0u64;
        let a947 = f37(v0)?;
        Ok(nat_lt(a946, a947))
    } else {
        Ok(false)
    }
}

pub fn f25(v0: u64) -> R<bool> {
    let a990 = f35(v0)?;
    let a991 = {
        let a988 = {
            let a986 = {
                let a948 = 1u64;
                let a949 = 1u64;
                nat_add(a948, a949)?
            };
            let a987 = {
                let a984 = {
                    let a982 = {
                        let a950 = 1u64;
                        let a951 = 1u64;
                        nat_add(a950, a951)?
                    };
                    let a983 = {
                        let a980 = {
                            let a978 = {
                                let a952 = 1u64;
                                let a953 = 1u64;
                                nat_add(a952, a953)?
                            };
                            let a979 = {
                                let a976 = {
                                    let a974 = {
                                        let a954 = 1u64;
                                        let a955 = 1u64;
                                        nat_add(a954, a955)?
                                    };
                                    let a975 = {
                                        let a972 = {
                                            let a970 = {
                                                let a956 = 1u64;
                                                let a957 = 1u64;
                                                nat_add(a956, a957)?
                                            };
                                            let a971 = {
                                                let a968 = {
                                                    let a966 = {
                                                        let a958 = 1u64;
                                                        let a959 = 1u64;
                                                        nat_add(a958, a959)?
                                                    };
                                                    let a967 = {
                                                        let a964 = {
                                                            let a962 = {
                                                                let a960 = 1u64;
                                                                let a961 = 1u64;
                                                                nat_add(a960, a961)?
                                                            };
                                                            let a963 = 1u64;
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
                                    };
                                    nat_mul(a974, a975)?
                                };
                                let a977 = 1u64;
                                nat_add(a976, a977)?
                            };
                            nat_mul(a978, a979)?
                        };
                        let a981 = 1u64;
                        nat_add(a980, a981)?
                    };
                    nat_mul(a982, a983)?
                };
                let a985 = 1u64;
                nat_add(a984, a985)?
            };
            nat_mul(a986, a987)?
        };
        let a989 = 1u64;
        nat_add(a988, a989)?
    };
    if nat_eq(a990, a991) {
        let a992 = f37(v0)?;
        let a993 = 0u64;
        Ok(nat_eq(a992, a993))
    } else {
        Ok(false)
    }
}

pub fn f26(v0: u64) -> R<bool> {
    let a994 = f13(v0)?;
    let a995 = 0u64;
    Ok(nat_eq(a994, a995))
}

pub fn f27(v0: u64) -> R<u64> {
    f41(Fn0::F42, v0, 1u64)
}

pub fn f28(v0: u64) -> R<u64> {
    let a1062 = f30(v0)?;
    let a1063 = {
        let a1060 = {
            let a1036 = {
                let a1034 = {
                    let a996 = 1u64;
                    let a997 = 1u64;
                    nat_add(a996, a997)?
                };
                let a1035 = {
                    let a1032 = {
                        let a1030 = {
                            let a998 = 1u64;
                            let a999 = 1u64;
                            nat_add(a998, a999)?
                        };
                        let a1031 = {
                            let a1028 = {
                                let a1026 = {
                                    let a1000 = 1u64;
                                    let a1001 = 1u64;
                                    nat_add(a1000, a1001)?
                                };
                                let a1027 = {
                                    let a1024 = {
                                        let a1022 = {
                                            let a1002 = 1u64;
                                            let a1003 = 1u64;
                                            nat_add(a1002, a1003)?
                                        };
                                        let a1023 = {
                                            let a1020 = {
                                                let a1018 = {
                                                    let a1004 = 1u64;
                                                    let a1005 = 1u64;
                                                    nat_add(a1004, a1005)?
                                                };
                                                let a1019 = {
                                                    let a1016 = {
                                                        let a1014 = {
                                                            let a1006 = 1u64;
                                                            let a1007 = 1u64;
                                                            nat_add(a1006, a1007)?
                                                        };
                                                        let a1015 = {
                                                            let a1012 = {
                                                                let a1010 = {
                                                                    let a1008 = 1u64;
                                                                    let a1009 = 1u64;
                                                                    nat_add(a1008, a1009)?
                                                                };
                                                                let a1011 = 1u64;
                                                                nat_mul(a1010, a1011)?
                                                            };
                                                            let a1013 = 1u64;
                                                            nat_add(a1012, a1013)?
                                                        };
                                                        nat_mul(a1014, a1015)?
                                                    };
                                                    let a1017 = 1u64;
                                                    nat_add(a1016, a1017)?
                                                };
                                                nat_mul(a1018, a1019)?
                                            };
                                            let a1021 = 1u64;
                                            nat_add(a1020, a1021)?
                                        };
                                        nat_mul(a1022, a1023)?
                                    };
                                    let a1025 = 1u64;
                                    nat_add(a1024, a1025)?
                                };
                                nat_mul(a1026, a1027)?
                            };
                            let a1029 = 1u64;
                            nat_add(a1028, a1029)?
                        };
                        nat_mul(a1030, a1031)?
                    };
                    let a1033 = 1u64;
                    nat_add(a1032, a1033)?
                };
                nat_mul(a1034, a1035)?
            };
            let a1037 = 1u64;
            nat_add(a1036, a1037)?
        };
        let a1061 = f27({
            let a1058 = {
                let a1056 = {
                    let a1038 = 1u64;
                    let a1039 = 1u64;
                    nat_add(a1038, a1039)?
                };
                let a1057 = {
                    let a1054 = {
                        let a1052 = {
                            let a1040 = 1u64;
                            let a1041 = 1u64;
                            nat_add(a1040, a1041)?
                        };
                        let a1053 = {
                            let a1050 = {
                                let a1048 = {
                                    let a1042 = 1u64;
                                    let a1043 = 1u64;
                                    nat_add(a1042, a1043)?
                                };
                                let a1049 = {
                                    let a1046 = {
                                        let a1044 = 1u64;
                                        let a1045 = 1u64;
                                        nat_add(a1044, a1045)?
                                    };
                                    let a1047 = 1u64;
                                    nat_mul(a1046, a1047)?
                                };
                                nat_mul(a1048, a1049)?
                            };
                            let a1051 = 1u64;
                            nat_add(a1050, a1051)?
                        };
                        nat_mul(a1052, a1053)?
                    };
                    let a1055 = 1u64;
                    nat_add(a1054, a1055)?
                };
                nat_mul(a1056, a1057)?
            };
            let a1059 = 1u64;
            nat_add(a1058, a1059)?
        })?;
        nat_mul(a1060, a1061)?
    };
    nat_add(a1062, a1063)
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a1070 = {
        let a1064 = f33(v0)?;
        let a1065 = f33(v1)?;
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
    let a1072 = 0u64;
    Ok(nat_rem(a1070, a1071, a1072))
}

pub fn f30(v0: u64) -> R<u64> {
    let a1097 = v0;
    let a1098 = f27({
        let a1095 = {
            let a1093 = {
                let a1073 = 1u64;
                let a1074 = 1u64;
                nat_add(a1073, a1074)?
            };
            let a1094 = {
                let a1091 = {
                    let a1089 = {
                        let a1075 = 1u64;
                        let a1076 = 1u64;
                        nat_add(a1075, a1076)?
                    };
                    let a1090 = {
                        let a1087 = {
                            let a1085 = {
                                let a1077 = 1u64;
                                let a1078 = 1u64;
                                nat_add(a1077, a1078)?
                            };
                            let a1086 = {
                                let a1083 = {
                                    let a1081 = {
                                        let a1079 = 1u64;
                                        let a1080 = 1u64;
                                        nat_add(a1079, a1080)?
                                    };
                                    let a1082 = 1u64;
                                    nat_mul(a1081, a1082)?
                                };
                                let a1084 = 1u64;
                                nat_add(a1083, a1084)?
                            };
                            nat_mul(a1085, a1086)?
                        };
                        let a1088 = 1u64;
                        nat_add(a1087, a1088)?
                    };
                    nat_mul(a1089, a1090)?
                };
                let a1092 = 1u64;
                nat_add(a1091, a1092)?
            };
            nat_mul(a1093, a1094)?
        };
        let a1096 = 1u64;
        nat_add(a1095, a1096)?
    })?;
    nat_mul(a1097, a1098)
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f43(v0)?;
    let v3: Adt2 = f43(v1)?;
    f44(f29(v0, v1)?, {
        let a1153 = {
            let a1101 = match v2.clone() {
                Adt2::C0(h1099, _, _, _) => {
                    h1099
                }
            };
            let a1102 = match v3.clone() {
                Adt2::C0(h1100, _, _, _) => {
                    h1100
                }
            };
            nat_add(a1101, a1102)?
        };
        let a1154 = {
            let a1151 = {
                let a1149 = {
                    let a1103 = 1u64;
                    let a1104 = 1u64;
                    nat_add(a1103, a1104)?
                };
                let a1150 = {
                    let a1147 = {
                        let a1145 = {
                            let a1105 = 1u64;
                            let a1106 = 1u64;
                            nat_add(a1105, a1106)?
                        };
                        let a1146 = {
                            let a1143 = {
                                let a1141 = {
                                    let a1107 = 1u64;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                };
                                let a1142 = {
                                    let a1139 = {
                                        let a1137 = {
                                            let a1109 = 1u64;
                                            let a1110 = 1u64;
                                            nat_add(a1109, a1110)?
                                        };
                                        let a1138 = {
                                            let a1135 = {
                                                let a1133 = {
                                                    let a1111 = 1u64;
                                                    let a1112 = 1u64;
                                                    nat_add(a1111, a1112)?
                                                };
                                                let a1134 = {
                                                    let a1131 = {
                                                        let a1129 = {
                                                            let a1113 = 1u64;
                                                            let a1114 = 1u64;
                                                            nat_add(a1113, a1114)?
                                                        };
                                                        let a1130 = {
                                                            let a1127 = {
                                                                let a1125 = {
                                                                    let a1115 = 1u64;
                                                                    let a1116 = 1u64;
                                                                    nat_add(a1115, a1116)?
                                                                };
                                                                let a1126 = {
                                                                    let a1123 = {
                                                                        let a1117 = 1u64;
                                                                        let a1118 = 1u64;
                                                                        nat_add(a1117, a1118)?
                                                                    };
                                                                    let a1124 = {
                                                                        let a1121 = {
                                                                            let a1119 = 1u64;
                                                                            let a1120 = 1u64;
                                                                            nat_add(a1119, a1120)?
                                                                        };
                                                                        let a1122 = 1u64;
                                                                        nat_mul(a1121, a1122)?
                                                                    };
                                                                    nat_mul(a1123, a1124)?
                                                                };
                                                                nat_mul(a1125, a1126)?
                                                            };
                                                            let a1128 = 1u64;
                                                            nat_add(a1127, a1128)?
                                                        };
                                                        nat_mul(a1129, a1130)?
                                                    };
                                                    let a1132 = 1u64;
                                                    nat_add(a1131, a1132)?
                                                };
                                                nat_mul(a1133, a1134)?
                                            };
                                            let a1136 = 1u64;
                                            nat_add(a1135, a1136)?
                                        };
                                        nat_mul(a1137, a1138)?
                                    };
                                    let a1140 = 1u64;
                                    nat_add(a1139, a1140)?
                                };
                                nat_mul(a1141, a1142)?
                            };
                            let a1144 = 1u64;
                            nat_add(a1143, a1144)?
                        };
                        nat_mul(a1145, a1146)?
                    };
                    let a1148 = 1u64;
                    nat_add(a1147, a1148)?
                };
                nat_mul(a1149, a1150)?
            };
            let a1152 = 1u64;
            nat_add(a1151, a1152)?
        };
        nat_sub(a1153, a1154)
    }, f45({
        let a1157 = match v2 {
            Adt2::C0(_, h1155, _, _) => {
                h1155
            }
        };
        let a1158 = match v3 {
            Adt2::C0(_, h1156, _, _) => {
                h1156
            }
        };
        nat_mul(a1157, a1158)?
    }, {
        let a1175 = {
            let a1159 = 1u64;
            let a1160 = 1u64;
            nat_add(a1159, a1160)?
        };
        let a1176 = {
            let a1173 = {
                let a1161 = 1u64;
                let a1162 = 1u64;
                nat_add(a1161, a1162)?
            };
            let a1174 = {
                let a1171 = {
                    let a1169 = {
                        let a1163 = 1u64;
                        let a1164 = 1u64;
                        nat_add(a1163, a1164)?
                    };
                    let a1170 = {
                        let a1167 = {
                            let a1165 = 1u64;
                            let a1166 = 1u64;
                            nat_add(a1165, a1166)?
                        };
                        let a1168 = 1u64;
                        nat_mul(a1167, a1168)?
                    };
                    nat_mul(a1169, a1170)?
                };
                let a1172 = 1u64;
                nat_add(a1171, a1172)?
            };
            nat_mul(a1173, a1174)?
        };
        nat_mul(a1175, a1176)?
    })?)
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h1177, _) = f46(Fn1::F47, {
        let a1196 = {
            let a1178 = 1u64;
            let a1179 = 1u64;
            nat_add(a1178, a1179)?
        };
        let a1197 = {
            let a1194 = {
                let a1180 = 1u64;
                let a1181 = 1u64;
                nat_add(a1180, a1181)?
            };
            let a1195 = {
                let a1192 = {
                    let a1182 = 1u64;
                    let a1183 = 1u64;
                    nat_add(a1182, a1183)?
                };
                let a1193 = {
                    let a1190 = {
                        let a1184 = 1u64;
                        let a1185 = 1u64;
                        nat_add(a1184, a1185)?
                    };
                    let a1191 = {
                        let a1188 = {
                            let a1186 = 1u64;
                            let a1187 = 1u64;
                            nat_add(a1186, a1187)?
                        };
                        let a1189 = 1u64;
                        nat_mul(a1188, a1189)?
                    };
                    nat_mul(a1190, a1191)?
                };
                nat_mul(a1192, a1193)?
            };
            nat_mul(a1194, a1195)?
        };
        nat_mul(a1196, a1197)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h1177)
}

pub fn f33(v0: u64) -> R<u64> {
    let a1222 = v0;
    let a1223 = f27({
        let a1220 = {
            let a1218 = {
                let a1198 = 1u64;
                let a1199 = 1u64;
                nat_add(a1198, a1199)?
            };
            let a1219 = {
                let a1216 = {
                    let a1214 = {
                        let a1200 = 1u64;
                        let a1201 = 1u64;
                        nat_add(a1200, a1201)?
                    };
                    let a1215 = {
                        let a1212 = {
                            let a1210 = {
                                let a1202 = 1u64;
                                let a1203 = 1u64;
                                nat_add(a1202, a1203)?
                            };
                            let a1211 = {
                                let a1208 = {
                                    let a1206 = {
                                        let a1204 = 1u64;
                                        let a1205 = 1u64;
                                        nat_add(a1204, a1205)?
                                    };
                                    let a1207 = 1u64;
                                    nat_mul(a1206, a1207)?
                                };
                                let a1209 = 1u64;
                                nat_add(a1208, a1209)?
                            };
                            nat_mul(a1210, a1211)?
                        };
                        let a1213 = 1u64;
                        nat_add(a1212, a1213)?
                    };
                    nat_mul(a1214, a1215)?
                };
                let a1217 = 1u64;
                nat_add(a1216, a1217)?
            };
            nat_mul(a1218, a1219)?
        };
        let a1221 = 1u64;
        nat_add(a1220, a1221)?
    })?;
    let a1224 = 0u64;
    Ok(nat_quot(a1222, a1223, a1224))
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f48(v0)?;
    let v3: u64 = {
        let a1237 = f36(v0)?;
        let a1238 = {
            let a1235 = {
                let a1225 = 1u64;
                let a1226 = 1u64;
                nat_add(a1225, a1226)?
            };
            let a1236 = {
                let a1233 = {
                    let a1227 = 1u64;
                    let a1228 = 1u64;
                    nat_add(a1227, a1228)?
                };
                let a1234 = {
                    let a1231 = {
                        let a1229 = 1u64;
                        let a1230 = 1u64;
                        nat_add(a1229, a1230)?
                    };
                    let a1232 = 1u64;
                    nat_mul(a1231, a1232)?
                };
                nat_mul(a1233, a1234)?
            };
            nat_mul(a1235, a1236)?
        };
        nat_mul(a1237, a1238)?
    };
    let v4: u64 = f45({
        let a1251 = f36(v1)?;
        let a1252 = {
            let a1249 = {
                let a1239 = 1u64;
                let a1240 = 1u64;
                nat_add(a1239, a1240)?
            };
            let a1250 = {
                let a1247 = {
                    let a1241 = 1u64;
                    let a1242 = 1u64;
                    nat_add(a1241, a1242)?
                };
                let a1248 = {
                    let a1245 = {
                        let a1243 = 1u64;
                        let a1244 = 1u64;
                        nat_add(a1243, a1244)?
                    };
                    let a1246 = 1u64;
                    nat_mul(a1245, a1246)?
                };
                nat_mul(a1247, a1248)?
            };
            nat_mul(a1249, a1250)?
        };
        nat_mul(a1251, a1252)?
    }, {
        let a1253 = v2;
        let a1254 = f48(v1)?;
        nat_sub(a1253, a1254)
    })?;
    let a1255 = f33(v0)?;
    let a1256 = f33(v1)?;
    if nat_eq(a1255, a1256) {
        f44(f33(v0)?, v2, {
            let a1257 = v3;
            let a1258 = v4;
            nat_add(a1257, a1258)?
        })
    } else {
        let a1259 = v3;
        let a1260 = v4;
        if nat_eq(a1259, a1260) {
            Ok(0u64)
        } else {
            f44(f33(v0)?, v2, {
                let a1261 = v3;
                let a1262 = v4;
                nat_sub(a1261, a1262)
            })
        }
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a1285 = f13(v0)?;
    let a1286 = f27({
        let a1283 = {
            let a1281 = {
                let a1263 = 1u64;
                let a1264 = 1u64;
                nat_add(a1263, a1264)?
            };
            let a1282 = {
                let a1279 = {
                    let a1277 = {
                        let a1265 = 1u64;
                        let a1266 = 1u64;
                        nat_add(a1265, a1266)?
                    };
                    let a1278 = {
                        let a1275 = {
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
                        let a1276 = 1u64;
                        nat_add(a1275, a1276)?
                    };
                    nat_mul(a1277, a1278)?
                };
                let a1280 = 1u64;
                nat_add(a1279, a1280)?
            };
            nat_mul(a1281, a1282)?
        };
        let a1284 = 1u64;
        nat_add(a1283, a1284)?
    })?;
    let a1287 = 0u64;
    Ok(nat_quot(a1285, a1286, a1287))
}

pub fn f36(v0: u64) -> R<u64> {
    let a1312 = f37(v0)?;
    let a1313 = {
        let a1288 = f35(v0)?;
        let a1289 = 0u64;
        if nat_eq(a1288, a1289) {
            0u64
        } else {
            f27({
                let a1310 = {
                    let a1308 = {
                        let a1290 = 1u64;
                        let a1291 = 1u64;
                        nat_add(a1290, a1291)?
                    };
                    let a1309 = {
                        let a1306 = {
                            let a1304 = {
                                let a1292 = 1u64;
                                let a1293 = 1u64;
                                nat_add(a1292, a1293)?
                            };
                            let a1305 = {
                                let a1302 = {
                                    let a1300 = {
                                        let a1294 = 1u64;
                                        let a1295 = 1u64;
                                        nat_add(a1294, a1295)?
                                    };
                                    let a1301 = {
                                        let a1298 = {
                                            let a1296 = 1u64;
                                            let a1297 = 1u64;
                                            nat_add(a1296, a1297)?
                                        };
                                        let a1299 = 1u64;
                                        nat_mul(a1298, a1299)?
                                    };
                                    nat_mul(a1300, a1301)?
                                };
                                let a1303 = 1u64;
                                nat_add(a1302, a1303)?
                            };
                            nat_mul(a1304, a1305)?
                        };
                        let a1307 = 1u64;
                        nat_add(a1306, a1307)?
                    };
                    nat_mul(a1308, a1309)?
                };
                let a1311 = 1u64;
                nat_add(a1310, a1311)?
            })?
        }
    };
    nat_add(a1312, a1313)
}

pub fn f37(v0: u64) -> R<u64> {
    let a1336 = v0;
    let a1337 = f27({
        let a1334 = {
            let a1332 = {
                let a1314 = 1u64;
                let a1315 = 1u64;
                nat_add(a1314, a1315)?
            };
            let a1333 = {
                let a1330 = {
                    let a1328 = {
                        let a1316 = 1u64;
                        let a1317 = 1u64;
                        nat_add(a1316, a1317)?
                    };
                    let a1329 = {
                        let a1326 = {
                            let a1324 = {
                                let a1318 = 1u64;
                                let a1319 = 1u64;
                                nat_add(a1318, a1319)?
                            };
                            let a1325 = {
                                let a1322 = {
                                    let a1320 = 1u64;
                                    let a1321 = 1u64;
                                    nat_add(a1320, a1321)?
                                };
                                let a1323 = 1u64;
                                nat_mul(a1322, a1323)?
                            };
                            nat_mul(a1324, a1325)?
                        };
                        let a1327 = 1u64;
                        nat_add(a1326, a1327)?
                    };
                    nat_mul(a1328, a1329)?
                };
                let a1331 = 1u64;
                nat_add(a1330, a1331)?
            };
            nat_mul(a1332, a1333)?
        };
        let a1335 = 1u64;
        nat_add(a1334, a1335)?
    })?;
    let a1338 = 0u64;
    Ok(nat_rem(a1336, a1337, a1338))
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f43(v0)?;
    let v3: Adt2 = f43(v1)?;
    let v4: bool = {
        let a1341 = match v2.clone() {
            Adt2::C0(_, h1339, _, _) => {
                h1339
            }
        };
        let a1342 = match v3.clone() {
            Adt2::C0(_, h1340, _, _) => {
                h1340
            }
        };
        nat_lt(a1341, a1342)
    };
    let v5: u64 = {
        let a1386 = match v2.clone() {
            Adt2::C0(_, h1343, _, _) => {
                h1343
            }
        };
        let a1387 = f27(if v4 {
            let a1364 = {
                let a1362 = {
                    let a1344 = 1u64;
                    let a1345 = 1u64;
                    nat_add(a1344, a1345)?
                };
                let a1363 = {
                    let a1360 = {
                        let a1358 = {
                            let a1346 = 1u64;
                            let a1347 = 1u64;
                            nat_add(a1346, a1347)?
                        };
                        let a1359 = {
                            let a1356 = {
                                let a1348 = 1u64;
                                let a1349 = 1u64;
                                nat_add(a1348, a1349)?
                            };
                            let a1357 = {
                                let a1354 = {
                                    let a1352 = {
                                        let a1350 = 1u64;
                                        let a1351 = 1u64;
                                        nat_add(a1350, a1351)?
                                    };
                                    let a1353 = 1u64;
                                    nat_mul(a1352, a1353)?
                                };
                                let a1355 = 1u64;
                                nat_add(a1354, a1355)?
                            };
                            nat_mul(a1356, a1357)?
                        };
                        nat_mul(a1358, a1359)?
                    };
                    let a1361 = 1u64;
                    nat_add(a1360, a1361)?
                };
                nat_mul(a1362, a1363)?
            };
            let a1365 = 1u64;
            nat_add(a1364, a1365)?
        } else {
            let a1384 = {
                let a1366 = 1u64;
                let a1367 = 1u64;
                nat_add(a1366, a1367)?
            };
            let a1385 = {
                let a1382 = {
                    let a1380 = {
                        let a1368 = 1u64;
                        let a1369 = 1u64;
                        nat_add(a1368, a1369)?
                    };
                    let a1381 = {
                        let a1378 = {
                            let a1370 = 1u64;
                            let a1371 = 1u64;
                            nat_add(a1370, a1371)?
                        };
                        let a1379 = {
                            let a1376 = {
                                let a1374 = {
                                    let a1372 = 1u64;
                                    let a1373 = 1u64;
                                    nat_add(a1372, a1373)?
                                };
                                let a1375 = 1u64;
                                nat_mul(a1374, a1375)?
                            };
                            let a1377 = 1u64;
                            nat_add(a1376, a1377)?
                        };
                        nat_mul(a1378, a1379)?
                    };
                    nat_mul(a1380, a1381)?
                };
                let a1383 = 1u64;
                nat_add(a1382, a1383)?
            };
            nat_mul(a1384, a1385)?
        })?;
        nat_mul(a1386, a1387)?
    };
    let v6: u64 = {
        let a1444 = {
            let a1442 = {
                let a1439 = match v2 {
                    Adt2::C0(h1388, _, _, _) => {
                        h1388
                    }
                };
                let a1440 = {
                    let a1437 = {
                        let a1435 = {
                            let a1389 = 1u64;
                            let a1390 = 1u64;
                            nat_add(a1389, a1390)?
                        };
                        let a1436 = {
                            let a1433 = {
                                let a1431 = {
                                    let a1391 = 1u64;
                                    let a1392 = 1u64;
                                    nat_add(a1391, a1392)?
                                };
                                let a1432 = {
                                    let a1429 = {
                                        let a1427 = {
                                            let a1393 = 1u64;
                                            let a1394 = 1u64;
                                            nat_add(a1393, a1394)?
                                        };
                                        let a1428 = {
                                            let a1425 = {
                                                let a1423 = {
                                                    let a1395 = 1u64;
                                                    let a1396 = 1u64;
                                                    nat_add(a1395, a1396)?
                                                };
                                                let a1424 = {
                                                    let a1421 = {
                                                        let a1419 = {
                                                            let a1397 = 1u64;
                                                            let a1398 = 1u64;
                                                            nat_add(a1397, a1398)?
                                                        };
                                                        let a1420 = {
                                                            let a1417 = {
                                                                let a1415 = {
                                                                    let a1399 = 1u64;
                                                                    let a1400 = 1u64;
                                                                    nat_add(a1399, a1400)?
                                                                };
                                                                let a1416 = {
                                                                    let a1413 = {
                                                                        let a1411 = {
                                                                            let a1401 = 1u64;
                                                                            let a1402 = 1u64;
                                                                            nat_add(a1401, a1402)?
                                                                        };
                                                                        let a1412 = {
                                                                            let a1409 = {
                                                                                let a1403 = 1u64;
                                                                                let a1404 = 1u64;
                                                                                nat_add(a1403, a1404)?
                                                                            };
                                                                            let a1410 = {
                                                                                let a1407 = {
                                                                                    let a1405 = 1u64;
                                                                                    let a1406 = 1u64;
                                                                                    nat_add(a1405, a1406)?
                                                                                };
                                                                                let a1408 = 1u64;
                                                                                nat_mul(a1407, a1408)?
                                                                            };
                                                                            nat_mul(a1409, a1410)?
                                                                        };
                                                                        nat_mul(a1411, a1412)?
                                                                    };
                                                                    let a1414 = 1u64;
                                                                    nat_add(a1413, a1414)?
                                                                };
                                                                nat_mul(a1415, a1416)?
                                                            };
                                                            let a1418 = 1u64;
                                                            nat_add(a1417, a1418)?
                                                        };
                                                        nat_mul(a1419, a1420)?
                                                    };
                                                    let a1422 = 1u64;
                                                    nat_add(a1421, a1422)?
                                                };
                                                nat_mul(a1423, a1424)?
                                            };
                                            let a1426 = 1u64;
                                            nat_add(a1425, a1426)?
                                        };
                                        nat_mul(a1427, a1428)?
                                    };
                                    let a1430 = 1u64;
                                    nat_add(a1429, a1430)?
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
                nat_add(a1439, a1440)?
            };
            let a1443 = match v3.clone() {
                Adt2::C0(h1441, _, _, _) => {
                    h1441
                }
            };
            nat_sub(a1442, a1443)
        };
        let a1445 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a1444, a1445)
    };
    let v7: u64 = {
        let a1447 = v5;
        let a1448 = match v3.clone() {
            Adt2::C0(_, h1446, _, _) => {
                h1446
            }
        };
        let a1449 = 0u64;
        nat_quot(a1447, a1448, a1449)
    };
    let v8: bool = {
        let a1454 = 0u64;
        let a1455 = {
            let a1451 = v5;
            let a1452 = match v3 {
                Adt2::C0(_, h1450, _, _) => {
                    h1450
                }
            };
            let a1453 = 0u64;
            nat_rem(a1451, a1452, a1453)
        };
        nat_lt(a1454, a1455)
    };
    f44(f29(v0, v1)?, v6, f49(v7, v8)?)
}

pub fn f39(v0: bool, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = if v0 {
        let a1463 = {
            let a1461 = 3u64;
            let a1462 = {
                let a1458 = {
                    let a1456 = v1;
                    let a1457 = v2;
                    nat_mul(a1456, a1457)?
                };
                let a1459 = 256u64;
                let a1460 = 0u64;
                nat_rem(a1458, a1459, a1460)
            };
            nat_mul(a1461, a1462)?
        };
        let a1464 = 256u64;
        let a1465 = 0u64;
        nat_quot(a1463, a1464, a1465)
    } else {
        let a1469 = {
            let a1466 = v1;
            let a1467 = v2;
            let a1468 = 0u64;
            nat_quot(a1466, a1467, a1468)
        };
        let a1470 = 4u64;
        let a1471 = 0u64;
        nat_rem(a1469, a1470, a1471)
    };
    let a1474 = {
        let a1472 = v3;
        let a1473 = 255u64;
        nat_add(a1472, a1473)?
    };
    let a1475 = 256u64;
    let a1476 = 0u64;
    Ok(nat_rem(a1474, a1475, a1476))
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1477 = v0;
        let a1478 = v1;
        let a1479 = 0u64;
        nat_quot(a1477, a1478, a1479)
    };
    let v3: u64 = {
        let a1483 = {
            let a1480 = v0;
            let a1481 = v1;
            let a1482 = 0u64;
            nat_rem(a1480, a1481, a1482)
        };
        let a1484 = 2u64;
        nat_mul(a1483, a1484)?
    };
    let a1494 = v2;
    let a1495 = {
        let a1485 = v1;
        let a1486 = v3;
        if if nat_lt(a1485, a1486) {
            true
        } else {
            let a1487 = v3;
            let a1488 = v1;
            if nat_eq(a1487, a1488) {
                let a1492 = {
                    let a1489 = v2;
                    let a1490 = 2u64;
                    let a1491 = 0u64;
                    nat_rem(a1489, a1490, a1491)
                };
                let a1493 = 1u64;
                nat_eq(a1492, a1493)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1494, a1495)
}

pub fn f41(v0: Fn0, p1499: u64, p1500: u64) -> R<u64> {
    let m1498 = (p1499, p1500);
    {
        let mut m1498 = m1498;
        loop {
            let step = {
                let (v1, v2) = m1498;
                let m1496 = v1;
                if m1496 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1496 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1497: Fn0 = v0.clone();
                        c1497.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: u64) -> R<u64> {
    let a1501 = v0;
    let a1502 = v0;
    nat_add(a1501, a1502)
}

pub fn f43(v0: u64) -> R<Adt2> {
    f32(f48(v0)?, f36(v0)?, 0u64, f27({
        let a1523 = {
            let a1521 = {
                let a1503 = 1u64;
                let a1504 = 1u64;
                nat_add(a1503, a1504)?
            };
            let a1522 = {
                let a1519 = {
                    let a1517 = {
                        let a1505 = 1u64;
                        let a1506 = 1u64;
                        nat_add(a1505, a1506)?
                    };
                    let a1518 = {
                        let a1515 = {
                            let a1513 = {
                                let a1507 = 1u64;
                                let a1508 = 1u64;
                                nat_add(a1507, a1508)?
                            };
                            let a1514 = {
                                let a1511 = {
                                    let a1509 = 1u64;
                                    let a1510 = 1u64;
                                    nat_add(a1509, a1510)?
                                };
                                let a1512 = 1u64;
                                nat_mul(a1511, a1512)?
                            };
                            nat_mul(a1513, a1514)?
                        };
                        let a1516 = 1u64;
                        nat_add(a1515, a1516)?
                    };
                    nat_mul(a1517, a1518)?
                };
                let a1520 = 1u64;
                nat_add(a1519, a1520)?
            };
            nat_mul(a1521, a1522)?
        };
        let a1524 = 1u64;
        nat_add(a1523, a1524)?
    })?)
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1547 = f27({
        let a1545 = {
            let a1543 = {
                let a1525 = 1u64;
                let a1526 = 1u64;
                nat_add(a1525, a1526)?
            };
            let a1544 = {
                let a1541 = {
                    let a1539 = {
                        let a1527 = 1u64;
                        let a1528 = 1u64;
                        nat_add(a1527, a1528)?
                    };
                    let a1540 = {
                        let a1537 = {
                            let a1529 = 1u64;
                            let a1530 = 1u64;
                            nat_add(a1529, a1530)?
                        };
                        let a1538 = {
                            let a1535 = {
                                let a1533 = {
                                    let a1531 = 1u64;
                                    let a1532 = 1u64;
                                    nat_add(a1531, a1532)?
                                };
                                let a1534 = 1u64;
                                nat_mul(a1533, a1534)?
                            };
                            let a1536 = 1u64;
                            nat_add(a1535, a1536)?
                        };
                        nat_mul(a1537, a1538)?
                    };
                    nat_mul(a1539, a1540)?
                };
                let a1542 = 1u64;
                nat_add(a1541, a1542)?
            };
            nat_mul(a1543, a1544)?
        };
        let a1546 = 1u64;
        nat_add(a1545, a1546)?
    })?;
    let a1548 = v2;
    if nat_le(a1547, a1548) {
        f50(v0, {
            let a1549 = v1;
            let a1550 = 1u64;
            nat_add(a1549, a1550)?
        }, f45(v2, 1u64)?)
    } else {
        let v3: Adt2 = f32(v1, v2, {
            let a1587 = {
                let a1585 = {
                    let a1551 = 1u64;
                    let a1552 = 1u64;
                    nat_add(a1551, a1552)?
                };
                let a1586 = {
                    let a1583 = {
                        let a1553 = 1u64;
                        let a1554 = 1u64;
                        nat_add(a1553, a1554)?
                    };
                    let a1584 = {
                        let a1581 = {
                            let a1555 = 1u64;
                            let a1556 = 1u64;
                            nat_add(a1555, a1556)?
                        };
                        let a1582 = {
                            let a1579 = {
                                let a1557 = 1u64;
                                let a1558 = 1u64;
                                nat_add(a1557, a1558)?
                            };
                            let a1580 = {
                                let a1577 = {
                                    let a1559 = 1u64;
                                    let a1560 = 1u64;
                                    nat_add(a1559, a1560)?
                                };
                                let a1578 = {
                                    let a1575 = {
                                        let a1561 = 1u64;
                                        let a1562 = 1u64;
                                        nat_add(a1561, a1562)?
                                    };
                                    let a1576 = {
                                        let a1573 = {
                                            let a1563 = 1u64;
                                            let a1564 = 1u64;
                                            nat_add(a1563, a1564)?
                                        };
                                        let a1574 = {
                                            let a1571 = {
                                                let a1565 = 1u64;
                                                let a1566 = 1u64;
                                                nat_add(a1565, a1566)?
                                            };
                                            let a1572 = {
                                                let a1569 = {
                                                    let a1567 = 1u64;
                                                    let a1568 = 1u64;
                                                    nat_add(a1567, a1568)?
                                                };
                                                let a1570 = 1u64;
                                                nat_mul(a1569, a1570)?
                                            };
                                            nat_mul(a1571, a1572)?
                                        };
                                        nat_mul(a1573, a1574)?
                                    };
                                    nat_mul(a1575, a1576)?
                                };
                                nat_mul(a1577, a1578)?
                            };
                            nat_mul(a1579, a1580)?
                        };
                        nat_mul(a1581, a1582)?
                    };
                    nat_mul(a1583, a1584)?
                };
                nat_mul(a1585, a1586)?
            };
            let a1588 = 1u64;
            nat_add(a1587, a1588)?
        }, f27({
            let a1607 = {
                let a1589 = 1u64;
                let a1590 = 1u64;
                nat_add(a1589, a1590)?
            };
            let a1608 = {
                let a1605 = {
                    let a1603 = {
                        let a1591 = 1u64;
                        let a1592 = 1u64;
                        nat_add(a1591, a1592)?
                    };
                    let a1604 = {
                        let a1601 = {
                            let a1593 = 1u64;
                            let a1594 = 1u64;
                            nat_add(a1593, a1594)?
                        };
                        let a1602 = {
                            let a1599 = {
                                let a1597 = {
                                    let a1595 = 1u64;
                                    let a1596 = 1u64;
                                    nat_add(a1595, a1596)?
                                };
                                let a1598 = 1u64;
                                nat_mul(a1597, a1598)?
                            };
                            let a1600 = 1u64;
                            nat_add(a1599, a1600)?
                        };
                        nat_mul(a1601, a1602)?
                    };
                    nat_mul(a1603, a1604)?
                };
                let a1606 = 1u64;
                nat_add(a1605, a1606)?
            };
            nat_mul(a1607, a1608)?
        })?)?;
        f50(v0, match v3.clone() {
            Adt2::C0(h1609, _, _, _) => {
                h1609
            }
        }, match v3 {
            Adt2::C0(_, h1610, _, _) => {
                h1610
            }
        })
    }
}

pub fn f45(v0: u64, v1: u64) -> R<u64> {
    let a1611 = v1;
    let a1612 = 0u64;
    if nat_eq(a1611, a1612) {
        Ok(v0)
    } else {
        let a1637 = {
            let a1635 = {
                let a1613 = 1u64;
                let a1614 = 1u64;
                nat_add(a1613, a1614)?
            };
            let a1636 = {
                let a1633 = {
                    let a1615 = 1u64;
                    let a1616 = 1u64;
                    nat_add(a1615, a1616)?
                };
                let a1634 = {
                    let a1631 = {
                        let a1617 = 1u64;
                        let a1618 = 1u64;
                        nat_add(a1617, a1618)?
                    };
                    let a1632 = {
                        let a1629 = {
                            let a1619 = 1u64;
                            let a1620 = 1u64;
                            nat_add(a1619, a1620)?
                        };
                        let a1630 = {
                            let a1627 = {
                                let a1621 = 1u64;
                                let a1622 = 1u64;
                                nat_add(a1621, a1622)?
                            };
                            let a1628 = {
                                let a1625 = {
                                    let a1623 = 1u64;
                                    let a1624 = 1u64;
                                    nat_add(a1623, a1624)?
                                };
                                let a1626 = 1u64;
                                nat_mul(a1625, a1626)?
                            };
                            nat_mul(a1627, a1628)?
                        };
                        nat_mul(a1629, a1630)?
                    };
                    nat_mul(a1631, a1632)?
                };
                nat_mul(a1633, a1634)?
            };
            nat_mul(a1635, a1636)?
        };
        let a1638 = v1;
        if nat_le(a1637, a1638) {
            let a1639 = v0;
            let a1640 = 0u64;
            if nat_eq(a1639, a1640) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f27(v1)?;
            f49({
                let a1641 = v0;
                let a1642 = v2;
                let a1643 = 0u64;
                nat_quot(a1641, a1642, a1643)
            }, {
                let a1647 = 0u64;
                let a1648 = {
                    let a1644 = v0;
                    let a1645 = v2;
                    let a1646 = 0u64;
                    nat_rem(a1644, a1645, a1646)
                };
                nat_lt(a1647, a1648)
            })
        }
    }
}

pub fn f46(v0: Fn1, p1653: u64, p1654: Adt2) -> R<(Adt2, bool)> {
    let m1652 = (p1653, p1654);
    {
        let mut m1652 = m1652;
        loop {
            let step = {
                let (v1, v2) = m1652;
                let m1649 = v1;
                if m1649 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1649 - 1;
                    let m1651 = {
                        let c1650: Fn1 = v0.clone();
                        c1650.apply(v2.clone())?
                    };
                    match m1651 {
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
                Ok(next) => { m1652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Adt2) -> R<Option<Adt2>> {
    f51(v0)
}

pub fn f48(v0: u64) -> R<u64> {
    let a1693 = {
        let a1655 = f35(v0)?;
        let a1656 = 0u64;
        if nat_eq(a1655, a1656) {
            1u64
        } else {
            f35(v0)?
        }
    };
    let a1694 = {
        let a1691 = {
            let a1657 = 1u64;
            let a1658 = 1u64;
            nat_add(a1657, a1658)?
        };
        let a1692 = {
            let a1689 = {
                let a1659 = 1u64;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            };
            let a1690 = {
                let a1687 = {
                    let a1661 = 1u64;
                    let a1662 = 1u64;
                    nat_add(a1661, a1662)?
                };
                let a1688 = {
                    let a1685 = {
                        let a1663 = 1u64;
                        let a1664 = 1u64;
                        nat_add(a1663, a1664)?
                    };
                    let a1686 = {
                        let a1683 = {
                            let a1665 = 1u64;
                            let a1666 = 1u64;
                            nat_add(a1665, a1666)?
                        };
                        let a1684 = {
                            let a1681 = {
                                let a1667 = 1u64;
                                let a1668 = 1u64;
                                nat_add(a1667, a1668)?
                            };
                            let a1682 = {
                                let a1679 = {
                                    let a1669 = 1u64;
                                    let a1670 = 1u64;
                                    nat_add(a1669, a1670)?
                                };
                                let a1680 = {
                                    let a1677 = {
                                        let a1671 = 1u64;
                                        let a1672 = 1u64;
                                        nat_add(a1671, a1672)?
                                    };
                                    let a1678 = {
                                        let a1675 = {
                                            let a1673 = 1u64;
                                            let a1674 = 1u64;
                                            nat_add(a1673, a1674)?
                                        };
                                        let a1676 = 1u64;
                                        nat_mul(a1675, a1676)?
                                    };
                                    nat_mul(a1677, a1678)?
                                };
                                nat_mul(a1679, a1680)?
                            };
                            nat_mul(a1681, a1682)?
                        };
                        nat_mul(a1683, a1684)?
                    };
                    nat_mul(a1685, a1686)?
                };
                nat_mul(a1687, a1688)?
            };
            nat_mul(a1689, a1690)?
        };
        nat_mul(a1691, a1692)?
    };
    nat_add(a1693, a1694)
}

pub fn f49(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1702 = {
            let a1699 = v0;
            let a1700 = {
                let a1697 = {
                    let a1695 = 1u64;
                    let a1696 = 1u64;
                    nat_add(a1695, a1696)?
                };
                let a1698 = 1u64;
                nat_mul(a1697, a1698)?
            };
            let a1701 = 0u64;
            nat_rem(a1699, a1700, a1701)
        };
        let a1703 = 0u64;
        nat_eq(a1702, a1703)
    } else {
        false
    } {
        let a1704 = v0;
        let a1705 = 1u64;
        nat_add(a1704, a1705)
    } else {
        Ok(v0)
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1744 = v1;
    let a1745 = {
        let a1742 = {
            let a1740 = {
                let a1706 = 1u64;
                let a1707 = 1u64;
                nat_add(a1706, a1707)?
            };
            let a1741 = {
                let a1738 = {
                    let a1708 = 1u64;
                    let a1709 = 1u64;
                    nat_add(a1708, a1709)?
                };
                let a1739 = {
                    let a1736 = {
                        let a1710 = 1u64;
                        let a1711 = 1u64;
                        nat_add(a1710, a1711)?
                    };
                    let a1737 = {
                        let a1734 = {
                            let a1712 = 1u64;
                            let a1713 = 1u64;
                            nat_add(a1712, a1713)?
                        };
                        let a1735 = {
                            let a1732 = {
                                let a1714 = 1u64;
                                let a1715 = 1u64;
                                nat_add(a1714, a1715)?
                            };
                            let a1733 = {
                                let a1730 = {
                                    let a1716 = 1u64;
                                    let a1717 = 1u64;
                                    nat_add(a1716, a1717)?
                                };
                                let a1731 = {
                                    let a1728 = {
                                        let a1718 = 1u64;
                                        let a1719 = 1u64;
                                        nat_add(a1718, a1719)?
                                    };
                                    let a1729 = {
                                        let a1726 = {
                                            let a1720 = 1u64;
                                            let a1721 = 1u64;
                                            nat_add(a1720, a1721)?
                                        };
                                        let a1727 = {
                                            let a1724 = {
                                                let a1722 = 1u64;
                                                let a1723 = 1u64;
                                                nat_add(a1722, a1723)?
                                            };
                                            let a1725 = 1u64;
                                            nat_mul(a1724, a1725)?
                                        };
                                        nat_mul(a1726, a1727)?
                                    };
                                    nat_mul(a1728, a1729)?
                                };
                                nat_mul(a1730, a1731)?
                            };
                            nat_mul(a1732, a1733)?
                        };
                        nat_mul(a1734, a1735)?
                    };
                    nat_mul(a1736, a1737)?
                };
                nat_mul(a1738, a1739)?
            };
            nat_mul(a1740, a1741)?
        };
        let a1743 = 1u64;
        nat_add(a1742, a1743)?
    };
    if nat_lt(a1744, a1745) {
        f52(v0, {
            let a1782 = {
                let a1780 = {
                    let a1746 = 1u64;
                    let a1747 = 1u64;
                    nat_add(a1746, a1747)?
                };
                let a1781 = {
                    let a1778 = {
                        let a1748 = 1u64;
                        let a1749 = 1u64;
                        nat_add(a1748, a1749)?
                    };
                    let a1779 = {
                        let a1776 = {
                            let a1750 = 1u64;
                            let a1751 = 1u64;
                            nat_add(a1750, a1751)?
                        };
                        let a1777 = {
                            let a1774 = {
                                let a1752 = 1u64;
                                let a1753 = 1u64;
                                nat_add(a1752, a1753)?
                            };
                            let a1775 = {
                                let a1772 = {
                                    let a1754 = 1u64;
                                    let a1755 = 1u64;
                                    nat_add(a1754, a1755)?
                                };
                                let a1773 = {
                                    let a1770 = {
                                        let a1756 = 1u64;
                                        let a1757 = 1u64;
                                        nat_add(a1756, a1757)?
                                    };
                                    let a1771 = {
                                        let a1768 = {
                                            let a1758 = 1u64;
                                            let a1759 = 1u64;
                                            nat_add(a1758, a1759)?
                                        };
                                        let a1769 = {
                                            let a1766 = {
                                                let a1760 = 1u64;
                                                let a1761 = 1u64;
                                                nat_add(a1760, a1761)?
                                            };
                                            let a1767 = {
                                                let a1764 = {
                                                    let a1762 = 1u64;
                                                    let a1763 = 1u64;
                                                    nat_add(a1762, a1763)?
                                                };
                                                let a1765 = 1u64;
                                                nat_mul(a1764, a1765)?
                                            };
                                            nat_mul(a1766, a1767)?
                                        };
                                        nat_mul(a1768, a1769)?
                                    };
                                    nat_mul(a1770, a1771)?
                                };
                                nat_mul(a1772, a1773)?
                            };
                            nat_mul(a1774, a1775)?
                        };
                        nat_mul(a1776, a1777)?
                    };
                    nat_mul(a1778, a1779)?
                };
                nat_mul(a1780, a1781)?
            };
            let a1783 = 1u64;
            nat_add(a1782, a1783)?
        }, f45(v2, {
            let a1822 = {
                let a1820 = {
                    let a1818 = {
                        let a1784 = 1u64;
                        let a1785 = 1u64;
                        nat_add(a1784, a1785)?
                    };
                    let a1819 = {
                        let a1816 = {
                            let a1786 = 1u64;
                            let a1787 = 1u64;
                            nat_add(a1786, a1787)?
                        };
                        let a1817 = {
                            let a1814 = {
                                let a1788 = 1u64;
                                let a1789 = 1u64;
                                nat_add(a1788, a1789)?
                            };
                            let a1815 = {
                                let a1812 = {
                                    let a1790 = 1u64;
                                    let a1791 = 1u64;
                                    nat_add(a1790, a1791)?
                                };
                                let a1813 = {
                                    let a1810 = {
                                        let a1792 = 1u64;
                                        let a1793 = 1u64;
                                        nat_add(a1792, a1793)?
                                    };
                                    let a1811 = {
                                        let a1808 = {
                                            let a1794 = 1u64;
                                            let a1795 = 1u64;
                                            nat_add(a1794, a1795)?
                                        };
                                        let a1809 = {
                                            let a1806 = {
                                                let a1796 = 1u64;
                                                let a1797 = 1u64;
                                                nat_add(a1796, a1797)?
                                            };
                                            let a1807 = {
                                                let a1804 = {
                                                    let a1798 = 1u64;
                                                    let a1799 = 1u64;
                                                    nat_add(a1798, a1799)?
                                                };
                                                let a1805 = {
                                                    let a1802 = {
                                                        let a1800 = 1u64;
                                                        let a1801 = 1u64;
                                                        nat_add(a1800, a1801)?
                                                    };
                                                    let a1803 = 1u64;
                                                    nat_mul(a1802, a1803)?
                                                };
                                                nat_mul(a1804, a1805)?
                                            };
                                            nat_mul(a1806, a1807)?
                                        };
                                        nat_mul(a1808, a1809)?
                                    };
                                    nat_mul(a1810, a1811)?
                                };
                                nat_mul(a1812, a1813)?
                            };
                            nat_mul(a1814, a1815)?
                        };
                        nat_mul(a1816, a1817)?
                    };
                    nat_mul(a1818, a1819)?
                };
                let a1821 = 1u64;
                nat_add(a1820, a1821)?
            };
            let a1823 = v1;
            nat_sub(a1822, a1823)
        })?)
    } else {
        f52(v0, v1, v2)
    }
}

pub fn f51(v0: Adt2) -> R<Option<Adt2>> {
    let a1825 = 0u64;
    let a1826 = match v0.clone() {
        Adt2::C0(_, h1824, _, _) => {
            h1824
        }
    };
    if if if nat_lt(a1825, a1826) {
        let a1829 = match v0.clone() {
            Adt2::C0(_, h1827, _, _) => {
                h1827
            }
        };
        let a1830 = match v0.clone() {
            Adt2::C0(_, _, _, h1828) => {
                h1828
            }
        };
        nat_lt(a1829, a1830)
    } else {
        false
    } {
        let a1833 = match v0.clone() {
            Adt2::C0(_, _, h1831, _) => {
                h1831
            }
        };
        let a1834 = match v0.clone() {
            Adt2::C0(h1832, _, _, _) => {
                h1832
            }
        };
        nat_lt(a1833, a1834)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1836 = match v0.clone() {
                Adt2::C0(h1835, _, _, _) => {
                    h1835
                }
            };
            let a1837 = 1u64;
            nat_sub(a1836, a1837)
        }, {
            let a1840 = match v0.clone() {
                Adt2::C0(_, h1838, _, _) => {
                    h1838
                }
            };
            let a1841 = match v0.clone() {
                Adt2::C0(_, h1839, _, _) => {
                    h1839
                }
            };
            nat_add(a1840, a1841)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1842, _) => {
                h1842
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1843) => {
                h1843
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1856 = v2;
        let a1857 = {
            let a1854 = {
                let a1844 = 1u64;
                let a1845 = 1u64;
                nat_add(a1844, a1845)?
            };
            let a1855 = {
                let a1852 = {
                    let a1846 = 1u64;
                    let a1847 = 1u64;
                    nat_add(a1846, a1847)?
                };
                let a1853 = {
                    let a1850 = {
                        let a1848 = 1u64;
                        let a1849 = 1u64;
                        nat_add(a1848, a1849)?
                    };
                    let a1851 = 1u64;
                    nat_mul(a1850, a1851)?
                };
                nat_mul(a1852, a1853)?
            };
            nat_mul(a1854, a1855)?
        };
        let a1858 = 0u64;
        nat_rem(a1856, a1857, a1858)
    };
    let v4: u64 = {
        let a1871 = v2;
        let a1872 = {
            let a1869 = {
                let a1859 = 1u64;
                let a1860 = 1u64;
                nat_add(a1859, a1860)?
            };
            let a1870 = {
                let a1867 = {
                    let a1861 = 1u64;
                    let a1862 = 1u64;
                    nat_add(a1861, a1862)?
                };
                let a1868 = {
                    let a1865 = {
                        let a1863 = 1u64;
                        let a1864 = 1u64;
                        nat_add(a1863, a1864)?
                    };
                    let a1866 = 1u64;
                    nat_mul(a1865, a1866)?
                };
                nat_mul(a1867, a1868)?
            };
            nat_mul(a1869, a1870)?
        };
        let a1873 = 0u64;
        nat_quot(a1871, a1872, a1873)
    };
    let v5: bool = {
        let a1882 = {
            let a1880 = {
                let a1874 = 1u64;
                let a1875 = 1u64;
                nat_add(a1874, a1875)?
            };
            let a1881 = {
                let a1878 = {
                    let a1876 = 1u64;
                    let a1877 = 1u64;
                    nat_add(a1876, a1877)?
                };
                let a1879 = 1u64;
                nat_mul(a1878, a1879)?
            };
            nat_mul(a1880, a1881)?
        };
        let a1883 = v3;
        if nat_lt(a1882, a1883) {
            true
        } else {
            let a1892 = v3;
            let a1893 = {
                let a1890 = {
                    let a1884 = 1u64;
                    let a1885 = 1u64;
                    nat_add(a1884, a1885)?
                };
                let a1891 = {
                    let a1888 = {
                        let a1886 = 1u64;
                        let a1887 = 1u64;
                        nat_add(a1886, a1887)?
                    };
                    let a1889 = 1u64;
                    nat_mul(a1888, a1889)?
                };
                nat_mul(a1890, a1891)?
            };
            if nat_eq(a1892, a1893) {
                let a1901 = {
                    let a1898 = v4;
                    let a1899 = {
                        let a1896 = {
                            let a1894 = 1u64;
                            let a1895 = 1u64;
                            nat_add(a1894, a1895)?
                        };
                        let a1897 = 1u64;
                        nat_mul(a1896, a1897)?
                    };
                    let a1900 = 0u64;
                    nat_rem(a1898, a1899, a1900)
                };
                let a1902 = 1u64;
                nat_eq(a1901, a1902)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1903 = v4;
        let a1904 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1903, a1904)?
    };
    let v7: bool = {
        let a1933 = {
            let a1931 = {
                let a1907 = {
                    let a1905 = 1u64;
                    let a1906 = 1u64;
                    nat_add(a1905, a1906)?
                };
                let a1908 = 1u64;
                nat_mul(a1907, a1908)?
            };
            let a1932 = f27({
                let a1929 = {
                    let a1927 = {
                        let a1909 = 1u64;
                        let a1910 = 1u64;
                        nat_add(a1909, a1910)?
                    };
                    let a1928 = {
                        let a1925 = {
                            let a1923 = {
                                let a1911 = 1u64;
                                let a1912 = 1u64;
                                nat_add(a1911, a1912)?
                            };
                            let a1924 = {
                                let a1921 = {
                                    let a1919 = {
                                        let a1913 = 1u64;
                                        let a1914 = 1u64;
                                        nat_add(a1913, a1914)?
                                    };
                                    let a1920 = {
                                        let a1917 = {
                                            let a1915 = 1u64;
                                            let a1916 = 1u64;
                                            nat_add(a1915, a1916)?
                                        };
                                        let a1918 = 1u64;
                                        nat_mul(a1917, a1918)?
                                    };
                                    nat_mul(a1919, a1920)?
                                };
                                let a1922 = 1u64;
                                nat_add(a1921, a1922)?
                            };
                            nat_mul(a1923, a1924)?
                        };
                        let a1926 = 1u64;
                        nat_add(a1925, a1926)?
                    };
                    nat_mul(a1927, a1928)?
                };
                let a1930 = 1u64;
                nat_add(a1929, a1930)?
            })?;
            nat_mul(a1931, a1932)?
        };
        let a1934 = v6;
        nat_le(a1933, a1934)
    };
    let v8: u64 = if v7 {
        let a1939 = v6;
        let a1940 = {
            let a1937 = {
                let a1935 = 1u64;
                let a1936 = 1u64;
                nat_add(a1935, a1936)?
            };
            let a1938 = 1u64;
            nat_mul(a1937, a1938)?
        };
        let a1941 = 0u64;
        nat_quot(a1939, a1940, a1941)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1942 = v1;
        let a1943 = 1u64;
        nat_add(a1942, a1943)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1982 = v9;
        let a1983 = {
            let a1980 = {
                let a1978 = {
                    let a1944 = 1u64;
                    let a1945 = 1u64;
                    nat_add(a1944, a1945)?
                };
                let a1979 = {
                    let a1976 = {
                        let a1946 = 1u64;
                        let a1947 = 1u64;
                        nat_add(a1946, a1947)?
                    };
                    let a1977 = {
                        let a1974 = {
                            let a1948 = 1u64;
                            let a1949 = 1u64;
                            nat_add(a1948, a1949)?
                        };
                        let a1975 = {
                            let a1972 = {
                                let a1950 = 1u64;
                                let a1951 = 1u64;
                                nat_add(a1950, a1951)?
                            };
                            let a1973 = {
                                let a1970 = {
                                    let a1952 = 1u64;
                                    let a1953 = 1u64;
                                    nat_add(a1952, a1953)?
                                };
                                let a1971 = {
                                    let a1968 = {
                                        let a1954 = 1u64;
                                        let a1955 = 1u64;
                                        nat_add(a1954, a1955)?
                                    };
                                    let a1969 = {
                                        let a1966 = {
                                            let a1956 = 1u64;
                                            let a1957 = 1u64;
                                            nat_add(a1956, a1957)?
                                        };
                                        let a1967 = {
                                            let a1964 = {
                                                let a1958 = 1u64;
                                                let a1959 = 1u64;
                                                nat_add(a1958, a1959)?
                                            };
                                            let a1965 = {
                                                let a1962 = {
                                                    let a1960 = 1u64;
                                                    let a1961 = 1u64;
                                                    nat_add(a1960, a1961)?
                                                };
                                                let a1963 = 1u64;
                                                nat_mul(a1962, a1963)?
                                            };
                                            nat_mul(a1964, a1965)?
                                        };
                                        nat_mul(a1966, a1967)?
                                    };
                                    nat_mul(a1968, a1969)?
                                };
                                nat_mul(a1970, a1971)?
                            };
                            nat_mul(a1972, a1973)?
                        };
                        nat_mul(a1974, a1975)?
                    };
                    nat_mul(a1976, a1977)?
                };
                nat_mul(a1978, a1979)?
            };
            let a1981 = 1u64;
            nat_add(a1980, a1981)?
        };
        if if nat_eq(a1982, a1983) {
            let a2006 = v8;
            let a2007 = f27({
                let a2004 = {
                    let a2002 = {
                        let a1984 = 1u64;
                        let a1985 = 1u64;
                        nat_add(a1984, a1985)?
                    };
                    let a2003 = {
                        let a2000 = {
                            let a1998 = {
                                let a1986 = 1u64;
                                let a1987 = 1u64;
                                nat_add(a1986, a1987)?
                            };
                            let a1999 = {
                                let a1996 = {
                                    let a1994 = {
                                        let a1988 = 1u64;
                                        let a1989 = 1u64;
                                        nat_add(a1988, a1989)?
                                    };
                                    let a1995 = {
                                        let a1992 = {
                                            let a1990 = 1u64;
                                            let a1991 = 1u64;
                                            nat_add(a1990, a1991)?
                                        };
                                        let a1993 = 1u64;
                                        nat_mul(a1992, a1993)?
                                    };
                                    nat_mul(a1994, a1995)?
                                };
                                let a1997 = 1u64;
                                nat_add(a1996, a1997)?
                            };
                            nat_mul(a1998, a1999)?
                        };
                        let a2001 = 1u64;
                        nat_add(a2000, a2001)?
                    };
                    nat_mul(a2002, a2003)?
                };
                let a2005 = 1u64;
                nat_add(a2004, a2005)?
            })?;
            nat_lt(a2006, a2007)
        } else {
            false
        } {
            0u64
        } else {
            let a2044 = v9;
            let a2045 = {
                let a2042 = {
                    let a2008 = 1u64;
                    let a2009 = 1u64;
                    nat_add(a2008, a2009)?
                };
                let a2043 = {
                    let a2040 = {
                        let a2010 = 1u64;
                        let a2011 = 1u64;
                        nat_add(a2010, a2011)?
                    };
                    let a2041 = {
                        let a2038 = {
                            let a2012 = 1u64;
                            let a2013 = 1u64;
                            nat_add(a2012, a2013)?
                        };
                        let a2039 = {
                            let a2036 = {
                                let a2014 = 1u64;
                                let a2015 = 1u64;
                                nat_add(a2014, a2015)?
                            };
                            let a2037 = {
                                let a2034 = {
                                    let a2016 = 1u64;
                                    let a2017 = 1u64;
                                    nat_add(a2016, a2017)?
                                };
                                let a2035 = {
                                    let a2032 = {
                                        let a2018 = 1u64;
                                        let a2019 = 1u64;
                                        nat_add(a2018, a2019)?
                                    };
                                    let a2033 = {
                                        let a2030 = {
                                            let a2020 = 1u64;
                                            let a2021 = 1u64;
                                            nat_add(a2020, a2021)?
                                        };
                                        let a2031 = {
                                            let a2028 = {
                                                let a2022 = 1u64;
                                                let a2023 = 1u64;
                                                nat_add(a2022, a2023)?
                                            };
                                            let a2029 = {
                                                let a2026 = {
                                                    let a2024 = 1u64;
                                                    let a2025 = 1u64;
                                                    nat_add(a2024, a2025)?
                                                };
                                                let a2027 = 1u64;
                                                nat_mul(a2026, a2027)?
                                            };
                                            nat_mul(a2028, a2029)?
                                        };
                                        nat_mul(a2030, a2031)?
                                    };
                                    nat_mul(a2032, a2033)?
                                };
                                nat_mul(a2034, a2035)?
                            };
                            nat_mul(a2036, a2037)?
                        };
                        nat_mul(a2038, a2039)?
                    };
                    nat_mul(a2040, a2041)?
                };
                nat_mul(a2042, a2043)?
            };
            nat_sub(a2044, a2045)
        }
    };
    let a2046 = v8;
    let a2047 = 0u64;
    if nat_eq(a2046, a2047) {
        f30(v0)
    } else {
        let a2100 = {
            let a2098 = {
                let a2096 = {
                    let a2048 = 1u64;
                    let a2049 = 1u64;
                    nat_add(a2048, a2049)?
                };
                let a2097 = {
                    let a2094 = {
                        let a2092 = {
                            let a2050 = 1u64;
                            let a2051 = 1u64;
                            nat_add(a2050, a2051)?
                        };
                        let a2093 = {
                            let a2090 = {
                                let a2088 = {
                                    let a2052 = 1u64;
                                    let a2053 = 1u64;
                                    nat_add(a2052, a2053)?
                                };
                                let a2089 = {
                                    let a2086 = {
                                        let a2084 = {
                                            let a2054 = 1u64;
                                            let a2055 = 1u64;
                                            nat_add(a2054, a2055)?
                                        };
                                        let a2085 = {
                                            let a2082 = {
                                                let a2080 = {
                                                    let a2056 = 1u64;
                                                    let a2057 = 1u64;
                                                    nat_add(a2056, a2057)?
                                                };
                                                let a2081 = {
                                                    let a2078 = {
                                                        let a2076 = {
                                                            let a2058 = 1u64;
                                                            let a2059 = 1u64;
                                                            nat_add(a2058, a2059)?
                                                        };
                                                        let a2077 = {
                                                            let a2074 = {
                                                                let a2072 = {
                                                                    let a2060 = 1u64;
                                                                    let a2061 = 1u64;
                                                                    nat_add(a2060, a2061)?
                                                                };
                                                                let a2073 = {
                                                                    let a2070 = {
                                                                        let a2068 = {
                                                                            let a2062 = 1u64;
                                                                            let a2063 = 1u64;
                                                                            nat_add(a2062, a2063)?
                                                                        };
                                                                        let a2069 = {
                                                                            let a2066 = {
                                                                                let a2064 = 1u64;
                                                                                let a2065 = 1u64;
                                                                                nat_add(a2064, a2065)?
                                                                            };
                                                                            let a2067 = 1u64;
                                                                            nat_mul(a2066, a2067)?
                                                                        };
                                                                        nat_mul(a2068, a2069)?
                                                                    };
                                                                    let a2071 = 1u64;
                                                                    nat_add(a2070, a2071)?
                                                                };
                                                                nat_mul(a2072, a2073)?
                                                            };
                                                            let a2075 = 1u64;
                                                            nat_add(a2074, a2075)?
                                                        };
                                                        nat_mul(a2076, a2077)?
                                                    };
                                                    let a2079 = 1u64;
                                                    nat_add(a2078, a2079)?
                                                };
                                                nat_mul(a2080, a2081)?
                                            };
                                            let a2083 = 1u64;
                                            nat_add(a2082, a2083)?
                                        };
                                        nat_mul(a2084, a2085)?
                                    };
                                    let a2087 = 1u64;
                                    nat_add(a2086, a2087)?
                                };
                                nat_mul(a2088, a2089)?
                            };
                            let a2091 = 1u64;
                            nat_add(a2090, a2091)?
                        };
                        nat_mul(a2092, a2093)?
                    };
                    let a2095 = 1u64;
                    nat_add(a2094, a2095)?
                };
                nat_mul(a2096, a2097)?
            };
            let a2099 = 1u64;
            nat_add(a2098, a2099)?
        };
        let a2101 = v9;
        if nat_le(a2100, a2101) {
            f28(v0)
        } else {
            let a2153 = {
                let a2126 = f30(v0)?;
                let a2127 = {
                    let a2124 = v10;
                    let a2125 = f27({
                        let a2122 = {
                            let a2120 = {
                                let a2102 = 1u64;
                                let a2103 = 1u64;
                                nat_add(a2102, a2103)?
                            };
                            let a2121 = {
                                let a2118 = {
                                    let a2116 = {
                                        let a2104 = 1u64;
                                        let a2105 = 1u64;
                                        nat_add(a2104, a2105)?
                                    };
                                    let a2117 = {
                                        let a2114 = {
                                            let a2112 = {
                                                let a2106 = 1u64;
                                                let a2107 = 1u64;
                                                nat_add(a2106, a2107)?
                                            };
                                            let a2113 = {
                                                let a2110 = {
                                                    let a2108 = 1u64;
                                                    let a2109 = 1u64;
                                                    nat_add(a2108, a2109)?
                                                };
                                                let a2111 = 1u64;
                                                nat_mul(a2110, a2111)?
                                            };
                                            nat_mul(a2112, a2113)?
                                        };
                                        let a2115 = 1u64;
                                        nat_add(a2114, a2115)?
                                    };
                                    nat_mul(a2116, a2117)?
                                };
                                let a2119 = 1u64;
                                nat_add(a2118, a2119)?
                            };
                            nat_mul(a2120, a2121)?
                        };
                        let a2123 = 1u64;
                        nat_add(a2122, a2123)?
                    })?;
                    nat_mul(a2124, a2125)?
                };
                nat_add(a2126, a2127)?
            };
            let a2154 = {
                let a2150 = v8;
                let a2151 = f27({
                    let a2148 = {
                        let a2146 = {
                            let a2128 = 1u64;
                            let a2129 = 1u64;
                            nat_add(a2128, a2129)?
                        };
                        let a2147 = {
                            let a2144 = {
                                let a2142 = {
                                    let a2130 = 1u64;
                                    let a2131 = 1u64;
                                    nat_add(a2130, a2131)?
                                };
                                let a2143 = {
                                    let a2140 = {
                                        let a2138 = {
                                            let a2132 = 1u64;
                                            let a2133 = 1u64;
                                            nat_add(a2132, a2133)?
                                        };
                                        let a2139 = {
                                            let a2136 = {
                                                let a2134 = 1u64;
                                                let a2135 = 1u64;
                                                nat_add(a2134, a2135)?
                                            };
                                            let a2137 = 1u64;
                                            nat_mul(a2136, a2137)?
                                        };
                                        nat_mul(a2138, a2139)?
                                    };
                                    let a2141 = 1u64;
                                    nat_add(a2140, a2141)?
                                };
                                nat_mul(a2142, a2143)?
                            };
                            let a2145 = 1u64;
                            nat_add(a2144, a2145)?
                        };
                        nat_mul(a2146, a2147)?
                    };
                    let a2149 = 1u64;
                    nat_add(a2148, a2149)?
                })?;
                let a2152 = 0u64;
                nat_rem(a2150, a2151, a2152)
            };
            nat_add(a2153, a2154)
        }
    }
}
