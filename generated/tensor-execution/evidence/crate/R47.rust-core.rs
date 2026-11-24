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
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F14,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F14 => f14(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F24,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F24 => f24(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Adt0> {
    if f1(v0, v1)? {
        Ok(Adt0::C0(true, v1, f2(v0, f3(v1)?)?))
    } else {
        Ok(Adt0::C0(false, v0, f2(v1, f3(v0)?)?))
    }
}

pub fn f1(v0: u64, v1: u64) -> R<bool> {
    if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        Ok(false)
    } else {
        if if f5(v0)? {
            f5(v1)?
        } else {
            false
        } {
            Ok(false)
        } else {
            let a1 = f6(v0)?;
            let a2 = f6(v1)?;
            if nat_eq(a1, a2) {
                let a3 = f6(v0)?;
                let a4 = 0u64;
                if nat_eq(a3, a4) {
                    let a5 = f7(v0)?;
                    let a6 = f7(v1)?;
                    Ok(nat_lt(a5, a6))
                } else {
                    let a7 = f7(v1)?;
                    let a8 = f7(v0)?;
                    Ok(nat_lt(a7, a8))
                }
            } else {
                let a9 = f6(v0)?;
                let a10 = 1u64;
                Ok(nat_eq(a9, a10))
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        let a97 = {
            let a75 = {
                let a51 = {
                    let a49 = {
                        let a11 = 1u64;
                        let a12 = 1u64;
                        nat_add(a11, a12)?
                    };
                    let a50 = {
                        let a47 = {
                            let a45 = {
                                let a13 = 1u64;
                                let a14 = 1u64;
                                nat_add(a13, a14)?
                            };
                            let a46 = {
                                let a43 = {
                                    let a41 = {
                                        let a15 = 1u64;
                                        let a16 = 1u64;
                                        nat_add(a15, a16)?
                                    };
                                    let a42 = {
                                        let a39 = {
                                            let a37 = {
                                                let a17 = 1u64;
                                                let a18 = 1u64;
                                                nat_add(a17, a18)?
                                            };
                                            let a38 = {
                                                let a35 = {
                                                    let a33 = {
                                                        let a19 = 1u64;
                                                        let a20 = 1u64;
                                                        nat_add(a19, a20)?
                                                    };
                                                    let a34 = {
                                                        let a31 = {
                                                            let a29 = {
                                                                let a21 = 1u64;
                                                                let a22 = 1u64;
                                                                nat_add(a21, a22)?
                                                            };
                                                            let a30 = {
                                                                let a27 = {
                                                                    let a25 = {
                                                                        let a23 = 1u64;
                                                                        let a24 = 1u64;
                                                                        nat_add(a23, a24)?
                                                                    };
                                                                    let a26 = 1u64;
                                                                    nat_mul(a25, a26)?
                                                                };
                                                                let a28 = 1u64;
                                                                nat_add(a27, a28)?
                                                            };
                                                            nat_mul(a29, a30)?
                                                        };
                                                        let a32 = 1u64;
                                                        nat_add(a31, a32)?
                                                    };
                                                    nat_mul(a33, a34)?
                                                };
                                                let a36 = 1u64;
                                                nat_add(a35, a36)?
                                            };
                                            nat_mul(a37, a38)?
                                        };
                                        let a40 = 1u64;
                                        nat_add(a39, a40)?
                                    };
                                    nat_mul(a41, a42)?
                                };
                                let a44 = 1u64;
                                nat_add(a43, a44)?
                            };
                            nat_mul(a45, a46)?
                        };
                        let a48 = 1u64;
                        nat_add(a47, a48)?
                    };
                    nat_mul(a49, a50)?
                };
                let a52 = 1u64;
                nat_add(a51, a52)?
            };
            let a76 = f8({
                let a73 = {
                    let a71 = {
                        let a53 = 1u64;
                        let a54 = 1u64;
                        nat_add(a53, a54)?
                    };
                    let a72 = {
                        let a69 = {
                            let a67 = {
                                let a55 = 1u64;
                                let a56 = 1u64;
                                nat_add(a55, a56)?
                            };
                            let a68 = {
                                let a65 = {
                                    let a63 = {
                                        let a57 = 1u64;
                                        let a58 = 1u64;
                                        nat_add(a57, a58)?
                                    };
                                    let a64 = {
                                        let a61 = {
                                            let a59 = 1u64;
                                            let a60 = 1u64;
                                            nat_add(a59, a60)?
                                        };
                                        let a62 = 1u64;
                                        nat_mul(a61, a62)?
                                    };
                                    nat_mul(a63, a64)?
                                };
                                let a66 = 1u64;
                                nat_add(a65, a66)?
                            };
                            nat_mul(a67, a68)?
                        };
                        let a70 = 1u64;
                        nat_add(a69, a70)?
                    };
                    nat_mul(a71, a72)?
                };
                let a74 = 1u64;
                nat_add(a73, a74)?
            })?;
            nat_mul(a75, a76)?
        };
        let a98 = f8({
            let a95 = {
                let a77 = 1u64;
                let a78 = 1u64;
                nat_add(a77, a78)?
            };
            let a96 = {
                let a93 = {
                    let a91 = {
                        let a79 = 1u64;
                        let a80 = 1u64;
                        nat_add(a79, a80)?
                    };
                    let a92 = {
                        let a89 = {
                            let a87 = {
                                let a81 = 1u64;
                                let a82 = 1u64;
                                nat_add(a81, a82)?
                            };
                            let a88 = {
                                let a85 = {
                                    let a83 = 1u64;
                                    let a84 = 1u64;
                                    nat_add(a83, a84)?
                                };
                                let a86 = 1u64;
                                nat_mul(a85, a86)?
                            };
                            nat_mul(a87, a88)?
                        };
                        let a90 = 1u64;
                        nat_add(a89, a90)?
                    };
                    nat_mul(a91, a92)?
                };
                let a94 = 1u64;
                nat_add(a93, a94)?
            };
            nat_mul(a95, a96)?
        })?;
        nat_add(a97, a98)
    } else {
        if if f9(v0)? {
            f9(v1)?
        } else {
            false
        } {
            let a99 = f6(v0)?;
            let a100 = f6(v1)?;
            if nat_eq(a99, a100) {
                Ok(v0)
            } else {
                let a187 = {
                    let a165 = {
                        let a141 = {
                            let a139 = {
                                let a101 = 1u64;
                                let a102 = 1u64;
                                nat_add(a101, a102)?
                            };
                            let a140 = {
                                let a137 = {
                                    let a135 = {
                                        let a103 = 1u64;
                                        let a104 = 1u64;
                                        nat_add(a103, a104)?
                                    };
                                    let a136 = {
                                        let a133 = {
                                            let a131 = {
                                                let a105 = 1u64;
                                                let a106 = 1u64;
                                                nat_add(a105, a106)?
                                            };
                                            let a132 = {
                                                let a129 = {
                                                    let a127 = {
                                                        let a107 = 1u64;
                                                        let a108 = 1u64;
                                                        nat_add(a107, a108)?
                                                    };
                                                    let a128 = {
                                                        let a125 = {
                                                            let a123 = {
                                                                let a109 = 1u64;
                                                                let a110 = 1u64;
                                                                nat_add(a109, a110)?
                                                            };
                                                            let a124 = {
                                                                let a121 = {
                                                                    let a119 = {
                                                                        let a111 = 1u64;
                                                                        let a112 = 1u64;
                                                                        nat_add(a111, a112)?
                                                                    };
                                                                    let a120 = {
                                                                        let a117 = {
                                                                            let a115 = {
                                                                                let a113 = 1u64;
                                                                                let a114 = 1u64;
                                                                                nat_add(a113, a114)?
                                                                            };
                                                                            let a116 = 1u64;
                                                                            nat_mul(a115, a116)?
                                                                        };
                                                                        let a118 = 1u64;
                                                                        nat_add(a117, a118)?
                                                                    };
                                                                    nat_mul(a119, a120)?
                                                                };
                                                                let a122 = 1u64;
                                                                nat_add(a121, a122)?
                                                            };
                                                            nat_mul(a123, a124)?
                                                        };
                                                        let a126 = 1u64;
                                                        nat_add(a125, a126)?
                                                    };
                                                    nat_mul(a127, a128)?
                                                };
                                                let a130 = 1u64;
                                                nat_add(a129, a130)?
                                            };
                                            nat_mul(a131, a132)?
                                        };
                                        let a134 = 1u64;
                                        nat_add(a133, a134)?
                                    };
                                    nat_mul(a135, a136)?
                                };
                                let a138 = 1u64;
                                nat_add(a137, a138)?
                            };
                            nat_mul(a139, a140)?
                        };
                        let a142 = 1u64;
                        nat_add(a141, a142)?
                    };
                    let a166 = f8({
                        let a163 = {
                            let a161 = {
                                let a143 = 1u64;
                                let a144 = 1u64;
                                nat_add(a143, a144)?
                            };
                            let a162 = {
                                let a159 = {
                                    let a157 = {
                                        let a145 = 1u64;
                                        let a146 = 1u64;
                                        nat_add(a145, a146)?
                                    };
                                    let a158 = {
                                        let a155 = {
                                            let a153 = {
                                                let a147 = 1u64;
                                                let a148 = 1u64;
                                                nat_add(a147, a148)?
                                            };
                                            let a154 = {
                                                let a151 = {
                                                    let a149 = 1u64;
                                                    let a150 = 1u64;
                                                    nat_add(a149, a150)?
                                                };
                                                let a152 = 1u64;
                                                nat_mul(a151, a152)?
                                            };
                                            nat_mul(a153, a154)?
                                        };
                                        let a156 = 1u64;
                                        nat_add(a155, a156)?
                                    };
                                    nat_mul(a157, a158)?
                                };
                                let a160 = 1u64;
                                nat_add(a159, a160)?
                            };
                            nat_mul(a161, a162)?
                        };
                        let a164 = 1u64;
                        nat_add(a163, a164)?
                    })?;
                    nat_mul(a165, a166)?
                };
                let a188 = f8({
                    let a185 = {
                        let a167 = 1u64;
                        let a168 = 1u64;
                        nat_add(a167, a168)?
                    };
                    let a186 = {
                        let a183 = {
                            let a181 = {
                                let a169 = 1u64;
                                let a170 = 1u64;
                                nat_add(a169, a170)?
                            };
                            let a182 = {
                                let a179 = {
                                    let a177 = {
                                        let a171 = 1u64;
                                        let a172 = 1u64;
                                        nat_add(a171, a172)?
                                    };
                                    let a178 = {
                                        let a175 = {
                                            let a173 = 1u64;
                                            let a174 = 1u64;
                                            nat_add(a173, a174)?
                                        };
                                        let a176 = 1u64;
                                        nat_mul(a175, a176)?
                                    };
                                    nat_mul(a177, a178)?
                                };
                                let a180 = 1u64;
                                nat_add(a179, a180)?
                            };
                            nat_mul(a181, a182)?
                        };
                        let a184 = 1u64;
                        nat_add(a183, a184)?
                    };
                    nat_mul(a185, a186)?
                })?;
                nat_add(a187, a188)
            }
        } else {
            if f9(v0)? {
                Ok(v0)
            } else {
                if f9(v1)? {
                    Ok(v1)
                } else {
                    let a189 = f7(v1)?;
                    let a190 = f7(v0)?;
                    if nat_le(a189, a190) {
                        f10(v0, v1)
                    } else {
                        f10(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let a237 = {
        let a215 = v0;
        let a216 = f8({
            let a213 = {
                let a211 = {
                    let a191 = 1u64;
                    let a192 = 1u64;
                    nat_add(a191, a192)?
                };
                let a212 = {
                    let a209 = {
                        let a207 = {
                            let a193 = 1u64;
                            let a194 = 1u64;
                            nat_add(a193, a194)?
                        };
                        let a208 = {
                            let a205 = {
                                let a203 = {
                                    let a195 = 1u64;
                                    let a196 = 1u64;
                                    nat_add(a195, a196)?
                                };
                                let a204 = {
                                    let a201 = {
                                        let a199 = {
                                            let a197 = 1u64;
                                            let a198 = 1u64;
                                            nat_add(a197, a198)?
                                        };
                                        let a200 = 1u64;
                                        nat_mul(a199, a200)?
                                    };
                                    let a202 = 1u64;
                                    nat_add(a201, a202)?
                                };
                                nat_mul(a203, a204)?
                            };
                            let a206 = 1u64;
                            nat_add(a205, a206)?
                        };
                        nat_mul(a207, a208)?
                    };
                    let a210 = 1u64;
                    nat_add(a209, a210)?
                };
                nat_mul(a211, a212)?
            };
            let a214 = 1u64;
            nat_add(a213, a214)?
        })?;
        nat_add(a215, a216)?
    };
    let a238 = f8({
        let a235 = {
            let a217 = 1u64;
            let a218 = 1u64;
            nat_add(a217, a218)?
        };
        let a236 = {
            let a233 = {
                let a219 = 1u64;
                let a220 = 1u64;
                nat_add(a219, a220)?
            };
            let a234 = {
                let a231 = {
                    let a221 = 1u64;
                    let a222 = 1u64;
                    nat_add(a221, a222)?
                };
                let a232 = {
                    let a229 = {
                        let a223 = 1u64;
                        let a224 = 1u64;
                        nat_add(a223, a224)?
                    };
                    let a230 = {
                        let a227 = {
                            let a225 = 1u64;
                            let a226 = 1u64;
                            nat_add(a225, a226)?
                        };
                        let a228 = 1u64;
                        nat_mul(a227, a228)?
                    };
                    nat_mul(a229, a230)?
                };
                nat_mul(a231, a232)?
            };
            nat_mul(a233, a234)?
        };
        nat_mul(a235, a236)?
    })?;
    let a239 = 0u64;
    Ok(nat_rem(a237, a238, a239))
}

pub fn f4(v0: u64) -> R<bool> {
    let a282 = f11(v0)?;
    let a283 = {
        let a280 = {
            let a278 = {
                let a240 = 1u64;
                let a241 = 1u64;
                nat_add(a240, a241)?
            };
            let a279 = {
                let a276 = {
                    let a274 = {
                        let a242 = 1u64;
                        let a243 = 1u64;
                        nat_add(a242, a243)?
                    };
                    let a275 = {
                        let a272 = {
                            let a270 = {
                                let a244 = 1u64;
                                let a245 = 1u64;
                                nat_add(a244, a245)?
                            };
                            let a271 = {
                                let a268 = {
                                    let a266 = {
                                        let a246 = 1u64;
                                        let a247 = 1u64;
                                        nat_add(a246, a247)?
                                    };
                                    let a267 = {
                                        let a264 = {
                                            let a262 = {
                                                let a248 = 1u64;
                                                let a249 = 1u64;
                                                nat_add(a248, a249)?
                                            };
                                            let a263 = {
                                                let a260 = {
                                                    let a258 = {
                                                        let a250 = 1u64;
                                                        let a251 = 1u64;
                                                        nat_add(a250, a251)?
                                                    };
                                                    let a259 = {
                                                        let a256 = {
                                                            let a254 = {
                                                                let a252 = 1u64;
                                                                let a253 = 1u64;
                                                                nat_add(a252, a253)?
                                                            };
                                                            let a255 = 1u64;
                                                            nat_mul(a254, a255)?
                                                        };
                                                        let a257 = 1u64;
                                                        nat_add(a256, a257)?
                                                    };
                                                    nat_mul(a258, a259)?
                                                };
                                                let a261 = 1u64;
                                                nat_add(a260, a261)?
                                            };
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
    if nat_eq(a282, a283) {
        let a284 = 0u64;
        let a285 = f12(v0)?;
        Ok(nat_lt(a284, a285))
    } else {
        Ok(false)
    }
}

pub fn f5(v0: u64) -> R<bool> {
    let a286 = f7(v0)?;
    let a287 = 0u64;
    Ok(nat_eq(a286, a287))
}

pub fn f6(v0: u64) -> R<u64> {
    let a312 = v0;
    let a313 = f8({
        let a310 = {
            let a308 = {
                let a288 = 1u64;
                let a289 = 1u64;
                nat_add(a288, a289)?
            };
            let a309 = {
                let a306 = {
                    let a304 = {
                        let a290 = 1u64;
                        let a291 = 1u64;
                        nat_add(a290, a291)?
                    };
                    let a305 = {
                        let a302 = {
                            let a300 = {
                                let a292 = 1u64;
                                let a293 = 1u64;
                                nat_add(a292, a293)?
                            };
                            let a301 = {
                                let a298 = {
                                    let a296 = {
                                        let a294 = 1u64;
                                        let a295 = 1u64;
                                        nat_add(a294, a295)?
                                    };
                                    let a297 = 1u64;
                                    nat_mul(a296, a297)?
                                };
                                let a299 = 1u64;
                                nat_add(a298, a299)?
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
    let a314 = 0u64;
    Ok(nat_quot(a312, a313, a314))
}

pub fn f7(v0: u64) -> R<u64> {
    let a339 = v0;
    let a340 = f8({
        let a337 = {
            let a335 = {
                let a315 = 1u64;
                let a316 = 1u64;
                nat_add(a315, a316)?
            };
            let a336 = {
                let a333 = {
                    let a331 = {
                        let a317 = 1u64;
                        let a318 = 1u64;
                        nat_add(a317, a318)?
                    };
                    let a332 = {
                        let a329 = {
                            let a327 = {
                                let a319 = 1u64;
                                let a320 = 1u64;
                                nat_add(a319, a320)?
                            };
                            let a328 = {
                                let a325 = {
                                    let a323 = {
                                        let a321 = 1u64;
                                        let a322 = 1u64;
                                        nat_add(a321, a322)?
                                    };
                                    let a324 = 1u64;
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
            };
            nat_mul(a335, a336)?
        };
        let a338 = 1u64;
        nat_add(a337, a338)?
    })?;
    let a341 = 0u64;
    Ok(nat_rem(a339, a340, a341))
}

pub fn f8(v0: u64) -> R<u64> {
    f13(Fn0::F14, v0, 1u64)
}

pub fn f9(v0: u64) -> R<bool> {
    let a384 = f11(v0)?;
    let a385 = {
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
    if nat_eq(a384, a385) {
        let a386 = f12(v0)?;
        let a387 = 0u64;
        Ok(nat_eq(a386, a387))
    } else {
        Ok(false)
    }
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f15(v0)?;
    let v3: u64 = {
        let a400 = f16(v0)?;
        let a401 = {
            let a398 = {
                let a388 = 1u64;
                let a389 = 1u64;
                nat_add(a388, a389)?
            };
            let a399 = {
                let a396 = {
                    let a390 = 1u64;
                    let a391 = 1u64;
                    nat_add(a390, a391)?
                };
                let a397 = {
                    let a394 = {
                        let a392 = 1u64;
                        let a393 = 1u64;
                        nat_add(a392, a393)?
                    };
                    let a395 = 1u64;
                    nat_mul(a394, a395)?
                };
                nat_mul(a396, a397)?
            };
            nat_mul(a398, a399)?
        };
        nat_mul(a400, a401)?
    };
    let v4: u64 = f17({
        let a414 = f16(v1)?;
        let a415 = {
            let a412 = {
                let a402 = 1u64;
                let a403 = 1u64;
                nat_add(a402, a403)?
            };
            let a413 = {
                let a410 = {
                    let a404 = 1u64;
                    let a405 = 1u64;
                    nat_add(a404, a405)?
                };
                let a411 = {
                    let a408 = {
                        let a406 = 1u64;
                        let a407 = 1u64;
                        nat_add(a406, a407)?
                    };
                    let a409 = 1u64;
                    nat_mul(a408, a409)?
                };
                nat_mul(a410, a411)?
            };
            nat_mul(a412, a413)?
        };
        nat_mul(a414, a415)?
    }, {
        let a416 = v2;
        let a417 = f15(v1)?;
        nat_sub(a416, a417)
    })?;
    let a418 = f6(v0)?;
    let a419 = f6(v1)?;
    if nat_eq(a418, a419) {
        f18(f6(v0)?, v2, {
            let a420 = v3;
            let a421 = v4;
            nat_add(a420, a421)?
        })
    } else {
        let a422 = v3;
        let a423 = v4;
        if nat_eq(a422, a423) {
            Ok(0u64)
        } else {
            f18(f6(v0)?, v2, {
                let a424 = v3;
                let a425 = v4;
                nat_sub(a424, a425)
            })
        }
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a448 = f7(v0)?;
    let a449 = f8({
        let a446 = {
            let a444 = {
                let a426 = 1u64;
                let a427 = 1u64;
                nat_add(a426, a427)?
            };
            let a445 = {
                let a442 = {
                    let a440 = {
                        let a428 = 1u64;
                        let a429 = 1u64;
                        nat_add(a428, a429)?
                    };
                    let a441 = {
                        let a438 = {
                            let a436 = {
                                let a430 = 1u64;
                                let a431 = 1u64;
                                nat_add(a430, a431)?
                            };
                            let a437 = {
                                let a434 = {
                                    let a432 = 1u64;
                                    let a433 = 1u64;
                                    nat_add(a432, a433)?
                                };
                                let a435 = 1u64;
                                nat_mul(a434, a435)?
                            };
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
    })?;
    let a450 = 0u64;
    Ok(nat_quot(a448, a449, a450))
}

pub fn f12(v0: u64) -> R<u64> {
    let a473 = v0;
    let a474 = f8({
        let a471 = {
            let a469 = {
                let a451 = 1u64;
                let a452 = 1u64;
                nat_add(a451, a452)?
            };
            let a470 = {
                let a467 = {
                    let a465 = {
                        let a453 = 1u64;
                        let a454 = 1u64;
                        nat_add(a453, a454)?
                    };
                    let a466 = {
                        let a463 = {
                            let a461 = {
                                let a455 = 1u64;
                                let a456 = 1u64;
                                nat_add(a455, a456)?
                            };
                            let a462 = {
                                let a459 = {
                                    let a457 = 1u64;
                                    let a458 = 1u64;
                                    nat_add(a457, a458)?
                                };
                                let a460 = 1u64;
                                nat_mul(a459, a460)?
                            };
                            nat_mul(a461, a462)?
                        };
                        let a464 = 1u64;
                        nat_add(a463, a464)?
                    };
                    nat_mul(a465, a466)?
                };
                let a468 = 1u64;
                nat_add(a467, a468)?
            };
            nat_mul(a469, a470)?
        };
        let a472 = 1u64;
        nat_add(a471, a472)?
    })?;
    let a475 = 0u64;
    Ok(nat_rem(a473, a474, a475))
}

pub fn f13(v0: Fn0, p479: u64, p480: u64) -> R<u64> {
    let m478 = (p479, p480);
    {
        let mut m478 = m478;
        loop {
            let step = {
                let (v1, v2) = m478;
                let m476 = v1;
                if m476 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m476 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c477: Fn0 = v0.clone();
                        c477.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a481 = v0;
    let a482 = v0;
    nat_add(a481, a482)
}

pub fn f15(v0: u64) -> R<u64> {
    let a521 = {
        let a483 = f11(v0)?;
        let a484 = 0u64;
        if nat_eq(a483, a484) {
            1u64
        } else {
            f11(v0)?
        }
    };
    let a522 = {
        let a519 = {
            let a485 = 1u64;
            let a486 = 1u64;
            nat_add(a485, a486)?
        };
        let a520 = {
            let a517 = {
                let a487 = 1u64;
                let a488 = 1u64;
                nat_add(a487, a488)?
            };
            let a518 = {
                let a515 = {
                    let a489 = 1u64;
                    let a490 = 1u64;
                    nat_add(a489, a490)?
                };
                let a516 = {
                    let a513 = {
                        let a491 = 1u64;
                        let a492 = 1u64;
                        nat_add(a491, a492)?
                    };
                    let a514 = {
                        let a511 = {
                            let a493 = 1u64;
                            let a494 = 1u64;
                            nat_add(a493, a494)?
                        };
                        let a512 = {
                            let a509 = {
                                let a495 = 1u64;
                                let a496 = 1u64;
                                nat_add(a495, a496)?
                            };
                            let a510 = {
                                let a507 = {
                                    let a497 = 1u64;
                                    let a498 = 1u64;
                                    nat_add(a497, a498)?
                                };
                                let a508 = {
                                    let a505 = {
                                        let a499 = 1u64;
                                        let a500 = 1u64;
                                        nat_add(a499, a500)?
                                    };
                                    let a506 = {
                                        let a503 = {
                                            let a501 = 1u64;
                                            let a502 = 1u64;
                                            nat_add(a501, a502)?
                                        };
                                        let a504 = 1u64;
                                        nat_mul(a503, a504)?
                                    };
                                    nat_mul(a505, a506)?
                                };
                                nat_mul(a507, a508)?
                            };
                            nat_mul(a509, a510)?
                        };
                        nat_mul(a511, a512)?
                    };
                    nat_mul(a513, a514)?
                };
                nat_mul(a515, a516)?
            };
            nat_mul(a517, a518)?
        };
        nat_mul(a519, a520)?
    };
    nat_add(a521, a522)
}

pub fn f16(v0: u64) -> R<u64> {
    let a547 = f12(v0)?;
    let a548 = {
        let a523 = f11(v0)?;
        let a524 = 0u64;
        if nat_eq(a523, a524) {
            0u64
        } else {
            f8({
                let a545 = {
                    let a543 = {
                        let a525 = 1u64;
                        let a526 = 1u64;
                        nat_add(a525, a526)?
                    };
                    let a544 = {
                        let a541 = {
                            let a539 = {
                                let a527 = 1u64;
                                let a528 = 1u64;
                                nat_add(a527, a528)?
                            };
                            let a540 = {
                                let a537 = {
                                    let a535 = {
                                        let a529 = 1u64;
                                        let a530 = 1u64;
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
                    nat_mul(a543, a544)?
                };
                let a546 = 1u64;
                nat_add(a545, a546)?
            })?
        }
    };
    nat_add(a547, a548)
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let a549 = v1;
    let a550 = 0u64;
    if nat_eq(a549, a550) {
        Ok(v0)
    } else {
        let a575 = {
            let a573 = {
                let a551 = 1u64;
                let a552 = 1u64;
                nat_add(a551, a552)?
            };
            let a574 = {
                let a571 = {
                    let a553 = 1u64;
                    let a554 = 1u64;
                    nat_add(a553, a554)?
                };
                let a572 = {
                    let a569 = {
                        let a555 = 1u64;
                        let a556 = 1u64;
                        nat_add(a555, a556)?
                    };
                    let a570 = {
                        let a567 = {
                            let a557 = 1u64;
                            let a558 = 1u64;
                            nat_add(a557, a558)?
                        };
                        let a568 = {
                            let a565 = {
                                let a559 = 1u64;
                                let a560 = 1u64;
                                nat_add(a559, a560)?
                            };
                            let a566 = {
                                let a563 = {
                                    let a561 = 1u64;
                                    let a562 = 1u64;
                                    nat_add(a561, a562)?
                                };
                                let a564 = 1u64;
                                nat_mul(a563, a564)?
                            };
                            nat_mul(a565, a566)?
                        };
                        nat_mul(a567, a568)?
                    };
                    nat_mul(a569, a570)?
                };
                nat_mul(a571, a572)?
            };
            nat_mul(a573, a574)?
        };
        let a576 = v1;
        if nat_le(a575, a576) {
            let a577 = v0;
            let a578 = 0u64;
            if nat_eq(a577, a578) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f8(v1)?;
            f19({
                let a579 = v0;
                let a580 = v2;
                let a581 = 0u64;
                nat_quot(a579, a580, a581)
            }, {
                let a585 = 0u64;
                let a586 = {
                    let a582 = v0;
                    let a583 = v2;
                    let a584 = 0u64;
                    nat_rem(a582, a583, a584)
                };
                nat_lt(a585, a586)
            })
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a609 = f8({
        let a607 = {
            let a605 = {
                let a587 = 1u64;
                let a588 = 1u64;
                nat_add(a587, a588)?
            };
            let a606 = {
                let a603 = {
                    let a601 = {
                        let a589 = 1u64;
                        let a590 = 1u64;
                        nat_add(a589, a590)?
                    };
                    let a602 = {
                        let a599 = {
                            let a591 = 1u64;
                            let a592 = 1u64;
                            nat_add(a591, a592)?
                        };
                        let a600 = {
                            let a597 = {
                                let a595 = {
                                    let a593 = 1u64;
                                    let a594 = 1u64;
                                    nat_add(a593, a594)?
                                };
                                let a596 = 1u64;
                                nat_mul(a595, a596)?
                            };
                            let a598 = 1u64;
                            nat_add(a597, a598)?
                        };
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
    })?;
    let a610 = v2;
    if nat_le(a609, a610) {
        f20(v0, {
            let a611 = v1;
            let a612 = 1u64;
            nat_add(a611, a612)?
        }, f17(v2, 1u64)?)
    } else {
        let v3: Adt1 = f21(v1, v2, {
            let a649 = {
                let a647 = {
                    let a613 = 1u64;
                    let a614 = 1u64;
                    nat_add(a613, a614)?
                };
                let a648 = {
                    let a645 = {
                        let a615 = 1u64;
                        let a616 = 1u64;
                        nat_add(a615, a616)?
                    };
                    let a646 = {
                        let a643 = {
                            let a617 = 1u64;
                            let a618 = 1u64;
                            nat_add(a617, a618)?
                        };
                        let a644 = {
                            let a641 = {
                                let a619 = 1u64;
                                let a620 = 1u64;
                                nat_add(a619, a620)?
                            };
                            let a642 = {
                                let a639 = {
                                    let a621 = 1u64;
                                    let a622 = 1u64;
                                    nat_add(a621, a622)?
                                };
                                let a640 = {
                                    let a637 = {
                                        let a623 = 1u64;
                                        let a624 = 1u64;
                                        nat_add(a623, a624)?
                                    };
                                    let a638 = {
                                        let a635 = {
                                            let a625 = 1u64;
                                            let a626 = 1u64;
                                            nat_add(a625, a626)?
                                        };
                                        let a636 = {
                                            let a633 = {
                                                let a627 = 1u64;
                                                let a628 = 1u64;
                                                nat_add(a627, a628)?
                                            };
                                            let a634 = {
                                                let a631 = {
                                                    let a629 = 1u64;
                                                    let a630 = 1u64;
                                                    nat_add(a629, a630)?
                                                };
                                                let a632 = 1u64;
                                                nat_mul(a631, a632)?
                                            };
                                            nat_mul(a633, a634)?
                                        };
                                        nat_mul(a635, a636)?
                                    };
                                    nat_mul(a637, a638)?
                                };
                                nat_mul(a639, a640)?
                            };
                            nat_mul(a641, a642)?
                        };
                        nat_mul(a643, a644)?
                    };
                    nat_mul(a645, a646)?
                };
                nat_mul(a647, a648)?
            };
            let a650 = 1u64;
            nat_add(a649, a650)?
        }, f8({
            let a669 = {
                let a651 = 1u64;
                let a652 = 1u64;
                nat_add(a651, a652)?
            };
            let a670 = {
                let a667 = {
                    let a665 = {
                        let a653 = 1u64;
                        let a654 = 1u64;
                        nat_add(a653, a654)?
                    };
                    let a666 = {
                        let a663 = {
                            let a655 = 1u64;
                            let a656 = 1u64;
                            nat_add(a655, a656)?
                        };
                        let a664 = {
                            let a661 = {
                                let a659 = {
                                    let a657 = 1u64;
                                    let a658 = 1u64;
                                    nat_add(a657, a658)?
                                };
                                let a660 = 1u64;
                                nat_mul(a659, a660)?
                            };
                            let a662 = 1u64;
                            nat_add(a661, a662)?
                        };
                        nat_mul(a663, a664)?
                    };
                    nat_mul(a665, a666)?
                };
                let a668 = 1u64;
                nat_add(a667, a668)?
            };
            nat_mul(a669, a670)?
        })?)?;
        f20(v0, match v3.clone() {
            Adt1::C0(h671, _, _, _) => {
                h671
            }
        }, match v3 {
            Adt1::C0(_, h672, _, _) => {
                h672
            }
        })
    }
}

pub fn f19(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a680 = {
            let a677 = v0;
            let a678 = {
                let a675 = {
                    let a673 = 1u64;
                    let a674 = 1u64;
                    nat_add(a673, a674)?
                };
                let a676 = 1u64;
                nat_mul(a675, a676)?
            };
            let a679 = 0u64;
            nat_rem(a677, a678, a679)
        };
        let a681 = 0u64;
        nat_eq(a680, a681)
    } else {
        false
    } {
        let a682 = v0;
        let a683 = 1u64;
        nat_add(a682, a683)
    } else {
        Ok(v0)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a722 = v1;
    let a723 = {
        let a720 = {
            let a718 = {
                let a684 = 1u64;
                let a685 = 1u64;
                nat_add(a684, a685)?
            };
            let a719 = {
                let a716 = {
                    let a686 = 1u64;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                };
                let a717 = {
                    let a714 = {
                        let a688 = 1u64;
                        let a689 = 1u64;
                        nat_add(a688, a689)?
                    };
                    let a715 = {
                        let a712 = {
                            let a690 = 1u64;
                            let a691 = 1u64;
                            nat_add(a690, a691)?
                        };
                        let a713 = {
                            let a710 = {
                                let a692 = 1u64;
                                let a693 = 1u64;
                                nat_add(a692, a693)?
                            };
                            let a711 = {
                                let a708 = {
                                    let a694 = 1u64;
                                    let a695 = 1u64;
                                    nat_add(a694, a695)?
                                };
                                let a709 = {
                                    let a706 = {
                                        let a696 = 1u64;
                                        let a697 = 1u64;
                                        nat_add(a696, a697)?
                                    };
                                    let a707 = {
                                        let a704 = {
                                            let a698 = 1u64;
                                            let a699 = 1u64;
                                            nat_add(a698, a699)?
                                        };
                                        let a705 = {
                                            let a702 = {
                                                let a700 = 1u64;
                                                let a701 = 1u64;
                                                nat_add(a700, a701)?
                                            };
                                            let a703 = 1u64;
                                            nat_mul(a702, a703)?
                                        };
                                        nat_mul(a704, a705)?
                                    };
                                    nat_mul(a706, a707)?
                                };
                                nat_mul(a708, a709)?
                            };
                            nat_mul(a710, a711)?
                        };
                        nat_mul(a712, a713)?
                    };
                    nat_mul(a714, a715)?
                };
                nat_mul(a716, a717)?
            };
            nat_mul(a718, a719)?
        };
        let a721 = 1u64;
        nat_add(a720, a721)?
    };
    if nat_lt(a722, a723) {
        f22(v0, {
            let a760 = {
                let a758 = {
                    let a724 = 1u64;
                    let a725 = 1u64;
                    nat_add(a724, a725)?
                };
                let a759 = {
                    let a756 = {
                        let a726 = 1u64;
                        let a727 = 1u64;
                        nat_add(a726, a727)?
                    };
                    let a757 = {
                        let a754 = {
                            let a728 = 1u64;
                            let a729 = 1u64;
                            nat_add(a728, a729)?
                        };
                        let a755 = {
                            let a752 = {
                                let a730 = 1u64;
                                let a731 = 1u64;
                                nat_add(a730, a731)?
                            };
                            let a753 = {
                                let a750 = {
                                    let a732 = 1u64;
                                    let a733 = 1u64;
                                    nat_add(a732, a733)?
                                };
                                let a751 = {
                                    let a748 = {
                                        let a734 = 1u64;
                                        let a735 = 1u64;
                                        nat_add(a734, a735)?
                                    };
                                    let a749 = {
                                        let a746 = {
                                            let a736 = 1u64;
                                            let a737 = 1u64;
                                            nat_add(a736, a737)?
                                        };
                                        let a747 = {
                                            let a744 = {
                                                let a738 = 1u64;
                                                let a739 = 1u64;
                                                nat_add(a738, a739)?
                                            };
                                            let a745 = {
                                                let a742 = {
                                                    let a740 = 1u64;
                                                    let a741 = 1u64;
                                                    nat_add(a740, a741)?
                                                };
                                                let a743 = 1u64;
                                                nat_mul(a742, a743)?
                                            };
                                            nat_mul(a744, a745)?
                                        };
                                        nat_mul(a746, a747)?
                                    };
                                    nat_mul(a748, a749)?
                                };
                                nat_mul(a750, a751)?
                            };
                            nat_mul(a752, a753)?
                        };
                        nat_mul(a754, a755)?
                    };
                    nat_mul(a756, a757)?
                };
                nat_mul(a758, a759)?
            };
            let a761 = 1u64;
            nat_add(a760, a761)?
        }, f17(v2, {
            let a800 = {
                let a798 = {
                    let a796 = {
                        let a762 = 1u64;
                        let a763 = 1u64;
                        nat_add(a762, a763)?
                    };
                    let a797 = {
                        let a794 = {
                            let a764 = 1u64;
                            let a765 = 1u64;
                            nat_add(a764, a765)?
                        };
                        let a795 = {
                            let a792 = {
                                let a766 = 1u64;
                                let a767 = 1u64;
                                nat_add(a766, a767)?
                            };
                            let a793 = {
                                let a790 = {
                                    let a768 = 1u64;
                                    let a769 = 1u64;
                                    nat_add(a768, a769)?
                                };
                                let a791 = {
                                    let a788 = {
                                        let a770 = 1u64;
                                        let a771 = 1u64;
                                        nat_add(a770, a771)?
                                    };
                                    let a789 = {
                                        let a786 = {
                                            let a772 = 1u64;
                                            let a773 = 1u64;
                                            nat_add(a772, a773)?
                                        };
                                        let a787 = {
                                            let a784 = {
                                                let a774 = 1u64;
                                                let a775 = 1u64;
                                                nat_add(a774, a775)?
                                            };
                                            let a785 = {
                                                let a782 = {
                                                    let a776 = 1u64;
                                                    let a777 = 1u64;
                                                    nat_add(a776, a777)?
                                                };
                                                let a783 = {
                                                    let a780 = {
                                                        let a778 = 1u64;
                                                        let a779 = 1u64;
                                                        nat_add(a778, a779)?
                                                    };
                                                    let a781 = 1u64;
                                                    nat_mul(a780, a781)?
                                                };
                                                nat_mul(a782, a783)?
                                            };
                                            nat_mul(a784, a785)?
                                        };
                                        nat_mul(a786, a787)?
                                    };
                                    nat_mul(a788, a789)?
                                };
                                nat_mul(a790, a791)?
                            };
                            nat_mul(a792, a793)?
                        };
                        nat_mul(a794, a795)?
                    };
                    nat_mul(a796, a797)?
                };
                let a799 = 1u64;
                nat_add(a798, a799)?
            };
            let a801 = v1;
            nat_sub(a800, a801)
        })?)
    } else {
        f22(v0, v1, v2)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h802, _) = f23(Fn1::F24, {
        let a821 = {
            let a803 = 1u64;
            let a804 = 1u64;
            nat_add(a803, a804)?
        };
        let a822 = {
            let a819 = {
                let a805 = 1u64;
                let a806 = 1u64;
                nat_add(a805, a806)?
            };
            let a820 = {
                let a817 = {
                    let a807 = 1u64;
                    let a808 = 1u64;
                    nat_add(a807, a808)?
                };
                let a818 = {
                    let a815 = {
                        let a809 = 1u64;
                        let a810 = 1u64;
                        nat_add(a809, a810)?
                    };
                    let a816 = {
                        let a813 = {
                            let a811 = 1u64;
                            let a812 = 1u64;
                            nat_add(a811, a812)?
                        };
                        let a814 = 1u64;
                        nat_mul(a813, a814)?
                    };
                    nat_mul(a815, a816)?
                };
                nat_mul(a817, a818)?
            };
            nat_mul(a819, a820)?
        };
        nat_mul(a821, a822)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h802)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a835 = v2;
        let a836 = {
            let a833 = {
                let a823 = 1u64;
                let a824 = 1u64;
                nat_add(a823, a824)?
            };
            let a834 = {
                let a831 = {
                    let a825 = 1u64;
                    let a826 = 1u64;
                    nat_add(a825, a826)?
                };
                let a832 = {
                    let a829 = {
                        let a827 = 1u64;
                        let a828 = 1u64;
                        nat_add(a827, a828)?
                    };
                    let a830 = 1u64;
                    nat_mul(a829, a830)?
                };
                nat_mul(a831, a832)?
            };
            nat_mul(a833, a834)?
        };
        let a837 = 0u64;
        nat_rem(a835, a836, a837)
    };
    let v4: u64 = {
        let a850 = v2;
        let a851 = {
            let a848 = {
                let a838 = 1u64;
                let a839 = 1u64;
                nat_add(a838, a839)?
            };
            let a849 = {
                let a846 = {
                    let a840 = 1u64;
                    let a841 = 1u64;
                    nat_add(a840, a841)?
                };
                let a847 = {
                    let a844 = {
                        let a842 = 1u64;
                        let a843 = 1u64;
                        nat_add(a842, a843)?
                    };
                    let a845 = 1u64;
                    nat_mul(a844, a845)?
                };
                nat_mul(a846, a847)?
            };
            nat_mul(a848, a849)?
        };
        let a852 = 0u64;
        nat_quot(a850, a851, a852)
    };
    let v5: bool = {
        let a861 = {
            let a859 = {
                let a853 = 1u64;
                let a854 = 1u64;
                nat_add(a853, a854)?
            };
            let a860 = {
                let a857 = {
                    let a855 = 1u64;
                    let a856 = 1u64;
                    nat_add(a855, a856)?
                };
                let a858 = 1u64;
                nat_mul(a857, a858)?
            };
            nat_mul(a859, a860)?
        };
        let a862 = v3;
        if nat_lt(a861, a862) {
            true
        } else {
            let a871 = v3;
            let a872 = {
                let a869 = {
                    let a863 = 1u64;
                    let a864 = 1u64;
                    nat_add(a863, a864)?
                };
                let a870 = {
                    let a867 = {
                        let a865 = 1u64;
                        let a866 = 1u64;
                        nat_add(a865, a866)?
                    };
                    let a868 = 1u64;
                    nat_mul(a867, a868)?
                };
                nat_mul(a869, a870)?
            };
            if nat_eq(a871, a872) {
                let a880 = {
                    let a877 = v4;
                    let a878 = {
                        let a875 = {
                            let a873 = 1u64;
                            let a874 = 1u64;
                            nat_add(a873, a874)?
                        };
                        let a876 = 1u64;
                        nat_mul(a875, a876)?
                    };
                    let a879 = 0u64;
                    nat_rem(a877, a878, a879)
                };
                let a881 = 1u64;
                nat_eq(a880, a881)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a882 = v4;
        let a883 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a882, a883)?
    };
    let v7: bool = {
        let a912 = {
            let a910 = {
                let a886 = {
                    let a884 = 1u64;
                    let a885 = 1u64;
                    nat_add(a884, a885)?
                };
                let a887 = 1u64;
                nat_mul(a886, a887)?
            };
            let a911 = f8({
                let a908 = {
                    let a906 = {
                        let a888 = 1u64;
                        let a889 = 1u64;
                        nat_add(a888, a889)?
                    };
                    let a907 = {
                        let a904 = {
                            let a902 = {
                                let a890 = 1u64;
                                let a891 = 1u64;
                                nat_add(a890, a891)?
                            };
                            let a903 = {
                                let a900 = {
                                    let a898 = {
                                        let a892 = 1u64;
                                        let a893 = 1u64;
                                        nat_add(a892, a893)?
                                    };
                                    let a899 = {
                                        let a896 = {
                                            let a894 = 1u64;
                                            let a895 = 1u64;
                                            nat_add(a894, a895)?
                                        };
                                        let a897 = 1u64;
                                        nat_mul(a896, a897)?
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
                    };
                    nat_mul(a906, a907)?
                };
                let a909 = 1u64;
                nat_add(a908, a909)?
            })?;
            nat_mul(a910, a911)?
        };
        let a913 = v6;
        nat_le(a912, a913)
    };
    let v8: u64 = if v7 {
        let a918 = v6;
        let a919 = {
            let a916 = {
                let a914 = 1u64;
                let a915 = 1u64;
                nat_add(a914, a915)?
            };
            let a917 = 1u64;
            nat_mul(a916, a917)?
        };
        let a920 = 0u64;
        nat_quot(a918, a919, a920)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a921 = v1;
        let a922 = 1u64;
        nat_add(a921, a922)?
    } else {
        v1
    };
    let v10: u64 = {
        let a961 = v9;
        let a962 = {
            let a959 = {
                let a957 = {
                    let a923 = 1u64;
                    let a924 = 1u64;
                    nat_add(a923, a924)?
                };
                let a958 = {
                    let a955 = {
                        let a925 = 1u64;
                        let a926 = 1u64;
                        nat_add(a925, a926)?
                    };
                    let a956 = {
                        let a953 = {
                            let a927 = 1u64;
                            let a928 = 1u64;
                            nat_add(a927, a928)?
                        };
                        let a954 = {
                            let a951 = {
                                let a929 = 1u64;
                                let a930 = 1u64;
                                nat_add(a929, a930)?
                            };
                            let a952 = {
                                let a949 = {
                                    let a931 = 1u64;
                                    let a932 = 1u64;
                                    nat_add(a931, a932)?
                                };
                                let a950 = {
                                    let a947 = {
                                        let a933 = 1u64;
                                        let a934 = 1u64;
                                        nat_add(a933, a934)?
                                    };
                                    let a948 = {
                                        let a945 = {
                                            let a935 = 1u64;
                                            let a936 = 1u64;
                                            nat_add(a935, a936)?
                                        };
                                        let a946 = {
                                            let a943 = {
                                                let a937 = 1u64;
                                                let a938 = 1u64;
                                                nat_add(a937, a938)?
                                            };
                                            let a944 = {
                                                let a941 = {
                                                    let a939 = 1u64;
                                                    let a940 = 1u64;
                                                    nat_add(a939, a940)?
                                                };
                                                let a942 = 1u64;
                                                nat_mul(a941, a942)?
                                            };
                                            nat_mul(a943, a944)?
                                        };
                                        nat_mul(a945, a946)?
                                    };
                                    nat_mul(a947, a948)?
                                };
                                nat_mul(a949, a950)?
                            };
                            nat_mul(a951, a952)?
                        };
                        nat_mul(a953, a954)?
                    };
                    nat_mul(a955, a956)?
                };
                nat_mul(a957, a958)?
            };
            let a960 = 1u64;
            nat_add(a959, a960)?
        };
        if if nat_eq(a961, a962) {
            let a985 = v8;
            let a986 = f8({
                let a983 = {
                    let a981 = {
                        let a963 = 1u64;
                        let a964 = 1u64;
                        nat_add(a963, a964)?
                    };
                    let a982 = {
                        let a979 = {
                            let a977 = {
                                let a965 = 1u64;
                                let a966 = 1u64;
                                nat_add(a965, a966)?
                            };
                            let a978 = {
                                let a975 = {
                                    let a973 = {
                                        let a967 = 1u64;
                                        let a968 = 1u64;
                                        nat_add(a967, a968)?
                                    };
                                    let a974 = {
                                        let a971 = {
                                            let a969 = 1u64;
                                            let a970 = 1u64;
                                            nat_add(a969, a970)?
                                        };
                                        let a972 = 1u64;
                                        nat_mul(a971, a972)?
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
            })?;
            nat_lt(a985, a986)
        } else {
            false
        } {
            0u64
        } else {
            let a1023 = v9;
            let a1024 = {
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
            nat_sub(a1023, a1024)
        }
    };
    let a1025 = v8;
    let a1026 = 0u64;
    if nat_eq(a1025, a1026) {
        f25(v0)
    } else {
        let a1079 = {
            let a1077 = {
                let a1075 = {
                    let a1027 = 1u64;
                    let a1028 = 1u64;
                    nat_add(a1027, a1028)?
                };
                let a1076 = {
                    let a1073 = {
                        let a1071 = {
                            let a1029 = 1u64;
                            let a1030 = 1u64;
                            nat_add(a1029, a1030)?
                        };
                        let a1072 = {
                            let a1069 = {
                                let a1067 = {
                                    let a1031 = 1u64;
                                    let a1032 = 1u64;
                                    nat_add(a1031, a1032)?
                                };
                                let a1068 = {
                                    let a1065 = {
                                        let a1063 = {
                                            let a1033 = 1u64;
                                            let a1034 = 1u64;
                                            nat_add(a1033, a1034)?
                                        };
                                        let a1064 = {
                                            let a1061 = {
                                                let a1059 = {
                                                    let a1035 = 1u64;
                                                    let a1036 = 1u64;
                                                    nat_add(a1035, a1036)?
                                                };
                                                let a1060 = {
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
                                                                        let a1047 = {
                                                                            let a1041 = 1u64;
                                                                            let a1042 = 1u64;
                                                                            nat_add(a1041, a1042)?
                                                                        };
                                                                        let a1048 = {
                                                                            let a1045 = {
                                                                                let a1043 = 1u64;
                                                                                let a1044 = 1u64;
                                                                                nat_add(a1043, a1044)?
                                                                            };
                                                                            let a1046 = 1u64;
                                                                            nat_mul(a1045, a1046)?
                                                                        };
                                                                        nat_mul(a1047, a1048)?
                                                                    };
                                                                    let a1050 = 1u64;
                                                                    nat_add(a1049, a1050)?
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
                                                };
                                                nat_mul(a1059, a1060)?
                                            };
                                            let a1062 = 1u64;
                                            nat_add(a1061, a1062)?
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
                };
                nat_mul(a1075, a1076)?
            };
            let a1078 = 1u64;
            nat_add(a1077, a1078)?
        };
        let a1080 = v9;
        if nat_le(a1079, a1080) {
            f26(v0)
        } else {
            let a1132 = {
                let a1105 = f25(v0)?;
                let a1106 = {
                    let a1103 = v10;
                    let a1104 = f8({
                        let a1101 = {
                            let a1099 = {
                                let a1081 = 1u64;
                                let a1082 = 1u64;
                                nat_add(a1081, a1082)?
                            };
                            let a1100 = {
                                let a1097 = {
                                    let a1095 = {
                                        let a1083 = 1u64;
                                        let a1084 = 1u64;
                                        nat_add(a1083, a1084)?
                                    };
                                    let a1096 = {
                                        let a1093 = {
                                            let a1091 = {
                                                let a1085 = 1u64;
                                                let a1086 = 1u64;
                                                nat_add(a1085, a1086)?
                                            };
                                            let a1092 = {
                                                let a1089 = {
                                                    let a1087 = 1u64;
                                                    let a1088 = 1u64;
                                                    nat_add(a1087, a1088)?
                                                };
                                                let a1090 = 1u64;
                                                nat_mul(a1089, a1090)?
                                            };
                                            nat_mul(a1091, a1092)?
                                        };
                                        let a1094 = 1u64;
                                        nat_add(a1093, a1094)?
                                    };
                                    nat_mul(a1095, a1096)?
                                };
                                let a1098 = 1u64;
                                nat_add(a1097, a1098)?
                            };
                            nat_mul(a1099, a1100)?
                        };
                        let a1102 = 1u64;
                        nat_add(a1101, a1102)?
                    })?;
                    nat_mul(a1103, a1104)?
                };
                nat_add(a1105, a1106)?
            };
            let a1133 = {
                let a1129 = v8;
                let a1130 = f8({
                    let a1127 = {
                        let a1125 = {
                            let a1107 = 1u64;
                            let a1108 = 1u64;
                            nat_add(a1107, a1108)?
                        };
                        let a1126 = {
                            let a1123 = {
                                let a1121 = {
                                    let a1109 = 1u64;
                                    let a1110 = 1u64;
                                    nat_add(a1109, a1110)?
                                };
                                let a1122 = {
                                    let a1119 = {
                                        let a1117 = {
                                            let a1111 = 1u64;
                                            let a1112 = 1u64;
                                            nat_add(a1111, a1112)?
                                        };
                                        let a1118 = {
                                            let a1115 = {
                                                let a1113 = 1u64;
                                                let a1114 = 1u64;
                                                nat_add(a1113, a1114)?
                                            };
                                            let a1116 = 1u64;
                                            nat_mul(a1115, a1116)?
                                        };
                                        nat_mul(a1117, a1118)?
                                    };
                                    let a1120 = 1u64;
                                    nat_add(a1119, a1120)?
                                };
                                nat_mul(a1121, a1122)?
                            };
                            let a1124 = 1u64;
                            nat_add(a1123, a1124)?
                        };
                        nat_mul(a1125, a1126)?
                    };
                    let a1128 = 1u64;
                    nat_add(a1127, a1128)?
                })?;
                let a1131 = 0u64;
                nat_rem(a1129, a1130, a1131)
            };
            nat_add(a1132, a1133)
        }
    }
}

pub fn f23(v0: Fn1, p1138: u64, p1139: Adt1) -> R<(Adt1, bool)> {
    let m1137 = (p1138, p1139);
    {
        let mut m1137 = m1137;
        loop {
            let step = {
                let (v1, v2) = m1137;
                let m1134 = v1;
                if m1134 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1134 - 1;
                    let m1136 = {
                        let c1135: Fn1 = v0.clone();
                        c1135.apply(v2.clone())?
                    };
                    match m1136 {
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
                Ok(next) => { m1137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Adt1) -> R<Option<Adt1>> {
    f27(v0)
}

pub fn f25(v0: u64) -> R<u64> {
    let a1164 = v0;
    let a1165 = f8({
        let a1162 = {
            let a1160 = {
                let a1140 = 1u64;
                let a1141 = 1u64;
                nat_add(a1140, a1141)?
            };
            let a1161 = {
                let a1158 = {
                    let a1156 = {
                        let a1142 = 1u64;
                        let a1143 = 1u64;
                        nat_add(a1142, a1143)?
                    };
                    let a1157 = {
                        let a1154 = {
                            let a1152 = {
                                let a1144 = 1u64;
                                let a1145 = 1u64;
                                nat_add(a1144, a1145)?
                            };
                            let a1153 = {
                                let a1150 = {
                                    let a1148 = {
                                        let a1146 = 1u64;
                                        let a1147 = 1u64;
                                        nat_add(a1146, a1147)?
                                    };
                                    let a1149 = 1u64;
                                    nat_mul(a1148, a1149)?
                                };
                                let a1151 = 1u64;
                                nat_add(a1150, a1151)?
                            };
                            nat_mul(a1152, a1153)?
                        };
                        let a1155 = 1u64;
                        nat_add(a1154, a1155)?
                    };
                    nat_mul(a1156, a1157)?
                };
                let a1159 = 1u64;
                nat_add(a1158, a1159)?
            };
            nat_mul(a1160, a1161)?
        };
        let a1163 = 1u64;
        nat_add(a1162, a1163)?
    })?;
    nat_mul(a1164, a1165)
}

pub fn f26(v0: u64) -> R<u64> {
    let a1232 = f25(v0)?;
    let a1233 = {
        let a1230 = {
            let a1206 = {
                let a1204 = {
                    let a1166 = 1u64;
                    let a1167 = 1u64;
                    nat_add(a1166, a1167)?
                };
                let a1205 = {
                    let a1202 = {
                        let a1200 = {
                            let a1168 = 1u64;
                            let a1169 = 1u64;
                            nat_add(a1168, a1169)?
                        };
                        let a1201 = {
                            let a1198 = {
                                let a1196 = {
                                    let a1170 = 1u64;
                                    let a1171 = 1u64;
                                    nat_add(a1170, a1171)?
                                };
                                let a1197 = {
                                    let a1194 = {
                                        let a1192 = {
                                            let a1172 = 1u64;
                                            let a1173 = 1u64;
                                            nat_add(a1172, a1173)?
                                        };
                                        let a1193 = {
                                            let a1190 = {
                                                let a1188 = {
                                                    let a1174 = 1u64;
                                                    let a1175 = 1u64;
                                                    nat_add(a1174, a1175)?
                                                };
                                                let a1189 = {
                                                    let a1186 = {
                                                        let a1184 = {
                                                            let a1176 = 1u64;
                                                            let a1177 = 1u64;
                                                            nat_add(a1176, a1177)?
                                                        };
                                                        let a1185 = {
                                                            let a1182 = {
                                                                let a1180 = {
                                                                    let a1178 = 1u64;
                                                                    let a1179 = 1u64;
                                                                    nat_add(a1178, a1179)?
                                                                };
                                                                let a1181 = 1u64;
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
                                        };
                                        nat_mul(a1192, a1193)?
                                    };
                                    let a1195 = 1u64;
                                    nat_add(a1194, a1195)?
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
                };
                nat_mul(a1204, a1205)?
            };
            let a1207 = 1u64;
            nat_add(a1206, a1207)?
        };
        let a1231 = f8({
            let a1228 = {
                let a1226 = {
                    let a1208 = 1u64;
                    let a1209 = 1u64;
                    nat_add(a1208, a1209)?
                };
                let a1227 = {
                    let a1224 = {
                        let a1222 = {
                            let a1210 = 1u64;
                            let a1211 = 1u64;
                            nat_add(a1210, a1211)?
                        };
                        let a1223 = {
                            let a1220 = {
                                let a1218 = {
                                    let a1212 = 1u64;
                                    let a1213 = 1u64;
                                    nat_add(a1212, a1213)?
                                };
                                let a1219 = {
                                    let a1216 = {
                                        let a1214 = 1u64;
                                        let a1215 = 1u64;
                                        nat_add(a1214, a1215)?
                                    };
                                    let a1217 = 1u64;
                                    nat_mul(a1216, a1217)?
                                };
                                nat_mul(a1218, a1219)?
                            };
                            let a1221 = 1u64;
                            nat_add(a1220, a1221)?
                        };
                        nat_mul(a1222, a1223)?
                    };
                    let a1225 = 1u64;
                    nat_add(a1224, a1225)?
                };
                nat_mul(a1226, a1227)?
            };
            let a1229 = 1u64;
            nat_add(a1228, a1229)?
        })?;
        nat_mul(a1230, a1231)?
    };
    nat_add(a1232, a1233)
}

pub fn f27(v0: Adt1) -> R<Option<Adt1>> {
    let a1235 = 0u64;
    let a1236 = match v0.clone() {
        Adt1::C0(_, h1234, _, _) => {
            h1234
        }
    };
    if if if nat_lt(a1235, a1236) {
        let a1239 = match v0.clone() {
            Adt1::C0(_, h1237, _, _) => {
                h1237
            }
        };
        let a1240 = match v0.clone() {
            Adt1::C0(_, _, _, h1238) => {
                h1238
            }
        };
        nat_lt(a1239, a1240)
    } else {
        false
    } {
        let a1243 = match v0.clone() {
            Adt1::C0(_, _, h1241, _) => {
                h1241
            }
        };
        let a1244 = match v0.clone() {
            Adt1::C0(h1242, _, _, _) => {
                h1242
            }
        };
        nat_lt(a1243, a1244)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1246 = match v0.clone() {
                Adt1::C0(h1245, _, _, _) => {
                    h1245
                }
            };
            let a1247 = 1u64;
            nat_sub(a1246, a1247)
        }, {
            let a1250 = match v0.clone() {
                Adt1::C0(_, h1248, _, _) => {
                    h1248
                }
            };
            let a1251 = match v0.clone() {
                Adt1::C0(_, h1249, _, _) => {
                    h1249
                }
            };
            nat_add(a1250, a1251)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1252, _) => {
                h1252
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1253) => {
                h1253
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}
