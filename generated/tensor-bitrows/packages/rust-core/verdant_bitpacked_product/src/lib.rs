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
    F38,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F38 => f38(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F43,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F43 => f43(p0),
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
        Adt0::C7(v75, v76) => {
            let a104 = v75;
            let a105 = 4294967296u64;
            if if nat_lt(a104, a105) {
                let a106 = v76;
                let a107 = 2139095040u64;
                nat_lt(a106, a107)
            } else {
                false
            } {
                Ok(Adt1::C2(f16(f5(v75, v76)?)?))
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
        Adt0::C11(v62, v63, v64) => {
            let a100 = v64;
            let a101 = 18u64;
            if if nat_eq(a100, a101) {
                let a102 = v63;
                let a103 = if v62 {
                    64u64
                } else {
                    128u64
                };
                nat_eq(a102, a103)
            } else {
                false
            } {
                Ok(Adt1::C2(if v62 {
                    4u64
                } else {
                    8u64
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C12(v65, v66, v67) => {
            f13(v65, v66, v67)
        }
        Adt0::C13(v68, v69) => {
            f14(v68, v69)
        }
        Adt0::C14(v70, v71, v72, v73, v74) => {
            f15(v70, v71, v72, v73, v74)
        }
        Adt0::C15(v47, v48, v49) => {
            let v50: u64 = if v47 {
                52u64
            } else {
                64u64
            };
            let a79 = v48;
            let a80 = 256u64;
            if if nat_eq(a79, a80) {
                let a83 = v49;
                let a84 = {
                    let a81 = v50;
                    let a82 = 2u64;
                    nat_add(a81, a82)?
                };
                nat_eq(a83, a84)
            } else {
                false
            } {
                Ok(Adt1::C2(v50))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C16(v51, v52) => {
            f11(v51, v52)
        }
        Adt0::C17(v53, v54, v55, v56, v57) => {
            f12(v53, v54, v55, v56, v57)
        }
        Adt0::C18(v58, v59, v60) => {
            let v61: u64 = {
                let a85 = v59;
                let a86 = v60;
                nat_mul(a85, a86)?
            };
            if v58 {
                let a87 = v61;
                let a88 = 243u64;
                if nat_lt(a87, a88) {
                    Ok(Adt1::C2({
                        let a93 = {
                            let a91 = {
                                let a89 = v61;
                                let a90 = 256u64;
                                nat_mul(a89, a90)?
                            };
                            let a92 = 242u64;
                            nat_add(a91, a92)?
                        };
                        let a94 = 243u64;
                        let a95 = 0u64;
                        nat_quot(a93, a94, a95)
                    }))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                let a96 = v59;
                let a97 = 1u64;
                if if nat_eq(a96, a97) {
                    let a98 = v60;
                    let a99 = 256u64;
                    nat_lt(a98, a99)
                } else {
                    false
                } {
                    Ok(Adt1::C2(v60))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
        Adt0::C19(v41, v42, v43) => {
            let v44: u64 = if v41 {
                54u64
            } else {
                66u64
            };
            let a72 = {
                let a69 = v42;
                let a70 = v44;
                let a71 = 0u64;
                nat_rem(a69, a70, a71)
            };
            let a73 = 0u64;
            if if nat_eq(a72, a73) {
                let a77 = {
                    let a74 = v42;
                    let a75 = v44;
                    let a76 = 0u64;
                    nat_quot(a74, a75, a76)
                };
                let a78 = v43;
                nat_eq(a77, a78)
            } else {
                false
            } {
                Ok(Adt1::C2(v44))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C20(v45, v46) => {
            f10(v45, v46)
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a108 = 0u64;
    let a109 = v2;
    if if nat_lt(a108, a109) {
        let a110 = 0u64;
        let a111 = v3;
        nat_lt(a110, a111)
    } else {
        false
    } {
        Ok(Some({
            let a117 = {
                let a114 = {
                    let a112 = v0;
                    let a113 = v1;
                    nat_mul(a112, a113)?
                };
                let a115 = v2;
                let a116 = 0u64;
                nat_quot(a114, a115, a116)
            };
            let a118 = v3;
            let a119 = 0u64;
            nat_rem(a117, a118, a119)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f2(v0: u64) -> u64 {
    let a120 = v0;
    let a121 = 0u64;
    if nat_eq(a120, a121) {
        1u64
    } else {
        let a122 = v0;
        let a123 = 1u64;
        if nat_eq(a122, a123) {
            2u64
        } else {
            let a124 = v0;
            let a125 = 2u64;
            if nat_eq(a124, a125) {
                4u64
            } else {
                let a126 = v0;
                let a127 = 3u64;
                if nat_eq(a126, a127) {
                    8u64
                } else {
                    let a128 = v0;
                    let a129 = 4u64;
                    if nat_eq(a128, a129) {
                        16u64
                    } else {
                        let a130 = v0;
                        let a131 = 5u64;
                        if nat_eq(a130, a131) {
                            32u64
                        } else {
                            let a132 = v0;
                            let a133 = 6u64;
                            if nat_eq(a132, a133) {
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
    let a134 = v1;
    let a135 = 16u64;
    if if if nat_lt(a134, a135) {
        let a136 = v2;
        let a137 = 256u64;
        nat_lt(a136, a137)
    } else {
        false
    } {
        let a138 = v3;
        let a139 = 4294967296u64;
        nat_lt(a138, a139)
    } else {
        false
    } {
        let v4: u64 = {
            let a140 = v2;
            let a141 = 16u64;
            let a142 = 0u64;
            nat_rem(a140, a141, a142)
        };
        let v5: u64 = {
            let a143 = v2;
            let a144 = 16u64;
            let a145 = 0u64;
            nat_quot(a143, a144, a145)
        };
        if v0 {
            Ok(Adt1::C4({
                let a146 = 18568u64;
                let a147 = v4;
                nat_add(a146, a147)?
            }, {
                let a148 = 18568u64;
                let a149 = v5;
                nat_add(a148, a149)?
            }))
        } else {
            let v6: u64 = {
                let a155 = f2({
                    let a150 = v1;
                    let a151 = 8u64;
                    let a152 = 0u64;
                    nat_rem(a150, a151, a152)
                });
                let a156 = {
                    let a153 = v1;
                    let a154 = 8u64;
                    if nat_lt(a153, a154) {
                        1u64
                    } else {
                        256u64
                    }
                };
                nat_mul(a155, a156)?
            };
            Ok(Adt1::C3(f17(v4, v3, v6)?, f17(v5, v3, {
                let a157 = v6;
                let a158 = 65536u64;
                nat_mul(a157, a158)?
            })?))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a159 = v0;
    let a160 = 16u64;
    if if if nat_lt(a159, a160) {
        let a161 = v1;
        let a162 = 256u64;
        nat_lt(a161, a162)
    } else {
        false
    } {
        let a163 = v2;
        let a164 = 4294967296u64;
        nat_lt(a163, a164)
    } else {
        false
    } {
        let v3: u64 = {
            let a170 = f2({
                let a165 = v0;
                let a166 = 8u64;
                let a167 = 0u64;
                nat_rem(a165, a166, a167)
            });
            let a171 = {
                let a168 = v0;
                let a169 = 8u64;
                if nat_lt(a168, a169) {
                    1u64
                } else {
                    256u64
                }
            };
            nat_mul(a170, a171)?
        };
        Ok(Adt1::C3(f18({
            let a172 = v1;
            let a173 = 16u64;
            let a174 = 0u64;
            nat_rem(a172, a173, a174)
        }, v2, v3)?, f18({
            let a175 = v1;
            let a176 = 16u64;
            let a177 = 0u64;
            nat_quot(a175, a176, a177)
        }, v2, {
            let a178 = v3;
            let a179 = 65536u64;
            nat_mul(a178, a179)?
        })?))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    if if if if f19(v0)? {
        true
    } else {
        f19(v1)?
    } {
        true
    } else {
        if f20(v0)? {
            f21(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f20(v1)? {
            f21(v0)?
        } else {
            false
        }
    } {
        let a266 = {
            let a244 = {
                let a220 = {
                    let a218 = {
                        let a180 = 1u64;
                        let a181 = 1u64;
                        nat_add(a180, a181)?
                    };
                    let a219 = {
                        let a216 = {
                            let a214 = {
                                let a182 = 1u64;
                                let a183 = 1u64;
                                nat_add(a182, a183)?
                            };
                            let a215 = {
                                let a212 = {
                                    let a210 = {
                                        let a184 = 1u64;
                                        let a185 = 1u64;
                                        nat_add(a184, a185)?
                                    };
                                    let a211 = {
                                        let a208 = {
                                            let a206 = {
                                                let a186 = 1u64;
                                                let a187 = 1u64;
                                                nat_add(a186, a187)?
                                            };
                                            let a207 = {
                                                let a204 = {
                                                    let a202 = {
                                                        let a188 = 1u64;
                                                        let a189 = 1u64;
                                                        nat_add(a188, a189)?
                                                    };
                                                    let a203 = {
                                                        let a200 = {
                                                            let a198 = {
                                                                let a190 = 1u64;
                                                                let a191 = 1u64;
                                                                nat_add(a190, a191)?
                                                            };
                                                            let a199 = {
                                                                let a196 = {
                                                                    let a194 = {
                                                                        let a192 = 1u64;
                                                                        let a193 = 1u64;
                                                                        nat_add(a192, a193)?
                                                                    };
                                                                    let a195 = 1u64;
                                                                    nat_mul(a194, a195)?
                                                                };
                                                                let a197 = 1u64;
                                                                nat_add(a196, a197)?
                                                            };
                                                            nat_mul(a198, a199)?
                                                        };
                                                        let a201 = 1u64;
                                                        nat_add(a200, a201)?
                                                    };
                                                    nat_mul(a202, a203)?
                                                };
                                                let a205 = 1u64;
                                                nat_add(a204, a205)?
                                            };
                                            nat_mul(a206, a207)?
                                        };
                                        let a209 = 1u64;
                                        nat_add(a208, a209)?
                                    };
                                    nat_mul(a210, a211)?
                                };
                                let a213 = 1u64;
                                nat_add(a212, a213)?
                            };
                            nat_mul(a214, a215)?
                        };
                        let a217 = 1u64;
                        nat_add(a216, a217)?
                    };
                    nat_mul(a218, a219)?
                };
                let a221 = 1u64;
                nat_add(a220, a221)?
            };
            let a245 = f22({
                let a242 = {
                    let a240 = {
                        let a222 = 1u64;
                        let a223 = 1u64;
                        nat_add(a222, a223)?
                    };
                    let a241 = {
                        let a238 = {
                            let a236 = {
                                let a224 = 1u64;
                                let a225 = 1u64;
                                nat_add(a224, a225)?
                            };
                            let a237 = {
                                let a234 = {
                                    let a232 = {
                                        let a226 = 1u64;
                                        let a227 = 1u64;
                                        nat_add(a226, a227)?
                                    };
                                    let a233 = {
                                        let a230 = {
                                            let a228 = 1u64;
                                            let a229 = 1u64;
                                            nat_add(a228, a229)?
                                        };
                                        let a231 = 1u64;
                                        nat_mul(a230, a231)?
                                    };
                                    nat_mul(a232, a233)?
                                };
                                let a235 = 1u64;
                                nat_add(a234, a235)?
                            };
                            nat_mul(a236, a237)?
                        };
                        let a239 = 1u64;
                        nat_add(a238, a239)?
                    };
                    nat_mul(a240, a241)?
                };
                let a243 = 1u64;
                nat_add(a242, a243)?
            })?;
            nat_mul(a244, a245)?
        };
        let a267 = f22({
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
        })?;
        nat_add(a266, a267)
    } else {
        if if f20(v0)? {
            true
        } else {
            f20(v1)?
        } {
            f23(f24(v0, v1)?)
        } else {
            if if f21(v0)? {
                true
            } else {
                f21(v1)?
            } {
                f25(f24(v0, v1)?)
            } else {
                f26(v0, v1)
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a271 = {
            let a268 = v0;
            let a269 = 32768u64;
            let a270 = 0u64;
            nat_quot(a268, a269, a270)
        };
        let a272 = 2147483648u64;
        nat_mul(a271, a272)?
    };
    let v2: u64 = {
        let a276 = {
            let a273 = v0;
            let a274 = 32768u64;
            let a275 = 0u64;
            nat_rem(a273, a274, a275)
        };
        let a277 = 1024u64;
        let a278 = 0u64;
        nat_quot(a276, a277, a278)
    };
    let v3: u64 = {
        let a279 = v0;
        let a280 = 1024u64;
        let a281 = 0u64;
        nat_rem(a279, a280, a281)
    };
    let a316 = v1;
    let a317 = {
        let a282 = v2;
        let a283 = 31u64;
        if nat_eq(a282, a283) {
            let a292 = 2139095040u64;
            let a293 = {
                let a290 = {
                    let a284 = 0u64;
                    let a285 = v3;
                    if if nat_lt(a284, a285) {
                        let a286 = v3;
                        let a287 = 512u64;
                        nat_lt(a286, a287)
                    } else {
                        false
                    } {
                        let a288 = v3;
                        let a289 = 512u64;
                        nat_add(a288, a289)?
                    } else {
                        v3
                    }
                };
                let a291 = 8192u64;
                nat_mul(a290, a291)?
            };
            nat_add(a292, a293)?
        } else {
            let a294 = v2;
            let a295 = 0u64;
            if nat_eq(a294, a295) {
                let a296 = v3;
                let a297 = 0u64;
                if nat_eq(a296, a297) {
                    0u64
                } else {
                    let v4: Adt2 = f27(113u64, v3, 0u64, 1024u64)?;
                    let a306 = {
                        let a299 = match v4.clone() {
                            Adt2::C0(h298, _, _, _) => {
                                h298
                            }
                        };
                        let a300 = 8388608u64;
                        nat_mul(a299, a300)?
                    };
                    let a307 = {
                        let a304 = {
                            let a302 = match v4 {
                                Adt2::C0(_, h301, _, _) => {
                                    h301
                                }
                            };
                            let a303 = 1024u64;
                            nat_sub(a302, a303)
                        };
                        let a305 = 8192u64;
                        nat_mul(a304, a305)?
                    };
                    nat_add(a306, a307)?
                }
            } else {
                let a314 = {
                    let a310 = {
                        let a308 = v2;
                        let a309 = 112u64;
                        nat_add(a308, a309)?
                    };
                    let a311 = 8388608u64;
                    nat_mul(a310, a311)?
                };
                let a315 = {
                    let a312 = v3;
                    let a313 = 8192u64;
                    nat_mul(a312, a313)?
                };
                nat_add(a314, a315)?
            }
        }
    };
    nat_add(a316, a317)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f19(v0)? {
        true
    } else {
        f19(v1)?
    } {
        let a404 = {
            let a382 = {
                let a358 = {
                    let a356 = {
                        let a318 = 1u64;
                        let a319 = 1u64;
                        nat_add(a318, a319)?
                    };
                    let a357 = {
                        let a354 = {
                            let a352 = {
                                let a320 = 1u64;
                                let a321 = 1u64;
                                nat_add(a320, a321)?
                            };
                            let a353 = {
                                let a350 = {
                                    let a348 = {
                                        let a322 = 1u64;
                                        let a323 = 1u64;
                                        nat_add(a322, a323)?
                                    };
                                    let a349 = {
                                        let a346 = {
                                            let a344 = {
                                                let a324 = 1u64;
                                                let a325 = 1u64;
                                                nat_add(a324, a325)?
                                            };
                                            let a345 = {
                                                let a342 = {
                                                    let a340 = {
                                                        let a326 = 1u64;
                                                        let a327 = 1u64;
                                                        nat_add(a326, a327)?
                                                    };
                                                    let a341 = {
                                                        let a338 = {
                                                            let a336 = {
                                                                let a328 = 1u64;
                                                                let a329 = 1u64;
                                                                nat_add(a328, a329)?
                                                            };
                                                            let a337 = {
                                                                let a334 = {
                                                                    let a332 = {
                                                                        let a330 = 1u64;
                                                                        let a331 = 1u64;
                                                                        nat_add(a330, a331)?
                                                                    };
                                                                    let a333 = 1u64;
                                                                    nat_mul(a332, a333)?
                                                                };
                                                                let a335 = 1u64;
                                                                nat_add(a334, a335)?
                                                            };
                                                            nat_mul(a336, a337)?
                                                        };
                                                        let a339 = 1u64;
                                                        nat_add(a338, a339)?
                                                    };
                                                    nat_mul(a340, a341)?
                                                };
                                                let a343 = 1u64;
                                                nat_add(a342, a343)?
                                            };
                                            nat_mul(a344, a345)?
                                        };
                                        let a347 = 1u64;
                                        nat_add(a346, a347)?
                                    };
                                    nat_mul(a348, a349)?
                                };
                                let a351 = 1u64;
                                nat_add(a350, a351)?
                            };
                            nat_mul(a352, a353)?
                        };
                        let a355 = 1u64;
                        nat_add(a354, a355)?
                    };
                    nat_mul(a356, a357)?
                };
                let a359 = 1u64;
                nat_add(a358, a359)?
            };
            let a383 = f22({
                let a380 = {
                    let a378 = {
                        let a360 = 1u64;
                        let a361 = 1u64;
                        nat_add(a360, a361)?
                    };
                    let a379 = {
                        let a376 = {
                            let a374 = {
                                let a362 = 1u64;
                                let a363 = 1u64;
                                nat_add(a362, a363)?
                            };
                            let a375 = {
                                let a372 = {
                                    let a370 = {
                                        let a364 = 1u64;
                                        let a365 = 1u64;
                                        nat_add(a364, a365)?
                                    };
                                    let a371 = {
                                        let a368 = {
                                            let a366 = 1u64;
                                            let a367 = 1u64;
                                            nat_add(a366, a367)?
                                        };
                                        let a369 = 1u64;
                                        nat_mul(a368, a369)?
                                    };
                                    nat_mul(a370, a371)?
                                };
                                let a373 = 1u64;
                                nat_add(a372, a373)?
                            };
                            nat_mul(a374, a375)?
                        };
                        let a377 = 1u64;
                        nat_add(a376, a377)?
                    };
                    nat_mul(a378, a379)?
                };
                let a381 = 1u64;
                nat_add(a380, a381)?
            })?;
            nat_mul(a382, a383)?
        };
        let a405 = f22({
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
        })?;
        nat_add(a404, a405)
    } else {
        if if f20(v0)? {
            f20(v1)?
        } else {
            false
        } {
            let a406 = f28(v0)?;
            let a407 = f28(v1)?;
            if nat_eq(a406, a407) {
                Ok(v0)
            } else {
                let a494 = {
                    let a472 = {
                        let a448 = {
                            let a446 = {
                                let a408 = 1u64;
                                let a409 = 1u64;
                                nat_add(a408, a409)?
                            };
                            let a447 = {
                                let a444 = {
                                    let a442 = {
                                        let a410 = 1u64;
                                        let a411 = 1u64;
                                        nat_add(a410, a411)?
                                    };
                                    let a443 = {
                                        let a440 = {
                                            let a438 = {
                                                let a412 = 1u64;
                                                let a413 = 1u64;
                                                nat_add(a412, a413)?
                                            };
                                            let a439 = {
                                                let a436 = {
                                                    let a434 = {
                                                        let a414 = 1u64;
                                                        let a415 = 1u64;
                                                        nat_add(a414, a415)?
                                                    };
                                                    let a435 = {
                                                        let a432 = {
                                                            let a430 = {
                                                                let a416 = 1u64;
                                                                let a417 = 1u64;
                                                                nat_add(a416, a417)?
                                                            };
                                                            let a431 = {
                                                                let a428 = {
                                                                    let a426 = {
                                                                        let a418 = 1u64;
                                                                        let a419 = 1u64;
                                                                        nat_add(a418, a419)?
                                                                    };
                                                                    let a427 = {
                                                                        let a424 = {
                                                                            let a422 = {
                                                                                let a420 = 1u64;
                                                                                let a421 = 1u64;
                                                                                nat_add(a420, a421)?
                                                                            };
                                                                            let a423 = 1u64;
                                                                            nat_mul(a422, a423)?
                                                                        };
                                                                        let a425 = 1u64;
                                                                        nat_add(a424, a425)?
                                                                    };
                                                                    nat_mul(a426, a427)?
                                                                };
                                                                let a429 = 1u64;
                                                                nat_add(a428, a429)?
                                                            };
                                                            nat_mul(a430, a431)?
                                                        };
                                                        let a433 = 1u64;
                                                        nat_add(a432, a433)?
                                                    };
                                                    nat_mul(a434, a435)?
                                                };
                                                let a437 = 1u64;
                                                nat_add(a436, a437)?
                                            };
                                            nat_mul(a438, a439)?
                                        };
                                        let a441 = 1u64;
                                        nat_add(a440, a441)?
                                    };
                                    nat_mul(a442, a443)?
                                };
                                let a445 = 1u64;
                                nat_add(a444, a445)?
                            };
                            nat_mul(a446, a447)?
                        };
                        let a449 = 1u64;
                        nat_add(a448, a449)?
                    };
                    let a473 = f22({
                        let a470 = {
                            let a468 = {
                                let a450 = 1u64;
                                let a451 = 1u64;
                                nat_add(a450, a451)?
                            };
                            let a469 = {
                                let a466 = {
                                    let a464 = {
                                        let a452 = 1u64;
                                        let a453 = 1u64;
                                        nat_add(a452, a453)?
                                    };
                                    let a465 = {
                                        let a462 = {
                                            let a460 = {
                                                let a454 = 1u64;
                                                let a455 = 1u64;
                                                nat_add(a454, a455)?
                                            };
                                            let a461 = {
                                                let a458 = {
                                                    let a456 = 1u64;
                                                    let a457 = 1u64;
                                                    nat_add(a456, a457)?
                                                };
                                                let a459 = 1u64;
                                                nat_mul(a458, a459)?
                                            };
                                            nat_mul(a460, a461)?
                                        };
                                        let a463 = 1u64;
                                        nat_add(a462, a463)?
                                    };
                                    nat_mul(a464, a465)?
                                };
                                let a467 = 1u64;
                                nat_add(a466, a467)?
                            };
                            nat_mul(a468, a469)?
                        };
                        let a471 = 1u64;
                        nat_add(a470, a471)?
                    })?;
                    nat_mul(a472, a473)?
                };
                let a495 = f22({
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
                })?;
                nat_add(a494, a495)
            }
        } else {
            if f20(v0)? {
                Ok(v0)
            } else {
                if f20(v1)? {
                    Ok(v1)
                } else {
                    let a496 = f29(v1)?;
                    let a497 = f29(v0)?;
                    if nat_le(a496, a497) {
                        f30(v0, v1)
                    } else {
                        f30(v1, v0)
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
    let a501 = {
        let a498 = v1;
        let a499 = v3;
        let a500 = 0u64;
        nat_rem(a498, a499, a500)
    };
    let a502 = 0u64;
    if if nat_eq(a501, a502) {
        let a508 = {
            let a506 = {
                let a503 = v1;
                let a504 = v3;
                let a505 = 0u64;
                nat_quot(a503, a504, a505)
            };
            let a507 = v4;
            nat_mul(a506, a507)?
        };
        let a509 = v2;
        nat_eq(a508, a509)
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
    let a510 = v0;
    let a511 = 4294967296u64;
    if if if if nat_lt(a510, a511) {
        let a512 = v1;
        let a513 = 4294967296u64;
        nat_lt(a512, a513)
    } else {
        false
    } {
        let a514 = v2;
        let a515 = 4294967296u64;
        nat_lt(a514, a515)
    } else {
        false
    } {
        let a516 = v3;
        let a517 = 65536u64;
        nat_lt(a516, a517)
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
    let a518 = v1;
    let a519 = 256u64;
    if nat_lt(a518, a519) {
        let v2: u64 = f31(v0, v1, 1u64)?;
        let v3: u64 = f31(v0, v1, if v0 {
            3u64
        } else {
            4u64
        })?;
        let v4: u64 = f31(v0, v1, if v0 {
            9u64
        } else {
            16u64
        })?;
        let v5: u64 = f31(v0, v1, if v0 {
            27u64
        } else {
            64u64
        })?;
        let v6: u64 = if v0 {
            f31(true, v1, 81u64)?
        } else {
            0u64
        };
        Ok(Adt1::C2({
            let a534 = v2;
            let a535 = {
                let a532 = 256u64;
                let a533 = {
                    let a530 = v3;
                    let a531 = {
                        let a528 = 256u64;
                        let a529 = {
                            let a526 = v4;
                            let a527 = {
                                let a524 = 256u64;
                                let a525 = {
                                    let a522 = v5;
                                    let a523 = {
                                        let a520 = 256u64;
                                        let a521 = v6;
                                        nat_mul(a520, a521)?
                                    };
                                    nat_add(a522, a523)?
                                };
                                nat_mul(a524, a525)?
                            };
                            nat_add(a526, a527)?
                        };
                        nat_mul(a528, a529)?
                    };
                    nat_add(a530, a531)?
                };
                nat_mul(a532, a533)?
            };
            nat_add(a534, a535)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f11(v0: bool, v1: u64) -> R<Adt1> {
    if v0 {
        let a536 = v1;
        let a537 = 32u64;
        if nat_lt(a536, a537) {
            Ok(Adt1::C1(v1, 32u64, 5u64, 1u64))
        } else {
            let a538 = v1;
            let a539 = 48u64;
            if nat_lt(a538, a539) {
                Ok(Adt1::C1({
                    let a542 = 160u64;
                    let a543 = {
                        let a540 = v1;
                        let a541 = 32u64;
                        nat_sub(a540, a541)
                    };
                    nat_add(a542, a543)?
                }, 16u64, 5u64, 1u64))
            } else {
                let a544 = v1;
                let a545 = 52u64;
                if nat_lt(a544, a545) {
                    Ok(Adt1::C1({
                        let a548 = 240u64;
                        let a549 = {
                            let a546 = v1;
                            let a547 = 48u64;
                            nat_sub(a546, a547)
                        };
                        nat_add(a548, a549)?
                    }, 4u64, 4u64, 3u64))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    } else {
        let a550 = v1;
        let a551 = 64u64;
        if nat_lt(a550, a551) {
            Ok(Adt1::C1({
                let a560 = {
                    let a555 = 128u64;
                    let a556 = {
                        let a552 = v1;
                        let a553 = 32u64;
                        let a554 = 0u64;
                        nat_quot(a552, a553, a554)
                    };
                    nat_mul(a555, a556)?
                };
                let a561 = {
                    let a557 = v1;
                    let a558 = 32u64;
                    let a559 = 0u64;
                    nat_rem(a557, a558, a559)
                };
                nat_add(a560, a561)?
            }, 32u64, 4u64, 1u64))
        } else {
            Ok(Adt1::C0)
        }
    }
}

pub fn f12(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a562 = v2;
    let a563 = 256u64;
    if if nat_lt(a562, a563) {
        let a564 = v1;
        let a565 = if v0 {
            5u64
        } else {
            4u64
        };
        nat_lt(a564, a565)
    } else {
        false
    } {
        let m566 = f15(true, 0u64, 0u64, v3, v4)?;
        match m566 {
            Adt1::C0 => {
                Ok(Adt1::C0)
            }
            Adt1::C1(_, _, _, _) => {
                Ok(Adt1::C0)
            }
            Adt1::C2(v5) => {
                let a567 = v5;
                let a568 = 3u64;
                if nat_lt(a567, a568) {
                    Ok(Adt1::C2(if v0 {
                        let a571 = {
                            let a569 = v2;
                            let a570 = 3u64;
                            nat_mul(a569, a570)?
                        };
                        let a572 = v5;
                        nat_add(a571, a572)?
                    } else {
                        let a577 = v2;
                        let a578 = {
                            let a575 = v5;
                            let a576 = f2({
                                let a573 = v1;
                                let a574 = 2u64;
                                nat_mul(a573, a574)?
                            });
                            nat_mul(a575, a576)?
                        };
                        nat_add(a577, a578)?
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

pub fn f13(v0: bool, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = f29(v2)?;
    let a579 = v2;
    let a580 = 4294967296u64;
    if if nat_lt(a579, a580) {
        let a581 = v3;
        let a582 = 2139095040u64;
        nat_lt(a581, a582)
    } else {
        false
    } {
        Ok(Adt1::C2(if v0 {
            let a583 = v1;
            let a584 = v3;
            if nat_lt(a583, a584) {
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

pub fn f14(v0: bool, v1: u64) -> R<Adt1> {
    let v2: u64 = if v0 {
        v1
    } else {
        f32(v1, 1124073472u64)?
    };
    let v3: u64 = if v0 {
        let a585 = v2;
        let a586 = 0u64;
        if nat_eq(a585, a586) {
            0u64
        } else {
            f32(1065353216u64, v2)?
        }
    } else {
        0u64
    };
    let a587 = v3;
    let a588 = 2139095040u64;
    if nat_lt(a587, a588) {
        Ok(Adt1::C3(f16(v2)?, v3))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a589 = v2;
    let a590 = 256u64;
    if if nat_lt(a589, a590) {
        let a591 = v1;
        let a592 = if v0 {
            4u64
        } else {
            8u64
        };
        nat_lt(a591, a592)
    } else {
        false
    } {
        let v5: u64 = if v0 {
            f5(v3, v4)?
        } else {
            v3
        };
        let v6: u64 = f29(v5)?;
        let v7: u64 = if v0 {
            let a593 = v6;
            let a594 = 1056964608u64;
            if nat_lt(a593, a594) {
                1u64
            } else {
                let a595 = v5;
                let a596 = 2147483648u64;
                if nat_lt(a595, a596) {
                    let a597 = v6;
                    let a598 = 1069547520u64;
                    if nat_lt(a597, a598) {
                        2u64
                    } else {
                        3u64
                    }
                } else {
                    0u64
                }
            }
        } else {
            let a599 = v3;
            let a600 = 2147483648u64;
            if if nat_lt(a599, a600) {
                true
            } else {
                let a601 = v6;
                let a602 = 0u64;
                nat_eq(a601, a602)
            } {
                1u64
            } else {
                0u64
            }
        };
        Ok(Adt1::C2({
            let a607 = v2;
            let a608 = {
                let a605 = v7;
                let a606 = f2(if v0 {
                    let a603 = v1;
                    let a604 = 2u64;
                    nat_mul(a603, a604)?
                } else {
                    v1
                });
                nat_mul(a605, a606)?
            };
            nat_add(a607, a608)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a609 = f28(v0)?;
        let a610 = 32768u64;
        nat_mul(a609, a610)?
    };
    let v2: u64 = f33(v0)?;
    let a629 = v1;
    let a630 = {
        let a611 = v2;
        let a612 = 255u64;
        if nat_eq(a611, a612) {
            let a613 = f34(v0)?;
            let a614 = 0u64;
            if nat_eq(a613, a614) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a615 = 143u64;
            let a616 = v2;
            if nat_le(a615, a616) {
                31744u64
            } else {
                let a617 = v2;
                let a618 = 102u64;
                if nat_lt(a617, a618) {
                    0u64
                } else {
                    let a619 = 113u64;
                    let a620 = v2;
                    if nat_le(a619, a620) {
                        let a625 = {
                            let a623 = {
                                let a621 = v2;
                                let a622 = 113u64;
                                nat_sub(a621, a622)
                            };
                            let a624 = 1024u64;
                            nat_mul(a623, a624)?
                        };
                        let a626 = f35(f36(v0)?, 8192u64)?;
                        nat_add(a625, a626)?
                    } else {
                        f35(f36(v0)?, f22({
                            let a627 = 126u64;
                            let a628 = v2;
                            nat_sub(a627, a628)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a629, a630)
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a637 = {
        let a634 = {
            let a631 = v1;
            let a632 = v2;
            let a633 = 0u64;
            nat_quot(a631, a632, a633)
        };
        let a635 = 2u64;
        let a636 = 0u64;
        nat_rem(a634, a635, a636)
    };
    let a638 = 1u64;
    if nat_eq(a637, a638) {
        Ok(v0)
    } else {
        let a639 = 240u64;
        let a640 = v0;
        nat_add(a639, a640)
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a647 = {
        let a644 = {
            let a641 = v1;
            let a642 = v2;
            let a643 = 0u64;
            nat_quot(a641, a642, a643)
        };
        let a645 = 2u64;
        let a646 = 0u64;
        nat_rem(a644, a645, a646)
    };
    let a648 = 1u64;
    if nat_eq(a647, a648) {
        let a649 = 16u64;
        let a650 = v0;
        nat_add(a649, a650)
    } else {
        Ok(v0)
    }
}

pub fn f19(v0: u64) -> R<bool> {
    let a693 = f33(v0)?;
    let a694 = {
        let a691 = {
            let a689 = {
                let a651 = 1u64;
                let a652 = 1u64;
                nat_add(a651, a652)?
            };
            let a690 = {
                let a687 = {
                    let a685 = {
                        let a653 = 1u64;
                        let a654 = 1u64;
                        nat_add(a653, a654)?
                    };
                    let a686 = {
                        let a683 = {
                            let a681 = {
                                let a655 = 1u64;
                                let a656 = 1u64;
                                nat_add(a655, a656)?
                            };
                            let a682 = {
                                let a679 = {
                                    let a677 = {
                                        let a657 = 1u64;
                                        let a658 = 1u64;
                                        nat_add(a657, a658)?
                                    };
                                    let a678 = {
                                        let a675 = {
                                            let a673 = {
                                                let a659 = 1u64;
                                                let a660 = 1u64;
                                                nat_add(a659, a660)?
                                            };
                                            let a674 = {
                                                let a671 = {
                                                    let a669 = {
                                                        let a661 = 1u64;
                                                        let a662 = 1u64;
                                                        nat_add(a661, a662)?
                                                    };
                                                    let a670 = {
                                                        let a667 = {
                                                            let a665 = {
                                                                let a663 = 1u64;
                                                                let a664 = 1u64;
                                                                nat_add(a663, a664)?
                                                            };
                                                            let a666 = 1u64;
                                                            nat_mul(a665, a666)?
                                                        };
                                                        let a668 = 1u64;
                                                        nat_add(a667, a668)?
                                                    };
                                                    nat_mul(a669, a670)?
                                                };
                                                let a672 = 1u64;
                                                nat_add(a671, a672)?
                                            };
                                            nat_mul(a673, a674)?
                                        };
                                        let a676 = 1u64;
                                        nat_add(a675, a676)?
                                    };
                                    nat_mul(a677, a678)?
                                };
                                let a680 = 1u64;
                                nat_add(a679, a680)?
                            };
                            nat_mul(a681, a682)?
                        };
                        let a684 = 1u64;
                        nat_add(a683, a684)?
                    };
                    nat_mul(a685, a686)?
                };
                let a688 = 1u64;
                nat_add(a687, a688)?
            };
            nat_mul(a689, a690)?
        };
        let a692 = 1u64;
        nat_add(a691, a692)?
    };
    if nat_eq(a693, a694) {
        let a695 = 0u64;
        let a696 = f34(v0)?;
        Ok(nat_lt(a695, a696))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u64) -> R<bool> {
    let a739 = f33(v0)?;
    let a740 = {
        let a737 = {
            let a735 = {
                let a697 = 1u64;
                let a698 = 1u64;
                nat_add(a697, a698)?
            };
            let a736 = {
                let a733 = {
                    let a731 = {
                        let a699 = 1u64;
                        let a700 = 1u64;
                        nat_add(a699, a700)?
                    };
                    let a732 = {
                        let a729 = {
                            let a727 = {
                                let a701 = 1u64;
                                let a702 = 1u64;
                                nat_add(a701, a702)?
                            };
                            let a728 = {
                                let a725 = {
                                    let a723 = {
                                        let a703 = 1u64;
                                        let a704 = 1u64;
                                        nat_add(a703, a704)?
                                    };
                                    let a724 = {
                                        let a721 = {
                                            let a719 = {
                                                let a705 = 1u64;
                                                let a706 = 1u64;
                                                nat_add(a705, a706)?
                                            };
                                            let a720 = {
                                                let a717 = {
                                                    let a715 = {
                                                        let a707 = 1u64;
                                                        let a708 = 1u64;
                                                        nat_add(a707, a708)?
                                                    };
                                                    let a716 = {
                                                        let a713 = {
                                                            let a711 = {
                                                                let a709 = 1u64;
                                                                let a710 = 1u64;
                                                                nat_add(a709, a710)?
                                                            };
                                                            let a712 = 1u64;
                                                            nat_mul(a711, a712)?
                                                        };
                                                        let a714 = 1u64;
                                                        nat_add(a713, a714)?
                                                    };
                                                    nat_mul(a715, a716)?
                                                };
                                                let a718 = 1u64;
                                                nat_add(a717, a718)?
                                            };
                                            nat_mul(a719, a720)?
                                        };
                                        let a722 = 1u64;
                                        nat_add(a721, a722)?
                                    };
                                    nat_mul(a723, a724)?
                                };
                                let a726 = 1u64;
                                nat_add(a725, a726)?
                            };
                            nat_mul(a727, a728)?
                        };
                        let a730 = 1u64;
                        nat_add(a729, a730)?
                    };
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
    if nat_eq(a739, a740) {
        let a741 = f34(v0)?;
        let a742 = 0u64;
        Ok(nat_eq(a741, a742))
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64) -> R<bool> {
    let a743 = f29(v0)?;
    let a744 = 0u64;
    Ok(nat_eq(a743, a744))
}

pub fn f22(v0: u64) -> R<u64> {
    f37(Fn0::F38, v0, 1u64)
}

pub fn f23(v0: u64) -> R<u64> {
    let a811 = f25(v0)?;
    let a812 = {
        let a809 = {
            let a785 = {
                let a783 = {
                    let a745 = 1u64;
                    let a746 = 1u64;
                    nat_add(a745, a746)?
                };
                let a784 = {
                    let a781 = {
                        let a779 = {
                            let a747 = 1u64;
                            let a748 = 1u64;
                            nat_add(a747, a748)?
                        };
                        let a780 = {
                            let a777 = {
                                let a775 = {
                                    let a749 = 1u64;
                                    let a750 = 1u64;
                                    nat_add(a749, a750)?
                                };
                                let a776 = {
                                    let a773 = {
                                        let a771 = {
                                            let a751 = 1u64;
                                            let a752 = 1u64;
                                            nat_add(a751, a752)?
                                        };
                                        let a772 = {
                                            let a769 = {
                                                let a767 = {
                                                    let a753 = 1u64;
                                                    let a754 = 1u64;
                                                    nat_add(a753, a754)?
                                                };
                                                let a768 = {
                                                    let a765 = {
                                                        let a763 = {
                                                            let a755 = 1u64;
                                                            let a756 = 1u64;
                                                            nat_add(a755, a756)?
                                                        };
                                                        let a764 = {
                                                            let a761 = {
                                                                let a759 = {
                                                                    let a757 = 1u64;
                                                                    let a758 = 1u64;
                                                                    nat_add(a757, a758)?
                                                                };
                                                                let a760 = 1u64;
                                                                nat_mul(a759, a760)?
                                                            };
                                                            let a762 = 1u64;
                                                            nat_add(a761, a762)?
                                                        };
                                                        nat_mul(a763, a764)?
                                                    };
                                                    let a766 = 1u64;
                                                    nat_add(a765, a766)?
                                                };
                                                nat_mul(a767, a768)?
                                            };
                                            let a770 = 1u64;
                                            nat_add(a769, a770)?
                                        };
                                        nat_mul(a771, a772)?
                                    };
                                    let a774 = 1u64;
                                    nat_add(a773, a774)?
                                };
                                nat_mul(a775, a776)?
                            };
                            let a778 = 1u64;
                            nat_add(a777, a778)?
                        };
                        nat_mul(a779, a780)?
                    };
                    let a782 = 1u64;
                    nat_add(a781, a782)?
                };
                nat_mul(a783, a784)?
            };
            let a786 = 1u64;
            nat_add(a785, a786)?
        };
        let a810 = f22({
            let a807 = {
                let a805 = {
                    let a787 = 1u64;
                    let a788 = 1u64;
                    nat_add(a787, a788)?
                };
                let a806 = {
                    let a803 = {
                        let a801 = {
                            let a789 = 1u64;
                            let a790 = 1u64;
                            nat_add(a789, a790)?
                        };
                        let a802 = {
                            let a799 = {
                                let a797 = {
                                    let a791 = 1u64;
                                    let a792 = 1u64;
                                    nat_add(a791, a792)?
                                };
                                let a798 = {
                                    let a795 = {
                                        let a793 = 1u64;
                                        let a794 = 1u64;
                                        nat_add(a793, a794)?
                                    };
                                    let a796 = 1u64;
                                    nat_mul(a795, a796)?
                                };
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
        })?;
        nat_mul(a809, a810)?
    };
    nat_add(a811, a812)
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a819 = {
        let a813 = f28(v0)?;
        let a814 = f28(v1)?;
        nat_add(a813, a814)?
    };
    let a820 = {
        let a817 = {
            let a815 = 1u64;
            let a816 = 1u64;
            nat_add(a815, a816)?
        };
        let a818 = 1u64;
        nat_mul(a817, a818)?
    };
    let a821 = 0u64;
    Ok(nat_rem(a819, a820, a821))
}

pub fn f25(v0: u64) -> R<u64> {
    let a846 = v0;
    let a847 = f22({
        let a844 = {
            let a842 = {
                let a822 = 1u64;
                let a823 = 1u64;
                nat_add(a822, a823)?
            };
            let a843 = {
                let a840 = {
                    let a838 = {
                        let a824 = 1u64;
                        let a825 = 1u64;
                        nat_add(a824, a825)?
                    };
                    let a839 = {
                        let a836 = {
                            let a834 = {
                                let a826 = 1u64;
                                let a827 = 1u64;
                                nat_add(a826, a827)?
                            };
                            let a835 = {
                                let a832 = {
                                    let a830 = {
                                        let a828 = 1u64;
                                        let a829 = 1u64;
                                        nat_add(a828, a829)?
                                    };
                                    let a831 = 1u64;
                                    nat_mul(a830, a831)?
                                };
                                let a833 = 1u64;
                                nat_add(a832, a833)?
                            };
                            nat_mul(a834, a835)?
                        };
                        let a837 = 1u64;
                        nat_add(a836, a837)?
                    };
                    nat_mul(a838, a839)?
                };
                let a841 = 1u64;
                nat_add(a840, a841)?
            };
            nat_mul(a842, a843)?
        };
        let a845 = 1u64;
        nat_add(a844, a845)?
    })?;
    nat_mul(a846, a847)
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f39(v0)?;
    let v3: Adt2 = f39(v1)?;
    f40(f24(v0, v1)?, {
        let a902 = {
            let a850 = match v2.clone() {
                Adt2::C0(h848, _, _, _) => {
                    h848
                }
            };
            let a851 = match v3.clone() {
                Adt2::C0(h849, _, _, _) => {
                    h849
                }
            };
            nat_add(a850, a851)?
        };
        let a903 = {
            let a900 = {
                let a898 = {
                    let a852 = 1u64;
                    let a853 = 1u64;
                    nat_add(a852, a853)?
                };
                let a899 = {
                    let a896 = {
                        let a894 = {
                            let a854 = 1u64;
                            let a855 = 1u64;
                            nat_add(a854, a855)?
                        };
                        let a895 = {
                            let a892 = {
                                let a890 = {
                                    let a856 = 1u64;
                                    let a857 = 1u64;
                                    nat_add(a856, a857)?
                                };
                                let a891 = {
                                    let a888 = {
                                        let a886 = {
                                            let a858 = 1u64;
                                            let a859 = 1u64;
                                            nat_add(a858, a859)?
                                        };
                                        let a887 = {
                                            let a884 = {
                                                let a882 = {
                                                    let a860 = 1u64;
                                                    let a861 = 1u64;
                                                    nat_add(a860, a861)?
                                                };
                                                let a883 = {
                                                    let a880 = {
                                                        let a878 = {
                                                            let a862 = 1u64;
                                                            let a863 = 1u64;
                                                            nat_add(a862, a863)?
                                                        };
                                                        let a879 = {
                                                            let a876 = {
                                                                let a874 = {
                                                                    let a864 = 1u64;
                                                                    let a865 = 1u64;
                                                                    nat_add(a864, a865)?
                                                                };
                                                                let a875 = {
                                                                    let a872 = {
                                                                        let a866 = 1u64;
                                                                        let a867 = 1u64;
                                                                        nat_add(a866, a867)?
                                                                    };
                                                                    let a873 = {
                                                                        let a870 = {
                                                                            let a868 = 1u64;
                                                                            let a869 = 1u64;
                                                                            nat_add(a868, a869)?
                                                                        };
                                                                        let a871 = 1u64;
                                                                        nat_mul(a870, a871)?
                                                                    };
                                                                    nat_mul(a872, a873)?
                                                                };
                                                                nat_mul(a874, a875)?
                                                            };
                                                            let a877 = 1u64;
                                                            nat_add(a876, a877)?
                                                        };
                                                        nat_mul(a878, a879)?
                                                    };
                                                    let a881 = 1u64;
                                                    nat_add(a880, a881)?
                                                };
                                                nat_mul(a882, a883)?
                                            };
                                            let a885 = 1u64;
                                            nat_add(a884, a885)?
                                        };
                                        nat_mul(a886, a887)?
                                    };
                                    let a889 = 1u64;
                                    nat_add(a888, a889)?
                                };
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
        nat_sub(a902, a903)
    }, f41({
        let a906 = match v2 {
            Adt2::C0(_, h904, _, _) => {
                h904
            }
        };
        let a907 = match v3 {
            Adt2::C0(_, h905, _, _) => {
                h905
            }
        };
        nat_mul(a906, a907)?
    }, {
        let a924 = {
            let a908 = 1u64;
            let a909 = 1u64;
            nat_add(a908, a909)?
        };
        let a925 = {
            let a922 = {
                let a910 = 1u64;
                let a911 = 1u64;
                nat_add(a910, a911)?
            };
            let a923 = {
                let a920 = {
                    let a918 = {
                        let a912 = 1u64;
                        let a913 = 1u64;
                        nat_add(a912, a913)?
                    };
                    let a919 = {
                        let a916 = {
                            let a914 = 1u64;
                            let a915 = 1u64;
                            nat_add(a914, a915)?
                        };
                        let a917 = 1u64;
                        nat_mul(a916, a917)?
                    };
                    nat_mul(a918, a919)?
                };
                let a921 = 1u64;
                nat_add(a920, a921)?
            };
            nat_mul(a922, a923)?
        };
        nat_mul(a924, a925)?
    })?)
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h926, _) = f42(Fn1::F43, {
        let a945 = {
            let a927 = 1u64;
            let a928 = 1u64;
            nat_add(a927, a928)?
        };
        let a946 = {
            let a943 = {
                let a929 = 1u64;
                let a930 = 1u64;
                nat_add(a929, a930)?
            };
            let a944 = {
                let a941 = {
                    let a931 = 1u64;
                    let a932 = 1u64;
                    nat_add(a931, a932)?
                };
                let a942 = {
                    let a939 = {
                        let a933 = 1u64;
                        let a934 = 1u64;
                        nat_add(a933, a934)?
                    };
                    let a940 = {
                        let a937 = {
                            let a935 = 1u64;
                            let a936 = 1u64;
                            nat_add(a935, a936)?
                        };
                        let a938 = 1u64;
                        nat_mul(a937, a938)?
                    };
                    nat_mul(a939, a940)?
                };
                nat_mul(a941, a942)?
            };
            nat_mul(a943, a944)?
        };
        nat_mul(a945, a946)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h926)
}

pub fn f28(v0: u64) -> R<u64> {
    let a971 = v0;
    let a972 = f22({
        let a969 = {
            let a967 = {
                let a947 = 1u64;
                let a948 = 1u64;
                nat_add(a947, a948)?
            };
            let a968 = {
                let a965 = {
                    let a963 = {
                        let a949 = 1u64;
                        let a950 = 1u64;
                        nat_add(a949, a950)?
                    };
                    let a964 = {
                        let a961 = {
                            let a959 = {
                                let a951 = 1u64;
                                let a952 = 1u64;
                                nat_add(a951, a952)?
                            };
                            let a960 = {
                                let a957 = {
                                    let a955 = {
                                        let a953 = 1u64;
                                        let a954 = 1u64;
                                        nat_add(a953, a954)?
                                    };
                                    let a956 = 1u64;
                                    nat_mul(a955, a956)?
                                };
                                let a958 = 1u64;
                                nat_add(a957, a958)?
                            };
                            nat_mul(a959, a960)?
                        };
                        let a962 = 1u64;
                        nat_add(a961, a962)?
                    };
                    nat_mul(a963, a964)?
                };
                let a966 = 1u64;
                nat_add(a965, a966)?
            };
            nat_mul(a967, a968)?
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    })?;
    let a973 = 0u64;
    Ok(nat_quot(a971, a972, a973))
}

pub fn f29(v0: u64) -> R<u64> {
    let a998 = v0;
    let a999 = f22({
        let a996 = {
            let a994 = {
                let a974 = 1u64;
                let a975 = 1u64;
                nat_add(a974, a975)?
            };
            let a995 = {
                let a992 = {
                    let a990 = {
                        let a976 = 1u64;
                        let a977 = 1u64;
                        nat_add(a976, a977)?
                    };
                    let a991 = {
                        let a988 = {
                            let a986 = {
                                let a978 = 1u64;
                                let a979 = 1u64;
                                nat_add(a978, a979)?
                            };
                            let a987 = {
                                let a984 = {
                                    let a982 = {
                                        let a980 = 1u64;
                                        let a981 = 1u64;
                                        nat_add(a980, a981)?
                                    };
                                    let a983 = 1u64;
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
                    nat_mul(a990, a991)?
                };
                let a993 = 1u64;
                nat_add(a992, a993)?
            };
            nat_mul(a994, a995)?
        };
        let a997 = 1u64;
        nat_add(a996, a997)?
    })?;
    let a1000 = 0u64;
    Ok(nat_rem(a998, a999, a1000))
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f44(v0)?;
    let v3: u64 = {
        let a1013 = f36(v0)?;
        let a1014 = {
            let a1011 = {
                let a1001 = 1u64;
                let a1002 = 1u64;
                nat_add(a1001, a1002)?
            };
            let a1012 = {
                let a1009 = {
                    let a1003 = 1u64;
                    let a1004 = 1u64;
                    nat_add(a1003, a1004)?
                };
                let a1010 = {
                    let a1007 = {
                        let a1005 = 1u64;
                        let a1006 = 1u64;
                        nat_add(a1005, a1006)?
                    };
                    let a1008 = 1u64;
                    nat_mul(a1007, a1008)?
                };
                nat_mul(a1009, a1010)?
            };
            nat_mul(a1011, a1012)?
        };
        nat_mul(a1013, a1014)?
    };
    let v4: u64 = f41({
        let a1027 = f36(v1)?;
        let a1028 = {
            let a1025 = {
                let a1015 = 1u64;
                let a1016 = 1u64;
                nat_add(a1015, a1016)?
            };
            let a1026 = {
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
            nat_mul(a1025, a1026)?
        };
        nat_mul(a1027, a1028)?
    }, {
        let a1029 = v2;
        let a1030 = f44(v1)?;
        nat_sub(a1029, a1030)
    })?;
    let a1031 = f28(v0)?;
    let a1032 = f28(v1)?;
    if nat_eq(a1031, a1032) {
        f40(f28(v0)?, v2, {
            let a1033 = v3;
            let a1034 = v4;
            nat_add(a1033, a1034)?
        })
    } else {
        let a1035 = v3;
        let a1036 = v4;
        if nat_eq(a1035, a1036) {
            Ok(0u64)
        } else {
            f40(f28(v0)?, v2, {
                let a1037 = v3;
                let a1038 = v4;
                nat_sub(a1037, a1038)
            })
        }
    }
}

pub fn f31(v0: bool, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = if v0 {
        let a1046 = {
            let a1044 = 3u64;
            let a1045 = {
                let a1041 = {
                    let a1039 = v1;
                    let a1040 = v2;
                    nat_mul(a1039, a1040)?
                };
                let a1042 = 256u64;
                let a1043 = 0u64;
                nat_rem(a1041, a1042, a1043)
            };
            nat_mul(a1044, a1045)?
        };
        let a1047 = 256u64;
        let a1048 = 0u64;
        nat_quot(a1046, a1047, a1048)
    } else {
        let a1052 = {
            let a1049 = v1;
            let a1050 = v2;
            let a1051 = 0u64;
            nat_quot(a1049, a1050, a1051)
        };
        let a1053 = 4u64;
        let a1054 = 0u64;
        nat_rem(a1052, a1053, a1054)
    };
    let a1057 = {
        let a1055 = v3;
        let a1056 = 255u64;
        nat_add(a1055, a1056)?
    };
    let a1058 = 256u64;
    let a1059 = 0u64;
    Ok(nat_rem(a1057, a1058, a1059))
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    if if if if f19(v0)? {
        true
    } else {
        f19(v1)?
    } {
        true
    } else {
        if f20(v0)? {
            f20(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f21(v0)? {
            f21(v1)?
        } else {
            false
        }
    } {
        let a1146 = {
            let a1124 = {
                let a1100 = {
                    let a1098 = {
                        let a1060 = 1u64;
                        let a1061 = 1u64;
                        nat_add(a1060, a1061)?
                    };
                    let a1099 = {
                        let a1096 = {
                            let a1094 = {
                                let a1062 = 1u64;
                                let a1063 = 1u64;
                                nat_add(a1062, a1063)?
                            };
                            let a1095 = {
                                let a1092 = {
                                    let a1090 = {
                                        let a1064 = 1u64;
                                        let a1065 = 1u64;
                                        nat_add(a1064, a1065)?
                                    };
                                    let a1091 = {
                                        let a1088 = {
                                            let a1086 = {
                                                let a1066 = 1u64;
                                                let a1067 = 1u64;
                                                nat_add(a1066, a1067)?
                                            };
                                            let a1087 = {
                                                let a1084 = {
                                                    let a1082 = {
                                                        let a1068 = 1u64;
                                                        let a1069 = 1u64;
                                                        nat_add(a1068, a1069)?
                                                    };
                                                    let a1083 = {
                                                        let a1080 = {
                                                            let a1078 = {
                                                                let a1070 = 1u64;
                                                                let a1071 = 1u64;
                                                                nat_add(a1070, a1071)?
                                                            };
                                                            let a1079 = {
                                                                let a1076 = {
                                                                    let a1074 = {
                                                                        let a1072 = 1u64;
                                                                        let a1073 = 1u64;
                                                                        nat_add(a1072, a1073)?
                                                                    };
                                                                    let a1075 = 1u64;
                                                                    nat_mul(a1074, a1075)?
                                                                };
                                                                let a1077 = 1u64;
                                                                nat_add(a1076, a1077)?
                                                            };
                                                            nat_mul(a1078, a1079)?
                                                        };
                                                        let a1081 = 1u64;
                                                        nat_add(a1080, a1081)?
                                                    };
                                                    nat_mul(a1082, a1083)?
                                                };
                                                let a1085 = 1u64;
                                                nat_add(a1084, a1085)?
                                            };
                                            nat_mul(a1086, a1087)?
                                        };
                                        let a1089 = 1u64;
                                        nat_add(a1088, a1089)?
                                    };
                                    nat_mul(a1090, a1091)?
                                };
                                let a1093 = 1u64;
                                nat_add(a1092, a1093)?
                            };
                            nat_mul(a1094, a1095)?
                        };
                        let a1097 = 1u64;
                        nat_add(a1096, a1097)?
                    };
                    nat_mul(a1098, a1099)?
                };
                let a1101 = 1u64;
                nat_add(a1100, a1101)?
            };
            let a1125 = f22({
                let a1122 = {
                    let a1120 = {
                        let a1102 = 1u64;
                        let a1103 = 1u64;
                        nat_add(a1102, a1103)?
                    };
                    let a1121 = {
                        let a1118 = {
                            let a1116 = {
                                let a1104 = 1u64;
                                let a1105 = 1u64;
                                nat_add(a1104, a1105)?
                            };
                            let a1117 = {
                                let a1114 = {
                                    let a1112 = {
                                        let a1106 = 1u64;
                                        let a1107 = 1u64;
                                        nat_add(a1106, a1107)?
                                    };
                                    let a1113 = {
                                        let a1110 = {
                                            let a1108 = 1u64;
                                            let a1109 = 1u64;
                                            nat_add(a1108, a1109)?
                                        };
                                        let a1111 = 1u64;
                                        nat_mul(a1110, a1111)?
                                    };
                                    nat_mul(a1112, a1113)?
                                };
                                let a1115 = 1u64;
                                nat_add(a1114, a1115)?
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
            })?;
            nat_mul(a1124, a1125)?
        };
        let a1147 = f22({
            let a1144 = {
                let a1126 = 1u64;
                let a1127 = 1u64;
                nat_add(a1126, a1127)?
            };
            let a1145 = {
                let a1142 = {
                    let a1140 = {
                        let a1128 = 1u64;
                        let a1129 = 1u64;
                        nat_add(a1128, a1129)?
                    };
                    let a1141 = {
                        let a1138 = {
                            let a1136 = {
                                let a1130 = 1u64;
                                let a1131 = 1u64;
                                nat_add(a1130, a1131)?
                            };
                            let a1137 = {
                                let a1134 = {
                                    let a1132 = 1u64;
                                    let a1133 = 1u64;
                                    nat_add(a1132, a1133)?
                                };
                                let a1135 = 1u64;
                                nat_mul(a1134, a1135)?
                            };
                            nat_mul(a1136, a1137)?
                        };
                        let a1139 = 1u64;
                        nat_add(a1138, a1139)?
                    };
                    nat_mul(a1140, a1141)?
                };
                let a1143 = 1u64;
                nat_add(a1142, a1143)?
            };
            nat_mul(a1144, a1145)?
        })?;
        nat_add(a1146, a1147)
    } else {
        if if f20(v0)? {
            true
        } else {
            f21(v1)?
        } {
            f23(f24(v0, v1)?)
        } else {
            if if f20(v1)? {
                true
            } else {
                f21(v0)?
            } {
                f25(f24(v0, v1)?)
            } else {
                f45(v0, v1)
            }
        }
    }
}

pub fn f33(v0: u64) -> R<u64> {
    let a1170 = f29(v0)?;
    let a1171 = f22({
        let a1168 = {
            let a1166 = {
                let a1148 = 1u64;
                let a1149 = 1u64;
                nat_add(a1148, a1149)?
            };
            let a1167 = {
                let a1164 = {
                    let a1162 = {
                        let a1150 = 1u64;
                        let a1151 = 1u64;
                        nat_add(a1150, a1151)?
                    };
                    let a1163 = {
                        let a1160 = {
                            let a1158 = {
                                let a1152 = 1u64;
                                let a1153 = 1u64;
                                nat_add(a1152, a1153)?
                            };
                            let a1159 = {
                                let a1156 = {
                                    let a1154 = 1u64;
                                    let a1155 = 1u64;
                                    nat_add(a1154, a1155)?
                                };
                                let a1157 = 1u64;
                                nat_mul(a1156, a1157)?
                            };
                            nat_mul(a1158, a1159)?
                        };
                        let a1161 = 1u64;
                        nat_add(a1160, a1161)?
                    };
                    nat_mul(a1162, a1163)?
                };
                let a1165 = 1u64;
                nat_add(a1164, a1165)?
            };
            nat_mul(a1166, a1167)?
        };
        let a1169 = 1u64;
        nat_add(a1168, a1169)?
    })?;
    let a1172 = 0u64;
    Ok(nat_quot(a1170, a1171, a1172))
}

pub fn f34(v0: u64) -> R<u64> {
    let a1195 = v0;
    let a1196 = f22({
        let a1193 = {
            let a1191 = {
                let a1173 = 1u64;
                let a1174 = 1u64;
                nat_add(a1173, a1174)?
            };
            let a1192 = {
                let a1189 = {
                    let a1187 = {
                        let a1175 = 1u64;
                        let a1176 = 1u64;
                        nat_add(a1175, a1176)?
                    };
                    let a1188 = {
                        let a1185 = {
                            let a1183 = {
                                let a1177 = 1u64;
                                let a1178 = 1u64;
                                nat_add(a1177, a1178)?
                            };
                            let a1184 = {
                                let a1181 = {
                                    let a1179 = 1u64;
                                    let a1180 = 1u64;
                                    nat_add(a1179, a1180)?
                                };
                                let a1182 = 1u64;
                                nat_mul(a1181, a1182)?
                            };
                            nat_mul(a1183, a1184)?
                        };
                        let a1186 = 1u64;
                        nat_add(a1185, a1186)?
                    };
                    nat_mul(a1187, a1188)?
                };
                let a1190 = 1u64;
                nat_add(a1189, a1190)?
            };
            nat_mul(a1191, a1192)?
        };
        let a1194 = 1u64;
        nat_add(a1193, a1194)?
    })?;
    let a1197 = 0u64;
    Ok(nat_rem(a1195, a1196, a1197))
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1198 = v0;
        let a1199 = v1;
        let a1200 = 0u64;
        nat_quot(a1198, a1199, a1200)
    };
    let v3: u64 = {
        let a1204 = {
            let a1201 = v0;
            let a1202 = v1;
            let a1203 = 0u64;
            nat_rem(a1201, a1202, a1203)
        };
        let a1205 = 2u64;
        nat_mul(a1204, a1205)?
    };
    let a1215 = v2;
    let a1216 = {
        let a1206 = v1;
        let a1207 = v3;
        if if nat_lt(a1206, a1207) {
            true
        } else {
            let a1208 = v3;
            let a1209 = v1;
            if nat_eq(a1208, a1209) {
                let a1213 = {
                    let a1210 = v2;
                    let a1211 = 2u64;
                    let a1212 = 0u64;
                    nat_rem(a1210, a1211, a1212)
                };
                let a1214 = 1u64;
                nat_eq(a1213, a1214)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1215, a1216)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1241 = f34(v0)?;
    let a1242 = {
        let a1217 = f33(v0)?;
        let a1218 = 0u64;
        if nat_eq(a1217, a1218) {
            0u64
        } else {
            f22({
                let a1239 = {
                    let a1237 = {
                        let a1219 = 1u64;
                        let a1220 = 1u64;
                        nat_add(a1219, a1220)?
                    };
                    let a1238 = {
                        let a1235 = {
                            let a1233 = {
                                let a1221 = 1u64;
                                let a1222 = 1u64;
                                nat_add(a1221, a1222)?
                            };
                            let a1234 = {
                                let a1231 = {
                                    let a1229 = {
                                        let a1223 = 1u64;
                                        let a1224 = 1u64;
                                        nat_add(a1223, a1224)?
                                    };
                                    let a1230 = {
                                        let a1227 = {
                                            let a1225 = 1u64;
                                            let a1226 = 1u64;
                                            nat_add(a1225, a1226)?
                                        };
                                        let a1228 = 1u64;
                                        nat_mul(a1227, a1228)?
                                    };
                                    nat_mul(a1229, a1230)?
                                };
                                let a1232 = 1u64;
                                nat_add(a1231, a1232)?
                            };
                            nat_mul(a1233, a1234)?
                        };
                        let a1236 = 1u64;
                        nat_add(a1235, a1236)?
                    };
                    nat_mul(a1237, a1238)?
                };
                let a1240 = 1u64;
                nat_add(a1239, a1240)?
            })?
        }
    };
    nat_add(a1241, a1242)
}

pub fn f37(v0: Fn0, p1246: u64, p1247: u64) -> R<u64> {
    let m1245 = (p1246, p1247);
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let (v1, v2) = m1245;
                let m1243 = v1;
                if m1243 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m1243 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c1244: Fn0 = v0.clone();
                        c1244.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: u64) -> R<u64> {
    let a1248 = v0;
    let a1249 = v0;
    nat_add(a1248, a1249)
}

pub fn f39(v0: u64) -> R<Adt2> {
    f27(f44(v0)?, f36(v0)?, 0u64, f22({
        let a1270 = {
            let a1268 = {
                let a1250 = 1u64;
                let a1251 = 1u64;
                nat_add(a1250, a1251)?
            };
            let a1269 = {
                let a1266 = {
                    let a1264 = {
                        let a1252 = 1u64;
                        let a1253 = 1u64;
                        nat_add(a1252, a1253)?
                    };
                    let a1265 = {
                        let a1262 = {
                            let a1260 = {
                                let a1254 = 1u64;
                                let a1255 = 1u64;
                                nat_add(a1254, a1255)?
                            };
                            let a1261 = {
                                let a1258 = {
                                    let a1256 = 1u64;
                                    let a1257 = 1u64;
                                    nat_add(a1256, a1257)?
                                };
                                let a1259 = 1u64;
                                nat_mul(a1258, a1259)?
                            };
                            nat_mul(a1260, a1261)?
                        };
                        let a1263 = 1u64;
                        nat_add(a1262, a1263)?
                    };
                    nat_mul(a1264, a1265)?
                };
                let a1267 = 1u64;
                nat_add(a1266, a1267)?
            };
            nat_mul(a1268, a1269)?
        };
        let a1271 = 1u64;
        nat_add(a1270, a1271)?
    })?)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1294 = f22({
        let a1292 = {
            let a1290 = {
                let a1272 = 1u64;
                let a1273 = 1u64;
                nat_add(a1272, a1273)?
            };
            let a1291 = {
                let a1288 = {
                    let a1286 = {
                        let a1274 = 1u64;
                        let a1275 = 1u64;
                        nat_add(a1274, a1275)?
                    };
                    let a1287 = {
                        let a1284 = {
                            let a1276 = 1u64;
                            let a1277 = 1u64;
                            nat_add(a1276, a1277)?
                        };
                        let a1285 = {
                            let a1282 = {
                                let a1280 = {
                                    let a1278 = 1u64;
                                    let a1279 = 1u64;
                                    nat_add(a1278, a1279)?
                                };
                                let a1281 = 1u64;
                                nat_mul(a1280, a1281)?
                            };
                            let a1283 = 1u64;
                            nat_add(a1282, a1283)?
                        };
                        nat_mul(a1284, a1285)?
                    };
                    nat_mul(a1286, a1287)?
                };
                let a1289 = 1u64;
                nat_add(a1288, a1289)?
            };
            nat_mul(a1290, a1291)?
        };
        let a1293 = 1u64;
        nat_add(a1292, a1293)?
    })?;
    let a1295 = v2;
    if nat_le(a1294, a1295) {
        f46(v0, {
            let a1296 = v1;
            let a1297 = 1u64;
            nat_add(a1296, a1297)?
        }, f41(v2, 1u64)?)
    } else {
        let v3: Adt2 = f27(v1, v2, {
            let a1334 = {
                let a1332 = {
                    let a1298 = 1u64;
                    let a1299 = 1u64;
                    nat_add(a1298, a1299)?
                };
                let a1333 = {
                    let a1330 = {
                        let a1300 = 1u64;
                        let a1301 = 1u64;
                        nat_add(a1300, a1301)?
                    };
                    let a1331 = {
                        let a1328 = {
                            let a1302 = 1u64;
                            let a1303 = 1u64;
                            nat_add(a1302, a1303)?
                        };
                        let a1329 = {
                            let a1326 = {
                                let a1304 = 1u64;
                                let a1305 = 1u64;
                                nat_add(a1304, a1305)?
                            };
                            let a1327 = {
                                let a1324 = {
                                    let a1306 = 1u64;
                                    let a1307 = 1u64;
                                    nat_add(a1306, a1307)?
                                };
                                let a1325 = {
                                    let a1322 = {
                                        let a1308 = 1u64;
                                        let a1309 = 1u64;
                                        nat_add(a1308, a1309)?
                                    };
                                    let a1323 = {
                                        let a1320 = {
                                            let a1310 = 1u64;
                                            let a1311 = 1u64;
                                            nat_add(a1310, a1311)?
                                        };
                                        let a1321 = {
                                            let a1318 = {
                                                let a1312 = 1u64;
                                                let a1313 = 1u64;
                                                nat_add(a1312, a1313)?
                                            };
                                            let a1319 = {
                                                let a1316 = {
                                                    let a1314 = 1u64;
                                                    let a1315 = 1u64;
                                                    nat_add(a1314, a1315)?
                                                };
                                                let a1317 = 1u64;
                                                nat_mul(a1316, a1317)?
                                            };
                                            nat_mul(a1318, a1319)?
                                        };
                                        nat_mul(a1320, a1321)?
                                    };
                                    nat_mul(a1322, a1323)?
                                };
                                nat_mul(a1324, a1325)?
                            };
                            nat_mul(a1326, a1327)?
                        };
                        nat_mul(a1328, a1329)?
                    };
                    nat_mul(a1330, a1331)?
                };
                nat_mul(a1332, a1333)?
            };
            let a1335 = 1u64;
            nat_add(a1334, a1335)?
        }, f22({
            let a1354 = {
                let a1336 = 1u64;
                let a1337 = 1u64;
                nat_add(a1336, a1337)?
            };
            let a1355 = {
                let a1352 = {
                    let a1350 = {
                        let a1338 = 1u64;
                        let a1339 = 1u64;
                        nat_add(a1338, a1339)?
                    };
                    let a1351 = {
                        let a1348 = {
                            let a1340 = 1u64;
                            let a1341 = 1u64;
                            nat_add(a1340, a1341)?
                        };
                        let a1349 = {
                            let a1346 = {
                                let a1344 = {
                                    let a1342 = 1u64;
                                    let a1343 = 1u64;
                                    nat_add(a1342, a1343)?
                                };
                                let a1345 = 1u64;
                                nat_mul(a1344, a1345)?
                            };
                            let a1347 = 1u64;
                            nat_add(a1346, a1347)?
                        };
                        nat_mul(a1348, a1349)?
                    };
                    nat_mul(a1350, a1351)?
                };
                let a1353 = 1u64;
                nat_add(a1352, a1353)?
            };
            nat_mul(a1354, a1355)?
        })?)?;
        f46(v0, match v3.clone() {
            Adt2::C0(h1356, _, _, _) => {
                h1356
            }
        }, match v3 {
            Adt2::C0(_, h1357, _, _) => {
                h1357
            }
        })
    }
}

pub fn f41(v0: u64, v1: u64) -> R<u64> {
    let a1358 = v1;
    let a1359 = 0u64;
    if nat_eq(a1358, a1359) {
        Ok(v0)
    } else {
        let a1384 = {
            let a1382 = {
                let a1360 = 1u64;
                let a1361 = 1u64;
                nat_add(a1360, a1361)?
            };
            let a1383 = {
                let a1380 = {
                    let a1362 = 1u64;
                    let a1363 = 1u64;
                    nat_add(a1362, a1363)?
                };
                let a1381 = {
                    let a1378 = {
                        let a1364 = 1u64;
                        let a1365 = 1u64;
                        nat_add(a1364, a1365)?
                    };
                    let a1379 = {
                        let a1376 = {
                            let a1366 = 1u64;
                            let a1367 = 1u64;
                            nat_add(a1366, a1367)?
                        };
                        let a1377 = {
                            let a1374 = {
                                let a1368 = 1u64;
                                let a1369 = 1u64;
                                nat_add(a1368, a1369)?
                            };
                            let a1375 = {
                                let a1372 = {
                                    let a1370 = 1u64;
                                    let a1371 = 1u64;
                                    nat_add(a1370, a1371)?
                                };
                                let a1373 = 1u64;
                                nat_mul(a1372, a1373)?
                            };
                            nat_mul(a1374, a1375)?
                        };
                        nat_mul(a1376, a1377)?
                    };
                    nat_mul(a1378, a1379)?
                };
                nat_mul(a1380, a1381)?
            };
            nat_mul(a1382, a1383)?
        };
        let a1385 = v1;
        if nat_le(a1384, a1385) {
            let a1386 = v0;
            let a1387 = 0u64;
            if nat_eq(a1386, a1387) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f22(v1)?;
            f47({
                let a1388 = v0;
                let a1389 = v2;
                let a1390 = 0u64;
                nat_quot(a1388, a1389, a1390)
            }, {
                let a1394 = 0u64;
                let a1395 = {
                    let a1391 = v0;
                    let a1392 = v2;
                    let a1393 = 0u64;
                    nat_rem(a1391, a1392, a1393)
                };
                nat_lt(a1394, a1395)
            })
        }
    }
}

pub fn f42(v0: Fn1, p1400: u64, p1401: Adt2) -> R<(Adt2, bool)> {
    let m1399 = (p1400, p1401);
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let (v1, v2) = m1399;
                let m1396 = v1;
                if m1396 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1396 - 1;
                    let m1398 = {
                        let c1397: Fn1 = v0.clone();
                        c1397.apply(v2.clone())?
                    };
                    match m1398 {
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
                Ok(next) => { m1399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt2) -> R<Option<Adt2>> {
    f48(v0)
}

pub fn f44(v0: u64) -> R<u64> {
    let a1440 = {
        let a1402 = f33(v0)?;
        let a1403 = 0u64;
        if nat_eq(a1402, a1403) {
            1u64
        } else {
            f33(v0)?
        }
    };
    let a1441 = {
        let a1438 = {
            let a1404 = 1u64;
            let a1405 = 1u64;
            nat_add(a1404, a1405)?
        };
        let a1439 = {
            let a1436 = {
                let a1406 = 1u64;
                let a1407 = 1u64;
                nat_add(a1406, a1407)?
            };
            let a1437 = {
                let a1434 = {
                    let a1408 = 1u64;
                    let a1409 = 1u64;
                    nat_add(a1408, a1409)?
                };
                let a1435 = {
                    let a1432 = {
                        let a1410 = 1u64;
                        let a1411 = 1u64;
                        nat_add(a1410, a1411)?
                    };
                    let a1433 = {
                        let a1430 = {
                            let a1412 = 1u64;
                            let a1413 = 1u64;
                            nat_add(a1412, a1413)?
                        };
                        let a1431 = {
                            let a1428 = {
                                let a1414 = 1u64;
                                let a1415 = 1u64;
                                nat_add(a1414, a1415)?
                            };
                            let a1429 = {
                                let a1426 = {
                                    let a1416 = 1u64;
                                    let a1417 = 1u64;
                                    nat_add(a1416, a1417)?
                                };
                                let a1427 = {
                                    let a1424 = {
                                        let a1418 = 1u64;
                                        let a1419 = 1u64;
                                        nat_add(a1418, a1419)?
                                    };
                                    let a1425 = {
                                        let a1422 = {
                                            let a1420 = 1u64;
                                            let a1421 = 1u64;
                                            nat_add(a1420, a1421)?
                                        };
                                        let a1423 = 1u64;
                                        nat_mul(a1422, a1423)?
                                    };
                                    nat_mul(a1424, a1425)?
                                };
                                nat_mul(a1426, a1427)?
                            };
                            nat_mul(a1428, a1429)?
                        };
                        nat_mul(a1430, a1431)?
                    };
                    nat_mul(a1432, a1433)?
                };
                nat_mul(a1434, a1435)?
            };
            nat_mul(a1436, a1437)?
        };
        nat_mul(a1438, a1439)?
    };
    nat_add(a1440, a1441)
}

pub fn f45(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f39(v0)?;
    let v3: Adt2 = f39(v1)?;
    let v4: bool = {
        let a1444 = match v2.clone() {
            Adt2::C0(_, h1442, _, _) => {
                h1442
            }
        };
        let a1445 = match v3.clone() {
            Adt2::C0(_, h1443, _, _) => {
                h1443
            }
        };
        nat_lt(a1444, a1445)
    };
    let v5: u64 = {
        let a1489 = match v2.clone() {
            Adt2::C0(_, h1446, _, _) => {
                h1446
            }
        };
        let a1490 = f22(if v4 {
            let a1467 = {
                let a1465 = {
                    let a1447 = 1u64;
                    let a1448 = 1u64;
                    nat_add(a1447, a1448)?
                };
                let a1466 = {
                    let a1463 = {
                        let a1461 = {
                            let a1449 = 1u64;
                            let a1450 = 1u64;
                            nat_add(a1449, a1450)?
                        };
                        let a1462 = {
                            let a1459 = {
                                let a1451 = 1u64;
                                let a1452 = 1u64;
                                nat_add(a1451, a1452)?
                            };
                            let a1460 = {
                                let a1457 = {
                                    let a1455 = {
                                        let a1453 = 1u64;
                                        let a1454 = 1u64;
                                        nat_add(a1453, a1454)?
                                    };
                                    let a1456 = 1u64;
                                    nat_mul(a1455, a1456)?
                                };
                                let a1458 = 1u64;
                                nat_add(a1457, a1458)?
                            };
                            nat_mul(a1459, a1460)?
                        };
                        nat_mul(a1461, a1462)?
                    };
                    let a1464 = 1u64;
                    nat_add(a1463, a1464)?
                };
                nat_mul(a1465, a1466)?
            };
            let a1468 = 1u64;
            nat_add(a1467, a1468)?
        } else {
            let a1487 = {
                let a1469 = 1u64;
                let a1470 = 1u64;
                nat_add(a1469, a1470)?
            };
            let a1488 = {
                let a1485 = {
                    let a1483 = {
                        let a1471 = 1u64;
                        let a1472 = 1u64;
                        nat_add(a1471, a1472)?
                    };
                    let a1484 = {
                        let a1481 = {
                            let a1473 = 1u64;
                            let a1474 = 1u64;
                            nat_add(a1473, a1474)?
                        };
                        let a1482 = {
                            let a1479 = {
                                let a1477 = {
                                    let a1475 = 1u64;
                                    let a1476 = 1u64;
                                    nat_add(a1475, a1476)?
                                };
                                let a1478 = 1u64;
                                nat_mul(a1477, a1478)?
                            };
                            let a1480 = 1u64;
                            nat_add(a1479, a1480)?
                        };
                        nat_mul(a1481, a1482)?
                    };
                    nat_mul(a1483, a1484)?
                };
                let a1486 = 1u64;
                nat_add(a1485, a1486)?
            };
            nat_mul(a1487, a1488)?
        })?;
        nat_mul(a1489, a1490)?
    };
    let v6: u64 = {
        let a1547 = {
            let a1545 = {
                let a1542 = match v2 {
                    Adt2::C0(h1491, _, _, _) => {
                        h1491
                    }
                };
                let a1543 = {
                    let a1540 = {
                        let a1538 = {
                            let a1492 = 1u64;
                            let a1493 = 1u64;
                            nat_add(a1492, a1493)?
                        };
                        let a1539 = {
                            let a1536 = {
                                let a1534 = {
                                    let a1494 = 1u64;
                                    let a1495 = 1u64;
                                    nat_add(a1494, a1495)?
                                };
                                let a1535 = {
                                    let a1532 = {
                                        let a1530 = {
                                            let a1496 = 1u64;
                                            let a1497 = 1u64;
                                            nat_add(a1496, a1497)?
                                        };
                                        let a1531 = {
                                            let a1528 = {
                                                let a1526 = {
                                                    let a1498 = 1u64;
                                                    let a1499 = 1u64;
                                                    nat_add(a1498, a1499)?
                                                };
                                                let a1527 = {
                                                    let a1524 = {
                                                        let a1522 = {
                                                            let a1500 = 1u64;
                                                            let a1501 = 1u64;
                                                            nat_add(a1500, a1501)?
                                                        };
                                                        let a1523 = {
                                                            let a1520 = {
                                                                let a1518 = {
                                                                    let a1502 = 1u64;
                                                                    let a1503 = 1u64;
                                                                    nat_add(a1502, a1503)?
                                                                };
                                                                let a1519 = {
                                                                    let a1516 = {
                                                                        let a1514 = {
                                                                            let a1504 = 1u64;
                                                                            let a1505 = 1u64;
                                                                            nat_add(a1504, a1505)?
                                                                        };
                                                                        let a1515 = {
                                                                            let a1512 = {
                                                                                let a1506 = 1u64;
                                                                                let a1507 = 1u64;
                                                                                nat_add(a1506, a1507)?
                                                                            };
                                                                            let a1513 = {
                                                                                let a1510 = {
                                                                                    let a1508 = 1u64;
                                                                                    let a1509 = 1u64;
                                                                                    nat_add(a1508, a1509)?
                                                                                };
                                                                                let a1511 = 1u64;
                                                                                nat_mul(a1510, a1511)?
                                                                            };
                                                                            nat_mul(a1512, a1513)?
                                                                        };
                                                                        nat_mul(a1514, a1515)?
                                                                    };
                                                                    let a1517 = 1u64;
                                                                    nat_add(a1516, a1517)?
                                                                };
                                                                nat_mul(a1518, a1519)?
                                                            };
                                                            let a1521 = 1u64;
                                                            nat_add(a1520, a1521)?
                                                        };
                                                        nat_mul(a1522, a1523)?
                                                    };
                                                    let a1525 = 1u64;
                                                    nat_add(a1524, a1525)?
                                                };
                                                nat_mul(a1526, a1527)?
                                            };
                                            let a1529 = 1u64;
                                            nat_add(a1528, a1529)?
                                        };
                                        nat_mul(a1530, a1531)?
                                    };
                                    let a1533 = 1u64;
                                    nat_add(a1532, a1533)?
                                };
                                nat_mul(a1534, a1535)?
                            };
                            let a1537 = 1u64;
                            nat_add(a1536, a1537)?
                        };
                        nat_mul(a1538, a1539)?
                    };
                    let a1541 = 1u64;
                    nat_add(a1540, a1541)?
                };
                nat_add(a1542, a1543)?
            };
            let a1546 = match v3.clone() {
                Adt2::C0(h1544, _, _, _) => {
                    h1544
                }
            };
            nat_sub(a1545, a1546)
        };
        let a1548 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a1547, a1548)
    };
    let v7: u64 = {
        let a1550 = v5;
        let a1551 = match v3.clone() {
            Adt2::C0(_, h1549, _, _) => {
                h1549
            }
        };
        let a1552 = 0u64;
        nat_quot(a1550, a1551, a1552)
    };
    let v8: bool = {
        let a1557 = 0u64;
        let a1558 = {
            let a1554 = v5;
            let a1555 = match v3 {
                Adt2::C0(_, h1553, _, _) => {
                    h1553
                }
            };
            let a1556 = 0u64;
            nat_rem(a1554, a1555, a1556)
        };
        nat_lt(a1557, a1558)
    };
    f40(f24(v0, v1)?, v6, f47(v7, v8)?)
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1597 = v1;
    let a1598 = {
        let a1595 = {
            let a1593 = {
                let a1559 = 1u64;
                let a1560 = 1u64;
                nat_add(a1559, a1560)?
            };
            let a1594 = {
                let a1591 = {
                    let a1561 = 1u64;
                    let a1562 = 1u64;
                    nat_add(a1561, a1562)?
                };
                let a1592 = {
                    let a1589 = {
                        let a1563 = 1u64;
                        let a1564 = 1u64;
                        nat_add(a1563, a1564)?
                    };
                    let a1590 = {
                        let a1587 = {
                            let a1565 = 1u64;
                            let a1566 = 1u64;
                            nat_add(a1565, a1566)?
                        };
                        let a1588 = {
                            let a1585 = {
                                let a1567 = 1u64;
                                let a1568 = 1u64;
                                nat_add(a1567, a1568)?
                            };
                            let a1586 = {
                                let a1583 = {
                                    let a1569 = 1u64;
                                    let a1570 = 1u64;
                                    nat_add(a1569, a1570)?
                                };
                                let a1584 = {
                                    let a1581 = {
                                        let a1571 = 1u64;
                                        let a1572 = 1u64;
                                        nat_add(a1571, a1572)?
                                    };
                                    let a1582 = {
                                        let a1579 = {
                                            let a1573 = 1u64;
                                            let a1574 = 1u64;
                                            nat_add(a1573, a1574)?
                                        };
                                        let a1580 = {
                                            let a1577 = {
                                                let a1575 = 1u64;
                                                let a1576 = 1u64;
                                                nat_add(a1575, a1576)?
                                            };
                                            let a1578 = 1u64;
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
                        nat_mul(a1587, a1588)?
                    };
                    nat_mul(a1589, a1590)?
                };
                nat_mul(a1591, a1592)?
            };
            nat_mul(a1593, a1594)?
        };
        let a1596 = 1u64;
        nat_add(a1595, a1596)?
    };
    if nat_lt(a1597, a1598) {
        f49(v0, {
            let a1635 = {
                let a1633 = {
                    let a1599 = 1u64;
                    let a1600 = 1u64;
                    nat_add(a1599, a1600)?
                };
                let a1634 = {
                    let a1631 = {
                        let a1601 = 1u64;
                        let a1602 = 1u64;
                        nat_add(a1601, a1602)?
                    };
                    let a1632 = {
                        let a1629 = {
                            let a1603 = 1u64;
                            let a1604 = 1u64;
                            nat_add(a1603, a1604)?
                        };
                        let a1630 = {
                            let a1627 = {
                                let a1605 = 1u64;
                                let a1606 = 1u64;
                                nat_add(a1605, a1606)?
                            };
                            let a1628 = {
                                let a1625 = {
                                    let a1607 = 1u64;
                                    let a1608 = 1u64;
                                    nat_add(a1607, a1608)?
                                };
                                let a1626 = {
                                    let a1623 = {
                                        let a1609 = 1u64;
                                        let a1610 = 1u64;
                                        nat_add(a1609, a1610)?
                                    };
                                    let a1624 = {
                                        let a1621 = {
                                            let a1611 = 1u64;
                                            let a1612 = 1u64;
                                            nat_add(a1611, a1612)?
                                        };
                                        let a1622 = {
                                            let a1619 = {
                                                let a1613 = 1u64;
                                                let a1614 = 1u64;
                                                nat_add(a1613, a1614)?
                                            };
                                            let a1620 = {
                                                let a1617 = {
                                                    let a1615 = 1u64;
                                                    let a1616 = 1u64;
                                                    nat_add(a1615, a1616)?
                                                };
                                                let a1618 = 1u64;
                                                nat_mul(a1617, a1618)?
                                            };
                                            nat_mul(a1619, a1620)?
                                        };
                                        nat_mul(a1621, a1622)?
                                    };
                                    nat_mul(a1623, a1624)?
                                };
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
            let a1636 = 1u64;
            nat_add(a1635, a1636)?
        }, f41(v2, {
            let a1675 = {
                let a1673 = {
                    let a1671 = {
                        let a1637 = 1u64;
                        let a1638 = 1u64;
                        nat_add(a1637, a1638)?
                    };
                    let a1672 = {
                        let a1669 = {
                            let a1639 = 1u64;
                            let a1640 = 1u64;
                            nat_add(a1639, a1640)?
                        };
                        let a1670 = {
                            let a1667 = {
                                let a1641 = 1u64;
                                let a1642 = 1u64;
                                nat_add(a1641, a1642)?
                            };
                            let a1668 = {
                                let a1665 = {
                                    let a1643 = 1u64;
                                    let a1644 = 1u64;
                                    nat_add(a1643, a1644)?
                                };
                                let a1666 = {
                                    let a1663 = {
                                        let a1645 = 1u64;
                                        let a1646 = 1u64;
                                        nat_add(a1645, a1646)?
                                    };
                                    let a1664 = {
                                        let a1661 = {
                                            let a1647 = 1u64;
                                            let a1648 = 1u64;
                                            nat_add(a1647, a1648)?
                                        };
                                        let a1662 = {
                                            let a1659 = {
                                                let a1649 = 1u64;
                                                let a1650 = 1u64;
                                                nat_add(a1649, a1650)?
                                            };
                                            let a1660 = {
                                                let a1657 = {
                                                    let a1651 = 1u64;
                                                    let a1652 = 1u64;
                                                    nat_add(a1651, a1652)?
                                                };
                                                let a1658 = {
                                                    let a1655 = {
                                                        let a1653 = 1u64;
                                                        let a1654 = 1u64;
                                                        nat_add(a1653, a1654)?
                                                    };
                                                    let a1656 = 1u64;
                                                    nat_mul(a1655, a1656)?
                                                };
                                                nat_mul(a1657, a1658)?
                                            };
                                            nat_mul(a1659, a1660)?
                                        };
                                        nat_mul(a1661, a1662)?
                                    };
                                    nat_mul(a1663, a1664)?
                                };
                                nat_mul(a1665, a1666)?
                            };
                            nat_mul(a1667, a1668)?
                        };
                        nat_mul(a1669, a1670)?
                    };
                    nat_mul(a1671, a1672)?
                };
                let a1674 = 1u64;
                nat_add(a1673, a1674)?
            };
            let a1676 = v1;
            nat_sub(a1675, a1676)
        })?)
    } else {
        f49(v0, v1, v2)
    }
}

pub fn f47(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1684 = {
            let a1681 = v0;
            let a1682 = {
                let a1679 = {
                    let a1677 = 1u64;
                    let a1678 = 1u64;
                    nat_add(a1677, a1678)?
                };
                let a1680 = 1u64;
                nat_mul(a1679, a1680)?
            };
            let a1683 = 0u64;
            nat_rem(a1681, a1682, a1683)
        };
        let a1685 = 0u64;
        nat_eq(a1684, a1685)
    } else {
        false
    } {
        let a1686 = v0;
        let a1687 = 1u64;
        nat_add(a1686, a1687)
    } else {
        Ok(v0)
    }
}

pub fn f48(v0: Adt2) -> R<Option<Adt2>> {
    let a1689 = 0u64;
    let a1690 = match v0.clone() {
        Adt2::C0(_, h1688, _, _) => {
            h1688
        }
    };
    if if if nat_lt(a1689, a1690) {
        let a1693 = match v0.clone() {
            Adt2::C0(_, h1691, _, _) => {
                h1691
            }
        };
        let a1694 = match v0.clone() {
            Adt2::C0(_, _, _, h1692) => {
                h1692
            }
        };
        nat_lt(a1693, a1694)
    } else {
        false
    } {
        let a1697 = match v0.clone() {
            Adt2::C0(_, _, h1695, _) => {
                h1695
            }
        };
        let a1698 = match v0.clone() {
            Adt2::C0(h1696, _, _, _) => {
                h1696
            }
        };
        nat_lt(a1697, a1698)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1700 = match v0.clone() {
                Adt2::C0(h1699, _, _, _) => {
                    h1699
                }
            };
            let a1701 = 1u64;
            nat_sub(a1700, a1701)
        }, {
            let a1704 = match v0.clone() {
                Adt2::C0(_, h1702, _, _) => {
                    h1702
                }
            };
            let a1705 = match v0.clone() {
                Adt2::C0(_, h1703, _, _) => {
                    h1703
                }
            };
            nat_add(a1704, a1705)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1706, _) => {
                h1706
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1707) => {
                h1707
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1720 = v2;
        let a1721 = {
            let a1718 = {
                let a1708 = 1u64;
                let a1709 = 1u64;
                nat_add(a1708, a1709)?
            };
            let a1719 = {
                let a1716 = {
                    let a1710 = 1u64;
                    let a1711 = 1u64;
                    nat_add(a1710, a1711)?
                };
                let a1717 = {
                    let a1714 = {
                        let a1712 = 1u64;
                        let a1713 = 1u64;
                        nat_add(a1712, a1713)?
                    };
                    let a1715 = 1u64;
                    nat_mul(a1714, a1715)?
                };
                nat_mul(a1716, a1717)?
            };
            nat_mul(a1718, a1719)?
        };
        let a1722 = 0u64;
        nat_rem(a1720, a1721, a1722)
    };
    let v4: u64 = {
        let a1735 = v2;
        let a1736 = {
            let a1733 = {
                let a1723 = 1u64;
                let a1724 = 1u64;
                nat_add(a1723, a1724)?
            };
            let a1734 = {
                let a1731 = {
                    let a1725 = 1u64;
                    let a1726 = 1u64;
                    nat_add(a1725, a1726)?
                };
                let a1732 = {
                    let a1729 = {
                        let a1727 = 1u64;
                        let a1728 = 1u64;
                        nat_add(a1727, a1728)?
                    };
                    let a1730 = 1u64;
                    nat_mul(a1729, a1730)?
                };
                nat_mul(a1731, a1732)?
            };
            nat_mul(a1733, a1734)?
        };
        let a1737 = 0u64;
        nat_quot(a1735, a1736, a1737)
    };
    let v5: bool = {
        let a1746 = {
            let a1744 = {
                let a1738 = 1u64;
                let a1739 = 1u64;
                nat_add(a1738, a1739)?
            };
            let a1745 = {
                let a1742 = {
                    let a1740 = 1u64;
                    let a1741 = 1u64;
                    nat_add(a1740, a1741)?
                };
                let a1743 = 1u64;
                nat_mul(a1742, a1743)?
            };
            nat_mul(a1744, a1745)?
        };
        let a1747 = v3;
        if nat_lt(a1746, a1747) {
            true
        } else {
            let a1756 = v3;
            let a1757 = {
                let a1754 = {
                    let a1748 = 1u64;
                    let a1749 = 1u64;
                    nat_add(a1748, a1749)?
                };
                let a1755 = {
                    let a1752 = {
                        let a1750 = 1u64;
                        let a1751 = 1u64;
                        nat_add(a1750, a1751)?
                    };
                    let a1753 = 1u64;
                    nat_mul(a1752, a1753)?
                };
                nat_mul(a1754, a1755)?
            };
            if nat_eq(a1756, a1757) {
                let a1765 = {
                    let a1762 = v4;
                    let a1763 = {
                        let a1760 = {
                            let a1758 = 1u64;
                            let a1759 = 1u64;
                            nat_add(a1758, a1759)?
                        };
                        let a1761 = 1u64;
                        nat_mul(a1760, a1761)?
                    };
                    let a1764 = 0u64;
                    nat_rem(a1762, a1763, a1764)
                };
                let a1766 = 1u64;
                nat_eq(a1765, a1766)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1767 = v4;
        let a1768 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1767, a1768)?
    };
    let v7: bool = {
        let a1797 = {
            let a1795 = {
                let a1771 = {
                    let a1769 = 1u64;
                    let a1770 = 1u64;
                    nat_add(a1769, a1770)?
                };
                let a1772 = 1u64;
                nat_mul(a1771, a1772)?
            };
            let a1796 = f22({
                let a1793 = {
                    let a1791 = {
                        let a1773 = 1u64;
                        let a1774 = 1u64;
                        nat_add(a1773, a1774)?
                    };
                    let a1792 = {
                        let a1789 = {
                            let a1787 = {
                                let a1775 = 1u64;
                                let a1776 = 1u64;
                                nat_add(a1775, a1776)?
                            };
                            let a1788 = {
                                let a1785 = {
                                    let a1783 = {
                                        let a1777 = 1u64;
                                        let a1778 = 1u64;
                                        nat_add(a1777, a1778)?
                                    };
                                    let a1784 = {
                                        let a1781 = {
                                            let a1779 = 1u64;
                                            let a1780 = 1u64;
                                            nat_add(a1779, a1780)?
                                        };
                                        let a1782 = 1u64;
                                        nat_mul(a1781, a1782)?
                                    };
                                    nat_mul(a1783, a1784)?
                                };
                                let a1786 = 1u64;
                                nat_add(a1785, a1786)?
                            };
                            nat_mul(a1787, a1788)?
                        };
                        let a1790 = 1u64;
                        nat_add(a1789, a1790)?
                    };
                    nat_mul(a1791, a1792)?
                };
                let a1794 = 1u64;
                nat_add(a1793, a1794)?
            })?;
            nat_mul(a1795, a1796)?
        };
        let a1798 = v6;
        nat_le(a1797, a1798)
    };
    let v8: u64 = if v7 {
        let a1803 = v6;
        let a1804 = {
            let a1801 = {
                let a1799 = 1u64;
                let a1800 = 1u64;
                nat_add(a1799, a1800)?
            };
            let a1802 = 1u64;
            nat_mul(a1801, a1802)?
        };
        let a1805 = 0u64;
        nat_quot(a1803, a1804, a1805)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1806 = v1;
        let a1807 = 1u64;
        nat_add(a1806, a1807)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1846 = v9;
        let a1847 = {
            let a1844 = {
                let a1842 = {
                    let a1808 = 1u64;
                    let a1809 = 1u64;
                    nat_add(a1808, a1809)?
                };
                let a1843 = {
                    let a1840 = {
                        let a1810 = 1u64;
                        let a1811 = 1u64;
                        nat_add(a1810, a1811)?
                    };
                    let a1841 = {
                        let a1838 = {
                            let a1812 = 1u64;
                            let a1813 = 1u64;
                            nat_add(a1812, a1813)?
                        };
                        let a1839 = {
                            let a1836 = {
                                let a1814 = 1u64;
                                let a1815 = 1u64;
                                nat_add(a1814, a1815)?
                            };
                            let a1837 = {
                                let a1834 = {
                                    let a1816 = 1u64;
                                    let a1817 = 1u64;
                                    nat_add(a1816, a1817)?
                                };
                                let a1835 = {
                                    let a1832 = {
                                        let a1818 = 1u64;
                                        let a1819 = 1u64;
                                        nat_add(a1818, a1819)?
                                    };
                                    let a1833 = {
                                        let a1830 = {
                                            let a1820 = 1u64;
                                            let a1821 = 1u64;
                                            nat_add(a1820, a1821)?
                                        };
                                        let a1831 = {
                                            let a1828 = {
                                                let a1822 = 1u64;
                                                let a1823 = 1u64;
                                                nat_add(a1822, a1823)?
                                            };
                                            let a1829 = {
                                                let a1826 = {
                                                    let a1824 = 1u64;
                                                    let a1825 = 1u64;
                                                    nat_add(a1824, a1825)?
                                                };
                                                let a1827 = 1u64;
                                                nat_mul(a1826, a1827)?
                                            };
                                            nat_mul(a1828, a1829)?
                                        };
                                        nat_mul(a1830, a1831)?
                                    };
                                    nat_mul(a1832, a1833)?
                                };
                                nat_mul(a1834, a1835)?
                            };
                            nat_mul(a1836, a1837)?
                        };
                        nat_mul(a1838, a1839)?
                    };
                    nat_mul(a1840, a1841)?
                };
                nat_mul(a1842, a1843)?
            };
            let a1845 = 1u64;
            nat_add(a1844, a1845)?
        };
        if if nat_eq(a1846, a1847) {
            let a1870 = v8;
            let a1871 = f22({
                let a1868 = {
                    let a1866 = {
                        let a1848 = 1u64;
                        let a1849 = 1u64;
                        nat_add(a1848, a1849)?
                    };
                    let a1867 = {
                        let a1864 = {
                            let a1862 = {
                                let a1850 = 1u64;
                                let a1851 = 1u64;
                                nat_add(a1850, a1851)?
                            };
                            let a1863 = {
                                let a1860 = {
                                    let a1858 = {
                                        let a1852 = 1u64;
                                        let a1853 = 1u64;
                                        nat_add(a1852, a1853)?
                                    };
                                    let a1859 = {
                                        let a1856 = {
                                            let a1854 = 1u64;
                                            let a1855 = 1u64;
                                            nat_add(a1854, a1855)?
                                        };
                                        let a1857 = 1u64;
                                        nat_mul(a1856, a1857)?
                                    };
                                    nat_mul(a1858, a1859)?
                                };
                                let a1861 = 1u64;
                                nat_add(a1860, a1861)?
                            };
                            nat_mul(a1862, a1863)?
                        };
                        let a1865 = 1u64;
                        nat_add(a1864, a1865)?
                    };
                    nat_mul(a1866, a1867)?
                };
                let a1869 = 1u64;
                nat_add(a1868, a1869)?
            })?;
            nat_lt(a1870, a1871)
        } else {
            false
        } {
            0u64
        } else {
            let a1908 = v9;
            let a1909 = {
                let a1906 = {
                    let a1872 = 1u64;
                    let a1873 = 1u64;
                    nat_add(a1872, a1873)?
                };
                let a1907 = {
                    let a1904 = {
                        let a1874 = 1u64;
                        let a1875 = 1u64;
                        nat_add(a1874, a1875)?
                    };
                    let a1905 = {
                        let a1902 = {
                            let a1876 = 1u64;
                            let a1877 = 1u64;
                            nat_add(a1876, a1877)?
                        };
                        let a1903 = {
                            let a1900 = {
                                let a1878 = 1u64;
                                let a1879 = 1u64;
                                nat_add(a1878, a1879)?
                            };
                            let a1901 = {
                                let a1898 = {
                                    let a1880 = 1u64;
                                    let a1881 = 1u64;
                                    nat_add(a1880, a1881)?
                                };
                                let a1899 = {
                                    let a1896 = {
                                        let a1882 = 1u64;
                                        let a1883 = 1u64;
                                        nat_add(a1882, a1883)?
                                    };
                                    let a1897 = {
                                        let a1894 = {
                                            let a1884 = 1u64;
                                            let a1885 = 1u64;
                                            nat_add(a1884, a1885)?
                                        };
                                        let a1895 = {
                                            let a1892 = {
                                                let a1886 = 1u64;
                                                let a1887 = 1u64;
                                                nat_add(a1886, a1887)?
                                            };
                                            let a1893 = {
                                                let a1890 = {
                                                    let a1888 = 1u64;
                                                    let a1889 = 1u64;
                                                    nat_add(a1888, a1889)?
                                                };
                                                let a1891 = 1u64;
                                                nat_mul(a1890, a1891)?
                                            };
                                            nat_mul(a1892, a1893)?
                                        };
                                        nat_mul(a1894, a1895)?
                                    };
                                    nat_mul(a1896, a1897)?
                                };
                                nat_mul(a1898, a1899)?
                            };
                            nat_mul(a1900, a1901)?
                        };
                        nat_mul(a1902, a1903)?
                    };
                    nat_mul(a1904, a1905)?
                };
                nat_mul(a1906, a1907)?
            };
            nat_sub(a1908, a1909)
        }
    };
    let a1910 = v8;
    let a1911 = 0u64;
    if nat_eq(a1910, a1911) {
        f25(v0)
    } else {
        let a1964 = {
            let a1962 = {
                let a1960 = {
                    let a1912 = 1u64;
                    let a1913 = 1u64;
                    nat_add(a1912, a1913)?
                };
                let a1961 = {
                    let a1958 = {
                        let a1956 = {
                            let a1914 = 1u64;
                            let a1915 = 1u64;
                            nat_add(a1914, a1915)?
                        };
                        let a1957 = {
                            let a1954 = {
                                let a1952 = {
                                    let a1916 = 1u64;
                                    let a1917 = 1u64;
                                    nat_add(a1916, a1917)?
                                };
                                let a1953 = {
                                    let a1950 = {
                                        let a1948 = {
                                            let a1918 = 1u64;
                                            let a1919 = 1u64;
                                            nat_add(a1918, a1919)?
                                        };
                                        let a1949 = {
                                            let a1946 = {
                                                let a1944 = {
                                                    let a1920 = 1u64;
                                                    let a1921 = 1u64;
                                                    nat_add(a1920, a1921)?
                                                };
                                                let a1945 = {
                                                    let a1942 = {
                                                        let a1940 = {
                                                            let a1922 = 1u64;
                                                            let a1923 = 1u64;
                                                            nat_add(a1922, a1923)?
                                                        };
                                                        let a1941 = {
                                                            let a1938 = {
                                                                let a1936 = {
                                                                    let a1924 = 1u64;
                                                                    let a1925 = 1u64;
                                                                    nat_add(a1924, a1925)?
                                                                };
                                                                let a1937 = {
                                                                    let a1934 = {
                                                                        let a1932 = {
                                                                            let a1926 = 1u64;
                                                                            let a1927 = 1u64;
                                                                            nat_add(a1926, a1927)?
                                                                        };
                                                                        let a1933 = {
                                                                            let a1930 = {
                                                                                let a1928 = 1u64;
                                                                                let a1929 = 1u64;
                                                                                nat_add(a1928, a1929)?
                                                                            };
                                                                            let a1931 = 1u64;
                                                                            nat_mul(a1930, a1931)?
                                                                        };
                                                                        nat_mul(a1932, a1933)?
                                                                    };
                                                                    let a1935 = 1u64;
                                                                    nat_add(a1934, a1935)?
                                                                };
                                                                nat_mul(a1936, a1937)?
                                                            };
                                                            let a1939 = 1u64;
                                                            nat_add(a1938, a1939)?
                                                        };
                                                        nat_mul(a1940, a1941)?
                                                    };
                                                    let a1943 = 1u64;
                                                    nat_add(a1942, a1943)?
                                                };
                                                nat_mul(a1944, a1945)?
                                            };
                                            let a1947 = 1u64;
                                            nat_add(a1946, a1947)?
                                        };
                                        nat_mul(a1948, a1949)?
                                    };
                                    let a1951 = 1u64;
                                    nat_add(a1950, a1951)?
                                };
                                nat_mul(a1952, a1953)?
                            };
                            let a1955 = 1u64;
                            nat_add(a1954, a1955)?
                        };
                        nat_mul(a1956, a1957)?
                    };
                    let a1959 = 1u64;
                    nat_add(a1958, a1959)?
                };
                nat_mul(a1960, a1961)?
            };
            let a1963 = 1u64;
            nat_add(a1962, a1963)?
        };
        let a1965 = v9;
        if nat_le(a1964, a1965) {
            f23(v0)
        } else {
            let a2017 = {
                let a1990 = f25(v0)?;
                let a1991 = {
                    let a1988 = v10;
                    let a1989 = f22({
                        let a1986 = {
                            let a1984 = {
                                let a1966 = 1u64;
                                let a1967 = 1u64;
                                nat_add(a1966, a1967)?
                            };
                            let a1985 = {
                                let a1982 = {
                                    let a1980 = {
                                        let a1968 = 1u64;
                                        let a1969 = 1u64;
                                        nat_add(a1968, a1969)?
                                    };
                                    let a1981 = {
                                        let a1978 = {
                                            let a1976 = {
                                                let a1970 = 1u64;
                                                let a1971 = 1u64;
                                                nat_add(a1970, a1971)?
                                            };
                                            let a1977 = {
                                                let a1974 = {
                                                    let a1972 = 1u64;
                                                    let a1973 = 1u64;
                                                    nat_add(a1972, a1973)?
                                                };
                                                let a1975 = 1u64;
                                                nat_mul(a1974, a1975)?
                                            };
                                            nat_mul(a1976, a1977)?
                                        };
                                        let a1979 = 1u64;
                                        nat_add(a1978, a1979)?
                                    };
                                    nat_mul(a1980, a1981)?
                                };
                                let a1983 = 1u64;
                                nat_add(a1982, a1983)?
                            };
                            nat_mul(a1984, a1985)?
                        };
                        let a1987 = 1u64;
                        nat_add(a1986, a1987)?
                    })?;
                    nat_mul(a1988, a1989)?
                };
                nat_add(a1990, a1991)?
            };
            let a2018 = {
                let a2014 = v8;
                let a2015 = f22({
                    let a2012 = {
                        let a2010 = {
                            let a1992 = 1u64;
                            let a1993 = 1u64;
                            nat_add(a1992, a1993)?
                        };
                        let a2011 = {
                            let a2008 = {
                                let a2006 = {
                                    let a1994 = 1u64;
                                    let a1995 = 1u64;
                                    nat_add(a1994, a1995)?
                                };
                                let a2007 = {
                                    let a2004 = {
                                        let a2002 = {
                                            let a1996 = 1u64;
                                            let a1997 = 1u64;
                                            nat_add(a1996, a1997)?
                                        };
                                        let a2003 = {
                                            let a2000 = {
                                                let a1998 = 1u64;
                                                let a1999 = 1u64;
                                                nat_add(a1998, a1999)?
                                            };
                                            let a2001 = 1u64;
                                            nat_mul(a2000, a2001)?
                                        };
                                        nat_mul(a2002, a2003)?
                                    };
                                    let a2005 = 1u64;
                                    nat_add(a2004, a2005)?
                                };
                                nat_mul(a2006, a2007)?
                            };
                            let a2009 = 1u64;
                            nat_add(a2008, a2009)?
                        };
                        nat_mul(a2010, a2011)?
                    };
                    let a2013 = 1u64;
                    nat_add(a2012, a2013)?
                })?;
                let a2016 = 0u64;
                nat_rem(a2014, a2015, a2016)
            };
            nat_add(a2017, a2018)
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt1> {
    f0(p0)
}
