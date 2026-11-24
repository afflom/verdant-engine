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
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64, u64),
    C4(u64),
    C5(u64),
    C6(bool, bool, u64),
    C7(u64, u64, u64, u64),
    C8(bool, bool, u64, u64, u64),
    C9(bool, bool, u64, u64, u64),
    C10(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64),
    C2,
    C3(u64, u64, u64),
    C4(u64, u64, u64),
    C5(u64, u64, u64),
    C6(u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F16,
    F18,
    F19,
    F35,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F16 => f16(p0),
            Fn0::F18 => f18(p0),
            Fn0::F19 => f19(p0),
            Fn0::F35 => f35(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F44,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F44 => f44(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v22, v23) => {
            let v24: u64 = {
                let a8 = v23;
                let a9 = 2147483648u64;
                let a10 = 0u64;
                nat_rem(a8, a9, a10)
            };
            let a11 = v23;
            let a12 = 4294967296u64;
            if if if nat_lt(a11, a12) {
                let a13 = v22;
                let a14 = 2139095040u64;
                nat_lt(a13, a14)
            } else {
                false
            } {
                let a15 = v24;
                let a16 = 2139095040u64;
                nat_lt(a15, a16)
            } else {
                false
            } {
                Ok(Adt1::C1({
                    let a17 = v22;
                    let a18 = v24;
                    if nat_lt(a17, a18) {
                        v24
                    } else {
                        v22
                    }
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C1(v25) => {
            let a19 = v25;
            let a20 = 2139095040u64;
            if nat_lt(a19, a20) {
                Ok(Adt1::C3(3u64, v25, 1123942400u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C2(v26) => {
            let a21 = v26;
            let a22 = 0u64;
            if nat_eq(a21, a22) {
                Ok(Adt1::C2)
            } else {
                let a23 = v26;
                let a24 = 2139095040u64;
                if nat_lt(a23, a24) {
                    Ok(Adt1::C3(3u64, 1065353216u64, v26))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
        Adt0::C3(v27, v28) => {
            let a25 = v27;
            let a26 = 4294967296u64;
            if if nat_lt(a25, a26) {
                let a27 = v28;
                let a28 = 2139095040u64;
                nat_lt(a27, a28)
            } else {
                false
            } {
                Ok(Adt1::C3(2u64, v27, v28))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C4(v29) => {
            let a29 = v29;
            let a30 = 4294967296u64;
            if nat_lt(a29, a30) {
                f5(v29, {
                    let a31 = v29;
                    let a32 = 2147483648u64;
                    let a33 = 0u64;
                    nat_rem(a31, a32, a33)
                })
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C5(v30) => {
            let a34 = v30;
            let a35 = 32u64;
            if nat_eq(a34, a35) {
                Ok(Adt1::C1(0u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C6(v1, v2, v3) => {
            let v4: u64 = {
                let a2 = if v1 {
                    4u64
                } else {
                    2u64
                };
                let a3 = if v2 {
                    4u64
                } else {
                    0u64
                };
                nat_add(a2, a3)?
            };
            let a6 = v3;
            let a7 = {
                let a4 = v4;
                let a5 = 16u64;
                nat_add(a4, a5)?
            };
            if nat_eq(a6, a7) {
                Ok(Adt1::C1(v4))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C7(v5, v6, v7, v8) => {
            f1(v5, v6, v7, v8)
        }
        Adt0::C8(v9, v10, v11, v12, v13) => {
            f2(v9, v10, v11, v12, v13)
        }
        Adt0::C9(v14, v15, v16, v17, v18) => {
            f3(v14, v15, v16, v17, v18)
        }
        Adt0::C10(v19, v20, v21) => {
            f4(v19, v20, v21)
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let a36 = v3;
    let a37 = 4294967296u64;
    if if nat_lt(a36, a37) {
        let a38 = f6(v3)?;
        let a39 = 2139095040u64;
        nat_lt(a38, a39)
    } else {
        false
    } {
        Ok(Adt1::C4({
            let a40 = f7(v3, v0);
            let a41 = 0u64;
            if nat_eq(a40, a41) {
                v3
            } else {
                v0
            }
        }, {
            let a42 = f7(v1, v3);
            let a43 = 0u64;
            if nat_eq(a42, a43) {
                v3
            } else {
                v1
            }
        }, {
            let a44 = f6(v2)?;
            let a45 = f6(v3)?;
            if nat_lt(a44, a45) {
                v3
            } else {
                v2
            }
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f2(v0: bool, v1: bool, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let v5: u64 = f8(if v0 {
        f9(v3, f10(v2)?)?
    } else {
        v4
    }, if v0 {
        if v1 {
            1106771968u64
        } else {
            1097859072u64
        }
    } else {
        if v1 {
            3246391296u64
        } else {
            3238002688u64
        }
    })?;
    let v6: u64 = if f11(v5)? {
        0u64
    } else {
        f8(1065353216u64, v5)?
    };
    let a46 = f6(v6)?;
    let a47 = 2139095040u64;
    if nat_lt(a46, a47) {
        Ok(Adt1::C5(f12(v5)?, f12(v2)?, v6))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: bool, v1: bool, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let v5: u64 = f9(f13(if v0 {
        f9(v4, f10(v2)?)?
    } else {
        v4
    }, v3)?, if v0 {
        1056964608u64
    } else {
        if v1 {
            1099169792u64
        } else {
            1091043328u64
        }
    })?;
    let a48 = v5;
    let a49 = 1115684864u64;
    if nat_lt(a48, a49) {
        let v6: u64 = {
            let a50 = v5;
            let a51 = 1065353216u64;
            if nat_lt(a50, a51) {
                0u64
            } else {
                let a54 = f14(v5)?;
                let a55 = f15(Fn0::F16, {
                    let a52 = 150u64;
                    let a53 = f17(v5)?;
                    nat_sub(a52, a53)
                }, 1u64)?;
                let a56 = 0u64;
                nat_quot(a54, a55, a56)
            }
        };
        let v7: u64 = if v1 {
            31u64
        } else {
            15u64
        };
        Ok(Adt1::C1({
            let a57 = v7;
            let a58 = v6;
            if nat_lt(a57, a58) {
                v7
            } else {
                v6
            }
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let a59 = v0;
    let a60 = 16u64;
    if if if nat_lt(a59, a60) {
        let a61 = v1;
        let a62 = 32u64;
        nat_lt(a61, a62)
    } else {
        false
    } {
        let a63 = v2;
        let a64 = 32u64;
        nat_lt(a63, a64)
    } else {
        false
    } {
        Ok(Adt1::C6({
            let a73 = {
                let a65 = v1;
                let a66 = 16u64;
                let a67 = 0u64;
                nat_rem(a65, a66, a67)
            };
            let a74 = {
                let a71 = 16u64;
                let a72 = {
                    let a68 = v2;
                    let a69 = 16u64;
                    let a70 = 0u64;
                    nat_rem(a68, a69, a70)
                };
                nat_mul(a71, a72)?
            };
            nat_add(a73, a74)?
        }, {
            let a85 = f15(Fn0::F18, v0, 1u64)?;
            let a86 = {
                let a83 = {
                    let a75 = v1;
                    let a76 = 16u64;
                    let a77 = 0u64;
                    nat_quot(a75, a76, a77)
                };
                let a84 = {
                    let a81 = 65536u64;
                    let a82 = {
                        let a78 = v2;
                        let a79 = 16u64;
                        let a80 = 0u64;
                        nat_quot(a78, a79, a80)
                    };
                    nat_mul(a81, a82)?
                };
                nat_add(a83, a84)?
            };
            nat_mul(a85, a86)?
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: u64, v1: u64) -> R<Adt1> {
    let a87 = v1;
    let a88 = 1056964608u64;
    if nat_lt(a87, a88) {
        Ok(Adt1::C1(0u64))
    } else {
        let v2: u64 = {
            let a89 = v1;
            let a90 = 8388608u64;
            let a91 = 0u64;
            nat_quot(a89, a90, a91)
        };
        let a92 = 134u64;
        let a93 = v2;
        if nat_lt(a92, a93) {
            Ok(Adt1::C0)
        } else {
            let v3: u64 = f15(Fn0::F19, {
                let a94 = 150u64;
                let a95 = v2;
                nat_sub(a94, a95)
            }, 1u64)?;
            let v4: u64 = {
                let a99 = 8388608u64;
                let a100 = {
                    let a96 = v1;
                    let a97 = 8388608u64;
                    let a98 = 0u64;
                    nat_rem(a96, a97, a98)
                };
                nat_add(a99, a100)?
            };
            let v5: u64 = {
                let a106 = {
                    let a104 = v4;
                    let a105 = {
                        let a101 = v3;
                        let a102 = 2u64;
                        let a103 = 0u64;
                        nat_quot(a101, a102, a103)
                    };
                    nat_add(a104, a105)?
                };
                let a107 = v3;
                let a108 = 0u64;
                nat_quot(a106, a107, a108)
            };
            let a109 = v0;
            let a110 = 2147483648u64;
            if nat_lt(a109, a110) {
                let a111 = v5;
                let a112 = 128u64;
                if nat_lt(a111, a112) {
                    Ok(Adt1::C1(v5))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                let a113 = v5;
                let a114 = 128u64;
                if nat_le(a113, a114) {
                    Ok(Adt1::C1({
                        let a115 = 256u64;
                        let a116 = v5;
                        nat_sub(a115, a116)
                    }))
                } else {
                    Ok(Adt1::C0)
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let a141 = v0;
    let a142 = f20({
        let a139 = {
            let a137 = {
                let a117 = 1u64;
                let a118 = 1u64;
                nat_add(a117, a118)?
            };
            let a138 = {
                let a135 = {
                    let a133 = {
                        let a119 = 1u64;
                        let a120 = 1u64;
                        nat_add(a119, a120)?
                    };
                    let a134 = {
                        let a131 = {
                            let a129 = {
                                let a121 = 1u64;
                                let a122 = 1u64;
                                nat_add(a121, a122)?
                            };
                            let a130 = {
                                let a127 = {
                                    let a125 = {
                                        let a123 = 1u64;
                                        let a124 = 1u64;
                                        nat_add(a123, a124)?
                                    };
                                    let a126 = 1u64;
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
                    nat_mul(a133, a134)?
                };
                let a136 = 1u64;
                nat_add(a135, a136)?
            };
            nat_mul(a137, a138)?
        };
        let a140 = 1u64;
        nat_add(a139, a140)?
    })?;
    let a143 = 0u64;
    Ok(nat_rem(a141, a142, a143))
}

pub fn f7(v0: u64, v1: u64) -> u64 {
    if f21(v1) {
        3u64
    } else {
        if f21(v0) {
            4u64
        } else {
            let a144 = v0;
            let a145 = v1;
            if if nat_eq(a144, a145) {
                true
            } else {
                let a146 = f22(v0);
                let a147 = 0u64;
                if nat_eq(a146, a147) {
                    let a148 = f22(v1);
                    let a149 = 0u64;
                    nat_eq(a148, a149)
                } else {
                    false
                }
            } {
                1u64
            } else {
                let a150 = f23(v0);
                let a151 = f23(v1);
                if nat_eq(a150, a151) {
                    let a152 = f23(v0);
                    let a153 = 0u64;
                    if nat_eq(a152, a153) {
                        let a154 = f22(v0);
                        let a155 = f22(v1);
                        if nat_lt(a154, a155) {
                            0u64
                        } else {
                            2u64
                        }
                    } else {
                        let a156 = f22(v1);
                        let a157 = f22(v0);
                        if nat_lt(a156, a157) {
                            0u64
                        } else {
                            2u64
                        }
                    }
                } else {
                    let a158 = f23(v0);
                    let a159 = 1u64;
                    if nat_eq(a158, a159) {
                        0u64
                    } else {
                        2u64
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
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
        if f11(v0)? {
            f11(v1)?
        } else {
            false
        }
    } {
        let a246 = {
            let a224 = {
                let a200 = {
                    let a198 = {
                        let a160 = 1u64;
                        let a161 = 1u64;
                        nat_add(a160, a161)?
                    };
                    let a199 = {
                        let a196 = {
                            let a194 = {
                                let a162 = 1u64;
                                let a163 = 1u64;
                                nat_add(a162, a163)?
                            };
                            let a195 = {
                                let a192 = {
                                    let a190 = {
                                        let a164 = 1u64;
                                        let a165 = 1u64;
                                        nat_add(a164, a165)?
                                    };
                                    let a191 = {
                                        let a188 = {
                                            let a186 = {
                                                let a166 = 1u64;
                                                let a167 = 1u64;
                                                nat_add(a166, a167)?
                                            };
                                            let a187 = {
                                                let a184 = {
                                                    let a182 = {
                                                        let a168 = 1u64;
                                                        let a169 = 1u64;
                                                        nat_add(a168, a169)?
                                                    };
                                                    let a183 = {
                                                        let a180 = {
                                                            let a178 = {
                                                                let a170 = 1u64;
                                                                let a171 = 1u64;
                                                                nat_add(a170, a171)?
                                                            };
                                                            let a179 = {
                                                                let a176 = {
                                                                    let a174 = {
                                                                        let a172 = 1u64;
                                                                        let a173 = 1u64;
                                                                        nat_add(a172, a173)?
                                                                    };
                                                                    let a175 = 1u64;
                                                                    nat_mul(a174, a175)?
                                                                };
                                                                let a177 = 1u64;
                                                                nat_add(a176, a177)?
                                                            };
                                                            nat_mul(a178, a179)?
                                                        };
                                                        let a181 = 1u64;
                                                        nat_add(a180, a181)?
                                                    };
                                                    nat_mul(a182, a183)?
                                                };
                                                let a185 = 1u64;
                                                nat_add(a184, a185)?
                                            };
                                            nat_mul(a186, a187)?
                                        };
                                        let a189 = 1u64;
                                        nat_add(a188, a189)?
                                    };
                                    nat_mul(a190, a191)?
                                };
                                let a193 = 1u64;
                                nat_add(a192, a193)?
                            };
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
            let a225 = f20({
                let a222 = {
                    let a220 = {
                        let a202 = 1u64;
                        let a203 = 1u64;
                        nat_add(a202, a203)?
                    };
                    let a221 = {
                        let a218 = {
                            let a216 = {
                                let a204 = 1u64;
                                let a205 = 1u64;
                                nat_add(a204, a205)?
                            };
                            let a217 = {
                                let a214 = {
                                    let a212 = {
                                        let a206 = 1u64;
                                        let a207 = 1u64;
                                        nat_add(a206, a207)?
                                    };
                                    let a213 = {
                                        let a210 = {
                                            let a208 = 1u64;
                                            let a209 = 1u64;
                                            nat_add(a208, a209)?
                                        };
                                        let a211 = 1u64;
                                        nat_mul(a210, a211)?
                                    };
                                    nat_mul(a212, a213)?
                                };
                                let a215 = 1u64;
                                nat_add(a214, a215)?
                            };
                            nat_mul(a216, a217)?
                        };
                        let a219 = 1u64;
                        nat_add(a218, a219)?
                    };
                    nat_mul(a220, a221)?
                };
                let a223 = 1u64;
                nat_add(a222, a223)?
            })?;
            nat_mul(a224, a225)?
        };
        let a247 = f20({
            let a244 = {
                let a226 = 1u64;
                let a227 = 1u64;
                nat_add(a226, a227)?
            };
            let a245 = {
                let a242 = {
                    let a240 = {
                        let a228 = 1u64;
                        let a229 = 1u64;
                        nat_add(a228, a229)?
                    };
                    let a241 = {
                        let a238 = {
                            let a236 = {
                                let a230 = 1u64;
                                let a231 = 1u64;
                                nat_add(a230, a231)?
                            };
                            let a237 = {
                                let a234 = {
                                    let a232 = 1u64;
                                    let a233 = 1u64;
                                    nat_add(a232, a233)?
                                };
                                let a235 = 1u64;
                                nat_mul(a234, a235)?
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
            };
            nat_mul(a244, a245)?
        })?;
        nat_add(a246, a247)
    } else {
        if if f25(v0)? {
            true
        } else {
            f11(v1)?
        } {
            f26(f27(v0, v1)?)
        } else {
            if if f25(v1)? {
                true
            } else {
                f11(v0)?
            } {
                f28(f27(v0, v1)?)
            } else {
                f29(v0, v1)
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    if if f24(v0)? {
        true
    } else {
        f24(v1)?
    } {
        let a334 = {
            let a312 = {
                let a288 = {
                    let a286 = {
                        let a248 = 1u64;
                        let a249 = 1u64;
                        nat_add(a248, a249)?
                    };
                    let a287 = {
                        let a284 = {
                            let a282 = {
                                let a250 = 1u64;
                                let a251 = 1u64;
                                nat_add(a250, a251)?
                            };
                            let a283 = {
                                let a280 = {
                                    let a278 = {
                                        let a252 = 1u64;
                                        let a253 = 1u64;
                                        nat_add(a252, a253)?
                                    };
                                    let a279 = {
                                        let a276 = {
                                            let a274 = {
                                                let a254 = 1u64;
                                                let a255 = 1u64;
                                                nat_add(a254, a255)?
                                            };
                                            let a275 = {
                                                let a272 = {
                                                    let a270 = {
                                                        let a256 = 1u64;
                                                        let a257 = 1u64;
                                                        nat_add(a256, a257)?
                                                    };
                                                    let a271 = {
                                                        let a268 = {
                                                            let a266 = {
                                                                let a258 = 1u64;
                                                                let a259 = 1u64;
                                                                nat_add(a258, a259)?
                                                            };
                                                            let a267 = {
                                                                let a264 = {
                                                                    let a262 = {
                                                                        let a260 = 1u64;
                                                                        let a261 = 1u64;
                                                                        nat_add(a260, a261)?
                                                                    };
                                                                    let a263 = 1u64;
                                                                    nat_mul(a262, a263)?
                                                                };
                                                                let a265 = 1u64;
                                                                nat_add(a264, a265)?
                                                            };
                                                            nat_mul(a266, a267)?
                                                        };
                                                        let a269 = 1u64;
                                                        nat_add(a268, a269)?
                                                    };
                                                    nat_mul(a270, a271)?
                                                };
                                                let a273 = 1u64;
                                                nat_add(a272, a273)?
                                            };
                                            nat_mul(a274, a275)?
                                        };
                                        let a277 = 1u64;
                                        nat_add(a276, a277)?
                                    };
                                    nat_mul(a278, a279)?
                                };
                                let a281 = 1u64;
                                nat_add(a280, a281)?
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
            let a313 = f20({
                let a310 = {
                    let a308 = {
                        let a290 = 1u64;
                        let a291 = 1u64;
                        nat_add(a290, a291)?
                    };
                    let a309 = {
                        let a306 = {
                            let a304 = {
                                let a292 = 1u64;
                                let a293 = 1u64;
                                nat_add(a292, a293)?
                            };
                            let a305 = {
                                let a302 = {
                                    let a300 = {
                                        let a294 = 1u64;
                                        let a295 = 1u64;
                                        nat_add(a294, a295)?
                                    };
                                    let a301 = {
                                        let a298 = {
                                            let a296 = 1u64;
                                            let a297 = 1u64;
                                            nat_add(a296, a297)?
                                        };
                                        let a299 = 1u64;
                                        nat_mul(a298, a299)?
                                    };
                                    nat_mul(a300, a301)?
                                };
                                let a303 = 1u64;
                                nat_add(a302, a303)?
                            };
                            nat_mul(a304, a305)?
                        };
                        let a307 = 1u64;
                        nat_add(a306, a307)?
                    };
                    nat_mul(a308, a309)?
                };
                let a311 = 1u64;
                nat_add(a310, a311)?
            })?;
            nat_mul(a312, a313)?
        };
        let a335 = f20({
            let a332 = {
                let a314 = 1u64;
                let a315 = 1u64;
                nat_add(a314, a315)?
            };
            let a333 = {
                let a330 = {
                    let a328 = {
                        let a316 = 1u64;
                        let a317 = 1u64;
                        nat_add(a316, a317)?
                    };
                    let a329 = {
                        let a326 = {
                            let a324 = {
                                let a318 = 1u64;
                                let a319 = 1u64;
                                nat_add(a318, a319)?
                            };
                            let a325 = {
                                let a322 = {
                                    let a320 = 1u64;
                                    let a321 = 1u64;
                                    nat_add(a320, a321)?
                                };
                                let a323 = 1u64;
                                nat_mul(a322, a323)?
                            };
                            nat_mul(a324, a325)?
                        };
                        let a327 = 1u64;
                        nat_add(a326, a327)?
                    };
                    nat_mul(a328, a329)?
                };
                let a331 = 1u64;
                nat_add(a330, a331)?
            };
            nat_mul(a332, a333)?
        })?;
        nat_add(a334, a335)
    } else {
        if if f25(v0)? {
            f25(v1)?
        } else {
            false
        } {
            let a336 = f30(v0)?;
            let a337 = f30(v1)?;
            if nat_eq(a336, a337) {
                Ok(v0)
            } else {
                let a424 = {
                    let a402 = {
                        let a378 = {
                            let a376 = {
                                let a338 = 1u64;
                                let a339 = 1u64;
                                nat_add(a338, a339)?
                            };
                            let a377 = {
                                let a374 = {
                                    let a372 = {
                                        let a340 = 1u64;
                                        let a341 = 1u64;
                                        nat_add(a340, a341)?
                                    };
                                    let a373 = {
                                        let a370 = {
                                            let a368 = {
                                                let a342 = 1u64;
                                                let a343 = 1u64;
                                                nat_add(a342, a343)?
                                            };
                                            let a369 = {
                                                let a366 = {
                                                    let a364 = {
                                                        let a344 = 1u64;
                                                        let a345 = 1u64;
                                                        nat_add(a344, a345)?
                                                    };
                                                    let a365 = {
                                                        let a362 = {
                                                            let a360 = {
                                                                let a346 = 1u64;
                                                                let a347 = 1u64;
                                                                nat_add(a346, a347)?
                                                            };
                                                            let a361 = {
                                                                let a358 = {
                                                                    let a356 = {
                                                                        let a348 = 1u64;
                                                                        let a349 = 1u64;
                                                                        nat_add(a348, a349)?
                                                                    };
                                                                    let a357 = {
                                                                        let a354 = {
                                                                            let a352 = {
                                                                                let a350 = 1u64;
                                                                                let a351 = 1u64;
                                                                                nat_add(a350, a351)?
                                                                            };
                                                                            let a353 = 1u64;
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
                    let a403 = f20({
                        let a400 = {
                            let a398 = {
                                let a380 = 1u64;
                                let a381 = 1u64;
                                nat_add(a380, a381)?
                            };
                            let a399 = {
                                let a396 = {
                                    let a394 = {
                                        let a382 = 1u64;
                                        let a383 = 1u64;
                                        nat_add(a382, a383)?
                                    };
                                    let a395 = {
                                        let a392 = {
                                            let a390 = {
                                                let a384 = 1u64;
                                                let a385 = 1u64;
                                                nat_add(a384, a385)?
                                            };
                                            let a391 = {
                                                let a388 = {
                                                    let a386 = 1u64;
                                                    let a387 = 1u64;
                                                    nat_add(a386, a387)?
                                                };
                                                let a389 = 1u64;
                                                nat_mul(a388, a389)?
                                            };
                                            nat_mul(a390, a391)?
                                        };
                                        let a393 = 1u64;
                                        nat_add(a392, a393)?
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
                    })?;
                    nat_mul(a402, a403)?
                };
                let a425 = f20({
                    let a422 = {
                        let a404 = 1u64;
                        let a405 = 1u64;
                        nat_add(a404, a405)?
                    };
                    let a423 = {
                        let a420 = {
                            let a418 = {
                                let a406 = 1u64;
                                let a407 = 1u64;
                                nat_add(a406, a407)?
                            };
                            let a419 = {
                                let a416 = {
                                    let a414 = {
                                        let a408 = 1u64;
                                        let a409 = 1u64;
                                        nat_add(a408, a409)?
                                    };
                                    let a415 = {
                                        let a412 = {
                                            let a410 = 1u64;
                                            let a411 = 1u64;
                                            nat_add(a410, a411)?
                                        };
                                        let a413 = 1u64;
                                        nat_mul(a412, a413)?
                                    };
                                    nat_mul(a414, a415)?
                                };
                                let a417 = 1u64;
                                nat_add(a416, a417)?
                            };
                            nat_mul(a418, a419)?
                        };
                        let a421 = 1u64;
                        nat_add(a420, a421)?
                    };
                    nat_mul(a422, a423)?
                })?;
                nat_add(a424, a425)
            }
        } else {
            if f25(v0)? {
                Ok(v0)
            } else {
                if f25(v1)? {
                    Ok(v1)
                } else {
                    let a426 = f6(v1)?;
                    let a427 = f6(v0)?;
                    if nat_le(a426, a427) {
                        f31(v0, v1)
                    } else {
                        f31(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let a474 = {
        let a452 = v0;
        let a453 = f20({
            let a450 = {
                let a448 = {
                    let a428 = 1u64;
                    let a429 = 1u64;
                    nat_add(a428, a429)?
                };
                let a449 = {
                    let a446 = {
                        let a444 = {
                            let a430 = 1u64;
                            let a431 = 1u64;
                            nat_add(a430, a431)?
                        };
                        let a445 = {
                            let a442 = {
                                let a440 = {
                                    let a432 = 1u64;
                                    let a433 = 1u64;
                                    nat_add(a432, a433)?
                                };
                                let a441 = {
                                    let a438 = {
                                        let a436 = {
                                            let a434 = 1u64;
                                            let a435 = 1u64;
                                            nat_add(a434, a435)?
                                        };
                                        let a437 = 1u64;
                                        nat_mul(a436, a437)?
                                    };
                                    let a439 = 1u64;
                                    nat_add(a438, a439)?
                                };
                                nat_mul(a440, a441)?
                            };
                            let a443 = 1u64;
                            nat_add(a442, a443)?
                        };
                        nat_mul(a444, a445)?
                    };
                    let a447 = 1u64;
                    nat_add(a446, a447)?
                };
                nat_mul(a448, a449)?
            };
            let a451 = 1u64;
            nat_add(a450, a451)?
        })?;
        nat_add(a452, a453)?
    };
    let a475 = f20({
        let a472 = {
            let a454 = 1u64;
            let a455 = 1u64;
            nat_add(a454, a455)?
        };
        let a473 = {
            let a470 = {
                let a456 = 1u64;
                let a457 = 1u64;
                nat_add(a456, a457)?
            };
            let a471 = {
                let a468 = {
                    let a458 = 1u64;
                    let a459 = 1u64;
                    nat_add(a458, a459)?
                };
                let a469 = {
                    let a466 = {
                        let a460 = 1u64;
                        let a461 = 1u64;
                        nat_add(a460, a461)?
                    };
                    let a467 = {
                        let a464 = {
                            let a462 = 1u64;
                            let a463 = 1u64;
                            nat_add(a462, a463)?
                        };
                        let a465 = 1u64;
                        nat_mul(a464, a465)?
                    };
                    nat_mul(a466, a467)?
                };
                nat_mul(a468, a469)?
            };
            nat_mul(a470, a471)?
        };
        nat_mul(a472, a473)?
    })?;
    let a476 = 0u64;
    Ok(nat_rem(a474, a475, a476))
}

pub fn f11(v0: u64) -> R<bool> {
    let a477 = f6(v0)?;
    let a478 = 0u64;
    Ok(nat_eq(a477, a478))
}

pub fn f12(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a479 = f30(v0)?;
        let a480 = 32768u64;
        nat_mul(a479, a480)?
    };
    let v2: u64 = f17(v0)?;
    let a499 = v1;
    let a500 = {
        let a481 = v2;
        let a482 = 255u64;
        if nat_eq(a481, a482) {
            let a483 = f32(v0)?;
            let a484 = 0u64;
            if nat_eq(a483, a484) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a485 = 143u64;
            let a486 = v2;
            if nat_le(a485, a486) {
                31744u64
            } else {
                let a487 = v2;
                let a488 = 102u64;
                if nat_lt(a487, a488) {
                    0u64
                } else {
                    let a489 = 113u64;
                    let a490 = v2;
                    if nat_le(a489, a490) {
                        let a495 = {
                            let a493 = {
                                let a491 = v2;
                                let a492 = 113u64;
                                nat_sub(a491, a492)
                            };
                            let a494 = 1024u64;
                            nat_mul(a493, a494)?
                        };
                        let a496 = f33(f14(v0)?, 8192u64)?;
                        nat_add(a495, a496)?
                    } else {
                        f33(f14(v0)?, f20({
                            let a497 = 126u64;
                            let a498 = v2;
                            nat_sub(a497, a498)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a499, a500)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    if if if if f24(v0)? {
        true
    } else {
        f24(v1)?
    } {
        true
    } else {
        if f25(v0)? {
            f11(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f25(v1)? {
            f11(v0)?
        } else {
            false
        }
    } {
        let a587 = {
            let a565 = {
                let a541 = {
                    let a539 = {
                        let a501 = 1u64;
                        let a502 = 1u64;
                        nat_add(a501, a502)?
                    };
                    let a540 = {
                        let a537 = {
                            let a535 = {
                                let a503 = 1u64;
                                let a504 = 1u64;
                                nat_add(a503, a504)?
                            };
                            let a536 = {
                                let a533 = {
                                    let a531 = {
                                        let a505 = 1u64;
                                        let a506 = 1u64;
                                        nat_add(a505, a506)?
                                    };
                                    let a532 = {
                                        let a529 = {
                                            let a527 = {
                                                let a507 = 1u64;
                                                let a508 = 1u64;
                                                nat_add(a507, a508)?
                                            };
                                            let a528 = {
                                                let a525 = {
                                                    let a523 = {
                                                        let a509 = 1u64;
                                                        let a510 = 1u64;
                                                        nat_add(a509, a510)?
                                                    };
                                                    let a524 = {
                                                        let a521 = {
                                                            let a519 = {
                                                                let a511 = 1u64;
                                                                let a512 = 1u64;
                                                                nat_add(a511, a512)?
                                                            };
                                                            let a520 = {
                                                                let a517 = {
                                                                    let a515 = {
                                                                        let a513 = 1u64;
                                                                        let a514 = 1u64;
                                                                        nat_add(a513, a514)?
                                                                    };
                                                                    let a516 = 1u64;
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
                            nat_mul(a535, a536)?
                        };
                        let a538 = 1u64;
                        nat_add(a537, a538)?
                    };
                    nat_mul(a539, a540)?
                };
                let a542 = 1u64;
                nat_add(a541, a542)?
            };
            let a566 = f20({
                let a563 = {
                    let a561 = {
                        let a543 = 1u64;
                        let a544 = 1u64;
                        nat_add(a543, a544)?
                    };
                    let a562 = {
                        let a559 = {
                            let a557 = {
                                let a545 = 1u64;
                                let a546 = 1u64;
                                nat_add(a545, a546)?
                            };
                            let a558 = {
                                let a555 = {
                                    let a553 = {
                                        let a547 = 1u64;
                                        let a548 = 1u64;
                                        nat_add(a547, a548)?
                                    };
                                    let a554 = {
                                        let a551 = {
                                            let a549 = 1u64;
                                            let a550 = 1u64;
                                            nat_add(a549, a550)?
                                        };
                                        let a552 = 1u64;
                                        nat_mul(a551, a552)?
                                    };
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
            })?;
            nat_mul(a565, a566)?
        };
        let a588 = f20({
            let a585 = {
                let a567 = 1u64;
                let a568 = 1u64;
                nat_add(a567, a568)?
            };
            let a586 = {
                let a583 = {
                    let a581 = {
                        let a569 = 1u64;
                        let a570 = 1u64;
                        nat_add(a569, a570)?
                    };
                    let a582 = {
                        let a579 = {
                            let a577 = {
                                let a571 = 1u64;
                                let a572 = 1u64;
                                nat_add(a571, a572)?
                            };
                            let a578 = {
                                let a575 = {
                                    let a573 = 1u64;
                                    let a574 = 1u64;
                                    nat_add(a573, a574)?
                                };
                                let a576 = 1u64;
                                nat_mul(a575, a576)?
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
        })?;
        nat_add(a587, a588)
    } else {
        if if f25(v0)? {
            true
        } else {
            f25(v1)?
        } {
            f26(f27(v0, v1)?)
        } else {
            if if f11(v0)? {
                true
            } else {
                f11(v1)?
            } {
                f28(f27(v0, v1)?)
            } else {
                f34(v0, v1)
            }
        }
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a613 = f32(v0)?;
    let a614 = {
        let a589 = f17(v0)?;
        let a590 = 0u64;
        if nat_eq(a589, a590) {
            0u64
        } else {
            f20({
                let a611 = {
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
                                    let a601 = {
                                        let a595 = 1u64;
                                        let a596 = 1u64;
                                        nat_add(a595, a596)?
                                    };
                                    let a602 = {
                                        let a599 = {
                                            let a597 = 1u64;
                                            let a598 = 1u64;
                                            nat_add(a597, a598)?
                                        };
                                        let a600 = 1u64;
                                        nat_mul(a599, a600)?
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
            })?
        }
    };
    nat_add(a613, a614)
}

pub fn f15(v0: Fn0, p618: u64, p619: u64) -> R<u64> {
    let m617 = (p618, p619);
    {
        let mut m617 = m617;
        loop {
            let step = {
                let (v1, v2) = m617;
                let m615 = v1;
                if m615 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m615 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c616: Fn0 = v0.clone();
                        c616.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a620 = v0;
    let a621 = v0;
    nat_add(a620, a621)
}

pub fn f17(v0: u64) -> R<u64> {
    let a644 = f6(v0)?;
    let a645 = f20({
        let a642 = {
            let a640 = {
                let a622 = 1u64;
                let a623 = 1u64;
                nat_add(a622, a623)?
            };
            let a641 = {
                let a638 = {
                    let a636 = {
                        let a624 = 1u64;
                        let a625 = 1u64;
                        nat_add(a624, a625)?
                    };
                    let a637 = {
                        let a634 = {
                            let a632 = {
                                let a626 = 1u64;
                                let a627 = 1u64;
                                nat_add(a626, a627)?
                            };
                            let a633 = {
                                let a630 = {
                                    let a628 = 1u64;
                                    let a629 = 1u64;
                                    nat_add(a628, a629)?
                                };
                                let a631 = 1u64;
                                nat_mul(a630, a631)?
                            };
                            nat_mul(a632, a633)?
                        };
                        let a635 = 1u64;
                        nat_add(a634, a635)?
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
    })?;
    let a646 = 0u64;
    Ok(nat_quot(a644, a645, a646))
}

pub fn f18(v0: u64) -> R<u64> {
    let a647 = v0;
    let a648 = v0;
    nat_add(a647, a648)
}

pub fn f19(v0: u64) -> R<u64> {
    let a649 = v0;
    let a650 = v0;
    nat_add(a649, a650)
}

pub fn f20(v0: u64) -> R<u64> {
    f15(Fn0::F35, v0, 1u64)
}

pub fn f21(v0: u64) -> bool {
    let a651 = 2139095040u64;
    let a652 = f22(v0);
    nat_lt(a651, a652)
}

pub fn f22(v0: u64) -> u64 {
    let a653 = v0;
    let a654 = 2147483648u64;
    let a655 = 0u64;
    nat_rem(a653, a654, a655)
}

pub fn f23(v0: u64) -> u64 {
    let a656 = v0;
    let a657 = 2147483648u64;
    let a658 = 0u64;
    nat_quot(a656, a657, a658)
}

pub fn f24(v0: u64) -> R<bool> {
    let a701 = f17(v0)?;
    let a702 = {
        let a699 = {
            let a697 = {
                let a659 = 1u64;
                let a660 = 1u64;
                nat_add(a659, a660)?
            };
            let a698 = {
                let a695 = {
                    let a693 = {
                        let a661 = 1u64;
                        let a662 = 1u64;
                        nat_add(a661, a662)?
                    };
                    let a694 = {
                        let a691 = {
                            let a689 = {
                                let a663 = 1u64;
                                let a664 = 1u64;
                                nat_add(a663, a664)?
                            };
                            let a690 = {
                                let a687 = {
                                    let a685 = {
                                        let a665 = 1u64;
                                        let a666 = 1u64;
                                        nat_add(a665, a666)?
                                    };
                                    let a686 = {
                                        let a683 = {
                                            let a681 = {
                                                let a667 = 1u64;
                                                let a668 = 1u64;
                                                nat_add(a667, a668)?
                                            };
                                            let a682 = {
                                                let a679 = {
                                                    let a677 = {
                                                        let a669 = 1u64;
                                                        let a670 = 1u64;
                                                        nat_add(a669, a670)?
                                                    };
                                                    let a678 = {
                                                        let a675 = {
                                                            let a673 = {
                                                                let a671 = 1u64;
                                                                let a672 = 1u64;
                                                                nat_add(a671, a672)?
                                                            };
                                                            let a674 = 1u64;
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
                    nat_mul(a693, a694)?
                };
                let a696 = 1u64;
                nat_add(a695, a696)?
            };
            nat_mul(a697, a698)?
        };
        let a700 = 1u64;
        nat_add(a699, a700)?
    };
    if nat_eq(a701, a702) {
        let a703 = 0u64;
        let a704 = f32(v0)?;
        Ok(nat_lt(a703, a704))
    } else {
        Ok(false)
    }
}

pub fn f25(v0: u64) -> R<bool> {
    let a747 = f17(v0)?;
    let a748 = {
        let a745 = {
            let a743 = {
                let a705 = 1u64;
                let a706 = 1u64;
                nat_add(a705, a706)?
            };
            let a744 = {
                let a741 = {
                    let a739 = {
                        let a707 = 1u64;
                        let a708 = 1u64;
                        nat_add(a707, a708)?
                    };
                    let a740 = {
                        let a737 = {
                            let a735 = {
                                let a709 = 1u64;
                                let a710 = 1u64;
                                nat_add(a709, a710)?
                            };
                            let a736 = {
                                let a733 = {
                                    let a731 = {
                                        let a711 = 1u64;
                                        let a712 = 1u64;
                                        nat_add(a711, a712)?
                                    };
                                    let a732 = {
                                        let a729 = {
                                            let a727 = {
                                                let a713 = 1u64;
                                                let a714 = 1u64;
                                                nat_add(a713, a714)?
                                            };
                                            let a728 = {
                                                let a725 = {
                                                    let a723 = {
                                                        let a715 = 1u64;
                                                        let a716 = 1u64;
                                                        nat_add(a715, a716)?
                                                    };
                                                    let a724 = {
                                                        let a721 = {
                                                            let a719 = {
                                                                let a717 = 1u64;
                                                                let a718 = 1u64;
                                                                nat_add(a717, a718)?
                                                            };
                                                            let a720 = 1u64;
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
    if nat_eq(a747, a748) {
        let a749 = f32(v0)?;
        let a750 = 0u64;
        Ok(nat_eq(a749, a750))
    } else {
        Ok(false)
    }
}

pub fn f26(v0: u64) -> R<u64> {
    let a817 = f28(v0)?;
    let a818 = {
        let a815 = {
            let a791 = {
                let a789 = {
                    let a751 = 1u64;
                    let a752 = 1u64;
                    nat_add(a751, a752)?
                };
                let a790 = {
                    let a787 = {
                        let a785 = {
                            let a753 = 1u64;
                            let a754 = 1u64;
                            nat_add(a753, a754)?
                        };
                        let a786 = {
                            let a783 = {
                                let a781 = {
                                    let a755 = 1u64;
                                    let a756 = 1u64;
                                    nat_add(a755, a756)?
                                };
                                let a782 = {
                                    let a779 = {
                                        let a777 = {
                                            let a757 = 1u64;
                                            let a758 = 1u64;
                                            nat_add(a757, a758)?
                                        };
                                        let a778 = {
                                            let a775 = {
                                                let a773 = {
                                                    let a759 = 1u64;
                                                    let a760 = 1u64;
                                                    nat_add(a759, a760)?
                                                };
                                                let a774 = {
                                                    let a771 = {
                                                        let a769 = {
                                                            let a761 = 1u64;
                                                            let a762 = 1u64;
                                                            nat_add(a761, a762)?
                                                        };
                                                        let a770 = {
                                                            let a767 = {
                                                                let a765 = {
                                                                    let a763 = 1u64;
                                                                    let a764 = 1u64;
                                                                    nat_add(a763, a764)?
                                                                };
                                                                let a766 = 1u64;
                                                                nat_mul(a765, a766)?
                                                            };
                                                            let a768 = 1u64;
                                                            nat_add(a767, a768)?
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
                                };
                                nat_mul(a781, a782)?
                            };
                            let a784 = 1u64;
                            nat_add(a783, a784)?
                        };
                        nat_mul(a785, a786)?
                    };
                    let a788 = 1u64;
                    nat_add(a787, a788)?
                };
                nat_mul(a789, a790)?
            };
            let a792 = 1u64;
            nat_add(a791, a792)?
        };
        let a816 = f20({
            let a813 = {
                let a811 = {
                    let a793 = 1u64;
                    let a794 = 1u64;
                    nat_add(a793, a794)?
                };
                let a812 = {
                    let a809 = {
                        let a807 = {
                            let a795 = 1u64;
                            let a796 = 1u64;
                            nat_add(a795, a796)?
                        };
                        let a808 = {
                            let a805 = {
                                let a803 = {
                                    let a797 = 1u64;
                                    let a798 = 1u64;
                                    nat_add(a797, a798)?
                                };
                                let a804 = {
                                    let a801 = {
                                        let a799 = 1u64;
                                        let a800 = 1u64;
                                        nat_add(a799, a800)?
                                    };
                                    let a802 = 1u64;
                                    nat_mul(a801, a802)?
                                };
                                nat_mul(a803, a804)?
                            };
                            let a806 = 1u64;
                            nat_add(a805, a806)?
                        };
                        nat_mul(a807, a808)?
                    };
                    let a810 = 1u64;
                    nat_add(a809, a810)?
                };
                nat_mul(a811, a812)?
            };
            let a814 = 1u64;
            nat_add(a813, a814)?
        })?;
        nat_mul(a815, a816)?
    };
    nat_add(a817, a818)
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let a825 = {
        let a819 = f30(v0)?;
        let a820 = f30(v1)?;
        nat_add(a819, a820)?
    };
    let a826 = {
        let a823 = {
            let a821 = 1u64;
            let a822 = 1u64;
            nat_add(a821, a822)?
        };
        let a824 = 1u64;
        nat_mul(a823, a824)?
    };
    let a827 = 0u64;
    Ok(nat_rem(a825, a826, a827))
}

pub fn f28(v0: u64) -> R<u64> {
    let a852 = v0;
    let a853 = f20({
        let a850 = {
            let a848 = {
                let a828 = 1u64;
                let a829 = 1u64;
                nat_add(a828, a829)?
            };
            let a849 = {
                let a846 = {
                    let a844 = {
                        let a830 = 1u64;
                        let a831 = 1u64;
                        nat_add(a830, a831)?
                    };
                    let a845 = {
                        let a842 = {
                            let a840 = {
                                let a832 = 1u64;
                                let a833 = 1u64;
                                nat_add(a832, a833)?
                            };
                            let a841 = {
                                let a838 = {
                                    let a836 = {
                                        let a834 = 1u64;
                                        let a835 = 1u64;
                                        nat_add(a834, a835)?
                                    };
                                    let a837 = 1u64;
                                    nat_mul(a836, a837)?
                                };
                                let a839 = 1u64;
                                nat_add(a838, a839)?
                            };
                            nat_mul(a840, a841)?
                        };
                        let a843 = 1u64;
                        nat_add(a842, a843)?
                    };
                    nat_mul(a844, a845)?
                };
                let a847 = 1u64;
                nat_add(a846, a847)?
            };
            nat_mul(a848, a849)?
        };
        let a851 = 1u64;
        nat_add(a850, a851)?
    })?;
    nat_mul(a852, a853)
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f36(v0)?;
    let v3: Adt2 = f36(v1)?;
    let v4: bool = {
        let a856 = match v2.clone() {
            Adt2::C0(_, h854, _, _) => {
                h854
            }
        };
        let a857 = match v3.clone() {
            Adt2::C0(_, h855, _, _) => {
                h855
            }
        };
        nat_lt(a856, a857)
    };
    let v5: u64 = {
        let a901 = match v2.clone() {
            Adt2::C0(_, h858, _, _) => {
                h858
            }
        };
        let a902 = f20(if v4 {
            let a879 = {
                let a877 = {
                    let a859 = 1u64;
                    let a860 = 1u64;
                    nat_add(a859, a860)?
                };
                let a878 = {
                    let a875 = {
                        let a873 = {
                            let a861 = 1u64;
                            let a862 = 1u64;
                            nat_add(a861, a862)?
                        };
                        let a874 = {
                            let a871 = {
                                let a863 = 1u64;
                                let a864 = 1u64;
                                nat_add(a863, a864)?
                            };
                            let a872 = {
                                let a869 = {
                                    let a867 = {
                                        let a865 = 1u64;
                                        let a866 = 1u64;
                                        nat_add(a865, a866)?
                                    };
                                    let a868 = 1u64;
                                    nat_mul(a867, a868)?
                                };
                                let a870 = 1u64;
                                nat_add(a869, a870)?
                            };
                            nat_mul(a871, a872)?
                        };
                        nat_mul(a873, a874)?
                    };
                    let a876 = 1u64;
                    nat_add(a875, a876)?
                };
                nat_mul(a877, a878)?
            };
            let a880 = 1u64;
            nat_add(a879, a880)?
        } else {
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
                            let a885 = 1u64;
                            let a886 = 1u64;
                            nat_add(a885, a886)?
                        };
                        let a894 = {
                            let a891 = {
                                let a889 = {
                                    let a887 = 1u64;
                                    let a888 = 1u64;
                                    nat_add(a887, a888)?
                                };
                                let a890 = 1u64;
                                nat_mul(a889, a890)?
                            };
                            let a892 = 1u64;
                            nat_add(a891, a892)?
                        };
                        nat_mul(a893, a894)?
                    };
                    nat_mul(a895, a896)?
                };
                let a898 = 1u64;
                nat_add(a897, a898)?
            };
            nat_mul(a899, a900)?
        })?;
        nat_mul(a901, a902)?
    };
    let v6: u64 = {
        let a959 = {
            let a957 = {
                let a954 = match v2 {
                    Adt2::C0(h903, _, _, _) => {
                        h903
                    }
                };
                let a955 = {
                    let a952 = {
                        let a950 = {
                            let a904 = 1u64;
                            let a905 = 1u64;
                            nat_add(a904, a905)?
                        };
                        let a951 = {
                            let a948 = {
                                let a946 = {
                                    let a906 = 1u64;
                                    let a907 = 1u64;
                                    nat_add(a906, a907)?
                                };
                                let a947 = {
                                    let a944 = {
                                        let a942 = {
                                            let a908 = 1u64;
                                            let a909 = 1u64;
                                            nat_add(a908, a909)?
                                        };
                                        let a943 = {
                                            let a940 = {
                                                let a938 = {
                                                    let a910 = 1u64;
                                                    let a911 = 1u64;
                                                    nat_add(a910, a911)?
                                                };
                                                let a939 = {
                                                    let a936 = {
                                                        let a934 = {
                                                            let a912 = 1u64;
                                                            let a913 = 1u64;
                                                            nat_add(a912, a913)?
                                                        };
                                                        let a935 = {
                                                            let a932 = {
                                                                let a930 = {
                                                                    let a914 = 1u64;
                                                                    let a915 = 1u64;
                                                                    nat_add(a914, a915)?
                                                                };
                                                                let a931 = {
                                                                    let a928 = {
                                                                        let a926 = {
                                                                            let a916 = 1u64;
                                                                            let a917 = 1u64;
                                                                            nat_add(a916, a917)?
                                                                        };
                                                                        let a927 = {
                                                                            let a924 = {
                                                                                let a918 = 1u64;
                                                                                let a919 = 1u64;
                                                                                nat_add(a918, a919)?
                                                                            };
                                                                            let a925 = {
                                                                                let a922 = {
                                                                                    let a920 = 1u64;
                                                                                    let a921 = 1u64;
                                                                                    nat_add(a920, a921)?
                                                                                };
                                                                                let a923 = 1u64;
                                                                                nat_mul(a922, a923)?
                                                                            };
                                                                            nat_mul(a924, a925)?
                                                                        };
                                                                        nat_mul(a926, a927)?
                                                                    };
                                                                    let a929 = 1u64;
                                                                    nat_add(a928, a929)?
                                                                };
                                                                nat_mul(a930, a931)?
                                                            };
                                                            let a933 = 1u64;
                                                            nat_add(a932, a933)?
                                                        };
                                                        nat_mul(a934, a935)?
                                                    };
                                                    let a937 = 1u64;
                                                    nat_add(a936, a937)?
                                                };
                                                nat_mul(a938, a939)?
                                            };
                                            let a941 = 1u64;
                                            nat_add(a940, a941)?
                                        };
                                        nat_mul(a942, a943)?
                                    };
                                    let a945 = 1u64;
                                    nat_add(a944, a945)?
                                };
                                nat_mul(a946, a947)?
                            };
                            let a949 = 1u64;
                            nat_add(a948, a949)?
                        };
                        nat_mul(a950, a951)?
                    };
                    let a953 = 1u64;
                    nat_add(a952, a953)?
                };
                nat_add(a954, a955)?
            };
            let a958 = match v3.clone() {
                Adt2::C0(h956, _, _, _) => {
                    h956
                }
            };
            nat_sub(a957, a958)
        };
        let a960 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a959, a960)
    };
    let v7: u64 = {
        let a962 = v5;
        let a963 = match v3.clone() {
            Adt2::C0(_, h961, _, _) => {
                h961
            }
        };
        let a964 = 0u64;
        nat_quot(a962, a963, a964)
    };
    let v8: bool = {
        let a969 = 0u64;
        let a970 = {
            let a966 = v5;
            let a967 = match v3 {
                Adt2::C0(_, h965, _, _) => {
                    h965
                }
            };
            let a968 = 0u64;
            nat_rem(a966, a967, a968)
        };
        nat_lt(a969, a970)
    };
    f37(f27(v0, v1)?, v6, f38(v7, v8)?)
}

pub fn f30(v0: u64) -> R<u64> {
    let a995 = v0;
    let a996 = f20({
        let a993 = {
            let a991 = {
                let a971 = 1u64;
                let a972 = 1u64;
                nat_add(a971, a972)?
            };
            let a992 = {
                let a989 = {
                    let a987 = {
                        let a973 = 1u64;
                        let a974 = 1u64;
                        nat_add(a973, a974)?
                    };
                    let a988 = {
                        let a985 = {
                            let a983 = {
                                let a975 = 1u64;
                                let a976 = 1u64;
                                nat_add(a975, a976)?
                            };
                            let a984 = {
                                let a981 = {
                                    let a979 = {
                                        let a977 = 1u64;
                                        let a978 = 1u64;
                                        nat_add(a977, a978)?
                                    };
                                    let a980 = 1u64;
                                    nat_mul(a979, a980)?
                                };
                                let a982 = 1u64;
                                nat_add(a981, a982)?
                            };
                            nat_mul(a983, a984)?
                        };
                        let a986 = 1u64;
                        nat_add(a985, a986)?
                    };
                    nat_mul(a987, a988)?
                };
                let a990 = 1u64;
                nat_add(a989, a990)?
            };
            nat_mul(a991, a992)?
        };
        let a994 = 1u64;
        nat_add(a993, a994)?
    })?;
    let a997 = 0u64;
    Ok(nat_quot(a995, a996, a997))
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f39(v0)?;
    let v3: u64 = {
        let a1010 = f14(v0)?;
        let a1011 = {
            let a1008 = {
                let a998 = 1u64;
                let a999 = 1u64;
                nat_add(a998, a999)?
            };
            let a1009 = {
                let a1006 = {
                    let a1000 = 1u64;
                    let a1001 = 1u64;
                    nat_add(a1000, a1001)?
                };
                let a1007 = {
                    let a1004 = {
                        let a1002 = 1u64;
                        let a1003 = 1u64;
                        nat_add(a1002, a1003)?
                    };
                    let a1005 = 1u64;
                    nat_mul(a1004, a1005)?
                };
                nat_mul(a1006, a1007)?
            };
            nat_mul(a1008, a1009)?
        };
        nat_mul(a1010, a1011)?
    };
    let v4: u64 = f40({
        let a1024 = f14(v1)?;
        let a1025 = {
            let a1022 = {
                let a1012 = 1u64;
                let a1013 = 1u64;
                nat_add(a1012, a1013)?
            };
            let a1023 = {
                let a1020 = {
                    let a1014 = 1u64;
                    let a1015 = 1u64;
                    nat_add(a1014, a1015)?
                };
                let a1021 = {
                    let a1018 = {
                        let a1016 = 1u64;
                        let a1017 = 1u64;
                        nat_add(a1016, a1017)?
                    };
                    let a1019 = 1u64;
                    nat_mul(a1018, a1019)?
                };
                nat_mul(a1020, a1021)?
            };
            nat_mul(a1022, a1023)?
        };
        nat_mul(a1024, a1025)?
    }, {
        let a1026 = v2;
        let a1027 = f39(v1)?;
        nat_sub(a1026, a1027)
    })?;
    let a1028 = f30(v0)?;
    let a1029 = f30(v1)?;
    if nat_eq(a1028, a1029) {
        f37(f30(v0)?, v2, {
            let a1030 = v3;
            let a1031 = v4;
            nat_add(a1030, a1031)?
        })
    } else {
        let a1032 = v3;
        let a1033 = v4;
        if nat_eq(a1032, a1033) {
            Ok(0u64)
        } else {
            f37(f30(v0)?, v2, {
                let a1034 = v3;
                let a1035 = v4;
                nat_sub(a1034, a1035)
            })
        }
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1058 = v0;
    let a1059 = f20({
        let a1056 = {
            let a1054 = {
                let a1036 = 1u64;
                let a1037 = 1u64;
                nat_add(a1036, a1037)?
            };
            let a1055 = {
                let a1052 = {
                    let a1050 = {
                        let a1038 = 1u64;
                        let a1039 = 1u64;
                        nat_add(a1038, a1039)?
                    };
                    let a1051 = {
                        let a1048 = {
                            let a1046 = {
                                let a1040 = 1u64;
                                let a1041 = 1u64;
                                nat_add(a1040, a1041)?
                            };
                            let a1047 = {
                                let a1044 = {
                                    let a1042 = 1u64;
                                    let a1043 = 1u64;
                                    nat_add(a1042, a1043)?
                                };
                                let a1045 = 1u64;
                                nat_mul(a1044, a1045)?
                            };
                            nat_mul(a1046, a1047)?
                        };
                        let a1049 = 1u64;
                        nat_add(a1048, a1049)?
                    };
                    nat_mul(a1050, a1051)?
                };
                let a1053 = 1u64;
                nat_add(a1052, a1053)?
            };
            nat_mul(a1054, a1055)?
        };
        let a1057 = 1u64;
        nat_add(a1056, a1057)?
    })?;
    let a1060 = 0u64;
    Ok(nat_rem(a1058, a1059, a1060))
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a1061 = v0;
        let a1062 = v1;
        let a1063 = 0u64;
        nat_quot(a1061, a1062, a1063)
    };
    let v3: u64 = {
        let a1067 = {
            let a1064 = v0;
            let a1065 = v1;
            let a1066 = 0u64;
            nat_rem(a1064, a1065, a1066)
        };
        let a1068 = 2u64;
        nat_mul(a1067, a1068)?
    };
    let a1078 = v2;
    let a1079 = {
        let a1069 = v1;
        let a1070 = v3;
        if if nat_lt(a1069, a1070) {
            true
        } else {
            let a1071 = v3;
            let a1072 = v1;
            if nat_eq(a1071, a1072) {
                let a1076 = {
                    let a1073 = v2;
                    let a1074 = 2u64;
                    let a1075 = 0u64;
                    nat_rem(a1073, a1074, a1075)
                };
                let a1077 = 1u64;
                nat_eq(a1076, a1077)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1078, a1079)
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f36(v0)?;
    let v3: Adt2 = f36(v1)?;
    f37(f27(v0, v1)?, {
        let a1134 = {
            let a1082 = match v2.clone() {
                Adt2::C0(h1080, _, _, _) => {
                    h1080
                }
            };
            let a1083 = match v3.clone() {
                Adt2::C0(h1081, _, _, _) => {
                    h1081
                }
            };
            nat_add(a1082, a1083)?
        };
        let a1135 = {
            let a1132 = {
                let a1130 = {
                    let a1084 = 1u64;
                    let a1085 = 1u64;
                    nat_add(a1084, a1085)?
                };
                let a1131 = {
                    let a1128 = {
                        let a1126 = {
                            let a1086 = 1u64;
                            let a1087 = 1u64;
                            nat_add(a1086, a1087)?
                        };
                        let a1127 = {
                            let a1124 = {
                                let a1122 = {
                                    let a1088 = 1u64;
                                    let a1089 = 1u64;
                                    nat_add(a1088, a1089)?
                                };
                                let a1123 = {
                                    let a1120 = {
                                        let a1118 = {
                                            let a1090 = 1u64;
                                            let a1091 = 1u64;
                                            nat_add(a1090, a1091)?
                                        };
                                        let a1119 = {
                                            let a1116 = {
                                                let a1114 = {
                                                    let a1092 = 1u64;
                                                    let a1093 = 1u64;
                                                    nat_add(a1092, a1093)?
                                                };
                                                let a1115 = {
                                                    let a1112 = {
                                                        let a1110 = {
                                                            let a1094 = 1u64;
                                                            let a1095 = 1u64;
                                                            nat_add(a1094, a1095)?
                                                        };
                                                        let a1111 = {
                                                            let a1108 = {
                                                                let a1106 = {
                                                                    let a1096 = 1u64;
                                                                    let a1097 = 1u64;
                                                                    nat_add(a1096, a1097)?
                                                                };
                                                                let a1107 = {
                                                                    let a1104 = {
                                                                        let a1098 = 1u64;
                                                                        let a1099 = 1u64;
                                                                        nat_add(a1098, a1099)?
                                                                    };
                                                                    let a1105 = {
                                                                        let a1102 = {
                                                                            let a1100 = 1u64;
                                                                            let a1101 = 1u64;
                                                                            nat_add(a1100, a1101)?
                                                                        };
                                                                        let a1103 = 1u64;
                                                                        nat_mul(a1102, a1103)?
                                                                    };
                                                                    nat_mul(a1104, a1105)?
                                                                };
                                                                nat_mul(a1106, a1107)?
                                                            };
                                                            let a1109 = 1u64;
                                                            nat_add(a1108, a1109)?
                                                        };
                                                        nat_mul(a1110, a1111)?
                                                    };
                                                    let a1113 = 1u64;
                                                    nat_add(a1112, a1113)?
                                                };
                                                nat_mul(a1114, a1115)?
                                            };
                                            let a1117 = 1u64;
                                            nat_add(a1116, a1117)?
                                        };
                                        nat_mul(a1118, a1119)?
                                    };
                                    let a1121 = 1u64;
                                    nat_add(a1120, a1121)?
                                };
                                nat_mul(a1122, a1123)?
                            };
                            let a1125 = 1u64;
                            nat_add(a1124, a1125)?
                        };
                        nat_mul(a1126, a1127)?
                    };
                    let a1129 = 1u64;
                    nat_add(a1128, a1129)?
                };
                nat_mul(a1130, a1131)?
            };
            let a1133 = 1u64;
            nat_add(a1132, a1133)?
        };
        nat_sub(a1134, a1135)
    }, f40({
        let a1138 = match v2 {
            Adt2::C0(_, h1136, _, _) => {
                h1136
            }
        };
        let a1139 = match v3 {
            Adt2::C0(_, h1137, _, _) => {
                h1137
            }
        };
        nat_mul(a1138, a1139)?
    }, {
        let a1156 = {
            let a1140 = 1u64;
            let a1141 = 1u64;
            nat_add(a1140, a1141)?
        };
        let a1157 = {
            let a1154 = {
                let a1142 = 1u64;
                let a1143 = 1u64;
                nat_add(a1142, a1143)?
            };
            let a1155 = {
                let a1152 = {
                    let a1150 = {
                        let a1144 = 1u64;
                        let a1145 = 1u64;
                        nat_add(a1144, a1145)?
                    };
                    let a1151 = {
                        let a1148 = {
                            let a1146 = 1u64;
                            let a1147 = 1u64;
                            nat_add(a1146, a1147)?
                        };
                        let a1149 = 1u64;
                        nat_mul(a1148, a1149)?
                    };
                    nat_mul(a1150, a1151)?
                };
                let a1153 = 1u64;
                nat_add(a1152, a1153)?
            };
            nat_mul(a1154, a1155)?
        };
        nat_mul(a1156, a1157)?
    })?)
}

pub fn f35(v0: u64) -> R<u64> {
    let a1158 = v0;
    let a1159 = v0;
    nat_add(a1158, a1159)
}

pub fn f36(v0: u64) -> R<Adt2> {
    f41(f39(v0)?, f14(v0)?, 0u64, f20({
        let a1180 = {
            let a1178 = {
                let a1160 = 1u64;
                let a1161 = 1u64;
                nat_add(a1160, a1161)?
            };
            let a1179 = {
                let a1176 = {
                    let a1174 = {
                        let a1162 = 1u64;
                        let a1163 = 1u64;
                        nat_add(a1162, a1163)?
                    };
                    let a1175 = {
                        let a1172 = {
                            let a1170 = {
                                let a1164 = 1u64;
                                let a1165 = 1u64;
                                nat_add(a1164, a1165)?
                            };
                            let a1171 = {
                                let a1168 = {
                                    let a1166 = 1u64;
                                    let a1167 = 1u64;
                                    nat_add(a1166, a1167)?
                                };
                                let a1169 = 1u64;
                                nat_mul(a1168, a1169)?
                            };
                            nat_mul(a1170, a1171)?
                        };
                        let a1173 = 1u64;
                        nat_add(a1172, a1173)?
                    };
                    nat_mul(a1174, a1175)?
                };
                let a1177 = 1u64;
                nat_add(a1176, a1177)?
            };
            nat_mul(a1178, a1179)?
        };
        let a1181 = 1u64;
        nat_add(a1180, a1181)?
    })?)
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1204 = f20({
        let a1202 = {
            let a1200 = {
                let a1182 = 1u64;
                let a1183 = 1u64;
                nat_add(a1182, a1183)?
            };
            let a1201 = {
                let a1198 = {
                    let a1196 = {
                        let a1184 = 1u64;
                        let a1185 = 1u64;
                        nat_add(a1184, a1185)?
                    };
                    let a1197 = {
                        let a1194 = {
                            let a1186 = 1u64;
                            let a1187 = 1u64;
                            nat_add(a1186, a1187)?
                        };
                        let a1195 = {
                            let a1192 = {
                                let a1190 = {
                                    let a1188 = 1u64;
                                    let a1189 = 1u64;
                                    nat_add(a1188, a1189)?
                                };
                                let a1191 = 1u64;
                                nat_mul(a1190, a1191)?
                            };
                            let a1193 = 1u64;
                            nat_add(a1192, a1193)?
                        };
                        nat_mul(a1194, a1195)?
                    };
                    nat_mul(a1196, a1197)?
                };
                let a1199 = 1u64;
                nat_add(a1198, a1199)?
            };
            nat_mul(a1200, a1201)?
        };
        let a1203 = 1u64;
        nat_add(a1202, a1203)?
    })?;
    let a1205 = v2;
    if nat_le(a1204, a1205) {
        f42(v0, {
            let a1206 = v1;
            let a1207 = 1u64;
            nat_add(a1206, a1207)?
        }, f40(v2, 1u64)?)
    } else {
        let v3: Adt2 = f41(v1, v2, {
            let a1244 = {
                let a1242 = {
                    let a1208 = 1u64;
                    let a1209 = 1u64;
                    nat_add(a1208, a1209)?
                };
                let a1243 = {
                    let a1240 = {
                        let a1210 = 1u64;
                        let a1211 = 1u64;
                        nat_add(a1210, a1211)?
                    };
                    let a1241 = {
                        let a1238 = {
                            let a1212 = 1u64;
                            let a1213 = 1u64;
                            nat_add(a1212, a1213)?
                        };
                        let a1239 = {
                            let a1236 = {
                                let a1214 = 1u64;
                                let a1215 = 1u64;
                                nat_add(a1214, a1215)?
                            };
                            let a1237 = {
                                let a1234 = {
                                    let a1216 = 1u64;
                                    let a1217 = 1u64;
                                    nat_add(a1216, a1217)?
                                };
                                let a1235 = {
                                    let a1232 = {
                                        let a1218 = 1u64;
                                        let a1219 = 1u64;
                                        nat_add(a1218, a1219)?
                                    };
                                    let a1233 = {
                                        let a1230 = {
                                            let a1220 = 1u64;
                                            let a1221 = 1u64;
                                            nat_add(a1220, a1221)?
                                        };
                                        let a1231 = {
                                            let a1228 = {
                                                let a1222 = 1u64;
                                                let a1223 = 1u64;
                                                nat_add(a1222, a1223)?
                                            };
                                            let a1229 = {
                                                let a1226 = {
                                                    let a1224 = 1u64;
                                                    let a1225 = 1u64;
                                                    nat_add(a1224, a1225)?
                                                };
                                                let a1227 = 1u64;
                                                nat_mul(a1226, a1227)?
                                            };
                                            nat_mul(a1228, a1229)?
                                        };
                                        nat_mul(a1230, a1231)?
                                    };
                                    nat_mul(a1232, a1233)?
                                };
                                nat_mul(a1234, a1235)?
                            };
                            nat_mul(a1236, a1237)?
                        };
                        nat_mul(a1238, a1239)?
                    };
                    nat_mul(a1240, a1241)?
                };
                nat_mul(a1242, a1243)?
            };
            let a1245 = 1u64;
            nat_add(a1244, a1245)?
        }, f20({
            let a1264 = {
                let a1246 = 1u64;
                let a1247 = 1u64;
                nat_add(a1246, a1247)?
            };
            let a1265 = {
                let a1262 = {
                    let a1260 = {
                        let a1248 = 1u64;
                        let a1249 = 1u64;
                        nat_add(a1248, a1249)?
                    };
                    let a1261 = {
                        let a1258 = {
                            let a1250 = 1u64;
                            let a1251 = 1u64;
                            nat_add(a1250, a1251)?
                        };
                        let a1259 = {
                            let a1256 = {
                                let a1254 = {
                                    let a1252 = 1u64;
                                    let a1253 = 1u64;
                                    nat_add(a1252, a1253)?
                                };
                                let a1255 = 1u64;
                                nat_mul(a1254, a1255)?
                            };
                            let a1257 = 1u64;
                            nat_add(a1256, a1257)?
                        };
                        nat_mul(a1258, a1259)?
                    };
                    nat_mul(a1260, a1261)?
                };
                let a1263 = 1u64;
                nat_add(a1262, a1263)?
            };
            nat_mul(a1264, a1265)?
        })?)?;
        f42(v0, match v3.clone() {
            Adt2::C0(h1266, _, _, _) => {
                h1266
            }
        }, match v3 {
            Adt2::C0(_, h1267, _, _) => {
                h1267
            }
        })
    }
}

pub fn f38(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1275 = {
            let a1272 = v0;
            let a1273 = {
                let a1270 = {
                    let a1268 = 1u64;
                    let a1269 = 1u64;
                    nat_add(a1268, a1269)?
                };
                let a1271 = 1u64;
                nat_mul(a1270, a1271)?
            };
            let a1274 = 0u64;
            nat_rem(a1272, a1273, a1274)
        };
        let a1276 = 0u64;
        nat_eq(a1275, a1276)
    } else {
        false
    } {
        let a1277 = v0;
        let a1278 = 1u64;
        nat_add(a1277, a1278)
    } else {
        Ok(v0)
    }
}

pub fn f39(v0: u64) -> R<u64> {
    let a1317 = {
        let a1279 = f17(v0)?;
        let a1280 = 0u64;
        if nat_eq(a1279, a1280) {
            1u64
        } else {
            f17(v0)?
        }
    };
    let a1318 = {
        let a1315 = {
            let a1281 = 1u64;
            let a1282 = 1u64;
            nat_add(a1281, a1282)?
        };
        let a1316 = {
            let a1313 = {
                let a1283 = 1u64;
                let a1284 = 1u64;
                nat_add(a1283, a1284)?
            };
            let a1314 = {
                let a1311 = {
                    let a1285 = 1u64;
                    let a1286 = 1u64;
                    nat_add(a1285, a1286)?
                };
                let a1312 = {
                    let a1309 = {
                        let a1287 = 1u64;
                        let a1288 = 1u64;
                        nat_add(a1287, a1288)?
                    };
                    let a1310 = {
                        let a1307 = {
                            let a1289 = 1u64;
                            let a1290 = 1u64;
                            nat_add(a1289, a1290)?
                        };
                        let a1308 = {
                            let a1305 = {
                                let a1291 = 1u64;
                                let a1292 = 1u64;
                                nat_add(a1291, a1292)?
                            };
                            let a1306 = {
                                let a1303 = {
                                    let a1293 = 1u64;
                                    let a1294 = 1u64;
                                    nat_add(a1293, a1294)?
                                };
                                let a1304 = {
                                    let a1301 = {
                                        let a1295 = 1u64;
                                        let a1296 = 1u64;
                                        nat_add(a1295, a1296)?
                                    };
                                    let a1302 = {
                                        let a1299 = {
                                            let a1297 = 1u64;
                                            let a1298 = 1u64;
                                            nat_add(a1297, a1298)?
                                        };
                                        let a1300 = 1u64;
                                        nat_mul(a1299, a1300)?
                                    };
                                    nat_mul(a1301, a1302)?
                                };
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
    nat_add(a1317, a1318)
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let a1319 = v1;
    let a1320 = 0u64;
    if nat_eq(a1319, a1320) {
        Ok(v0)
    } else {
        let a1345 = {
            let a1343 = {
                let a1321 = 1u64;
                let a1322 = 1u64;
                nat_add(a1321, a1322)?
            };
            let a1344 = {
                let a1341 = {
                    let a1323 = 1u64;
                    let a1324 = 1u64;
                    nat_add(a1323, a1324)?
                };
                let a1342 = {
                    let a1339 = {
                        let a1325 = 1u64;
                        let a1326 = 1u64;
                        nat_add(a1325, a1326)?
                    };
                    let a1340 = {
                        let a1337 = {
                            let a1327 = 1u64;
                            let a1328 = 1u64;
                            nat_add(a1327, a1328)?
                        };
                        let a1338 = {
                            let a1335 = {
                                let a1329 = 1u64;
                                let a1330 = 1u64;
                                nat_add(a1329, a1330)?
                            };
                            let a1336 = {
                                let a1333 = {
                                    let a1331 = 1u64;
                                    let a1332 = 1u64;
                                    nat_add(a1331, a1332)?
                                };
                                let a1334 = 1u64;
                                nat_mul(a1333, a1334)?
                            };
                            nat_mul(a1335, a1336)?
                        };
                        nat_mul(a1337, a1338)?
                    };
                    nat_mul(a1339, a1340)?
                };
                nat_mul(a1341, a1342)?
            };
            nat_mul(a1343, a1344)?
        };
        let a1346 = v1;
        if nat_le(a1345, a1346) {
            let a1347 = v0;
            let a1348 = 0u64;
            if nat_eq(a1347, a1348) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f20(v1)?;
            f38({
                let a1349 = v0;
                let a1350 = v2;
                let a1351 = 0u64;
                nat_quot(a1349, a1350, a1351)
            }, {
                let a1355 = 0u64;
                let a1356 = {
                    let a1352 = v0;
                    let a1353 = v2;
                    let a1354 = 0u64;
                    nat_rem(a1352, a1353, a1354)
                };
                nat_lt(a1355, a1356)
            })
        }
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h1357, _) = f43(Fn1::F44, {
        let a1376 = {
            let a1358 = 1u64;
            let a1359 = 1u64;
            nat_add(a1358, a1359)?
        };
        let a1377 = {
            let a1374 = {
                let a1360 = 1u64;
                let a1361 = 1u64;
                nat_add(a1360, a1361)?
            };
            let a1375 = {
                let a1372 = {
                    let a1362 = 1u64;
                    let a1363 = 1u64;
                    nat_add(a1362, a1363)?
                };
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
                nat_mul(a1372, a1373)?
            };
            nat_mul(a1374, a1375)?
        };
        nat_mul(a1376, a1377)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h1357)
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1416 = v1;
    let a1417 = {
        let a1414 = {
            let a1412 = {
                let a1378 = 1u64;
                let a1379 = 1u64;
                nat_add(a1378, a1379)?
            };
            let a1413 = {
                let a1410 = {
                    let a1380 = 1u64;
                    let a1381 = 1u64;
                    nat_add(a1380, a1381)?
                };
                let a1411 = {
                    let a1408 = {
                        let a1382 = 1u64;
                        let a1383 = 1u64;
                        nat_add(a1382, a1383)?
                    };
                    let a1409 = {
                        let a1406 = {
                            let a1384 = 1u64;
                            let a1385 = 1u64;
                            nat_add(a1384, a1385)?
                        };
                        let a1407 = {
                            let a1404 = {
                                let a1386 = 1u64;
                                let a1387 = 1u64;
                                nat_add(a1386, a1387)?
                            };
                            let a1405 = {
                                let a1402 = {
                                    let a1388 = 1u64;
                                    let a1389 = 1u64;
                                    nat_add(a1388, a1389)?
                                };
                                let a1403 = {
                                    let a1400 = {
                                        let a1390 = 1u64;
                                        let a1391 = 1u64;
                                        nat_add(a1390, a1391)?
                                    };
                                    let a1401 = {
                                        let a1398 = {
                                            let a1392 = 1u64;
                                            let a1393 = 1u64;
                                            nat_add(a1392, a1393)?
                                        };
                                        let a1399 = {
                                            let a1396 = {
                                                let a1394 = 1u64;
                                                let a1395 = 1u64;
                                                nat_add(a1394, a1395)?
                                            };
                                            let a1397 = 1u64;
                                            nat_mul(a1396, a1397)?
                                        };
                                        nat_mul(a1398, a1399)?
                                    };
                                    nat_mul(a1400, a1401)?
                                };
                                nat_mul(a1402, a1403)?
                            };
                            nat_mul(a1404, a1405)?
                        };
                        nat_mul(a1406, a1407)?
                    };
                    nat_mul(a1408, a1409)?
                };
                nat_mul(a1410, a1411)?
            };
            nat_mul(a1412, a1413)?
        };
        let a1415 = 1u64;
        nat_add(a1414, a1415)?
    };
    if nat_lt(a1416, a1417) {
        f45(v0, {
            let a1454 = {
                let a1452 = {
                    let a1418 = 1u64;
                    let a1419 = 1u64;
                    nat_add(a1418, a1419)?
                };
                let a1453 = {
                    let a1450 = {
                        let a1420 = 1u64;
                        let a1421 = 1u64;
                        nat_add(a1420, a1421)?
                    };
                    let a1451 = {
                        let a1448 = {
                            let a1422 = 1u64;
                            let a1423 = 1u64;
                            nat_add(a1422, a1423)?
                        };
                        let a1449 = {
                            let a1446 = {
                                let a1424 = 1u64;
                                let a1425 = 1u64;
                                nat_add(a1424, a1425)?
                            };
                            let a1447 = {
                                let a1444 = {
                                    let a1426 = 1u64;
                                    let a1427 = 1u64;
                                    nat_add(a1426, a1427)?
                                };
                                let a1445 = {
                                    let a1442 = {
                                        let a1428 = 1u64;
                                        let a1429 = 1u64;
                                        nat_add(a1428, a1429)?
                                    };
                                    let a1443 = {
                                        let a1440 = {
                                            let a1430 = 1u64;
                                            let a1431 = 1u64;
                                            nat_add(a1430, a1431)?
                                        };
                                        let a1441 = {
                                            let a1438 = {
                                                let a1432 = 1u64;
                                                let a1433 = 1u64;
                                                nat_add(a1432, a1433)?
                                            };
                                            let a1439 = {
                                                let a1436 = {
                                                    let a1434 = 1u64;
                                                    let a1435 = 1u64;
                                                    nat_add(a1434, a1435)?
                                                };
                                                let a1437 = 1u64;
                                                nat_mul(a1436, a1437)?
                                            };
                                            nat_mul(a1438, a1439)?
                                        };
                                        nat_mul(a1440, a1441)?
                                    };
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
            let a1455 = 1u64;
            nat_add(a1454, a1455)?
        }, f40(v2, {
            let a1494 = {
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
            let a1495 = v1;
            nat_sub(a1494, a1495)
        })?)
    } else {
        f45(v0, v1, v2)
    }
}

pub fn f43(v0: Fn1, p1500: u64, p1501: Adt2) -> R<(Adt2, bool)> {
    let m1499 = (p1500, p1501);
    {
        let mut m1499 = m1499;
        loop {
            let step = {
                let (v1, v2) = m1499;
                let m1496 = v1;
                if m1496 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1496 - 1;
                    let m1498 = {
                        let c1497: Fn1 = v0.clone();
                        c1497.apply(v2.clone())?
                    };
                    match m1498 {
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
                Ok(next) => { m1499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Adt2) -> R<Option<Adt2>> {
    f46(v0)
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1514 = v2;
        let a1515 = {
            let a1512 = {
                let a1502 = 1u64;
                let a1503 = 1u64;
                nat_add(a1502, a1503)?
            };
            let a1513 = {
                let a1510 = {
                    let a1504 = 1u64;
                    let a1505 = 1u64;
                    nat_add(a1504, a1505)?
                };
                let a1511 = {
                    let a1508 = {
                        let a1506 = 1u64;
                        let a1507 = 1u64;
                        nat_add(a1506, a1507)?
                    };
                    let a1509 = 1u64;
                    nat_mul(a1508, a1509)?
                };
                nat_mul(a1510, a1511)?
            };
            nat_mul(a1512, a1513)?
        };
        let a1516 = 0u64;
        nat_rem(a1514, a1515, a1516)
    };
    let v4: u64 = {
        let a1529 = v2;
        let a1530 = {
            let a1527 = {
                let a1517 = 1u64;
                let a1518 = 1u64;
                nat_add(a1517, a1518)?
            };
            let a1528 = {
                let a1525 = {
                    let a1519 = 1u64;
                    let a1520 = 1u64;
                    nat_add(a1519, a1520)?
                };
                let a1526 = {
                    let a1523 = {
                        let a1521 = 1u64;
                        let a1522 = 1u64;
                        nat_add(a1521, a1522)?
                    };
                    let a1524 = 1u64;
                    nat_mul(a1523, a1524)?
                };
                nat_mul(a1525, a1526)?
            };
            nat_mul(a1527, a1528)?
        };
        let a1531 = 0u64;
        nat_quot(a1529, a1530, a1531)
    };
    let v5: bool = {
        let a1540 = {
            let a1538 = {
                let a1532 = 1u64;
                let a1533 = 1u64;
                nat_add(a1532, a1533)?
            };
            let a1539 = {
                let a1536 = {
                    let a1534 = 1u64;
                    let a1535 = 1u64;
                    nat_add(a1534, a1535)?
                };
                let a1537 = 1u64;
                nat_mul(a1536, a1537)?
            };
            nat_mul(a1538, a1539)?
        };
        let a1541 = v3;
        if nat_lt(a1540, a1541) {
            true
        } else {
            let a1550 = v3;
            let a1551 = {
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
            if nat_eq(a1550, a1551) {
                let a1559 = {
                    let a1556 = v4;
                    let a1557 = {
                        let a1554 = {
                            let a1552 = 1u64;
                            let a1553 = 1u64;
                            nat_add(a1552, a1553)?
                        };
                        let a1555 = 1u64;
                        nat_mul(a1554, a1555)?
                    };
                    let a1558 = 0u64;
                    nat_rem(a1556, a1557, a1558)
                };
                let a1560 = 1u64;
                nat_eq(a1559, a1560)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1561 = v4;
        let a1562 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1561, a1562)?
    };
    let v7: bool = {
        let a1591 = {
            let a1589 = {
                let a1565 = {
                    let a1563 = 1u64;
                    let a1564 = 1u64;
                    nat_add(a1563, a1564)?
                };
                let a1566 = 1u64;
                nat_mul(a1565, a1566)?
            };
            let a1590 = f20({
                let a1587 = {
                    let a1585 = {
                        let a1567 = 1u64;
                        let a1568 = 1u64;
                        nat_add(a1567, a1568)?
                    };
                    let a1586 = {
                        let a1583 = {
                            let a1581 = {
                                let a1569 = 1u64;
                                let a1570 = 1u64;
                                nat_add(a1569, a1570)?
                            };
                            let a1582 = {
                                let a1579 = {
                                    let a1577 = {
                                        let a1571 = 1u64;
                                        let a1572 = 1u64;
                                        nat_add(a1571, a1572)?
                                    };
                                    let a1578 = {
                                        let a1575 = {
                                            let a1573 = 1u64;
                                            let a1574 = 1u64;
                                            nat_add(a1573, a1574)?
                                        };
                                        let a1576 = 1u64;
                                        nat_mul(a1575, a1576)?
                                    };
                                    nat_mul(a1577, a1578)?
                                };
                                let a1580 = 1u64;
                                nat_add(a1579, a1580)?
                            };
                            nat_mul(a1581, a1582)?
                        };
                        let a1584 = 1u64;
                        nat_add(a1583, a1584)?
                    };
                    nat_mul(a1585, a1586)?
                };
                let a1588 = 1u64;
                nat_add(a1587, a1588)?
            })?;
            nat_mul(a1589, a1590)?
        };
        let a1592 = v6;
        nat_le(a1591, a1592)
    };
    let v8: u64 = if v7 {
        let a1597 = v6;
        let a1598 = {
            let a1595 = {
                let a1593 = 1u64;
                let a1594 = 1u64;
                nat_add(a1593, a1594)?
            };
            let a1596 = 1u64;
            nat_mul(a1595, a1596)?
        };
        let a1599 = 0u64;
        nat_quot(a1597, a1598, a1599)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1600 = v1;
        let a1601 = 1u64;
        nat_add(a1600, a1601)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1640 = v9;
        let a1641 = {
            let a1638 = {
                let a1636 = {
                    let a1602 = 1u64;
                    let a1603 = 1u64;
                    nat_add(a1602, a1603)?
                };
                let a1637 = {
                    let a1634 = {
                        let a1604 = 1u64;
                        let a1605 = 1u64;
                        nat_add(a1604, a1605)?
                    };
                    let a1635 = {
                        let a1632 = {
                            let a1606 = 1u64;
                            let a1607 = 1u64;
                            nat_add(a1606, a1607)?
                        };
                        let a1633 = {
                            let a1630 = {
                                let a1608 = 1u64;
                                let a1609 = 1u64;
                                nat_add(a1608, a1609)?
                            };
                            let a1631 = {
                                let a1628 = {
                                    let a1610 = 1u64;
                                    let a1611 = 1u64;
                                    nat_add(a1610, a1611)?
                                };
                                let a1629 = {
                                    let a1626 = {
                                        let a1612 = 1u64;
                                        let a1613 = 1u64;
                                        nat_add(a1612, a1613)?
                                    };
                                    let a1627 = {
                                        let a1624 = {
                                            let a1614 = 1u64;
                                            let a1615 = 1u64;
                                            nat_add(a1614, a1615)?
                                        };
                                        let a1625 = {
                                            let a1622 = {
                                                let a1616 = 1u64;
                                                let a1617 = 1u64;
                                                nat_add(a1616, a1617)?
                                            };
                                            let a1623 = {
                                                let a1620 = {
                                                    let a1618 = 1u64;
                                                    let a1619 = 1u64;
                                                    nat_add(a1618, a1619)?
                                                };
                                                let a1621 = 1u64;
                                                nat_mul(a1620, a1621)?
                                            };
                                            nat_mul(a1622, a1623)?
                                        };
                                        nat_mul(a1624, a1625)?
                                    };
                                    nat_mul(a1626, a1627)?
                                };
                                nat_mul(a1628, a1629)?
                            };
                            nat_mul(a1630, a1631)?
                        };
                        nat_mul(a1632, a1633)?
                    };
                    nat_mul(a1634, a1635)?
                };
                nat_mul(a1636, a1637)?
            };
            let a1639 = 1u64;
            nat_add(a1638, a1639)?
        };
        if if nat_eq(a1640, a1641) {
            let a1664 = v8;
            let a1665 = f20({
                let a1662 = {
                    let a1660 = {
                        let a1642 = 1u64;
                        let a1643 = 1u64;
                        nat_add(a1642, a1643)?
                    };
                    let a1661 = {
                        let a1658 = {
                            let a1656 = {
                                let a1644 = 1u64;
                                let a1645 = 1u64;
                                nat_add(a1644, a1645)?
                            };
                            let a1657 = {
                                let a1654 = {
                                    let a1652 = {
                                        let a1646 = 1u64;
                                        let a1647 = 1u64;
                                        nat_add(a1646, a1647)?
                                    };
                                    let a1653 = {
                                        let a1650 = {
                                            let a1648 = 1u64;
                                            let a1649 = 1u64;
                                            nat_add(a1648, a1649)?
                                        };
                                        let a1651 = 1u64;
                                        nat_mul(a1650, a1651)?
                                    };
                                    nat_mul(a1652, a1653)?
                                };
                                let a1655 = 1u64;
                                nat_add(a1654, a1655)?
                            };
                            nat_mul(a1656, a1657)?
                        };
                        let a1659 = 1u64;
                        nat_add(a1658, a1659)?
                    };
                    nat_mul(a1660, a1661)?
                };
                let a1663 = 1u64;
                nat_add(a1662, a1663)?
            })?;
            nat_lt(a1664, a1665)
        } else {
            false
        } {
            0u64
        } else {
            let a1702 = v9;
            let a1703 = {
                let a1700 = {
                    let a1666 = 1u64;
                    let a1667 = 1u64;
                    nat_add(a1666, a1667)?
                };
                let a1701 = {
                    let a1698 = {
                        let a1668 = 1u64;
                        let a1669 = 1u64;
                        nat_add(a1668, a1669)?
                    };
                    let a1699 = {
                        let a1696 = {
                            let a1670 = 1u64;
                            let a1671 = 1u64;
                            nat_add(a1670, a1671)?
                        };
                        let a1697 = {
                            let a1694 = {
                                let a1672 = 1u64;
                                let a1673 = 1u64;
                                nat_add(a1672, a1673)?
                            };
                            let a1695 = {
                                let a1692 = {
                                    let a1674 = 1u64;
                                    let a1675 = 1u64;
                                    nat_add(a1674, a1675)?
                                };
                                let a1693 = {
                                    let a1690 = {
                                        let a1676 = 1u64;
                                        let a1677 = 1u64;
                                        nat_add(a1676, a1677)?
                                    };
                                    let a1691 = {
                                        let a1688 = {
                                            let a1678 = 1u64;
                                            let a1679 = 1u64;
                                            nat_add(a1678, a1679)?
                                        };
                                        let a1689 = {
                                            let a1686 = {
                                                let a1680 = 1u64;
                                                let a1681 = 1u64;
                                                nat_add(a1680, a1681)?
                                            };
                                            let a1687 = {
                                                let a1684 = {
                                                    let a1682 = 1u64;
                                                    let a1683 = 1u64;
                                                    nat_add(a1682, a1683)?
                                                };
                                                let a1685 = 1u64;
                                                nat_mul(a1684, a1685)?
                                            };
                                            nat_mul(a1686, a1687)?
                                        };
                                        nat_mul(a1688, a1689)?
                                    };
                                    nat_mul(a1690, a1691)?
                                };
                                nat_mul(a1692, a1693)?
                            };
                            nat_mul(a1694, a1695)?
                        };
                        nat_mul(a1696, a1697)?
                    };
                    nat_mul(a1698, a1699)?
                };
                nat_mul(a1700, a1701)?
            };
            nat_sub(a1702, a1703)
        }
    };
    let a1704 = v8;
    let a1705 = 0u64;
    if nat_eq(a1704, a1705) {
        f28(v0)
    } else {
        let a1758 = {
            let a1756 = {
                let a1754 = {
                    let a1706 = 1u64;
                    let a1707 = 1u64;
                    nat_add(a1706, a1707)?
                };
                let a1755 = {
                    let a1752 = {
                        let a1750 = {
                            let a1708 = 1u64;
                            let a1709 = 1u64;
                            nat_add(a1708, a1709)?
                        };
                        let a1751 = {
                            let a1748 = {
                                let a1746 = {
                                    let a1710 = 1u64;
                                    let a1711 = 1u64;
                                    nat_add(a1710, a1711)?
                                };
                                let a1747 = {
                                    let a1744 = {
                                        let a1742 = {
                                            let a1712 = 1u64;
                                            let a1713 = 1u64;
                                            nat_add(a1712, a1713)?
                                        };
                                        let a1743 = {
                                            let a1740 = {
                                                let a1738 = {
                                                    let a1714 = 1u64;
                                                    let a1715 = 1u64;
                                                    nat_add(a1714, a1715)?
                                                };
                                                let a1739 = {
                                                    let a1736 = {
                                                        let a1734 = {
                                                            let a1716 = 1u64;
                                                            let a1717 = 1u64;
                                                            nat_add(a1716, a1717)?
                                                        };
                                                        let a1735 = {
                                                            let a1732 = {
                                                                let a1730 = {
                                                                    let a1718 = 1u64;
                                                                    let a1719 = 1u64;
                                                                    nat_add(a1718, a1719)?
                                                                };
                                                                let a1731 = {
                                                                    let a1728 = {
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
                                                                    let a1729 = 1u64;
                                                                    nat_add(a1728, a1729)?
                                                                };
                                                                nat_mul(a1730, a1731)?
                                                            };
                                                            let a1733 = 1u64;
                                                            nat_add(a1732, a1733)?
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
                                };
                                nat_mul(a1746, a1747)?
                            };
                            let a1749 = 1u64;
                            nat_add(a1748, a1749)?
                        };
                        nat_mul(a1750, a1751)?
                    };
                    let a1753 = 1u64;
                    nat_add(a1752, a1753)?
                };
                nat_mul(a1754, a1755)?
            };
            let a1757 = 1u64;
            nat_add(a1756, a1757)?
        };
        let a1759 = v9;
        if nat_le(a1758, a1759) {
            f26(v0)
        } else {
            let a1811 = {
                let a1784 = f28(v0)?;
                let a1785 = {
                    let a1782 = v10;
                    let a1783 = f20({
                        let a1780 = {
                            let a1778 = {
                                let a1760 = 1u64;
                                let a1761 = 1u64;
                                nat_add(a1760, a1761)?
                            };
                            let a1779 = {
                                let a1776 = {
                                    let a1774 = {
                                        let a1762 = 1u64;
                                        let a1763 = 1u64;
                                        nat_add(a1762, a1763)?
                                    };
                                    let a1775 = {
                                        let a1772 = {
                                            let a1770 = {
                                                let a1764 = 1u64;
                                                let a1765 = 1u64;
                                                nat_add(a1764, a1765)?
                                            };
                                            let a1771 = {
                                                let a1768 = {
                                                    let a1766 = 1u64;
                                                    let a1767 = 1u64;
                                                    nat_add(a1766, a1767)?
                                                };
                                                let a1769 = 1u64;
                                                nat_mul(a1768, a1769)?
                                            };
                                            nat_mul(a1770, a1771)?
                                        };
                                        let a1773 = 1u64;
                                        nat_add(a1772, a1773)?
                                    };
                                    nat_mul(a1774, a1775)?
                                };
                                let a1777 = 1u64;
                                nat_add(a1776, a1777)?
                            };
                            nat_mul(a1778, a1779)?
                        };
                        let a1781 = 1u64;
                        nat_add(a1780, a1781)?
                    })?;
                    nat_mul(a1782, a1783)?
                };
                nat_add(a1784, a1785)?
            };
            let a1812 = {
                let a1808 = v8;
                let a1809 = f20({
                    let a1806 = {
                        let a1804 = {
                            let a1786 = 1u64;
                            let a1787 = 1u64;
                            nat_add(a1786, a1787)?
                        };
                        let a1805 = {
                            let a1802 = {
                                let a1800 = {
                                    let a1788 = 1u64;
                                    let a1789 = 1u64;
                                    nat_add(a1788, a1789)?
                                };
                                let a1801 = {
                                    let a1798 = {
                                        let a1796 = {
                                            let a1790 = 1u64;
                                            let a1791 = 1u64;
                                            nat_add(a1790, a1791)?
                                        };
                                        let a1797 = {
                                            let a1794 = {
                                                let a1792 = 1u64;
                                                let a1793 = 1u64;
                                                nat_add(a1792, a1793)?
                                            };
                                            let a1795 = 1u64;
                                            nat_mul(a1794, a1795)?
                                        };
                                        nat_mul(a1796, a1797)?
                                    };
                                    let a1799 = 1u64;
                                    nat_add(a1798, a1799)?
                                };
                                nat_mul(a1800, a1801)?
                            };
                            let a1803 = 1u64;
                            nat_add(a1802, a1803)?
                        };
                        nat_mul(a1804, a1805)?
                    };
                    let a1807 = 1u64;
                    nat_add(a1806, a1807)?
                })?;
                let a1810 = 0u64;
                nat_rem(a1808, a1809, a1810)
            };
            nat_add(a1811, a1812)
        }
    }
}

pub fn f46(v0: Adt2) -> R<Option<Adt2>> {
    let a1814 = 0u64;
    let a1815 = match v0.clone() {
        Adt2::C0(_, h1813, _, _) => {
            h1813
        }
    };
    if if if nat_lt(a1814, a1815) {
        let a1818 = match v0.clone() {
            Adt2::C0(_, h1816, _, _) => {
                h1816
            }
        };
        let a1819 = match v0.clone() {
            Adt2::C0(_, _, _, h1817) => {
                h1817
            }
        };
        nat_lt(a1818, a1819)
    } else {
        false
    } {
        let a1822 = match v0.clone() {
            Adt2::C0(_, _, h1820, _) => {
                h1820
            }
        };
        let a1823 = match v0.clone() {
            Adt2::C0(h1821, _, _, _) => {
                h1821
            }
        };
        nat_lt(a1822, a1823)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1825 = match v0.clone() {
                Adt2::C0(h1824, _, _, _) => {
                    h1824
                }
            };
            let a1826 = 1u64;
            nat_sub(a1825, a1826)
        }, {
            let a1829 = match v0.clone() {
                Adt2::C0(_, h1827, _, _) => {
                    h1827
                }
            };
            let a1830 = match v0.clone() {
                Adt2::C0(_, h1828, _, _) => {
                    h1828
                }
            };
            nat_add(a1829, a1830)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1831, _) => {
                h1831
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1832) => {
                h1832
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}
