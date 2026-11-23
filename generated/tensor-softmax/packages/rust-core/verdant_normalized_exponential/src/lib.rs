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
    C1(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F15,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F15 => f15(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F25 => f25(p0),
        }
    }
}

pub fn f0(v0: Adt0) -> R<Option<u64>> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2) => {
            if if f1(v1)? {
                f1(v2)?
            } else {
                false
            } {
                Ok(Some(if f2(v1, v2)? {
                    v2
                } else {
                    v1
                }))
            } else {
                Ok(None::<u64>)
            }
        }
        Adt0::C1(v3, v4) => {
            if if if f1(v3)? {
                f1(v4)?
            } else {
                false
            } {
                let a2 = v4;
                let a3 = 4286578688u64;
                !nat_eq(a2, a3)
            } else {
                false
            } {
                Ok(Some(f3(v3, f4(v4)?)?))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f1(v0: u64) -> R<bool> {
    let a4 = v0;
    let a5 = 4294967296u64;
    if if nat_lt(a4, a5) {
        let a6 = v0;
        let a7 = 2139095040u64;
        !nat_eq(a6, a7)
    } else {
        false
    } {
        Ok(!f5(v0)?)
    } else {
        Ok(false)
    }
}

pub fn f2(v0: u64, v1: u64) -> R<bool> {
    if if f5(v0)? {
        true
    } else {
        f5(v1)?
    } {
        Ok(false)
    } else {
        if if f6(v0)? {
            f6(v1)?
        } else {
            false
        } {
            Ok(false)
        } else {
            let a8 = f7(v0)?;
            let a9 = f7(v1)?;
            if nat_eq(a8, a9) {
                let a10 = f7(v0)?;
                let a11 = 0u64;
                if nat_eq(a10, a11) {
                    let a12 = f8(v0)?;
                    let a13 = f8(v1)?;
                    Ok(nat_lt(a12, a13))
                } else {
                    let a14 = f8(v1)?;
                    let a15 = f8(v0)?;
                    Ok(nat_lt(a14, a15))
                }
            } else {
                let a16 = f7(v0)?;
                let a17 = 1u64;
                Ok(nat_eq(a16, a17))
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    if if f5(v0)? {
        true
    } else {
        f5(v1)?
    } {
        let a104 = {
            let a82 = {
                let a58 = {
                    let a56 = {
                        let a18 = 1u64;
                        let a19 = 1u64;
                        nat_add(a18, a19)?
                    };
                    let a57 = {
                        let a54 = {
                            let a52 = {
                                let a20 = 1u64;
                                let a21 = 1u64;
                                nat_add(a20, a21)?
                            };
                            let a53 = {
                                let a50 = {
                                    let a48 = {
                                        let a22 = 1u64;
                                        let a23 = 1u64;
                                        nat_add(a22, a23)?
                                    };
                                    let a49 = {
                                        let a46 = {
                                            let a44 = {
                                                let a24 = 1u64;
                                                let a25 = 1u64;
                                                nat_add(a24, a25)?
                                            };
                                            let a45 = {
                                                let a42 = {
                                                    let a40 = {
                                                        let a26 = 1u64;
                                                        let a27 = 1u64;
                                                        nat_add(a26, a27)?
                                                    };
                                                    let a41 = {
                                                        let a38 = {
                                                            let a36 = {
                                                                let a28 = 1u64;
                                                                let a29 = 1u64;
                                                                nat_add(a28, a29)?
                                                            };
                                                            let a37 = {
                                                                let a34 = {
                                                                    let a32 = {
                                                                        let a30 = 1u64;
                                                                        let a31 = 1u64;
                                                                        nat_add(a30, a31)?
                                                                    };
                                                                    let a33 = 1u64;
                                                                    nat_mul(a32, a33)?
                                                                };
                                                                let a35 = 1u64;
                                                                nat_add(a34, a35)?
                                                            };
                                                            nat_mul(a36, a37)?
                                                        };
                                                        let a39 = 1u64;
                                                        nat_add(a38, a39)?
                                                    };
                                                    nat_mul(a40, a41)?
                                                };
                                                let a43 = 1u64;
                                                nat_add(a42, a43)?
                                            };
                                            nat_mul(a44, a45)?
                                        };
                                        let a47 = 1u64;
                                        nat_add(a46, a47)?
                                    };
                                    nat_mul(a48, a49)?
                                };
                                let a51 = 1u64;
                                nat_add(a50, a51)?
                            };
                            nat_mul(a52, a53)?
                        };
                        let a55 = 1u64;
                        nat_add(a54, a55)?
                    };
                    nat_mul(a56, a57)?
                };
                let a59 = 1u64;
                nat_add(a58, a59)?
            };
            let a83 = f9({
                let a80 = {
                    let a78 = {
                        let a60 = 1u64;
                        let a61 = 1u64;
                        nat_add(a60, a61)?
                    };
                    let a79 = {
                        let a76 = {
                            let a74 = {
                                let a62 = 1u64;
                                let a63 = 1u64;
                                nat_add(a62, a63)?
                            };
                            let a75 = {
                                let a72 = {
                                    let a70 = {
                                        let a64 = 1u64;
                                        let a65 = 1u64;
                                        nat_add(a64, a65)?
                                    };
                                    let a71 = {
                                        let a68 = {
                                            let a66 = 1u64;
                                            let a67 = 1u64;
                                            nat_add(a66, a67)?
                                        };
                                        let a69 = 1u64;
                                        nat_mul(a68, a69)?
                                    };
                                    nat_mul(a70, a71)?
                                };
                                let a73 = 1u64;
                                nat_add(a72, a73)?
                            };
                            nat_mul(a74, a75)?
                        };
                        let a77 = 1u64;
                        nat_add(a76, a77)?
                    };
                    nat_mul(a78, a79)?
                };
                let a81 = 1u64;
                nat_add(a80, a81)?
            })?;
            nat_mul(a82, a83)?
        };
        let a105 = f9({
            let a102 = {
                let a84 = 1u64;
                let a85 = 1u64;
                nat_add(a84, a85)?
            };
            let a103 = {
                let a100 = {
                    let a98 = {
                        let a86 = 1u64;
                        let a87 = 1u64;
                        nat_add(a86, a87)?
                    };
                    let a99 = {
                        let a96 = {
                            let a94 = {
                                let a88 = 1u64;
                                let a89 = 1u64;
                                nat_add(a88, a89)?
                            };
                            let a95 = {
                                let a92 = {
                                    let a90 = 1u64;
                                    let a91 = 1u64;
                                    nat_add(a90, a91)?
                                };
                                let a93 = 1u64;
                                nat_mul(a92, a93)?
                            };
                            nat_mul(a94, a95)?
                        };
                        let a97 = 1u64;
                        nat_add(a96, a97)?
                    };
                    nat_mul(a98, a99)?
                };
                let a101 = 1u64;
                nat_add(a100, a101)?
            };
            nat_mul(a102, a103)?
        })?;
        nat_add(a104, a105)
    } else {
        if if f10(v0)? {
            f10(v1)?
        } else {
            false
        } {
            let a106 = f7(v0)?;
            let a107 = f7(v1)?;
            if nat_eq(a106, a107) {
                Ok(v0)
            } else {
                let a194 = {
                    let a172 = {
                        let a148 = {
                            let a146 = {
                                let a108 = 1u64;
                                let a109 = 1u64;
                                nat_add(a108, a109)?
                            };
                            let a147 = {
                                let a144 = {
                                    let a142 = {
                                        let a110 = 1u64;
                                        let a111 = 1u64;
                                        nat_add(a110, a111)?
                                    };
                                    let a143 = {
                                        let a140 = {
                                            let a138 = {
                                                let a112 = 1u64;
                                                let a113 = 1u64;
                                                nat_add(a112, a113)?
                                            };
                                            let a139 = {
                                                let a136 = {
                                                    let a134 = {
                                                        let a114 = 1u64;
                                                        let a115 = 1u64;
                                                        nat_add(a114, a115)?
                                                    };
                                                    let a135 = {
                                                        let a132 = {
                                                            let a130 = {
                                                                let a116 = 1u64;
                                                                let a117 = 1u64;
                                                                nat_add(a116, a117)?
                                                            };
                                                            let a131 = {
                                                                let a128 = {
                                                                    let a126 = {
                                                                        let a118 = 1u64;
                                                                        let a119 = 1u64;
                                                                        nat_add(a118, a119)?
                                                                    };
                                                                    let a127 = {
                                                                        let a124 = {
                                                                            let a122 = {
                                                                                let a120 = 1u64;
                                                                                let a121 = 1u64;
                                                                                nat_add(a120, a121)?
                                                                            };
                                                                            let a123 = 1u64;
                                                                            nat_mul(a122, a123)?
                                                                        };
                                                                        let a125 = 1u64;
                                                                        nat_add(a124, a125)?
                                                                    };
                                                                    nat_mul(a126, a127)?
                                                                };
                                                                let a129 = 1u64;
                                                                nat_add(a128, a129)?
                                                            };
                                                            nat_mul(a130, a131)?
                                                        };
                                                        let a133 = 1u64;
                                                        nat_add(a132, a133)?
                                                    };
                                                    nat_mul(a134, a135)?
                                                };
                                                let a137 = 1u64;
                                                nat_add(a136, a137)?
                                            };
                                            nat_mul(a138, a139)?
                                        };
                                        let a141 = 1u64;
                                        nat_add(a140, a141)?
                                    };
                                    nat_mul(a142, a143)?
                                };
                                let a145 = 1u64;
                                nat_add(a144, a145)?
                            };
                            nat_mul(a146, a147)?
                        };
                        let a149 = 1u64;
                        nat_add(a148, a149)?
                    };
                    let a173 = f9({
                        let a170 = {
                            let a168 = {
                                let a150 = 1u64;
                                let a151 = 1u64;
                                nat_add(a150, a151)?
                            };
                            let a169 = {
                                let a166 = {
                                    let a164 = {
                                        let a152 = 1u64;
                                        let a153 = 1u64;
                                        nat_add(a152, a153)?
                                    };
                                    let a165 = {
                                        let a162 = {
                                            let a160 = {
                                                let a154 = 1u64;
                                                let a155 = 1u64;
                                                nat_add(a154, a155)?
                                            };
                                            let a161 = {
                                                let a158 = {
                                                    let a156 = 1u64;
                                                    let a157 = 1u64;
                                                    nat_add(a156, a157)?
                                                };
                                                let a159 = 1u64;
                                                nat_mul(a158, a159)?
                                            };
                                            nat_mul(a160, a161)?
                                        };
                                        let a163 = 1u64;
                                        nat_add(a162, a163)?
                                    };
                                    nat_mul(a164, a165)?
                                };
                                let a167 = 1u64;
                                nat_add(a166, a167)?
                            };
                            nat_mul(a168, a169)?
                        };
                        let a171 = 1u64;
                        nat_add(a170, a171)?
                    })?;
                    nat_mul(a172, a173)?
                };
                let a195 = f9({
                    let a192 = {
                        let a174 = 1u64;
                        let a175 = 1u64;
                        nat_add(a174, a175)?
                    };
                    let a193 = {
                        let a190 = {
                            let a188 = {
                                let a176 = 1u64;
                                let a177 = 1u64;
                                nat_add(a176, a177)?
                            };
                            let a189 = {
                                let a186 = {
                                    let a184 = {
                                        let a178 = 1u64;
                                        let a179 = 1u64;
                                        nat_add(a178, a179)?
                                    };
                                    let a185 = {
                                        let a182 = {
                                            let a180 = 1u64;
                                            let a181 = 1u64;
                                            nat_add(a180, a181)?
                                        };
                                        let a183 = 1u64;
                                        nat_mul(a182, a183)?
                                    };
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
                })?;
                nat_add(a194, a195)
            }
        } else {
            if f10(v0)? {
                Ok(v0)
            } else {
                if f10(v1)? {
                    Ok(v1)
                } else {
                    let a196 = f8(v1)?;
                    let a197 = f8(v0)?;
                    if nat_le(a196, a197) {
                        f11(v0, v1)
                    } else {
                        f11(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> R<u64> {
    let a244 = {
        let a222 = v0;
        let a223 = f9({
            let a220 = {
                let a218 = {
                    let a198 = 1u64;
                    let a199 = 1u64;
                    nat_add(a198, a199)?
                };
                let a219 = {
                    let a216 = {
                        let a214 = {
                            let a200 = 1u64;
                            let a201 = 1u64;
                            nat_add(a200, a201)?
                        };
                        let a215 = {
                            let a212 = {
                                let a210 = {
                                    let a202 = 1u64;
                                    let a203 = 1u64;
                                    nat_add(a202, a203)?
                                };
                                let a211 = {
                                    let a208 = {
                                        let a206 = {
                                            let a204 = 1u64;
                                            let a205 = 1u64;
                                            nat_add(a204, a205)?
                                        };
                                        let a207 = 1u64;
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
        })?;
        nat_add(a222, a223)?
    };
    let a245 = f9({
        let a242 = {
            let a224 = 1u64;
            let a225 = 1u64;
            nat_add(a224, a225)?
        };
        let a243 = {
            let a240 = {
                let a226 = 1u64;
                let a227 = 1u64;
                nat_add(a226, a227)?
            };
            let a241 = {
                let a238 = {
                    let a228 = 1u64;
                    let a229 = 1u64;
                    nat_add(a228, a229)?
                };
                let a239 = {
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
                nat_mul(a238, a239)?
            };
            nat_mul(a240, a241)?
        };
        nat_mul(a242, a243)?
    })?;
    let a246 = 0u64;
    Ok(nat_rem(a244, a245, a246))
}

pub fn f5(v0: u64) -> R<bool> {
    let a289 = f12(v0)?;
    let a290 = {
        let a287 = {
            let a285 = {
                let a247 = 1u64;
                let a248 = 1u64;
                nat_add(a247, a248)?
            };
            let a286 = {
                let a283 = {
                    let a281 = {
                        let a249 = 1u64;
                        let a250 = 1u64;
                        nat_add(a249, a250)?
                    };
                    let a282 = {
                        let a279 = {
                            let a277 = {
                                let a251 = 1u64;
                                let a252 = 1u64;
                                nat_add(a251, a252)?
                            };
                            let a278 = {
                                let a275 = {
                                    let a273 = {
                                        let a253 = 1u64;
                                        let a254 = 1u64;
                                        nat_add(a253, a254)?
                                    };
                                    let a274 = {
                                        let a271 = {
                                            let a269 = {
                                                let a255 = 1u64;
                                                let a256 = 1u64;
                                                nat_add(a255, a256)?
                                            };
                                            let a270 = {
                                                let a267 = {
                                                    let a265 = {
                                                        let a257 = 1u64;
                                                        let a258 = 1u64;
                                                        nat_add(a257, a258)?
                                                    };
                                                    let a266 = {
                                                        let a263 = {
                                                            let a261 = {
                                                                let a259 = 1u64;
                                                                let a260 = 1u64;
                                                                nat_add(a259, a260)?
                                                            };
                                                            let a262 = 1u64;
                                                            nat_mul(a261, a262)?
                                                        };
                                                        let a264 = 1u64;
                                                        nat_add(a263, a264)?
                                                    };
                                                    nat_mul(a265, a266)?
                                                };
                                                let a268 = 1u64;
                                                nat_add(a267, a268)?
                                            };
                                            nat_mul(a269, a270)?
                                        };
                                        let a272 = 1u64;
                                        nat_add(a271, a272)?
                                    };
                                    nat_mul(a273, a274)?
                                };
                                let a276 = 1u64;
                                nat_add(a275, a276)?
                            };
                            nat_mul(a277, a278)?
                        };
                        let a280 = 1u64;
                        nat_add(a279, a280)?
                    };
                    nat_mul(a281, a282)?
                };
                let a284 = 1u64;
                nat_add(a283, a284)?
            };
            nat_mul(a285, a286)?
        };
        let a288 = 1u64;
        nat_add(a287, a288)?
    };
    if nat_eq(a289, a290) {
        let a291 = 0u64;
        let a292 = f13(v0)?;
        Ok(nat_lt(a291, a292))
    } else {
        Ok(false)
    }
}

pub fn f6(v0: u64) -> R<bool> {
    let a293 = f8(v0)?;
    let a294 = 0u64;
    Ok(nat_eq(a293, a294))
}

pub fn f7(v0: u64) -> R<u64> {
    let a319 = v0;
    let a320 = f9({
        let a317 = {
            let a315 = {
                let a295 = 1u64;
                let a296 = 1u64;
                nat_add(a295, a296)?
            };
            let a316 = {
                let a313 = {
                    let a311 = {
                        let a297 = 1u64;
                        let a298 = 1u64;
                        nat_add(a297, a298)?
                    };
                    let a312 = {
                        let a309 = {
                            let a307 = {
                                let a299 = 1u64;
                                let a300 = 1u64;
                                nat_add(a299, a300)?
                            };
                            let a308 = {
                                let a305 = {
                                    let a303 = {
                                        let a301 = 1u64;
                                        let a302 = 1u64;
                                        nat_add(a301, a302)?
                                    };
                                    let a304 = 1u64;
                                    nat_mul(a303, a304)?
                                };
                                let a306 = 1u64;
                                nat_add(a305, a306)?
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
    Ok(nat_quot(a319, a320, a321))
}

pub fn f8(v0: u64) -> R<u64> {
    let a346 = v0;
    let a347 = f9({
        let a344 = {
            let a342 = {
                let a322 = 1u64;
                let a323 = 1u64;
                nat_add(a322, a323)?
            };
            let a343 = {
                let a340 = {
                    let a338 = {
                        let a324 = 1u64;
                        let a325 = 1u64;
                        nat_add(a324, a325)?
                    };
                    let a339 = {
                        let a336 = {
                            let a334 = {
                                let a326 = 1u64;
                                let a327 = 1u64;
                                nat_add(a326, a327)?
                            };
                            let a335 = {
                                let a332 = {
                                    let a330 = {
                                        let a328 = 1u64;
                                        let a329 = 1u64;
                                        nat_add(a328, a329)?
                                    };
                                    let a331 = 1u64;
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
    })?;
    let a348 = 0u64;
    Ok(nat_rem(a346, a347, a348))
}

pub fn f9(v0: u64) -> R<u64> {
    f14(Fn0::F15, v0, 1u64)
}

pub fn f10(v0: u64) -> R<bool> {
    let a391 = f12(v0)?;
    let a392 = {
        let a389 = {
            let a387 = {
                let a349 = 1u64;
                let a350 = 1u64;
                nat_add(a349, a350)?
            };
            let a388 = {
                let a385 = {
                    let a383 = {
                        let a351 = 1u64;
                        let a352 = 1u64;
                        nat_add(a351, a352)?
                    };
                    let a384 = {
                        let a381 = {
                            let a379 = {
                                let a353 = 1u64;
                                let a354 = 1u64;
                                nat_add(a353, a354)?
                            };
                            let a380 = {
                                let a377 = {
                                    let a375 = {
                                        let a355 = 1u64;
                                        let a356 = 1u64;
                                        nat_add(a355, a356)?
                                    };
                                    let a376 = {
                                        let a373 = {
                                            let a371 = {
                                                let a357 = 1u64;
                                                let a358 = 1u64;
                                                nat_add(a357, a358)?
                                            };
                                            let a372 = {
                                                let a369 = {
                                                    let a367 = {
                                                        let a359 = 1u64;
                                                        let a360 = 1u64;
                                                        nat_add(a359, a360)?
                                                    };
                                                    let a368 = {
                                                        let a365 = {
                                                            let a363 = {
                                                                let a361 = 1u64;
                                                                let a362 = 1u64;
                                                                nat_add(a361, a362)?
                                                            };
                                                            let a364 = 1u64;
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
                                        };
                                        let a374 = 1u64;
                                        nat_add(a373, a374)?
                                    };
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
    if nat_eq(a391, a392) {
        let a393 = f13(v0)?;
        let a394 = 0u64;
        Ok(nat_eq(a393, a394))
    } else {
        Ok(false)
    }
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f16(v0)?;
    let v3: u64 = {
        let a407 = f17(v0)?;
        let a408 = {
            let a405 = {
                let a395 = 1u64;
                let a396 = 1u64;
                nat_add(a395, a396)?
            };
            let a406 = {
                let a403 = {
                    let a397 = 1u64;
                    let a398 = 1u64;
                    nat_add(a397, a398)?
                };
                let a404 = {
                    let a401 = {
                        let a399 = 1u64;
                        let a400 = 1u64;
                        nat_add(a399, a400)?
                    };
                    let a402 = 1u64;
                    nat_mul(a401, a402)?
                };
                nat_mul(a403, a404)?
            };
            nat_mul(a405, a406)?
        };
        nat_mul(a407, a408)?
    };
    let v4: u64 = f18({
        let a421 = f17(v1)?;
        let a422 = {
            let a419 = {
                let a409 = 1u64;
                let a410 = 1u64;
                nat_add(a409, a410)?
            };
            let a420 = {
                let a417 = {
                    let a411 = 1u64;
                    let a412 = 1u64;
                    nat_add(a411, a412)?
                };
                let a418 = {
                    let a415 = {
                        let a413 = 1u64;
                        let a414 = 1u64;
                        nat_add(a413, a414)?
                    };
                    let a416 = 1u64;
                    nat_mul(a415, a416)?
                };
                nat_mul(a417, a418)?
            };
            nat_mul(a419, a420)?
        };
        nat_mul(a421, a422)?
    }, {
        let a423 = v2;
        let a424 = f16(v1)?;
        nat_sub(a423, a424)
    })?;
    let a425 = f7(v0)?;
    let a426 = f7(v1)?;
    if nat_eq(a425, a426) {
        f19(f7(v0)?, v2, {
            let a427 = v3;
            let a428 = v4;
            nat_add(a427, a428)?
        })
    } else {
        let a429 = v3;
        let a430 = v4;
        if nat_eq(a429, a430) {
            Ok(0u64)
        } else {
            f19(f7(v0)?, v2, {
                let a431 = v3;
                let a432 = v4;
                nat_sub(a431, a432)
            })
        }
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a455 = f8(v0)?;
    let a456 = f9({
        let a453 = {
            let a451 = {
                let a433 = 1u64;
                let a434 = 1u64;
                nat_add(a433, a434)?
            };
            let a452 = {
                let a449 = {
                    let a447 = {
                        let a435 = 1u64;
                        let a436 = 1u64;
                        nat_add(a435, a436)?
                    };
                    let a448 = {
                        let a445 = {
                            let a443 = {
                                let a437 = 1u64;
                                let a438 = 1u64;
                                nat_add(a437, a438)?
                            };
                            let a444 = {
                                let a441 = {
                                    let a439 = 1u64;
                                    let a440 = 1u64;
                                    nat_add(a439, a440)?
                                };
                                let a442 = 1u64;
                                nat_mul(a441, a442)?
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
    })?;
    let a457 = 0u64;
    Ok(nat_quot(a455, a456, a457))
}

pub fn f13(v0: u64) -> R<u64> {
    let a480 = v0;
    let a481 = f9({
        let a478 = {
            let a476 = {
                let a458 = 1u64;
                let a459 = 1u64;
                nat_add(a458, a459)?
            };
            let a477 = {
                let a474 = {
                    let a472 = {
                        let a460 = 1u64;
                        let a461 = 1u64;
                        nat_add(a460, a461)?
                    };
                    let a473 = {
                        let a470 = {
                            let a468 = {
                                let a462 = 1u64;
                                let a463 = 1u64;
                                nat_add(a462, a463)?
                            };
                            let a469 = {
                                let a466 = {
                                    let a464 = 1u64;
                                    let a465 = 1u64;
                                    nat_add(a464, a465)?
                                };
                                let a467 = 1u64;
                                nat_mul(a466, a467)?
                            };
                            nat_mul(a468, a469)?
                        };
                        let a471 = 1u64;
                        nat_add(a470, a471)?
                    };
                    nat_mul(a472, a473)?
                };
                let a475 = 1u64;
                nat_add(a474, a475)?
            };
            nat_mul(a476, a477)?
        };
        let a479 = 1u64;
        nat_add(a478, a479)?
    })?;
    let a482 = 0u64;
    Ok(nat_rem(a480, a481, a482))
}

pub fn f14(v0: Fn0, p486: u64, p487: u64) -> R<u64> {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v1, v2) = m485;
                let m483 = v1;
                if m483 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m483 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c484: Fn0 = v0.clone();
                        c484.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let a488 = v0;
    let a489 = v0;
    nat_add(a488, a489)
}

pub fn f16(v0: u64) -> R<u64> {
    let a528 = {
        let a490 = f12(v0)?;
        let a491 = 0u64;
        if nat_eq(a490, a491) {
            1u64
        } else {
            f12(v0)?
        }
    };
    let a529 = {
        let a526 = {
            let a492 = 1u64;
            let a493 = 1u64;
            nat_add(a492, a493)?
        };
        let a527 = {
            let a524 = {
                let a494 = 1u64;
                let a495 = 1u64;
                nat_add(a494, a495)?
            };
            let a525 = {
                let a522 = {
                    let a496 = 1u64;
                    let a497 = 1u64;
                    nat_add(a496, a497)?
                };
                let a523 = {
                    let a520 = {
                        let a498 = 1u64;
                        let a499 = 1u64;
                        nat_add(a498, a499)?
                    };
                    let a521 = {
                        let a518 = {
                            let a500 = 1u64;
                            let a501 = 1u64;
                            nat_add(a500, a501)?
                        };
                        let a519 = {
                            let a516 = {
                                let a502 = 1u64;
                                let a503 = 1u64;
                                nat_add(a502, a503)?
                            };
                            let a517 = {
                                let a514 = {
                                    let a504 = 1u64;
                                    let a505 = 1u64;
                                    nat_add(a504, a505)?
                                };
                                let a515 = {
                                    let a512 = {
                                        let a506 = 1u64;
                                        let a507 = 1u64;
                                        nat_add(a506, a507)?
                                    };
                                    let a513 = {
                                        let a510 = {
                                            let a508 = 1u64;
                                            let a509 = 1u64;
                                            nat_add(a508, a509)?
                                        };
                                        let a511 = 1u64;
                                        nat_mul(a510, a511)?
                                    };
                                    nat_mul(a512, a513)?
                                };
                                nat_mul(a514, a515)?
                            };
                            nat_mul(a516, a517)?
                        };
                        nat_mul(a518, a519)?
                    };
                    nat_mul(a520, a521)?
                };
                nat_mul(a522, a523)?
            };
            nat_mul(a524, a525)?
        };
        nat_mul(a526, a527)?
    };
    nat_add(a528, a529)
}

pub fn f17(v0: u64) -> R<u64> {
    let a554 = f13(v0)?;
    let a555 = {
        let a530 = f12(v0)?;
        let a531 = 0u64;
        if nat_eq(a530, a531) {
            0u64
        } else {
            f9({
                let a552 = {
                    let a550 = {
                        let a532 = 1u64;
                        let a533 = 1u64;
                        nat_add(a532, a533)?
                    };
                    let a551 = {
                        let a548 = {
                            let a546 = {
                                let a534 = 1u64;
                                let a535 = 1u64;
                                nat_add(a534, a535)?
                            };
                            let a547 = {
                                let a544 = {
                                    let a542 = {
                                        let a536 = 1u64;
                                        let a537 = 1u64;
                                        nat_add(a536, a537)?
                                    };
                                    let a543 = {
                                        let a540 = {
                                            let a538 = 1u64;
                                            let a539 = 1u64;
                                            nat_add(a538, a539)?
                                        };
                                        let a541 = 1u64;
                                        nat_mul(a540, a541)?
                                    };
                                    nat_mul(a542, a543)?
                                };
                                let a545 = 1u64;
                                nat_add(a544, a545)?
                            };
                            nat_mul(a546, a547)?
                        };
                        let a549 = 1u64;
                        nat_add(a548, a549)?
                    };
                    nat_mul(a550, a551)?
                };
                let a553 = 1u64;
                nat_add(a552, a553)?
            })?
        }
    };
    nat_add(a554, a555)
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let a556 = v1;
    let a557 = 0u64;
    if nat_eq(a556, a557) {
        Ok(v0)
    } else {
        let a582 = {
            let a580 = {
                let a558 = 1u64;
                let a559 = 1u64;
                nat_add(a558, a559)?
            };
            let a581 = {
                let a578 = {
                    let a560 = 1u64;
                    let a561 = 1u64;
                    nat_add(a560, a561)?
                };
                let a579 = {
                    let a576 = {
                        let a562 = 1u64;
                        let a563 = 1u64;
                        nat_add(a562, a563)?
                    };
                    let a577 = {
                        let a574 = {
                            let a564 = 1u64;
                            let a565 = 1u64;
                            nat_add(a564, a565)?
                        };
                        let a575 = {
                            let a572 = {
                                let a566 = 1u64;
                                let a567 = 1u64;
                                nat_add(a566, a567)?
                            };
                            let a573 = {
                                let a570 = {
                                    let a568 = 1u64;
                                    let a569 = 1u64;
                                    nat_add(a568, a569)?
                                };
                                let a571 = 1u64;
                                nat_mul(a570, a571)?
                            };
                            nat_mul(a572, a573)?
                        };
                        nat_mul(a574, a575)?
                    };
                    nat_mul(a576, a577)?
                };
                nat_mul(a578, a579)?
            };
            nat_mul(a580, a581)?
        };
        let a583 = v1;
        if nat_le(a582, a583) {
            let a584 = v0;
            let a585 = 0u64;
            if nat_eq(a584, a585) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f9(v1)?;
            f20({
                let a586 = v0;
                let a587 = v2;
                let a588 = 0u64;
                nat_quot(a586, a587, a588)
            }, {
                let a592 = 0u64;
                let a593 = {
                    let a589 = v0;
                    let a590 = v2;
                    let a591 = 0u64;
                    nat_rem(a589, a590, a591)
                };
                nat_lt(a592, a593)
            })
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a616 = f9({
        let a614 = {
            let a612 = {
                let a594 = 1u64;
                let a595 = 1u64;
                nat_add(a594, a595)?
            };
            let a613 = {
                let a610 = {
                    let a608 = {
                        let a596 = 1u64;
                        let a597 = 1u64;
                        nat_add(a596, a597)?
                    };
                    let a609 = {
                        let a606 = {
                            let a598 = 1u64;
                            let a599 = 1u64;
                            nat_add(a598, a599)?
                        };
                        let a607 = {
                            let a604 = {
                                let a602 = {
                                    let a600 = 1u64;
                                    let a601 = 1u64;
                                    nat_add(a600, a601)?
                                };
                                let a603 = 1u64;
                                nat_mul(a602, a603)?
                            };
                            let a605 = 1u64;
                            nat_add(a604, a605)?
                        };
                        nat_mul(a606, a607)?
                    };
                    nat_mul(a608, a609)?
                };
                let a611 = 1u64;
                nat_add(a610, a611)?
            };
            nat_mul(a612, a613)?
        };
        let a615 = 1u64;
        nat_add(a614, a615)?
    })?;
    let a617 = v2;
    if nat_le(a616, a617) {
        f21(v0, {
            let a618 = v1;
            let a619 = 1u64;
            nat_add(a618, a619)?
        }, f18(v2, 1u64)?)
    } else {
        let v3: Adt1 = f22(v1, v2, {
            let a656 = {
                let a654 = {
                    let a620 = 1u64;
                    let a621 = 1u64;
                    nat_add(a620, a621)?
                };
                let a655 = {
                    let a652 = {
                        let a622 = 1u64;
                        let a623 = 1u64;
                        nat_add(a622, a623)?
                    };
                    let a653 = {
                        let a650 = {
                            let a624 = 1u64;
                            let a625 = 1u64;
                            nat_add(a624, a625)?
                        };
                        let a651 = {
                            let a648 = {
                                let a626 = 1u64;
                                let a627 = 1u64;
                                nat_add(a626, a627)?
                            };
                            let a649 = {
                                let a646 = {
                                    let a628 = 1u64;
                                    let a629 = 1u64;
                                    nat_add(a628, a629)?
                                };
                                let a647 = {
                                    let a644 = {
                                        let a630 = 1u64;
                                        let a631 = 1u64;
                                        nat_add(a630, a631)?
                                    };
                                    let a645 = {
                                        let a642 = {
                                            let a632 = 1u64;
                                            let a633 = 1u64;
                                            nat_add(a632, a633)?
                                        };
                                        let a643 = {
                                            let a640 = {
                                                let a634 = 1u64;
                                                let a635 = 1u64;
                                                nat_add(a634, a635)?
                                            };
                                            let a641 = {
                                                let a638 = {
                                                    let a636 = 1u64;
                                                    let a637 = 1u64;
                                                    nat_add(a636, a637)?
                                                };
                                                let a639 = 1u64;
                                                nat_mul(a638, a639)?
                                            };
                                            nat_mul(a640, a641)?
                                        };
                                        nat_mul(a642, a643)?
                                    };
                                    nat_mul(a644, a645)?
                                };
                                nat_mul(a646, a647)?
                            };
                            nat_mul(a648, a649)?
                        };
                        nat_mul(a650, a651)?
                    };
                    nat_mul(a652, a653)?
                };
                nat_mul(a654, a655)?
            };
            let a657 = 1u64;
            nat_add(a656, a657)?
        }, f9({
            let a676 = {
                let a658 = 1u64;
                let a659 = 1u64;
                nat_add(a658, a659)?
            };
            let a677 = {
                let a674 = {
                    let a672 = {
                        let a660 = 1u64;
                        let a661 = 1u64;
                        nat_add(a660, a661)?
                    };
                    let a673 = {
                        let a670 = {
                            let a662 = 1u64;
                            let a663 = 1u64;
                            nat_add(a662, a663)?
                        };
                        let a671 = {
                            let a668 = {
                                let a666 = {
                                    let a664 = 1u64;
                                    let a665 = 1u64;
                                    nat_add(a664, a665)?
                                };
                                let a667 = 1u64;
                                nat_mul(a666, a667)?
                            };
                            let a669 = 1u64;
                            nat_add(a668, a669)?
                        };
                        nat_mul(a670, a671)?
                    };
                    nat_mul(a672, a673)?
                };
                let a675 = 1u64;
                nat_add(a674, a675)?
            };
            nat_mul(a676, a677)?
        })?)?;
        f21(v0, match v3.clone() {
            Adt1::C0(h678, _, _, _) => {
                h678
            }
        }, match v3 {
            Adt1::C0(_, h679, _, _) => {
                h679
            }
        })
    }
}

pub fn f20(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a687 = {
            let a684 = v0;
            let a685 = {
                let a682 = {
                    let a680 = 1u64;
                    let a681 = 1u64;
                    nat_add(a680, a681)?
                };
                let a683 = 1u64;
                nat_mul(a682, a683)?
            };
            let a686 = 0u64;
            nat_rem(a684, a685, a686)
        };
        let a688 = 0u64;
        nat_eq(a687, a688)
    } else {
        false
    } {
        let a689 = v0;
        let a690 = 1u64;
        nat_add(a689, a690)
    } else {
        Ok(v0)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a729 = v1;
    let a730 = {
        let a727 = {
            let a725 = {
                let a691 = 1u64;
                let a692 = 1u64;
                nat_add(a691, a692)?
            };
            let a726 = {
                let a723 = {
                    let a693 = 1u64;
                    let a694 = 1u64;
                    nat_add(a693, a694)?
                };
                let a724 = {
                    let a721 = {
                        let a695 = 1u64;
                        let a696 = 1u64;
                        nat_add(a695, a696)?
                    };
                    let a722 = {
                        let a719 = {
                            let a697 = 1u64;
                            let a698 = 1u64;
                            nat_add(a697, a698)?
                        };
                        let a720 = {
                            let a717 = {
                                let a699 = 1u64;
                                let a700 = 1u64;
                                nat_add(a699, a700)?
                            };
                            let a718 = {
                                let a715 = {
                                    let a701 = 1u64;
                                    let a702 = 1u64;
                                    nat_add(a701, a702)?
                                };
                                let a716 = {
                                    let a713 = {
                                        let a703 = 1u64;
                                        let a704 = 1u64;
                                        nat_add(a703, a704)?
                                    };
                                    let a714 = {
                                        let a711 = {
                                            let a705 = 1u64;
                                            let a706 = 1u64;
                                            nat_add(a705, a706)?
                                        };
                                        let a712 = {
                                            let a709 = {
                                                let a707 = 1u64;
                                                let a708 = 1u64;
                                                nat_add(a707, a708)?
                                            };
                                            let a710 = 1u64;
                                            nat_mul(a709, a710)?
                                        };
                                        nat_mul(a711, a712)?
                                    };
                                    nat_mul(a713, a714)?
                                };
                                nat_mul(a715, a716)?
                            };
                            nat_mul(a717, a718)?
                        };
                        nat_mul(a719, a720)?
                    };
                    nat_mul(a721, a722)?
                };
                nat_mul(a723, a724)?
            };
            nat_mul(a725, a726)?
        };
        let a728 = 1u64;
        nat_add(a727, a728)?
    };
    if nat_lt(a729, a730) {
        f23(v0, {
            let a767 = {
                let a765 = {
                    let a731 = 1u64;
                    let a732 = 1u64;
                    nat_add(a731, a732)?
                };
                let a766 = {
                    let a763 = {
                        let a733 = 1u64;
                        let a734 = 1u64;
                        nat_add(a733, a734)?
                    };
                    let a764 = {
                        let a761 = {
                            let a735 = 1u64;
                            let a736 = 1u64;
                            nat_add(a735, a736)?
                        };
                        let a762 = {
                            let a759 = {
                                let a737 = 1u64;
                                let a738 = 1u64;
                                nat_add(a737, a738)?
                            };
                            let a760 = {
                                let a757 = {
                                    let a739 = 1u64;
                                    let a740 = 1u64;
                                    nat_add(a739, a740)?
                                };
                                let a758 = {
                                    let a755 = {
                                        let a741 = 1u64;
                                        let a742 = 1u64;
                                        nat_add(a741, a742)?
                                    };
                                    let a756 = {
                                        let a753 = {
                                            let a743 = 1u64;
                                            let a744 = 1u64;
                                            nat_add(a743, a744)?
                                        };
                                        let a754 = {
                                            let a751 = {
                                                let a745 = 1u64;
                                                let a746 = 1u64;
                                                nat_add(a745, a746)?
                                            };
                                            let a752 = {
                                                let a749 = {
                                                    let a747 = 1u64;
                                                    let a748 = 1u64;
                                                    nat_add(a747, a748)?
                                                };
                                                let a750 = 1u64;
                                                nat_mul(a749, a750)?
                                            };
                                            nat_mul(a751, a752)?
                                        };
                                        nat_mul(a753, a754)?
                                    };
                                    nat_mul(a755, a756)?
                                };
                                nat_mul(a757, a758)?
                            };
                            nat_mul(a759, a760)?
                        };
                        nat_mul(a761, a762)?
                    };
                    nat_mul(a763, a764)?
                };
                nat_mul(a765, a766)?
            };
            let a768 = 1u64;
            nat_add(a767, a768)?
        }, f18(v2, {
            let a807 = {
                let a805 = {
                    let a803 = {
                        let a769 = 1u64;
                        let a770 = 1u64;
                        nat_add(a769, a770)?
                    };
                    let a804 = {
                        let a801 = {
                            let a771 = 1u64;
                            let a772 = 1u64;
                            nat_add(a771, a772)?
                        };
                        let a802 = {
                            let a799 = {
                                let a773 = 1u64;
                                let a774 = 1u64;
                                nat_add(a773, a774)?
                            };
                            let a800 = {
                                let a797 = {
                                    let a775 = 1u64;
                                    let a776 = 1u64;
                                    nat_add(a775, a776)?
                                };
                                let a798 = {
                                    let a795 = {
                                        let a777 = 1u64;
                                        let a778 = 1u64;
                                        nat_add(a777, a778)?
                                    };
                                    let a796 = {
                                        let a793 = {
                                            let a779 = 1u64;
                                            let a780 = 1u64;
                                            nat_add(a779, a780)?
                                        };
                                        let a794 = {
                                            let a791 = {
                                                let a781 = 1u64;
                                                let a782 = 1u64;
                                                nat_add(a781, a782)?
                                            };
                                            let a792 = {
                                                let a789 = {
                                                    let a783 = 1u64;
                                                    let a784 = 1u64;
                                                    nat_add(a783, a784)?
                                                };
                                                let a790 = {
                                                    let a787 = {
                                                        let a785 = 1u64;
                                                        let a786 = 1u64;
                                                        nat_add(a785, a786)?
                                                    };
                                                    let a788 = 1u64;
                                                    nat_mul(a787, a788)?
                                                };
                                                nat_mul(a789, a790)?
                                            };
                                            nat_mul(a791, a792)?
                                        };
                                        nat_mul(a793, a794)?
                                    };
                                    nat_mul(a795, a796)?
                                };
                                nat_mul(a797, a798)?
                            };
                            nat_mul(a799, a800)?
                        };
                        nat_mul(a801, a802)?
                    };
                    nat_mul(a803, a804)?
                };
                let a806 = 1u64;
                nat_add(a805, a806)?
            };
            let a808 = v1;
            nat_sub(a807, a808)
        })?)
    } else {
        f23(v0, v1, v2)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h809, _) = f24(Fn1::F25, {
        let a828 = {
            let a810 = 1u64;
            let a811 = 1u64;
            nat_add(a810, a811)?
        };
        let a829 = {
            let a826 = {
                let a812 = 1u64;
                let a813 = 1u64;
                nat_add(a812, a813)?
            };
            let a827 = {
                let a824 = {
                    let a814 = 1u64;
                    let a815 = 1u64;
                    nat_add(a814, a815)?
                };
                let a825 = {
                    let a822 = {
                        let a816 = 1u64;
                        let a817 = 1u64;
                        nat_add(a816, a817)?
                    };
                    let a823 = {
                        let a820 = {
                            let a818 = 1u64;
                            let a819 = 1u64;
                            nat_add(a818, a819)?
                        };
                        let a821 = 1u64;
                        nat_mul(a820, a821)?
                    };
                    nat_mul(a822, a823)?
                };
                nat_mul(a824, a825)?
            };
            nat_mul(a826, a827)?
        };
        nat_mul(a828, a829)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h809)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a842 = v2;
        let a843 = {
            let a840 = {
                let a830 = 1u64;
                let a831 = 1u64;
                nat_add(a830, a831)?
            };
            let a841 = {
                let a838 = {
                    let a832 = 1u64;
                    let a833 = 1u64;
                    nat_add(a832, a833)?
                };
                let a839 = {
                    let a836 = {
                        let a834 = 1u64;
                        let a835 = 1u64;
                        nat_add(a834, a835)?
                    };
                    let a837 = 1u64;
                    nat_mul(a836, a837)?
                };
                nat_mul(a838, a839)?
            };
            nat_mul(a840, a841)?
        };
        let a844 = 0u64;
        nat_rem(a842, a843, a844)
    };
    let v4: u64 = {
        let a857 = v2;
        let a858 = {
            let a855 = {
                let a845 = 1u64;
                let a846 = 1u64;
                nat_add(a845, a846)?
            };
            let a856 = {
                let a853 = {
                    let a847 = 1u64;
                    let a848 = 1u64;
                    nat_add(a847, a848)?
                };
                let a854 = {
                    let a851 = {
                        let a849 = 1u64;
                        let a850 = 1u64;
                        nat_add(a849, a850)?
                    };
                    let a852 = 1u64;
                    nat_mul(a851, a852)?
                };
                nat_mul(a853, a854)?
            };
            nat_mul(a855, a856)?
        };
        let a859 = 0u64;
        nat_quot(a857, a858, a859)
    };
    let v5: bool = {
        let a868 = {
            let a866 = {
                let a860 = 1u64;
                let a861 = 1u64;
                nat_add(a860, a861)?
            };
            let a867 = {
                let a864 = {
                    let a862 = 1u64;
                    let a863 = 1u64;
                    nat_add(a862, a863)?
                };
                let a865 = 1u64;
                nat_mul(a864, a865)?
            };
            nat_mul(a866, a867)?
        };
        let a869 = v3;
        if nat_lt(a868, a869) {
            true
        } else {
            let a878 = v3;
            let a879 = {
                let a876 = {
                    let a870 = 1u64;
                    let a871 = 1u64;
                    nat_add(a870, a871)?
                };
                let a877 = {
                    let a874 = {
                        let a872 = 1u64;
                        let a873 = 1u64;
                        nat_add(a872, a873)?
                    };
                    let a875 = 1u64;
                    nat_mul(a874, a875)?
                };
                nat_mul(a876, a877)?
            };
            if nat_eq(a878, a879) {
                let a887 = {
                    let a884 = v4;
                    let a885 = {
                        let a882 = {
                            let a880 = 1u64;
                            let a881 = 1u64;
                            nat_add(a880, a881)?
                        };
                        let a883 = 1u64;
                        nat_mul(a882, a883)?
                    };
                    let a886 = 0u64;
                    nat_rem(a884, a885, a886)
                };
                let a888 = 1u64;
                nat_eq(a887, a888)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a889 = v4;
        let a890 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a889, a890)?
    };
    let v7: bool = {
        let a919 = {
            let a917 = {
                let a893 = {
                    let a891 = 1u64;
                    let a892 = 1u64;
                    nat_add(a891, a892)?
                };
                let a894 = 1u64;
                nat_mul(a893, a894)?
            };
            let a918 = f9({
                let a915 = {
                    let a913 = {
                        let a895 = 1u64;
                        let a896 = 1u64;
                        nat_add(a895, a896)?
                    };
                    let a914 = {
                        let a911 = {
                            let a909 = {
                                let a897 = 1u64;
                                let a898 = 1u64;
                                nat_add(a897, a898)?
                            };
                            let a910 = {
                                let a907 = {
                                    let a905 = {
                                        let a899 = 1u64;
                                        let a900 = 1u64;
                                        nat_add(a899, a900)?
                                    };
                                    let a906 = {
                                        let a903 = {
                                            let a901 = 1u64;
                                            let a902 = 1u64;
                                            nat_add(a901, a902)?
                                        };
                                        let a904 = 1u64;
                                        nat_mul(a903, a904)?
                                    };
                                    nat_mul(a905, a906)?
                                };
                                let a908 = 1u64;
                                nat_add(a907, a908)?
                            };
                            nat_mul(a909, a910)?
                        };
                        let a912 = 1u64;
                        nat_add(a911, a912)?
                    };
                    nat_mul(a913, a914)?
                };
                let a916 = 1u64;
                nat_add(a915, a916)?
            })?;
            nat_mul(a917, a918)?
        };
        let a920 = v6;
        nat_le(a919, a920)
    };
    let v8: u64 = if v7 {
        let a925 = v6;
        let a926 = {
            let a923 = {
                let a921 = 1u64;
                let a922 = 1u64;
                nat_add(a921, a922)?
            };
            let a924 = 1u64;
            nat_mul(a923, a924)?
        };
        let a927 = 0u64;
        nat_quot(a925, a926, a927)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a928 = v1;
        let a929 = 1u64;
        nat_add(a928, a929)?
    } else {
        v1
    };
    let v10: u64 = {
        let a968 = v9;
        let a969 = {
            let a966 = {
                let a964 = {
                    let a930 = 1u64;
                    let a931 = 1u64;
                    nat_add(a930, a931)?
                };
                let a965 = {
                    let a962 = {
                        let a932 = 1u64;
                        let a933 = 1u64;
                        nat_add(a932, a933)?
                    };
                    let a963 = {
                        let a960 = {
                            let a934 = 1u64;
                            let a935 = 1u64;
                            nat_add(a934, a935)?
                        };
                        let a961 = {
                            let a958 = {
                                let a936 = 1u64;
                                let a937 = 1u64;
                                nat_add(a936, a937)?
                            };
                            let a959 = {
                                let a956 = {
                                    let a938 = 1u64;
                                    let a939 = 1u64;
                                    nat_add(a938, a939)?
                                };
                                let a957 = {
                                    let a954 = {
                                        let a940 = 1u64;
                                        let a941 = 1u64;
                                        nat_add(a940, a941)?
                                    };
                                    let a955 = {
                                        let a952 = {
                                            let a942 = 1u64;
                                            let a943 = 1u64;
                                            nat_add(a942, a943)?
                                        };
                                        let a953 = {
                                            let a950 = {
                                                let a944 = 1u64;
                                                let a945 = 1u64;
                                                nat_add(a944, a945)?
                                            };
                                            let a951 = {
                                                let a948 = {
                                                    let a946 = 1u64;
                                                    let a947 = 1u64;
                                                    nat_add(a946, a947)?
                                                };
                                                let a949 = 1u64;
                                                nat_mul(a948, a949)?
                                            };
                                            nat_mul(a950, a951)?
                                        };
                                        nat_mul(a952, a953)?
                                    };
                                    nat_mul(a954, a955)?
                                };
                                nat_mul(a956, a957)?
                            };
                            nat_mul(a958, a959)?
                        };
                        nat_mul(a960, a961)?
                    };
                    nat_mul(a962, a963)?
                };
                nat_mul(a964, a965)?
            };
            let a967 = 1u64;
            nat_add(a966, a967)?
        };
        if if nat_eq(a968, a969) {
            let a992 = v8;
            let a993 = f9({
                let a990 = {
                    let a988 = {
                        let a970 = 1u64;
                        let a971 = 1u64;
                        nat_add(a970, a971)?
                    };
                    let a989 = {
                        let a986 = {
                            let a984 = {
                                let a972 = 1u64;
                                let a973 = 1u64;
                                nat_add(a972, a973)?
                            };
                            let a985 = {
                                let a982 = {
                                    let a980 = {
                                        let a974 = 1u64;
                                        let a975 = 1u64;
                                        nat_add(a974, a975)?
                                    };
                                    let a981 = {
                                        let a978 = {
                                            let a976 = 1u64;
                                            let a977 = 1u64;
                                            nat_add(a976, a977)?
                                        };
                                        let a979 = 1u64;
                                        nat_mul(a978, a979)?
                                    };
                                    nat_mul(a980, a981)?
                                };
                                let a983 = 1u64;
                                nat_add(a982, a983)?
                            };
                            nat_mul(a984, a985)?
                        };
                        let a987 = 1u64;
                        nat_add(a986, a987)?
                    };
                    nat_mul(a988, a989)?
                };
                let a991 = 1u64;
                nat_add(a990, a991)?
            })?;
            nat_lt(a992, a993)
        } else {
            false
        } {
            0u64
        } else {
            let a1030 = v9;
            let a1031 = {
                let a1028 = {
                    let a994 = 1u64;
                    let a995 = 1u64;
                    nat_add(a994, a995)?
                };
                let a1029 = {
                    let a1026 = {
                        let a996 = 1u64;
                        let a997 = 1u64;
                        nat_add(a996, a997)?
                    };
                    let a1027 = {
                        let a1024 = {
                            let a998 = 1u64;
                            let a999 = 1u64;
                            nat_add(a998, a999)?
                        };
                        let a1025 = {
                            let a1022 = {
                                let a1000 = 1u64;
                                let a1001 = 1u64;
                                nat_add(a1000, a1001)?
                            };
                            let a1023 = {
                                let a1020 = {
                                    let a1002 = 1u64;
                                    let a1003 = 1u64;
                                    nat_add(a1002, a1003)?
                                };
                                let a1021 = {
                                    let a1018 = {
                                        let a1004 = 1u64;
                                        let a1005 = 1u64;
                                        nat_add(a1004, a1005)?
                                    };
                                    let a1019 = {
                                        let a1016 = {
                                            let a1006 = 1u64;
                                            let a1007 = 1u64;
                                            nat_add(a1006, a1007)?
                                        };
                                        let a1017 = {
                                            let a1014 = {
                                                let a1008 = 1u64;
                                                let a1009 = 1u64;
                                                nat_add(a1008, a1009)?
                                            };
                                            let a1015 = {
                                                let a1012 = {
                                                    let a1010 = 1u64;
                                                    let a1011 = 1u64;
                                                    nat_add(a1010, a1011)?
                                                };
                                                let a1013 = 1u64;
                                                nat_mul(a1012, a1013)?
                                            };
                                            nat_mul(a1014, a1015)?
                                        };
                                        nat_mul(a1016, a1017)?
                                    };
                                    nat_mul(a1018, a1019)?
                                };
                                nat_mul(a1020, a1021)?
                            };
                            nat_mul(a1022, a1023)?
                        };
                        nat_mul(a1024, a1025)?
                    };
                    nat_mul(a1026, a1027)?
                };
                nat_mul(a1028, a1029)?
            };
            nat_sub(a1030, a1031)
        }
    };
    let a1032 = v8;
    let a1033 = 0u64;
    if nat_eq(a1032, a1033) {
        f26(v0)
    } else {
        let a1086 = {
            let a1084 = {
                let a1082 = {
                    let a1034 = 1u64;
                    let a1035 = 1u64;
                    nat_add(a1034, a1035)?
                };
                let a1083 = {
                    let a1080 = {
                        let a1078 = {
                            let a1036 = 1u64;
                            let a1037 = 1u64;
                            nat_add(a1036, a1037)?
                        };
                        let a1079 = {
                            let a1076 = {
                                let a1074 = {
                                    let a1038 = 1u64;
                                    let a1039 = 1u64;
                                    nat_add(a1038, a1039)?
                                };
                                let a1075 = {
                                    let a1072 = {
                                        let a1070 = {
                                            let a1040 = 1u64;
                                            let a1041 = 1u64;
                                            nat_add(a1040, a1041)?
                                        };
                                        let a1071 = {
                                            let a1068 = {
                                                let a1066 = {
                                                    let a1042 = 1u64;
                                                    let a1043 = 1u64;
                                                    nat_add(a1042, a1043)?
                                                };
                                                let a1067 = {
                                                    let a1064 = {
                                                        let a1062 = {
                                                            let a1044 = 1u64;
                                                            let a1045 = 1u64;
                                                            nat_add(a1044, a1045)?
                                                        };
                                                        let a1063 = {
                                                            let a1060 = {
                                                                let a1058 = {
                                                                    let a1046 = 1u64;
                                                                    let a1047 = 1u64;
                                                                    nat_add(a1046, a1047)?
                                                                };
                                                                let a1059 = {
                                                                    let a1056 = {
                                                                        let a1054 = {
                                                                            let a1048 = 1u64;
                                                                            let a1049 = 1u64;
                                                                            nat_add(a1048, a1049)?
                                                                        };
                                                                        let a1055 = {
                                                                            let a1052 = {
                                                                                let a1050 = 1u64;
                                                                                let a1051 = 1u64;
                                                                                nat_add(a1050, a1051)?
                                                                            };
                                                                            let a1053 = 1u64;
                                                                            nat_mul(a1052, a1053)?
                                                                        };
                                                                        nat_mul(a1054, a1055)?
                                                                    };
                                                                    let a1057 = 1u64;
                                                                    nat_add(a1056, a1057)?
                                                                };
                                                                nat_mul(a1058, a1059)?
                                                            };
                                                            let a1061 = 1u64;
                                                            nat_add(a1060, a1061)?
                                                        };
                                                        nat_mul(a1062, a1063)?
                                                    };
                                                    let a1065 = 1u64;
                                                    nat_add(a1064, a1065)?
                                                };
                                                nat_mul(a1066, a1067)?
                                            };
                                            let a1069 = 1u64;
                                            nat_add(a1068, a1069)?
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
                };
                nat_mul(a1082, a1083)?
            };
            let a1085 = 1u64;
            nat_add(a1084, a1085)?
        };
        let a1087 = v9;
        if nat_le(a1086, a1087) {
            f27(v0)
        } else {
            let a1139 = {
                let a1112 = f26(v0)?;
                let a1113 = {
                    let a1110 = v10;
                    let a1111 = f9({
                        let a1108 = {
                            let a1106 = {
                                let a1088 = 1u64;
                                let a1089 = 1u64;
                                nat_add(a1088, a1089)?
                            };
                            let a1107 = {
                                let a1104 = {
                                    let a1102 = {
                                        let a1090 = 1u64;
                                        let a1091 = 1u64;
                                        nat_add(a1090, a1091)?
                                    };
                                    let a1103 = {
                                        let a1100 = {
                                            let a1098 = {
                                                let a1092 = 1u64;
                                                let a1093 = 1u64;
                                                nat_add(a1092, a1093)?
                                            };
                                            let a1099 = {
                                                let a1096 = {
                                                    let a1094 = 1u64;
                                                    let a1095 = 1u64;
                                                    nat_add(a1094, a1095)?
                                                };
                                                let a1097 = 1u64;
                                                nat_mul(a1096, a1097)?
                                            };
                                            nat_mul(a1098, a1099)?
                                        };
                                        let a1101 = 1u64;
                                        nat_add(a1100, a1101)?
                                    };
                                    nat_mul(a1102, a1103)?
                                };
                                let a1105 = 1u64;
                                nat_add(a1104, a1105)?
                            };
                            nat_mul(a1106, a1107)?
                        };
                        let a1109 = 1u64;
                        nat_add(a1108, a1109)?
                    })?;
                    nat_mul(a1110, a1111)?
                };
                nat_add(a1112, a1113)?
            };
            let a1140 = {
                let a1136 = v8;
                let a1137 = f9({
                    let a1134 = {
                        let a1132 = {
                            let a1114 = 1u64;
                            let a1115 = 1u64;
                            nat_add(a1114, a1115)?
                        };
                        let a1133 = {
                            let a1130 = {
                                let a1128 = {
                                    let a1116 = 1u64;
                                    let a1117 = 1u64;
                                    nat_add(a1116, a1117)?
                                };
                                let a1129 = {
                                    let a1126 = {
                                        let a1124 = {
                                            let a1118 = 1u64;
                                            let a1119 = 1u64;
                                            nat_add(a1118, a1119)?
                                        };
                                        let a1125 = {
                                            let a1122 = {
                                                let a1120 = 1u64;
                                                let a1121 = 1u64;
                                                nat_add(a1120, a1121)?
                                            };
                                            let a1123 = 1u64;
                                            nat_mul(a1122, a1123)?
                                        };
                                        nat_mul(a1124, a1125)?
                                    };
                                    let a1127 = 1u64;
                                    nat_add(a1126, a1127)?
                                };
                                nat_mul(a1128, a1129)?
                            };
                            let a1131 = 1u64;
                            nat_add(a1130, a1131)?
                        };
                        nat_mul(a1132, a1133)?
                    };
                    let a1135 = 1u64;
                    nat_add(a1134, a1135)?
                })?;
                let a1138 = 0u64;
                nat_rem(a1136, a1137, a1138)
            };
            nat_add(a1139, a1140)
        }
    }
}

pub fn f24(v0: Fn1, p1145: u64, p1146: Adt1) -> R<(Adt1, bool)> {
    let m1144 = (p1145, p1146);
    {
        let mut m1144 = m1144;
        loop {
            let step = {
                let (v1, v2) = m1144;
                let m1141 = v1;
                if m1141 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1141 - 1;
                    let m1143 = {
                        let c1142: Fn1 = v0.clone();
                        c1142.apply(v2.clone())?
                    };
                    match m1143 {
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
                Ok(next) => { m1144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt1) -> R<Option<Adt1>> {
    f28(v0)
}

pub fn f26(v0: u64) -> R<u64> {
    let a1171 = v0;
    let a1172 = f9({
        let a1169 = {
            let a1167 = {
                let a1147 = 1u64;
                let a1148 = 1u64;
                nat_add(a1147, a1148)?
            };
            let a1168 = {
                let a1165 = {
                    let a1163 = {
                        let a1149 = 1u64;
                        let a1150 = 1u64;
                        nat_add(a1149, a1150)?
                    };
                    let a1164 = {
                        let a1161 = {
                            let a1159 = {
                                let a1151 = 1u64;
                                let a1152 = 1u64;
                                nat_add(a1151, a1152)?
                            };
                            let a1160 = {
                                let a1157 = {
                                    let a1155 = {
                                        let a1153 = 1u64;
                                        let a1154 = 1u64;
                                        nat_add(a1153, a1154)?
                                    };
                                    let a1156 = 1u64;
                                    nat_mul(a1155, a1156)?
                                };
                                let a1158 = 1u64;
                                nat_add(a1157, a1158)?
                            };
                            nat_mul(a1159, a1160)?
                        };
                        let a1162 = 1u64;
                        nat_add(a1161, a1162)?
                    };
                    nat_mul(a1163, a1164)?
                };
                let a1166 = 1u64;
                nat_add(a1165, a1166)?
            };
            nat_mul(a1167, a1168)?
        };
        let a1170 = 1u64;
        nat_add(a1169, a1170)?
    })?;
    nat_mul(a1171, a1172)
}

pub fn f27(v0: u64) -> R<u64> {
    let a1239 = f26(v0)?;
    let a1240 = {
        let a1237 = {
            let a1213 = {
                let a1211 = {
                    let a1173 = 1u64;
                    let a1174 = 1u64;
                    nat_add(a1173, a1174)?
                };
                let a1212 = {
                    let a1209 = {
                        let a1207 = {
                            let a1175 = 1u64;
                            let a1176 = 1u64;
                            nat_add(a1175, a1176)?
                        };
                        let a1208 = {
                            let a1205 = {
                                let a1203 = {
                                    let a1177 = 1u64;
                                    let a1178 = 1u64;
                                    nat_add(a1177, a1178)?
                                };
                                let a1204 = {
                                    let a1201 = {
                                        let a1199 = {
                                            let a1179 = 1u64;
                                            let a1180 = 1u64;
                                            nat_add(a1179, a1180)?
                                        };
                                        let a1200 = {
                                            let a1197 = {
                                                let a1195 = {
                                                    let a1181 = 1u64;
                                                    let a1182 = 1u64;
                                                    nat_add(a1181, a1182)?
                                                };
                                                let a1196 = {
                                                    let a1193 = {
                                                        let a1191 = {
                                                            let a1183 = 1u64;
                                                            let a1184 = 1u64;
                                                            nat_add(a1183, a1184)?
                                                        };
                                                        let a1192 = {
                                                            let a1189 = {
                                                                let a1187 = {
                                                                    let a1185 = 1u64;
                                                                    let a1186 = 1u64;
                                                                    nat_add(a1185, a1186)?
                                                                };
                                                                let a1188 = 1u64;
                                                                nat_mul(a1187, a1188)?
                                                            };
                                                            let a1190 = 1u64;
                                                            nat_add(a1189, a1190)?
                                                        };
                                                        nat_mul(a1191, a1192)?
                                                    };
                                                    let a1194 = 1u64;
                                                    nat_add(a1193, a1194)?
                                                };
                                                nat_mul(a1195, a1196)?
                                            };
                                            let a1198 = 1u64;
                                            nat_add(a1197, a1198)?
                                        };
                                        nat_mul(a1199, a1200)?
                                    };
                                    let a1202 = 1u64;
                                    nat_add(a1201, a1202)?
                                };
                                nat_mul(a1203, a1204)?
                            };
                            let a1206 = 1u64;
                            nat_add(a1205, a1206)?
                        };
                        nat_mul(a1207, a1208)?
                    };
                    let a1210 = 1u64;
                    nat_add(a1209, a1210)?
                };
                nat_mul(a1211, a1212)?
            };
            let a1214 = 1u64;
            nat_add(a1213, a1214)?
        };
        let a1238 = f9({
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
    nat_add(a1239, a1240)
}

pub fn f28(v0: Adt1) -> R<Option<Adt1>> {
    let a1242 = 0u64;
    let a1243 = match v0.clone() {
        Adt1::C0(_, h1241, _, _) => {
            h1241
        }
    };
    if if if nat_lt(a1242, a1243) {
        let a1246 = match v0.clone() {
            Adt1::C0(_, h1244, _, _) => {
                h1244
            }
        };
        let a1247 = match v0.clone() {
            Adt1::C0(_, _, _, h1245) => {
                h1245
            }
        };
        nat_lt(a1246, a1247)
    } else {
        false
    } {
        let a1250 = match v0.clone() {
            Adt1::C0(_, _, h1248, _) => {
                h1248
            }
        };
        let a1251 = match v0.clone() {
            Adt1::C0(h1249, _, _, _) => {
                h1249
            }
        };
        nat_lt(a1250, a1251)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1253 = match v0.clone() {
                Adt1::C0(h1252, _, _, _) => {
                    h1252
                }
            };
            let a1254 = 1u64;
            nat_sub(a1253, a1254)
        }, {
            let a1257 = match v0.clone() {
                Adt1::C0(_, h1255, _, _) => {
                    h1255
                }
            };
            let a1258 = match v0.clone() {
                Adt1::C0(_, h1256, _, _) => {
                    h1256
                }
            };
            nat_add(a1257, a1258)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1259, _) => {
                h1259
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1260) => {
                h1260
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn run(p0: Adt0) -> R<Option<u64>> {
    f0(p0)
}
