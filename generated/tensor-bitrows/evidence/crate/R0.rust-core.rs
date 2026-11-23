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
    F36,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F36 => f36(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F41,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F41 => f41(p0),
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
        Adt0::C7(v69, v70) => {
            let a94 = v69;
            let a95 = 4294967296u64;
            if if nat_lt(a94, a95) {
                let a96 = v70;
                let a97 = 2139095040u64;
                nat_lt(a96, a97)
            } else {
                false
            } {
                Ok(Adt1::C2(f15(f5(v69, v70)?)?))
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
        Adt0::C11(v56, v57, v58) => {
            let a90 = v58;
            let a91 = 18u64;
            if if nat_eq(a90, a91) {
                let a92 = v57;
                let a93 = if v56 {
                    64u64
                } else {
                    128u64
                };
                nat_eq(a92, a93)
            } else {
                false
            } {
                Ok(Adt1::C2(if v56 {
                    4u64
                } else {
                    8u64
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C12(v59, v60, v61) => {
            f12(v59, v60, v61)
        }
        Adt0::C13(v62, v63) => {
            f13(v62, v63)
        }
        Adt0::C14(v64, v65, v66, v67, v68) => {
            f14(v64, v65, v66, v67, v68)
        }
        Adt0::C15(v41, v42, v43) => {
            let v44: u64 = if v41 {
                52u64
            } else {
                64u64
            };
            let a69 = v42;
            let a70 = 256u64;
            if if nat_eq(a69, a70) {
                let a73 = v43;
                let a74 = {
                    let a71 = v44;
                    let a72 = 2u64;
                    nat_add(a71, a72)?
                };
                nat_eq(a73, a74)
            } else {
                false
            } {
                Ok(Adt1::C2(v44))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C16(v45, v46) => {
            f10(v45, v46)
        }
        Adt0::C17(v47, v48, v49, v50, v51) => {
            f11(v47, v48, v49, v50, v51)
        }
        Adt0::C18(v52, v53, v54) => {
            let v55: u64 = {
                let a75 = v53;
                let a76 = v54;
                nat_mul(a75, a76)?
            };
            if v52 {
                let a77 = v55;
                let a78 = 243u64;
                if nat_lt(a77, a78) {
                    Ok(Adt1::C2({
                        let a83 = {
                            let a81 = {
                                let a79 = v55;
                                let a80 = 256u64;
                                nat_mul(a79, a80)?
                            };
                            let a82 = 242u64;
                            nat_add(a81, a82)?
                        };
                        let a84 = 243u64;
                        let a85 = 0u64;
                        nat_quot(a83, a84, a85)
                    }))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                let a86 = v53;
                let a87 = 1u64;
                if if nat_eq(a86, a87) {
                    let a88 = v54;
                    let a89 = 256u64;
                    nat_lt(a88, a89)
                } else {
                    false
                } {
                    Ok(Adt1::C2(v54))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a98 = 0u64;
    let a99 = v2;
    if if nat_lt(a98, a99) {
        let a100 = 0u64;
        let a101 = v3;
        nat_lt(a100, a101)
    } else {
        false
    } {
        Ok(Some({
            let a107 = {
                let a104 = {
                    let a102 = v0;
                    let a103 = v1;
                    nat_mul(a102, a103)?
                };
                let a105 = v2;
                let a106 = 0u64;
                nat_quot(a104, a105, a106)
            };
            let a108 = v3;
            let a109 = 0u64;
            nat_rem(a107, a108, a109)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a110 = v0;
    let a111 = 0u64;
    if nat_eq(a110, a111) {
        1u64
    } else {
        let a112 = v0;
        let a113 = 1u64;
        if nat_eq(a112, a113) {
            2u64
        } else {
            let a114 = v0;
            let a115 = 2u64;
            if nat_eq(a114, a115) {
                4u64
            } else {
                let a116 = v0;
                let a117 = 3u64;
                if nat_eq(a116, a117) {
                    8u64
                } else {
                    let a118 = v0;
                    let a119 = 4u64;
                    if nat_eq(a118, a119) {
                        16u64
                    } else {
                        let a120 = v0;
                        let a121 = 5u64;
                        if nat_eq(a120, a121) {
                            32u64
                        } else {
                            let a122 = v0;
                            let a123 = 6u64;
                            if nat_eq(a122, a123) {
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
    let a124 = v1;
    let a125 = 16u64;
    if if if nat_lt(a124, a125) {
        let a126 = v2;
        let a127 = 256u64;
        nat_lt(a126, a127)
    } else {
        false
    } {
        let a128 = v3;
        let a129 = 4294967296u64;
        nat_lt(a128, a129)
    } else {
        false
    } {
        let v4: u64 = {
            let a130 = v2;
            let a131 = 16u64;
            let a132 = 0u64;
            nat_rem(a130, a131, a132)
        };
        let v5: u64 = {
            let a133 = v2;
            let a134 = 16u64;
            let a135 = 0u64;
            nat_quot(a133, a134, a135)
        };
        if v0 {
            Ok(Adt1::C4({
                let a136 = 18568u64;
                let a137 = v4;
                nat_add(a136, a137)?
            }, {
                let a138 = 18568u64;
                let a139 = v5;
                nat_add(a138, a139)?
            }))
        } else {
            let v6: u64 = {
                let a145 = f2({
                    let a140 = v1;
                    let a141 = 8u64;
                    let a142 = 0u64;
                    nat_rem(a140, a141, a142)
                });
                let a146 = {
                    let a143 = v1;
                    let a144 = 8u64;
                    if nat_lt(a143, a144) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a145, a146)?
            };
            Ok(Adt1::C3(f16(v4, v3, v6)?, f16(v5, v3, {
                let a147 = v6;
                let a148 = 65536u64;
                nat_mul(a147, a148)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a149 = v0;
    let a150 = 16u64;
    if if if nat_lt(a149, a150) {
        let a151 = v1;
        let a152 = 256u64;
        nat_lt(a151, a152)
    } else {
        false
    } {
        let a153 = v2;
        let a154 = 4294967296u64;
        nat_lt(a153, a154)
    } else {
        false
    } {
        let v3: u64 = {
            let a160 = f2({
                let a155 = v0;
                let a156 = 8u64;
                let a157 = 0u64;
                nat_rem(a155, a156, a157)
            });
            let a161 = {
                let a158 = v0;
                let a159 = 8u64;
                if nat_lt(a158, a159) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a160, a161)?
        };
        Ok(Adt1::C3(f17({
            let a162 = v1;
            let a163 = 16u64;
            let a164 = 0u64;
            nat_rem(a162, a163, a164)
        }, v2, v3)?, f17({
            let a165 = v1;
            let a166 = 16u64;
            let a167 = 0u64;
            nat_quot(a165, a166, a167)
        }, v2, {
            let a168 = v3;
            let a169 = 65536u64;
            nat_mul(a168, a169)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        true
    } else {
        if f19(v0)? {
            f20(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f19(v1)? {
            f20(v0)?
        } else {
            false
        }
    } {
        let a256 = {
            let a234 = {
                let a210 = {
                    let a208 = {
                        let a170 = 1u64;
                        let a171 = 1u64;
                        nat_add(a170, a171)?
                    };
                    let a209 = {
                        let a206 = {
                            let a204 = {
                                let a172 = 1u64;
                                let a173 = 1u64;
                                nat_add(a172, a173)?
                            };
                            let a205 = {
                                let a202 = {
                                    let a200 = {
                                        let a174 = 1u64;
                                        let a175 = 1u64;
                                        nat_add(a174, a175)?
                                    };
                                    let a201 = {
                                        let a198 = {
                                            let a196 = {
                                                let a176 = 1u64;
                                                let a177 = 1u64;
                                                nat_add(a176, a177)?
                                            };
                                            let a197 = {
                                                let a194 = {
                                                    let a192 = {
                                                        let a178 = 1u64;
                                                        let a179 = 1u64;
                                                        nat_add(a178, a179)?
                                                    };
                                                    let a193 = {
                                                        let a190 = {
                                                            let a188 = {
                                                                let a180 = 1u64;
                                                                let a181 = 1u64;
                                                                nat_add(a180, a181)?
                                                            };
                                                            let a189 = {
                                                                let a186 = {
                                                                    let a184 = {
                                                                        let a182 = 1u64;
                                                                        let a183 = 1u64;
                                                                        nat_add(a182, a183)?
                                                                    };
                                                                    let a185 = 1u64;
                                                                    nat_mul(a184, a185)?
                                                                };
                                                                let a187 = 1u64;
                                                                nat_add(a186, a187)?
                                                            };
                                                            nat_mul(a188, a189)?
                                                        };
                                                        let a191 = 1u64;
                                                        nat_add(a190, a191)?
                                                    };
                                                    nat_mul(a192, a193)?
                                                };
                                                let a195 = 1u64;
                                                nat_add(a194, a195)?
                                            };
                                            nat_mul(a196, a197)?
                                        };
                                        let a199 = 1u64;
                                        nat_add(a198, a199)?
                                    };
                                    nat_mul(a200, a201)?
                                };
                                let a203 = 1u64;
                                nat_add(a202, a203)?
                            };
                            nat_mul(a204, a205)?
                        };
                        let a207 = 1u64;
                        nat_add(a206, a207)?
                    };
                    nat_mul(a208, a209)?
                };
                let a211 = 1u64;
                nat_add(a210, a211)?
            };
            let a235 = f21({
                let a232 = {
                    let a230 = {
                        let a212 = 1u64;
                        let a213 = 1u64;
                        nat_add(a212, a213)?
                    };
                    let a231 = {
                        let a228 = {
                            let a226 = {
                                let a214 = 1u64;
                                let a215 = 1u64;
                                nat_add(a214, a215)?
                            };
                            let a227 = {
                                let a224 = {
                                    let a222 = {
                                        let a216 = 1u64;
                                        let a217 = 1u64;
                                        nat_add(a216, a217)?
                                    };
                                    let a223 = {
                                        let a220 = {
                                            let a218 = 1u64;
                                            let a219 = 1u64;
                                            nat_add(a218, a219)?
                                        };
                                        let a221 = 1u64;
                                        nat_mul(a220, a221)?
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
            })?;
            nat_mul(a234, a235)?
        };
        let a257 = f21({
            let a254 = {
                let a236 = 1u64;
                let a237 = 1u64;
                nat_add(a236, a237)?
            };
            let a255 = {
                let a252 = {
                    let a250 = {
                        let a238 = 1u64;
                        let a239 = 1u64;
                        nat_add(a238, a239)?
                    };
                    let a251 = {
                        let a248 = {
                            let a246 = {
                                let a240 = 1u64;
                                let a241 = 1u64;
                                nat_add(a240, a241)?
                            };
                            let a247 = {
                                let a244 = {
                                    let a242 = 1u64;
                                    let a243 = 1u64;
                                    nat_add(a242, a243)?
                                };
                                let a245 = 1u64;
                                nat_mul(a244, a245)?
                            };
                            nat_mul(a246, a247)?
                        };
                        let a249 = 1u64;
                        nat_add(a248, a249)?
                    };
                    nat_mul(a250, a251)?
                };
                let a253 = 1u64;
                nat_add(a252, a253)?
            };
            nat_mul(a254, a255)?
        })?;
        nat_add(a256, a257)
    } else {
        if if f19(v0)? {
            true
        } else {
            f19(v1)?
        } {
            f22(f23(v0, v1)?)
        } else {
            if if f20(v0)? {
                true
            } else {
                f20(v1)?
            } {
                f24(f23(v0, v1)?)
            } else {
                f25(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a261 = {
            let a258 = v0;
            let a259 = 32768u64;
            let a260 = 0u64;
            nat_quot(a258, a259, a260)
        };
        let a262 = 2147483648u64;
        nat_mul(a261, a262)?
    };
    let v2: u64 = {
        let a266 = {
            let a263 = v0;
            let a264 = 32768u64;
            let a265 = 0u64;
            nat_rem(a263, a264, a265)
        };
        let a267 = 1024u64;
        let a268 = 0u64;
        nat_quot(a266, a267, a268)
    };
    let v3: u64 = {
        let a269 = v0;
        let a270 = 1024u64;
        let a271 = 0u64;
        nat_rem(a269, a270, a271)
    };
    let a306 = v1;
    let a307 = {
        let a272 = v2;
        let a273 = 31u64;
        if nat_eq(a272, a273) {
            let a282 = 2139095040u64;
            let a283 = {
                let a280 = {
                    let a274 = 0u64;
                    let a275 = v3;
                    if if nat_lt(a274, a275) {
                        let a276 = v3;
                        let a277 = 512u64;
                        nat_lt(a276, a277)
                    } else {
                        false
                    } {
                        let a278 = v3;
                        let a279 = 512u64;
                        nat_add(a278, a279)?
                    } else {
                        v3
                    }
                };
                let a281 = 8192u64;
                nat_mul(a280, a281)?
            };
            nat_add(a282, a283)?
        } else {
            let a284 = v2;
            let a285 = 0u64;
            if nat_eq(a284, a285) {
                let a286 = v3;
                let a287 = 0u64;
                if nat_eq(a286, a287) {
                    0u64
                } else {
                    let v4: Adt2 = f26(113u64, v3, 0u64, 1024u64)?;
                    let a296 = {
                        let a289 = match v4.clone() {
                            Adt2::C0(h288, _, _, _) => {
                                h288
                            }
                        };
                        let a290 = 8388608u64;
                        nat_mul(a289, a290)?
                    };
                    let a297 = {
                        let a294 = {
                            let a292 = match v4 {
                                Adt2::C0(_, h291, _, _) => {
                                    h291
                                }
                            };
                            let a293 = 1024u64;
                            nat_sub(a292, a293)
                        };
                        let a295 = 8192u64;
                        nat_mul(a294, a295)?
                    };
                    nat_add(a296, a297)?
                }
            } else {
                let a304 = {
                    let a300 = {
                        let a298 = v2;
                        let a299 = 112u64;
                        nat_add(a298, a299)?
                    };
                    let a301 = 8388608u64;
                    nat_mul(a300, a301)?
                };
                let a305 = {
                    let a302 = v3;
                    let a303 = 8192u64;
                    nat_mul(a302, a303)?
                };
                nat_add(a304, a305)?
            }
        }
    };
    nat_add(a306, a307)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        let a394 = {
            let a372 = {
                let a348 = {
                    let a346 = {
                        let a308 = 1u64;
                        let a309 = 1u64;
                        nat_add(a308, a309)?
                    };
                    let a347 = {
                        let a344 = {
                            let a342 = {
                                let a310 = 1u64;
                                let a311 = 1u64;
                                nat_add(a310, a311)?
                            };
                            let a343 = {
                                let a340 = {
                                    let a338 = {
                                        let a312 = 1u64;
                                        let a313 = 1u64;
                                        nat_add(a312, a313)?
                                    };
                                    let a339 = {
                                        let a336 = {
                                            let a334 = {
                                                let a314 = 1u64;
                                                let a315 = 1u64;
                                                nat_add(a314, a315)?
                                            };
                                            let a335 = {
                                                let a332 = {
                                                    let a330 = {
                                                        let a316 = 1u64;
                                                        let a317 = 1u64;
                                                        nat_add(a316, a317)?
                                                    };
                                                    let a331 = {
                                                        let a328 = {
                                                            let a326 = {
                                                                let a318 = 1u64;
                                                                let a319 = 1u64;
                                                                nat_add(a318, a319)?
                                                            };
                                                            let a327 = {
                                                                let a324 = {
                                                                    let a322 = {
                                                                        let a320 = 1u64;
                                                                        let a321 = 1u64;
                                                                        nat_add(a320, a321)?
                                                                    };
                                                                    let a323 = 1u64;
                                                                    nat_mul(a322, a323)?
                                                                };
                                                                let a325 = 1u64;
                                                                nat_add(a324, a325)?
                                                            };
                                                            nat_mul(a326, a327)?
                                                        };
                                                        let a329 = 1u64;
                                                        nat_add(a328, a329)?
                                                    };
                                                    nat_mul(a330, a331)?
                                                };
                                                let a333 = 1u64;
                                                nat_add(a332, a333)?
                                            };
                                            nat_mul(a334, a335)?
                                        };
                                        let a337 = 1u64;
                                        nat_add(a336, a337)?
                                    };
                                    nat_mul(a338, a339)?
                                };
                                let a341 = 1u64;
                                nat_add(a340, a341)?
                            };
                            nat_mul(a342, a343)?
                        };
                        let a345 = 1u64;
                        nat_add(a344, a345)?
                    };
                    nat_mul(a346, a347)?
                };
                let a349 = 1u64;
                nat_add(a348, a349)?
            };
            let a373 = f21({
                let a370 = {
                    let a368 = {
                        let a350 = 1u64;
                        let a351 = 1u64;
                        nat_add(a350, a351)?
                    };
                    let a369 = {
                        let a366 = {
                            let a364 = {
                                let a352 = 1u64;
                                let a353 = 1u64;
                                nat_add(a352, a353)?
                            };
                            let a365 = {
                                let a362 = {
                                    let a360 = {
                                        let a354 = 1u64;
                                        let a355 = 1u64;
                                        nat_add(a354, a355)?
                                    };
                                    let a361 = {
                                        let a358 = {
                                            let a356 = 1u64;
                                            let a357 = 1u64;
                                            nat_add(a356, a357)?
                                        };
                                        let a359 = 1u64;
                                        nat_mul(a358, a359)?
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
            })?;
            nat_mul(a372, a373)?
        };
        let a395 = f21({
            let a392 = {
                let a374 = 1u64;
                let a375 = 1u64;
                nat_add(a374, a375)?
            };
            let a393 = {
                let a390 = {
                    let a388 = {
                        let a376 = 1u64;
                        let a377 = 1u64;
                        nat_add(a376, a377)?
                    };
                    let a389 = {
                        let a386 = {
                            let a384 = {
                                let a378 = 1u64;
                                let a379 = 1u64;
                                nat_add(a378, a379)?
                            };
                            let a385 = {
                                let a382 = {
                                    let a380 = 1u64;
                                    let a381 = 1u64;
                                    nat_add(a380, a381)?
                                };
                                let a383 = 1u64;
                                nat_mul(a382, a383)?
                            };
                            nat_mul(a384, a385)?
                        };
                        let a387 = 1u64;
                        nat_add(a386, a387)?
                    };
                    nat_mul(a388, a389)?
                };
                let a391 = 1u64;
                nat_add(a390, a391)?
            };
            nat_mul(a392, a393)?
        })?;
        nat_add(a394, a395)
    } else {
        if if f19(v0)? {
            f19(v1)?
        } else {
            false
        } {
            let a396 = f27(v0)?;
            let a397 = f27(v1)?;
            if nat_eq(a396, a397) {
                Ok(v0)
            } else {
                let a484 = {
                    let a462 = {
                        let a438 = {
                            let a436 = {
                                let a398 = 1u64;
                                let a399 = 1u64;
                                nat_add(a398, a399)?
                            };
                            let a437 = {
                                let a434 = {
                                    let a432 = {
                                        let a400 = 1u64;
                                        let a401 = 1u64;
                                        nat_add(a400, a401)?
                                    };
                                    let a433 = {
                                        let a430 = {
                                            let a428 = {
                                                let a402 = 1u64;
                                                let a403 = 1u64;
                                                nat_add(a402, a403)?
                                            };
                                            let a429 = {
                                                let a426 = {
                                                    let a424 = {
                                                        let a404 = 1u64;
                                                        let a405 = 1u64;
                                                        nat_add(a404, a405)?
                                                    };
                                                    let a425 = {
                                                        let a422 = {
                                                            let a420 = {
                                                                let a406 = 1u64;
                                                                let a407 = 1u64;
                                                                nat_add(a406, a407)?
                                                            };
                                                            let a421 = {
                                                                let a418 = {
                                                                    let a416 = {
                                                                        let a408 = 1u64;
                                                                        let a409 = 1u64;
                                                                        nat_add(a408, a409)?
                                                                    };
                                                                    let a417 = {
                                                                        let a414 = {
                                                                            let a412 = {
                                                                                let a410 = 1u64;
                                                                                let a411 = 1u64;
                                                                                nat_add(a410, a411)?
                                                                            };
                                                                            let a413 = 1u64;
                                                                            nat_mul(a412, a413)?
                                                                        };
                                                                        let a415 = 1u64;
                                                                        nat_add(a414, a415)?
                                                                    };
                                                                    nat_mul(a416, a417)?
                                                                };
                                                                let a419 = 1u64;
                                                                nat_add(a418, a419)?
                                                            };
                                                            nat_mul(a420, a421)?
                                                        };
                                                        let a423 = 1u64;
                                                        nat_add(a422, a423)?
                                                    };
                                                    nat_mul(a424, a425)?
                                                };
                                                let a427 = 1u64;
                                                nat_add(a426, a427)?
                                            };
                                            nat_mul(a428, a429)?
                                        };
                                        let a431 = 1u64;
                                        nat_add(a430, a431)?
                                    };
                                    nat_mul(a432, a433)?
                                };
                                let a435 = 1u64;
                                nat_add(a434, a435)?
                            };
                            nat_mul(a436, a437)?
                        };
                        let a439 = 1u64;
                        nat_add(a438, a439)?
                    };
                    let a463 = f21({
                        let a460 = {
                            let a458 = {
                                let a440 = 1u64;
                                let a441 = 1u64;
                                nat_add(a440, a441)?
                            };
                            let a459 = {
                                let a456 = {
                                    let a454 = {
                                        let a442 = 1u64;
                                        let a443 = 1u64;
                                        nat_add(a442, a443)?
                                    };
                                    let a455 = {
                                        let a452 = {
                                            let a450 = {
                                                let a444 = 1u64;
                                                let a445 = 1u64;
                                                nat_add(a444, a445)?
                                            };
                                            let a451 = {
                                                let a448 = {
                                                    let a446 = 1u64;
                                                    let a447 = 1u64;
                                                    nat_add(a446, a447)?
                                                };
                                                let a449 = 1u64;
                                                nat_mul(a448, a449)?
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
                    })?;
                    nat_mul(a462, a463)?
                };
                let a485 = f21({
                    let a482 = {
                        let a464 = 1u64;
                        let a465 = 1u64;
                        nat_add(a464, a465)?
                    };
                    let a483 = {
                        let a480 = {
                            let a478 = {
                                let a466 = 1u64;
                                let a467 = 1u64;
                                nat_add(a466, a467)?
                            };
                            let a479 = {
                                let a476 = {
                                    let a474 = {
                                        let a468 = 1u64;
                                        let a469 = 1u64;
                                        nat_add(a468, a469)?
                                    };
                                    let a475 = {
                                        let a472 = {
                                            let a470 = 1u64;
                                            let a471 = 1u64;
                                            nat_add(a470, a471)?
                                        };
                                        let a473 = 1u64;
                                        nat_mul(a472, a473)?
                                    };
                                    nat_mul(a474, a475)?
                                };
                                let a477 = 1u64;
                                nat_add(a476, a477)?
                            };
                            nat_mul(a478, a479)?
                        };
                        let a481 = 1u64;
                        nat_add(a480, a481)?
                    };
                    nat_mul(a482, a483)?
                })?;
                nat_add(a484, a485)
            }
        } else {
            if f19(v0)? {
                Ok(v0)
            } else {
                if f19(v1)? {
                    Ok(v1)
                } else {
                    let a486 = f28(v1)?;
                    let a487 = f28(v0)?;
                    if nat_le(a486, a487) {
                        f29(v0, v1)
                    } else {
                        f29(v1, v0)
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
    let a491 = {
        let a488 = v1;
        let a489 = v3;
        let a490 = 0u64;
        nat_rem(a488, a489, a490)
    };
    let a492 = 0u64;
    if if nat_eq(a491, a492) {
        let a498 = {
            let a496 = {
                let a493 = v1;
                let a494 = v3;
                let a495 = 0u64;
                nat_quot(a493, a494, a495)
            };
            let a497 = v4;
            nat_mul(a496, a497)?
        };
        let a499 = v2;
        nat_eq(a498, a499)
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
    let a500 = v0;
    let a501 = 4294967296u64;
    if if if if nat_lt(a500, a501) {
        let a502 = v1;
        let a503 = 4294967296u64;
        nat_lt(a502, a503)
    } else {
        false
    } {
        let a504 = v2;
        let a505 = 4294967296u64;
        nat_lt(a504, a505)
    } else {
        false
    } {
        let a506 = v3;
        let a507 = 65536u64;
        nat_lt(a506, a507)
    } else {
        false
    } {
        let v4: u64 = f5(f6(v3)?, v2)?;
        Ok(Adt1::C2(f7(v0, f5(v4, v1)?)?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f10(v0: bool, v1: u64) -> R<Adt1> {
    if v0 {
        let a508 = v1;
        let a509 = 32u64;
        if nat_lt(a508, a509) {
            Ok(Adt1::C1(v1, 32u64, 5u64, 1u64))
        } else {
            let a510 = v1;
            let a511 = 48u64;
            if nat_lt(a510, a511) {
                Ok(Adt1::C1({
                    let a514 = 160u64;
                    let a515 = {
                        let a512 = v1;
                        let a513 = 32u64;
                        nat_sub(a512, a513)
                    };
                    nat_add(a514, a515)?
                }, 16u64, 5u64, 1u64))
            } else {
                let a516 = v1;
                let a517 = 52u64;
                if nat_lt(a516, a517) {
                    Ok(Adt1::C1({
                        let a520 = 240u64;
                        let a521 = {
                            let a518 = v1;
                            let a519 = 48u64;
                            nat_sub(a518, a519)
                        };
                        nat_add(a520, a521)?
                    }, 4u64, 4u64, 3u64))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    } else {
        let a522 = v1;
        let a523 = 64u64;
        if nat_lt(a522, a523) {
            Ok(Adt1::C1({
                let a532 = {
                    let a527 = 128u64;
                    let a528 = {
                        let a524 = v1;
                        let a525 = 32u64;
                        let a526 = 0u64;
                        nat_quot(a524, a525, a526)
                    };
                    nat_mul(a527, a528)?
                };
                let a533 = {
                    let a529 = v1;
                    let a530 = 32u64;
                    let a531 = 0u64;
                    nat_rem(a529, a530, a531)
                };
                nat_add(a532, a533)?
            }, 32u64, 4u64, 1u64))
        } else {
            Ok(Adt1::C0)
        }
    }
}

pub fn f11(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a534 = v2;
    let a535 = 256u64;
    if if nat_lt(a534, a535) {
        let a536 = v1;
        let a537 = if v0 {
            5u64
        } else {
            4u64
        };
        nat_lt(a536, a537)
    } else {
        false
    } {
        let m538 = f14(true, 0u64, 0u64, v3, v4)?;
        match m538 {
            Adt1::C0 => {
                Ok(Adt1::C0)
            }
            Adt1::C1(_, _, _, _) => {
                Ok(Adt1::C0)
            }
            Adt1::C2(v5) => {
                let a539 = v5;
                let a540 = 3u64;
                if nat_lt(a539, a540) {
                    Ok(Adt1::C2(if v0 {
                        let a543 = {
                            let a541 = v2;
                            let a542 = 3u64;
                            nat_mul(a541, a542)?
                        };
                        let a544 = v5;
                        nat_add(a543, a544)?
                    } else {
                        let a549 = v2;
                        let a550 = {
                            let a547 = v5;
                            let a548 = f2({
                                let a545 = v1;
                                let a546 = 2u64;
                                nat_mul(a545, a546)?
                            });
                            nat_mul(a547, a548)?
                        };
                        nat_add(a549, a550)?
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
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f12(v0: bool, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = f28(v2)?;
    let a551 = v2;
    let a552 = 4294967296u64;
    if if nat_lt(a551, a552) {
        let a553 = v3;
        let a554 = 2139095040u64;
        nat_lt(a553, a554)
    } else {
        false
    } {
        Ok(Adt1::C2(if v0 {
            let a555 = v1;
            let a556 = v3;
            if nat_lt(a555, a556) {
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

pub fn f13(v0: bool, v1: u64) -> R<Adt1> {
    let v2: u64 = if v0 {
        v1
    } else {
        f30(v1, 1124073472u64)?
    };
    let v3: u64 = if v0 {
        let a557 = v2;
        let a558 = 0u64;
        if nat_eq(a557, a558) {
            0u64
        } else {
            f30(1065353216u64, v2)?
        }
    } else {
        0u64
    };
    let a559 = v3;
    let a560 = 2139095040u64;
    if nat_lt(a559, a560) {
        Ok(Adt1::C3(f15(v2)?, v3))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f14(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a561 = v2;
    let a562 = 256u64;
    if if nat_lt(a561, a562) {
        let a563 = v1;
        let a564 = if v0 {
            4u64
        } else {
            8u64
        };
        nat_lt(a563, a564)
    } else {
        false
    } {
        let v5: u64 = if v0 {
            f5(v3, v4)?
        } else {
            v3
        };
        let v6: u64 = f28(v5)?;
        let v7: u64 = if v0 {
            let a565 = v6;
            let a566 = 1056964608u64;
            if nat_lt(a565, a566) {
                1u64
            } else {
                let a567 = v5;
                let a568 = 2147483648u64;
                if nat_lt(a567, a568) {
                    let a569 = v6;
                    let a570 = 1069547520u64;
                    if nat_lt(a569, a570) {
                        2u64
                    } else {
                        3u64
                    }
                } else {
                    0u64
                }
            }
        } else {
            let a571 = v3;
            let a572 = 2147483648u64;
            if if nat_lt(a571, a572) {
                true
            } else {
                let a573 = v6;
                let a574 = 0u64;
                nat_eq(a573, a574)
            } {
                1u64
            } else {
                0u64
            }
        };
        Ok(Adt1::C2({
            let a579 = v2;
            let a580 = {
                let a577 = v7;
                let a578 = f2(if v0 {
                    let a575 = v1;
                    let a576 = 2u64;
                    nat_mul(a575, a576)?
                } else {
                    v1
                });
                nat_mul(a577, a578)?
            };
            nat_add(a579, a580)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a581 = f27(v0)?;
        let a582 = 32768u64;
        nat_mul(a581, a582)?
    };
    let v2: u64 = f31(v0)?;
    let a601 = v1;
    let a602 = {
        let a583 = v2;
        let a584 = 255u64;
        if nat_eq(a583, a584) {
            let a585 = f32(v0)?;
            let a586 = 0u64;
            if nat_eq(a585, a586) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a587 = 143u64;
            let a588 = v2;
            if nat_le(a587, a588) {
                31744u64
            } else {
                let a589 = v2;
                let a590 = 102u64;
                if nat_lt(a589, a590) {
                    0u64
                } else {
                    let a591 = 113u64;
                    let a592 = v2;
                    if nat_le(a591, a592) {
                        let a597 = {
                            let a595 = {
                                let a593 = v2;
                                let a594 = 113u64;
                                nat_sub(a593, a594)
                            };
                            let a596 = 1024u64;
                            nat_mul(a595, a596)?
                        };
                        let a598 = f33(f34(v0)?, 8192u64)?;
                        nat_add(a597, a598)?
                    } else {
                        f33(f34(v0)?, f21({
                            let a599 = 126u64;
                            let a600 = v2;
                            nat_sub(a599, a600)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a601, a602)
}

pub fn f16(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a609 = {
        let a606 = {
            let a603 = v1;
            let a604 = v2;
            let a605 = 0u64;
            nat_quot(a603, a604, a605)
        };
        let a607 = 2u64;
        let a608 = 0u64;
        nat_rem(a606, a607, a608)
    };
    let a610 = 1u64;
    if nat_eq(a609, a610) {
        Ok(v0)
    } else {
        let a611 = 240u64;
        let a612 = v0;
        nat_add(a611, a612)
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a619 = {
        let a616 = {
            let a613 = v1;
            let a614 = v2;
            let a615 = 0u64;
            nat_quot(a613, a614, a615)
        };
        let a617 = 2u64;
        let a618 = 0u64;
        nat_rem(a616, a617, a618)
    };
    let a620 = 1u64;
    if nat_eq(a619, a620) {
        let a621 = 16u64;
        let a622 = v0;
        nat_add(a621, a622)
    } else {
        Ok(v0)
    }
}

pub fn f18(v0: u64) -> R<bool> {
    let a665 = f31(v0)?;
    let a666 = {
        let a663 = {
            let a661 = {
                let a623 = 1u64;
                let a624 = 1u64;
                nat_add(a623, a624)?
            };
            let a662 = {
                let a659 = {
                    let a657 = {
                        let a625 = 1u64;
                        let a626 = 1u64;
                        nat_add(a625, a626)?
                    };
                    let a658 = {
                        let a655 = {
                            let a653 = {
                                let a627 = 1u64;
                                let a628 = 1u64;
                                nat_add(a627, a628)?
                            };
                            let a654 = {
                                let a651 = {
                                    let a649 = {
                                        let a629 = 1u64;
                                        let a630 = 1u64;
                                        nat_add(a629, a630)?
                                    };
                                    let a650 = {
                                        let a647 = {
                                            let a645 = {
                                                let a631 = 1u64;
                                                let a632 = 1u64;
                                                nat_add(a631, a632)?
                                            };
                                            let a646 = {
                                                let a643 = {
                                                    let a641 = {
                                                        let a633 = 1u64;
                                                        let a634 = 1u64;
                                                        nat_add(a633, a634)?
                                                    };
                                                    let a642 = {
                                                        let a639 = {
                                                            let a637 = {
                                                                let a635 = 1u64;
                                                                let a636 = 1u64;
                                                                nat_add(a635, a636)?
                                                            };
                                                            let a638 = 1u64;
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
            nat_mul(a661, a662)?
        };
        let a664 = 1u64;
        nat_add(a663, a664)?
    };
    if nat_eq(a665, a666) {
        let a667 = 0u64;
        let a668 = f32(v0)?;
        Ok(nat_lt(a667, a668))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: u64) -> R<bool> {
    let a711 = f31(v0)?;
    let a712 = {
        let a709 = {
            let a707 = {
                let a669 = 1u64;
                let a670 = 1u64;
                nat_add(a669, a670)?
            };
            let a708 = {
                let a705 = {
                    let a703 = {
                        let a671 = 1u64;
                        let a672 = 1u64;
                        nat_add(a671, a672)?
                    };
                    let a704 = {
                        let a701 = {
                            let a699 = {
                                let a673 = 1u64;
                                let a674 = 1u64;
                                nat_add(a673, a674)?
                            };
                            let a700 = {
                                let a697 = {
                                    let a695 = {
                                        let a675 = 1u64;
                                        let a676 = 1u64;
                                        nat_add(a675, a676)?
                                    };
                                    let a696 = {
                                        let a693 = {
                                            let a691 = {
                                                let a677 = 1u64;
                                                let a678 = 1u64;
                                                nat_add(a677, a678)?
                                            };
                                            let a692 = {
                                                let a689 = {
                                                    let a687 = {
                                                        let a679 = 1u64;
                                                        let a680 = 1u64;
                                                        nat_add(a679, a680)?
                                                    };
                                                    let a688 = {
                                                        let a685 = {
                                                            let a683 = {
                                                                let a681 = 1u64;
                                                                let a682 = 1u64;
                                                                nat_add(a681, a682)?
                                                            };
                                                            let a684 = 1u64;
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
                                    nat_mul(a695, a696)?
                                };
                                let a698 = 1u64;
                                nat_add(a697, a698)?
                            };
                            nat_mul(a699, a700)?
                        };
                        let a702 = 1u64;
                        nat_add(a701, a702)?
                    };
                    nat_mul(a703, a704)?
                };
                let a706 = 1u64;
                nat_add(a705, a706)?
            };
            nat_mul(a707, a708)?
        };
        let a710 = 1u64;
        nat_add(a709, a710)?
    };
    if nat_eq(a711, a712) {
        let a713 = f32(v0)?;
        let a714 = 0u64;
        Ok(nat_eq(a713, a714))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u64) -> R<bool> {
    let a715 = f28(v0)?;
    let a716 = 0u64;
    Ok(nat_eq(a715, a716))
}

pub fn f21(v0: u64) -> R<u64> {
    f35(Fn0::F36, v0, 1u64)
}

pub fn f22(v0: u64) -> R<u64> {
    let a783 = f24(v0)?;
    let a784 = {
        let a781 = {
            let a757 = {
                let a755 = {
                    let a717 = 1u64;
                    let a718 = 1u64;
                    nat_add(a717, a718)?
                };
                let a756 = {
                    let a753 = {
                        let a751 = {
                            let a719 = 1u64;
                            let a720 = 1u64;
                            nat_add(a719, a720)?
                        };
                        let a752 = {
                            let a749 = {
                                let a747 = {
                                    let a721 = 1u64;
                                    let a722 = 1u64;
                                    nat_add(a721, a722)?
                                };
                                let a748 = {
                                    let a745 = {
                                        let a743 = {
                                            let a723 = 1u64;
                                            let a724 = 1u64;
                                            nat_add(a723, a724)?
                                        };
                                        let a744 = {
                                            let a741 = {
                                                let a739 = {
                                                    let a725 = 1u64;
                                                    let a726 = 1u64;
                                                    nat_add(a725, a726)?
                                                };
                                                let a740 = {
                                                    let a737 = {
                                                        let a735 = {
                                                            let a727 = 1u64;
                                                            let a728 = 1u64;
                                                            nat_add(a727, a728)?
                                                        };
                                                        let a736 = {
                                                            let a733 = {
                                                                let a731 = {
                                                                    let a729 = 1u64;
                                                                    let a730 = 1u64;
                                                                    nat_add(a729, a730)?
                                                                };
                                                                let a732 = 1u64;
                                                                nat_mul(a731, a732)?
                                                            };
                                                            let a734 = 1u64;
                                                            nat_add(a733, a734)?
                                                        };
                                                        nat_mul(a735, a736)?
                                                    };
                                                    let a738 = 1u64;
                                                    nat_add(a737, a738)?
                                                };
                                                nat_mul(a739, a740)?
                                            };
                                            let a742 = 1u64;
                                            nat_add(a741, a742)?
                                        };
                                        nat_mul(a743, a744)?
                                    };
                                    let a746 = 1u64;
                                    nat_add(a745, a746)?
                                };
                                nat_mul(a747, a748)?
                            };
                            let a750 = 1u64;
                            nat_add(a749, a750)?
                        };
                        nat_mul(a751, a752)?
                    };
                    let a754 = 1u64;
                    nat_add(a753, a754)?
                };
                nat_mul(a755, a756)?
            };
            let a758 = 1u64;
            nat_add(a757, a758)?
        };
        let a782 = f21({
            let a779 = {
                let a777 = {
                    let a759 = 1u64;
                    let a760 = 1u64;
                    nat_add(a759, a760)?
                };
                let a778 = {
                    let a775 = {
                        let a773 = {
                            let a761 = 1u64;
                            let a762 = 1u64;
                            nat_add(a761, a762)?
                        };
                        let a774 = {
                            let a771 = {
                                let a769 = {
                                    let a763 = 1u64;
                                    let a764 = 1u64;
                                    nat_add(a763, a764)?
                                };
                                let a770 = {
                                    let a767 = {
                                        let a765 = 1u64;
                                        let a766 = 1u64;
                                        nat_add(a765, a766)?
                                    };
                                    let a768 = 1u64;
                                    nat_mul(a767, a768)?
                                };
                                nat_mul(a769, a770)?
                            };
                            let a772 = 1u64;
                            nat_add(a771, a772)?
                        };
                        nat_mul(a773, a774)?
                    };
                    let a776 = 1u64;
                    nat_add(a775, a776)?
                };
                nat_mul(a777, a778)?
            };
            let a780 = 1u64;
            nat_add(a779, a780)?
        })?;
        nat_mul(a781, a782)?
    };
    nat_add(a783, a784)
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let a791 = {
        let a785 = f27(v0)?;
        let a786 = f27(v1)?;
        nat_add(a785, a786)?
    };
    let a792 = {
        let a789 = {
            let a787 = 1u64;
            let a788 = 1u64;
            nat_add(a787, a788)?
        };
        let a790 = 1u64;
        nat_mul(a789, a790)?
    };
    let a793 = 0u64;
    Ok(nat_rem(a791, a792, a793))
}

pub fn f24(v0: u64) -> R<u64> {
    let a818 = v0;
    let a819 = f21({
        let a816 = {
            let a814 = {
                let a794 = 1u64;
                let a795 = 1u64;
                nat_add(a794, a795)?
            };
            let a815 = {
                let a812 = {
                    let a810 = {
                        let a796 = 1u64;
                        let a797 = 1u64;
                        nat_add(a796, a797)?
                    };
                    let a811 = {
                        let a808 = {
                            let a806 = {
                                let a798 = 1u64;
                                let a799 = 1u64;
                                nat_add(a798, a799)?
                            };
                            let a807 = {
                                let a804 = {
                                    let a802 = {
                                        let a800 = 1u64;
                                        let a801 = 1u64;
                                        nat_add(a800, a801)?
                                    };
                                    let a803 = 1u64;
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
                    nat_mul(a810, a811)?
                };
                let a813 = 1u64;
                nat_add(a812, a813)?
            };
            nat_mul(a814, a815)?
        };
        let a817 = 1u64;
        nat_add(a816, a817)?
    })?;
    nat_mul(a818, a819)
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f37(v0)?;
    let v3: Adt2 = f37(v1)?;
    f38(f23(v0, v1)?, {
        let a874 = {
            let a822 = match v2.clone() {
                Adt2::C0(h820, _, _, _) => {
                    h820
                }
            };
            let a823 = match v3.clone() {
                Adt2::C0(h821, _, _, _) => {
                    h821
                }
            };
            nat_add(a822, a823)?
        };
        let a875 = {
            let a872 = {
                let a870 = {
                    let a824 = 1u64;
                    let a825 = 1u64;
                    nat_add(a824, a825)?
                };
                let a871 = {
                    let a868 = {
                        let a866 = {
                            let a826 = 1u64;
                            let a827 = 1u64;
                            nat_add(a826, a827)?
                        };
                        let a867 = {
                            let a864 = {
                                let a862 = {
                                    let a828 = 1u64;
                                    let a829 = 1u64;
                                    nat_add(a828, a829)?
                                };
                                let a863 = {
                                    let a860 = {
                                        let a858 = {
                                            let a830 = 1u64;
                                            let a831 = 1u64;
                                            nat_add(a830, a831)?
                                        };
                                        let a859 = {
                                            let a856 = {
                                                let a854 = {
                                                    let a832 = 1u64;
                                                    let a833 = 1u64;
                                                    nat_add(a832, a833)?
                                                };
                                                let a855 = {
                                                    let a852 = {
                                                        let a850 = {
                                                            let a834 = 1u64;
                                                            let a835 = 1u64;
                                                            nat_add(a834, a835)?
                                                        };
                                                        let a851 = {
                                                            let a848 = {
                                                                let a846 = {
                                                                    let a836 = 1u64;
                                                                    let a837 = 1u64;
                                                                    nat_add(a836, a837)?
                                                                };
                                                                let a847 = {
                                                                    let a844 = {
                                                                        let a838 = 1u64;
                                                                        let a839 = 1u64;
                                                                        nat_add(a838, a839)?
                                                                    };
                                                                    let a845 = {
                                                                        let a842 = {
                                                                            let a840 = 1u64;
                                                                            let a841 = 1u64;
                                                                            nat_add(a840, a841)?
                                                                        };
                                                                        let a843 = 1u64;
                                                                        nat_mul(a842, a843)?
                                                                    };
                                                                    nat_mul(a844, a845)?
                                                                };
                                                                nat_mul(a846, a847)?
                                                            };
                                                            let a849 = 1u64;
                                                            nat_add(a848, a849)?
                                                        };
                                                        nat_mul(a850, a851)?
                                                    };
                                                    let a853 = 1u64;
                                                    nat_add(a852, a853)?
                                                };
                                                nat_mul(a854, a855)?
                                            };
                                            let a857 = 1u64;
                                            nat_add(a856, a857)?
                                        };
                                        nat_mul(a858, a859)?
                                    };
                                    let a861 = 1u64;
                                    nat_add(a860, a861)?
                                };
                                nat_mul(a862, a863)?
                            };
                            let a865 = 1u64;
                            nat_add(a864, a865)?
                        };
                        nat_mul(a866, a867)?
                    };
                    let a869 = 1u64;
                    nat_add(a868, a869)?
                };
                nat_mul(a870, a871)?
            };
            let a873 = 1u64;
            nat_add(a872, a873)?
        };
        nat_sub(a874, a875)
    }, f39({
        let a878 = match v2 {
            Adt2::C0(_, h876, _, _) => {
                h876
            }
        };
        let a879 = match v3 {
            Adt2::C0(_, h877, _, _) => {
                h877
            }
        };
        nat_mul(a878, a879)?
    }, {
        let a896 = {
            let a880 = 1u64;
            let a881 = 1u64;
            nat_add(a880, a881)?
        };
        let a897 = {
            let a894 = {
                let a882 = 1u64;
                let a883 = 1u64;
                nat_add(a882, a883)?
            };
            let a895 = {
                let a892 = {
                    let a890 = {
                        let a884 = 1u64;
                        let a885 = 1u64;
                        nat_add(a884, a885)?
                    };
                    let a891 = {
                        let a888 = {
                            let a886 = 1u64;
                            let a887 = 1u64;
                            nat_add(a886, a887)?
                        };
                        let a889 = 1u64;
                        nat_mul(a888, a889)?
                    };
                    nat_mul(a890, a891)?
                };
                let a893 = 1u64;
                nat_add(a892, a893)?
            };
            nat_mul(a894, a895)?
        };
        nat_mul(a896, a897)?
    })?)
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h898, _) = f40(Fn1::F41, {
        let a917 = {
            let a899 = 1u64;
            let a900 = 1u64;
            nat_add(a899, a900)?
        };
        let a918 = {
            let a915 = {
                let a901 = 1u64;
                let a902 = 1u64;
                nat_add(a901, a902)?
            };
            let a916 = {
                let a913 = {
                    let a903 = 1u64;
                    let a904 = 1u64;
                    nat_add(a903, a904)?
                };
                let a914 = {
                    let a911 = {
                        let a905 = 1u64;
                        let a906 = 1u64;
                        nat_add(a905, a906)?
                    };
                    let a912 = {
                        let a909 = {
                            let a907 = 1u64;
                            let a908 = 1u64;
                            nat_add(a907, a908)?
                        };
                        let a910 = 1u64;
                        nat_mul(a909, a910)?
                    };
                    nat_mul(a911, a912)?
                };
                nat_mul(a913, a914)?
            };
            nat_mul(a915, a916)?
        };
        nat_mul(a917, a918)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h898)
}

pub fn f27(v0: u64) -> R<u64> {
    let a943 = v0;
    let a944 = f21({
        let a941 = {
            let a939 = {
                let a919 = 1u64;
                let a920 = 1u64;
                nat_add(a919, a920)?
            };
            let a940 = {
                let a937 = {
                    let a935 = {
                        let a921 = 1u64;
                        let a922 = 1u64;
                        nat_add(a921, a922)?
                    };
                    let a936 = {
                        let a933 = {
                            let a931 = {
                                let a923 = 1u64;
                                let a924 = 1u64;
                                nat_add(a923, a924)?
                            };
                            let a932 = {
                                let a929 = {
                                    let a927 = {
                                        let a925 = 1u64;
                                        let a926 = 1u64;
                                        nat_add(a925, a926)?
                                    };
                                    let a928 = 1u64;
                                    nat_mul(a927, a928)?
                                };
                                let a930 = 1u64;
                                nat_add(a929, a930)?
                            };
                            nat_mul(a931, a932)?
                        };
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    };
                    nat_mul(a935, a936)?
                };
                let a938 = 1u64;
                nat_add(a937, a938)?
            };
            nat_mul(a939, a940)?
        };
        let a942 = 1u64;
        nat_add(a941, a942)?
    })?;
    let a945 = 0u64;
    Ok(nat_quot(a943, a944, a945))
}

pub fn f28(v0: u64) -> R<u64> {
    let a970 = v0;
    let a971 = f21({
        let a968 = {
            let a966 = {
                let a946 = 1u64;
                let a947 = 1u64;
                nat_add(a946, a947)?
            };
            let a967 = {
                let a964 = {
                    let a962 = {
                        let a948 = 1u64;
                        let a949 = 1u64;
                        nat_add(a948, a949)?
                    };
                    let a963 = {
                        let a960 = {
                            let a958 = {
                                let a950 = 1u64;
                                let a951 = 1u64;
                                nat_add(a950, a951)?
                            };
                            let a959 = {
                                let a956 = {
                                    let a954 = {
                                        let a952 = 1u64;
                                        let a953 = 1u64;
                                        nat_add(a952, a953)?
                                    };
                                    let a955 = 1u64;
                                    nat_mul(a954, a955)?
                                };
                                let a957 = 1u64;
                                nat_add(a956, a957)?
                            };
                            nat_mul(a958, a959)?
                        };
                        let a961 = 1u64;
                        nat_add(a960, a961)?
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
    })?;
    let a972 = 0u64;
    Ok(nat_rem(a970, a971, a972))
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f42(v0)?;
    let v3: u64 = {
        let a985 = f34(v0)?;
        let a986 = {
            let a983 = {
                let a973 = 1u64;
                let a974 = 1u64;
                nat_add(a973, a974)?
            };
            let a984 = {
                let a981 = {
                    let a975 = 1u64;
                    let a976 = 1u64;
                    nat_add(a975, a976)?
                };
                let a982 = {
                    let a979 = {
                        let a977 = 1u64;
                        let a978 = 1u64;
                        nat_add(a977, a978)?
                    };
                    let a980 = 1u64;
                    nat_mul(a979, a980)?
                };
                nat_mul(a981, a982)?
            };
            nat_mul(a983, a984)?
        };
        nat_mul(a985, a986)?
    };
    let v4: u64 = f39({
        let a999 = f34(v1)?;
        let a1000 = {
            let a997 = {
                let a987 = 1u64;
                let a988 = 1u64;
                nat_add(a987, a988)?
            };
            let a998 = {
                let a995 = {
                    let a989 = 1u64;
                    let a990 = 1u64;
                    nat_add(a989, a990)?
                };
                let a996 = {
                    let a993 = {
                        let a991 = 1u64;
                        let a992 = 1u64;
                        nat_add(a991, a992)?
                    };
                    let a994 = 1u64;
                    nat_mul(a993, a994)?
                };
                nat_mul(a995, a996)?
            };
            nat_mul(a997, a998)?
        };
        nat_mul(a999, a1000)?
    }, {
        let a1001 = v2;
        let a1002 = f42(v1)?;
        nat_sub(a1001, a1002)
    })?;
    let a1003 = f27(v0)?;
    let a1004 = f27(v1)?;
    if nat_eq(a1003, a1004) {
        f38(f27(v0)?, v2, {
            let a1005 = v3;
            let a1006 = v4;
            nat_add(a1005, a1006)?
        })
    } else {
        let a1007 = v3;
        let a1008 = v4;
        if nat_eq(a1007, a1008) {
            Ok(0u64)
        } else {
            f38(f27(v0)?, v2, {
                let a1009 = v3;
                let a1010 = v4;
                nat_sub(a1009, a1010)
            })
        }
    }
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    if if if if f18(v0)? {
        true
    } else {
        f18(v1)?
    } {
        true
    } else {
        if f19(v0)? {
            f19(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f20(v0)? {
            f20(v1)?
        } else {
            false
        }
    } {
        let a1097 = {
            let a1075 = {
                let a1051 = {
                    let a1049 = {
                        let a1011 = 1u64;
                        let a1012 = 1u64;
                        nat_add(a1011, a1012)?
                    };
                    let a1050 = {
                        let a1047 = {
                            let a1045 = {
                                let a1013 = 1u64;
                                let a1014 = 1u64;
                                nat_add(a1013, a1014)?
                            };
                            let a1046 = {
                                let a1043 = {
                                    let a1041 = {
                                        let a1015 = 1u64;
                                        let a1016 = 1u64;
                                        nat_add(a1015, a1016)?
                                    };
                                    let a1042 = {
                                        let a1039 = {
                                            let a1037 = {
                                                let a1017 = 1u64;
                                                let a1018 = 1u64;
                                                nat_add(a1017, a1018)?
                                            };
                                            let a1038 = {
                                                let a1035 = {
                                                    let a1033 = {
                                                        let a1019 = 1u64;
                                                        let a1020 = 1u64;
                                                        nat_add(a1019, a1020)?
                                                    };
                                                    let a1034 = {
                                                        let a1031 = {
                                                            let a1029 = {
                                                                let a1021 = 1u64;
                                                                let a1022 = 1u64;
                                                                nat_add(a1021, a1022)?
                                                            };
                                                            let a1030 = {
                                                                let a1027 = {
                                                                    let a1025 = {
                                                                        let a1023 = 1u64;
                                                                        let a1024 = 1u64;
                                                                        nat_add(a1023, a1024)?
                                                                    };
                                                                    let a1026 = 1u64;
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
                                            };
                                            nat_mul(a1037, a1038)?
                                        };
                                        let a1040 = 1u64;
                                        nat_add(a1039, a1040)?
                                    };
                                    nat_mul(a1041, a1042)?
                                };
                                let a1044 = 1u64;
                                nat_add(a1043, a1044)?
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
            let a1076 = f21({
                let a1073 = {
                    let a1071 = {
                        let a1053 = 1u64;
                        let a1054 = 1u64;
                        nat_add(a1053, a1054)?
                    };
                    let a1072 = {
                        let a1069 = {
                            let a1067 = {
                                let a1055 = 1u64;
                                let a1056 = 1u64;
                                nat_add(a1055, a1056)?
                            };
                            let a1068 = {
                                let a1065 = {
                                    let a1063 = {
                                        let a1057 = 1u64;
                                        let a1058 = 1u64;
                                        nat_add(a1057, a1058)?
                                    };
                                    let a1064 = {
                                        let a1061 = {
                                            let a1059 = 1u64;
                                            let a1060 = 1u64;
                                            nat_add(a1059, a1060)?
                                        };
                                        let a1062 = 1u64;
                                        nat_mul(a1061, a1062)?
                                    };
                                    nat_mul(a1063, a1064)?
                                };
                                let a1066 = 1u64;
                                nat_add(a1065, a1066)?
                            };
                            nat_mul(a1067, a1068)?
                        };
                        let a1070 = 1u64;
                        nat_add(a1069, a1070)?
                    };
                    nat_mul(a1071, a1072)?
                };
                let a1074 = 1u64;
                nat_add(a1073, a1074)?
            })?;
            nat_mul(a1075, a1076)?
        };
        let a1098 = f21({
            let a1095 = {
                let a1077 = 1u64;
                let a1078 = 1u64;
                nat_add(a1077, a1078)?
            };
            let a1096 = {
                let a1093 = {
                    let a1091 = {
                        let a1079 = 1u64;
                        let a1080 = 1u64;
                        nat_add(a1079, a1080)?
                    };
                    let a1092 = {
                        let a1089 = {
                            let a1087 = {
                                let a1081 = 1u64;
                                let a1082 = 1u64;
                                nat_add(a1081, a1082)?
                            };
                            let a1088 = {
                                let a1085 = {
                                    let a1083 = 1u64;
                                    let a1084 = 1u64;
                                    nat_add(a1083, a1084)?
                                };
                                let a1086 = 1u64;
                                nat_mul(a1085, a1086)?
                            };
                            nat_mul(a1087, a1088)?
                        };
                        let a1090 = 1u64;
                        nat_add(a1089, a1090)?
                    };
                    nat_mul(a1091, a1092)?
                };
                let a1094 = 1u64;
                nat_add(a1093, a1094)?
            };
            nat_mul(a1095, a1096)?
        })?;
        nat_add(a1097, a1098)
    } else {
        if if f19(v0)? {
            true
        } else {
            f20(v1)?
        } {
            f22(f23(v0, v1)?)
        } else {
            if if f19(v1)? {
                true
            } else {
                f20(v0)?
            } {
                f24(f23(v0, v1)?)
            } else {
                f43(v0, v1)
            }
        }
    }
}

pub fn f31(v0: u64) -> R<u64> {
    let a1121 = f28(v0)?;
    let a1122 = f21({
        let a1119 = {
            let a1117 = {
                let a1099 = 1u64;
                let a1100 = 1u64;
                nat_add(a1099, a1100)?
            };
            let a1118 = {
                let a1115 = {
                    let a1113 = {
                        let a1101 = 1u64;
                        let a1102 = 1u64;
                        nat_add(a1101, a1102)?
                    };
                    let a1114 = {
                        let a1111 = {
                            let a1109 = {
                                let a1103 = 1u64;
                                let a1104 = 1u64;
                                nat_add(a1103, a1104)?
                            };
                            let a1110 = {
                                let a1107 = {
                                    let a1105 = 1u64;
                                    let a1106 = 1u64;
                                    nat_add(a1105, a1106)?
                                };
                                let a1108 = 1u64;
                                nat_mul(a1107, a1108)?
                            };
                            nat_mul(a1109, a1110)?
                        };
                        let a1112 = 1u64;
                        nat_add(a1111, a1112)?
                    };
                    nat_mul(a1113, a1114)?
                };
                let a1116 = 1u64;
                nat_add(a1115, a1116)?
            };
            nat_mul(a1117, a1118)?
        };
        let a1120 = 1u64;
        nat_add(a1119, a1120)?
    })?;
    let a1123 = 0u64;
    Ok(nat_quot(a1121, a1122, a1123))
}

pub fn f32(v0: u64) -> R<u64> {
    let a1146 = v0;
    let a1147 = f21({
        let a1144 = {
            let a1142 = {
                let a1124 = 1u64;
                let a1125 = 1u64;
                nat_add(a1124, a1125)?
            };
            let a1143 = {
                let a1140 = {
                    let a1138 = {
                        let a1126 = 1u64;
                        let a1127 = 1u64;
                        nat_add(a1126, a1127)?
                    };
                    let a1139 = {
                        let a1136 = {
                            let a1134 = {
                                let a1128 = 1u64;
                                let a1129 = 1u64;
                                nat_add(a1128, a1129)?
                            };
                            let a1135 = {
                                let a1132 = {
                                    let a1130 = 1u64;
                                    let a1131 = 1u64;
                                    nat_add(a1130, a1131)?
                                };
                                let a1133 = 1u64;
                                nat_mul(a1132, a1133)?
                            };
                            nat_mul(a1134, a1135)?
                        };
                        let a1137 = 1u64;
                        nat_add(a1136, a1137)?
                    };
                    nat_mul(a1138, a1139)?
                };
                let a1141 = 1u64;
                nat_add(a1140, a1141)?
            };
            nat_mul(a1142, a1143)?
        };
        let a1145 = 1u64;
        nat_add(a1144, a1145)?
    })?;
    let a1148 = 0u64;
    Ok(nat_rem(a1146, a1147, a1148))
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1149 = v0;
        let a1150 = v1;
        let a1151 = 0u64;
        nat_quot(a1149, a1150, a1151)
    };
    let v3: u64 = {
        let a1155 = {
            let a1152 = v0;
            let a1153 = v1;
            let a1154 = 0u64;
            nat_rem(a1152, a1153, a1154)
        };
        let a1156 = 2u64;
        nat_mul(a1155, a1156)?
    };
    let a1166 = v2;
    let a1167 = {
        let a1157 = v1;
        let a1158 = v3;
        if if nat_lt(a1157, a1158) {
            true
        } else {
            let a1159 = v3;
            let a1160 = v1;
            if nat_eq(a1159, a1160) {
                let a1164 = {
                    let a1161 = v2;
                    let a1162 = 2u64;
                    let a1163 = 0u64;
                    nat_rem(a1161, a1162, a1163)
                };
                let a1165 = 1u64;
                nat_eq(a1164, a1165)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1166, a1167)
}

pub fn f34(v0: u64) -> R<u64> {
    let a1192 = f32(v0)?;
    let a1193 = {
        let a1168 = f31(v0)?;
        let a1169 = 0u64;
        if nat_eq(a1168, a1169) {
            0u64
        } else {
            f21({
                let a1190 = {
                    let a1188 = {
                        let a1170 = 1u64;
                        let a1171 = 1u64;
                        nat_add(a1170, a1171)?
                    };
                    let a1189 = {
                        let a1186 = {
                            let a1184 = {
                                let a1172 = 1u64;
                                let a1173 = 1u64;
                                nat_add(a1172, a1173)?
                            };
                            let a1185 = {
                                let a1182 = {
                                    let a1180 = {
                                        let a1174 = 1u64;
                                        let a1175 = 1u64;
                                        nat_add(a1174, a1175)?
                                    };
                                    let a1181 = {
                                        let a1178 = {
                                            let a1176 = 1u64;
                                            let a1177 = 1u64;
                                            nat_add(a1176, a1177)?
                                        };
                                        let a1179 = 1u64;
                                        nat_mul(a1178, a1179)?
                                    };
                                    nat_mul(a1180, a1181)?
                                };
                                let a1183 = 1u64;
                                nat_add(a1182, a1183)?
                            };
                            nat_mul(a1184, a1185)?
                        };
                        let a1187 = 1u64;
                        nat_add(a1186, a1187)?
                    };
                    nat_mul(a1188, a1189)?
                };
                let a1191 = 1u64;
                nat_add(a1190, a1191)?
            })?
        }
    };
    nat_add(a1192, a1193)
}

pub fn f35(v0: Fn0, p1197: u64, p1198: u64) -> R<u64> {
    let m1196 = (p1197, p1198);
    {
        let mut m1196 = m1196;
        loop {
            let step = {
                let (v1, v2) = m1196;
                let m1194 = v1;
                if m1194 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1194 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1195: Fn0 = v0.clone();
                        c1195.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(v0: u64) -> R<u64> {
    let a1199 = v0;
    let a1200 = v0;
    nat_add(a1199, a1200)
}

pub fn f37(v0: u64) -> R<Adt2> {
    f26(f42(v0)?, f34(v0)?, 0u64, f21({
        let a1221 = {
            let a1219 = {
                let a1201 = 1u64;
                let a1202 = 1u64;
                nat_add(a1201, a1202)?
            };
            let a1220 = {
                let a1217 = {
                    let a1215 = {
                        let a1203 = 1u64;
                        let a1204 = 1u64;
                        nat_add(a1203, a1204)?
                    };
                    let a1216 = {
                        let a1213 = {
                            let a1211 = {
                                let a1205 = 1u64;
                                let a1206 = 1u64;
                                nat_add(a1205, a1206)?
                            };
                            let a1212 = {
                                let a1209 = {
                                    let a1207 = 1u64;
                                    let a1208 = 1u64;
                                    nat_add(a1207, a1208)?
                                };
                                let a1210 = 1u64;
                                nat_mul(a1209, a1210)?
                            };
                            nat_mul(a1211, a1212)?
                        };
                        let a1214 = 1u64;
                        nat_add(a1213, a1214)?
                    };
                    nat_mul(a1215, a1216)?
                };
                let a1218 = 1u64;
                nat_add(a1217, a1218)?
            };
            nat_mul(a1219, a1220)?
        };
        let a1222 = 1u64;
        nat_add(a1221, a1222)?
    })?)
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1245 = f21({
        let a1243 = {
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
        };
        let a1244 = 1u64;
        nat_add(a1243, a1244)?
    })?;
    let a1246 = v2;
    if nat_le(a1245, a1246) {
        f44(v0, {
            let a1247 = v1;
            let a1248 = 1u64;
            nat_add(a1247, a1248)?
        }, f39(v2, 1u64)?)
    } else {
        let v3: Adt2 = f26(v1, v2, {
            let a1285 = {
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
            let a1286 = 1u64;
            nat_add(a1285, a1286)?
        }, f21({
            let a1305 = {
                let a1287 = 1u64;
                let a1288 = 1u64;
                nat_add(a1287, a1288)?
            };
            let a1306 = {
                let a1303 = {
                    let a1301 = {
                        let a1289 = 1u64;
                        let a1290 = 1u64;
                        nat_add(a1289, a1290)?
                    };
                    let a1302 = {
                        let a1299 = {
                            let a1291 = 1u64;
                            let a1292 = 1u64;
                            nat_add(a1291, a1292)?
                        };
                        let a1300 = {
                            let a1297 = {
                                let a1295 = {
                                    let a1293 = 1u64;
                                    let a1294 = 1u64;
                                    nat_add(a1293, a1294)?
                                };
                                let a1296 = 1u64;
                                nat_mul(a1295, a1296)?
                            };
                            let a1298 = 1u64;
                            nat_add(a1297, a1298)?
                        };
                        nat_mul(a1299, a1300)?
                    };
                    nat_mul(a1301, a1302)?
                };
                let a1304 = 1u64;
                nat_add(a1303, a1304)?
            };
            nat_mul(a1305, a1306)?
        })?)?;
        f44(v0, match v3.clone() {
            Adt2::C0(h1307, _, _, _) => {
                h1307
            }
        }, match v3 {
            Adt2::C0(_, h1308, _, _) => {
                h1308
            }
        })
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1309 = v1;
    let a1310 = 0u64;
    if nat_eq(a1309, a1310) {
        Ok(v0)
    } else {
        let a1335 = {
            let a1333 = {
                let a1311 = 1u64;
                let a1312 = 1u64;
                nat_add(a1311, a1312)?
            };
            let a1334 = {
                let a1331 = {
                    let a1313 = 1u64;
                    let a1314 = 1u64;
                    nat_add(a1313, a1314)?
                };
                let a1332 = {
                    let a1329 = {
                        let a1315 = 1u64;
                        let a1316 = 1u64;
                        nat_add(a1315, a1316)?
                    };
                    let a1330 = {
                        let a1327 = {
                            let a1317 = 1u64;
                            let a1318 = 1u64;
                            nat_add(a1317, a1318)?
                        };
                        let a1328 = {
                            let a1325 = {
                                let a1319 = 1u64;
                                let a1320 = 1u64;
                                nat_add(a1319, a1320)?
                            };
                            let a1326 = {
                                let a1323 = {
                                    let a1321 = 1u64;
                                    let a1322 = 1u64;
                                    nat_add(a1321, a1322)?
                                };
                                let a1324 = 1u64;
                                nat_mul(a1323, a1324)?
                            };
                            nat_mul(a1325, a1326)?
                        };
                        nat_mul(a1327, a1328)?
                    };
                    nat_mul(a1329, a1330)?
                };
                nat_mul(a1331, a1332)?
            };
            nat_mul(a1333, a1334)?
        };
        let a1336 = v1;
        if nat_le(a1335, a1336) {
            let a1337 = v0;
            let a1338 = 0u64;
            if nat_eq(a1337, a1338) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f21(v1)?;
            f45({
                let a1339 = v0;
                let a1340 = v2;
                let a1341 = 0u64;
                nat_quot(a1339, a1340, a1341)
            }, {
                let a1345 = 0u64;
                let a1346 = {
                    let a1342 = v0;
                    let a1343 = v2;
                    let a1344 = 0u64;
                    nat_rem(a1342, a1343, a1344)
                };
                nat_lt(a1345, a1346)
            })
        }
    }
}

pub fn f40(v0: Fn1, p1351: u64, p1352: Adt2) -> R<(Adt2, bool)> {
    let m1350 = (p1351, p1352);
    {
        let mut m1350 = m1350;
        loop {
            let step = {
                let (v1, v2) = m1350;
                let m1347 = v1;
                if m1347 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1347 - 1;
                    let m1349 = {
                        let c1348: Fn1 = v0.clone();
                        c1348.apply(v2.clone())?
                    };
                    match m1349 {
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
                Ok(next) => { m1350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt2) -> R<Option<Adt2>> {
    f46(v0)
}

pub fn f42(v0: u64) -> R<u64> {
    let a1391 = {
        let a1353 = f31(v0)?;
        let a1354 = 0u64;
        if nat_eq(a1353, a1354) {
            1u64
        } else {
            f31(v0)?
        }
    };
    let a1392 = {
        let a1389 = {
            let a1355 = 1u64;
            let a1356 = 1u64;
            nat_add(a1355, a1356)?
        };
        let a1390 = {
            let a1387 = {
                let a1357 = 1u64;
                let a1358 = 1u64;
                nat_add(a1357, a1358)?
            };
            let a1388 = {
                let a1385 = {
                    let a1359 = 1u64;
                    let a1360 = 1u64;
                    nat_add(a1359, a1360)?
                };
                let a1386 = {
                    let a1383 = {
                        let a1361 = 1u64;
                        let a1362 = 1u64;
                        nat_add(a1361, a1362)?
                    };
                    let a1384 = {
                        let a1381 = {
                            let a1363 = 1u64;
                            let a1364 = 1u64;
                            nat_add(a1363, a1364)?
                        };
                        let a1382 = {
                            let a1379 = {
                                let a1365 = 1u64;
                                let a1366 = 1u64;
                                nat_add(a1365, a1366)?
                            };
                            let a1380 = {
                                let a1377 = {
                                    let a1367 = 1u64;
                                    let a1368 = 1u64;
                                    nat_add(a1367, a1368)?
                                };
                                let a1378 = {
                                    let a1375 = {
                                        let a1369 = 1u64;
                                        let a1370 = 1u64;
                                        nat_add(a1369, a1370)?
                                    };
                                    let a1376 = {
                                        let a1373 = {
                                            let a1371 = 1u64;
                                            let a1372 = 1u64;
                                            nat_add(a1371, a1372)?
                                        };
                                        let a1374 = 1u64;
                                        nat_mul(a1373, a1374)?
                                    };
                                    nat_mul(a1375, a1376)?
                                };
                                nat_mul(a1377, a1378)?
                            };
                            nat_mul(a1379, a1380)?
                        };
                        nat_mul(a1381, a1382)?
                    };
                    nat_mul(a1383, a1384)?
                };
                nat_mul(a1385, a1386)?
            };
            nat_mul(a1387, a1388)?
        };
        nat_mul(a1389, a1390)?
    };
    nat_add(a1391, a1392)
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f37(v0)?;
    let v3: Adt2 = f37(v1)?;
    let v4: bool = {
        let a1395 = match v2.clone() {
            Adt2::C0(_, h1393, _, _) => {
                h1393
            }
        };
        let a1396 = match v3.clone() {
            Adt2::C0(_, h1394, _, _) => {
                h1394
            }
        };
        nat_lt(a1395, a1396)
    };
    let v5: u64 = {
        let a1440 = match v2.clone() {
            Adt2::C0(_, h1397, _, _) => {
                h1397
            }
        };
        let a1441 = f21(if v4 {
            let a1418 = {
                let a1416 = {
                    let a1398 = 1u64;
                    let a1399 = 1u64;
                    nat_add(a1398, a1399)?
                };
                let a1417 = {
                    let a1414 = {
                        let a1412 = {
                            let a1400 = 1u64;
                            let a1401 = 1u64;
                            nat_add(a1400, a1401)?
                        };
                        let a1413 = {
                            let a1410 = {
                                let a1402 = 1u64;
                                let a1403 = 1u64;
                                nat_add(a1402, a1403)?
                            };
                            let a1411 = {
                                let a1408 = {
                                    let a1406 = {
                                        let a1404 = 1u64;
                                        let a1405 = 1u64;
                                        nat_add(a1404, a1405)?
                                    };
                                    let a1407 = 1u64;
                                    nat_mul(a1406, a1407)?
                                };
                                let a1409 = 1u64;
                                nat_add(a1408, a1409)?
                            };
                            nat_mul(a1410, a1411)?
                        };
                        nat_mul(a1412, a1413)?
                    };
                    let a1415 = 1u64;
                    nat_add(a1414, a1415)?
                };
                nat_mul(a1416, a1417)?
            };
            let a1419 = 1u64;
            nat_add(a1418, a1419)?
        } else {
            let a1438 = {
                let a1420 = 1u64;
                let a1421 = 1u64;
                nat_add(a1420, a1421)?
            };
            let a1439 = {
                let a1436 = {
                    let a1434 = {
                        let a1422 = 1u64;
                        let a1423 = 1u64;
                        nat_add(a1422, a1423)?
                    };
                    let a1435 = {
                        let a1432 = {
                            let a1424 = 1u64;
                            let a1425 = 1u64;
                            nat_add(a1424, a1425)?
                        };
                        let a1433 = {
                            let a1430 = {
                                let a1428 = {
                                    let a1426 = 1u64;
                                    let a1427 = 1u64;
                                    nat_add(a1426, a1427)?
                                };
                                let a1429 = 1u64;
                                nat_mul(a1428, a1429)?
                            };
                            let a1431 = 1u64;
                            nat_add(a1430, a1431)?
                        };
                        nat_mul(a1432, a1433)?
                    };
                    nat_mul(a1434, a1435)?
                };
                let a1437 = 1u64;
                nat_add(a1436, a1437)?
            };
            nat_mul(a1438, a1439)?
        })?;
        nat_mul(a1440, a1441)?
    };
    let v6: u64 = {
        let a1498 = {
            let a1496 = {
                let a1493 = match v2 {
                    Adt2::C0(h1442, _, _, _) => {
                        h1442
                    }
                };
                let a1494 = {
                    let a1491 = {
                        let a1489 = {
                            let a1443 = 1u64;
                            let a1444 = 1u64;
                            nat_add(a1443, a1444)?
                        };
                        let a1490 = {
                            let a1487 = {
                                let a1485 = {
                                    let a1445 = 1u64;
                                    let a1446 = 1u64;
                                    nat_add(a1445, a1446)?
                                };
                                let a1486 = {
                                    let a1483 = {
                                        let a1481 = {
                                            let a1447 = 1u64;
                                            let a1448 = 1u64;
                                            nat_add(a1447, a1448)?
                                        };
                                        let a1482 = {
                                            let a1479 = {
                                                let a1477 = {
                                                    let a1449 = 1u64;
                                                    let a1450 = 1u64;
                                                    nat_add(a1449, a1450)?
                                                };
                                                let a1478 = {
                                                    let a1475 = {
                                                        let a1473 = {
                                                            let a1451 = 1u64;
                                                            let a1452 = 1u64;
                                                            nat_add(a1451, a1452)?
                                                        };
                                                        let a1474 = {
                                                            let a1471 = {
                                                                let a1469 = {
                                                                    let a1453 = 1u64;
                                                                    let a1454 = 1u64;
                                                                    nat_add(a1453, a1454)?
                                                                };
                                                                let a1470 = {
                                                                    let a1467 = {
                                                                        let a1465 = {
                                                                            let a1455 = 1u64;
                                                                            let a1456 = 1u64;
                                                                            nat_add(a1455, a1456)?
                                                                        };
                                                                        let a1466 = {
                                                                            let a1463 = {
                                                                                let a1457 = 1u64;
                                                                                let a1458 = 1u64;
                                                                                nat_add(a1457, a1458)?
                                                                            };
                                                                            let a1464 = {
                                                                                let a1461 = {
                                                                                    let a1459 = 1u64;
                                                                                    let a1460 = 1u64;
                                                                                    nat_add(a1459, a1460)?
                                                                                };
                                                                                let a1462 = 1u64;
                                                                                nat_mul(a1461, a1462)?
                                                                            };
                                                                            nat_mul(a1463, a1464)?
                                                                        };
                                                                        nat_mul(a1465, a1466)?
                                                                    };
                                                                    let a1468 = 1u64;
                                                                    nat_add(a1467, a1468)?
                                                                };
                                                                nat_mul(a1469, a1470)?
                                                            };
                                                            let a1472 = 1u64;
                                                            nat_add(a1471, a1472)?
                                                        };
                                                        nat_mul(a1473, a1474)?
                                                    };
                                                    let a1476 = 1u64;
                                                    nat_add(a1475, a1476)?
                                                };
                                                nat_mul(a1477, a1478)?
                                            };
                                            let a1480 = 1u64;
                                            nat_add(a1479, a1480)?
                                        };
                                        nat_mul(a1481, a1482)?
                                    };
                                    let a1484 = 1u64;
                                    nat_add(a1483, a1484)?
                                };
                                nat_mul(a1485, a1486)?
                            };
                            let a1488 = 1u64;
                            nat_add(a1487, a1488)?
                        };
                        nat_mul(a1489, a1490)?
                    };
                    let a1492 = 1u64;
                    nat_add(a1491, a1492)?
                };
                nat_add(a1493, a1494)?
            };
            let a1497 = match v3.clone() {
                Adt2::C0(h1495, _, _, _) => {
                    h1495
                }
            };
            nat_sub(a1496, a1497)
        };
        let a1499 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a1498, a1499)
    };
    let v7: u64 = {
        let a1501 = v5;
        let a1502 = match v3.clone() {
            Adt2::C0(_, h1500, _, _) => {
                h1500
            }
        };
        let a1503 = 0u64;
        nat_quot(a1501, a1502, a1503)
    };
    let v8: bool = {
        let a1508 = 0u64;
        let a1509 = {
            let a1505 = v5;
            let a1506 = match v3 {
                Adt2::C0(_, h1504, _, _) => {
                    h1504
                }
            };
            let a1507 = 0u64;
            nat_rem(a1505, a1506, a1507)
        };
        nat_lt(a1508, a1509)
    };
    f38(f23(v0, v1)?, v6, f45(v7, v8)?)
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1548 = v1;
    let a1549 = {
        let a1546 = {
            let a1544 = {
                let a1510 = 1u64;
                let a1511 = 1u64;
                nat_add(a1510, a1511)?
            };
            let a1545 = {
                let a1542 = {
                    let a1512 = 1u64;
                    let a1513 = 1u64;
                    nat_add(a1512, a1513)?
                };
                let a1543 = {
                    let a1540 = {
                        let a1514 = 1u64;
                        let a1515 = 1u64;
                        nat_add(a1514, a1515)?
                    };
                    let a1541 = {
                        let a1538 = {
                            let a1516 = 1u64;
                            let a1517 = 1u64;
                            nat_add(a1516, a1517)?
                        };
                        let a1539 = {
                            let a1536 = {
                                let a1518 = 1u64;
                                let a1519 = 1u64;
                                nat_add(a1518, a1519)?
                            };
                            let a1537 = {
                                let a1534 = {
                                    let a1520 = 1u64;
                                    let a1521 = 1u64;
                                    nat_add(a1520, a1521)?
                                };
                                let a1535 = {
                                    let a1532 = {
                                        let a1522 = 1u64;
                                        let a1523 = 1u64;
                                        nat_add(a1522, a1523)?
                                    };
                                    let a1533 = {
                                        let a1530 = {
                                            let a1524 = 1u64;
                                            let a1525 = 1u64;
                                            nat_add(a1524, a1525)?
                                        };
                                        let a1531 = {
                                            let a1528 = {
                                                let a1526 = 1u64;
                                                let a1527 = 1u64;
                                                nat_add(a1526, a1527)?
                                            };
                                            let a1529 = 1u64;
                                            nat_mul(a1528, a1529)?
                                        };
                                        nat_mul(a1530, a1531)?
                                    };
                                    nat_mul(a1532, a1533)?
                                };
                                nat_mul(a1534, a1535)?
                            };
                            nat_mul(a1536, a1537)?
                        };
                        nat_mul(a1538, a1539)?
                    };
                    nat_mul(a1540, a1541)?
                };
                nat_mul(a1542, a1543)?
            };
            nat_mul(a1544, a1545)?
        };
        let a1547 = 1u64;
        nat_add(a1546, a1547)?
    };
    if nat_lt(a1548, a1549) {
        f47(v0, {
            let a1586 = {
                let a1584 = {
                    let a1550 = 1u64;
                    let a1551 = 1u64;
                    nat_add(a1550, a1551)?
                };
                let a1585 = {
                    let a1582 = {
                        let a1552 = 1u64;
                        let a1553 = 1u64;
                        nat_add(a1552, a1553)?
                    };
                    let a1583 = {
                        let a1580 = {
                            let a1554 = 1u64;
                            let a1555 = 1u64;
                            nat_add(a1554, a1555)?
                        };
                        let a1581 = {
                            let a1578 = {
                                let a1556 = 1u64;
                                let a1557 = 1u64;
                                nat_add(a1556, a1557)?
                            };
                            let a1579 = {
                                let a1576 = {
                                    let a1558 = 1u64;
                                    let a1559 = 1u64;
                                    nat_add(a1558, a1559)?
                                };
                                let a1577 = {
                                    let a1574 = {
                                        let a1560 = 1u64;
                                        let a1561 = 1u64;
                                        nat_add(a1560, a1561)?
                                    };
                                    let a1575 = {
                                        let a1572 = {
                                            let a1562 = 1u64;
                                            let a1563 = 1u64;
                                            nat_add(a1562, a1563)?
                                        };
                                        let a1573 = {
                                            let a1570 = {
                                                let a1564 = 1u64;
                                                let a1565 = 1u64;
                                                nat_add(a1564, a1565)?
                                            };
                                            let a1571 = {
                                                let a1568 = {
                                                    let a1566 = 1u64;
                                                    let a1567 = 1u64;
                                                    nat_add(a1566, a1567)?
                                                };
                                                let a1569 = 1u64;
                                                nat_mul(a1568, a1569)?
                                            };
                                            nat_mul(a1570, a1571)?
                                        };
                                        nat_mul(a1572, a1573)?
                                    };
                                    nat_mul(a1574, a1575)?
                                };
                                nat_mul(a1576, a1577)?
                            };
                            nat_mul(a1578, a1579)?
                        };
                        nat_mul(a1580, a1581)?
                    };
                    nat_mul(a1582, a1583)?
                };
                nat_mul(a1584, a1585)?
            };
            let a1587 = 1u64;
            nat_add(a1586, a1587)?
        }, f39(v2, {
            let a1626 = {
                let a1624 = {
                    let a1622 = {
                        let a1588 = 1u64;
                        let a1589 = 1u64;
                        nat_add(a1588, a1589)?
                    };
                    let a1623 = {
                        let a1620 = {
                            let a1590 = 1u64;
                            let a1591 = 1u64;
                            nat_add(a1590, a1591)?
                        };
                        let a1621 = {
                            let a1618 = {
                                let a1592 = 1u64;
                                let a1593 = 1u64;
                                nat_add(a1592, a1593)?
                            };
                            let a1619 = {
                                let a1616 = {
                                    let a1594 = 1u64;
                                    let a1595 = 1u64;
                                    nat_add(a1594, a1595)?
                                };
                                let a1617 = {
                                    let a1614 = {
                                        let a1596 = 1u64;
                                        let a1597 = 1u64;
                                        nat_add(a1596, a1597)?
                                    };
                                    let a1615 = {
                                        let a1612 = {
                                            let a1598 = 1u64;
                                            let a1599 = 1u64;
                                            nat_add(a1598, a1599)?
                                        };
                                        let a1613 = {
                                            let a1610 = {
                                                let a1600 = 1u64;
                                                let a1601 = 1u64;
                                                nat_add(a1600, a1601)?
                                            };
                                            let a1611 = {
                                                let a1608 = {
                                                    let a1602 = 1u64;
                                                    let a1603 = 1u64;
                                                    nat_add(a1602, a1603)?
                                                };
                                                let a1609 = {
                                                    let a1606 = {
                                                        let a1604 = 1u64;
                                                        let a1605 = 1u64;
                                                        nat_add(a1604, a1605)?
                                                    };
                                                    let a1607 = 1u64;
                                                    nat_mul(a1606, a1607)?
                                                };
                                                nat_mul(a1608, a1609)?
                                            };
                                            nat_mul(a1610, a1611)?
                                        };
                                        nat_mul(a1612, a1613)?
                                    };
                                    nat_mul(a1614, a1615)?
                                };
                                nat_mul(a1616, a1617)?
                            };
                            nat_mul(a1618, a1619)?
                        };
                        nat_mul(a1620, a1621)?
                    };
                    nat_mul(a1622, a1623)?
                };
                let a1625 = 1u64;
                nat_add(a1624, a1625)?
            };
            let a1627 = v1;
            nat_sub(a1626, a1627)
        })?)
    } else {
        f47(v0, v1, v2)
    }
}

pub fn f45(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1635 = {
            let a1632 = v0;
            let a1633 = {
                let a1630 = {
                    let a1628 = 1u64;
                    let a1629 = 1u64;
                    nat_add(a1628, a1629)?
                };
                let a1631 = 1u64;
                nat_mul(a1630, a1631)?
            };
            let a1634 = 0u64;
            nat_rem(a1632, a1633, a1634)
        };
        let a1636 = 0u64;
        nat_eq(a1635, a1636)
    } else {
        false
    } {
        let a1637 = v0;
        let a1638 = 1u64;
        nat_add(a1637, a1638)
    } else {
        Ok(v0)
    }
}

pub fn f46(v0: Adt2) -> R<Option<Adt2>> {
    let a1640 = 0u64;
    let a1641 = match v0.clone() {
        Adt2::C0(_, h1639, _, _) => {
            h1639
        }
    };
    if if if nat_lt(a1640, a1641) {
        let a1644 = match v0.clone() {
            Adt2::C0(_, h1642, _, _) => {
                h1642
            }
        };
        let a1645 = match v0.clone() {
            Adt2::C0(_, _, _, h1643) => {
                h1643
            }
        };
        nat_lt(a1644, a1645)
    } else {
        false
    } {
        let a1648 = match v0.clone() {
            Adt2::C0(_, _, h1646, _) => {
                h1646
            }
        };
        let a1649 = match v0.clone() {
            Adt2::C0(h1647, _, _, _) => {
                h1647
            }
        };
        nat_lt(a1648, a1649)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1651 = match v0.clone() {
                Adt2::C0(h1650, _, _, _) => {
                    h1650
                }
            };
            let a1652 = 1u64;
            nat_sub(a1651, a1652)
        }, {
            let a1655 = match v0.clone() {
                Adt2::C0(_, h1653, _, _) => {
                    h1653
                }
            };
            let a1656 = match v0.clone() {
                Adt2::C0(_, h1654, _, _) => {
                    h1654
                }
            };
            nat_add(a1655, a1656)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1657, _) => {
                h1657
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1658) => {
                h1658
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1671 = v2;
        let a1672 = {
            let a1669 = {
                let a1659 = 1u64;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            };
            let a1670 = {
                let a1667 = {
                    let a1661 = 1u64;
                    let a1662 = 1u64;
                    nat_add(a1661, a1662)?
                };
                let a1668 = {
                    let a1665 = {
                        let a1663 = 1u64;
                        let a1664 = 1u64;
                        nat_add(a1663, a1664)?
                    };
                    let a1666 = 1u64;
                    nat_mul(a1665, a1666)?
                };
                nat_mul(a1667, a1668)?
            };
            nat_mul(a1669, a1670)?
        };
        let a1673 = 0u64;
        nat_rem(a1671, a1672, a1673)
    };
    let v4: u64 = {
        let a1686 = v2;
        let a1687 = {
            let a1684 = {
                let a1674 = 1u64;
                let a1675 = 1u64;
                nat_add(a1674, a1675)?
            };
            let a1685 = {
                let a1682 = {
                    let a1676 = 1u64;
                    let a1677 = 1u64;
                    nat_add(a1676, a1677)?
                };
                let a1683 = {
                    let a1680 = {
                        let a1678 = 1u64;
                        let a1679 = 1u64;
                        nat_add(a1678, a1679)?
                    };
                    let a1681 = 1u64;
                    nat_mul(a1680, a1681)?
                };
                nat_mul(a1682, a1683)?
            };
            nat_mul(a1684, a1685)?
        };
        let a1688 = 0u64;
        nat_quot(a1686, a1687, a1688)
    };
    let v5: bool = {
        let a1697 = {
            let a1695 = {
                let a1689 = 1u64;
                let a1690 = 1u64;
                nat_add(a1689, a1690)?
            };
            let a1696 = {
                let a1693 = {
                    let a1691 = 1u64;
                    let a1692 = 1u64;
                    nat_add(a1691, a1692)?
                };
                let a1694 = 1u64;
                nat_mul(a1693, a1694)?
            };
            nat_mul(a1695, a1696)?
        };
        let a1698 = v3;
        if nat_lt(a1697, a1698) {
            true
        } else {
            let a1707 = v3;
            let a1708 = {
                let a1705 = {
                    let a1699 = 1u64;
                    let a1700 = 1u64;
                    nat_add(a1699, a1700)?
                };
                let a1706 = {
                    let a1703 = {
                        let a1701 = 1u64;
                        let a1702 = 1u64;
                        nat_add(a1701, a1702)?
                    };
                    let a1704 = 1u64;
                    nat_mul(a1703, a1704)?
                };
                nat_mul(a1705, a1706)?
            };
            if nat_eq(a1707, a1708) {
                let a1716 = {
                    let a1713 = v4;
                    let a1714 = {
                        let a1711 = {
                            let a1709 = 1u64;
                            let a1710 = 1u64;
                            nat_add(a1709, a1710)?
                        };
                        let a1712 = 1u64;
                        nat_mul(a1711, a1712)?
                    };
                    let a1715 = 0u64;
                    nat_rem(a1713, a1714, a1715)
                };
                let a1717 = 1u64;
                nat_eq(a1716, a1717)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1718 = v4;
        let a1719 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1718, a1719)?
    };
    let v7: bool = {
        let a1748 = {
            let a1746 = {
                let a1722 = {
                    let a1720 = 1u64;
                    let a1721 = 1u64;
                    nat_add(a1720, a1721)?
                };
                let a1723 = 1u64;
                nat_mul(a1722, a1723)?
            };
            let a1747 = f21({
                let a1744 = {
                    let a1742 = {
                        let a1724 = 1u64;
                        let a1725 = 1u64;
                        nat_add(a1724, a1725)?
                    };
                    let a1743 = {
                        let a1740 = {
                            let a1738 = {
                                let a1726 = 1u64;
                                let a1727 = 1u64;
                                nat_add(a1726, a1727)?
                            };
                            let a1739 = {
                                let a1736 = {
                                    let a1734 = {
                                        let a1728 = 1u64;
                                        let a1729 = 1u64;
                                        nat_add(a1728, a1729)?
                                    };
                                    let a1735 = {
                                        let a1732 = {
                                            let a1730 = 1u64;
                                            let a1731 = 1u64;
                                            nat_add(a1730, a1731)?
                                        };
                                        let a1733 = 1u64;
                                        nat_mul(a1732, a1733)?
                                    };
                                    nat_mul(a1734, a1735)?
                                };
                                let a1737 = 1u64;
                                nat_add(a1736, a1737)?
                            };
                            nat_mul(a1738, a1739)?
                        };
                        let a1741 = 1u64;
                        nat_add(a1740, a1741)?
                    };
                    nat_mul(a1742, a1743)?
                };
                let a1745 = 1u64;
                nat_add(a1744, a1745)?
            })?;
            nat_mul(a1746, a1747)?
        };
        let a1749 = v6;
        nat_le(a1748, a1749)
    };
    let v8: u64 = if v7 {
        let a1754 = v6;
        let a1755 = {
            let a1752 = {
                let a1750 = 1u64;
                let a1751 = 1u64;
                nat_add(a1750, a1751)?
            };
            let a1753 = 1u64;
            nat_mul(a1752, a1753)?
        };
        let a1756 = 0u64;
        nat_quot(a1754, a1755, a1756)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1757 = v1;
        let a1758 = 1u64;
        nat_add(a1757, a1758)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1797 = v9;
        let a1798 = {
            let a1795 = {
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
            let a1796 = 1u64;
            nat_add(a1795, a1796)?
        };
        if if nat_eq(a1797, a1798) {
            let a1821 = v8;
            let a1822 = f21({
                let a1819 = {
                    let a1817 = {
                        let a1799 = 1u64;
                        let a1800 = 1u64;
                        nat_add(a1799, a1800)?
                    };
                    let a1818 = {
                        let a1815 = {
                            let a1813 = {
                                let a1801 = 1u64;
                                let a1802 = 1u64;
                                nat_add(a1801, a1802)?
                            };
                            let a1814 = {
                                let a1811 = {
                                    let a1809 = {
                                        let a1803 = 1u64;
                                        let a1804 = 1u64;
                                        nat_add(a1803, a1804)?
                                    };
                                    let a1810 = {
                                        let a1807 = {
                                            let a1805 = 1u64;
                                            let a1806 = 1u64;
                                            nat_add(a1805, a1806)?
                                        };
                                        let a1808 = 1u64;
                                        nat_mul(a1807, a1808)?
                                    };
                                    nat_mul(a1809, a1810)?
                                };
                                let a1812 = 1u64;
                                nat_add(a1811, a1812)?
                            };
                            nat_mul(a1813, a1814)?
                        };
                        let a1816 = 1u64;
                        nat_add(a1815, a1816)?
                    };
                    nat_mul(a1817, a1818)?
                };
                let a1820 = 1u64;
                nat_add(a1819, a1820)?
            })?;
            nat_lt(a1821, a1822)
        } else {
            false
        } {
            0u64
        } else {
            let a1859 = v9;
            let a1860 = {
                let a1857 = {
                    let a1823 = 1u64;
                    let a1824 = 1u64;
                    nat_add(a1823, a1824)?
                };
                let a1858 = {
                    let a1855 = {
                        let a1825 = 1u64;
                        let a1826 = 1u64;
                        nat_add(a1825, a1826)?
                    };
                    let a1856 = {
                        let a1853 = {
                            let a1827 = 1u64;
                            let a1828 = 1u64;
                            nat_add(a1827, a1828)?
                        };
                        let a1854 = {
                            let a1851 = {
                                let a1829 = 1u64;
                                let a1830 = 1u64;
                                nat_add(a1829, a1830)?
                            };
                            let a1852 = {
                                let a1849 = {
                                    let a1831 = 1u64;
                                    let a1832 = 1u64;
                                    nat_add(a1831, a1832)?
                                };
                                let a1850 = {
                                    let a1847 = {
                                        let a1833 = 1u64;
                                        let a1834 = 1u64;
                                        nat_add(a1833, a1834)?
                                    };
                                    let a1848 = {
                                        let a1845 = {
                                            let a1835 = 1u64;
                                            let a1836 = 1u64;
                                            nat_add(a1835, a1836)?
                                        };
                                        let a1846 = {
                                            let a1843 = {
                                                let a1837 = 1u64;
                                                let a1838 = 1u64;
                                                nat_add(a1837, a1838)?
                                            };
                                            let a1844 = {
                                                let a1841 = {
                                                    let a1839 = 1u64;
                                                    let a1840 = 1u64;
                                                    nat_add(a1839, a1840)?
                                                };
                                                let a1842 = 1u64;
                                                nat_mul(a1841, a1842)?
                                            };
                                            nat_mul(a1843, a1844)?
                                        };
                                        nat_mul(a1845, a1846)?
                                    };
                                    nat_mul(a1847, a1848)?
                                };
                                nat_mul(a1849, a1850)?
                            };
                            nat_mul(a1851, a1852)?
                        };
                        nat_mul(a1853, a1854)?
                    };
                    nat_mul(a1855, a1856)?
                };
                nat_mul(a1857, a1858)?
            };
            nat_sub(a1859, a1860)
        }
    };
    let a1861 = v8;
    let a1862 = 0u64;
    if nat_eq(a1861, a1862) {
        f24(v0)
    } else {
        let a1915 = {
            let a1913 = {
                let a1911 = {
                    let a1863 = 1u64;
                    let a1864 = 1u64;
                    nat_add(a1863, a1864)?
                };
                let a1912 = {
                    let a1909 = {
                        let a1907 = {
                            let a1865 = 1u64;
                            let a1866 = 1u64;
                            nat_add(a1865, a1866)?
                        };
                        let a1908 = {
                            let a1905 = {
                                let a1903 = {
                                    let a1867 = 1u64;
                                    let a1868 = 1u64;
                                    nat_add(a1867, a1868)?
                                };
                                let a1904 = {
                                    let a1901 = {
                                        let a1899 = {
                                            let a1869 = 1u64;
                                            let a1870 = 1u64;
                                            nat_add(a1869, a1870)?
                                        };
                                        let a1900 = {
                                            let a1897 = {
                                                let a1895 = {
                                                    let a1871 = 1u64;
                                                    let a1872 = 1u64;
                                                    nat_add(a1871, a1872)?
                                                };
                                                let a1896 = {
                                                    let a1893 = {
                                                        let a1891 = {
                                                            let a1873 = 1u64;
                                                            let a1874 = 1u64;
                                                            nat_add(a1873, a1874)?
                                                        };
                                                        let a1892 = {
                                                            let a1889 = {
                                                                let a1887 = {
                                                                    let a1875 = 1u64;
                                                                    let a1876 = 1u64;
                                                                    nat_add(a1875, a1876)?
                                                                };
                                                                let a1888 = {
                                                                    let a1885 = {
                                                                        let a1883 = {
                                                                            let a1877 = 1u64;
                                                                            let a1878 = 1u64;
                                                                            nat_add(a1877, a1878)?
                                                                        };
                                                                        let a1884 = {
                                                                            let a1881 = {
                                                                                let a1879 = 1u64;
                                                                                let a1880 = 1u64;
                                                                                nat_add(a1879, a1880)?
                                                                            };
                                                                            let a1882 = 1u64;
                                                                            nat_mul(a1881, a1882)?
                                                                        };
                                                                        nat_mul(a1883, a1884)?
                                                                    };
                                                                    let a1886 = 1u64;
                                                                    nat_add(a1885, a1886)?
                                                                };
                                                                nat_mul(a1887, a1888)?
                                                            };
                                                            let a1890 = 1u64;
                                                            nat_add(a1889, a1890)?
                                                        };
                                                        nat_mul(a1891, a1892)?
                                                    };
                                                    let a1894 = 1u64;
                                                    nat_add(a1893, a1894)?
                                                };
                                                nat_mul(a1895, a1896)?
                                            };
                                            let a1898 = 1u64;
                                            nat_add(a1897, a1898)?
                                        };
                                        nat_mul(a1899, a1900)?
                                    };
                                    let a1902 = 1u64;
                                    nat_add(a1901, a1902)?
                                };
                                nat_mul(a1903, a1904)?
                            };
                            let a1906 = 1u64;
                            nat_add(a1905, a1906)?
                        };
                        nat_mul(a1907, a1908)?
                    };
                    let a1910 = 1u64;
                    nat_add(a1909, a1910)?
                };
                nat_mul(a1911, a1912)?
            };
            let a1914 = 1u64;
            nat_add(a1913, a1914)?
        };
        let a1916 = v9;
        if nat_le(a1915, a1916) {
            f22(v0)
        } else {
            let a1968 = {
                let a1941 = f24(v0)?;
                let a1942 = {
                    let a1939 = v10;
                    let a1940 = f21({
                        let a1937 = {
                            let a1935 = {
                                let a1917 = 1u64;
                                let a1918 = 1u64;
                                nat_add(a1917, a1918)?
                            };
                            let a1936 = {
                                let a1933 = {
                                    let a1931 = {
                                        let a1919 = 1u64;
                                        let a1920 = 1u64;
                                        nat_add(a1919, a1920)?
                                    };
                                    let a1932 = {
                                        let a1929 = {
                                            let a1927 = {
                                                let a1921 = 1u64;
                                                let a1922 = 1u64;
                                                nat_add(a1921, a1922)?
                                            };
                                            let a1928 = {
                                                let a1925 = {
                                                    let a1923 = 1u64;
                                                    let a1924 = 1u64;
                                                    nat_add(a1923, a1924)?
                                                };
                                                let a1926 = 1u64;
                                                nat_mul(a1925, a1926)?
                                            };
                                            nat_mul(a1927, a1928)?
                                        };
                                        let a1930 = 1u64;
                                        nat_add(a1929, a1930)?
                                    };
                                    nat_mul(a1931, a1932)?
                                };
                                let a1934 = 1u64;
                                nat_add(a1933, a1934)?
                            };
                            nat_mul(a1935, a1936)?
                        };
                        let a1938 = 1u64;
                        nat_add(a1937, a1938)?
                    })?;
                    nat_mul(a1939, a1940)?
                };
                nat_add(a1941, a1942)?
            };
            let a1969 = {
                let a1965 = v8;
                let a1966 = f21({
                    let a1963 = {
                        let a1961 = {
                            let a1943 = 1u64;
                            let a1944 = 1u64;
                            nat_add(a1943, a1944)?
                        };
                        let a1962 = {
                            let a1959 = {
                                let a1957 = {
                                    let a1945 = 1u64;
                                    let a1946 = 1u64;
                                    nat_add(a1945, a1946)?
                                };
                                let a1958 = {
                                    let a1955 = {
                                        let a1953 = {
                                            let a1947 = 1u64;
                                            let a1948 = 1u64;
                                            nat_add(a1947, a1948)?
                                        };
                                        let a1954 = {
                                            let a1951 = {
                                                let a1949 = 1u64;
                                                let a1950 = 1u64;
                                                nat_add(a1949, a1950)?
                                            };
                                            let a1952 = 1u64;
                                            nat_mul(a1951, a1952)?
                                        };
                                        nat_mul(a1953, a1954)?
                                    };
                                    let a1956 = 1u64;
                                    nat_add(a1955, a1956)?
                                };
                                nat_mul(a1957, a1958)?
                            };
                            let a1960 = 1u64;
                            nat_add(a1959, a1960)?
                        };
                        nat_mul(a1961, a1962)?
                    };
                    let a1964 = 1u64;
                    nat_add(a1963, a1964)?
                })?;
                let a1967 = 0u64;
                nat_rem(a1965, a1966, a1967)
            };
            nat_add(a1968, a1969)
        }
    }
}
