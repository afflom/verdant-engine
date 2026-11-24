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
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F17,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F17 => f17(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F29 => f29(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let v4: u64 = f1(v0, v1)?;
    let v5: u64 = f2(v4, v3, 1262485504u64)?;
    Ok(Adt0::C0(v5, f1(v2, f3(v5)?)?))
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        true
    } else {
        if f5(v0)? {
            f6(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f5(v1)? {
            f6(v0)?
        } else {
            false
        }
    } {
        let a87 = {
            let a65 = {
                let a41 = {
                    let a39 = {
                        let a1 = 1u64;
                        let a2 = 1u64;
                        nat_add(a1, a2)?
                    };
                    let a40 = {
                        let a37 = {
                            let a35 = {
                                let a3 = 1u64;
                                let a4 = 1u64;
                                nat_add(a3, a4)?
                            };
                            let a36 = {
                                let a33 = {
                                    let a31 = {
                                        let a5 = 1u64;
                                        let a6 = 1u64;
                                        nat_add(a5, a6)?
                                    };
                                    let a32 = {
                                        let a29 = {
                                            let a27 = {
                                                let a7 = 1u64;
                                                let a8 = 1u64;
                                                nat_add(a7, a8)?
                                            };
                                            let a28 = {
                                                let a25 = {
                                                    let a23 = {
                                                        let a9 = 1u64;
                                                        let a10 = 1u64;
                                                        nat_add(a9, a10)?
                                                    };
                                                    let a24 = {
                                                        let a21 = {
                                                            let a19 = {
                                                                let a11 = 1u64;
                                                                let a12 = 1u64;
                                                                nat_add(a11, a12)?
                                                            };
                                                            let a20 = {
                                                                let a17 = {
                                                                    let a15 = {
                                                                        let a13 = 1u64;
                                                                        let a14 = 1u64;
                                                                        nat_add(a13, a14)?
                                                                    };
                                                                    let a16 = 1u64;
                                                                    nat_mul(a15, a16)?
                                                                };
                                                                let a18 = 1u64;
                                                                nat_add(a17, a18)?
                                                            };
                                                            nat_mul(a19, a20)?
                                                        };
                                                        let a22 = 1u64;
                                                        nat_add(a21, a22)?
                                                    };
                                                    nat_mul(a23, a24)?
                                                };
                                                let a26 = 1u64;
                                                nat_add(a25, a26)?
                                            };
                                            nat_mul(a27, a28)?
                                        };
                                        let a30 = 1u64;
                                        nat_add(a29, a30)?
                                    };
                                    nat_mul(a31, a32)?
                                };
                                let a34 = 1u64;
                                nat_add(a33, a34)?
                            };
                            nat_mul(a35, a36)?
                        };
                        let a38 = 1u64;
                        nat_add(a37, a38)?
                    };
                    nat_mul(a39, a40)?
                };
                let a42 = 1u64;
                nat_add(a41, a42)?
            };
            let a66 = f7({
                let a63 = {
                    let a61 = {
                        let a43 = 1u64;
                        let a44 = 1u64;
                        nat_add(a43, a44)?
                    };
                    let a62 = {
                        let a59 = {
                            let a57 = {
                                let a45 = 1u64;
                                let a46 = 1u64;
                                nat_add(a45, a46)?
                            };
                            let a58 = {
                                let a55 = {
                                    let a53 = {
                                        let a47 = 1u64;
                                        let a48 = 1u64;
                                        nat_add(a47, a48)?
                                    };
                                    let a54 = {
                                        let a51 = {
                                            let a49 = 1u64;
                                            let a50 = 1u64;
                                            nat_add(a49, a50)?
                                        };
                                        let a52 = 1u64;
                                        nat_mul(a51, a52)?
                                    };
                                    nat_mul(a53, a54)?
                                };
                                let a56 = 1u64;
                                nat_add(a55, a56)?
                            };
                            nat_mul(a57, a58)?
                        };
                        let a60 = 1u64;
                        nat_add(a59, a60)?
                    };
                    nat_mul(a61, a62)?
                };
                let a64 = 1u64;
                nat_add(a63, a64)?
            })?;
            nat_mul(a65, a66)?
        };
        let a88 = f7({
            let a85 = {
                let a67 = 1u64;
                let a68 = 1u64;
                nat_add(a67, a68)?
            };
            let a86 = {
                let a83 = {
                    let a81 = {
                        let a69 = 1u64;
                        let a70 = 1u64;
                        nat_add(a69, a70)?
                    };
                    let a82 = {
                        let a79 = {
                            let a77 = {
                                let a71 = 1u64;
                                let a72 = 1u64;
                                nat_add(a71, a72)?
                            };
                            let a78 = {
                                let a75 = {
                                    let a73 = 1u64;
                                    let a74 = 1u64;
                                    nat_add(a73, a74)?
                                };
                                let a76 = 1u64;
                                nat_mul(a75, a76)?
                            };
                            nat_mul(a77, a78)?
                        };
                        let a80 = 1u64;
                        nat_add(a79, a80)?
                    };
                    nat_mul(a81, a82)?
                };
                let a84 = 1u64;
                nat_add(a83, a84)?
            };
            nat_mul(a85, a86)?
        })?;
        nat_add(a87, a88)
    } else {
        if if f5(v0)? {
            true
        } else {
            f5(v1)?
        } {
            f8(f9(v0, v1)?)
        } else {
            if if f6(v0)? {
                true
            } else {
                f6(v1)?
            } {
                f10(f9(v0, v1)?)
            } else {
                f11(v0, v1)
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f12(v0, v2)?;
    let v4: u64 = {
        let a89 = v3;
        let a90 = v2;
        nat_sub(a89, a90)
    };
    let a91 = v4;
    let a92 = v1;
    if nat_lt(a91, a92) {
        Ok(v4)
    } else {
        Ok(v1)
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let a93 = v0;
    let a94 = 0u64;
    if nat_eq(a93, a94) {
        Ok(0u64)
    } else {
        let a95 = v0;
        let a96 = 2u64;
        if nat_lt(a95, a96) {
            let a101 = 1065353216u64;
            let a102 = {
                let a99 = {
                    let a97 = v0;
                    let a98 = 1u64;
                    nat_sub(a97, a98)
                };
                let a100 = 8388608u64;
                nat_mul(a99, a100)?
            };
            nat_add(a101, a102)
        } else {
            let a103 = v0;
            let a104 = 4u64;
            if nat_lt(a103, a104) {
                let a109 = 1073741824u64;
                let a110 = {
                    let a107 = {
                        let a105 = v0;
                        let a106 = 2u64;
                        nat_sub(a105, a106)
                    };
                    let a108 = 4194304u64;
                    nat_mul(a107, a108)?
                };
                nat_add(a109, a110)
            } else {
                let a111 = v0;
                let a112 = 8u64;
                if nat_lt(a111, a112) {
                    let a117 = 1082130432u64;
                    let a118 = {
                        let a115 = {
                            let a113 = v0;
                            let a114 = 4u64;
                            nat_sub(a113, a114)
                        };
                        let a116 = 2097152u64;
                        nat_mul(a115, a116)?
                    };
                    nat_add(a117, a118)
                } else {
                    let a119 = v0;
                    let a120 = 16u64;
                    if nat_lt(a119, a120) {
                        let a125 = 1090519040u64;
                        let a126 = {
                            let a123 = {
                                let a121 = v0;
                                let a122 = 8u64;
                                nat_sub(a121, a122)
                            };
                            let a124 = 1048576u64;
                            nat_mul(a123, a124)?
                        };
                        nat_add(a125, a126)
                    } else {
                        let a127 = v0;
                        let a128 = 32u64;
                        if nat_lt(a127, a128) {
                            let a133 = 1098907648u64;
                            let a134 = {
                                let a131 = {
                                    let a129 = v0;
                                    let a130 = 16u64;
                                    nat_sub(a129, a130)
                                };
                                let a132 = 524288u64;
                                nat_mul(a131, a132)?
                            };
                            nat_add(a133, a134)
                        } else {
                            let a135 = v0;
                            let a136 = 64u64;
                            if nat_lt(a135, a136) {
                                let a141 = 1107296256u64;
                                let a142 = {
                                    let a139 = {
                                        let a137 = v0;
                                        let a138 = 32u64;
                                        nat_sub(a137, a138)
                                    };
                                    let a140 = 262144u64;
                                    nat_mul(a139, a140)?
                                };
                                nat_add(a141, a142)
                            } else {
                                let a143 = v0;
                                let a144 = 128u64;
                                if nat_lt(a143, a144) {
                                    let a149 = 1115684864u64;
                                    let a150 = {
                                        let a147 = {
                                            let a145 = v0;
                                            let a146 = 64u64;
                                            nat_sub(a145, a146)
                                        };
                                        let a148 = 131072u64;
                                        nat_mul(a147, a148)?
                                    };
                                    nat_add(a149, a150)
                                } else {
                                    let a151 = v0;
                                    let a152 = 256u64;
                                    if nat_lt(a151, a152) {
                                        let a157 = 1124073472u64;
                                        let a158 = {
                                            let a155 = {
                                                let a153 = v0;
                                                let a154 = 128u64;
                                                nat_sub(a153, a154)
                                            };
                                            let a156 = 65536u64;
                                            nat_mul(a155, a156)?
                                        };
                                        nat_add(a157, a158)
                                    } else {
                                        Ok(1132462080u64)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> R<bool> {
    let a201 = f13(v0)?;
    let a202 = {
        let a199 = {
            let a197 = {
                let a159 = 1u64;
                let a160 = 1u64;
                nat_add(a159, a160)?
            };
            let a198 = {
                let a195 = {
                    let a193 = {
                        let a161 = 1u64;
                        let a162 = 1u64;
                        nat_add(a161, a162)?
                    };
                    let a194 = {
                        let a191 = {
                            let a189 = {
                                let a163 = 1u64;
                                let a164 = 1u64;
                                nat_add(a163, a164)?
                            };
                            let a190 = {
                                let a187 = {
                                    let a185 = {
                                        let a165 = 1u64;
                                        let a166 = 1u64;
                                        nat_add(a165, a166)?
                                    };
                                    let a186 = {
                                        let a183 = {
                                            let a181 = {
                                                let a167 = 1u64;
                                                let a168 = 1u64;
                                                nat_add(a167, a168)?
                                            };
                                            let a182 = {
                                                let a179 = {
                                                    let a177 = {
                                                        let a169 = 1u64;
                                                        let a170 = 1u64;
                                                        nat_add(a169, a170)?
                                                    };
                                                    let a178 = {
                                                        let a175 = {
                                                            let a173 = {
                                                                let a171 = 1u64;
                                                                let a172 = 1u64;
                                                                nat_add(a171, a172)?
                                                            };
                                                            let a174 = 1u64;
                                                            nat_mul(a173, a174)?
                                                        };
                                                        let a176 = 1u64;
                                                        nat_add(a175, a176)?
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
            };
            nat_mul(a197, a198)?
        };
        let a200 = 1u64;
        nat_add(a199, a200)?
    };
    if nat_eq(a201, a202) {
        let a203 = 0u64;
        let a204 = f14(v0)?;
        Ok(nat_lt(a203, a204))
    } else {
        Ok(false)
    }
}

pub fn f5(v0: u64) -> R<bool> {
    let a247 = f13(v0)?;
    let a248 = {
        let a245 = {
            let a243 = {
                let a205 = 1u64;
                let a206 = 1u64;
                nat_add(a205, a206)?
            };
            let a244 = {
                let a241 = {
                    let a239 = {
                        let a207 = 1u64;
                        let a208 = 1u64;
                        nat_add(a207, a208)?
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = {
                                let a209 = 1u64;
                                let a210 = 1u64;
                                nat_add(a209, a210)?
                            };
                            let a236 = {
                                let a233 = {
                                    let a231 = {
                                        let a211 = 1u64;
                                        let a212 = 1u64;
                                        nat_add(a211, a212)?
                                    };
                                    let a232 = {
                                        let a229 = {
                                            let a227 = {
                                                let a213 = 1u64;
                                                let a214 = 1u64;
                                                nat_add(a213, a214)?
                                            };
                                            let a228 = {
                                                let a225 = {
                                                    let a223 = {
                                                        let a215 = 1u64;
                                                        let a216 = 1u64;
                                                        nat_add(a215, a216)?
                                                    };
                                                    let a224 = {
                                                        let a221 = {
                                                            let a219 = {
                                                                let a217 = 1u64;
                                                                let a218 = 1u64;
                                                                nat_add(a217, a218)?
                                                            };
                                                            let a220 = 1u64;
                                                            nat_mul(a219, a220)?
                                                        };
                                                        let a222 = 1u64;
                                                        nat_add(a221, a222)?
                                                    };
                                                    nat_mul(a223, a224)?
                                                };
                                                let a226 = 1u64;
                                                nat_add(a225, a226)?
                                            };
                                            nat_mul(a227, a228)?
                                        };
                                        let a230 = 1u64;
                                        nat_add(a229, a230)?
                                    };
                                    nat_mul(a231, a232)?
                                };
                                let a234 = 1u64;
                                nat_add(a233, a234)?
                            };
                            nat_mul(a235, a236)?
                        };
                        let a238 = 1u64;
                        nat_add(a237, a238)?
                    };
                    nat_mul(a239, a240)?
                };
                let a242 = 1u64;
                nat_add(a241, a242)?
            };
            nat_mul(a243, a244)?
        };
        let a246 = 1u64;
        nat_add(a245, a246)?
    };
    if nat_eq(a247, a248) {
        let a249 = f14(v0)?;
        let a250 = 0u64;
        Ok(nat_eq(a249, a250))
    } else {
        Ok(false)
    }
}

pub fn f6(v0: u64) -> R<bool> {
    let a251 = f15(v0)?;
    let a252 = 0u64;
    Ok(nat_eq(a251, a252))
}

pub fn f7(v0: u64) -> R<u64> {
    f16(Fn0::F17, v0, 1u64)
}

pub fn f8(v0: u64) -> R<u64> {
    let a319 = f10(v0)?;
    let a320 = {
        let a317 = {
            let a293 = {
                let a291 = {
                    let a253 = 1u64;
                    let a254 = 1u64;
                    nat_add(a253, a254)?
                };
                let a292 = {
                    let a289 = {
                        let a287 = {
                            let a255 = 1u64;
                            let a256 = 1u64;
                            nat_add(a255, a256)?
                        };
                        let a288 = {
                            let a285 = {
                                let a283 = {
                                    let a257 = 1u64;
                                    let a258 = 1u64;
                                    nat_add(a257, a258)?
                                };
                                let a284 = {
                                    let a281 = {
                                        let a279 = {
                                            let a259 = 1u64;
                                            let a260 = 1u64;
                                            nat_add(a259, a260)?
                                        };
                                        let a280 = {
                                            let a277 = {
                                                let a275 = {
                                                    let a261 = 1u64;
                                                    let a262 = 1u64;
                                                    nat_add(a261, a262)?
                                                };
                                                let a276 = {
                                                    let a273 = {
                                                        let a271 = {
                                                            let a263 = 1u64;
                                                            let a264 = 1u64;
                                                            nat_add(a263, a264)?
                                                        };
                                                        let a272 = {
                                                            let a269 = {
                                                                let a267 = {
                                                                    let a265 = 1u64;
                                                                    let a266 = 1u64;
                                                                    nat_add(a265, a266)?
                                                                };
                                                                let a268 = 1u64;
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
                                        nat_mul(a279, a280)?
                                    };
                                    let a282 = 1u64;
                                    nat_add(a281, a282)?
                                };
                                nat_mul(a283, a284)?
                            };
                            let a286 = 1u64;
                            nat_add(a285, a286)?
                        };
                        nat_mul(a287, a288)?
                    };
                    let a290 = 1u64;
                    nat_add(a289, a290)?
                };
                nat_mul(a291, a292)?
            };
            let a294 = 1u64;
            nat_add(a293, a294)?
        };
        let a318 = f7({
            let a315 = {
                let a313 = {
                    let a295 = 1u64;
                    let a296 = 1u64;
                    nat_add(a295, a296)?
                };
                let a314 = {
                    let a311 = {
                        let a309 = {
                            let a297 = 1u64;
                            let a298 = 1u64;
                            nat_add(a297, a298)?
                        };
                        let a310 = {
                            let a307 = {
                                let a305 = {
                                    let a299 = 1u64;
                                    let a300 = 1u64;
                                    nat_add(a299, a300)?
                                };
                                let a306 = {
                                    let a303 = {
                                        let a301 = 1u64;
                                        let a302 = 1u64;
                                        nat_add(a301, a302)?
                                    };
                                    let a304 = 1u64;
                                    nat_mul(a303, a304)?
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
        })?;
        nat_mul(a317, a318)?
    };
    nat_add(a319, a320)
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let a327 = {
        let a321 = f18(v0)?;
        let a322 = f18(v1)?;
        nat_add(a321, a322)?
    };
    let a328 = {
        let a325 = {
            let a323 = 1u64;
            let a324 = 1u64;
            nat_add(a323, a324)?
        };
        let a326 = 1u64;
        nat_mul(a325, a326)?
    };
    let a329 = 0u64;
    Ok(nat_rem(a327, a328, a329))
}

pub fn f10(v0: u64) -> R<u64> {
    let a354 = v0;
    let a355 = f7({
        let a352 = {
            let a350 = {
                let a330 = 1u64;
                let a331 = 1u64;
                nat_add(a330, a331)?
            };
            let a351 = {
                let a348 = {
                    let a346 = {
                        let a332 = 1u64;
                        let a333 = 1u64;
                        nat_add(a332, a333)?
                    };
                    let a347 = {
                        let a344 = {
                            let a342 = {
                                let a334 = 1u64;
                                let a335 = 1u64;
                                nat_add(a334, a335)?
                            };
                            let a343 = {
                                let a340 = {
                                    let a338 = {
                                        let a336 = 1u64;
                                        let a337 = 1u64;
                                        nat_add(a336, a337)?
                                    };
                                    let a339 = 1u64;
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
            nat_mul(a350, a351)?
        };
        let a353 = 1u64;
        nat_add(a352, a353)?
    })?;
    nat_mul(a354, a355)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f19(v0)?;
    let v3: Adt1 = f19(v1)?;
    f20(f9(v0, v1)?, {
        let a410 = {
            let a358 = match v2.clone() {
                Adt1::C0(h356, _, _, _) => {
                    h356
                }
            };
            let a359 = match v3.clone() {
                Adt1::C0(h357, _, _, _) => {
                    h357
                }
            };
            nat_add(a358, a359)?
        };
        let a411 = {
            let a408 = {
                let a406 = {
                    let a360 = 1u64;
                    let a361 = 1u64;
                    nat_add(a360, a361)?
                };
                let a407 = {
                    let a404 = {
                        let a402 = {
                            let a362 = 1u64;
                            let a363 = 1u64;
                            nat_add(a362, a363)?
                        };
                        let a403 = {
                            let a400 = {
                                let a398 = {
                                    let a364 = 1u64;
                                    let a365 = 1u64;
                                    nat_add(a364, a365)?
                                };
                                let a399 = {
                                    let a396 = {
                                        let a394 = {
                                            let a366 = 1u64;
                                            let a367 = 1u64;
                                            nat_add(a366, a367)?
                                        };
                                        let a395 = {
                                            let a392 = {
                                                let a390 = {
                                                    let a368 = 1u64;
                                                    let a369 = 1u64;
                                                    nat_add(a368, a369)?
                                                };
                                                let a391 = {
                                                    let a388 = {
                                                        let a386 = {
                                                            let a370 = 1u64;
                                                            let a371 = 1u64;
                                                            nat_add(a370, a371)?
                                                        };
                                                        let a387 = {
                                                            let a384 = {
                                                                let a382 = {
                                                                    let a372 = 1u64;
                                                                    let a373 = 1u64;
                                                                    nat_add(a372, a373)?
                                                                };
                                                                let a383 = {
                                                                    let a380 = {
                                                                        let a374 = 1u64;
                                                                        let a375 = 1u64;
                                                                        nat_add(a374, a375)?
                                                                    };
                                                                    let a381 = {
                                                                        let a378 = {
                                                                            let a376 = 1u64;
                                                                            let a377 = 1u64;
                                                                            nat_add(a376, a377)?
                                                                        };
                                                                        let a379 = 1u64;
                                                                        nat_mul(a378, a379)?
                                                                    };
                                                                    nat_mul(a380, a381)?
                                                                };
                                                                nat_mul(a382, a383)?
                                                            };
                                                            let a385 = 1u64;
                                                            nat_add(a384, a385)?
                                                        };
                                                        nat_mul(a386, a387)?
                                                    };
                                                    let a389 = 1u64;
                                                    nat_add(a388, a389)?
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
                        };
                        nat_mul(a402, a403)?
                    };
                    let a405 = 1u64;
                    nat_add(a404, a405)?
                };
                nat_mul(a406, a407)?
            };
            let a409 = 1u64;
            nat_add(a408, a409)?
        };
        nat_sub(a410, a411)
    }, f21({
        let a414 = match v2 {
            Adt1::C0(_, h412, _, _) => {
                h412
            }
        };
        let a415 = match v3 {
            Adt1::C0(_, h413, _, _) => {
                h413
            }
        };
        nat_mul(a414, a415)?
    }, {
        let a432 = {
            let a416 = 1u64;
            let a417 = 1u64;
            nat_add(a416, a417)?
        };
        let a433 = {
            let a430 = {
                let a418 = 1u64;
                let a419 = 1u64;
                nat_add(a418, a419)?
            };
            let a431 = {
                let a428 = {
                    let a426 = {
                        let a420 = 1u64;
                        let a421 = 1u64;
                        nat_add(a420, a421)?
                    };
                    let a427 = {
                        let a424 = {
                            let a422 = 1u64;
                            let a423 = 1u64;
                            nat_add(a422, a423)?
                        };
                        let a425 = 1u64;
                        nat_mul(a424, a425)?
                    };
                    nat_mul(a426, a427)?
                };
                let a429 = 1u64;
                nat_add(a428, a429)?
            };
            nat_mul(a430, a431)?
        };
        nat_mul(a432, a433)?
    })?)
}

pub fn f12(v0: u64, v1: u64) -> R<u64> {
    if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        let a520 = {
            let a498 = {
                let a474 = {
                    let a472 = {
                        let a434 = 1u64;
                        let a435 = 1u64;
                        nat_add(a434, a435)?
                    };
                    let a473 = {
                        let a470 = {
                            let a468 = {
                                let a436 = 1u64;
                                let a437 = 1u64;
                                nat_add(a436, a437)?
                            };
                            let a469 = {
                                let a466 = {
                                    let a464 = {
                                        let a438 = 1u64;
                                        let a439 = 1u64;
                                        nat_add(a438, a439)?
                                    };
                                    let a465 = {
                                        let a462 = {
                                            let a460 = {
                                                let a440 = 1u64;
                                                let a441 = 1u64;
                                                nat_add(a440, a441)?
                                            };
                                            let a461 = {
                                                let a458 = {
                                                    let a456 = {
                                                        let a442 = 1u64;
                                                        let a443 = 1u64;
                                                        nat_add(a442, a443)?
                                                    };
                                                    let a457 = {
                                                        let a454 = {
                                                            let a452 = {
                                                                let a444 = 1u64;
                                                                let a445 = 1u64;
                                                                nat_add(a444, a445)?
                                                            };
                                                            let a453 = {
                                                                let a450 = {
                                                                    let a448 = {
                                                                        let a446 = 1u64;
                                                                        let a447 = 1u64;
                                                                        nat_add(a446, a447)?
                                                                    };
                                                                    let a449 = 1u64;
                                                                    nat_mul(a448, a449)?
                                                                };
                                                                let a451 = 1u64;
                                                                nat_add(a450, a451)?
                                                            };
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
            let a499 = f7({
                let a496 = {
                    let a494 = {
                        let a476 = 1u64;
                        let a477 = 1u64;
                        nat_add(a476, a477)?
                    };
                    let a495 = {
                        let a492 = {
                            let a490 = {
                                let a478 = 1u64;
                                let a479 = 1u64;
                                nat_add(a478, a479)?
                            };
                            let a491 = {
                                let a488 = {
                                    let a486 = {
                                        let a480 = 1u64;
                                        let a481 = 1u64;
                                        nat_add(a480, a481)?
                                    };
                                    let a487 = {
                                        let a484 = {
                                            let a482 = 1u64;
                                            let a483 = 1u64;
                                            nat_add(a482, a483)?
                                        };
                                        let a485 = 1u64;
                                        nat_mul(a484, a485)?
                                    };
                                    nat_mul(a486, a487)?
                                };
                                let a489 = 1u64;
                                nat_add(a488, a489)?
                            };
                            nat_mul(a490, a491)?
                        };
                        let a493 = 1u64;
                        nat_add(a492, a493)?
                    };
                    nat_mul(a494, a495)?
                };
                let a497 = 1u64;
                nat_add(a496, a497)?
            })?;
            nat_mul(a498, a499)?
        };
        let a521 = f7({
            let a518 = {
                let a500 = 1u64;
                let a501 = 1u64;
                nat_add(a500, a501)?
            };
            let a519 = {
                let a516 = {
                    let a514 = {
                        let a502 = 1u64;
                        let a503 = 1u64;
                        nat_add(a502, a503)?
                    };
                    let a515 = {
                        let a512 = {
                            let a510 = {
                                let a504 = 1u64;
                                let a505 = 1u64;
                                nat_add(a504, a505)?
                            };
                            let a511 = {
                                let a508 = {
                                    let a506 = 1u64;
                                    let a507 = 1u64;
                                    nat_add(a506, a507)?
                                };
                                let a509 = 1u64;
                                nat_mul(a508, a509)?
                            };
                            nat_mul(a510, a511)?
                        };
                        let a513 = 1u64;
                        nat_add(a512, a513)?
                    };
                    nat_mul(a514, a515)?
                };
                let a517 = 1u64;
                nat_add(a516, a517)?
            };
            nat_mul(a518, a519)?
        })?;
        nat_add(a520, a521)
    } else {
        if if f5(v0)? {
            f5(v1)?
        } else {
            false
        } {
            let a522 = f18(v0)?;
            let a523 = f18(v1)?;
            if nat_eq(a522, a523) {
                Ok(v0)
            } else {
                let a610 = {
                    let a588 = {
                        let a564 = {
                            let a562 = {
                                let a524 = 1u64;
                                let a525 = 1u64;
                                nat_add(a524, a525)?
                            };
                            let a563 = {
                                let a560 = {
                                    let a558 = {
                                        let a526 = 1u64;
                                        let a527 = 1u64;
                                        nat_add(a526, a527)?
                                    };
                                    let a559 = {
                                        let a556 = {
                                            let a554 = {
                                                let a528 = 1u64;
                                                let a529 = 1u64;
                                                nat_add(a528, a529)?
                                            };
                                            let a555 = {
                                                let a552 = {
                                                    let a550 = {
                                                        let a530 = 1u64;
                                                        let a531 = 1u64;
                                                        nat_add(a530, a531)?
                                                    };
                                                    let a551 = {
                                                        let a548 = {
                                                            let a546 = {
                                                                let a532 = 1u64;
                                                                let a533 = 1u64;
                                                                nat_add(a532, a533)?
                                                            };
                                                            let a547 = {
                                                                let a544 = {
                                                                    let a542 = {
                                                                        let a534 = 1u64;
                                                                        let a535 = 1u64;
                                                                        nat_add(a534, a535)?
                                                                    };
                                                                    let a543 = {
                                                                        let a540 = {
                                                                            let a538 = {
                                                                                let a536 = 1u64;
                                                                                let a537 = 1u64;
                                                                                nat_add(a536, a537)?
                                                                            };
                                                                            let a539 = 1u64;
                                                                            nat_mul(a538, a539)?
                                                                        };
                                                                        let a541 = 1u64;
                                                                        nat_add(a540, a541)?
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
                                            };
                                            nat_mul(a554, a555)?
                                        };
                                        let a557 = 1u64;
                                        nat_add(a556, a557)?
                                    };
                                    nat_mul(a558, a559)?
                                };
                                let a561 = 1u64;
                                nat_add(a560, a561)?
                            };
                            nat_mul(a562, a563)?
                        };
                        let a565 = 1u64;
                        nat_add(a564, a565)?
                    };
                    let a589 = f7({
                        let a586 = {
                            let a584 = {
                                let a566 = 1u64;
                                let a567 = 1u64;
                                nat_add(a566, a567)?
                            };
                            let a585 = {
                                let a582 = {
                                    let a580 = {
                                        let a568 = 1u64;
                                        let a569 = 1u64;
                                        nat_add(a568, a569)?
                                    };
                                    let a581 = {
                                        let a578 = {
                                            let a576 = {
                                                let a570 = 1u64;
                                                let a571 = 1u64;
                                                nat_add(a570, a571)?
                                            };
                                            let a577 = {
                                                let a574 = {
                                                    let a572 = 1u64;
                                                    let a573 = 1u64;
                                                    nat_add(a572, a573)?
                                                };
                                                let a575 = 1u64;
                                                nat_mul(a574, a575)?
                                            };
                                            nat_mul(a576, a577)?
                                        };
                                        let a579 = 1u64;
                                        nat_add(a578, a579)?
                                    };
                                    nat_mul(a580, a581)?
                                };
                                let a583 = 1u64;
                                nat_add(a582, a583)?
                            };
                            nat_mul(a584, a585)?
                        };
                        let a587 = 1u64;
                        nat_add(a586, a587)?
                    })?;
                    nat_mul(a588, a589)?
                };
                let a611 = f7({
                    let a608 = {
                        let a590 = 1u64;
                        let a591 = 1u64;
                        nat_add(a590, a591)?
                    };
                    let a609 = {
                        let a606 = {
                            let a604 = {
                                let a592 = 1u64;
                                let a593 = 1u64;
                                nat_add(a592, a593)?
                            };
                            let a605 = {
                                let a602 = {
                                    let a600 = {
                                        let a594 = 1u64;
                                        let a595 = 1u64;
                                        nat_add(a594, a595)?
                                    };
                                    let a601 = {
                                        let a598 = {
                                            let a596 = 1u64;
                                            let a597 = 1u64;
                                            nat_add(a596, a597)?
                                        };
                                        let a599 = 1u64;
                                        nat_mul(a598, a599)?
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
                })?;
                nat_add(a610, a611)
            }
        } else {
            if f5(v0)? {
                Ok(v0)
            } else {
                if f5(v1)? {
                    Ok(v1)
                } else {
                    let a612 = f15(v1)?;
                    let a613 = f15(v0)?;
                    if nat_le(a612, a613) {
                        f22(v0, v1)
                    } else {
                        f22(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<u64> {
    let a636 = f15(v0)?;
    let a637 = f7({
        let a634 = {
            let a632 = {
                let a614 = 1u64;
                let a615 = 1u64;
                nat_add(a614, a615)?
            };
            let a633 = {
                let a630 = {
                    let a628 = {
                        let a616 = 1u64;
                        let a617 = 1u64;
                        nat_add(a616, a617)?
                    };
                    let a629 = {
                        let a626 = {
                            let a624 = {
                                let a618 = 1u64;
                                let a619 = 1u64;
                                nat_add(a618, a619)?
                            };
                            let a625 = {
                                let a622 = {
                                    let a620 = 1u64;
                                    let a621 = 1u64;
                                    nat_add(a620, a621)?
                                };
                                let a623 = 1u64;
                                nat_mul(a622, a623)?
                            };
                            nat_mul(a624, a625)?
                        };
                        let a627 = 1u64;
                        nat_add(a626, a627)?
                    };
                    nat_mul(a628, a629)?
                };
                let a631 = 1u64;
                nat_add(a630, a631)?
            };
            nat_mul(a632, a633)?
        };
        let a635 = 1u64;
        nat_add(a634, a635)?
    })?;
    let a638 = 0u64;
    Ok(nat_quot(a636, a637, a638))
}

pub fn f14(v0: u64) -> R<u64> {
    let a661 = v0;
    let a662 = f7({
        let a659 = {
            let a657 = {
                let a639 = 1u64;
                let a640 = 1u64;
                nat_add(a639, a640)?
            };
            let a658 = {
                let a655 = {
                    let a653 = {
                        let a641 = 1u64;
                        let a642 = 1u64;
                        nat_add(a641, a642)?
                    };
                    let a654 = {
                        let a651 = {
                            let a649 = {
                                let a643 = 1u64;
                                let a644 = 1u64;
                                nat_add(a643, a644)?
                            };
                            let a650 = {
                                let a647 = {
                                    let a645 = 1u64;
                                    let a646 = 1u64;
                                    nat_add(a645, a646)?
                                };
                                let a648 = 1u64;
                                nat_mul(a647, a648)?
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
    })?;
    let a663 = 0u64;
    Ok(nat_rem(a661, a662, a663))
}

pub fn f15(v0: u64) -> R<u64> {
    let a688 = v0;
    let a689 = f7({
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
    let a690 = 0u64;
    Ok(nat_rem(a688, a689, a690))
}

pub fn f16(v0: Fn0, p694: u64, p695: u64) -> R<u64> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v1, v2) = m693;
                let m691 = v1;
                if m691 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m691 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c692: Fn0 = v0.clone();
                        c692.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: u64) -> R<u64> {
    let a696 = v0;
    let a697 = v0;
    nat_add(a696, a697)
}

pub fn f18(v0: u64) -> R<u64> {
    let a722 = v0;
    let a723 = f7({
        let a720 = {
            let a718 = {
                let a698 = 1u64;
                let a699 = 1u64;
                nat_add(a698, a699)?
            };
            let a719 = {
                let a716 = {
                    let a714 = {
                        let a700 = 1u64;
                        let a701 = 1u64;
                        nat_add(a700, a701)?
                    };
                    let a715 = {
                        let a712 = {
                            let a710 = {
                                let a702 = 1u64;
                                let a703 = 1u64;
                                nat_add(a702, a703)?
                            };
                            let a711 = {
                                let a708 = {
                                    let a706 = {
                                        let a704 = 1u64;
                                        let a705 = 1u64;
                                        nat_add(a704, a705)?
                                    };
                                    let a707 = 1u64;
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
                    nat_mul(a714, a715)?
                };
                let a717 = 1u64;
                nat_add(a716, a717)?
            };
            nat_mul(a718, a719)?
        };
        let a721 = 1u64;
        nat_add(a720, a721)?
    })?;
    let a724 = 0u64;
    Ok(nat_quot(a722, a723, a724))
}

pub fn f19(v0: u64) -> R<Adt1> {
    f23(f24(v0)?, f25(v0)?, 0u64, f7({
        let a745 = {
            let a743 = {
                let a725 = 1u64;
                let a726 = 1u64;
                nat_add(a725, a726)?
            };
            let a744 = {
                let a741 = {
                    let a739 = {
                        let a727 = 1u64;
                        let a728 = 1u64;
                        nat_add(a727, a728)?
                    };
                    let a740 = {
                        let a737 = {
                            let a735 = {
                                let a729 = 1u64;
                                let a730 = 1u64;
                                nat_add(a729, a730)?
                            };
                            let a736 = {
                                let a733 = {
                                    let a731 = 1u64;
                                    let a732 = 1u64;
                                    nat_add(a731, a732)?
                                };
                                let a734 = 1u64;
                                nat_mul(a733, a734)?
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
    })?)
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a769 = f7({
        let a767 = {
            let a765 = {
                let a747 = 1u64;
                let a748 = 1u64;
                nat_add(a747, a748)?
            };
            let a766 = {
                let a763 = {
                    let a761 = {
                        let a749 = 1u64;
                        let a750 = 1u64;
                        nat_add(a749, a750)?
                    };
                    let a762 = {
                        let a759 = {
                            let a751 = 1u64;
                            let a752 = 1u64;
                            nat_add(a751, a752)?
                        };
                        let a760 = {
                            let a757 = {
                                let a755 = {
                                    let a753 = 1u64;
                                    let a754 = 1u64;
                                    nat_add(a753, a754)?
                                };
                                let a756 = 1u64;
                                nat_mul(a755, a756)?
                            };
                            let a758 = 1u64;
                            nat_add(a757, a758)?
                        };
                        nat_mul(a759, a760)?
                    };
                    nat_mul(a761, a762)?
                };
                let a764 = 1u64;
                nat_add(a763, a764)?
            };
            nat_mul(a765, a766)?
        };
        let a768 = 1u64;
        nat_add(a767, a768)?
    })?;
    let a770 = v2;
    if nat_le(a769, a770) {
        f26(v0, {
            let a771 = v1;
            let a772 = 1u64;
            nat_add(a771, a772)?
        }, f21(v2, 1u64)?)
    } else {
        let v3: Adt1 = f23(v1, v2, {
            let a809 = {
                let a807 = {
                    let a773 = 1u64;
                    let a774 = 1u64;
                    nat_add(a773, a774)?
                };
                let a808 = {
                    let a805 = {
                        let a775 = 1u64;
                        let a776 = 1u64;
                        nat_add(a775, a776)?
                    };
                    let a806 = {
                        let a803 = {
                            let a777 = 1u64;
                            let a778 = 1u64;
                            nat_add(a777, a778)?
                        };
                        let a804 = {
                            let a801 = {
                                let a779 = 1u64;
                                let a780 = 1u64;
                                nat_add(a779, a780)?
                            };
                            let a802 = {
                                let a799 = {
                                    let a781 = 1u64;
                                    let a782 = 1u64;
                                    nat_add(a781, a782)?
                                };
                                let a800 = {
                                    let a797 = {
                                        let a783 = 1u64;
                                        let a784 = 1u64;
                                        nat_add(a783, a784)?
                                    };
                                    let a798 = {
                                        let a795 = {
                                            let a785 = 1u64;
                                            let a786 = 1u64;
                                            nat_add(a785, a786)?
                                        };
                                        let a796 = {
                                            let a793 = {
                                                let a787 = 1u64;
                                                let a788 = 1u64;
                                                nat_add(a787, a788)?
                                            };
                                            let a794 = {
                                                let a791 = {
                                                    let a789 = 1u64;
                                                    let a790 = 1u64;
                                                    nat_add(a789, a790)?
                                                };
                                                let a792 = 1u64;
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
                    nat_mul(a805, a806)?
                };
                nat_mul(a807, a808)?
            };
            let a810 = 1u64;
            nat_add(a809, a810)?
        }, f7({
            let a829 = {
                let a811 = 1u64;
                let a812 = 1u64;
                nat_add(a811, a812)?
            };
            let a830 = {
                let a827 = {
                    let a825 = {
                        let a813 = 1u64;
                        let a814 = 1u64;
                        nat_add(a813, a814)?
                    };
                    let a826 = {
                        let a823 = {
                            let a815 = 1u64;
                            let a816 = 1u64;
                            nat_add(a815, a816)?
                        };
                        let a824 = {
                            let a821 = {
                                let a819 = {
                                    let a817 = 1u64;
                                    let a818 = 1u64;
                                    nat_add(a817, a818)?
                                };
                                let a820 = 1u64;
                                nat_mul(a819, a820)?
                            };
                            let a822 = 1u64;
                            nat_add(a821, a822)?
                        };
                        nat_mul(a823, a824)?
                    };
                    nat_mul(a825, a826)?
                };
                let a828 = 1u64;
                nat_add(a827, a828)?
            };
            nat_mul(a829, a830)?
        })?)?;
        f26(v0, match v3.clone() {
            Adt1::C0(h831, _, _, _) => {
                h831
            }
        }, match v3 {
            Adt1::C0(_, h832, _, _) => {
                h832
            }
        })
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a833 = v1;
    let a834 = 0u64;
    if nat_eq(a833, a834) {
        Ok(v0)
    } else {
        let a859 = {
            let a857 = {
                let a835 = 1u64;
                let a836 = 1u64;
                nat_add(a835, a836)?
            };
            let a858 = {
                let a855 = {
                    let a837 = 1u64;
                    let a838 = 1u64;
                    nat_add(a837, a838)?
                };
                let a856 = {
                    let a853 = {
                        let a839 = 1u64;
                        let a840 = 1u64;
                        nat_add(a839, a840)?
                    };
                    let a854 = {
                        let a851 = {
                            let a841 = 1u64;
                            let a842 = 1u64;
                            nat_add(a841, a842)?
                        };
                        let a852 = {
                            let a849 = {
                                let a843 = 1u64;
                                let a844 = 1u64;
                                nat_add(a843, a844)?
                            };
                            let a850 = {
                                let a847 = {
                                    let a845 = 1u64;
                                    let a846 = 1u64;
                                    nat_add(a845, a846)?
                                };
                                let a848 = 1u64;
                                nat_mul(a847, a848)?
                            };
                            nat_mul(a849, a850)?
                        };
                        nat_mul(a851, a852)?
                    };
                    nat_mul(a853, a854)?
                };
                nat_mul(a855, a856)?
            };
            nat_mul(a857, a858)?
        };
        let a860 = v1;
        if nat_le(a859, a860) {
            let a861 = v0;
            let a862 = 0u64;
            if nat_eq(a861, a862) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f7(v1)?;
            f27({
                let a863 = v0;
                let a864 = v2;
                let a865 = 0u64;
                nat_quot(a863, a864, a865)
            }, {
                let a869 = 0u64;
                let a870 = {
                    let a866 = v0;
                    let a867 = v2;
                    let a868 = 0u64;
                    nat_rem(a866, a867, a868)
                };
                nat_lt(a869, a870)
            })
        }
    }
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f24(v0)?;
    let v3: u64 = {
        let a883 = f25(v0)?;
        let a884 = {
            let a881 = {
                let a871 = 1u64;
                let a872 = 1u64;
                nat_add(a871, a872)?
            };
            let a882 = {
                let a879 = {
                    let a873 = 1u64;
                    let a874 = 1u64;
                    nat_add(a873, a874)?
                };
                let a880 = {
                    let a877 = {
                        let a875 = 1u64;
                        let a876 = 1u64;
                        nat_add(a875, a876)?
                    };
                    let a878 = 1u64;
                    nat_mul(a877, a878)?
                };
                nat_mul(a879, a880)?
            };
            nat_mul(a881, a882)?
        };
        nat_mul(a883, a884)?
    };
    let v4: u64 = f21({
        let a897 = f25(v1)?;
        let a898 = {
            let a895 = {
                let a885 = 1u64;
                let a886 = 1u64;
                nat_add(a885, a886)?
            };
            let a896 = {
                let a893 = {
                    let a887 = 1u64;
                    let a888 = 1u64;
                    nat_add(a887, a888)?
                };
                let a894 = {
                    let a891 = {
                        let a889 = 1u64;
                        let a890 = 1u64;
                        nat_add(a889, a890)?
                    };
                    let a892 = 1u64;
                    nat_mul(a891, a892)?
                };
                nat_mul(a893, a894)?
            };
            nat_mul(a895, a896)?
        };
        nat_mul(a897, a898)?
    }, {
        let a899 = v2;
        let a900 = f24(v1)?;
        nat_sub(a899, a900)
    })?;
    let a901 = f18(v0)?;
    let a902 = f18(v1)?;
    if nat_eq(a901, a902) {
        f20(f18(v0)?, v2, {
            let a903 = v3;
            let a904 = v4;
            nat_add(a903, a904)?
        })
    } else {
        let a905 = v3;
        let a906 = v4;
        if nat_eq(a905, a906) {
            Ok(0u64)
        } else {
            f20(f18(v0)?, v2, {
                let a907 = v3;
                let a908 = v4;
                nat_sub(a907, a908)
            })
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h909, _) = f28(Fn1::F29, {
        let a928 = {
            let a910 = 1u64;
            let a911 = 1u64;
            nat_add(a910, a911)?
        };
        let a929 = {
            let a926 = {
                let a912 = 1u64;
                let a913 = 1u64;
                nat_add(a912, a913)?
            };
            let a927 = {
                let a924 = {
                    let a914 = 1u64;
                    let a915 = 1u64;
                    nat_add(a914, a915)?
                };
                let a925 = {
                    let a922 = {
                        let a916 = 1u64;
                        let a917 = 1u64;
                        nat_add(a916, a917)?
                    };
                    let a923 = {
                        let a920 = {
                            let a918 = 1u64;
                            let a919 = 1u64;
                            nat_add(a918, a919)?
                        };
                        let a921 = 1u64;
                        nat_mul(a920, a921)?
                    };
                    nat_mul(a922, a923)?
                };
                nat_mul(a924, a925)?
            };
            nat_mul(a926, a927)?
        };
        nat_mul(a928, a929)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h909)
}

pub fn f24(v0: u64) -> R<u64> {
    let a968 = {
        let a930 = f13(v0)?;
        let a931 = 0u64;
        if nat_eq(a930, a931) {
            1u64
        } else {
            f13(v0)?
        }
    };
    let a969 = {
        let a966 = {
            let a932 = 1u64;
            let a933 = 1u64;
            nat_add(a932, a933)?
        };
        let a967 = {
            let a964 = {
                let a934 = 1u64;
                let a935 = 1u64;
                nat_add(a934, a935)?
            };
            let a965 = {
                let a962 = {
                    let a936 = 1u64;
                    let a937 = 1u64;
                    nat_add(a936, a937)?
                };
                let a963 = {
                    let a960 = {
                        let a938 = 1u64;
                        let a939 = 1u64;
                        nat_add(a938, a939)?
                    };
                    let a961 = {
                        let a958 = {
                            let a940 = 1u64;
                            let a941 = 1u64;
                            nat_add(a940, a941)?
                        };
                        let a959 = {
                            let a956 = {
                                let a942 = 1u64;
                                let a943 = 1u64;
                                nat_add(a942, a943)?
                            };
                            let a957 = {
                                let a954 = {
                                    let a944 = 1u64;
                                    let a945 = 1u64;
                                    nat_add(a944, a945)?
                                };
                                let a955 = {
                                    let a952 = {
                                        let a946 = 1u64;
                                        let a947 = 1u64;
                                        nat_add(a946, a947)?
                                    };
                                    let a953 = {
                                        let a950 = {
                                            let a948 = 1u64;
                                            let a949 = 1u64;
                                            nat_add(a948, a949)?
                                        };
                                        let a951 = 1u64;
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
        nat_mul(a966, a967)?
    };
    nat_add(a968, a969)
}

pub fn f25(v0: u64) -> R<u64> {
    let a994 = f14(v0)?;
    let a995 = {
        let a970 = f13(v0)?;
        let a971 = 0u64;
        if nat_eq(a970, a971) {
            0u64
        } else {
            f7({
                let a992 = {
                    let a990 = {
                        let a972 = 1u64;
                        let a973 = 1u64;
                        nat_add(a972, a973)?
                    };
                    let a991 = {
                        let a988 = {
                            let a986 = {
                                let a974 = 1u64;
                                let a975 = 1u64;
                                nat_add(a974, a975)?
                            };
                            let a987 = {
                                let a984 = {
                                    let a982 = {
                                        let a976 = 1u64;
                                        let a977 = 1u64;
                                        nat_add(a976, a977)?
                                    };
                                    let a983 = {
                                        let a980 = {
                                            let a978 = 1u64;
                                            let a979 = 1u64;
                                            nat_add(a978, a979)?
                                        };
                                        let a981 = 1u64;
                                        nat_mul(a980, a981)?
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
                    nat_mul(a990, a991)?
                };
                let a993 = 1u64;
                nat_add(a992, a993)?
            })?
        }
    };
    nat_add(a994, a995)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1034 = v1;
    let a1035 = {
        let a1032 = {
            let a1030 = {
                let a996 = 1u64;
                let a997 = 1u64;
                nat_add(a996, a997)?
            };
            let a1031 = {
                let a1028 = {
                    let a998 = 1u64;
                    let a999 = 1u64;
                    nat_add(a998, a999)?
                };
                let a1029 = {
                    let a1026 = {
                        let a1000 = 1u64;
                        let a1001 = 1u64;
                        nat_add(a1000, a1001)?
                    };
                    let a1027 = {
                        let a1024 = {
                            let a1002 = 1u64;
                            let a1003 = 1u64;
                            nat_add(a1002, a1003)?
                        };
                        let a1025 = {
                            let a1022 = {
                                let a1004 = 1u64;
                                let a1005 = 1u64;
                                nat_add(a1004, a1005)?
                            };
                            let a1023 = {
                                let a1020 = {
                                    let a1006 = 1u64;
                                    let a1007 = 1u64;
                                    nat_add(a1006, a1007)?
                                };
                                let a1021 = {
                                    let a1018 = {
                                        let a1008 = 1u64;
                                        let a1009 = 1u64;
                                        nat_add(a1008, a1009)?
                                    };
                                    let a1019 = {
                                        let a1016 = {
                                            let a1010 = 1u64;
                                            let a1011 = 1u64;
                                            nat_add(a1010, a1011)?
                                        };
                                        let a1017 = {
                                            let a1014 = {
                                                let a1012 = 1u64;
                                                let a1013 = 1u64;
                                                nat_add(a1012, a1013)?
                                            };
                                            let a1015 = 1u64;
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
            nat_mul(a1030, a1031)?
        };
        let a1033 = 1u64;
        nat_add(a1032, a1033)?
    };
    if nat_lt(a1034, a1035) {
        f30(v0, {
            let a1072 = {
                let a1070 = {
                    let a1036 = 1u64;
                    let a1037 = 1u64;
                    nat_add(a1036, a1037)?
                };
                let a1071 = {
                    let a1068 = {
                        let a1038 = 1u64;
                        let a1039 = 1u64;
                        nat_add(a1038, a1039)?
                    };
                    let a1069 = {
                        let a1066 = {
                            let a1040 = 1u64;
                            let a1041 = 1u64;
                            nat_add(a1040, a1041)?
                        };
                        let a1067 = {
                            let a1064 = {
                                let a1042 = 1u64;
                                let a1043 = 1u64;
                                nat_add(a1042, a1043)?
                            };
                            let a1065 = {
                                let a1062 = {
                                    let a1044 = 1u64;
                                    let a1045 = 1u64;
                                    nat_add(a1044, a1045)?
                                };
                                let a1063 = {
                                    let a1060 = {
                                        let a1046 = 1u64;
                                        let a1047 = 1u64;
                                        nat_add(a1046, a1047)?
                                    };
                                    let a1061 = {
                                        let a1058 = {
                                            let a1048 = 1u64;
                                            let a1049 = 1u64;
                                            nat_add(a1048, a1049)?
                                        };
                                        let a1059 = {
                                            let a1056 = {
                                                let a1050 = 1u64;
                                                let a1051 = 1u64;
                                                nat_add(a1050, a1051)?
                                            };
                                            let a1057 = {
                                                let a1054 = {
                                                    let a1052 = 1u64;
                                                    let a1053 = 1u64;
                                                    nat_add(a1052, a1053)?
                                                };
                                                let a1055 = 1u64;
                                                nat_mul(a1054, a1055)?
                                            };
                                            nat_mul(a1056, a1057)?
                                        };
                                        nat_mul(a1058, a1059)?
                                    };
                                    nat_mul(a1060, a1061)?
                                };
                                nat_mul(a1062, a1063)?
                            };
                            nat_mul(a1064, a1065)?
                        };
                        nat_mul(a1066, a1067)?
                    };
                    nat_mul(a1068, a1069)?
                };
                nat_mul(a1070, a1071)?
            };
            let a1073 = 1u64;
            nat_add(a1072, a1073)?
        }, f21(v2, {
            let a1112 = {
                let a1110 = {
                    let a1108 = {
                        let a1074 = 1u64;
                        let a1075 = 1u64;
                        nat_add(a1074, a1075)?
                    };
                    let a1109 = {
                        let a1106 = {
                            let a1076 = 1u64;
                            let a1077 = 1u64;
                            nat_add(a1076, a1077)?
                        };
                        let a1107 = {
                            let a1104 = {
                                let a1078 = 1u64;
                                let a1079 = 1u64;
                                nat_add(a1078, a1079)?
                            };
                            let a1105 = {
                                let a1102 = {
                                    let a1080 = 1u64;
                                    let a1081 = 1u64;
                                    nat_add(a1080, a1081)?
                                };
                                let a1103 = {
                                    let a1100 = {
                                        let a1082 = 1u64;
                                        let a1083 = 1u64;
                                        nat_add(a1082, a1083)?
                                    };
                                    let a1101 = {
                                        let a1098 = {
                                            let a1084 = 1u64;
                                            let a1085 = 1u64;
                                            nat_add(a1084, a1085)?
                                        };
                                        let a1099 = {
                                            let a1096 = {
                                                let a1086 = 1u64;
                                                let a1087 = 1u64;
                                                nat_add(a1086, a1087)?
                                            };
                                            let a1097 = {
                                                let a1094 = {
                                                    let a1088 = 1u64;
                                                    let a1089 = 1u64;
                                                    nat_add(a1088, a1089)?
                                                };
                                                let a1095 = {
                                                    let a1092 = {
                                                        let a1090 = 1u64;
                                                        let a1091 = 1u64;
                                                        nat_add(a1090, a1091)?
                                                    };
                                                    let a1093 = 1u64;
                                                    nat_mul(a1092, a1093)?
                                                };
                                                nat_mul(a1094, a1095)?
                                            };
                                            nat_mul(a1096, a1097)?
                                        };
                                        nat_mul(a1098, a1099)?
                                    };
                                    nat_mul(a1100, a1101)?
                                };
                                nat_mul(a1102, a1103)?
                            };
                            nat_mul(a1104, a1105)?
                        };
                        nat_mul(a1106, a1107)?
                    };
                    nat_mul(a1108, a1109)?
                };
                let a1111 = 1u64;
                nat_add(a1110, a1111)?
            };
            let a1113 = v1;
            nat_sub(a1112, a1113)
        })?)
    } else {
        f30(v0, v1, v2)
    }
}

pub fn f27(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1121 = {
            let a1118 = v0;
            let a1119 = {
                let a1116 = {
                    let a1114 = 1u64;
                    let a1115 = 1u64;
                    nat_add(a1114, a1115)?
                };
                let a1117 = 1u64;
                nat_mul(a1116, a1117)?
            };
            let a1120 = 0u64;
            nat_rem(a1118, a1119, a1120)
        };
        let a1122 = 0u64;
        nat_eq(a1121, a1122)
    } else {
        false
    } {
        let a1123 = v0;
        let a1124 = 1u64;
        nat_add(a1123, a1124)
    } else {
        Ok(v0)
    }
}

pub fn f28(v0: Fn1, p1129: u64, p1130: Adt1) -> R<(Adt1, bool)> {
    let m1128 = (p1129, p1130);
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v1, v2) = m1128;
                let m1125 = v1;
                if m1125 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1125 - 1;
                    let m1127 = {
                        let c1126: Fn1 = v0.clone();
                        c1126.apply(v2.clone())?
                    };
                    match m1127 {
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
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    f31(v0)
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1143 = v2;
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
        let a1145 = 0u64;
        nat_rem(a1143, a1144, a1145)
    };
    let v4: u64 = {
        let a1158 = v2;
        let a1159 = {
            let a1156 = {
                let a1146 = 1u64;
                let a1147 = 1u64;
                nat_add(a1146, a1147)?
            };
            let a1157 = {
                let a1154 = {
                    let a1148 = 1u64;
                    let a1149 = 1u64;
                    nat_add(a1148, a1149)?
                };
                let a1155 = {
                    let a1152 = {
                        let a1150 = 1u64;
                        let a1151 = 1u64;
                        nat_add(a1150, a1151)?
                    };
                    let a1153 = 1u64;
                    nat_mul(a1152, a1153)?
                };
                nat_mul(a1154, a1155)?
            };
            nat_mul(a1156, a1157)?
        };
        let a1160 = 0u64;
        nat_quot(a1158, a1159, a1160)
    };
    let v5: bool = {
        let a1169 = {
            let a1167 = {
                let a1161 = 1u64;
                let a1162 = 1u64;
                nat_add(a1161, a1162)?
            };
            let a1168 = {
                let a1165 = {
                    let a1163 = 1u64;
                    let a1164 = 1u64;
                    nat_add(a1163, a1164)?
                };
                let a1166 = 1u64;
                nat_mul(a1165, a1166)?
            };
            nat_mul(a1167, a1168)?
        };
        let a1170 = v3;
        if nat_lt(a1169, a1170) {
            true
        } else {
            let a1179 = v3;
            let a1180 = {
                let a1177 = {
                    let a1171 = 1u64;
                    let a1172 = 1u64;
                    nat_add(a1171, a1172)?
                };
                let a1178 = {
                    let a1175 = {
                        let a1173 = 1u64;
                        let a1174 = 1u64;
                        nat_add(a1173, a1174)?
                    };
                    let a1176 = 1u64;
                    nat_mul(a1175, a1176)?
                };
                nat_mul(a1177, a1178)?
            };
            if nat_eq(a1179, a1180) {
                let a1188 = {
                    let a1185 = v4;
                    let a1186 = {
                        let a1183 = {
                            let a1181 = 1u64;
                            let a1182 = 1u64;
                            nat_add(a1181, a1182)?
                        };
                        let a1184 = 1u64;
                        nat_mul(a1183, a1184)?
                    };
                    let a1187 = 0u64;
                    nat_rem(a1185, a1186, a1187)
                };
                let a1189 = 1u64;
                nat_eq(a1188, a1189)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1190 = v4;
        let a1191 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1190, a1191)?
    };
    let v7: bool = {
        let a1220 = {
            let a1218 = {
                let a1194 = {
                    let a1192 = 1u64;
                    let a1193 = 1u64;
                    nat_add(a1192, a1193)?
                };
                let a1195 = 1u64;
                nat_mul(a1194, a1195)?
            };
            let a1219 = f7({
                let a1216 = {
                    let a1214 = {
                        let a1196 = 1u64;
                        let a1197 = 1u64;
                        nat_add(a1196, a1197)?
                    };
                    let a1215 = {
                        let a1212 = {
                            let a1210 = {
                                let a1198 = 1u64;
                                let a1199 = 1u64;
                                nat_add(a1198, a1199)?
                            };
                            let a1211 = {
                                let a1208 = {
                                    let a1206 = {
                                        let a1200 = 1u64;
                                        let a1201 = 1u64;
                                        nat_add(a1200, a1201)?
                                    };
                                    let a1207 = {
                                        let a1204 = {
                                            let a1202 = 1u64;
                                            let a1203 = 1u64;
                                            nat_add(a1202, a1203)?
                                        };
                                        let a1205 = 1u64;
                                        nat_mul(a1204, a1205)?
                                    };
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
            })?;
            nat_mul(a1218, a1219)?
        };
        let a1221 = v6;
        nat_le(a1220, a1221)
    };
    let v8: u64 = if v7 {
        let a1226 = v6;
        let a1227 = {
            let a1224 = {
                let a1222 = 1u64;
                let a1223 = 1u64;
                nat_add(a1222, a1223)?
            };
            let a1225 = 1u64;
            nat_mul(a1224, a1225)?
        };
        let a1228 = 0u64;
        nat_quot(a1226, a1227, a1228)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1229 = v1;
        let a1230 = 1u64;
        nat_add(a1229, a1230)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1269 = v9;
        let a1270 = {
            let a1267 = {
                let a1265 = {
                    let a1231 = 1u64;
                    let a1232 = 1u64;
                    nat_add(a1231, a1232)?
                };
                let a1266 = {
                    let a1263 = {
                        let a1233 = 1u64;
                        let a1234 = 1u64;
                        nat_add(a1233, a1234)?
                    };
                    let a1264 = {
                        let a1261 = {
                            let a1235 = 1u64;
                            let a1236 = 1u64;
                            nat_add(a1235, a1236)?
                        };
                        let a1262 = {
                            let a1259 = {
                                let a1237 = 1u64;
                                let a1238 = 1u64;
                                nat_add(a1237, a1238)?
                            };
                            let a1260 = {
                                let a1257 = {
                                    let a1239 = 1u64;
                                    let a1240 = 1u64;
                                    nat_add(a1239, a1240)?
                                };
                                let a1258 = {
                                    let a1255 = {
                                        let a1241 = 1u64;
                                        let a1242 = 1u64;
                                        nat_add(a1241, a1242)?
                                    };
                                    let a1256 = {
                                        let a1253 = {
                                            let a1243 = 1u64;
                                            let a1244 = 1u64;
                                            nat_add(a1243, a1244)?
                                        };
                                        let a1254 = {
                                            let a1251 = {
                                                let a1245 = 1u64;
                                                let a1246 = 1u64;
                                                nat_add(a1245, a1246)?
                                            };
                                            let a1252 = {
                                                let a1249 = {
                                                    let a1247 = 1u64;
                                                    let a1248 = 1u64;
                                                    nat_add(a1247, a1248)?
                                                };
                                                let a1250 = 1u64;
                                                nat_mul(a1249, a1250)?
                                            };
                                            nat_mul(a1251, a1252)?
                                        };
                                        nat_mul(a1253, a1254)?
                                    };
                                    nat_mul(a1255, a1256)?
                                };
                                nat_mul(a1257, a1258)?
                            };
                            nat_mul(a1259, a1260)?
                        };
                        nat_mul(a1261, a1262)?
                    };
                    nat_mul(a1263, a1264)?
                };
                nat_mul(a1265, a1266)?
            };
            let a1268 = 1u64;
            nat_add(a1267, a1268)?
        };
        if if nat_eq(a1269, a1270) {
            let a1293 = v8;
            let a1294 = f7({
                let a1291 = {
                    let a1289 = {
                        let a1271 = 1u64;
                        let a1272 = 1u64;
                        nat_add(a1271, a1272)?
                    };
                    let a1290 = {
                        let a1287 = {
                            let a1285 = {
                                let a1273 = 1u64;
                                let a1274 = 1u64;
                                nat_add(a1273, a1274)?
                            };
                            let a1286 = {
                                let a1283 = {
                                    let a1281 = {
                                        let a1275 = 1u64;
                                        let a1276 = 1u64;
                                        nat_add(a1275, a1276)?
                                    };
                                    let a1282 = {
                                        let a1279 = {
                                            let a1277 = 1u64;
                                            let a1278 = 1u64;
                                            nat_add(a1277, a1278)?
                                        };
                                        let a1280 = 1u64;
                                        nat_mul(a1279, a1280)?
                                    };
                                    nat_mul(a1281, a1282)?
                                };
                                let a1284 = 1u64;
                                nat_add(a1283, a1284)?
                            };
                            nat_mul(a1285, a1286)?
                        };
                        let a1288 = 1u64;
                        nat_add(a1287, a1288)?
                    };
                    nat_mul(a1289, a1290)?
                };
                let a1292 = 1u64;
                nat_add(a1291, a1292)?
            })?;
            nat_lt(a1293, a1294)
        } else {
            false
        } {
            0u64
        } else {
            let a1331 = v9;
            let a1332 = {
                let a1329 = {
                    let a1295 = 1u64;
                    let a1296 = 1u64;
                    nat_add(a1295, a1296)?
                };
                let a1330 = {
                    let a1327 = {
                        let a1297 = 1u64;
                        let a1298 = 1u64;
                        nat_add(a1297, a1298)?
                    };
                    let a1328 = {
                        let a1325 = {
                            let a1299 = 1u64;
                            let a1300 = 1u64;
                            nat_add(a1299, a1300)?
                        };
                        let a1326 = {
                            let a1323 = {
                                let a1301 = 1u64;
                                let a1302 = 1u64;
                                nat_add(a1301, a1302)?
                            };
                            let a1324 = {
                                let a1321 = {
                                    let a1303 = 1u64;
                                    let a1304 = 1u64;
                                    nat_add(a1303, a1304)?
                                };
                                let a1322 = {
                                    let a1319 = {
                                        let a1305 = 1u64;
                                        let a1306 = 1u64;
                                        nat_add(a1305, a1306)?
                                    };
                                    let a1320 = {
                                        let a1317 = {
                                            let a1307 = 1u64;
                                            let a1308 = 1u64;
                                            nat_add(a1307, a1308)?
                                        };
                                        let a1318 = {
                                            let a1315 = {
                                                let a1309 = 1u64;
                                                let a1310 = 1u64;
                                                nat_add(a1309, a1310)?
                                            };
                                            let a1316 = {
                                                let a1313 = {
                                                    let a1311 = 1u64;
                                                    let a1312 = 1u64;
                                                    nat_add(a1311, a1312)?
                                                };
                                                let a1314 = 1u64;
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
                    nat_mul(a1327, a1328)?
                };
                nat_mul(a1329, a1330)?
            };
            nat_sub(a1331, a1332)
        }
    };
    let a1333 = v8;
    let a1334 = 0u64;
    if nat_eq(a1333, a1334) {
        f10(v0)
    } else {
        let a1387 = {
            let a1385 = {
                let a1383 = {
                    let a1335 = 1u64;
                    let a1336 = 1u64;
                    nat_add(a1335, a1336)?
                };
                let a1384 = {
                    let a1381 = {
                        let a1379 = {
                            let a1337 = 1u64;
                            let a1338 = 1u64;
                            nat_add(a1337, a1338)?
                        };
                        let a1380 = {
                            let a1377 = {
                                let a1375 = {
                                    let a1339 = 1u64;
                                    let a1340 = 1u64;
                                    nat_add(a1339, a1340)?
                                };
                                let a1376 = {
                                    let a1373 = {
                                        let a1371 = {
                                            let a1341 = 1u64;
                                            let a1342 = 1u64;
                                            nat_add(a1341, a1342)?
                                        };
                                        let a1372 = {
                                            let a1369 = {
                                                let a1367 = {
                                                    let a1343 = 1u64;
                                                    let a1344 = 1u64;
                                                    nat_add(a1343, a1344)?
                                                };
                                                let a1368 = {
                                                    let a1365 = {
                                                        let a1363 = {
                                                            let a1345 = 1u64;
                                                            let a1346 = 1u64;
                                                            nat_add(a1345, a1346)?
                                                        };
                                                        let a1364 = {
                                                            let a1361 = {
                                                                let a1359 = {
                                                                    let a1347 = 1u64;
                                                                    let a1348 = 1u64;
                                                                    nat_add(a1347, a1348)?
                                                                };
                                                                let a1360 = {
                                                                    let a1357 = {
                                                                        let a1355 = {
                                                                            let a1349 = 1u64;
                                                                            let a1350 = 1u64;
                                                                            nat_add(a1349, a1350)?
                                                                        };
                                                                        let a1356 = {
                                                                            let a1353 = {
                                                                                let a1351 = 1u64;
                                                                                let a1352 = 1u64;
                                                                                nat_add(a1351, a1352)?
                                                                            };
                                                                            let a1354 = 1u64;
                                                                            nat_mul(a1353, a1354)?
                                                                        };
                                                                        nat_mul(a1355, a1356)?
                                                                    };
                                                                    let a1358 = 1u64;
                                                                    nat_add(a1357, a1358)?
                                                                };
                                                                nat_mul(a1359, a1360)?
                                                            };
                                                            let a1362 = 1u64;
                                                            nat_add(a1361, a1362)?
                                                        };
                                                        nat_mul(a1363, a1364)?
                                                    };
                                                    let a1366 = 1u64;
                                                    nat_add(a1365, a1366)?
                                                };
                                                nat_mul(a1367, a1368)?
                                            };
                                            let a1370 = 1u64;
                                            nat_add(a1369, a1370)?
                                        };
                                        nat_mul(a1371, a1372)?
                                    };
                                    let a1374 = 1u64;
                                    nat_add(a1373, a1374)?
                                };
                                nat_mul(a1375, a1376)?
                            };
                            let a1378 = 1u64;
                            nat_add(a1377, a1378)?
                        };
                        nat_mul(a1379, a1380)?
                    };
                    let a1382 = 1u64;
                    nat_add(a1381, a1382)?
                };
                nat_mul(a1383, a1384)?
            };
            let a1386 = 1u64;
            nat_add(a1385, a1386)?
        };
        let a1388 = v9;
        if nat_le(a1387, a1388) {
            f8(v0)
        } else {
            let a1440 = {
                let a1413 = f10(v0)?;
                let a1414 = {
                    let a1411 = v10;
                    let a1412 = f7({
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
                nat_add(a1413, a1414)?
            };
            let a1441 = {
                let a1437 = v8;
                let a1438 = f7({
                    let a1435 = {
                        let a1433 = {
                            let a1415 = 1u64;
                            let a1416 = 1u64;
                            nat_add(a1415, a1416)?
                        };
                        let a1434 = {
                            let a1431 = {
                                let a1429 = {
                                    let a1417 = 1u64;
                                    let a1418 = 1u64;
                                    nat_add(a1417, a1418)?
                                };
                                let a1430 = {
                                    let a1427 = {
                                        let a1425 = {
                                            let a1419 = 1u64;
                                            let a1420 = 1u64;
                                            nat_add(a1419, a1420)?
                                        };
                                        let a1426 = {
                                            let a1423 = {
                                                let a1421 = 1u64;
                                                let a1422 = 1u64;
                                                nat_add(a1421, a1422)?
                                            };
                                            let a1424 = 1u64;
                                            nat_mul(a1423, a1424)?
                                        };
                                        nat_mul(a1425, a1426)?
                                    };
                                    let a1428 = 1u64;
                                    nat_add(a1427, a1428)?
                                };
                                nat_mul(a1429, a1430)?
                            };
                            let a1432 = 1u64;
                            nat_add(a1431, a1432)?
                        };
                        nat_mul(a1433, a1434)?
                    };
                    let a1436 = 1u64;
                    nat_add(a1435, a1436)?
                })?;
                let a1439 = 0u64;
                nat_rem(a1437, a1438, a1439)
            };
            nat_add(a1440, a1441)
        }
    }
}

pub fn f31(v0: Adt1) -> R<Option<Adt1>> {
    let a1443 = 0u64;
    let a1444 = match v0.clone() {
        Adt1::C0(_, h1442, _, _) => {
            h1442
        }
    };
    if if if nat_lt(a1443, a1444) {
        let a1447 = match v0.clone() {
            Adt1::C0(_, h1445, _, _) => {
                h1445
            }
        };
        let a1448 = match v0.clone() {
            Adt1::C0(_, _, _, h1446) => {
                h1446
            }
        };
        nat_lt(a1447, a1448)
    } else {
        false
    } {
        let a1451 = match v0.clone() {
            Adt1::C0(_, _, h1449, _) => {
                h1449
            }
        };
        let a1452 = match v0.clone() {
            Adt1::C0(h1450, _, _, _) => {
                h1450
            }
        };
        nat_lt(a1451, a1452)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1454 = match v0.clone() {
                Adt1::C0(h1453, _, _, _) => {
                    h1453
                }
            };
            let a1455 = 1u64;
            nat_sub(a1454, a1455)
        }, {
            let a1458 = match v0.clone() {
                Adt1::C0(_, h1456, _, _) => {
                    h1456
                }
            };
            let a1459 = match v0.clone() {
                Adt1::C0(_, h1457, _, _) => {
                    h1457
                }
            };
            nat_add(a1458, a1459)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1460, _) => {
                h1460
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1461) => {
                h1461
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3)
}
