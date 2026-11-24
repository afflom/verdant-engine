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
    F19,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F19 => f19(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F30,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F30 => f30(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = f1(v0, f2(v1)?)?;
    let v5: u64 = f3(v2, v4)?;
    f4(v5, v3, 1262485504u64)
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if f5(v0)? {
        true
    } else {
        f5(v1)?
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
            let a66 = f6({
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
        let a88 = f6({
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
        if if f7(v0)? {
            f7(v1)?
        } else {
            false
        } {
            let a89 = f8(v0)?;
            let a90 = f8(v1)?;
            if nat_eq(a89, a90) {
                Ok(v0)
            } else {
                let a177 = {
                    let a155 = {
                        let a131 = {
                            let a129 = {
                                let a91 = 1u64;
                                let a92 = 1u64;
                                nat_add(a91, a92)?
                            };
                            let a130 = {
                                let a127 = {
                                    let a125 = {
                                        let a93 = 1u64;
                                        let a94 = 1u64;
                                        nat_add(a93, a94)?
                                    };
                                    let a126 = {
                                        let a123 = {
                                            let a121 = {
                                                let a95 = 1u64;
                                                let a96 = 1u64;
                                                nat_add(a95, a96)?
                                            };
                                            let a122 = {
                                                let a119 = {
                                                    let a117 = {
                                                        let a97 = 1u64;
                                                        let a98 = 1u64;
                                                        nat_add(a97, a98)?
                                                    };
                                                    let a118 = {
                                                        let a115 = {
                                                            let a113 = {
                                                                let a99 = 1u64;
                                                                let a100 = 1u64;
                                                                nat_add(a99, a100)?
                                                            };
                                                            let a114 = {
                                                                let a111 = {
                                                                    let a109 = {
                                                                        let a101 = 1u64;
                                                                        let a102 = 1u64;
                                                                        nat_add(a101, a102)?
                                                                    };
                                                                    let a110 = {
                                                                        let a107 = {
                                                                            let a105 = {
                                                                                let a103 = 1u64;
                                                                                let a104 = 1u64;
                                                                                nat_add(a103, a104)?
                                                                            };
                                                                            let a106 = 1u64;
                                                                            nat_mul(a105, a106)?
                                                                        };
                                                                        let a108 = 1u64;
                                                                        nat_add(a107, a108)?
                                                                    };
                                                                    nat_mul(a109, a110)?
                                                                };
                                                                let a112 = 1u64;
                                                                nat_add(a111, a112)?
                                                            };
                                                            nat_mul(a113, a114)?
                                                        };
                                                        let a116 = 1u64;
                                                        nat_add(a115, a116)?
                                                    };
                                                    nat_mul(a117, a118)?
                                                };
                                                let a120 = 1u64;
                                                nat_add(a119, a120)?
                                            };
                                            nat_mul(a121, a122)?
                                        };
                                        let a124 = 1u64;
                                        nat_add(a123, a124)?
                                    };
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
                    let a156 = f6({
                        let a153 = {
                            let a151 = {
                                let a133 = 1u64;
                                let a134 = 1u64;
                                nat_add(a133, a134)?
                            };
                            let a152 = {
                                let a149 = {
                                    let a147 = {
                                        let a135 = 1u64;
                                        let a136 = 1u64;
                                        nat_add(a135, a136)?
                                    };
                                    let a148 = {
                                        let a145 = {
                                            let a143 = {
                                                let a137 = 1u64;
                                                let a138 = 1u64;
                                                nat_add(a137, a138)?
                                            };
                                            let a144 = {
                                                let a141 = {
                                                    let a139 = 1u64;
                                                    let a140 = 1u64;
                                                    nat_add(a139, a140)?
                                                };
                                                let a142 = 1u64;
                                                nat_mul(a141, a142)?
                                            };
                                            nat_mul(a143, a144)?
                                        };
                                        let a146 = 1u64;
                                        nat_add(a145, a146)?
                                    };
                                    nat_mul(a147, a148)?
                                };
                                let a150 = 1u64;
                                nat_add(a149, a150)?
                            };
                            nat_mul(a151, a152)?
                        };
                        let a154 = 1u64;
                        nat_add(a153, a154)?
                    })?;
                    nat_mul(a155, a156)?
                };
                let a178 = f6({
                    let a175 = {
                        let a157 = 1u64;
                        let a158 = 1u64;
                        nat_add(a157, a158)?
                    };
                    let a176 = {
                        let a173 = {
                            let a171 = {
                                let a159 = 1u64;
                                let a160 = 1u64;
                                nat_add(a159, a160)?
                            };
                            let a172 = {
                                let a169 = {
                                    let a167 = {
                                        let a161 = 1u64;
                                        let a162 = 1u64;
                                        nat_add(a161, a162)?
                                    };
                                    let a168 = {
                                        let a165 = {
                                            let a163 = 1u64;
                                            let a164 = 1u64;
                                            nat_add(a163, a164)?
                                        };
                                        let a166 = 1u64;
                                        nat_mul(a165, a166)?
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
                })?;
                nat_add(a177, a178)
            }
        } else {
            if f7(v0)? {
                Ok(v0)
            } else {
                if f7(v1)? {
                    Ok(v1)
                } else {
                    let a179 = f9(v1)?;
                    let a180 = f9(v0)?;
                    if nat_le(a179, a180) {
                        f10(v0, v1)
                    } else {
                        f10(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let a227 = {
        let a205 = v0;
        let a206 = f6({
            let a203 = {
                let a201 = {
                    let a181 = 1u64;
                    let a182 = 1u64;
                    nat_add(a181, a182)?
                };
                let a202 = {
                    let a199 = {
                        let a197 = {
                            let a183 = 1u64;
                            let a184 = 1u64;
                            nat_add(a183, a184)?
                        };
                        let a198 = {
                            let a195 = {
                                let a193 = {
                                    let a185 = 1u64;
                                    let a186 = 1u64;
                                    nat_add(a185, a186)?
                                };
                                let a194 = {
                                    let a191 = {
                                        let a189 = {
                                            let a187 = 1u64;
                                            let a188 = 1u64;
                                            nat_add(a187, a188)?
                                        };
                                        let a190 = 1u64;
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
                nat_mul(a201, a202)?
            };
            let a204 = 1u64;
            nat_add(a203, a204)?
        })?;
        nat_add(a205, a206)?
    };
    let a228 = f6({
        let a225 = {
            let a207 = 1u64;
            let a208 = 1u64;
            nat_add(a207, a208)?
        };
        let a226 = {
            let a223 = {
                let a209 = 1u64;
                let a210 = 1u64;
                nat_add(a209, a210)?
            };
            let a224 = {
                let a221 = {
                    let a211 = 1u64;
                    let a212 = 1u64;
                    nat_add(a211, a212)?
                };
                let a222 = {
                    let a219 = {
                        let a213 = 1u64;
                        let a214 = 1u64;
                        nat_add(a213, a214)?
                    };
                    let a220 = {
                        let a217 = {
                            let a215 = 1u64;
                            let a216 = 1u64;
                            nat_add(a215, a216)?
                        };
                        let a218 = 1u64;
                        nat_mul(a217, a218)?
                    };
                    nat_mul(a219, a220)?
                };
                nat_mul(a221, a222)?
            };
            nat_mul(a223, a224)?
        };
        nat_mul(a225, a226)?
    })?;
    let a229 = 0u64;
    Ok(nat_rem(a227, a228, a229))
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    if if if if f5(v0)? {
        true
    } else {
        f5(v1)?
    } {
        true
    } else {
        if f7(v0)? {
            f11(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f7(v1)? {
            f11(v0)?
        } else {
            false
        }
    } {
        let a316 = {
            let a294 = {
                let a270 = {
                    let a268 = {
                        let a230 = 1u64;
                        let a231 = 1u64;
                        nat_add(a230, a231)?
                    };
                    let a269 = {
                        let a266 = {
                            let a264 = {
                                let a232 = 1u64;
                                let a233 = 1u64;
                                nat_add(a232, a233)?
                            };
                            let a265 = {
                                let a262 = {
                                    let a260 = {
                                        let a234 = 1u64;
                                        let a235 = 1u64;
                                        nat_add(a234, a235)?
                                    };
                                    let a261 = {
                                        let a258 = {
                                            let a256 = {
                                                let a236 = 1u64;
                                                let a237 = 1u64;
                                                nat_add(a236, a237)?
                                            };
                                            let a257 = {
                                                let a254 = {
                                                    let a252 = {
                                                        let a238 = 1u64;
                                                        let a239 = 1u64;
                                                        nat_add(a238, a239)?
                                                    };
                                                    let a253 = {
                                                        let a250 = {
                                                            let a248 = {
                                                                let a240 = 1u64;
                                                                let a241 = 1u64;
                                                                nat_add(a240, a241)?
                                                            };
                                                            let a249 = {
                                                                let a246 = {
                                                                    let a244 = {
                                                                        let a242 = 1u64;
                                                                        let a243 = 1u64;
                                                                        nat_add(a242, a243)?
                                                                    };
                                                                    let a245 = 1u64;
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
                                                    nat_mul(a252, a253)?
                                                };
                                                let a255 = 1u64;
                                                nat_add(a254, a255)?
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
                    };
                    nat_mul(a268, a269)?
                };
                let a271 = 1u64;
                nat_add(a270, a271)?
            };
            let a295 = f6({
                let a292 = {
                    let a290 = {
                        let a272 = 1u64;
                        let a273 = 1u64;
                        nat_add(a272, a273)?
                    };
                    let a291 = {
                        let a288 = {
                            let a286 = {
                                let a274 = 1u64;
                                let a275 = 1u64;
                                nat_add(a274, a275)?
                            };
                            let a287 = {
                                let a284 = {
                                    let a282 = {
                                        let a276 = 1u64;
                                        let a277 = 1u64;
                                        nat_add(a276, a277)?
                                    };
                                    let a283 = {
                                        let a280 = {
                                            let a278 = 1u64;
                                            let a279 = 1u64;
                                            nat_add(a278, a279)?
                                        };
                                        let a281 = 1u64;
                                        nat_mul(a280, a281)?
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
                    nat_mul(a290, a291)?
                };
                let a293 = 1u64;
                nat_add(a292, a293)?
            })?;
            nat_mul(a294, a295)?
        };
        let a317 = f6({
            let a314 = {
                let a296 = 1u64;
                let a297 = 1u64;
                nat_add(a296, a297)?
            };
            let a315 = {
                let a312 = {
                    let a310 = {
                        let a298 = 1u64;
                        let a299 = 1u64;
                        nat_add(a298, a299)?
                    };
                    let a311 = {
                        let a308 = {
                            let a306 = {
                                let a300 = 1u64;
                                let a301 = 1u64;
                                nat_add(a300, a301)?
                            };
                            let a307 = {
                                let a304 = {
                                    let a302 = 1u64;
                                    let a303 = 1u64;
                                    nat_add(a302, a303)?
                                };
                                let a305 = 1u64;
                                nat_mul(a304, a305)?
                            };
                            nat_mul(a306, a307)?
                        };
                        let a309 = 1u64;
                        nat_add(a308, a309)?
                    };
                    nat_mul(a310, a311)?
                };
                let a313 = 1u64;
                nat_add(a312, a313)?
            };
            nat_mul(a314, a315)?
        })?;
        nat_add(a316, a317)
    } else {
        if if f7(v0)? {
            true
        } else {
            f7(v1)?
        } {
            f12(f13(v0, v1)?)
        } else {
            if if f11(v0)? {
                true
            } else {
                f11(v1)?
            } {
                f14(f13(v0, v1)?)
            } else {
                f15(v0, v1)
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f1(v0, v2)?;
    let v4: u64 = {
        let a318 = v3;
        let a319 = v2;
        nat_sub(a318, a319)
    };
    let a320 = v4;
    let a321 = v1;
    if nat_lt(a320, a321) {
        Ok(v4)
    } else {
        Ok(v1)
    }
}

pub fn f5(v0: u64) -> R<bool> {
    let a364 = f16(v0)?;
    let a365 = {
        let a362 = {
            let a360 = {
                let a322 = 1u64;
                let a323 = 1u64;
                nat_add(a322, a323)?
            };
            let a361 = {
                let a358 = {
                    let a356 = {
                        let a324 = 1u64;
                        let a325 = 1u64;
                        nat_add(a324, a325)?
                    };
                    let a357 = {
                        let a354 = {
                            let a352 = {
                                let a326 = 1u64;
                                let a327 = 1u64;
                                nat_add(a326, a327)?
                            };
                            let a353 = {
                                let a350 = {
                                    let a348 = {
                                        let a328 = 1u64;
                                        let a329 = 1u64;
                                        nat_add(a328, a329)?
                                    };
                                    let a349 = {
                                        let a346 = {
                                            let a344 = {
                                                let a330 = 1u64;
                                                let a331 = 1u64;
                                                nat_add(a330, a331)?
                                            };
                                            let a345 = {
                                                let a342 = {
                                                    let a340 = {
                                                        let a332 = 1u64;
                                                        let a333 = 1u64;
                                                        nat_add(a332, a333)?
                                                    };
                                                    let a341 = {
                                                        let a338 = {
                                                            let a336 = {
                                                                let a334 = 1u64;
                                                                let a335 = 1u64;
                                                                nat_add(a334, a335)?
                                                            };
                                                            let a337 = 1u64;
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
            nat_mul(a360, a361)?
        };
        let a363 = 1u64;
        nat_add(a362, a363)?
    };
    if nat_eq(a364, a365) {
        let a366 = 0u64;
        let a367 = f17(v0)?;
        Ok(nat_lt(a366, a367))
    } else {
        Ok(false)
    }
}

pub fn f6(v0: u64) -> R<u64> {
    f18(Fn0::F19, v0, 1u64)
}

pub fn f7(v0: u64) -> R<bool> {
    let a410 = f16(v0)?;
    let a411 = {
        let a408 = {
            let a406 = {
                let a368 = 1u64;
                let a369 = 1u64;
                nat_add(a368, a369)?
            };
            let a407 = {
                let a404 = {
                    let a402 = {
                        let a370 = 1u64;
                        let a371 = 1u64;
                        nat_add(a370, a371)?
                    };
                    let a403 = {
                        let a400 = {
                            let a398 = {
                                let a372 = 1u64;
                                let a373 = 1u64;
                                nat_add(a372, a373)?
                            };
                            let a399 = {
                                let a396 = {
                                    let a394 = {
                                        let a374 = 1u64;
                                        let a375 = 1u64;
                                        nat_add(a374, a375)?
                                    };
                                    let a395 = {
                                        let a392 = {
                                            let a390 = {
                                                let a376 = 1u64;
                                                let a377 = 1u64;
                                                nat_add(a376, a377)?
                                            };
                                            let a391 = {
                                                let a388 = {
                                                    let a386 = {
                                                        let a378 = 1u64;
                                                        let a379 = 1u64;
                                                        nat_add(a378, a379)?
                                                    };
                                                    let a387 = {
                                                        let a384 = {
                                                            let a382 = {
                                                                let a380 = 1u64;
                                                                let a381 = 1u64;
                                                                nat_add(a380, a381)?
                                                            };
                                                            let a383 = 1u64;
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
    if nat_eq(a410, a411) {
        let a412 = f17(v0)?;
        let a413 = 0u64;
        Ok(nat_eq(a412, a413))
    } else {
        Ok(false)
    }
}

pub fn f8(v0: u64) -> R<u64> {
    let a438 = v0;
    let a439 = f6({
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
    })?;
    let a440 = 0u64;
    Ok(nat_quot(a438, a439, a440))
}

pub fn f9(v0: u64) -> R<u64> {
    let a465 = v0;
    let a466 = f6({
        let a463 = {
            let a461 = {
                let a441 = 1u64;
                let a442 = 1u64;
                nat_add(a441, a442)?
            };
            let a462 = {
                let a459 = {
                    let a457 = {
                        let a443 = 1u64;
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    };
                    let a458 = {
                        let a455 = {
                            let a453 = {
                                let a445 = 1u64;
                                let a446 = 1u64;
                                nat_add(a445, a446)?
                            };
                            let a454 = {
                                let a451 = {
                                    let a449 = {
                                        let a447 = 1u64;
                                        let a448 = 1u64;
                                        nat_add(a447, a448)?
                                    };
                                    let a450 = 1u64;
                                    nat_mul(a449, a450)?
                                };
                                let a452 = 1u64;
                                nat_add(a451, a452)?
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
        };
        let a464 = 1u64;
        nat_add(a463, a464)?
    })?;
    let a467 = 0u64;
    Ok(nat_rem(a465, a466, a467))
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f20(v0)?;
    let v3: u64 = {
        let a480 = f21(v0)?;
        let a481 = {
            let a478 = {
                let a468 = 1u64;
                let a469 = 1u64;
                nat_add(a468, a469)?
            };
            let a479 = {
                let a476 = {
                    let a470 = 1u64;
                    let a471 = 1u64;
                    nat_add(a470, a471)?
                };
                let a477 = {
                    let a474 = {
                        let a472 = 1u64;
                        let a473 = 1u64;
                        nat_add(a472, a473)?
                    };
                    let a475 = 1u64;
                    nat_mul(a474, a475)?
                };
                nat_mul(a476, a477)?
            };
            nat_mul(a478, a479)?
        };
        nat_mul(a480, a481)?
    };
    let v4: u64 = f22({
        let a494 = f21(v1)?;
        let a495 = {
            let a492 = {
                let a482 = 1u64;
                let a483 = 1u64;
                nat_add(a482, a483)?
            };
            let a493 = {
                let a490 = {
                    let a484 = 1u64;
                    let a485 = 1u64;
                    nat_add(a484, a485)?
                };
                let a491 = {
                    let a488 = {
                        let a486 = 1u64;
                        let a487 = 1u64;
                        nat_add(a486, a487)?
                    };
                    let a489 = 1u64;
                    nat_mul(a488, a489)?
                };
                nat_mul(a490, a491)?
            };
            nat_mul(a492, a493)?
        };
        nat_mul(a494, a495)?
    }, {
        let a496 = v2;
        let a497 = f20(v1)?;
        nat_sub(a496, a497)
    })?;
    let a498 = f8(v0)?;
    let a499 = f8(v1)?;
    if nat_eq(a498, a499) {
        f23(f8(v0)?, v2, {
            let a500 = v3;
            let a501 = v4;
            nat_add(a500, a501)?
        })
    } else {
        let a502 = v3;
        let a503 = v4;
        if nat_eq(a502, a503) {
            Ok(0u64)
        } else {
            f23(f8(v0)?, v2, {
                let a504 = v3;
                let a505 = v4;
                nat_sub(a504, a505)
            })
        }
    }
}

pub fn f11(v0: u64) -> R<bool> {
    let a506 = f9(v0)?;
    let a507 = 0u64;
    Ok(nat_eq(a506, a507))
}

pub fn f12(v0: u64) -> R<u64> {
    let a574 = f14(v0)?;
    let a575 = {
        let a572 = {
            let a548 = {
                let a546 = {
                    let a508 = 1u64;
                    let a509 = 1u64;
                    nat_add(a508, a509)?
                };
                let a547 = {
                    let a544 = {
                        let a542 = {
                            let a510 = 1u64;
                            let a511 = 1u64;
                            nat_add(a510, a511)?
                        };
                        let a543 = {
                            let a540 = {
                                let a538 = {
                                    let a512 = 1u64;
                                    let a513 = 1u64;
                                    nat_add(a512, a513)?
                                };
                                let a539 = {
                                    let a536 = {
                                        let a534 = {
                                            let a514 = 1u64;
                                            let a515 = 1u64;
                                            nat_add(a514, a515)?
                                        };
                                        let a535 = {
                                            let a532 = {
                                                let a530 = {
                                                    let a516 = 1u64;
                                                    let a517 = 1u64;
                                                    nat_add(a516, a517)?
                                                };
                                                let a531 = {
                                                    let a528 = {
                                                        let a526 = {
                                                            let a518 = 1u64;
                                                            let a519 = 1u64;
                                                            nat_add(a518, a519)?
                                                        };
                                                        let a527 = {
                                                            let a524 = {
                                                                let a522 = {
                                                                    let a520 = 1u64;
                                                                    let a521 = 1u64;
                                                                    nat_add(a520, a521)?
                                                                };
                                                                let a523 = 1u64;
                                                                nat_mul(a522, a523)?
                                                            };
                                                            let a525 = 1u64;
                                                            nat_add(a524, a525)?
                                                        };
                                                        nat_mul(a526, a527)?
                                                    };
                                                    let a529 = 1u64;
                                                    nat_add(a528, a529)?
                                                };
                                                nat_mul(a530, a531)?
                                            };
                                            let a533 = 1u64;
                                            nat_add(a532, a533)?
                                        };
                                        nat_mul(a534, a535)?
                                    };
                                    let a537 = 1u64;
                                    nat_add(a536, a537)?
                                };
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
        let a573 = f6({
            let a570 = {
                let a568 = {
                    let a550 = 1u64;
                    let a551 = 1u64;
                    nat_add(a550, a551)?
                };
                let a569 = {
                    let a566 = {
                        let a564 = {
                            let a552 = 1u64;
                            let a553 = 1u64;
                            nat_add(a552, a553)?
                        };
                        let a565 = {
                            let a562 = {
                                let a560 = {
                                    let a554 = 1u64;
                                    let a555 = 1u64;
                                    nat_add(a554, a555)?
                                };
                                let a561 = {
                                    let a558 = {
                                        let a556 = 1u64;
                                        let a557 = 1u64;
                                        nat_add(a556, a557)?
                                    };
                                    let a559 = 1u64;
                                    nat_mul(a558, a559)?
                                };
                                nat_mul(a560, a561)?
                            };
                            let a563 = 1u64;
                            nat_add(a562, a563)?
                        };
                        nat_mul(a564, a565)?
                    };
                    let a567 = 1u64;
                    nat_add(a566, a567)?
                };
                nat_mul(a568, a569)?
            };
            let a571 = 1u64;
            nat_add(a570, a571)?
        })?;
        nat_mul(a572, a573)?
    };
    nat_add(a574, a575)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let a582 = {
        let a576 = f8(v0)?;
        let a577 = f8(v1)?;
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
    let a584 = 0u64;
    Ok(nat_rem(a582, a583, a584))
}

pub fn f14(v0: u64) -> R<u64> {
    let a609 = v0;
    let a610 = f6({
        let a607 = {
            let a605 = {
                let a585 = 1u64;
                let a586 = 1u64;
                nat_add(a585, a586)?
            };
            let a606 = {
                let a603 = {
                    let a601 = {
                        let a587 = 1u64;
                        let a588 = 1u64;
                        nat_add(a587, a588)?
                    };
                    let a602 = {
                        let a599 = {
                            let a597 = {
                                let a589 = 1u64;
                                let a590 = 1u64;
                                nat_add(a589, a590)?
                            };
                            let a598 = {
                                let a595 = {
                                    let a593 = {
                                        let a591 = 1u64;
                                        let a592 = 1u64;
                                        nat_add(a591, a592)?
                                    };
                                    let a594 = 1u64;
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
    nat_mul(a609, a610)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f24(v0)?;
    let v3: Adt0 = f24(v1)?;
    f23(f13(v0, v1)?, {
        let a665 = {
            let a613 = match v2.clone() {
                Adt0::C0(h611, _, _, _) => {
                    h611
                }
            };
            let a614 = match v3.clone() {
                Adt0::C0(h612, _, _, _) => {
                    h612
                }
            };
            nat_add(a613, a614)?
        };
        let a666 = {
            let a663 = {
                let a661 = {
                    let a615 = 1u64;
                    let a616 = 1u64;
                    nat_add(a615, a616)?
                };
                let a662 = {
                    let a659 = {
                        let a657 = {
                            let a617 = 1u64;
                            let a618 = 1u64;
                            nat_add(a617, a618)?
                        };
                        let a658 = {
                            let a655 = {
                                let a653 = {
                                    let a619 = 1u64;
                                    let a620 = 1u64;
                                    nat_add(a619, a620)?
                                };
                                let a654 = {
                                    let a651 = {
                                        let a649 = {
                                            let a621 = 1u64;
                                            let a622 = 1u64;
                                            nat_add(a621, a622)?
                                        };
                                        let a650 = {
                                            let a647 = {
                                                let a645 = {
                                                    let a623 = 1u64;
                                                    let a624 = 1u64;
                                                    nat_add(a623, a624)?
                                                };
                                                let a646 = {
                                                    let a643 = {
                                                        let a641 = {
                                                            let a625 = 1u64;
                                                            let a626 = 1u64;
                                                            nat_add(a625, a626)?
                                                        };
                                                        let a642 = {
                                                            let a639 = {
                                                                let a637 = {
                                                                    let a627 = 1u64;
                                                                    let a628 = 1u64;
                                                                    nat_add(a627, a628)?
                                                                };
                                                                let a638 = {
                                                                    let a635 = {
                                                                        let a629 = 1u64;
                                                                        let a630 = 1u64;
                                                                        nat_add(a629, a630)?
                                                                    };
                                                                    let a636 = {
                                                                        let a633 = {
                                                                            let a631 = 1u64;
                                                                            let a632 = 1u64;
                                                                            nat_add(a631, a632)?
                                                                        };
                                                                        let a634 = 1u64;
                                                                        nat_mul(a633, a634)?
                                                                    };
                                                                    nat_mul(a635, a636)?
                                                                };
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
        nat_sub(a665, a666)
    }, f22({
        let a669 = match v2 {
            Adt0::C0(_, h667, _, _) => {
                h667
            }
        };
        let a670 = match v3 {
            Adt0::C0(_, h668, _, _) => {
                h668
            }
        };
        nat_mul(a669, a670)?
    }, {
        let a687 = {
            let a671 = 1u64;
            let a672 = 1u64;
            nat_add(a671, a672)?
        };
        let a688 = {
            let a685 = {
                let a673 = 1u64;
                let a674 = 1u64;
                nat_add(a673, a674)?
            };
            let a686 = {
                let a683 = {
                    let a681 = {
                        let a675 = 1u64;
                        let a676 = 1u64;
                        nat_add(a675, a676)?
                    };
                    let a682 = {
                        let a679 = {
                            let a677 = 1u64;
                            let a678 = 1u64;
                            nat_add(a677, a678)?
                        };
                        let a680 = 1u64;
                        nat_mul(a679, a680)?
                    };
                    nat_mul(a681, a682)?
                };
                let a684 = 1u64;
                nat_add(a683, a684)?
            };
            nat_mul(a685, a686)?
        };
        nat_mul(a687, a688)?
    })?)
}

pub fn f16(v0: u64) -> R<u64> {
    let a711 = f9(v0)?;
    let a712 = f6({
        let a709 = {
            let a707 = {
                let a689 = 1u64;
                let a690 = 1u64;
                nat_add(a689, a690)?
            };
            let a708 = {
                let a705 = {
                    let a703 = {
                        let a691 = 1u64;
                        let a692 = 1u64;
                        nat_add(a691, a692)?
                    };
                    let a704 = {
                        let a701 = {
                            let a699 = {
                                let a693 = 1u64;
                                let a694 = 1u64;
                                nat_add(a693, a694)?
                            };
                            let a700 = {
                                let a697 = {
                                    let a695 = 1u64;
                                    let a696 = 1u64;
                                    nat_add(a695, a696)?
                                };
                                let a698 = 1u64;
                                nat_mul(a697, a698)?
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
    })?;
    let a713 = 0u64;
    Ok(nat_quot(a711, a712, a713))
}

pub fn f17(v0: u64) -> R<u64> {
    let a736 = v0;
    let a737 = f6({
        let a734 = {
            let a732 = {
                let a714 = 1u64;
                let a715 = 1u64;
                nat_add(a714, a715)?
            };
            let a733 = {
                let a730 = {
                    let a728 = {
                        let a716 = 1u64;
                        let a717 = 1u64;
                        nat_add(a716, a717)?
                    };
                    let a729 = {
                        let a726 = {
                            let a724 = {
                                let a718 = 1u64;
                                let a719 = 1u64;
                                nat_add(a718, a719)?
                            };
                            let a725 = {
                                let a722 = {
                                    let a720 = 1u64;
                                    let a721 = 1u64;
                                    nat_add(a720, a721)?
                                };
                                let a723 = 1u64;
                                nat_mul(a722, a723)?
                            };
                            nat_mul(a724, a725)?
                        };
                        let a727 = 1u64;
                        nat_add(a726, a727)?
                    };
                    nat_mul(a728, a729)?
                };
                let a731 = 1u64;
                nat_add(a730, a731)?
            };
            nat_mul(a732, a733)?
        };
        let a735 = 1u64;
        nat_add(a734, a735)?
    })?;
    let a738 = 0u64;
    Ok(nat_rem(a736, a737, a738))
}

pub fn f18(v0: Fn0, p742: u64, p743: u64) -> R<u64> {
    let m741 = (p742, p743);
    {
        let mut m741 = m741;
        loop {
            let step = {
                let (v1, v2) = m741;
                let m739 = v1;
                if m739 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m739 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c740: Fn0 = v0.clone();
                        c740.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64) -> R<u64> {
    let a744 = v0;
    let a745 = v0;
    nat_add(a744, a745)
}

pub fn f20(v0: u64) -> R<u64> {
    let a784 = {
        let a746 = f16(v0)?;
        let a747 = 0u64;
        if nat_eq(a746, a747) {
            1u64
        } else {
            f16(v0)?
        }
    };
    let a785 = {
        let a782 = {
            let a748 = 1u64;
            let a749 = 1u64;
            nat_add(a748, a749)?
        };
        let a783 = {
            let a780 = {
                let a750 = 1u64;
                let a751 = 1u64;
                nat_add(a750, a751)?
            };
            let a781 = {
                let a778 = {
                    let a752 = 1u64;
                    let a753 = 1u64;
                    nat_add(a752, a753)?
                };
                let a779 = {
                    let a776 = {
                        let a754 = 1u64;
                        let a755 = 1u64;
                        nat_add(a754, a755)?
                    };
                    let a777 = {
                        let a774 = {
                            let a756 = 1u64;
                            let a757 = 1u64;
                            nat_add(a756, a757)?
                        };
                        let a775 = {
                            let a772 = {
                                let a758 = 1u64;
                                let a759 = 1u64;
                                nat_add(a758, a759)?
                            };
                            let a773 = {
                                let a770 = {
                                    let a760 = 1u64;
                                    let a761 = 1u64;
                                    nat_add(a760, a761)?
                                };
                                let a771 = {
                                    let a768 = {
                                        let a762 = 1u64;
                                        let a763 = 1u64;
                                        nat_add(a762, a763)?
                                    };
                                    let a769 = {
                                        let a766 = {
                                            let a764 = 1u64;
                                            let a765 = 1u64;
                                            nat_add(a764, a765)?
                                        };
                                        let a767 = 1u64;
                                        nat_mul(a766, a767)?
                                    };
                                    nat_mul(a768, a769)?
                                };
                                nat_mul(a770, a771)?
                            };
                            nat_mul(a772, a773)?
                        };
                        nat_mul(a774, a775)?
                    };
                    nat_mul(a776, a777)?
                };
                nat_mul(a778, a779)?
            };
            nat_mul(a780, a781)?
        };
        nat_mul(a782, a783)?
    };
    nat_add(a784, a785)
}

pub fn f21(v0: u64) -> R<u64> {
    let a810 = f17(v0)?;
    let a811 = {
        let a786 = f16(v0)?;
        let a787 = 0u64;
        if nat_eq(a786, a787) {
            0u64
        } else {
            f6({
                let a808 = {
                    let a806 = {
                        let a788 = 1u64;
                        let a789 = 1u64;
                        nat_add(a788, a789)?
                    };
                    let a807 = {
                        let a804 = {
                            let a802 = {
                                let a790 = 1u64;
                                let a791 = 1u64;
                                nat_add(a790, a791)?
                            };
                            let a803 = {
                                let a800 = {
                                    let a798 = {
                                        let a792 = 1u64;
                                        let a793 = 1u64;
                                        nat_add(a792, a793)?
                                    };
                                    let a799 = {
                                        let a796 = {
                                            let a794 = 1u64;
                                            let a795 = 1u64;
                                            nat_add(a794, a795)?
                                        };
                                        let a797 = 1u64;
                                        nat_mul(a796, a797)?
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
            })?
        }
    };
    nat_add(a810, a811)
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let a812 = v1;
    let a813 = 0u64;
    if nat_eq(a812, a813) {
        Ok(v0)
    } else {
        let a838 = {
            let a836 = {
                let a814 = 1u64;
                let a815 = 1u64;
                nat_add(a814, a815)?
            };
            let a837 = {
                let a834 = {
                    let a816 = 1u64;
                    let a817 = 1u64;
                    nat_add(a816, a817)?
                };
                let a835 = {
                    let a832 = {
                        let a818 = 1u64;
                        let a819 = 1u64;
                        nat_add(a818, a819)?
                    };
                    let a833 = {
                        let a830 = {
                            let a820 = 1u64;
                            let a821 = 1u64;
                            nat_add(a820, a821)?
                        };
                        let a831 = {
                            let a828 = {
                                let a822 = 1u64;
                                let a823 = 1u64;
                                nat_add(a822, a823)?
                            };
                            let a829 = {
                                let a826 = {
                                    let a824 = 1u64;
                                    let a825 = 1u64;
                                    nat_add(a824, a825)?
                                };
                                let a827 = 1u64;
                                nat_mul(a826, a827)?
                            };
                            nat_mul(a828, a829)?
                        };
                        nat_mul(a830, a831)?
                    };
                    nat_mul(a832, a833)?
                };
                nat_mul(a834, a835)?
            };
            nat_mul(a836, a837)?
        };
        let a839 = v1;
        if nat_le(a838, a839) {
            let a840 = v0;
            let a841 = 0u64;
            if nat_eq(a840, a841) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f6(v1)?;
            f25({
                let a842 = v0;
                let a843 = v2;
                let a844 = 0u64;
                nat_quot(a842, a843, a844)
            }, {
                let a848 = 0u64;
                let a849 = {
                    let a845 = v0;
                    let a846 = v2;
                    let a847 = 0u64;
                    nat_rem(a845, a846, a847)
                };
                nat_lt(a848, a849)
            })
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a872 = f6({
        let a870 = {
            let a868 = {
                let a850 = 1u64;
                let a851 = 1u64;
                nat_add(a850, a851)?
            };
            let a869 = {
                let a866 = {
                    let a864 = {
                        let a852 = 1u64;
                        let a853 = 1u64;
                        nat_add(a852, a853)?
                    };
                    let a865 = {
                        let a862 = {
                            let a854 = 1u64;
                            let a855 = 1u64;
                            nat_add(a854, a855)?
                        };
                        let a863 = {
                            let a860 = {
                                let a858 = {
                                    let a856 = 1u64;
                                    let a857 = 1u64;
                                    nat_add(a856, a857)?
                                };
                                let a859 = 1u64;
                                nat_mul(a858, a859)?
                            };
                            let a861 = 1u64;
                            nat_add(a860, a861)?
                        };
                        nat_mul(a862, a863)?
                    };
                    nat_mul(a864, a865)?
                };
                let a867 = 1u64;
                nat_add(a866, a867)?
            };
            nat_mul(a868, a869)?
        };
        let a871 = 1u64;
        nat_add(a870, a871)?
    })?;
    let a873 = v2;
    if nat_le(a872, a873) {
        f26(v0, {
            let a874 = v1;
            let a875 = 1u64;
            nat_add(a874, a875)?
        }, f22(v2, 1u64)?)
    } else {
        let v3: Adt0 = f27(v1, v2, {
            let a912 = {
                let a910 = {
                    let a876 = 1u64;
                    let a877 = 1u64;
                    nat_add(a876, a877)?
                };
                let a911 = {
                    let a908 = {
                        let a878 = 1u64;
                        let a879 = 1u64;
                        nat_add(a878, a879)?
                    };
                    let a909 = {
                        let a906 = {
                            let a880 = 1u64;
                            let a881 = 1u64;
                            nat_add(a880, a881)?
                        };
                        let a907 = {
                            let a904 = {
                                let a882 = 1u64;
                                let a883 = 1u64;
                                nat_add(a882, a883)?
                            };
                            let a905 = {
                                let a902 = {
                                    let a884 = 1u64;
                                    let a885 = 1u64;
                                    nat_add(a884, a885)?
                                };
                                let a903 = {
                                    let a900 = {
                                        let a886 = 1u64;
                                        let a887 = 1u64;
                                        nat_add(a886, a887)?
                                    };
                                    let a901 = {
                                        let a898 = {
                                            let a888 = 1u64;
                                            let a889 = 1u64;
                                            nat_add(a888, a889)?
                                        };
                                        let a899 = {
                                            let a896 = {
                                                let a890 = 1u64;
                                                let a891 = 1u64;
                                                nat_add(a890, a891)?
                                            };
                                            let a897 = {
                                                let a894 = {
                                                    let a892 = 1u64;
                                                    let a893 = 1u64;
                                                    nat_add(a892, a893)?
                                                };
                                                let a895 = 1u64;
                                                nat_mul(a894, a895)?
                                            };
                                            nat_mul(a896, a897)?
                                        };
                                        nat_mul(a898, a899)?
                                    };
                                    nat_mul(a900, a901)?
                                };
                                nat_mul(a902, a903)?
                            };
                            nat_mul(a904, a905)?
                        };
                        nat_mul(a906, a907)?
                    };
                    nat_mul(a908, a909)?
                };
                nat_mul(a910, a911)?
            };
            let a913 = 1u64;
            nat_add(a912, a913)?
        }, f6({
            let a932 = {
                let a914 = 1u64;
                let a915 = 1u64;
                nat_add(a914, a915)?
            };
            let a933 = {
                let a930 = {
                    let a928 = {
                        let a916 = 1u64;
                        let a917 = 1u64;
                        nat_add(a916, a917)?
                    };
                    let a929 = {
                        let a926 = {
                            let a918 = 1u64;
                            let a919 = 1u64;
                            nat_add(a918, a919)?
                        };
                        let a927 = {
                            let a924 = {
                                let a922 = {
                                    let a920 = 1u64;
                                    let a921 = 1u64;
                                    nat_add(a920, a921)?
                                };
                                let a923 = 1u64;
                                nat_mul(a922, a923)?
                            };
                            let a925 = 1u64;
                            nat_add(a924, a925)?
                        };
                        nat_mul(a926, a927)?
                    };
                    nat_mul(a928, a929)?
                };
                let a931 = 1u64;
                nat_add(a930, a931)?
            };
            nat_mul(a932, a933)?
        })?)?;
        f26(v0, match v3.clone() {
            Adt0::C0(h934, _, _, _) => {
                h934
            }
        }, match v3 {
            Adt0::C0(_, h935, _, _) => {
                h935
            }
        })
    }
}

pub fn f24(v0: u64) -> R<Adt0> {
    f27(f20(v0)?, f21(v0)?, 0u64, f6({
        let a956 = {
            let a954 = {
                let a936 = 1u64;
                let a937 = 1u64;
                nat_add(a936, a937)?
            };
            let a955 = {
                let a952 = {
                    let a950 = {
                        let a938 = 1u64;
                        let a939 = 1u64;
                        nat_add(a938, a939)?
                    };
                    let a951 = {
                        let a948 = {
                            let a946 = {
                                let a940 = 1u64;
                                let a941 = 1u64;
                                nat_add(a940, a941)?
                            };
                            let a947 = {
                                let a944 = {
                                    let a942 = 1u64;
                                    let a943 = 1u64;
                                    nat_add(a942, a943)?
                                };
                                let a945 = 1u64;
                                nat_mul(a944, a945)?
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
            nat_mul(a954, a955)?
        };
        let a957 = 1u64;
        nat_add(a956, a957)?
    })?)
}

pub fn f25(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a965 = {
            let a962 = v0;
            let a963 = {
                let a960 = {
                    let a958 = 1u64;
                    let a959 = 1u64;
                    nat_add(a958, a959)?
                };
                let a961 = 1u64;
                nat_mul(a960, a961)?
            };
            let a964 = 0u64;
            nat_rem(a962, a963, a964)
        };
        let a966 = 0u64;
        nat_eq(a965, a966)
    } else {
        false
    } {
        let a967 = v0;
        let a968 = 1u64;
        nat_add(a967, a968)
    } else {
        Ok(v0)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1007 = v1;
    let a1008 = {
        let a1005 = {
            let a1003 = {
                let a969 = 1u64;
                let a970 = 1u64;
                nat_add(a969, a970)?
            };
            let a1004 = {
                let a1001 = {
                    let a971 = 1u64;
                    let a972 = 1u64;
                    nat_add(a971, a972)?
                };
                let a1002 = {
                    let a999 = {
                        let a973 = 1u64;
                        let a974 = 1u64;
                        nat_add(a973, a974)?
                    };
                    let a1000 = {
                        let a997 = {
                            let a975 = 1u64;
                            let a976 = 1u64;
                            nat_add(a975, a976)?
                        };
                        let a998 = {
                            let a995 = {
                                let a977 = 1u64;
                                let a978 = 1u64;
                                nat_add(a977, a978)?
                            };
                            let a996 = {
                                let a993 = {
                                    let a979 = 1u64;
                                    let a980 = 1u64;
                                    nat_add(a979, a980)?
                                };
                                let a994 = {
                                    let a991 = {
                                        let a981 = 1u64;
                                        let a982 = 1u64;
                                        nat_add(a981, a982)?
                                    };
                                    let a992 = {
                                        let a989 = {
                                            let a983 = 1u64;
                                            let a984 = 1u64;
                                            nat_add(a983, a984)?
                                        };
                                        let a990 = {
                                            let a987 = {
                                                let a985 = 1u64;
                                                let a986 = 1u64;
                                                nat_add(a985, a986)?
                                            };
                                            let a988 = 1u64;
                                            nat_mul(a987, a988)?
                                        };
                                        nat_mul(a989, a990)?
                                    };
                                    nat_mul(a991, a992)?
                                };
                                nat_mul(a993, a994)?
                            };
                            nat_mul(a995, a996)?
                        };
                        nat_mul(a997, a998)?
                    };
                    nat_mul(a999, a1000)?
                };
                nat_mul(a1001, a1002)?
            };
            nat_mul(a1003, a1004)?
        };
        let a1006 = 1u64;
        nat_add(a1005, a1006)?
    };
    if nat_lt(a1007, a1008) {
        f28(v0, {
            let a1045 = {
                let a1043 = {
                    let a1009 = 1u64;
                    let a1010 = 1u64;
                    nat_add(a1009, a1010)?
                };
                let a1044 = {
                    let a1041 = {
                        let a1011 = 1u64;
                        let a1012 = 1u64;
                        nat_add(a1011, a1012)?
                    };
                    let a1042 = {
                        let a1039 = {
                            let a1013 = 1u64;
                            let a1014 = 1u64;
                            nat_add(a1013, a1014)?
                        };
                        let a1040 = {
                            let a1037 = {
                                let a1015 = 1u64;
                                let a1016 = 1u64;
                                nat_add(a1015, a1016)?
                            };
                            let a1038 = {
                                let a1035 = {
                                    let a1017 = 1u64;
                                    let a1018 = 1u64;
                                    nat_add(a1017, a1018)?
                                };
                                let a1036 = {
                                    let a1033 = {
                                        let a1019 = 1u64;
                                        let a1020 = 1u64;
                                        nat_add(a1019, a1020)?
                                    };
                                    let a1034 = {
                                        let a1031 = {
                                            let a1021 = 1u64;
                                            let a1022 = 1u64;
                                            nat_add(a1021, a1022)?
                                        };
                                        let a1032 = {
                                            let a1029 = {
                                                let a1023 = 1u64;
                                                let a1024 = 1u64;
                                                nat_add(a1023, a1024)?
                                            };
                                            let a1030 = {
                                                let a1027 = {
                                                    let a1025 = 1u64;
                                                    let a1026 = 1u64;
                                                    nat_add(a1025, a1026)?
                                                };
                                                let a1028 = 1u64;
                                                nat_mul(a1027, a1028)?
                                            };
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
            let a1046 = 1u64;
            nat_add(a1045, a1046)?
        }, f22(v2, {
            let a1085 = {
                let a1083 = {
                    let a1081 = {
                        let a1047 = 1u64;
                        let a1048 = 1u64;
                        nat_add(a1047, a1048)?
                    };
                    let a1082 = {
                        let a1079 = {
                            let a1049 = 1u64;
                            let a1050 = 1u64;
                            nat_add(a1049, a1050)?
                        };
                        let a1080 = {
                            let a1077 = {
                                let a1051 = 1u64;
                                let a1052 = 1u64;
                                nat_add(a1051, a1052)?
                            };
                            let a1078 = {
                                let a1075 = {
                                    let a1053 = 1u64;
                                    let a1054 = 1u64;
                                    nat_add(a1053, a1054)?
                                };
                                let a1076 = {
                                    let a1073 = {
                                        let a1055 = 1u64;
                                        let a1056 = 1u64;
                                        nat_add(a1055, a1056)?
                                    };
                                    let a1074 = {
                                        let a1071 = {
                                            let a1057 = 1u64;
                                            let a1058 = 1u64;
                                            nat_add(a1057, a1058)?
                                        };
                                        let a1072 = {
                                            let a1069 = {
                                                let a1059 = 1u64;
                                                let a1060 = 1u64;
                                                nat_add(a1059, a1060)?
                                            };
                                            let a1070 = {
                                                let a1067 = {
                                                    let a1061 = 1u64;
                                                    let a1062 = 1u64;
                                                    nat_add(a1061, a1062)?
                                                };
                                                let a1068 = {
                                                    let a1065 = {
                                                        let a1063 = 1u64;
                                                        let a1064 = 1u64;
                                                        nat_add(a1063, a1064)?
                                                    };
                                                    let a1066 = 1u64;
                                                    nat_mul(a1065, a1066)?
                                                };
                                                nat_mul(a1067, a1068)?
                                            };
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
                let a1084 = 1u64;
                nat_add(a1083, a1084)?
            };
            let a1086 = v1;
            nat_sub(a1085, a1086)
        })?)
    } else {
        f28(v0, v1, v2)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h1087, _) = f29(Fn1::F30, {
        let a1106 = {
            let a1088 = 1u64;
            let a1089 = 1u64;
            nat_add(a1088, a1089)?
        };
        let a1107 = {
            let a1104 = {
                let a1090 = 1u64;
                let a1091 = 1u64;
                nat_add(a1090, a1091)?
            };
            let a1105 = {
                let a1102 = {
                    let a1092 = 1u64;
                    let a1093 = 1u64;
                    nat_add(a1092, a1093)?
                };
                let a1103 = {
                    let a1100 = {
                        let a1094 = 1u64;
                        let a1095 = 1u64;
                        nat_add(a1094, a1095)?
                    };
                    let a1101 = {
                        let a1098 = {
                            let a1096 = 1u64;
                            let a1097 = 1u64;
                            nat_add(a1096, a1097)?
                        };
                        let a1099 = 1u64;
                        nat_mul(a1098, a1099)?
                    };
                    nat_mul(a1100, a1101)?
                };
                nat_mul(a1102, a1103)?
            };
            nat_mul(a1104, a1105)?
        };
        nat_mul(a1106, a1107)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h1087)
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1120 = v2;
        let a1121 = {
            let a1118 = {
                let a1108 = 1u64;
                let a1109 = 1u64;
                nat_add(a1108, a1109)?
            };
            let a1119 = {
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
            nat_mul(a1118, a1119)?
        };
        let a1122 = 0u64;
        nat_rem(a1120, a1121, a1122)
    };
    let v4: u64 = {
        let a1135 = v2;
        let a1136 = {
            let a1133 = {
                let a1123 = 1u64;
                let a1124 = 1u64;
                nat_add(a1123, a1124)?
            };
            let a1134 = {
                let a1131 = {
                    let a1125 = 1u64;
                    let a1126 = 1u64;
                    nat_add(a1125, a1126)?
                };
                let a1132 = {
                    let a1129 = {
                        let a1127 = 1u64;
                        let a1128 = 1u64;
                        nat_add(a1127, a1128)?
                    };
                    let a1130 = 1u64;
                    nat_mul(a1129, a1130)?
                };
                nat_mul(a1131, a1132)?
            };
            nat_mul(a1133, a1134)?
        };
        let a1137 = 0u64;
        nat_quot(a1135, a1136, a1137)
    };
    let v5: bool = {
        let a1146 = {
            let a1144 = {
                let a1138 = 1u64;
                let a1139 = 1u64;
                nat_add(a1138, a1139)?
            };
            let a1145 = {
                let a1142 = {
                    let a1140 = 1u64;
                    let a1141 = 1u64;
                    nat_add(a1140, a1141)?
                };
                let a1143 = 1u64;
                nat_mul(a1142, a1143)?
            };
            nat_mul(a1144, a1145)?
        };
        let a1147 = v3;
        if nat_lt(a1146, a1147) {
            true
        } else {
            let a1156 = v3;
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
            if nat_eq(a1156, a1157) {
                let a1165 = {
                    let a1162 = v4;
                    let a1163 = {
                        let a1160 = {
                            let a1158 = 1u64;
                            let a1159 = 1u64;
                            nat_add(a1158, a1159)?
                        };
                        let a1161 = 1u64;
                        nat_mul(a1160, a1161)?
                    };
                    let a1164 = 0u64;
                    nat_rem(a1162, a1163, a1164)
                };
                let a1166 = 1u64;
                nat_eq(a1165, a1166)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1167 = v4;
        let a1168 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1167, a1168)?
    };
    let v7: bool = {
        let a1197 = {
            let a1195 = {
                let a1171 = {
                    let a1169 = 1u64;
                    let a1170 = 1u64;
                    nat_add(a1169, a1170)?
                };
                let a1172 = 1u64;
                nat_mul(a1171, a1172)?
            };
            let a1196 = f6({
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
            nat_mul(a1195, a1196)?
        };
        let a1198 = v6;
        nat_le(a1197, a1198)
    };
    let v8: u64 = if v7 {
        let a1203 = v6;
        let a1204 = {
            let a1201 = {
                let a1199 = 1u64;
                let a1200 = 1u64;
                nat_add(a1199, a1200)?
            };
            let a1202 = 1u64;
            nat_mul(a1201, a1202)?
        };
        let a1205 = 0u64;
        nat_quot(a1203, a1204, a1205)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1206 = v1;
        let a1207 = 1u64;
        nat_add(a1206, a1207)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1246 = v9;
        let a1247 = {
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
        };
        if if nat_eq(a1246, a1247) {
            let a1270 = v8;
            let a1271 = f6({
                let a1268 = {
                    let a1266 = {
                        let a1248 = 1u64;
                        let a1249 = 1u64;
                        nat_add(a1248, a1249)?
                    };
                    let a1267 = {
                        let a1264 = {
                            let a1262 = {
                                let a1250 = 1u64;
                                let a1251 = 1u64;
                                nat_add(a1250, a1251)?
                            };
                            let a1263 = {
                                let a1260 = {
                                    let a1258 = {
                                        let a1252 = 1u64;
                                        let a1253 = 1u64;
                                        nat_add(a1252, a1253)?
                                    };
                                    let a1259 = {
                                        let a1256 = {
                                            let a1254 = 1u64;
                                            let a1255 = 1u64;
                                            nat_add(a1254, a1255)?
                                        };
                                        let a1257 = 1u64;
                                        nat_mul(a1256, a1257)?
                                    };
                                    nat_mul(a1258, a1259)?
                                };
                                let a1261 = 1u64;
                                nat_add(a1260, a1261)?
                            };
                            nat_mul(a1262, a1263)?
                        };
                        let a1265 = 1u64;
                        nat_add(a1264, a1265)?
                    };
                    nat_mul(a1266, a1267)?
                };
                let a1269 = 1u64;
                nat_add(a1268, a1269)?
            })?;
            nat_lt(a1270, a1271)
        } else {
            false
        } {
            0u64
        } else {
            let a1308 = v9;
            let a1309 = {
                let a1306 = {
                    let a1272 = 1u64;
                    let a1273 = 1u64;
                    nat_add(a1272, a1273)?
                };
                let a1307 = {
                    let a1304 = {
                        let a1274 = 1u64;
                        let a1275 = 1u64;
                        nat_add(a1274, a1275)?
                    };
                    let a1305 = {
                        let a1302 = {
                            let a1276 = 1u64;
                            let a1277 = 1u64;
                            nat_add(a1276, a1277)?
                        };
                        let a1303 = {
                            let a1300 = {
                                let a1278 = 1u64;
                                let a1279 = 1u64;
                                nat_add(a1278, a1279)?
                            };
                            let a1301 = {
                                let a1298 = {
                                    let a1280 = 1u64;
                                    let a1281 = 1u64;
                                    nat_add(a1280, a1281)?
                                };
                                let a1299 = {
                                    let a1296 = {
                                        let a1282 = 1u64;
                                        let a1283 = 1u64;
                                        nat_add(a1282, a1283)?
                                    };
                                    let a1297 = {
                                        let a1294 = {
                                            let a1284 = 1u64;
                                            let a1285 = 1u64;
                                            nat_add(a1284, a1285)?
                                        };
                                        let a1295 = {
                                            let a1292 = {
                                                let a1286 = 1u64;
                                                let a1287 = 1u64;
                                                nat_add(a1286, a1287)?
                                            };
                                            let a1293 = {
                                                let a1290 = {
                                                    let a1288 = 1u64;
                                                    let a1289 = 1u64;
                                                    nat_add(a1288, a1289)?
                                                };
                                                let a1291 = 1u64;
                                                nat_mul(a1290, a1291)?
                                            };
                                            nat_mul(a1292, a1293)?
                                        };
                                        nat_mul(a1294, a1295)?
                                    };
                                    nat_mul(a1296, a1297)?
                                };
                                nat_mul(a1298, a1299)?
                            };
                            nat_mul(a1300, a1301)?
                        };
                        nat_mul(a1302, a1303)?
                    };
                    nat_mul(a1304, a1305)?
                };
                nat_mul(a1306, a1307)?
            };
            nat_sub(a1308, a1309)
        }
    };
    let a1310 = v8;
    let a1311 = 0u64;
    if nat_eq(a1310, a1311) {
        f14(v0)
    } else {
        let a1364 = {
            let a1362 = {
                let a1360 = {
                    let a1312 = 1u64;
                    let a1313 = 1u64;
                    nat_add(a1312, a1313)?
                };
                let a1361 = {
                    let a1358 = {
                        let a1356 = {
                            let a1314 = 1u64;
                            let a1315 = 1u64;
                            nat_add(a1314, a1315)?
                        };
                        let a1357 = {
                            let a1354 = {
                                let a1352 = {
                                    let a1316 = 1u64;
                                    let a1317 = 1u64;
                                    nat_add(a1316, a1317)?
                                };
                                let a1353 = {
                                    let a1350 = {
                                        let a1348 = {
                                            let a1318 = 1u64;
                                            let a1319 = 1u64;
                                            nat_add(a1318, a1319)?
                                        };
                                        let a1349 = {
                                            let a1346 = {
                                                let a1344 = {
                                                    let a1320 = 1u64;
                                                    let a1321 = 1u64;
                                                    nat_add(a1320, a1321)?
                                                };
                                                let a1345 = {
                                                    let a1342 = {
                                                        let a1340 = {
                                                            let a1322 = 1u64;
                                                            let a1323 = 1u64;
                                                            nat_add(a1322, a1323)?
                                                        };
                                                        let a1341 = {
                                                            let a1338 = {
                                                                let a1336 = {
                                                                    let a1324 = 1u64;
                                                                    let a1325 = 1u64;
                                                                    nat_add(a1324, a1325)?
                                                                };
                                                                let a1337 = {
                                                                    let a1334 = {
                                                                        let a1332 = {
                                                                            let a1326 = 1u64;
                                                                            let a1327 = 1u64;
                                                                            nat_add(a1326, a1327)?
                                                                        };
                                                                        let a1333 = {
                                                                            let a1330 = {
                                                                                let a1328 = 1u64;
                                                                                let a1329 = 1u64;
                                                                                nat_add(a1328, a1329)?
                                                                            };
                                                                            let a1331 = 1u64;
                                                                            nat_mul(a1330, a1331)?
                                                                        };
                                                                        nat_mul(a1332, a1333)?
                                                                    };
                                                                    let a1335 = 1u64;
                                                                    nat_add(a1334, a1335)?
                                                                };
                                                                nat_mul(a1336, a1337)?
                                                            };
                                                            let a1339 = 1u64;
                                                            nat_add(a1338, a1339)?
                                                        };
                                                        nat_mul(a1340, a1341)?
                                                    };
                                                    let a1343 = 1u64;
                                                    nat_add(a1342, a1343)?
                                                };
                                                nat_mul(a1344, a1345)?
                                            };
                                            let a1347 = 1u64;
                                            nat_add(a1346, a1347)?
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
                        };
                        nat_mul(a1356, a1357)?
                    };
                    let a1359 = 1u64;
                    nat_add(a1358, a1359)?
                };
                nat_mul(a1360, a1361)?
            };
            let a1363 = 1u64;
            nat_add(a1362, a1363)?
        };
        let a1365 = v9;
        if nat_le(a1364, a1365) {
            f12(v0)
        } else {
            let a1417 = {
                let a1390 = f14(v0)?;
                let a1391 = {
                    let a1388 = v10;
                    let a1389 = f6({
                        let a1386 = {
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
                                            let a1376 = {
                                                let a1370 = 1u64;
                                                let a1371 = 1u64;
                                                nat_add(a1370, a1371)?
                                            };
                                            let a1377 = {
                                                let a1374 = {
                                                    let a1372 = 1u64;
                                                    let a1373 = 1u64;
                                                    nat_add(a1372, a1373)?
                                                };
                                                let a1375 = 1u64;
                                                nat_mul(a1374, a1375)?
                                            };
                                            nat_mul(a1376, a1377)?
                                        };
                                        let a1379 = 1u64;
                                        nat_add(a1378, a1379)?
                                    };
                                    nat_mul(a1380, a1381)?
                                };
                                let a1383 = 1u64;
                                nat_add(a1382, a1383)?
                            };
                            nat_mul(a1384, a1385)?
                        };
                        let a1387 = 1u64;
                        nat_add(a1386, a1387)?
                    })?;
                    nat_mul(a1388, a1389)?
                };
                nat_add(a1390, a1391)?
            };
            let a1418 = {
                let a1414 = v8;
                let a1415 = f6({
                    let a1412 = {
                        let a1410 = {
                            let a1392 = 1u64;
                            let a1393 = 1u64;
                            nat_add(a1392, a1393)?
                        };
                        let a1411 = {
                            let a1408 = {
                                let a1406 = {
                                    let a1394 = 1u64;
                                    let a1395 = 1u64;
                                    nat_add(a1394, a1395)?
                                };
                                let a1407 = {
                                    let a1404 = {
                                        let a1402 = {
                                            let a1396 = 1u64;
                                            let a1397 = 1u64;
                                            nat_add(a1396, a1397)?
                                        };
                                        let a1403 = {
                                            let a1400 = {
                                                let a1398 = 1u64;
                                                let a1399 = 1u64;
                                                nat_add(a1398, a1399)?
                                            };
                                            let a1401 = 1u64;
                                            nat_mul(a1400, a1401)?
                                        };
                                        nat_mul(a1402, a1403)?
                                    };
                                    let a1405 = 1u64;
                                    nat_add(a1404, a1405)?
                                };
                                nat_mul(a1406, a1407)?
                            };
                            let a1409 = 1u64;
                            nat_add(a1408, a1409)?
                        };
                        nat_mul(a1410, a1411)?
                    };
                    let a1413 = 1u64;
                    nat_add(a1412, a1413)?
                })?;
                let a1416 = 0u64;
                nat_rem(a1414, a1415, a1416)
            };
            nat_add(a1417, a1418)
        }
    }
}

pub fn f29(v0: Fn1, p1423: u64, p1424: Adt0) -> R<(Adt0, bool)> {
    let m1422 = (p1423, p1424);
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let (v1, v2) = m1422;
                let m1419 = v1;
                if m1419 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1419 - 1;
                    let m1421 = {
                        let c1420: Fn1 = v0.clone();
                        c1420.apply(v2.clone())?
                    };
                    match m1421 {
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
                Ok(next) => { m1422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: Adt0) -> R<Option<Adt0>> {
    f31(v0)
}

pub fn f31(v0: Adt0) -> R<Option<Adt0>> {
    let a1426 = 0u64;
    let a1427 = match v0.clone() {
        Adt0::C0(_, h1425, _, _) => {
            h1425
        }
    };
    if if if nat_lt(a1426, a1427) {
        let a1430 = match v0.clone() {
            Adt0::C0(_, h1428, _, _) => {
                h1428
            }
        };
        let a1431 = match v0.clone() {
            Adt0::C0(_, _, _, h1429) => {
                h1429
            }
        };
        nat_lt(a1430, a1431)
    } else {
        false
    } {
        let a1434 = match v0.clone() {
            Adt0::C0(_, _, h1432, _) => {
                h1432
            }
        };
        let a1435 = match v0.clone() {
            Adt0::C0(h1433, _, _, _) => {
                h1433
            }
        };
        nat_lt(a1434, a1435)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a1437 = match v0.clone() {
                Adt0::C0(h1436, _, _, _) => {
                    h1436
                }
            };
            let a1438 = 1u64;
            nat_sub(a1437, a1438)
        }, {
            let a1441 = match v0.clone() {
                Adt0::C0(_, h1439, _, _) => {
                    h1439
                }
            };
            let a1442 = match v0.clone() {
                Adt0::C0(_, h1440, _, _) => {
                    h1440
                }
            };
            nat_add(a1441, a1442)?
        }, match v0.clone() {
            Adt0::C0(_, _, h1443, _) => {
                h1443
            }
        }, match v0 {
            Adt0::C0(_, _, _, h1444) => {
                h1444
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<u64> {
    f0(p0, p1, p2, p3)
}
