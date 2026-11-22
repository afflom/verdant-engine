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
    F25,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F25 => f25(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<u64> {
    if f1(v0)? {
        Ok(0u64)
    } else {
        f2(v1, v0)
    }
}

pub fn f1(v0: u64) -> R<bool> {
    let a1 = f3(v0)?;
    let a2 = 0u64;
    Ok(nat_eq(a1, a2))
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if if if f4(v0)? {
        true
    } else {
        f4(v1)?
    } {
        true
    } else {
        if f5(v0)? {
            f5(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f1(v0)? {
            f1(v1)?
        } else {
            false
        }
    } {
        let a89 = {
            let a67 = {
                let a43 = {
                    let a41 = {
                        let a3 = 1u64;
                        let a4 = 1u64;
                        nat_add(a3, a4)?
                    };
                    let a42 = {
                        let a39 = {
                            let a37 = {
                                let a5 = 1u64;
                                let a6 = 1u64;
                                nat_add(a5, a6)?
                            };
                            let a38 = {
                                let a35 = {
                                    let a33 = {
                                        let a7 = 1u64;
                                        let a8 = 1u64;
                                        nat_add(a7, a8)?
                                    };
                                    let a34 = {
                                        let a31 = {
                                            let a29 = {
                                                let a9 = 1u64;
                                                let a10 = 1u64;
                                                nat_add(a9, a10)?
                                            };
                                            let a30 = {
                                                let a27 = {
                                                    let a25 = {
                                                        let a11 = 1u64;
                                                        let a12 = 1u64;
                                                        nat_add(a11, a12)?
                                                    };
                                                    let a26 = {
                                                        let a23 = {
                                                            let a21 = {
                                                                let a13 = 1u64;
                                                                let a14 = 1u64;
                                                                nat_add(a13, a14)?
                                                            };
                                                            let a22 = {
                                                                let a19 = {
                                                                    let a17 = {
                                                                        let a15 = 1u64;
                                                                        let a16 = 1u64;
                                                                        nat_add(a15, a16)?
                                                                    };
                                                                    let a18 = 1u64;
                                                                    nat_mul(a17, a18)?
                                                                };
                                                                let a20 = 1u64;
                                                                nat_add(a19, a20)?
                                                            };
                                                            nat_mul(a21, a22)?
                                                        };
                                                        let a24 = 1u64;
                                                        nat_add(a23, a24)?
                                                    };
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
            let a68 = f6({
                let a65 = {
                    let a63 = {
                        let a45 = 1u64;
                        let a46 = 1u64;
                        nat_add(a45, a46)?
                    };
                    let a64 = {
                        let a61 = {
                            let a59 = {
                                let a47 = 1u64;
                                let a48 = 1u64;
                                nat_add(a47, a48)?
                            };
                            let a60 = {
                                let a57 = {
                                    let a55 = {
                                        let a49 = 1u64;
                                        let a50 = 1u64;
                                        nat_add(a49, a50)?
                                    };
                                    let a56 = {
                                        let a53 = {
                                            let a51 = 1u64;
                                            let a52 = 1u64;
                                            nat_add(a51, a52)?
                                        };
                                        let a54 = 1u64;
                                        nat_mul(a53, a54)?
                                    };
                                    nat_mul(a55, a56)?
                                };
                                let a58 = 1u64;
                                nat_add(a57, a58)?
                            };
                            nat_mul(a59, a60)?
                        };
                        let a62 = 1u64;
                        nat_add(a61, a62)?
                    };
                    nat_mul(a63, a64)?
                };
                let a66 = 1u64;
                nat_add(a65, a66)?
            })?;
            nat_mul(a67, a68)?
        };
        let a90 = f6({
            let a87 = {
                let a69 = 1u64;
                let a70 = 1u64;
                nat_add(a69, a70)?
            };
            let a88 = {
                let a85 = {
                    let a83 = {
                        let a71 = 1u64;
                        let a72 = 1u64;
                        nat_add(a71, a72)?
                    };
                    let a84 = {
                        let a81 = {
                            let a79 = {
                                let a73 = 1u64;
                                let a74 = 1u64;
                                nat_add(a73, a74)?
                            };
                            let a80 = {
                                let a77 = {
                                    let a75 = 1u64;
                                    let a76 = 1u64;
                                    nat_add(a75, a76)?
                                };
                                let a78 = 1u64;
                                nat_mul(a77, a78)?
                            };
                            nat_mul(a79, a80)?
                        };
                        let a82 = 1u64;
                        nat_add(a81, a82)?
                    };
                    nat_mul(a83, a84)?
                };
                let a86 = 1u64;
                nat_add(a85, a86)?
            };
            nat_mul(a87, a88)?
        })?;
        nat_add(a89, a90)
    } else {
        if if f5(v0)? {
            true
        } else {
            f1(v1)?
        } {
            f7(f8(v0, v1)?)
        } else {
            if if f5(v1)? {
                true
            } else {
                f1(v0)?
            } {
                f9(f8(v0, v1)?)
            } else {
                f10(v0, v1)
            }
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let a115 = v0;
    let a116 = f6({
        let a113 = {
            let a111 = {
                let a91 = 1u64;
                let a92 = 1u64;
                nat_add(a91, a92)?
            };
            let a112 = {
                let a109 = {
                    let a107 = {
                        let a93 = 1u64;
                        let a94 = 1u64;
                        nat_add(a93, a94)?
                    };
                    let a108 = {
                        let a105 = {
                            let a103 = {
                                let a95 = 1u64;
                                let a96 = 1u64;
                                nat_add(a95, a96)?
                            };
                            let a104 = {
                                let a101 = {
                                    let a99 = {
                                        let a97 = 1u64;
                                        let a98 = 1u64;
                                        nat_add(a97, a98)?
                                    };
                                    let a100 = 1u64;
                                    nat_mul(a99, a100)?
                                };
                                let a102 = 1u64;
                                nat_add(a101, a102)?
                            };
                            nat_mul(a103, a104)?
                        };
                        let a106 = 1u64;
                        nat_add(a105, a106)?
                    };
                    nat_mul(a107, a108)?
                };
                let a110 = 1u64;
                nat_add(a109, a110)?
            };
            nat_mul(a111, a112)?
        };
        let a114 = 1u64;
        nat_add(a113, a114)?
    })?;
    let a117 = 0u64;
    Ok(nat_rem(a115, a116, a117))
}

pub fn f4(v0: u64) -> R<bool> {
    let a160 = f11(v0)?;
    let a161 = {
        let a158 = {
            let a156 = {
                let a118 = 1u64;
                let a119 = 1u64;
                nat_add(a118, a119)?
            };
            let a157 = {
                let a154 = {
                    let a152 = {
                        let a120 = 1u64;
                        let a121 = 1u64;
                        nat_add(a120, a121)?
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = {
                                let a122 = 1u64;
                                let a123 = 1u64;
                                nat_add(a122, a123)?
                            };
                            let a149 = {
                                let a146 = {
                                    let a144 = {
                                        let a124 = 1u64;
                                        let a125 = 1u64;
                                        nat_add(a124, a125)?
                                    };
                                    let a145 = {
                                        let a142 = {
                                            let a140 = {
                                                let a126 = 1u64;
                                                let a127 = 1u64;
                                                nat_add(a126, a127)?
                                            };
                                            let a141 = {
                                                let a138 = {
                                                    let a136 = {
                                                        let a128 = 1u64;
                                                        let a129 = 1u64;
                                                        nat_add(a128, a129)?
                                                    };
                                                    let a137 = {
                                                        let a134 = {
                                                            let a132 = {
                                                                let a130 = 1u64;
                                                                let a131 = 1u64;
                                                                nat_add(a130, a131)?
                                                            };
                                                            let a133 = 1u64;
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
                                            nat_mul(a140, a141)?
                                        };
                                        let a143 = 1u64;
                                        nat_add(a142, a143)?
                                    };
                                    nat_mul(a144, a145)?
                                };
                                let a147 = 1u64;
                                nat_add(a146, a147)?
                            };
                            nat_mul(a148, a149)?
                        };
                        let a151 = 1u64;
                        nat_add(a150, a151)?
                    };
                    nat_mul(a152, a153)?
                };
                let a155 = 1u64;
                nat_add(a154, a155)?
            };
            nat_mul(a156, a157)?
        };
        let a159 = 1u64;
        nat_add(a158, a159)?
    };
    if nat_eq(a160, a161) {
        let a162 = 0u64;
        let a163 = f12(v0)?;
        Ok(nat_lt(a162, a163))
    } else {
        Ok(false)
    }
}

pub fn f5(v0: u64) -> R<bool> {
    let a206 = f11(v0)?;
    let a207 = {
        let a204 = {
            let a202 = {
                let a164 = 1u64;
                let a165 = 1u64;
                nat_add(a164, a165)?
            };
            let a203 = {
                let a200 = {
                    let a198 = {
                        let a166 = 1u64;
                        let a167 = 1u64;
                        nat_add(a166, a167)?
                    };
                    let a199 = {
                        let a196 = {
                            let a194 = {
                                let a168 = 1u64;
                                let a169 = 1u64;
                                nat_add(a168, a169)?
                            };
                            let a195 = {
                                let a192 = {
                                    let a190 = {
                                        let a170 = 1u64;
                                        let a171 = 1u64;
                                        nat_add(a170, a171)?
                                    };
                                    let a191 = {
                                        let a188 = {
                                            let a186 = {
                                                let a172 = 1u64;
                                                let a173 = 1u64;
                                                nat_add(a172, a173)?
                                            };
                                            let a187 = {
                                                let a184 = {
                                                    let a182 = {
                                                        let a174 = 1u64;
                                                        let a175 = 1u64;
                                                        nat_add(a174, a175)?
                                                    };
                                                    let a183 = {
                                                        let a180 = {
                                                            let a178 = {
                                                                let a176 = 1u64;
                                                                let a177 = 1u64;
                                                                nat_add(a176, a177)?
                                                            };
                                                            let a179 = 1u64;
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
            nat_mul(a202, a203)?
        };
        let a205 = 1u64;
        nat_add(a204, a205)?
    };
    if nat_eq(a206, a207) {
        let a208 = f12(v0)?;
        let a209 = 0u64;
        Ok(nat_eq(a208, a209))
    } else {
        Ok(false)
    }
}

pub fn f6(v0: u64) -> R<u64> {
    f13(Fn0::F14, v0, 1u64)
}

pub fn f7(v0: u64) -> R<u64> {
    let a276 = f9(v0)?;
    let a277 = {
        let a274 = {
            let a250 = {
                let a248 = {
                    let a210 = 1u64;
                    let a211 = 1u64;
                    nat_add(a210, a211)?
                };
                let a249 = {
                    let a246 = {
                        let a244 = {
                            let a212 = 1u64;
                            let a213 = 1u64;
                            nat_add(a212, a213)?
                        };
                        let a245 = {
                            let a242 = {
                                let a240 = {
                                    let a214 = 1u64;
                                    let a215 = 1u64;
                                    nat_add(a214, a215)?
                                };
                                let a241 = {
                                    let a238 = {
                                        let a236 = {
                                            let a216 = 1u64;
                                            let a217 = 1u64;
                                            nat_add(a216, a217)?
                                        };
                                        let a237 = {
                                            let a234 = {
                                                let a232 = {
                                                    let a218 = 1u64;
                                                    let a219 = 1u64;
                                                    nat_add(a218, a219)?
                                                };
                                                let a233 = {
                                                    let a230 = {
                                                        let a228 = {
                                                            let a220 = 1u64;
                                                            let a221 = 1u64;
                                                            nat_add(a220, a221)?
                                                        };
                                                        let a229 = {
                                                            let a226 = {
                                                                let a224 = {
                                                                    let a222 = 1u64;
                                                                    let a223 = 1u64;
                                                                    nat_add(a222, a223)?
                                                                };
                                                                let a225 = 1u64;
                                                                nat_mul(a224, a225)?
                                                            };
                                                            let a227 = 1u64;
                                                            nat_add(a226, a227)?
                                                        };
                                                        nat_mul(a228, a229)?
                                                    };
                                                    let a231 = 1u64;
                                                    nat_add(a230, a231)?
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
                        };
                        nat_mul(a244, a245)?
                    };
                    let a247 = 1u64;
                    nat_add(a246, a247)?
                };
                nat_mul(a248, a249)?
            };
            let a251 = 1u64;
            nat_add(a250, a251)?
        };
        let a275 = f6({
            let a272 = {
                let a270 = {
                    let a252 = 1u64;
                    let a253 = 1u64;
                    nat_add(a252, a253)?
                };
                let a271 = {
                    let a268 = {
                        let a266 = {
                            let a254 = 1u64;
                            let a255 = 1u64;
                            nat_add(a254, a255)?
                        };
                        let a267 = {
                            let a264 = {
                                let a262 = {
                                    let a256 = 1u64;
                                    let a257 = 1u64;
                                    nat_add(a256, a257)?
                                };
                                let a263 = {
                                    let a260 = {
                                        let a258 = 1u64;
                                        let a259 = 1u64;
                                        nat_add(a258, a259)?
                                    };
                                    let a261 = 1u64;
                                    nat_mul(a260, a261)?
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
        })?;
        nat_mul(a274, a275)?
    };
    nat_add(a276, a277)
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a284 = {
        let a278 = f15(v0)?;
        let a279 = f15(v1)?;
        nat_add(a278, a279)?
    };
    let a285 = {
        let a282 = {
            let a280 = 1u64;
            let a281 = 1u64;
            nat_add(a280, a281)?
        };
        let a283 = 1u64;
        nat_mul(a282, a283)?
    };
    let a286 = 0u64;
    Ok(nat_rem(a284, a285, a286))
}

pub fn f9(v0: u64) -> R<u64> {
    let a311 = v0;
    let a312 = f6({
        let a309 = {
            let a307 = {
                let a287 = 1u64;
                let a288 = 1u64;
                nat_add(a287, a288)?
            };
            let a308 = {
                let a305 = {
                    let a303 = {
                        let a289 = 1u64;
                        let a290 = 1u64;
                        nat_add(a289, a290)?
                    };
                    let a304 = {
                        let a301 = {
                            let a299 = {
                                let a291 = 1u64;
                                let a292 = 1u64;
                                nat_add(a291, a292)?
                            };
                            let a300 = {
                                let a297 = {
                                    let a295 = {
                                        let a293 = 1u64;
                                        let a294 = 1u64;
                                        nat_add(a293, a294)?
                                    };
                                    let a296 = 1u64;
                                    nat_mul(a295, a296)?
                                };
                                let a298 = 1u64;
                                nat_add(a297, a298)?
                            };
                            nat_mul(a299, a300)?
                        };
                        let a302 = 1u64;
                        nat_add(a301, a302)?
                    };
                    nat_mul(a303, a304)?
                };
                let a306 = 1u64;
                nat_add(a305, a306)?
            };
            nat_mul(a307, a308)?
        };
        let a310 = 1u64;
        nat_add(a309, a310)?
    })?;
    nat_mul(a311, a312)
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f16(v0)?;
    let v3: Adt0 = f16(v1)?;
    let v4: bool = {
        let a315 = match v2.clone() {
            Adt0::C0(_, h313, _, _) => {
                h313
            }
        };
        let a316 = match v3.clone() {
            Adt0::C0(_, h314, _, _) => {
                h314
            }
        };
        nat_lt(a315, a316)
    };
    let v5: u64 = {
        let a360 = match v2.clone() {
            Adt0::C0(_, h317, _, _) => {
                h317
            }
        };
        let a361 = f6(if v4 {
            let a338 = {
                let a336 = {
                    let a318 = 1u64;
                    let a319 = 1u64;
                    nat_add(a318, a319)?
                };
                let a337 = {
                    let a334 = {
                        let a332 = {
                            let a320 = 1u64;
                            let a321 = 1u64;
                            nat_add(a320, a321)?
                        };
                        let a333 = {
                            let a330 = {
                                let a322 = 1u64;
                                let a323 = 1u64;
                                nat_add(a322, a323)?
                            };
                            let a331 = {
                                let a328 = {
                                    let a326 = {
                                        let a324 = 1u64;
                                        let a325 = 1u64;
                                        nat_add(a324, a325)?
                                    };
                                    let a327 = 1u64;
                                    nat_mul(a326, a327)?
                                };
                                let a329 = 1u64;
                                nat_add(a328, a329)?
                            };
                            nat_mul(a330, a331)?
                        };
                        nat_mul(a332, a333)?
                    };
                    let a335 = 1u64;
                    nat_add(a334, a335)?
                };
                nat_mul(a336, a337)?
            };
            let a339 = 1u64;
            nat_add(a338, a339)?
        } else {
            let a358 = {
                let a340 = 1u64;
                let a341 = 1u64;
                nat_add(a340, a341)?
            };
            let a359 = {
                let a356 = {
                    let a354 = {
                        let a342 = 1u64;
                        let a343 = 1u64;
                        nat_add(a342, a343)?
                    };
                    let a355 = {
                        let a352 = {
                            let a344 = 1u64;
                            let a345 = 1u64;
                            nat_add(a344, a345)?
                        };
                        let a353 = {
                            let a350 = {
                                let a348 = {
                                    let a346 = 1u64;
                                    let a347 = 1u64;
                                    nat_add(a346, a347)?
                                };
                                let a349 = 1u64;
                                nat_mul(a348, a349)?
                            };
                            let a351 = 1u64;
                            nat_add(a350, a351)?
                        };
                        nat_mul(a352, a353)?
                    };
                    nat_mul(a354, a355)?
                };
                let a357 = 1u64;
                nat_add(a356, a357)?
            };
            nat_mul(a358, a359)?
        })?;
        nat_mul(a360, a361)?
    };
    let v6: u64 = {
        let a418 = {
            let a416 = {
                let a413 = match v2 {
                    Adt0::C0(h362, _, _, _) => {
                        h362
                    }
                };
                let a414 = {
                    let a411 = {
                        let a409 = {
                            let a363 = 1u64;
                            let a364 = 1u64;
                            nat_add(a363, a364)?
                        };
                        let a410 = {
                            let a407 = {
                                let a405 = {
                                    let a365 = 1u64;
                                    let a366 = 1u64;
                                    nat_add(a365, a366)?
                                };
                                let a406 = {
                                    let a403 = {
                                        let a401 = {
                                            let a367 = 1u64;
                                            let a368 = 1u64;
                                            nat_add(a367, a368)?
                                        };
                                        let a402 = {
                                            let a399 = {
                                                let a397 = {
                                                    let a369 = 1u64;
                                                    let a370 = 1u64;
                                                    nat_add(a369, a370)?
                                                };
                                                let a398 = {
                                                    let a395 = {
                                                        let a393 = {
                                                            let a371 = 1u64;
                                                            let a372 = 1u64;
                                                            nat_add(a371, a372)?
                                                        };
                                                        let a394 = {
                                                            let a391 = {
                                                                let a389 = {
                                                                    let a373 = 1u64;
                                                                    let a374 = 1u64;
                                                                    nat_add(a373, a374)?
                                                                };
                                                                let a390 = {
                                                                    let a387 = {
                                                                        let a385 = {
                                                                            let a375 = 1u64;
                                                                            let a376 = 1u64;
                                                                            nat_add(a375, a376)?
                                                                        };
                                                                        let a386 = {
                                                                            let a383 = {
                                                                                let a377 = 1u64;
                                                                                let a378 = 1u64;
                                                                                nat_add(a377, a378)?
                                                                            };
                                                                            let a384 = {
                                                                                let a381 = {
                                                                                    let a379 = 1u64;
                                                                                    let a380 = 1u64;
                                                                                    nat_add(a379, a380)?
                                                                                };
                                                                                let a382 = 1u64;
                                                                                nat_mul(a381, a382)?
                                                                            };
                                                                            nat_mul(a383, a384)?
                                                                        };
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
                nat_add(a413, a414)?
            };
            let a417 = match v3.clone() {
                Adt0::C0(h415, _, _, _) => {
                    h415
                }
            };
            nat_sub(a416, a417)
        };
        let a419 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a418, a419)
    };
    let v7: u64 = {
        let a421 = v5;
        let a422 = match v3.clone() {
            Adt0::C0(_, h420, _, _) => {
                h420
            }
        };
        let a423 = 0u64;
        nat_quot(a421, a422, a423)
    };
    let v8: bool = {
        let a428 = 0u64;
        let a429 = {
            let a425 = v5;
            let a426 = match v3 {
                Adt0::C0(_, h424, _, _) => {
                    h424
                }
            };
            let a427 = 0u64;
            nat_rem(a425, a426, a427)
        };
        nat_lt(a428, a429)
    };
    f17(f8(v0, v1)?, v6, f18(v7, v8)?)
}

pub fn f11(v0: u64) -> R<u64> {
    let a452 = f3(v0)?;
    let a453 = f6({
        let a450 = {
            let a448 = {
                let a430 = 1u64;
                let a431 = 1u64;
                nat_add(a430, a431)?
            };
            let a449 = {
                let a446 = {
                    let a444 = {
                        let a432 = 1u64;
                        let a433 = 1u64;
                        nat_add(a432, a433)?
                    };
                    let a445 = {
                        let a442 = {
                            let a440 = {
                                let a434 = 1u64;
                                let a435 = 1u64;
                                nat_add(a434, a435)?
                            };
                            let a441 = {
                                let a438 = {
                                    let a436 = 1u64;
                                    let a437 = 1u64;
                                    nat_add(a436, a437)?
                                };
                                let a439 = 1u64;
                                nat_mul(a438, a439)?
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
    let a454 = 0u64;
    Ok(nat_quot(a452, a453, a454))
}

pub fn f12(v0: u64) -> R<u64> {
    let a477 = v0;
    let a478 = f6({
        let a475 = {
            let a473 = {
                let a455 = 1u64;
                let a456 = 1u64;
                nat_add(a455, a456)?
            };
            let a474 = {
                let a471 = {
                    let a469 = {
                        let a457 = 1u64;
                        let a458 = 1u64;
                        nat_add(a457, a458)?
                    };
                    let a470 = {
                        let a467 = {
                            let a465 = {
                                let a459 = 1u64;
                                let a460 = 1u64;
                                nat_add(a459, a460)?
                            };
                            let a466 = {
                                let a463 = {
                                    let a461 = 1u64;
                                    let a462 = 1u64;
                                    nat_add(a461, a462)?
                                };
                                let a464 = 1u64;
                                nat_mul(a463, a464)?
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
            };
            nat_mul(a473, a474)?
        };
        let a476 = 1u64;
        nat_add(a475, a476)?
    })?;
    let a479 = 0u64;
    Ok(nat_rem(a477, a478, a479))
}

pub fn f13(v0: Fn0, p483: u64, p484: u64) -> R<u64> {
    let m482 = (p483, p484);
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v1, v2) = m482;
                let m480 = v1;
                if m480 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m480 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c481: Fn0 = v0.clone();
                        c481.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a485 = v0;
    let a486 = v0;
    nat_add(a485, a486)
}

pub fn f15(v0: u64) -> R<u64> {
    let a511 = v0;
    let a512 = f6({
        let a509 = {
            let a507 = {
                let a487 = 1u64;
                let a488 = 1u64;
                nat_add(a487, a488)?
            };
            let a508 = {
                let a505 = {
                    let a503 = {
                        let a489 = 1u64;
                        let a490 = 1u64;
                        nat_add(a489, a490)?
                    };
                    let a504 = {
                        let a501 = {
                            let a499 = {
                                let a491 = 1u64;
                                let a492 = 1u64;
                                nat_add(a491, a492)?
                            };
                            let a500 = {
                                let a497 = {
                                    let a495 = {
                                        let a493 = 1u64;
                                        let a494 = 1u64;
                                        nat_add(a493, a494)?
                                    };
                                    let a496 = 1u64;
                                    nat_mul(a495, a496)?
                                };
                                let a498 = 1u64;
                                nat_add(a497, a498)?
                            };
                            nat_mul(a499, a500)?
                        };
                        let a502 = 1u64;
                        nat_add(a501, a502)?
                    };
                    nat_mul(a503, a504)?
                };
                let a506 = 1u64;
                nat_add(a505, a506)?
            };
            nat_mul(a507, a508)?
        };
        let a510 = 1u64;
        nat_add(a509, a510)?
    })?;
    let a513 = 0u64;
    Ok(nat_quot(a511, a512, a513))
}

pub fn f16(v0: u64) -> R<Adt0> {
    f19(f20(v0)?, f21(v0)?, 0u64, f6({
        let a534 = {
            let a532 = {
                let a514 = 1u64;
                let a515 = 1u64;
                nat_add(a514, a515)?
            };
            let a533 = {
                let a530 = {
                    let a528 = {
                        let a516 = 1u64;
                        let a517 = 1u64;
                        nat_add(a516, a517)?
                    };
                    let a529 = {
                        let a526 = {
                            let a524 = {
                                let a518 = 1u64;
                                let a519 = 1u64;
                                nat_add(a518, a519)?
                            };
                            let a525 = {
                                let a522 = {
                                    let a520 = 1u64;
                                    let a521 = 1u64;
                                    nat_add(a520, a521)?
                                };
                                let a523 = 1u64;
                                nat_mul(a522, a523)?
                            };
                            nat_mul(a524, a525)?
                        };
                        let a527 = 1u64;
                        nat_add(a526, a527)?
                    };
                    nat_mul(a528, a529)?
                };
                let a531 = 1u64;
                nat_add(a530, a531)?
            };
            nat_mul(a532, a533)?
        };
        let a535 = 1u64;
        nat_add(a534, a535)?
    })?)
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a558 = f6({
        let a556 = {
            let a554 = {
                let a536 = 1u64;
                let a537 = 1u64;
                nat_add(a536, a537)?
            };
            let a555 = {
                let a552 = {
                    let a550 = {
                        let a538 = 1u64;
                        let a539 = 1u64;
                        nat_add(a538, a539)?
                    };
                    let a551 = {
                        let a548 = {
                            let a540 = 1u64;
                            let a541 = 1u64;
                            nat_add(a540, a541)?
                        };
                        let a549 = {
                            let a546 = {
                                let a544 = {
                                    let a542 = 1u64;
                                    let a543 = 1u64;
                                    nat_add(a542, a543)?
                                };
                                let a545 = 1u64;
                                nat_mul(a544, a545)?
                            };
                            let a547 = 1u64;
                            nat_add(a546, a547)?
                        };
                        nat_mul(a548, a549)?
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
    })?;
    let a559 = v2;
    if nat_le(a558, a559) {
        f22(v0, {
            let a560 = v1;
            let a561 = 1u64;
            nat_add(a560, a561)?
        }, f23(v2, 1u64)?)
    } else {
        let v3: Adt0 = f19(v1, v2, {
            let a598 = {
                let a596 = {
                    let a562 = 1u64;
                    let a563 = 1u64;
                    nat_add(a562, a563)?
                };
                let a597 = {
                    let a594 = {
                        let a564 = 1u64;
                        let a565 = 1u64;
                        nat_add(a564, a565)?
                    };
                    let a595 = {
                        let a592 = {
                            let a566 = 1u64;
                            let a567 = 1u64;
                            nat_add(a566, a567)?
                        };
                        let a593 = {
                            let a590 = {
                                let a568 = 1u64;
                                let a569 = 1u64;
                                nat_add(a568, a569)?
                            };
                            let a591 = {
                                let a588 = {
                                    let a570 = 1u64;
                                    let a571 = 1u64;
                                    nat_add(a570, a571)?
                                };
                                let a589 = {
                                    let a586 = {
                                        let a572 = 1u64;
                                        let a573 = 1u64;
                                        nat_add(a572, a573)?
                                    };
                                    let a587 = {
                                        let a584 = {
                                            let a574 = 1u64;
                                            let a575 = 1u64;
                                            nat_add(a574, a575)?
                                        };
                                        let a585 = {
                                            let a582 = {
                                                let a576 = 1u64;
                                                let a577 = 1u64;
                                                nat_add(a576, a577)?
                                            };
                                            let a583 = {
                                                let a580 = {
                                                    let a578 = 1u64;
                                                    let a579 = 1u64;
                                                    nat_add(a578, a579)?
                                                };
                                                let a581 = 1u64;
                                                nat_mul(a580, a581)?
                                            };
                                            nat_mul(a582, a583)?
                                        };
                                        nat_mul(a584, a585)?
                                    };
                                    nat_mul(a586, a587)?
                                };
                                nat_mul(a588, a589)?
                            };
                            nat_mul(a590, a591)?
                        };
                        nat_mul(a592, a593)?
                    };
                    nat_mul(a594, a595)?
                };
                nat_mul(a596, a597)?
            };
            let a599 = 1u64;
            nat_add(a598, a599)?
        }, f6({
            let a618 = {
                let a600 = 1u64;
                let a601 = 1u64;
                nat_add(a600, a601)?
            };
            let a619 = {
                let a616 = {
                    let a614 = {
                        let a602 = 1u64;
                        let a603 = 1u64;
                        nat_add(a602, a603)?
                    };
                    let a615 = {
                        let a612 = {
                            let a604 = 1u64;
                            let a605 = 1u64;
                            nat_add(a604, a605)?
                        };
                        let a613 = {
                            let a610 = {
                                let a608 = {
                                    let a606 = 1u64;
                                    let a607 = 1u64;
                                    nat_add(a606, a607)?
                                };
                                let a609 = 1u64;
                                nat_mul(a608, a609)?
                            };
                            let a611 = 1u64;
                            nat_add(a610, a611)?
                        };
                        nat_mul(a612, a613)?
                    };
                    nat_mul(a614, a615)?
                };
                let a617 = 1u64;
                nat_add(a616, a617)?
            };
            nat_mul(a618, a619)?
        })?)?;
        f22(v0, match v3.clone() {
            Adt0::C0(h620, _, _, _) => {
                h620
            }
        }, match v3 {
            Adt0::C0(_, h621, _, _) => {
                h621
            }
        })
    }
}

pub fn f18(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a629 = {
            let a626 = v0;
            let a627 = {
                let a624 = {
                    let a622 = 1u64;
                    let a623 = 1u64;
                    nat_add(a622, a623)?
                };
                let a625 = 1u64;
                nat_mul(a624, a625)?
            };
            let a628 = 0u64;
            nat_rem(a626, a627, a628)
        };
        let a630 = 0u64;
        nat_eq(a629, a630)
    } else {
        false
    } {
        let a631 = v0;
        let a632 = 1u64;
        nat_add(a631, a632)
    } else {
        Ok(v0)
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h633, _) = f24(Fn1::F25, {
        let a652 = {
            let a634 = 1u64;
            let a635 = 1u64;
            nat_add(a634, a635)?
        };
        let a653 = {
            let a650 = {
                let a636 = 1u64;
                let a637 = 1u64;
                nat_add(a636, a637)?
            };
            let a651 = {
                let a648 = {
                    let a638 = 1u64;
                    let a639 = 1u64;
                    nat_add(a638, a639)?
                };
                let a649 = {
                    let a646 = {
                        let a640 = 1u64;
                        let a641 = 1u64;
                        nat_add(a640, a641)?
                    };
                    let a647 = {
                        let a644 = {
                            let a642 = 1u64;
                            let a643 = 1u64;
                            nat_add(a642, a643)?
                        };
                        let a645 = 1u64;
                        nat_mul(a644, a645)?
                    };
                    nat_mul(a646, a647)?
                };
                nat_mul(a648, a649)?
            };
            nat_mul(a650, a651)?
        };
        nat_mul(a652, a653)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h633)
}

pub fn f20(v0: u64) -> R<u64> {
    let a692 = {
        let a654 = f11(v0)?;
        let a655 = 0u64;
        if nat_eq(a654, a655) {
            1u64
        } else {
            f11(v0)?
        }
    };
    let a693 = {
        let a690 = {
            let a656 = 1u64;
            let a657 = 1u64;
            nat_add(a656, a657)?
        };
        let a691 = {
            let a688 = {
                let a658 = 1u64;
                let a659 = 1u64;
                nat_add(a658, a659)?
            };
            let a689 = {
                let a686 = {
                    let a660 = 1u64;
                    let a661 = 1u64;
                    nat_add(a660, a661)?
                };
                let a687 = {
                    let a684 = {
                        let a662 = 1u64;
                        let a663 = 1u64;
                        nat_add(a662, a663)?
                    };
                    let a685 = {
                        let a682 = {
                            let a664 = 1u64;
                            let a665 = 1u64;
                            nat_add(a664, a665)?
                        };
                        let a683 = {
                            let a680 = {
                                let a666 = 1u64;
                                let a667 = 1u64;
                                nat_add(a666, a667)?
                            };
                            let a681 = {
                                let a678 = {
                                    let a668 = 1u64;
                                    let a669 = 1u64;
                                    nat_add(a668, a669)?
                                };
                                let a679 = {
                                    let a676 = {
                                        let a670 = 1u64;
                                        let a671 = 1u64;
                                        nat_add(a670, a671)?
                                    };
                                    let a677 = {
                                        let a674 = {
                                            let a672 = 1u64;
                                            let a673 = 1u64;
                                            nat_add(a672, a673)?
                                        };
                                        let a675 = 1u64;
                                        nat_mul(a674, a675)?
                                    };
                                    nat_mul(a676, a677)?
                                };
                                nat_mul(a678, a679)?
                            };
                            nat_mul(a680, a681)?
                        };
                        nat_mul(a682, a683)?
                    };
                    nat_mul(a684, a685)?
                };
                nat_mul(a686, a687)?
            };
            nat_mul(a688, a689)?
        };
        nat_mul(a690, a691)?
    };
    nat_add(a692, a693)
}

pub fn f21(v0: u64) -> R<u64> {
    let a718 = f12(v0)?;
    let a719 = {
        let a694 = f11(v0)?;
        let a695 = 0u64;
        if nat_eq(a694, a695) {
            0u64
        } else {
            f6({
                let a716 = {
                    let a714 = {
                        let a696 = 1u64;
                        let a697 = 1u64;
                        nat_add(a696, a697)?
                    };
                    let a715 = {
                        let a712 = {
                            let a710 = {
                                let a698 = 1u64;
                                let a699 = 1u64;
                                nat_add(a698, a699)?
                            };
                            let a711 = {
                                let a708 = {
                                    let a706 = {
                                        let a700 = 1u64;
                                        let a701 = 1u64;
                                        nat_add(a700, a701)?
                                    };
                                    let a707 = {
                                        let a704 = {
                                            let a702 = 1u64;
                                            let a703 = 1u64;
                                            nat_add(a702, a703)?
                                        };
                                        let a705 = 1u64;
                                        nat_mul(a704, a705)?
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
                    nat_mul(a714, a715)?
                };
                let a717 = 1u64;
                nat_add(a716, a717)?
            })?
        }
    };
    nat_add(a718, a719)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a758 = v1;
    let a759 = {
        let a756 = {
            let a754 = {
                let a720 = 1u64;
                let a721 = 1u64;
                nat_add(a720, a721)?
            };
            let a755 = {
                let a752 = {
                    let a722 = 1u64;
                    let a723 = 1u64;
                    nat_add(a722, a723)?
                };
                let a753 = {
                    let a750 = {
                        let a724 = 1u64;
                        let a725 = 1u64;
                        nat_add(a724, a725)?
                    };
                    let a751 = {
                        let a748 = {
                            let a726 = 1u64;
                            let a727 = 1u64;
                            nat_add(a726, a727)?
                        };
                        let a749 = {
                            let a746 = {
                                let a728 = 1u64;
                                let a729 = 1u64;
                                nat_add(a728, a729)?
                            };
                            let a747 = {
                                let a744 = {
                                    let a730 = 1u64;
                                    let a731 = 1u64;
                                    nat_add(a730, a731)?
                                };
                                let a745 = {
                                    let a742 = {
                                        let a732 = 1u64;
                                        let a733 = 1u64;
                                        nat_add(a732, a733)?
                                    };
                                    let a743 = {
                                        let a740 = {
                                            let a734 = 1u64;
                                            let a735 = 1u64;
                                            nat_add(a734, a735)?
                                        };
                                        let a741 = {
                                            let a738 = {
                                                let a736 = 1u64;
                                                let a737 = 1u64;
                                                nat_add(a736, a737)?
                                            };
                                            let a739 = 1u64;
                                            nat_mul(a738, a739)?
                                        };
                                        nat_mul(a740, a741)?
                                    };
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
        let a757 = 1u64;
        nat_add(a756, a757)?
    };
    if nat_lt(a758, a759) {
        f26(v0, {
            let a796 = {
                let a794 = {
                    let a760 = 1u64;
                    let a761 = 1u64;
                    nat_add(a760, a761)?
                };
                let a795 = {
                    let a792 = {
                        let a762 = 1u64;
                        let a763 = 1u64;
                        nat_add(a762, a763)?
                    };
                    let a793 = {
                        let a790 = {
                            let a764 = 1u64;
                            let a765 = 1u64;
                            nat_add(a764, a765)?
                        };
                        let a791 = {
                            let a788 = {
                                let a766 = 1u64;
                                let a767 = 1u64;
                                nat_add(a766, a767)?
                            };
                            let a789 = {
                                let a786 = {
                                    let a768 = 1u64;
                                    let a769 = 1u64;
                                    nat_add(a768, a769)?
                                };
                                let a787 = {
                                    let a784 = {
                                        let a770 = 1u64;
                                        let a771 = 1u64;
                                        nat_add(a770, a771)?
                                    };
                                    let a785 = {
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
            let a797 = 1u64;
            nat_add(a796, a797)?
        }, f23(v2, {
            let a836 = {
                let a834 = {
                    let a832 = {
                        let a798 = 1u64;
                        let a799 = 1u64;
                        nat_add(a798, a799)?
                    };
                    let a833 = {
                        let a830 = {
                            let a800 = 1u64;
                            let a801 = 1u64;
                            nat_add(a800, a801)?
                        };
                        let a831 = {
                            let a828 = {
                                let a802 = 1u64;
                                let a803 = 1u64;
                                nat_add(a802, a803)?
                            };
                            let a829 = {
                                let a826 = {
                                    let a804 = 1u64;
                                    let a805 = 1u64;
                                    nat_add(a804, a805)?
                                };
                                let a827 = {
                                    let a824 = {
                                        let a806 = 1u64;
                                        let a807 = 1u64;
                                        nat_add(a806, a807)?
                                    };
                                    let a825 = {
                                        let a822 = {
                                            let a808 = 1u64;
                                            let a809 = 1u64;
                                            nat_add(a808, a809)?
                                        };
                                        let a823 = {
                                            let a820 = {
                                                let a810 = 1u64;
                                                let a811 = 1u64;
                                                nat_add(a810, a811)?
                                            };
                                            let a821 = {
                                                let a818 = {
                                                    let a812 = 1u64;
                                                    let a813 = 1u64;
                                                    nat_add(a812, a813)?
                                                };
                                                let a819 = {
                                                    let a816 = {
                                                        let a814 = 1u64;
                                                        let a815 = 1u64;
                                                        nat_add(a814, a815)?
                                                    };
                                                    let a817 = 1u64;
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
                                nat_mul(a826, a827)?
                            };
                            nat_mul(a828, a829)?
                        };
                        nat_mul(a830, a831)?
                    };
                    nat_mul(a832, a833)?
                };
                let a835 = 1u64;
                nat_add(a834, a835)?
            };
            let a837 = v1;
            nat_sub(a836, a837)
        })?)
    } else {
        f26(v0, v1, v2)
    }
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let a838 = v1;
    let a839 = 0u64;
    if nat_eq(a838, a839) {
        Ok(v0)
    } else {
        let a864 = {
            let a862 = {
                let a840 = 1u64;
                let a841 = 1u64;
                nat_add(a840, a841)?
            };
            let a863 = {
                let a860 = {
                    let a842 = 1u64;
                    let a843 = 1u64;
                    nat_add(a842, a843)?
                };
                let a861 = {
                    let a858 = {
                        let a844 = 1u64;
                        let a845 = 1u64;
                        nat_add(a844, a845)?
                    };
                    let a859 = {
                        let a856 = {
                            let a846 = 1u64;
                            let a847 = 1u64;
                            nat_add(a846, a847)?
                        };
                        let a857 = {
                            let a854 = {
                                let a848 = 1u64;
                                let a849 = 1u64;
                                nat_add(a848, a849)?
                            };
                            let a855 = {
                                let a852 = {
                                    let a850 = 1u64;
                                    let a851 = 1u64;
                                    nat_add(a850, a851)?
                                };
                                let a853 = 1u64;
                                nat_mul(a852, a853)?
                            };
                            nat_mul(a854, a855)?
                        };
                        nat_mul(a856, a857)?
                    };
                    nat_mul(a858, a859)?
                };
                nat_mul(a860, a861)?
            };
            nat_mul(a862, a863)?
        };
        let a865 = v1;
        if nat_le(a864, a865) {
            let a866 = v0;
            let a867 = 0u64;
            if nat_eq(a866, a867) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f6(v1)?;
            f18({
                let a868 = v0;
                let a869 = v2;
                let a870 = 0u64;
                nat_quot(a868, a869, a870)
            }, {
                let a874 = 0u64;
                let a875 = {
                    let a871 = v0;
                    let a872 = v2;
                    let a873 = 0u64;
                    nat_rem(a871, a872, a873)
                };
                nat_lt(a874, a875)
            })
        }
    }
}

pub fn f24(v0: Fn1, p880: u64, p881: Adt0) -> R<(Adt0, bool)> {
    let m879 = (p880, p881);
    {
        let mut m879 = m879;
        loop {
            let step = {
                let (v1, v2) = m879;
                let m876 = v1;
                if m876 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m876 - 1;
                    let m878 = {
                        let c877: Fn1 = v0.clone();
                        c877.apply(v2.clone())?
                    };
                    match m878 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f25(v0: Adt0) -> R<Option<Adt0>> {
    f27(v0)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a894 = v2;
        let a895 = {
            let a892 = {
                let a882 = 1u64;
                let a883 = 1u64;
                nat_add(a882, a883)?
            };
            let a893 = {
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
            nat_mul(a892, a893)?
        };
        let a896 = 0u64;
        nat_rem(a894, a895, a896)
    };
    let v4: u64 = {
        let a909 = v2;
        let a910 = {
            let a907 = {
                let a897 = 1u64;
                let a898 = 1u64;
                nat_add(a897, a898)?
            };
            let a908 = {
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
            nat_mul(a907, a908)?
        };
        let a911 = 0u64;
        nat_quot(a909, a910, a911)
    };
    let v5: bool = {
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
        let a921 = v3;
        if nat_lt(a920, a921) {
            true
        } else {
            let a930 = v3;
            let a931 = {
                let a928 = {
                    let a922 = 1u64;
                    let a923 = 1u64;
                    nat_add(a922, a923)?
                };
                let a929 = {
                    let a926 = {
                        let a924 = 1u64;
                        let a925 = 1u64;
                        nat_add(a924, a925)?
                    };
                    let a927 = 1u64;
                    nat_mul(a926, a927)?
                };
                nat_mul(a928, a929)?
            };
            if nat_eq(a930, a931) {
                let a939 = {
                    let a936 = v4;
                    let a937 = {
                        let a934 = {
                            let a932 = 1u64;
                            let a933 = 1u64;
                            nat_add(a932, a933)?
                        };
                        let a935 = 1u64;
                        nat_mul(a934, a935)?
                    };
                    let a938 = 0u64;
                    nat_rem(a936, a937, a938)
                };
                let a940 = 1u64;
                nat_eq(a939, a940)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a941 = v4;
        let a942 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a941, a942)?
    };
    let v7: bool = {
        let a971 = {
            let a969 = {
                let a945 = {
                    let a943 = 1u64;
                    let a944 = 1u64;
                    nat_add(a943, a944)?
                };
                let a946 = 1u64;
                nat_mul(a945, a946)?
            };
            let a970 = f6({
                let a967 = {
                    let a965 = {
                        let a947 = 1u64;
                        let a948 = 1u64;
                        nat_add(a947, a948)?
                    };
                    let a966 = {
                        let a963 = {
                            let a961 = {
                                let a949 = 1u64;
                                let a950 = 1u64;
                                nat_add(a949, a950)?
                            };
                            let a962 = {
                                let a959 = {
                                    let a957 = {
                                        let a951 = 1u64;
                                        let a952 = 1u64;
                                        nat_add(a951, a952)?
                                    };
                                    let a958 = {
                                        let a955 = {
                                            let a953 = 1u64;
                                            let a954 = 1u64;
                                            nat_add(a953, a954)?
                                        };
                                        let a956 = 1u64;
                                        nat_mul(a955, a956)?
                                    };
                                    nat_mul(a957, a958)?
                                };
                                let a960 = 1u64;
                                nat_add(a959, a960)?
                            };
                            nat_mul(a961, a962)?
                        };
                        let a964 = 1u64;
                        nat_add(a963, a964)?
                    };
                    nat_mul(a965, a966)?
                };
                let a968 = 1u64;
                nat_add(a967, a968)?
            })?;
            nat_mul(a969, a970)?
        };
        let a972 = v6;
        nat_le(a971, a972)
    };
    let v8: u64 = if v7 {
        let a977 = v6;
        let a978 = {
            let a975 = {
                let a973 = 1u64;
                let a974 = 1u64;
                nat_add(a973, a974)?
            };
            let a976 = 1u64;
            nat_mul(a975, a976)?
        };
        let a979 = 0u64;
        nat_quot(a977, a978, a979)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a980 = v1;
        let a981 = 1u64;
        nat_add(a980, a981)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1020 = v9;
        let a1021 = {
            let a1018 = {
                let a1016 = {
                    let a982 = 1u64;
                    let a983 = 1u64;
                    nat_add(a982, a983)?
                };
                let a1017 = {
                    let a1014 = {
                        let a984 = 1u64;
                        let a985 = 1u64;
                        nat_add(a984, a985)?
                    };
                    let a1015 = {
                        let a1012 = {
                            let a986 = 1u64;
                            let a987 = 1u64;
                            nat_add(a986, a987)?
                        };
                        let a1013 = {
                            let a1010 = {
                                let a988 = 1u64;
                                let a989 = 1u64;
                                nat_add(a988, a989)?
                            };
                            let a1011 = {
                                let a1008 = {
                                    let a990 = 1u64;
                                    let a991 = 1u64;
                                    nat_add(a990, a991)?
                                };
                                let a1009 = {
                                    let a1006 = {
                                        let a992 = 1u64;
                                        let a993 = 1u64;
                                        nat_add(a992, a993)?
                                    };
                                    let a1007 = {
                                        let a1004 = {
                                            let a994 = 1u64;
                                            let a995 = 1u64;
                                            nat_add(a994, a995)?
                                        };
                                        let a1005 = {
                                            let a1002 = {
                                                let a996 = 1u64;
                                                let a997 = 1u64;
                                                nat_add(a996, a997)?
                                            };
                                            let a1003 = {
                                                let a1000 = {
                                                    let a998 = 1u64;
                                                    let a999 = 1u64;
                                                    nat_add(a998, a999)?
                                                };
                                                let a1001 = 1u64;
                                                nat_mul(a1000, a1001)?
                                            };
                                            nat_mul(a1002, a1003)?
                                        };
                                        nat_mul(a1004, a1005)?
                                    };
                                    nat_mul(a1006, a1007)?
                                };
                                nat_mul(a1008, a1009)?
                            };
                            nat_mul(a1010, a1011)?
                        };
                        nat_mul(a1012, a1013)?
                    };
                    nat_mul(a1014, a1015)?
                };
                nat_mul(a1016, a1017)?
            };
            let a1019 = 1u64;
            nat_add(a1018, a1019)?
        };
        if if nat_eq(a1020, a1021) {
            let a1044 = v8;
            let a1045 = f6({
                let a1042 = {
                    let a1040 = {
                        let a1022 = 1u64;
                        let a1023 = 1u64;
                        nat_add(a1022, a1023)?
                    };
                    let a1041 = {
                        let a1038 = {
                            let a1036 = {
                                let a1024 = 1u64;
                                let a1025 = 1u64;
                                nat_add(a1024, a1025)?
                            };
                            let a1037 = {
                                let a1034 = {
                                    let a1032 = {
                                        let a1026 = 1u64;
                                        let a1027 = 1u64;
                                        nat_add(a1026, a1027)?
                                    };
                                    let a1033 = {
                                        let a1030 = {
                                            let a1028 = 1u64;
                                            let a1029 = 1u64;
                                            nat_add(a1028, a1029)?
                                        };
                                        let a1031 = 1u64;
                                        nat_mul(a1030, a1031)?
                                    };
                                    nat_mul(a1032, a1033)?
                                };
                                let a1035 = 1u64;
                                nat_add(a1034, a1035)?
                            };
                            nat_mul(a1036, a1037)?
                        };
                        let a1039 = 1u64;
                        nat_add(a1038, a1039)?
                    };
                    nat_mul(a1040, a1041)?
                };
                let a1043 = 1u64;
                nat_add(a1042, a1043)?
            })?;
            nat_lt(a1044, a1045)
        } else {
            false
        } {
            0u64
        } else {
            let a1082 = v9;
            let a1083 = {
                let a1080 = {
                    let a1046 = 1u64;
                    let a1047 = 1u64;
                    nat_add(a1046, a1047)?
                };
                let a1081 = {
                    let a1078 = {
                        let a1048 = 1u64;
                        let a1049 = 1u64;
                        nat_add(a1048, a1049)?
                    };
                    let a1079 = {
                        let a1076 = {
                            let a1050 = 1u64;
                            let a1051 = 1u64;
                            nat_add(a1050, a1051)?
                        };
                        let a1077 = {
                            let a1074 = {
                                let a1052 = 1u64;
                                let a1053 = 1u64;
                                nat_add(a1052, a1053)?
                            };
                            let a1075 = {
                                let a1072 = {
                                    let a1054 = 1u64;
                                    let a1055 = 1u64;
                                    nat_add(a1054, a1055)?
                                };
                                let a1073 = {
                                    let a1070 = {
                                        let a1056 = 1u64;
                                        let a1057 = 1u64;
                                        nat_add(a1056, a1057)?
                                    };
                                    let a1071 = {
                                        let a1068 = {
                                            let a1058 = 1u64;
                                            let a1059 = 1u64;
                                            nat_add(a1058, a1059)?
                                        };
                                        let a1069 = {
                                            let a1066 = {
                                                let a1060 = 1u64;
                                                let a1061 = 1u64;
                                                nat_add(a1060, a1061)?
                                            };
                                            let a1067 = {
                                                let a1064 = {
                                                    let a1062 = 1u64;
                                                    let a1063 = 1u64;
                                                    nat_add(a1062, a1063)?
                                                };
                                                let a1065 = 1u64;
                                                nat_mul(a1064, a1065)?
                                            };
                                            nat_mul(a1066, a1067)?
                                        };
                                        nat_mul(a1068, a1069)?
                                    };
                                    nat_mul(a1070, a1071)?
                                };
                                nat_mul(a1072, a1073)?
                            };
                            nat_mul(a1074, a1075)?
                        };
                        nat_mul(a1076, a1077)?
                    };
                    nat_mul(a1078, a1079)?
                };
                nat_mul(a1080, a1081)?
            };
            nat_sub(a1082, a1083)
        }
    };
    let a1084 = v8;
    let a1085 = 0u64;
    if nat_eq(a1084, a1085) {
        f9(v0)
    } else {
        let a1138 = {
            let a1136 = {
                let a1134 = {
                    let a1086 = 1u64;
                    let a1087 = 1u64;
                    nat_add(a1086, a1087)?
                };
                let a1135 = {
                    let a1132 = {
                        let a1130 = {
                            let a1088 = 1u64;
                            let a1089 = 1u64;
                            nat_add(a1088, a1089)?
                        };
                        let a1131 = {
                            let a1128 = {
                                let a1126 = {
                                    let a1090 = 1u64;
                                    let a1091 = 1u64;
                                    nat_add(a1090, a1091)?
                                };
                                let a1127 = {
                                    let a1124 = {
                                        let a1122 = {
                                            let a1092 = 1u64;
                                            let a1093 = 1u64;
                                            nat_add(a1092, a1093)?
                                        };
                                        let a1123 = {
                                            let a1120 = {
                                                let a1118 = {
                                                    let a1094 = 1u64;
                                                    let a1095 = 1u64;
                                                    nat_add(a1094, a1095)?
                                                };
                                                let a1119 = {
                                                    let a1116 = {
                                                        let a1114 = {
                                                            let a1096 = 1u64;
                                                            let a1097 = 1u64;
                                                            nat_add(a1096, a1097)?
                                                        };
                                                        let a1115 = {
                                                            let a1112 = {
                                                                let a1110 = {
                                                                    let a1098 = 1u64;
                                                                    let a1099 = 1u64;
                                                                    nat_add(a1098, a1099)?
                                                                };
                                                                let a1111 = {
                                                                    let a1108 = {
                                                                        let a1106 = {
                                                                            let a1100 = 1u64;
                                                                            let a1101 = 1u64;
                                                                            nat_add(a1100, a1101)?
                                                                        };
                                                                        let a1107 = {
                                                                            let a1104 = {
                                                                                let a1102 = 1u64;
                                                                                let a1103 = 1u64;
                                                                                nat_add(a1102, a1103)?
                                                                            };
                                                                            let a1105 = 1u64;
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
                nat_mul(a1134, a1135)?
            };
            let a1137 = 1u64;
            nat_add(a1136, a1137)?
        };
        let a1139 = v9;
        if nat_le(a1138, a1139) {
            f7(v0)
        } else {
            let a1191 = {
                let a1164 = f9(v0)?;
                let a1165 = {
                    let a1162 = v10;
                    let a1163 = f6({
                        let a1160 = {
                            let a1158 = {
                                let a1140 = 1u64;
                                let a1141 = 1u64;
                                nat_add(a1140, a1141)?
                            };
                            let a1159 = {
                                let a1156 = {
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
                                let a1157 = 1u64;
                                nat_add(a1156, a1157)?
                            };
                            nat_mul(a1158, a1159)?
                        };
                        let a1161 = 1u64;
                        nat_add(a1160, a1161)?
                    })?;
                    nat_mul(a1162, a1163)?
                };
                nat_add(a1164, a1165)?
            };
            let a1192 = {
                let a1188 = v8;
                let a1189 = f6({
                    let a1186 = {
                        let a1184 = {
                            let a1166 = 1u64;
                            let a1167 = 1u64;
                            nat_add(a1166, a1167)?
                        };
                        let a1185 = {
                            let a1182 = {
                                let a1180 = {
                                    let a1168 = 1u64;
                                    let a1169 = 1u64;
                                    nat_add(a1168, a1169)?
                                };
                                let a1181 = {
                                    let a1178 = {
                                        let a1176 = {
                                            let a1170 = 1u64;
                                            let a1171 = 1u64;
                                            nat_add(a1170, a1171)?
                                        };
                                        let a1177 = {
                                            let a1174 = {
                                                let a1172 = 1u64;
                                                let a1173 = 1u64;
                                                nat_add(a1172, a1173)?
                                            };
                                            let a1175 = 1u64;
                                            nat_mul(a1174, a1175)?
                                        };
                                        nat_mul(a1176, a1177)?
                                    };
                                    let a1179 = 1u64;
                                    nat_add(a1178, a1179)?
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
                })?;
                let a1190 = 0u64;
                nat_rem(a1188, a1189, a1190)
            };
            nat_add(a1191, a1192)
        }
    }
}

pub fn f27(v0: Adt0) -> R<Option<Adt0>> {
    let a1194 = 0u64;
    let a1195 = match v0.clone() {
        Adt0::C0(_, h1193, _, _) => {
            h1193
        }
    };
    if if if nat_lt(a1194, a1195) {
        let a1198 = match v0.clone() {
            Adt0::C0(_, h1196, _, _) => {
                h1196
            }
        };
        let a1199 = match v0.clone() {
            Adt0::C0(_, _, _, h1197) => {
                h1197
            }
        };
        nat_lt(a1198, a1199)
    } else {
        false
    } {
        let a1202 = match v0.clone() {
            Adt0::C0(_, _, h1200, _) => {
                h1200
            }
        };
        let a1203 = match v0.clone() {
            Adt0::C0(h1201, _, _, _) => {
                h1201
            }
        };
        nat_lt(a1202, a1203)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a1205 = match v0.clone() {
                Adt0::C0(h1204, _, _, _) => {
                    h1204
                }
            };
            let a1206 = 1u64;
            nat_sub(a1205, a1206)
        }, {
            let a1209 = match v0.clone() {
                Adt0::C0(_, h1207, _, _) => {
                    h1207
                }
            };
            let a1210 = match v0.clone() {
                Adt0::C0(_, h1208, _, _) => {
                    h1208
                }
            };
            nat_add(a1209, a1210)?
        }, match v0.clone() {
            Adt0::C0(_, _, h1211, _) => {
                h1211
            }
        }, match v0 {
            Adt0::C0(_, _, _, h1212) => {
                h1212
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn run(p0: u64, p1: u64) -> R<u64> {
    f0(p0, p1)
}
