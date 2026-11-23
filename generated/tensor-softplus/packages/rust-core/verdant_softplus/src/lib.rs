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
    C0(u64),
    C1(u64),
    C2(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(u64),
    C2(u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F18,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F18 => f18(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt2) -> R<Option<Adt2>> {
        match self {
            Fn1::F29 => f29(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            let a2 = 1101004800u64;
            let a3 = v1;
            if if nat_lt(a2, a3) {
                let a4 = v1;
                let a5 = 2139095040u64;
                nat_le(a4, a5)
            } else {
                false
            } {
                Ok(Adt1::C0(v1))
            } else {
                Ok(Adt1::C1(v1))
            }
        }
        Adt0::C1(v2) => {
            let v3: u64 = f1(1065353216u64, v2)?;
            let a6 = v3;
            let a7 = 1065353216u64;
            if nat_eq(a6, a7) {
                Ok(Adt1::C0(v2))
            } else {
                Ok(Adt1::C2(v3, v2))
            }
        }
        Adt0::C2(v4, v5, v6) => {
            let v7: u64 = f1(v4, 3212836864u64)?;
            let v8: u64 = f1(v5, f2(v7)?)?;
            Ok(Adt1::C0(f1(v6, f3(v8, v4)?)?))
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        let a94 = {
            let a72 = {
                let a48 = {
                    let a46 = {
                        let a8 = 1u64;
                        let a9 = 1u64;
                        nat_add(a8, a9)?
                    };
                    let a47 = {
                        let a44 = {
                            let a42 = {
                                let a10 = 1u64;
                                let a11 = 1u64;
                                nat_add(a10, a11)?
                            };
                            let a43 = {
                                let a40 = {
                                    let a38 = {
                                        let a12 = 1u64;
                                        let a13 = 1u64;
                                        nat_add(a12, a13)?
                                    };
                                    let a39 = {
                                        let a36 = {
                                            let a34 = {
                                                let a14 = 1u64;
                                                let a15 = 1u64;
                                                nat_add(a14, a15)?
                                            };
                                            let a35 = {
                                                let a32 = {
                                                    let a30 = {
                                                        let a16 = 1u64;
                                                        let a17 = 1u64;
                                                        nat_add(a16, a17)?
                                                    };
                                                    let a31 = {
                                                        let a28 = {
                                                            let a26 = {
                                                                let a18 = 1u64;
                                                                let a19 = 1u64;
                                                                nat_add(a18, a19)?
                                                            };
                                                            let a27 = {
                                                                let a24 = {
                                                                    let a22 = {
                                                                        let a20 = 1u64;
                                                                        let a21 = 1u64;
                                                                        nat_add(a20, a21)?
                                                                    };
                                                                    let a23 = 1u64;
                                                                    nat_mul(a22, a23)?
                                                                };
                                                                let a25 = 1u64;
                                                                nat_add(a24, a25)?
                                                            };
                                                            nat_mul(a26, a27)?
                                                        };
                                                        let a29 = 1u64;
                                                        nat_add(a28, a29)?
                                                    };
                                                    nat_mul(a30, a31)?
                                                };
                                                let a33 = 1u64;
                                                nat_add(a32, a33)?
                                            };
                                            nat_mul(a34, a35)?
                                        };
                                        let a37 = 1u64;
                                        nat_add(a36, a37)?
                                    };
                                    nat_mul(a38, a39)?
                                };
                                let a41 = 1u64;
                                nat_add(a40, a41)?
                            };
                            nat_mul(a42, a43)?
                        };
                        let a45 = 1u64;
                        nat_add(a44, a45)?
                    };
                    nat_mul(a46, a47)?
                };
                let a49 = 1u64;
                nat_add(a48, a49)?
            };
            let a73 = f5({
                let a70 = {
                    let a68 = {
                        let a50 = 1u64;
                        let a51 = 1u64;
                        nat_add(a50, a51)?
                    };
                    let a69 = {
                        let a66 = {
                            let a64 = {
                                let a52 = 1u64;
                                let a53 = 1u64;
                                nat_add(a52, a53)?
                            };
                            let a65 = {
                                let a62 = {
                                    let a60 = {
                                        let a54 = 1u64;
                                        let a55 = 1u64;
                                        nat_add(a54, a55)?
                                    };
                                    let a61 = {
                                        let a58 = {
                                            let a56 = 1u64;
                                            let a57 = 1u64;
                                            nat_add(a56, a57)?
                                        };
                                        let a59 = 1u64;
                                        nat_mul(a58, a59)?
                                    };
                                    nat_mul(a60, a61)?
                                };
                                let a63 = 1u64;
                                nat_add(a62, a63)?
                            };
                            nat_mul(a64, a65)?
                        };
                        let a67 = 1u64;
                        nat_add(a66, a67)?
                    };
                    nat_mul(a68, a69)?
                };
                let a71 = 1u64;
                nat_add(a70, a71)?
            })?;
            nat_mul(a72, a73)?
        };
        let a95 = f5({
            let a92 = {
                let a74 = 1u64;
                let a75 = 1u64;
                nat_add(a74, a75)?
            };
            let a93 = {
                let a90 = {
                    let a88 = {
                        let a76 = 1u64;
                        let a77 = 1u64;
                        nat_add(a76, a77)?
                    };
                    let a89 = {
                        let a86 = {
                            let a84 = {
                                let a78 = 1u64;
                                let a79 = 1u64;
                                nat_add(a78, a79)?
                            };
                            let a85 = {
                                let a82 = {
                                    let a80 = 1u64;
                                    let a81 = 1u64;
                                    nat_add(a80, a81)?
                                };
                                let a83 = 1u64;
                                nat_mul(a82, a83)?
                            };
                            nat_mul(a84, a85)?
                        };
                        let a87 = 1u64;
                        nat_add(a86, a87)?
                    };
                    nat_mul(a88, a89)?
                };
                let a91 = 1u64;
                nat_add(a90, a91)?
            };
            nat_mul(a92, a93)?
        })?;
        nat_add(a94, a95)
    } else {
        if if f6(v0)? {
            f6(v1)?
        } else {
            false
        } {
            let a96 = f7(v0)?;
            let a97 = f7(v1)?;
            if nat_eq(a96, a97) {
                Ok(v0)
            } else {
                let a184 = {
                    let a162 = {
                        let a138 = {
                            let a136 = {
                                let a98 = 1u64;
                                let a99 = 1u64;
                                nat_add(a98, a99)?
                            };
                            let a137 = {
                                let a134 = {
                                    let a132 = {
                                        let a100 = 1u64;
                                        let a101 = 1u64;
                                        nat_add(a100, a101)?
                                    };
                                    let a133 = {
                                        let a130 = {
                                            let a128 = {
                                                let a102 = 1u64;
                                                let a103 = 1u64;
                                                nat_add(a102, a103)?
                                            };
                                            let a129 = {
                                                let a126 = {
                                                    let a124 = {
                                                        let a104 = 1u64;
                                                        let a105 = 1u64;
                                                        nat_add(a104, a105)?
                                                    };
                                                    let a125 = {
                                                        let a122 = {
                                                            let a120 = {
                                                                let a106 = 1u64;
                                                                let a107 = 1u64;
                                                                nat_add(a106, a107)?
                                                            };
                                                            let a121 = {
                                                                let a118 = {
                                                                    let a116 = {
                                                                        let a108 = 1u64;
                                                                        let a109 = 1u64;
                                                                        nat_add(a108, a109)?
                                                                    };
                                                                    let a117 = {
                                                                        let a114 = {
                                                                            let a112 = {
                                                                                let a110 = 1u64;
                                                                                let a111 = 1u64;
                                                                                nat_add(a110, a111)?
                                                                            };
                                                                            let a113 = 1u64;
                                                                            nat_mul(a112, a113)?
                                                                        };
                                                                        let a115 = 1u64;
                                                                        nat_add(a114, a115)?
                                                                    };
                                                                    nat_mul(a116, a117)?
                                                                };
                                                                let a119 = 1u64;
                                                                nat_add(a118, a119)?
                                                            };
                                                            nat_mul(a120, a121)?
                                                        };
                                                        let a123 = 1u64;
                                                        nat_add(a122, a123)?
                                                    };
                                                    nat_mul(a124, a125)?
                                                };
                                                let a127 = 1u64;
                                                nat_add(a126, a127)?
                                            };
                                            nat_mul(a128, a129)?
                                        };
                                        let a131 = 1u64;
                                        nat_add(a130, a131)?
                                    };
                                    nat_mul(a132, a133)?
                                };
                                let a135 = 1u64;
                                nat_add(a134, a135)?
                            };
                            nat_mul(a136, a137)?
                        };
                        let a139 = 1u64;
                        nat_add(a138, a139)?
                    };
                    let a163 = f5({
                        let a160 = {
                            let a158 = {
                                let a140 = 1u64;
                                let a141 = 1u64;
                                nat_add(a140, a141)?
                            };
                            let a159 = {
                                let a156 = {
                                    let a154 = {
                                        let a142 = 1u64;
                                        let a143 = 1u64;
                                        nat_add(a142, a143)?
                                    };
                                    let a155 = {
                                        let a152 = {
                                            let a150 = {
                                                let a144 = 1u64;
                                                let a145 = 1u64;
                                                nat_add(a144, a145)?
                                            };
                                            let a151 = {
                                                let a148 = {
                                                    let a146 = 1u64;
                                                    let a147 = 1u64;
                                                    nat_add(a146, a147)?
                                                };
                                                let a149 = 1u64;
                                                nat_mul(a148, a149)?
                                            };
                                            nat_mul(a150, a151)?
                                        };
                                        let a153 = 1u64;
                                        nat_add(a152, a153)?
                                    };
                                    nat_mul(a154, a155)?
                                };
                                let a157 = 1u64;
                                nat_add(a156, a157)?
                            };
                            nat_mul(a158, a159)?
                        };
                        let a161 = 1u64;
                        nat_add(a160, a161)?
                    })?;
                    nat_mul(a162, a163)?
                };
                let a185 = f5({
                    let a182 = {
                        let a164 = 1u64;
                        let a165 = 1u64;
                        nat_add(a164, a165)?
                    };
                    let a183 = {
                        let a180 = {
                            let a178 = {
                                let a166 = 1u64;
                                let a167 = 1u64;
                                nat_add(a166, a167)?
                            };
                            let a179 = {
                                let a176 = {
                                    let a174 = {
                                        let a168 = 1u64;
                                        let a169 = 1u64;
                                        nat_add(a168, a169)?
                                    };
                                    let a175 = {
                                        let a172 = {
                                            let a170 = 1u64;
                                            let a171 = 1u64;
                                            nat_add(a170, a171)?
                                        };
                                        let a173 = 1u64;
                                        nat_mul(a172, a173)?
                                    };
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
                })?;
                nat_add(a184, a185)
            }
        } else {
            if f6(v0)? {
                Ok(v0)
            } else {
                if f6(v1)? {
                    Ok(v1)
                } else {
                    let a186 = f8(v1)?;
                    let a187 = f8(v0)?;
                    if nat_le(a186, a187) {
                        f9(v0, v1)
                    } else {
                        f9(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let a234 = {
        let a212 = v0;
        let a213 = f5({
            let a210 = {
                let a208 = {
                    let a188 = 1u64;
                    let a189 = 1u64;
                    nat_add(a188, a189)?
                };
                let a209 = {
                    let a206 = {
                        let a204 = {
                            let a190 = 1u64;
                            let a191 = 1u64;
                            nat_add(a190, a191)?
                        };
                        let a205 = {
                            let a202 = {
                                let a200 = {
                                    let a192 = 1u64;
                                    let a193 = 1u64;
                                    nat_add(a192, a193)?
                                };
                                let a201 = {
                                    let a198 = {
                                        let a196 = {
                                            let a194 = 1u64;
                                            let a195 = 1u64;
                                            nat_add(a194, a195)?
                                        };
                                        let a197 = 1u64;
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
        })?;
        nat_add(a212, a213)?
    };
    let a235 = f5({
        let a232 = {
            let a214 = 1u64;
            let a215 = 1u64;
            nat_add(a214, a215)?
        };
        let a233 = {
            let a230 = {
                let a216 = 1u64;
                let a217 = 1u64;
                nat_add(a216, a217)?
            };
            let a231 = {
                let a228 = {
                    let a218 = 1u64;
                    let a219 = 1u64;
                    nat_add(a218, a219)?
                };
                let a229 = {
                    let a226 = {
                        let a220 = 1u64;
                        let a221 = 1u64;
                        nat_add(a220, a221)?
                    };
                    let a227 = {
                        let a224 = {
                            let a222 = 1u64;
                            let a223 = 1u64;
                            nat_add(a222, a223)?
                        };
                        let a225 = 1u64;
                        nat_mul(a224, a225)?
                    };
                    nat_mul(a226, a227)?
                };
                nat_mul(a228, a229)?
            };
            nat_mul(a230, a231)?
        };
        nat_mul(a232, a233)?
    })?;
    let a236 = 0u64;
    Ok(nat_rem(a234, a235, a236))
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    if if if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        true
    } else {
        if f6(v0)? {
            f6(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f10(v0)? {
            f10(v1)?
        } else {
            false
        }
    } {
        let a323 = {
            let a301 = {
                let a277 = {
                    let a275 = {
                        let a237 = 1u64;
                        let a238 = 1u64;
                        nat_add(a237, a238)?
                    };
                    let a276 = {
                        let a273 = {
                            let a271 = {
                                let a239 = 1u64;
                                let a240 = 1u64;
                                nat_add(a239, a240)?
                            };
                            let a272 = {
                                let a269 = {
                                    let a267 = {
                                        let a241 = 1u64;
                                        let a242 = 1u64;
                                        nat_add(a241, a242)?
                                    };
                                    let a268 = {
                                        let a265 = {
                                            let a263 = {
                                                let a243 = 1u64;
                                                let a244 = 1u64;
                                                nat_add(a243, a244)?
                                            };
                                            let a264 = {
                                                let a261 = {
                                                    let a259 = {
                                                        let a245 = 1u64;
                                                        let a246 = 1u64;
                                                        nat_add(a245, a246)?
                                                    };
                                                    let a260 = {
                                                        let a257 = {
                                                            let a255 = {
                                                                let a247 = 1u64;
                                                                let a248 = 1u64;
                                                                nat_add(a247, a248)?
                                                            };
                                                            let a256 = {
                                                                let a253 = {
                                                                    let a251 = {
                                                                        let a249 = 1u64;
                                                                        let a250 = 1u64;
                                                                        nat_add(a249, a250)?
                                                                    };
                                                                    let a252 = 1u64;
                                                                    nat_mul(a251, a252)?
                                                                };
                                                                let a254 = 1u64;
                                                                nat_add(a253, a254)?
                                                            };
                                                            nat_mul(a255, a256)?
                                                        };
                                                        let a258 = 1u64;
                                                        nat_add(a257, a258)?
                                                    };
                                                    nat_mul(a259, a260)?
                                                };
                                                let a262 = 1u64;
                                                nat_add(a261, a262)?
                                            };
                                            nat_mul(a263, a264)?
                                        };
                                        let a266 = 1u64;
                                        nat_add(a265, a266)?
                                    };
                                    nat_mul(a267, a268)?
                                };
                                let a270 = 1u64;
                                nat_add(a269, a270)?
                            };
                            nat_mul(a271, a272)?
                        };
                        let a274 = 1u64;
                        nat_add(a273, a274)?
                    };
                    nat_mul(a275, a276)?
                };
                let a278 = 1u64;
                nat_add(a277, a278)?
            };
            let a302 = f5({
                let a299 = {
                    let a297 = {
                        let a279 = 1u64;
                        let a280 = 1u64;
                        nat_add(a279, a280)?
                    };
                    let a298 = {
                        let a295 = {
                            let a293 = {
                                let a281 = 1u64;
                                let a282 = 1u64;
                                nat_add(a281, a282)?
                            };
                            let a294 = {
                                let a291 = {
                                    let a289 = {
                                        let a283 = 1u64;
                                        let a284 = 1u64;
                                        nat_add(a283, a284)?
                                    };
                                    let a290 = {
                                        let a287 = {
                                            let a285 = 1u64;
                                            let a286 = 1u64;
                                            nat_add(a285, a286)?
                                        };
                                        let a288 = 1u64;
                                        nat_mul(a287, a288)?
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
            })?;
            nat_mul(a301, a302)?
        };
        let a324 = f5({
            let a321 = {
                let a303 = 1u64;
                let a304 = 1u64;
                nat_add(a303, a304)?
            };
            let a322 = {
                let a319 = {
                    let a317 = {
                        let a305 = 1u64;
                        let a306 = 1u64;
                        nat_add(a305, a306)?
                    };
                    let a318 = {
                        let a315 = {
                            let a313 = {
                                let a307 = 1u64;
                                let a308 = 1u64;
                                nat_add(a307, a308)?
                            };
                            let a314 = {
                                let a311 = {
                                    let a309 = 1u64;
                                    let a310 = 1u64;
                                    nat_add(a309, a310)?
                                };
                                let a312 = 1u64;
                                nat_mul(a311, a312)?
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
        })?;
        nat_add(a323, a324)
    } else {
        if if f6(v0)? {
            true
        } else {
            f10(v1)?
        } {
            f11(f12(v0, v1)?)
        } else {
            if if f6(v1)? {
                true
            } else {
                f10(v0)?
            } {
                f13(f12(v0, v1)?)
            } else {
                f14(v0, v1)
            }
        }
    }
}

pub fn f4(v0: u64) -> R<bool> {
    let a367 = f15(v0)?;
    let a368 = {
        let a365 = {
            let a363 = {
                let a325 = 1u64;
                let a326 = 1u64;
                nat_add(a325, a326)?
            };
            let a364 = {
                let a361 = {
                    let a359 = {
                        let a327 = 1u64;
                        let a328 = 1u64;
                        nat_add(a327, a328)?
                    };
                    let a360 = {
                        let a357 = {
                            let a355 = {
                                let a329 = 1u64;
                                let a330 = 1u64;
                                nat_add(a329, a330)?
                            };
                            let a356 = {
                                let a353 = {
                                    let a351 = {
                                        let a331 = 1u64;
                                        let a332 = 1u64;
                                        nat_add(a331, a332)?
                                    };
                                    let a352 = {
                                        let a349 = {
                                            let a347 = {
                                                let a333 = 1u64;
                                                let a334 = 1u64;
                                                nat_add(a333, a334)?
                                            };
                                            let a348 = {
                                                let a345 = {
                                                    let a343 = {
                                                        let a335 = 1u64;
                                                        let a336 = 1u64;
                                                        nat_add(a335, a336)?
                                                    };
                                                    let a344 = {
                                                        let a341 = {
                                                            let a339 = {
                                                                let a337 = 1u64;
                                                                let a338 = 1u64;
                                                                nat_add(a337, a338)?
                                                            };
                                                            let a340 = 1u64;
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
                                    };
                                    nat_mul(a351, a352)?
                                };
                                let a354 = 1u64;
                                nat_add(a353, a354)?
                            };
                            nat_mul(a355, a356)?
                        };
                        let a358 = 1u64;
                        nat_add(a357, a358)?
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
    if nat_eq(a367, a368) {
        let a369 = 0u64;
        let a370 = f16(v0)?;
        Ok(nat_lt(a369, a370))
    } else {
        Ok(false)
    }
}

pub fn f5(v0: u64) -> R<u64> {
    f17(Fn0::F18, v0, 1u64)
}

pub fn f6(v0: u64) -> R<bool> {
    let a413 = f15(v0)?;
    let a414 = {
        let a411 = {
            let a409 = {
                let a371 = 1u64;
                let a372 = 1u64;
                nat_add(a371, a372)?
            };
            let a410 = {
                let a407 = {
                    let a405 = {
                        let a373 = 1u64;
                        let a374 = 1u64;
                        nat_add(a373, a374)?
                    };
                    let a406 = {
                        let a403 = {
                            let a401 = {
                                let a375 = 1u64;
                                let a376 = 1u64;
                                nat_add(a375, a376)?
                            };
                            let a402 = {
                                let a399 = {
                                    let a397 = {
                                        let a377 = 1u64;
                                        let a378 = 1u64;
                                        nat_add(a377, a378)?
                                    };
                                    let a398 = {
                                        let a395 = {
                                            let a393 = {
                                                let a379 = 1u64;
                                                let a380 = 1u64;
                                                nat_add(a379, a380)?
                                            };
                                            let a394 = {
                                                let a391 = {
                                                    let a389 = {
                                                        let a381 = 1u64;
                                                        let a382 = 1u64;
                                                        nat_add(a381, a382)?
                                                    };
                                                    let a390 = {
                                                        let a387 = {
                                                            let a385 = {
                                                                let a383 = 1u64;
                                                                let a384 = 1u64;
                                                                nat_add(a383, a384)?
                                                            };
                                                            let a386 = 1u64;
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
            nat_mul(a409, a410)?
        };
        let a412 = 1u64;
        nat_add(a411, a412)?
    };
    if nat_eq(a413, a414) {
        let a415 = f16(v0)?;
        let a416 = 0u64;
        Ok(nat_eq(a415, a416))
    } else {
        Ok(false)
    }
}

pub fn f7(v0: u64) -> R<u64> {
    let a441 = v0;
    let a442 = f5({
        let a439 = {
            let a437 = {
                let a417 = 1u64;
                let a418 = 1u64;
                nat_add(a417, a418)?
            };
            let a438 = {
                let a435 = {
                    let a433 = {
                        let a419 = 1u64;
                        let a420 = 1u64;
                        nat_add(a419, a420)?
                    };
                    let a434 = {
                        let a431 = {
                            let a429 = {
                                let a421 = 1u64;
                                let a422 = 1u64;
                                nat_add(a421, a422)?
                            };
                            let a430 = {
                                let a427 = {
                                    let a425 = {
                                        let a423 = 1u64;
                                        let a424 = 1u64;
                                        nat_add(a423, a424)?
                                    };
                                    let a426 = 1u64;
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
            };
            nat_mul(a437, a438)?
        };
        let a440 = 1u64;
        nat_add(a439, a440)?
    })?;
    let a443 = 0u64;
    Ok(nat_quot(a441, a442, a443))
}

pub fn f8(v0: u64) -> R<u64> {
    let a468 = v0;
    let a469 = f5({
        let a466 = {
            let a464 = {
                let a444 = 1u64;
                let a445 = 1u64;
                nat_add(a444, a445)?
            };
            let a465 = {
                let a462 = {
                    let a460 = {
                        let a446 = 1u64;
                        let a447 = 1u64;
                        nat_add(a446, a447)?
                    };
                    let a461 = {
                        let a458 = {
                            let a456 = {
                                let a448 = 1u64;
                                let a449 = 1u64;
                                nat_add(a448, a449)?
                            };
                            let a457 = {
                                let a454 = {
                                    let a452 = {
                                        let a450 = 1u64;
                                        let a451 = 1u64;
                                        nat_add(a450, a451)?
                                    };
                                    let a453 = 1u64;
                                    nat_mul(a452, a453)?
                                };
                                let a455 = 1u64;
                                nat_add(a454, a455)?
                            };
                            nat_mul(a456, a457)?
                        };
                        let a459 = 1u64;
                        nat_add(a458, a459)?
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
    })?;
    let a470 = 0u64;
    Ok(nat_rem(a468, a469, a470))
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f19(v0)?;
    let v3: u64 = {
        let a483 = f20(v0)?;
        let a484 = {
            let a481 = {
                let a471 = 1u64;
                let a472 = 1u64;
                nat_add(a471, a472)?
            };
            let a482 = {
                let a479 = {
                    let a473 = 1u64;
                    let a474 = 1u64;
                    nat_add(a473, a474)?
                };
                let a480 = {
                    let a477 = {
                        let a475 = 1u64;
                        let a476 = 1u64;
                        nat_add(a475, a476)?
                    };
                    let a478 = 1u64;
                    nat_mul(a477, a478)?
                };
                nat_mul(a479, a480)?
            };
            nat_mul(a481, a482)?
        };
        nat_mul(a483, a484)?
    };
    let v4: u64 = f21({
        let a497 = f20(v1)?;
        let a498 = {
            let a495 = {
                let a485 = 1u64;
                let a486 = 1u64;
                nat_add(a485, a486)?
            };
            let a496 = {
                let a493 = {
                    let a487 = 1u64;
                    let a488 = 1u64;
                    nat_add(a487, a488)?
                };
                let a494 = {
                    let a491 = {
                        let a489 = 1u64;
                        let a490 = 1u64;
                        nat_add(a489, a490)?
                    };
                    let a492 = 1u64;
                    nat_mul(a491, a492)?
                };
                nat_mul(a493, a494)?
            };
            nat_mul(a495, a496)?
        };
        nat_mul(a497, a498)?
    }, {
        let a499 = v2;
        let a500 = f19(v1)?;
        nat_sub(a499, a500)
    })?;
    let a501 = f7(v0)?;
    let a502 = f7(v1)?;
    if nat_eq(a501, a502) {
        f22(f7(v0)?, v2, {
            let a503 = v3;
            let a504 = v4;
            nat_add(a503, a504)?
        })
    } else {
        let a505 = v3;
        let a506 = v4;
        if nat_eq(a505, a506) {
            Ok(0u64)
        } else {
            f22(f7(v0)?, v2, {
                let a507 = v3;
                let a508 = v4;
                nat_sub(a507, a508)
            })
        }
    }
}

pub fn f10(v0: u64) -> R<bool> {
    let a509 = f8(v0)?;
    let a510 = 0u64;
    Ok(nat_eq(a509, a510))
}

pub fn f11(v0: u64) -> R<u64> {
    let a577 = f13(v0)?;
    let a578 = {
        let a575 = {
            let a551 = {
                let a549 = {
                    let a511 = 1u64;
                    let a512 = 1u64;
                    nat_add(a511, a512)?
                };
                let a550 = {
                    let a547 = {
                        let a545 = {
                            let a513 = 1u64;
                            let a514 = 1u64;
                            nat_add(a513, a514)?
                        };
                        let a546 = {
                            let a543 = {
                                let a541 = {
                                    let a515 = 1u64;
                                    let a516 = 1u64;
                                    nat_add(a515, a516)?
                                };
                                let a542 = {
                                    let a539 = {
                                        let a537 = {
                                            let a517 = 1u64;
                                            let a518 = 1u64;
                                            nat_add(a517, a518)?
                                        };
                                        let a538 = {
                                            let a535 = {
                                                let a533 = {
                                                    let a519 = 1u64;
                                                    let a520 = 1u64;
                                                    nat_add(a519, a520)?
                                                };
                                                let a534 = {
                                                    let a531 = {
                                                        let a529 = {
                                                            let a521 = 1u64;
                                                            let a522 = 1u64;
                                                            nat_add(a521, a522)?
                                                        };
                                                        let a530 = {
                                                            let a527 = {
                                                                let a525 = {
                                                                    let a523 = 1u64;
                                                                    let a524 = 1u64;
                                                                    nat_add(a523, a524)?
                                                                };
                                                                let a526 = 1u64;
                                                                nat_mul(a525, a526)?
                                                            };
                                                            let a528 = 1u64;
                                                            nat_add(a527, a528)?
                                                        };
                                                        nat_mul(a529, a530)?
                                                    };
                                                    let a532 = 1u64;
                                                    nat_add(a531, a532)?
                                                };
                                                nat_mul(a533, a534)?
                                            };
                                            let a536 = 1u64;
                                            nat_add(a535, a536)?
                                        };
                                        nat_mul(a537, a538)?
                                    };
                                    let a540 = 1u64;
                                    nat_add(a539, a540)?
                                };
                                nat_mul(a541, a542)?
                            };
                            let a544 = 1u64;
                            nat_add(a543, a544)?
                        };
                        nat_mul(a545, a546)?
                    };
                    let a548 = 1u64;
                    nat_add(a547, a548)?
                };
                nat_mul(a549, a550)?
            };
            let a552 = 1u64;
            nat_add(a551, a552)?
        };
        let a576 = f5({
            let a573 = {
                let a571 = {
                    let a553 = 1u64;
                    let a554 = 1u64;
                    nat_add(a553, a554)?
                };
                let a572 = {
                    let a569 = {
                        let a567 = {
                            let a555 = 1u64;
                            let a556 = 1u64;
                            nat_add(a555, a556)?
                        };
                        let a568 = {
                            let a565 = {
                                let a563 = {
                                    let a557 = 1u64;
                                    let a558 = 1u64;
                                    nat_add(a557, a558)?
                                };
                                let a564 = {
                                    let a561 = {
                                        let a559 = 1u64;
                                        let a560 = 1u64;
                                        nat_add(a559, a560)?
                                    };
                                    let a562 = 1u64;
                                    nat_mul(a561, a562)?
                                };
                                nat_mul(a563, a564)?
                            };
                            let a566 = 1u64;
                            nat_add(a565, a566)?
                        };
                        nat_mul(a567, a568)?
                    };
                    let a570 = 1u64;
                    nat_add(a569, a570)?
                };
                nat_mul(a571, a572)?
            };
            let a574 = 1u64;
            nat_add(a573, a574)?
        })?;
        nat_mul(a575, a576)?
    };
    nat_add(a577, a578)
}

pub fn f12(v0: u64, v1: u64) -> R<u64> {
    let a585 = {
        let a579 = f7(v0)?;
        let a580 = f7(v1)?;
        nat_add(a579, a580)?
    };
    let a586 = {
        let a583 = {
            let a581 = 1u64;
            let a582 = 1u64;
            nat_add(a581, a582)?
        };
        let a584 = 1u64;
        nat_mul(a583, a584)?
    };
    let a587 = 0u64;
    Ok(nat_rem(a585, a586, a587))
}

pub fn f13(v0: u64) -> R<u64> {
    let a612 = v0;
    let a613 = f5({
        let a610 = {
            let a608 = {
                let a588 = 1u64;
                let a589 = 1u64;
                nat_add(a588, a589)?
            };
            let a609 = {
                let a606 = {
                    let a604 = {
                        let a590 = 1u64;
                        let a591 = 1u64;
                        nat_add(a590, a591)?
                    };
                    let a605 = {
                        let a602 = {
                            let a600 = {
                                let a592 = 1u64;
                                let a593 = 1u64;
                                nat_add(a592, a593)?
                            };
                            let a601 = {
                                let a598 = {
                                    let a596 = {
                                        let a594 = 1u64;
                                        let a595 = 1u64;
                                        nat_add(a594, a595)?
                                    };
                                    let a597 = 1u64;
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
            nat_mul(a608, a609)?
        };
        let a611 = 1u64;
        nat_add(a610, a611)?
    })?;
    nat_mul(a612, a613)
}

pub fn f14(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt2 = f23(v0)?;
    let v3: Adt2 = f23(v1)?;
    let v4: bool = {
        let a616 = match v2.clone() {
            Adt2::C0(_, h614, _, _) => {
                h614
            }
        };
        let a617 = match v3.clone() {
            Adt2::C0(_, h615, _, _) => {
                h615
            }
        };
        nat_lt(a616, a617)
    };
    let v5: u64 = {
        let a661 = match v2.clone() {
            Adt2::C0(_, h618, _, _) => {
                h618
            }
        };
        let a662 = f5(if v4 {
            let a639 = {
                let a637 = {
                    let a619 = 1u64;
                    let a620 = 1u64;
                    nat_add(a619, a620)?
                };
                let a638 = {
                    let a635 = {
                        let a633 = {
                            let a621 = 1u64;
                            let a622 = 1u64;
                            nat_add(a621, a622)?
                        };
                        let a634 = {
                            let a631 = {
                                let a623 = 1u64;
                                let a624 = 1u64;
                                nat_add(a623, a624)?
                            };
                            let a632 = {
                                let a629 = {
                                    let a627 = {
                                        let a625 = 1u64;
                                        let a626 = 1u64;
                                        nat_add(a625, a626)?
                                    };
                                    let a628 = 1u64;
                                    nat_mul(a627, a628)?
                                };
                                let a630 = 1u64;
                                nat_add(a629, a630)?
                            };
                            nat_mul(a631, a632)?
                        };
                        nat_mul(a633, a634)?
                    };
                    let a636 = 1u64;
                    nat_add(a635, a636)?
                };
                nat_mul(a637, a638)?
            };
            let a640 = 1u64;
            nat_add(a639, a640)?
        } else {
            let a659 = {
                let a641 = 1u64;
                let a642 = 1u64;
                nat_add(a641, a642)?
            };
            let a660 = {
                let a657 = {
                    let a655 = {
                        let a643 = 1u64;
                        let a644 = 1u64;
                        nat_add(a643, a644)?
                    };
                    let a656 = {
                        let a653 = {
                            let a645 = 1u64;
                            let a646 = 1u64;
                            nat_add(a645, a646)?
                        };
                        let a654 = {
                            let a651 = {
                                let a649 = {
                                    let a647 = 1u64;
                                    let a648 = 1u64;
                                    nat_add(a647, a648)?
                                };
                                let a650 = 1u64;
                                nat_mul(a649, a650)?
                            };
                            let a652 = 1u64;
                            nat_add(a651, a652)?
                        };
                        nat_mul(a653, a654)?
                    };
                    nat_mul(a655, a656)?
                };
                let a658 = 1u64;
                nat_add(a657, a658)?
            };
            nat_mul(a659, a660)?
        })?;
        nat_mul(a661, a662)?
    };
    let v6: u64 = {
        let a719 = {
            let a717 = {
                let a714 = match v2 {
                    Adt2::C0(h663, _, _, _) => {
                        h663
                    }
                };
                let a715 = {
                    let a712 = {
                        let a710 = {
                            let a664 = 1u64;
                            let a665 = 1u64;
                            nat_add(a664, a665)?
                        };
                        let a711 = {
                            let a708 = {
                                let a706 = {
                                    let a666 = 1u64;
                                    let a667 = 1u64;
                                    nat_add(a666, a667)?
                                };
                                let a707 = {
                                    let a704 = {
                                        let a702 = {
                                            let a668 = 1u64;
                                            let a669 = 1u64;
                                            nat_add(a668, a669)?
                                        };
                                        let a703 = {
                                            let a700 = {
                                                let a698 = {
                                                    let a670 = 1u64;
                                                    let a671 = 1u64;
                                                    nat_add(a670, a671)?
                                                };
                                                let a699 = {
                                                    let a696 = {
                                                        let a694 = {
                                                            let a672 = 1u64;
                                                            let a673 = 1u64;
                                                            nat_add(a672, a673)?
                                                        };
                                                        let a695 = {
                                                            let a692 = {
                                                                let a690 = {
                                                                    let a674 = 1u64;
                                                                    let a675 = 1u64;
                                                                    nat_add(a674, a675)?
                                                                };
                                                                let a691 = {
                                                                    let a688 = {
                                                                        let a686 = {
                                                                            let a676 = 1u64;
                                                                            let a677 = 1u64;
                                                                            nat_add(a676, a677)?
                                                                        };
                                                                        let a687 = {
                                                                            let a684 = {
                                                                                let a678 = 1u64;
                                                                                let a679 = 1u64;
                                                                                nat_add(a678, a679)?
                                                                            };
                                                                            let a685 = {
                                                                                let a682 = {
                                                                                    let a680 = 1u64;
                                                                                    let a681 = 1u64;
                                                                                    nat_add(a680, a681)?
                                                                                };
                                                                                let a683 = 1u64;
                                                                                nat_mul(a682, a683)?
                                                                            };
                                                                            nat_mul(a684, a685)?
                                                                        };
                                                                        nat_mul(a686, a687)?
                                                                    };
                                                                    let a689 = 1u64;
                                                                    nat_add(a688, a689)?
                                                                };
                                                                nat_mul(a690, a691)?
                                                            };
                                                            let a693 = 1u64;
                                                            nat_add(a692, a693)?
                                                        };
                                                        nat_mul(a694, a695)?
                                                    };
                                                    let a697 = 1u64;
                                                    nat_add(a696, a697)?
                                                };
                                                nat_mul(a698, a699)?
                                            };
                                            let a701 = 1u64;
                                            nat_add(a700, a701)?
                                        };
                                        nat_mul(a702, a703)?
                                    };
                                    let a705 = 1u64;
                                    nat_add(a704, a705)?
                                };
                                nat_mul(a706, a707)?
                            };
                            let a709 = 1u64;
                            nat_add(a708, a709)?
                        };
                        nat_mul(a710, a711)?
                    };
                    let a713 = 1u64;
                    nat_add(a712, a713)?
                };
                nat_add(a714, a715)?
            };
            let a718 = match v3.clone() {
                Adt2::C0(h716, _, _, _) => {
                    h716
                }
            };
            nat_sub(a717, a718)
        };
        let a720 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a719, a720)
    };
    let v7: u64 = {
        let a722 = v5;
        let a723 = match v3.clone() {
            Adt2::C0(_, h721, _, _) => {
                h721
            }
        };
        let a724 = 0u64;
        nat_quot(a722, a723, a724)
    };
    let v8: bool = {
        let a729 = 0u64;
        let a730 = {
            let a726 = v5;
            let a727 = match v3 {
                Adt2::C0(_, h725, _, _) => {
                    h725
                }
            };
            let a728 = 0u64;
            nat_rem(a726, a727, a728)
        };
        nat_lt(a729, a730)
    };
    f22(f12(v0, v1)?, v6, f24(v7, v8)?)
}

pub fn f15(v0: u64) -> R<u64> {
    let a753 = f8(v0)?;
    let a754 = f5({
        let a751 = {
            let a749 = {
                let a731 = 1u64;
                let a732 = 1u64;
                nat_add(a731, a732)?
            };
            let a750 = {
                let a747 = {
                    let a745 = {
                        let a733 = 1u64;
                        let a734 = 1u64;
                        nat_add(a733, a734)?
                    };
                    let a746 = {
                        let a743 = {
                            let a741 = {
                                let a735 = 1u64;
                                let a736 = 1u64;
                                nat_add(a735, a736)?
                            };
                            let a742 = {
                                let a739 = {
                                    let a737 = 1u64;
                                    let a738 = 1u64;
                                    nat_add(a737, a738)?
                                };
                                let a740 = 1u64;
                                nat_mul(a739, a740)?
                            };
                            nat_mul(a741, a742)?
                        };
                        let a744 = 1u64;
                        nat_add(a743, a744)?
                    };
                    nat_mul(a745, a746)?
                };
                let a748 = 1u64;
                nat_add(a747, a748)?
            };
            nat_mul(a749, a750)?
        };
        let a752 = 1u64;
        nat_add(a751, a752)?
    })?;
    let a755 = 0u64;
    Ok(nat_quot(a753, a754, a755))
}

pub fn f16(v0: u64) -> R<u64> {
    let a778 = v0;
    let a779 = f5({
        let a776 = {
            let a774 = {
                let a756 = 1u64;
                let a757 = 1u64;
                nat_add(a756, a757)?
            };
            let a775 = {
                let a772 = {
                    let a770 = {
                        let a758 = 1u64;
                        let a759 = 1u64;
                        nat_add(a758, a759)?
                    };
                    let a771 = {
                        let a768 = {
                            let a766 = {
                                let a760 = 1u64;
                                let a761 = 1u64;
                                nat_add(a760, a761)?
                            };
                            let a767 = {
                                let a764 = {
                                    let a762 = 1u64;
                                    let a763 = 1u64;
                                    nat_add(a762, a763)?
                                };
                                let a765 = 1u64;
                                nat_mul(a764, a765)?
                            };
                            nat_mul(a766, a767)?
                        };
                        let a769 = 1u64;
                        nat_add(a768, a769)?
                    };
                    nat_mul(a770, a771)?
                };
                let a773 = 1u64;
                nat_add(a772, a773)?
            };
            nat_mul(a774, a775)?
        };
        let a777 = 1u64;
        nat_add(a776, a777)?
    })?;
    let a780 = 0u64;
    Ok(nat_rem(a778, a779, a780))
}

pub fn f17(v0: Fn0, p784: u64, p785: u64) -> R<u64> {
    let m783 = (p784, p785);
    {
        let mut m783 = m783;
        loop {
            let step = {
                let (v1, v2) = m783;
                let m781 = v1;
                if m781 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m781 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c782: Fn0 = v0.clone();
                        c782.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m783 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a786 = v0;
    let a787 = v0;
    nat_add(a786, a787)
}

pub fn f19(v0: u64) -> R<u64> {
    let a826 = {
        let a788 = f15(v0)?;
        let a789 = 0u64;
        if nat_eq(a788, a789) {
            1u64
        } else {
            f15(v0)?
        }
    };
    let a827 = {
        let a824 = {
            let a790 = 1u64;
            let a791 = 1u64;
            nat_add(a790, a791)?
        };
        let a825 = {
            let a822 = {
                let a792 = 1u64;
                let a793 = 1u64;
                nat_add(a792, a793)?
            };
            let a823 = {
                let a820 = {
                    let a794 = 1u64;
                    let a795 = 1u64;
                    nat_add(a794, a795)?
                };
                let a821 = {
                    let a818 = {
                        let a796 = 1u64;
                        let a797 = 1u64;
                        nat_add(a796, a797)?
                    };
                    let a819 = {
                        let a816 = {
                            let a798 = 1u64;
                            let a799 = 1u64;
                            nat_add(a798, a799)?
                        };
                        let a817 = {
                            let a814 = {
                                let a800 = 1u64;
                                let a801 = 1u64;
                                nat_add(a800, a801)?
                            };
                            let a815 = {
                                let a812 = {
                                    let a802 = 1u64;
                                    let a803 = 1u64;
                                    nat_add(a802, a803)?
                                };
                                let a813 = {
                                    let a810 = {
                                        let a804 = 1u64;
                                        let a805 = 1u64;
                                        nat_add(a804, a805)?
                                    };
                                    let a811 = {
                                        let a808 = {
                                            let a806 = 1u64;
                                            let a807 = 1u64;
                                            nat_add(a806, a807)?
                                        };
                                        let a809 = 1u64;
                                        nat_mul(a808, a809)?
                                    };
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
    nat_add(a826, a827)
}

pub fn f20(v0: u64) -> R<u64> {
    let a852 = f16(v0)?;
    let a853 = {
        let a828 = f15(v0)?;
        let a829 = 0u64;
        if nat_eq(a828, a829) {
            0u64
        } else {
            f5({
                let a850 = {
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
                                    let a840 = {
                                        let a834 = 1u64;
                                        let a835 = 1u64;
                                        nat_add(a834, a835)?
                                    };
                                    let a841 = {
                                        let a838 = {
                                            let a836 = 1u64;
                                            let a837 = 1u64;
                                            nat_add(a836, a837)?
                                        };
                                        let a839 = 1u64;
                                        nat_mul(a838, a839)?
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
            })?
        }
    };
    nat_add(a852, a853)
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a854 = v1;
    let a855 = 0u64;
    if nat_eq(a854, a855) {
        Ok(v0)
    } else {
        let a880 = {
            let a878 = {
                let a856 = 1u64;
                let a857 = 1u64;
                nat_add(a856, a857)?
            };
            let a879 = {
                let a876 = {
                    let a858 = 1u64;
                    let a859 = 1u64;
                    nat_add(a858, a859)?
                };
                let a877 = {
                    let a874 = {
                        let a860 = 1u64;
                        let a861 = 1u64;
                        nat_add(a860, a861)?
                    };
                    let a875 = {
                        let a872 = {
                            let a862 = 1u64;
                            let a863 = 1u64;
                            nat_add(a862, a863)?
                        };
                        let a873 = {
                            let a870 = {
                                let a864 = 1u64;
                                let a865 = 1u64;
                                nat_add(a864, a865)?
                            };
                            let a871 = {
                                let a868 = {
                                    let a866 = 1u64;
                                    let a867 = 1u64;
                                    nat_add(a866, a867)?
                                };
                                let a869 = 1u64;
                                nat_mul(a868, a869)?
                            };
                            nat_mul(a870, a871)?
                        };
                        nat_mul(a872, a873)?
                    };
                    nat_mul(a874, a875)?
                };
                nat_mul(a876, a877)?
            };
            nat_mul(a878, a879)?
        };
        let a881 = v1;
        if nat_le(a880, a881) {
            let a882 = v0;
            let a883 = 0u64;
            if nat_eq(a882, a883) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f5(v1)?;
            f24({
                let a884 = v0;
                let a885 = v2;
                let a886 = 0u64;
                nat_quot(a884, a885, a886)
            }, {
                let a890 = 0u64;
                let a891 = {
                    let a887 = v0;
                    let a888 = v2;
                    let a889 = 0u64;
                    nat_rem(a887, a888, a889)
                };
                nat_lt(a890, a891)
            })
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a914 = f5({
        let a912 = {
            let a910 = {
                let a892 = 1u64;
                let a893 = 1u64;
                nat_add(a892, a893)?
            };
            let a911 = {
                let a908 = {
                    let a906 = {
                        let a894 = 1u64;
                        let a895 = 1u64;
                        nat_add(a894, a895)?
                    };
                    let a907 = {
                        let a904 = {
                            let a896 = 1u64;
                            let a897 = 1u64;
                            nat_add(a896, a897)?
                        };
                        let a905 = {
                            let a902 = {
                                let a900 = {
                                    let a898 = 1u64;
                                    let a899 = 1u64;
                                    nat_add(a898, a899)?
                                };
                                let a901 = 1u64;
                                nat_mul(a900, a901)?
                            };
                            let a903 = 1u64;
                            nat_add(a902, a903)?
                        };
                        nat_mul(a904, a905)?
                    };
                    nat_mul(a906, a907)?
                };
                let a909 = 1u64;
                nat_add(a908, a909)?
            };
            nat_mul(a910, a911)?
        };
        let a913 = 1u64;
        nat_add(a912, a913)?
    })?;
    let a915 = v2;
    if nat_le(a914, a915) {
        f25(v0, {
            let a916 = v1;
            let a917 = 1u64;
            nat_add(a916, a917)?
        }, f21(v2, 1u64)?)
    } else {
        let v3: Adt2 = f26(v1, v2, {
            let a954 = {
                let a952 = {
                    let a918 = 1u64;
                    let a919 = 1u64;
                    nat_add(a918, a919)?
                };
                let a953 = {
                    let a950 = {
                        let a920 = 1u64;
                        let a921 = 1u64;
                        nat_add(a920, a921)?
                    };
                    let a951 = {
                        let a948 = {
                            let a922 = 1u64;
                            let a923 = 1u64;
                            nat_add(a922, a923)?
                        };
                        let a949 = {
                            let a946 = {
                                let a924 = 1u64;
                                let a925 = 1u64;
                                nat_add(a924, a925)?
                            };
                            let a947 = {
                                let a944 = {
                                    let a926 = 1u64;
                                    let a927 = 1u64;
                                    nat_add(a926, a927)?
                                };
                                let a945 = {
                                    let a942 = {
                                        let a928 = 1u64;
                                        let a929 = 1u64;
                                        nat_add(a928, a929)?
                                    };
                                    let a943 = {
                                        let a940 = {
                                            let a930 = 1u64;
                                            let a931 = 1u64;
                                            nat_add(a930, a931)?
                                        };
                                        let a941 = {
                                            let a938 = {
                                                let a932 = 1u64;
                                                let a933 = 1u64;
                                                nat_add(a932, a933)?
                                            };
                                            let a939 = {
                                                let a936 = {
                                                    let a934 = 1u64;
                                                    let a935 = 1u64;
                                                    nat_add(a934, a935)?
                                                };
                                                let a937 = 1u64;
                                                nat_mul(a936, a937)?
                                            };
                                            nat_mul(a938, a939)?
                                        };
                                        nat_mul(a940, a941)?
                                    };
                                    nat_mul(a942, a943)?
                                };
                                nat_mul(a944, a945)?
                            };
                            nat_mul(a946, a947)?
                        };
                        nat_mul(a948, a949)?
                    };
                    nat_mul(a950, a951)?
                };
                nat_mul(a952, a953)?
            };
            let a955 = 1u64;
            nat_add(a954, a955)?
        }, f5({
            let a974 = {
                let a956 = 1u64;
                let a957 = 1u64;
                nat_add(a956, a957)?
            };
            let a975 = {
                let a972 = {
                    let a970 = {
                        let a958 = 1u64;
                        let a959 = 1u64;
                        nat_add(a958, a959)?
                    };
                    let a971 = {
                        let a968 = {
                            let a960 = 1u64;
                            let a961 = 1u64;
                            nat_add(a960, a961)?
                        };
                        let a969 = {
                            let a966 = {
                                let a964 = {
                                    let a962 = 1u64;
                                    let a963 = 1u64;
                                    nat_add(a962, a963)?
                                };
                                let a965 = 1u64;
                                nat_mul(a964, a965)?
                            };
                            let a967 = 1u64;
                            nat_add(a966, a967)?
                        };
                        nat_mul(a968, a969)?
                    };
                    nat_mul(a970, a971)?
                };
                let a973 = 1u64;
                nat_add(a972, a973)?
            };
            nat_mul(a974, a975)?
        })?)?;
        f25(v0, match v3.clone() {
            Adt2::C0(h976, _, _, _) => {
                h976
            }
        }, match v3 {
            Adt2::C0(_, h977, _, _) => {
                h977
            }
        })
    }
}

pub fn f23(v0: u64) -> R<Adt2> {
    f26(f19(v0)?, f20(v0)?, 0u64, f5({
        let a998 = {
            let a996 = {
                let a978 = 1u64;
                let a979 = 1u64;
                nat_add(a978, a979)?
            };
            let a997 = {
                let a994 = {
                    let a992 = {
                        let a980 = 1u64;
                        let a981 = 1u64;
                        nat_add(a980, a981)?
                    };
                    let a993 = {
                        let a990 = {
                            let a988 = {
                                let a982 = 1u64;
                                let a983 = 1u64;
                                nat_add(a982, a983)?
                            };
                            let a989 = {
                                let a986 = {
                                    let a984 = 1u64;
                                    let a985 = 1u64;
                                    nat_add(a984, a985)?
                                };
                                let a987 = 1u64;
                                nat_mul(a986, a987)?
                            };
                            nat_mul(a988, a989)?
                        };
                        let a991 = 1u64;
                        nat_add(a990, a991)?
                    };
                    nat_mul(a992, a993)?
                };
                let a995 = 1u64;
                nat_add(a994, a995)?
            };
            nat_mul(a996, a997)?
        };
        let a999 = 1u64;
        nat_add(a998, a999)?
    })?)
}

pub fn f24(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1007 = {
            let a1004 = v0;
            let a1005 = {
                let a1002 = {
                    let a1000 = 1u64;
                    let a1001 = 1u64;
                    nat_add(a1000, a1001)?
                };
                let a1003 = 1u64;
                nat_mul(a1002, a1003)?
            };
            let a1006 = 0u64;
            nat_rem(a1004, a1005, a1006)
        };
        let a1008 = 0u64;
        nat_eq(a1007, a1008)
    } else {
        false
    } {
        let a1009 = v0;
        let a1010 = 1u64;
        nat_add(a1009, a1010)
    } else {
        Ok(v0)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1049 = v1;
    let a1050 = {
        let a1047 = {
            let a1045 = {
                let a1011 = 1u64;
                let a1012 = 1u64;
                nat_add(a1011, a1012)?
            };
            let a1046 = {
                let a1043 = {
                    let a1013 = 1u64;
                    let a1014 = 1u64;
                    nat_add(a1013, a1014)?
                };
                let a1044 = {
                    let a1041 = {
                        let a1015 = 1u64;
                        let a1016 = 1u64;
                        nat_add(a1015, a1016)?
                    };
                    let a1042 = {
                        let a1039 = {
                            let a1017 = 1u64;
                            let a1018 = 1u64;
                            nat_add(a1017, a1018)?
                        };
                        let a1040 = {
                            let a1037 = {
                                let a1019 = 1u64;
                                let a1020 = 1u64;
                                nat_add(a1019, a1020)?
                            };
                            let a1038 = {
                                let a1035 = {
                                    let a1021 = 1u64;
                                    let a1022 = 1u64;
                                    nat_add(a1021, a1022)?
                                };
                                let a1036 = {
                                    let a1033 = {
                                        let a1023 = 1u64;
                                        let a1024 = 1u64;
                                        nat_add(a1023, a1024)?
                                    };
                                    let a1034 = {
                                        let a1031 = {
                                            let a1025 = 1u64;
                                            let a1026 = 1u64;
                                            nat_add(a1025, a1026)?
                                        };
                                        let a1032 = {
                                            let a1029 = {
                                                let a1027 = 1u64;
                                                let a1028 = 1u64;
                                                nat_add(a1027, a1028)?
                                            };
                                            let a1030 = 1u64;
                                            nat_mul(a1029, a1030)?
                                        };
                                        nat_mul(a1031, a1032)?
                                    };
                                    nat_mul(a1033, a1034)?
                                };
                                nat_mul(a1035, a1036)?
                            };
                            nat_mul(a1037, a1038)?
                        };
                        nat_mul(a1039, a1040)?
                    };
                    nat_mul(a1041, a1042)?
                };
                nat_mul(a1043, a1044)?
            };
            nat_mul(a1045, a1046)?
        };
        let a1048 = 1u64;
        nat_add(a1047, a1048)?
    };
    if nat_lt(a1049, a1050) {
        f27(v0, {
            let a1087 = {
                let a1085 = {
                    let a1051 = 1u64;
                    let a1052 = 1u64;
                    nat_add(a1051, a1052)?
                };
                let a1086 = {
                    let a1083 = {
                        let a1053 = 1u64;
                        let a1054 = 1u64;
                        nat_add(a1053, a1054)?
                    };
                    let a1084 = {
                        let a1081 = {
                            let a1055 = 1u64;
                            let a1056 = 1u64;
                            nat_add(a1055, a1056)?
                        };
                        let a1082 = {
                            let a1079 = {
                                let a1057 = 1u64;
                                let a1058 = 1u64;
                                nat_add(a1057, a1058)?
                            };
                            let a1080 = {
                                let a1077 = {
                                    let a1059 = 1u64;
                                    let a1060 = 1u64;
                                    nat_add(a1059, a1060)?
                                };
                                let a1078 = {
                                    let a1075 = {
                                        let a1061 = 1u64;
                                        let a1062 = 1u64;
                                        nat_add(a1061, a1062)?
                                    };
                                    let a1076 = {
                                        let a1073 = {
                                            let a1063 = 1u64;
                                            let a1064 = 1u64;
                                            nat_add(a1063, a1064)?
                                        };
                                        let a1074 = {
                                            let a1071 = {
                                                let a1065 = 1u64;
                                                let a1066 = 1u64;
                                                nat_add(a1065, a1066)?
                                            };
                                            let a1072 = {
                                                let a1069 = {
                                                    let a1067 = 1u64;
                                                    let a1068 = 1u64;
                                                    nat_add(a1067, a1068)?
                                                };
                                                let a1070 = 1u64;
                                                nat_mul(a1069, a1070)?
                                            };
                                            nat_mul(a1071, a1072)?
                                        };
                                        nat_mul(a1073, a1074)?
                                    };
                                    nat_mul(a1075, a1076)?
                                };
                                nat_mul(a1077, a1078)?
                            };
                            nat_mul(a1079, a1080)?
                        };
                        nat_mul(a1081, a1082)?
                    };
                    nat_mul(a1083, a1084)?
                };
                nat_mul(a1085, a1086)?
            };
            let a1088 = 1u64;
            nat_add(a1087, a1088)?
        }, f21(v2, {
            let a1127 = {
                let a1125 = {
                    let a1123 = {
                        let a1089 = 1u64;
                        let a1090 = 1u64;
                        nat_add(a1089, a1090)?
                    };
                    let a1124 = {
                        let a1121 = {
                            let a1091 = 1u64;
                            let a1092 = 1u64;
                            nat_add(a1091, a1092)?
                        };
                        let a1122 = {
                            let a1119 = {
                                let a1093 = 1u64;
                                let a1094 = 1u64;
                                nat_add(a1093, a1094)?
                            };
                            let a1120 = {
                                let a1117 = {
                                    let a1095 = 1u64;
                                    let a1096 = 1u64;
                                    nat_add(a1095, a1096)?
                                };
                                let a1118 = {
                                    let a1115 = {
                                        let a1097 = 1u64;
                                        let a1098 = 1u64;
                                        nat_add(a1097, a1098)?
                                    };
                                    let a1116 = {
                                        let a1113 = {
                                            let a1099 = 1u64;
                                            let a1100 = 1u64;
                                            nat_add(a1099, a1100)?
                                        };
                                        let a1114 = {
                                            let a1111 = {
                                                let a1101 = 1u64;
                                                let a1102 = 1u64;
                                                nat_add(a1101, a1102)?
                                            };
                                            let a1112 = {
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
                                            nat_mul(a1111, a1112)?
                                        };
                                        nat_mul(a1113, a1114)?
                                    };
                                    nat_mul(a1115, a1116)?
                                };
                                nat_mul(a1117, a1118)?
                            };
                            nat_mul(a1119, a1120)?
                        };
                        nat_mul(a1121, a1122)?
                    };
                    nat_mul(a1123, a1124)?
                };
                let a1126 = 1u64;
                nat_add(a1125, a1126)?
            };
            let a1128 = v1;
            nat_sub(a1127, a1128)
        })?)
    } else {
        f27(v0, v1, v2)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    let (h1129, _) = f28(Fn1::F29, {
        let a1148 = {
            let a1130 = 1u64;
            let a1131 = 1u64;
            nat_add(a1130, a1131)?
        };
        let a1149 = {
            let a1146 = {
                let a1132 = 1u64;
                let a1133 = 1u64;
                nat_add(a1132, a1133)?
            };
            let a1147 = {
                let a1144 = {
                    let a1134 = 1u64;
                    let a1135 = 1u64;
                    nat_add(a1134, a1135)?
                };
                let a1145 = {
                    let a1142 = {
                        let a1136 = 1u64;
                        let a1137 = 1u64;
                        nat_add(a1136, a1137)?
                    };
                    let a1143 = {
                        let a1140 = {
                            let a1138 = 1u64;
                            let a1139 = 1u64;
                            nat_add(a1138, a1139)?
                        };
                        let a1141 = 1u64;
                        nat_mul(a1140, a1141)?
                    };
                    nat_mul(a1142, a1143)?
                };
                nat_mul(a1144, a1145)?
            };
            nat_mul(a1146, a1147)?
        };
        nat_mul(a1148, a1149)?
    }, Adt2::C0(v0, v1, v2, v3))?;
    Ok(h1129)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1162 = v2;
        let a1163 = {
            let a1160 = {
                let a1150 = 1u64;
                let a1151 = 1u64;
                nat_add(a1150, a1151)?
            };
            let a1161 = {
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
            nat_mul(a1160, a1161)?
        };
        let a1164 = 0u64;
        nat_rem(a1162, a1163, a1164)
    };
    let v4: u64 = {
        let a1177 = v2;
        let a1178 = {
            let a1175 = {
                let a1165 = 1u64;
                let a1166 = 1u64;
                nat_add(a1165, a1166)?
            };
            let a1176 = {
                let a1173 = {
                    let a1167 = 1u64;
                    let a1168 = 1u64;
                    nat_add(a1167, a1168)?
                };
                let a1174 = {
                    let a1171 = {
                        let a1169 = 1u64;
                        let a1170 = 1u64;
                        nat_add(a1169, a1170)?
                    };
                    let a1172 = 1u64;
                    nat_mul(a1171, a1172)?
                };
                nat_mul(a1173, a1174)?
            };
            nat_mul(a1175, a1176)?
        };
        let a1179 = 0u64;
        nat_quot(a1177, a1178, a1179)
    };
    let v5: bool = {
        let a1188 = {
            let a1186 = {
                let a1180 = 1u64;
                let a1181 = 1u64;
                nat_add(a1180, a1181)?
            };
            let a1187 = {
                let a1184 = {
                    let a1182 = 1u64;
                    let a1183 = 1u64;
                    nat_add(a1182, a1183)?
                };
                let a1185 = 1u64;
                nat_mul(a1184, a1185)?
            };
            nat_mul(a1186, a1187)?
        };
        let a1189 = v3;
        if nat_lt(a1188, a1189) {
            true
        } else {
            let a1198 = v3;
            let a1199 = {
                let a1196 = {
                    let a1190 = 1u64;
                    let a1191 = 1u64;
                    nat_add(a1190, a1191)?
                };
                let a1197 = {
                    let a1194 = {
                        let a1192 = 1u64;
                        let a1193 = 1u64;
                        nat_add(a1192, a1193)?
                    };
                    let a1195 = 1u64;
                    nat_mul(a1194, a1195)?
                };
                nat_mul(a1196, a1197)?
            };
            if nat_eq(a1198, a1199) {
                let a1207 = {
                    let a1204 = v4;
                    let a1205 = {
                        let a1202 = {
                            let a1200 = 1u64;
                            let a1201 = 1u64;
                            nat_add(a1200, a1201)?
                        };
                        let a1203 = 1u64;
                        nat_mul(a1202, a1203)?
                    };
                    let a1206 = 0u64;
                    nat_rem(a1204, a1205, a1206)
                };
                let a1208 = 1u64;
                nat_eq(a1207, a1208)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1209 = v4;
        let a1210 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1209, a1210)?
    };
    let v7: bool = {
        let a1239 = {
            let a1237 = {
                let a1213 = {
                    let a1211 = 1u64;
                    let a1212 = 1u64;
                    nat_add(a1211, a1212)?
                };
                let a1214 = 1u64;
                nat_mul(a1213, a1214)?
            };
            let a1238 = f5({
                let a1235 = {
                    let a1233 = {
                        let a1215 = 1u64;
                        let a1216 = 1u64;
                        nat_add(a1215, a1216)?
                    };
                    let a1234 = {
                        let a1231 = {
                            let a1229 = {
                                let a1217 = 1u64;
                                let a1218 = 1u64;
                                nat_add(a1217, a1218)?
                            };
                            let a1230 = {
                                let a1227 = {
                                    let a1225 = {
                                        let a1219 = 1u64;
                                        let a1220 = 1u64;
                                        nat_add(a1219, a1220)?
                                    };
                                    let a1226 = {
                                        let a1223 = {
                                            let a1221 = 1u64;
                                            let a1222 = 1u64;
                                            nat_add(a1221, a1222)?
                                        };
                                        let a1224 = 1u64;
                                        nat_mul(a1223, a1224)?
                                    };
                                    nat_mul(a1225, a1226)?
                                };
                                let a1228 = 1u64;
                                nat_add(a1227, a1228)?
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
            })?;
            nat_mul(a1237, a1238)?
        };
        let a1240 = v6;
        nat_le(a1239, a1240)
    };
    let v8: u64 = if v7 {
        let a1245 = v6;
        let a1246 = {
            let a1243 = {
                let a1241 = 1u64;
                let a1242 = 1u64;
                nat_add(a1241, a1242)?
            };
            let a1244 = 1u64;
            nat_mul(a1243, a1244)?
        };
        let a1247 = 0u64;
        nat_quot(a1245, a1246, a1247)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1248 = v1;
        let a1249 = 1u64;
        nat_add(a1248, a1249)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1288 = v9;
        let a1289 = {
            let a1286 = {
                let a1284 = {
                    let a1250 = 1u64;
                    let a1251 = 1u64;
                    nat_add(a1250, a1251)?
                };
                let a1285 = {
                    let a1282 = {
                        let a1252 = 1u64;
                        let a1253 = 1u64;
                        nat_add(a1252, a1253)?
                    };
                    let a1283 = {
                        let a1280 = {
                            let a1254 = 1u64;
                            let a1255 = 1u64;
                            nat_add(a1254, a1255)?
                        };
                        let a1281 = {
                            let a1278 = {
                                let a1256 = 1u64;
                                let a1257 = 1u64;
                                nat_add(a1256, a1257)?
                            };
                            let a1279 = {
                                let a1276 = {
                                    let a1258 = 1u64;
                                    let a1259 = 1u64;
                                    nat_add(a1258, a1259)?
                                };
                                let a1277 = {
                                    let a1274 = {
                                        let a1260 = 1u64;
                                        let a1261 = 1u64;
                                        nat_add(a1260, a1261)?
                                    };
                                    let a1275 = {
                                        let a1272 = {
                                            let a1262 = 1u64;
                                            let a1263 = 1u64;
                                            nat_add(a1262, a1263)?
                                        };
                                        let a1273 = {
                                            let a1270 = {
                                                let a1264 = 1u64;
                                                let a1265 = 1u64;
                                                nat_add(a1264, a1265)?
                                            };
                                            let a1271 = {
                                                let a1268 = {
                                                    let a1266 = 1u64;
                                                    let a1267 = 1u64;
                                                    nat_add(a1266, a1267)?
                                                };
                                                let a1269 = 1u64;
                                                nat_mul(a1268, a1269)?
                                            };
                                            nat_mul(a1270, a1271)?
                                        };
                                        nat_mul(a1272, a1273)?
                                    };
                                    nat_mul(a1274, a1275)?
                                };
                                nat_mul(a1276, a1277)?
                            };
                            nat_mul(a1278, a1279)?
                        };
                        nat_mul(a1280, a1281)?
                    };
                    nat_mul(a1282, a1283)?
                };
                nat_mul(a1284, a1285)?
            };
            let a1287 = 1u64;
            nat_add(a1286, a1287)?
        };
        if if nat_eq(a1288, a1289) {
            let a1312 = v8;
            let a1313 = f5({
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
            })?;
            nat_lt(a1312, a1313)
        } else {
            false
        } {
            0u64
        } else {
            let a1350 = v9;
            let a1351 = {
                let a1348 = {
                    let a1314 = 1u64;
                    let a1315 = 1u64;
                    nat_add(a1314, a1315)?
                };
                let a1349 = {
                    let a1346 = {
                        let a1316 = 1u64;
                        let a1317 = 1u64;
                        nat_add(a1316, a1317)?
                    };
                    let a1347 = {
                        let a1344 = {
                            let a1318 = 1u64;
                            let a1319 = 1u64;
                            nat_add(a1318, a1319)?
                        };
                        let a1345 = {
                            let a1342 = {
                                let a1320 = 1u64;
                                let a1321 = 1u64;
                                nat_add(a1320, a1321)?
                            };
                            let a1343 = {
                                let a1340 = {
                                    let a1322 = 1u64;
                                    let a1323 = 1u64;
                                    nat_add(a1322, a1323)?
                                };
                                let a1341 = {
                                    let a1338 = {
                                        let a1324 = 1u64;
                                        let a1325 = 1u64;
                                        nat_add(a1324, a1325)?
                                    };
                                    let a1339 = {
                                        let a1336 = {
                                            let a1326 = 1u64;
                                            let a1327 = 1u64;
                                            nat_add(a1326, a1327)?
                                        };
                                        let a1337 = {
                                            let a1334 = {
                                                let a1328 = 1u64;
                                                let a1329 = 1u64;
                                                nat_add(a1328, a1329)?
                                            };
                                            let a1335 = {
                                                let a1332 = {
                                                    let a1330 = 1u64;
                                                    let a1331 = 1u64;
                                                    nat_add(a1330, a1331)?
                                                };
                                                let a1333 = 1u64;
                                                nat_mul(a1332, a1333)?
                                            };
                                            nat_mul(a1334, a1335)?
                                        };
                                        nat_mul(a1336, a1337)?
                                    };
                                    nat_mul(a1338, a1339)?
                                };
                                nat_mul(a1340, a1341)?
                            };
                            nat_mul(a1342, a1343)?
                        };
                        nat_mul(a1344, a1345)?
                    };
                    nat_mul(a1346, a1347)?
                };
                nat_mul(a1348, a1349)?
            };
            nat_sub(a1350, a1351)
        }
    };
    let a1352 = v8;
    let a1353 = 0u64;
    if nat_eq(a1352, a1353) {
        f13(v0)
    } else {
        let a1406 = {
            let a1404 = {
                let a1402 = {
                    let a1354 = 1u64;
                    let a1355 = 1u64;
                    nat_add(a1354, a1355)?
                };
                let a1403 = {
                    let a1400 = {
                        let a1398 = {
                            let a1356 = 1u64;
                            let a1357 = 1u64;
                            nat_add(a1356, a1357)?
                        };
                        let a1399 = {
                            let a1396 = {
                                let a1394 = {
                                    let a1358 = 1u64;
                                    let a1359 = 1u64;
                                    nat_add(a1358, a1359)?
                                };
                                let a1395 = {
                                    let a1392 = {
                                        let a1390 = {
                                            let a1360 = 1u64;
                                            let a1361 = 1u64;
                                            nat_add(a1360, a1361)?
                                        };
                                        let a1391 = {
                                            let a1388 = {
                                                let a1386 = {
                                                    let a1362 = 1u64;
                                                    let a1363 = 1u64;
                                                    nat_add(a1362, a1363)?
                                                };
                                                let a1387 = {
                                                    let a1384 = {
                                                        let a1382 = {
                                                            let a1364 = 1u64;
                                                            let a1365 = 1u64;
                                                            nat_add(a1364, a1365)?
                                                        };
                                                        let a1383 = {
                                                            let a1380 = {
                                                                let a1378 = {
                                                                    let a1366 = 1u64;
                                                                    let a1367 = 1u64;
                                                                    nat_add(a1366, a1367)?
                                                                };
                                                                let a1379 = {
                                                                    let a1376 = {
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
                                                                    let a1377 = 1u64;
                                                                    nat_add(a1376, a1377)?
                                                                };
                                                                nat_mul(a1378, a1379)?
                                                            };
                                                            let a1381 = 1u64;
                                                            nat_add(a1380, a1381)?
                                                        };
                                                        nat_mul(a1382, a1383)?
                                                    };
                                                    let a1385 = 1u64;
                                                    nat_add(a1384, a1385)?
                                                };
                                                nat_mul(a1386, a1387)?
                                            };
                                            let a1389 = 1u64;
                                            nat_add(a1388, a1389)?
                                        };
                                        nat_mul(a1390, a1391)?
                                    };
                                    let a1393 = 1u64;
                                    nat_add(a1392, a1393)?
                                };
                                nat_mul(a1394, a1395)?
                            };
                            let a1397 = 1u64;
                            nat_add(a1396, a1397)?
                        };
                        nat_mul(a1398, a1399)?
                    };
                    let a1401 = 1u64;
                    nat_add(a1400, a1401)?
                };
                nat_mul(a1402, a1403)?
            };
            let a1405 = 1u64;
            nat_add(a1404, a1405)?
        };
        let a1407 = v9;
        if nat_le(a1406, a1407) {
            f11(v0)
        } else {
            let a1459 = {
                let a1432 = f13(v0)?;
                let a1433 = {
                    let a1430 = v10;
                    let a1431 = f5({
                        let a1428 = {
                            let a1426 = {
                                let a1408 = 1u64;
                                let a1409 = 1u64;
                                nat_add(a1408, a1409)?
                            };
                            let a1427 = {
                                let a1424 = {
                                    let a1422 = {
                                        let a1410 = 1u64;
                                        let a1411 = 1u64;
                                        nat_add(a1410, a1411)?
                                    };
                                    let a1423 = {
                                        let a1420 = {
                                            let a1418 = {
                                                let a1412 = 1u64;
                                                let a1413 = 1u64;
                                                nat_add(a1412, a1413)?
                                            };
                                            let a1419 = {
                                                let a1416 = {
                                                    let a1414 = 1u64;
                                                    let a1415 = 1u64;
                                                    nat_add(a1414, a1415)?
                                                };
                                                let a1417 = 1u64;
                                                nat_mul(a1416, a1417)?
                                            };
                                            nat_mul(a1418, a1419)?
                                        };
                                        let a1421 = 1u64;
                                        nat_add(a1420, a1421)?
                                    };
                                    nat_mul(a1422, a1423)?
                                };
                                let a1425 = 1u64;
                                nat_add(a1424, a1425)?
                            };
                            nat_mul(a1426, a1427)?
                        };
                        let a1429 = 1u64;
                        nat_add(a1428, a1429)?
                    })?;
                    nat_mul(a1430, a1431)?
                };
                nat_add(a1432, a1433)?
            };
            let a1460 = {
                let a1456 = v8;
                let a1457 = f5({
                    let a1454 = {
                        let a1452 = {
                            let a1434 = 1u64;
                            let a1435 = 1u64;
                            nat_add(a1434, a1435)?
                        };
                        let a1453 = {
                            let a1450 = {
                                let a1448 = {
                                    let a1436 = 1u64;
                                    let a1437 = 1u64;
                                    nat_add(a1436, a1437)?
                                };
                                let a1449 = {
                                    let a1446 = {
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
                                    let a1447 = 1u64;
                                    nat_add(a1446, a1447)?
                                };
                                nat_mul(a1448, a1449)?
                            };
                            let a1451 = 1u64;
                            nat_add(a1450, a1451)?
                        };
                        nat_mul(a1452, a1453)?
                    };
                    let a1455 = 1u64;
                    nat_add(a1454, a1455)?
                })?;
                let a1458 = 0u64;
                nat_rem(a1456, a1457, a1458)
            };
            nat_add(a1459, a1460)
        }
    }
}

pub fn f28(v0: Fn1, p1465: u64, p1466: Adt2) -> R<(Adt2, bool)> {
    let m1464 = (p1465, p1466);
    {
        let mut m1464 = m1464;
        loop {
            let step = {
                let (v1, v2) = m1464;
                let m1461 = v1;
                if m1461 == 0 {
                    Err::<(u64, Adt2), R<(Adt2, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1461 - 1;
                    let m1463 = {
                        let c1462: Fn1 = v0.clone();
                        c1462.apply(v2.clone())?
                    };
                    match m1463 {
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
                Ok(next) => { m1464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt2) -> R<Option<Adt2>> {
    f30(v0)
}

pub fn f30(v0: Adt2) -> R<Option<Adt2>> {
    let a1468 = 0u64;
    let a1469 = match v0.clone() {
        Adt2::C0(_, h1467, _, _) => {
            h1467
        }
    };
    if if if nat_lt(a1468, a1469) {
        let a1472 = match v0.clone() {
            Adt2::C0(_, h1470, _, _) => {
                h1470
            }
        };
        let a1473 = match v0.clone() {
            Adt2::C0(_, _, _, h1471) => {
                h1471
            }
        };
        nat_lt(a1472, a1473)
    } else {
        false
    } {
        let a1476 = match v0.clone() {
            Adt2::C0(_, _, h1474, _) => {
                h1474
            }
        };
        let a1477 = match v0.clone() {
            Adt2::C0(h1475, _, _, _) => {
                h1475
            }
        };
        nat_lt(a1476, a1477)
    } else {
        false
    } {
        Ok(Some(Adt2::C0({
            let a1479 = match v0.clone() {
                Adt2::C0(h1478, _, _, _) => {
                    h1478
                }
            };
            let a1480 = 1u64;
            nat_sub(a1479, a1480)
        }, {
            let a1483 = match v0.clone() {
                Adt2::C0(_, h1481, _, _) => {
                    h1481
                }
            };
            let a1484 = match v0.clone() {
                Adt2::C0(_, h1482, _, _) => {
                    h1482
                }
            };
            nat_add(a1483, a1484)?
        }, match v0.clone() {
            Adt2::C0(_, _, h1485, _) => {
                h1485
            }
        }, match v0 {
            Adt2::C0(_, _, _, h1486) => {
                h1486
            }
        })))
    } else {
        Ok(None::<Adt2>)
    }
}

pub fn run(p0: Adt0) -> R<Adt1> {
    f0(p0)
}
