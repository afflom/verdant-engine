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
    F28,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F28 => f28(p0),
        }
    }
}

pub fn f0(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if v0 {
        f1(f2(v1, v3)?, v2)
    } else {
        f1(v1, f2(v2, v3)?)
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if f3(v0)? {
        true
    } else {
        f3(v1)?
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
            let a66 = f4({
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
        let a88 = f4({
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
            f5(v1)?
        } else {
            false
        } {
            let a89 = f6(v0)?;
            let a90 = f6(v1)?;
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
                    let a156 = f4({
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
                let a178 = f4({
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
            if f5(v0)? {
                Ok(v0)
            } else {
                if f5(v1)? {
                    Ok(v1)
                } else {
                    let a179 = f7(v1)?;
                    let a180 = f7(v0)?;
                    if nat_le(a179, a180) {
                        f8(v0, v1)
                    } else {
                        f8(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if if if f3(v0)? {
        true
    } else {
        f3(v1)?
    } {
        true
    } else {
        if f5(v0)? {
            f9(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f5(v1)? {
            f9(v0)?
        } else {
            false
        }
    } {
        let a267 = {
            let a245 = {
                let a221 = {
                    let a219 = {
                        let a181 = 1u64;
                        let a182 = 1u64;
                        nat_add(a181, a182)?
                    };
                    let a220 = {
                        let a217 = {
                            let a215 = {
                                let a183 = 1u64;
                                let a184 = 1u64;
                                nat_add(a183, a184)?
                            };
                            let a216 = {
                                let a213 = {
                                    let a211 = {
                                        let a185 = 1u64;
                                        let a186 = 1u64;
                                        nat_add(a185, a186)?
                                    };
                                    let a212 = {
                                        let a209 = {
                                            let a207 = {
                                                let a187 = 1u64;
                                                let a188 = 1u64;
                                                nat_add(a187, a188)?
                                            };
                                            let a208 = {
                                                let a205 = {
                                                    let a203 = {
                                                        let a189 = 1u64;
                                                        let a190 = 1u64;
                                                        nat_add(a189, a190)?
                                                    };
                                                    let a204 = {
                                                        let a201 = {
                                                            let a199 = {
                                                                let a191 = 1u64;
                                                                let a192 = 1u64;
                                                                nat_add(a191, a192)?
                                                            };
                                                            let a200 = {
                                                                let a197 = {
                                                                    let a195 = {
                                                                        let a193 = 1u64;
                                                                        let a194 = 1u64;
                                                                        nat_add(a193, a194)?
                                                                    };
                                                                    let a196 = 1u64;
                                                                    nat_mul(a195, a196)?
                                                                };
                                                                let a198 = 1u64;
                                                                nat_add(a197, a198)?
                                                            };
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
                            };
                            nat_mul(a215, a216)?
                        };
                        let a218 = 1u64;
                        nat_add(a217, a218)?
                    };
                    nat_mul(a219, a220)?
                };
                let a222 = 1u64;
                nat_add(a221, a222)?
            };
            let a246 = f4({
                let a243 = {
                    let a241 = {
                        let a223 = 1u64;
                        let a224 = 1u64;
                        nat_add(a223, a224)?
                    };
                    let a242 = {
                        let a239 = {
                            let a237 = {
                                let a225 = 1u64;
                                let a226 = 1u64;
                                nat_add(a225, a226)?
                            };
                            let a238 = {
                                let a235 = {
                                    let a233 = {
                                        let a227 = 1u64;
                                        let a228 = 1u64;
                                        nat_add(a227, a228)?
                                    };
                                    let a234 = {
                                        let a231 = {
                                            let a229 = 1u64;
                                            let a230 = 1u64;
                                            nat_add(a229, a230)?
                                        };
                                        let a232 = 1u64;
                                        nat_mul(a231, a232)?
                                    };
                                    nat_mul(a233, a234)?
                                };
                                let a236 = 1u64;
                                nat_add(a235, a236)?
                            };
                            nat_mul(a237, a238)?
                        };
                        let a240 = 1u64;
                        nat_add(a239, a240)?
                    };
                    nat_mul(a241, a242)?
                };
                let a244 = 1u64;
                nat_add(a243, a244)?
            })?;
            nat_mul(a245, a246)?
        };
        let a268 = f4({
            let a265 = {
                let a247 = 1u64;
                let a248 = 1u64;
                nat_add(a247, a248)?
            };
            let a266 = {
                let a263 = {
                    let a261 = {
                        let a249 = 1u64;
                        let a250 = 1u64;
                        nat_add(a249, a250)?
                    };
                    let a262 = {
                        let a259 = {
                            let a257 = {
                                let a251 = 1u64;
                                let a252 = 1u64;
                                nat_add(a251, a252)?
                            };
                            let a258 = {
                                let a255 = {
                                    let a253 = 1u64;
                                    let a254 = 1u64;
                                    nat_add(a253, a254)?
                                };
                                let a256 = 1u64;
                                nat_mul(a255, a256)?
                            };
                            nat_mul(a257, a258)?
                        };
                        let a260 = 1u64;
                        nat_add(a259, a260)?
                    };
                    nat_mul(a261, a262)?
                };
                let a264 = 1u64;
                nat_add(a263, a264)?
            };
            nat_mul(a265, a266)?
        })?;
        nat_add(a267, a268)
    } else {
        if if f5(v0)? {
            true
        } else {
            f5(v1)?
        } {
            f10(f11(v0, v1)?)
        } else {
            if if f9(v0)? {
                true
            } else {
                f9(v1)?
            } {
                f12(f11(v0, v1)?)
            } else {
                f13(v0, v1)
            }
        }
    }
}

pub fn f3(v0: u64) -> R<bool> {
    let a311 = f14(v0)?;
    let a312 = {
        let a309 = {
            let a307 = {
                let a269 = 1u64;
                let a270 = 1u64;
                nat_add(a269, a270)?
            };
            let a308 = {
                let a305 = {
                    let a303 = {
                        let a271 = 1u64;
                        let a272 = 1u64;
                        nat_add(a271, a272)?
                    };
                    let a304 = {
                        let a301 = {
                            let a299 = {
                                let a273 = 1u64;
                                let a274 = 1u64;
                                nat_add(a273, a274)?
                            };
                            let a300 = {
                                let a297 = {
                                    let a295 = {
                                        let a275 = 1u64;
                                        let a276 = 1u64;
                                        nat_add(a275, a276)?
                                    };
                                    let a296 = {
                                        let a293 = {
                                            let a291 = {
                                                let a277 = 1u64;
                                                let a278 = 1u64;
                                                nat_add(a277, a278)?
                                            };
                                            let a292 = {
                                                let a289 = {
                                                    let a287 = {
                                                        let a279 = 1u64;
                                                        let a280 = 1u64;
                                                        nat_add(a279, a280)?
                                                    };
                                                    let a288 = {
                                                        let a285 = {
                                                            let a283 = {
                                                                let a281 = 1u64;
                                                                let a282 = 1u64;
                                                                nat_add(a281, a282)?
                                                            };
                                                            let a284 = 1u64;
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
    };
    if nat_eq(a311, a312) {
        let a313 = 0u64;
        let a314 = f15(v0)?;
        Ok(nat_lt(a313, a314))
    } else {
        Ok(false)
    }
}

pub fn f4(v0: u64) -> R<u64> {
    f16(Fn0::F17, v0, 1u64)
}

pub fn f5(v0: u64) -> R<bool> {
    let a357 = f14(v0)?;
    let a358 = {
        let a355 = {
            let a353 = {
                let a315 = 1u64;
                let a316 = 1u64;
                nat_add(a315, a316)?
            };
            let a354 = {
                let a351 = {
                    let a349 = {
                        let a317 = 1u64;
                        let a318 = 1u64;
                        nat_add(a317, a318)?
                    };
                    let a350 = {
                        let a347 = {
                            let a345 = {
                                let a319 = 1u64;
                                let a320 = 1u64;
                                nat_add(a319, a320)?
                            };
                            let a346 = {
                                let a343 = {
                                    let a341 = {
                                        let a321 = 1u64;
                                        let a322 = 1u64;
                                        nat_add(a321, a322)?
                                    };
                                    let a342 = {
                                        let a339 = {
                                            let a337 = {
                                                let a323 = 1u64;
                                                let a324 = 1u64;
                                                nat_add(a323, a324)?
                                            };
                                            let a338 = {
                                                let a335 = {
                                                    let a333 = {
                                                        let a325 = 1u64;
                                                        let a326 = 1u64;
                                                        nat_add(a325, a326)?
                                                    };
                                                    let a334 = {
                                                        let a331 = {
                                                            let a329 = {
                                                                let a327 = 1u64;
                                                                let a328 = 1u64;
                                                                nat_add(a327, a328)?
                                                            };
                                                            let a330 = 1u64;
                                                            nat_mul(a329, a330)?
                                                        };
                                                        let a332 = 1u64;
                                                        nat_add(a331, a332)?
                                                    };
                                                    nat_mul(a333, a334)?
                                                };
                                                let a336 = 1u64;
                                                nat_add(a335, a336)?
                                            };
                                            nat_mul(a337, a338)?
                                        };
                                        let a340 = 1u64;
                                        nat_add(a339, a340)?
                                    };
                                    nat_mul(a341, a342)?
                                };
                                let a344 = 1u64;
                                nat_add(a343, a344)?
                            };
                            nat_mul(a345, a346)?
                        };
                        let a348 = 1u64;
                        nat_add(a347, a348)?
                    };
                    nat_mul(a349, a350)?
                };
                let a352 = 1u64;
                nat_add(a351, a352)?
            };
            nat_mul(a353, a354)?
        };
        let a356 = 1u64;
        nat_add(a355, a356)?
    };
    if nat_eq(a357, a358) {
        let a359 = f15(v0)?;
        let a360 = 0u64;
        Ok(nat_eq(a359, a360))
    } else {
        Ok(false)
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let a385 = v0;
    let a386 = f4({
        let a383 = {
            let a381 = {
                let a361 = 1u64;
                let a362 = 1u64;
                nat_add(a361, a362)?
            };
            let a382 = {
                let a379 = {
                    let a377 = {
                        let a363 = 1u64;
                        let a364 = 1u64;
                        nat_add(a363, a364)?
                    };
                    let a378 = {
                        let a375 = {
                            let a373 = {
                                let a365 = 1u64;
                                let a366 = 1u64;
                                nat_add(a365, a366)?
                            };
                            let a374 = {
                                let a371 = {
                                    let a369 = {
                                        let a367 = 1u64;
                                        let a368 = 1u64;
                                        nat_add(a367, a368)?
                                    };
                                    let a370 = 1u64;
                                    nat_mul(a369, a370)?
                                };
                                let a372 = 1u64;
                                nat_add(a371, a372)?
                            };
                            nat_mul(a373, a374)?
                        };
                        let a376 = 1u64;
                        nat_add(a375, a376)?
                    };
                    nat_mul(a377, a378)?
                };
                let a380 = 1u64;
                nat_add(a379, a380)?
            };
            nat_mul(a381, a382)?
        };
        let a384 = 1u64;
        nat_add(a383, a384)?
    })?;
    let a387 = 0u64;
    Ok(nat_quot(a385, a386, a387))
}

pub fn f7(v0: u64) -> R<u64> {
    let a412 = v0;
    let a413 = f4({
        let a410 = {
            let a408 = {
                let a388 = 1u64;
                let a389 = 1u64;
                nat_add(a388, a389)?
            };
            let a409 = {
                let a406 = {
                    let a404 = {
                        let a390 = 1u64;
                        let a391 = 1u64;
                        nat_add(a390, a391)?
                    };
                    let a405 = {
                        let a402 = {
                            let a400 = {
                                let a392 = 1u64;
                                let a393 = 1u64;
                                nat_add(a392, a393)?
                            };
                            let a401 = {
                                let a398 = {
                                    let a396 = {
                                        let a394 = 1u64;
                                        let a395 = 1u64;
                                        nat_add(a394, a395)?
                                    };
                                    let a397 = 1u64;
                                    nat_mul(a396, a397)?
                                };
                                let a399 = 1u64;
                                nat_add(a398, a399)?
                            };
                            nat_mul(a400, a401)?
                        };
                        let a403 = 1u64;
                        nat_add(a402, a403)?
                    };
                    nat_mul(a404, a405)?
                };
                let a407 = 1u64;
                nat_add(a406, a407)?
            };
            nat_mul(a408, a409)?
        };
        let a411 = 1u64;
        nat_add(a410, a411)?
    })?;
    let a414 = 0u64;
    Ok(nat_rem(a412, a413, a414))
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f18(v0)?;
    let v3: u64 = {
        let a427 = f19(v0)?;
        let a428 = {
            let a425 = {
                let a415 = 1u64;
                let a416 = 1u64;
                nat_add(a415, a416)?
            };
            let a426 = {
                let a423 = {
                    let a417 = 1u64;
                    let a418 = 1u64;
                    nat_add(a417, a418)?
                };
                let a424 = {
                    let a421 = {
                        let a419 = 1u64;
                        let a420 = 1u64;
                        nat_add(a419, a420)?
                    };
                    let a422 = 1u64;
                    nat_mul(a421, a422)?
                };
                nat_mul(a423, a424)?
            };
            nat_mul(a425, a426)?
        };
        nat_mul(a427, a428)?
    };
    let v4: u64 = f20({
        let a441 = f19(v1)?;
        let a442 = {
            let a439 = {
                let a429 = 1u64;
                let a430 = 1u64;
                nat_add(a429, a430)?
            };
            let a440 = {
                let a437 = {
                    let a431 = 1u64;
                    let a432 = 1u64;
                    nat_add(a431, a432)?
                };
                let a438 = {
                    let a435 = {
                        let a433 = 1u64;
                        let a434 = 1u64;
                        nat_add(a433, a434)?
                    };
                    let a436 = 1u64;
                    nat_mul(a435, a436)?
                };
                nat_mul(a437, a438)?
            };
            nat_mul(a439, a440)?
        };
        nat_mul(a441, a442)?
    }, {
        let a443 = v2;
        let a444 = f18(v1)?;
        nat_sub(a443, a444)
    })?;
    let a445 = f6(v0)?;
    let a446 = f6(v1)?;
    if nat_eq(a445, a446) {
        f21(f6(v0)?, v2, {
            let a447 = v3;
            let a448 = v4;
            nat_add(a447, a448)?
        })
    } else {
        let a449 = v3;
        let a450 = v4;
        if nat_eq(a449, a450) {
            Ok(0u64)
        } else {
            f21(f6(v0)?, v2, {
                let a451 = v3;
                let a452 = v4;
                nat_sub(a451, a452)
            })
        }
    }
}

pub fn f9(v0: u64) -> R<bool> {
    let a453 = f7(v0)?;
    let a454 = 0u64;
    Ok(nat_eq(a453, a454))
}

pub fn f10(v0: u64) -> R<u64> {
    let a521 = f12(v0)?;
    let a522 = {
        let a519 = {
            let a495 = {
                let a493 = {
                    let a455 = 1u64;
                    let a456 = 1u64;
                    nat_add(a455, a456)?
                };
                let a494 = {
                    let a491 = {
                        let a489 = {
                            let a457 = 1u64;
                            let a458 = 1u64;
                            nat_add(a457, a458)?
                        };
                        let a490 = {
                            let a487 = {
                                let a485 = {
                                    let a459 = 1u64;
                                    let a460 = 1u64;
                                    nat_add(a459, a460)?
                                };
                                let a486 = {
                                    let a483 = {
                                        let a481 = {
                                            let a461 = 1u64;
                                            let a462 = 1u64;
                                            nat_add(a461, a462)?
                                        };
                                        let a482 = {
                                            let a479 = {
                                                let a477 = {
                                                    let a463 = 1u64;
                                                    let a464 = 1u64;
                                                    nat_add(a463, a464)?
                                                };
                                                let a478 = {
                                                    let a475 = {
                                                        let a473 = {
                                                            let a465 = 1u64;
                                                            let a466 = 1u64;
                                                            nat_add(a465, a466)?
                                                        };
                                                        let a474 = {
                                                            let a471 = {
                                                                let a469 = {
                                                                    let a467 = 1u64;
                                                                    let a468 = 1u64;
                                                                    nat_add(a467, a468)?
                                                                };
                                                                let a470 = 1u64;
                                                                nat_mul(a469, a470)?
                                                            };
                                                            let a472 = 1u64;
                                                            nat_add(a471, a472)?
                                                        };
                                                        nat_mul(a473, a474)?
                                                    };
                                                    let a476 = 1u64;
                                                    nat_add(a475, a476)?
                                                };
                                                nat_mul(a477, a478)?
                                            };
                                            let a480 = 1u64;
                                            nat_add(a479, a480)?
                                        };
                                        nat_mul(a481, a482)?
                                    };
                                    let a484 = 1u64;
                                    nat_add(a483, a484)?
                                };
                                nat_mul(a485, a486)?
                            };
                            let a488 = 1u64;
                            nat_add(a487, a488)?
                        };
                        nat_mul(a489, a490)?
                    };
                    let a492 = 1u64;
                    nat_add(a491, a492)?
                };
                nat_mul(a493, a494)?
            };
            let a496 = 1u64;
            nat_add(a495, a496)?
        };
        let a520 = f4({
            let a517 = {
                let a515 = {
                    let a497 = 1u64;
                    let a498 = 1u64;
                    nat_add(a497, a498)?
                };
                let a516 = {
                    let a513 = {
                        let a511 = {
                            let a499 = 1u64;
                            let a500 = 1u64;
                            nat_add(a499, a500)?
                        };
                        let a512 = {
                            let a509 = {
                                let a507 = {
                                    let a501 = 1u64;
                                    let a502 = 1u64;
                                    nat_add(a501, a502)?
                                };
                                let a508 = {
                                    let a505 = {
                                        let a503 = 1u64;
                                        let a504 = 1u64;
                                        nat_add(a503, a504)?
                                    };
                                    let a506 = 1u64;
                                    nat_mul(a505, a506)?
                                };
                                nat_mul(a507, a508)?
                            };
                            let a510 = 1u64;
                            nat_add(a509, a510)?
                        };
                        nat_mul(a511, a512)?
                    };
                    let a514 = 1u64;
                    nat_add(a513, a514)?
                };
                nat_mul(a515, a516)?
            };
            let a518 = 1u64;
            nat_add(a517, a518)?
        })?;
        nat_mul(a519, a520)?
    };
    nat_add(a521, a522)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a529 = {
        let a523 = f6(v0)?;
        let a524 = f6(v1)?;
        nat_add(a523, a524)?
    };
    let a530 = {
        let a527 = {
            let a525 = 1u64;
            let a526 = 1u64;
            nat_add(a525, a526)?
        };
        let a528 = 1u64;
        nat_mul(a527, a528)?
    };
    let a531 = 0u64;
    Ok(nat_rem(a529, a530, a531))
}

pub fn f12(v0: u64) -> R<u64> {
    let a556 = v0;
    let a557 = f4({
        let a554 = {
            let a552 = {
                let a532 = 1u64;
                let a533 = 1u64;
                nat_add(a532, a533)?
            };
            let a553 = {
                let a550 = {
                    let a548 = {
                        let a534 = 1u64;
                        let a535 = 1u64;
                        nat_add(a534, a535)?
                    };
                    let a549 = {
                        let a546 = {
                            let a544 = {
                                let a536 = 1u64;
                                let a537 = 1u64;
                                nat_add(a536, a537)?
                            };
                            let a545 = {
                                let a542 = {
                                    let a540 = {
                                        let a538 = 1u64;
                                        let a539 = 1u64;
                                        nat_add(a538, a539)?
                                    };
                                    let a541 = 1u64;
                                    nat_mul(a540, a541)?
                                };
                                let a543 = 1u64;
                                nat_add(a542, a543)?
                            };
                            nat_mul(a544, a545)?
                        };
                        let a547 = 1u64;
                        nat_add(a546, a547)?
                    };
                    nat_mul(a548, a549)?
                };
                let a551 = 1u64;
                nat_add(a550, a551)?
            };
            nat_mul(a552, a553)?
        };
        let a555 = 1u64;
        nat_add(a554, a555)?
    })?;
    nat_mul(a556, a557)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f22(v0)?;
    let v3: Adt0 = f22(v1)?;
    f21(f11(v0, v1)?, {
        let a612 = {
            let a560 = match v2.clone() {
                Adt0::C0(h558, _, _, _) => {
                    h558
                }
            };
            let a561 = match v3.clone() {
                Adt0::C0(h559, _, _, _) => {
                    h559
                }
            };
            nat_add(a560, a561)?
        };
        let a613 = {
            let a610 = {
                let a608 = {
                    let a562 = 1u64;
                    let a563 = 1u64;
                    nat_add(a562, a563)?
                };
                let a609 = {
                    let a606 = {
                        let a604 = {
                            let a564 = 1u64;
                            let a565 = 1u64;
                            nat_add(a564, a565)?
                        };
                        let a605 = {
                            let a602 = {
                                let a600 = {
                                    let a566 = 1u64;
                                    let a567 = 1u64;
                                    nat_add(a566, a567)?
                                };
                                let a601 = {
                                    let a598 = {
                                        let a596 = {
                                            let a568 = 1u64;
                                            let a569 = 1u64;
                                            nat_add(a568, a569)?
                                        };
                                        let a597 = {
                                            let a594 = {
                                                let a592 = {
                                                    let a570 = 1u64;
                                                    let a571 = 1u64;
                                                    nat_add(a570, a571)?
                                                };
                                                let a593 = {
                                                    let a590 = {
                                                        let a588 = {
                                                            let a572 = 1u64;
                                                            let a573 = 1u64;
                                                            nat_add(a572, a573)?
                                                        };
                                                        let a589 = {
                                                            let a586 = {
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
                nat_mul(a608, a609)?
            };
            let a611 = 1u64;
            nat_add(a610, a611)?
        };
        nat_sub(a612, a613)
    }, f20({
        let a616 = match v2 {
            Adt0::C0(_, h614, _, _) => {
                h614
            }
        };
        let a617 = match v3 {
            Adt0::C0(_, h615, _, _) => {
                h615
            }
        };
        nat_mul(a616, a617)?
    }, {
        let a634 = {
            let a618 = 1u64;
            let a619 = 1u64;
            nat_add(a618, a619)?
        };
        let a635 = {
            let a632 = {
                let a620 = 1u64;
                let a621 = 1u64;
                nat_add(a620, a621)?
            };
            let a633 = {
                let a630 = {
                    let a628 = {
                        let a622 = 1u64;
                        let a623 = 1u64;
                        nat_add(a622, a623)?
                    };
                    let a629 = {
                        let a626 = {
                            let a624 = 1u64;
                            let a625 = 1u64;
                            nat_add(a624, a625)?
                        };
                        let a627 = 1u64;
                        nat_mul(a626, a627)?
                    };
                    nat_mul(a628, a629)?
                };
                let a631 = 1u64;
                nat_add(a630, a631)?
            };
            nat_mul(a632, a633)?
        };
        nat_mul(a634, a635)?
    })?)
}

pub fn f14(v0: u64) -> R<u64> {
    let a658 = f7(v0)?;
    let a659 = f4({
        let a656 = {
            let a654 = {
                let a636 = 1u64;
                let a637 = 1u64;
                nat_add(a636, a637)?
            };
            let a655 = {
                let a652 = {
                    let a650 = {
                        let a638 = 1u64;
                        let a639 = 1u64;
                        nat_add(a638, a639)?
                    };
                    let a651 = {
                        let a648 = {
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
                        let a649 = 1u64;
                        nat_add(a648, a649)?
                    };
                    nat_mul(a650, a651)?
                };
                let a653 = 1u64;
                nat_add(a652, a653)?
            };
            nat_mul(a654, a655)?
        };
        let a657 = 1u64;
        nat_add(a656, a657)?
    })?;
    let a660 = 0u64;
    Ok(nat_quot(a658, a659, a660))
}

pub fn f15(v0: u64) -> R<u64> {
    let a683 = v0;
    let a684 = f4({
        let a681 = {
            let a679 = {
                let a661 = 1u64;
                let a662 = 1u64;
                nat_add(a661, a662)?
            };
            let a680 = {
                let a677 = {
                    let a675 = {
                        let a663 = 1u64;
                        let a664 = 1u64;
                        nat_add(a663, a664)?
                    };
                    let a676 = {
                        let a673 = {
                            let a671 = {
                                let a665 = 1u64;
                                let a666 = 1u64;
                                nat_add(a665, a666)?
                            };
                            let a672 = {
                                let a669 = {
                                    let a667 = 1u64;
                                    let a668 = 1u64;
                                    nat_add(a667, a668)?
                                };
                                let a670 = 1u64;
                                nat_mul(a669, a670)?
                            };
                            nat_mul(a671, a672)?
                        };
                        let a674 = 1u64;
                        nat_add(a673, a674)?
                    };
                    nat_mul(a675, a676)?
                };
                let a678 = 1u64;
                nat_add(a677, a678)?
            };
            nat_mul(a679, a680)?
        };
        let a682 = 1u64;
        nat_add(a681, a682)?
    })?;
    let a685 = 0u64;
    Ok(nat_rem(a683, a684, a685))
}

pub fn f16(v0: Fn0, p689: u64, p690: u64) -> R<u64> {
    let m688 = (p689, p690);
    {
        let mut m688 = m688;
        loop {
            let step = {
                let (v1, v2) = m688;
                let m686 = v1;
                if m686 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m686 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c687: Fn0 = v0.clone();
                        c687.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: u64) -> R<u64> {
    let a691 = v0;
    let a692 = v0;
    nat_add(a691, a692)
}

pub fn f18(v0: u64) -> R<u64> {
    let a731 = {
        let a693 = f14(v0)?;
        let a694 = 0u64;
        if nat_eq(a693, a694) {
            1u64
        } else {
            f14(v0)?
        }
    };
    let a732 = {
        let a729 = {
            let a695 = 1u64;
            let a696 = 1u64;
            nat_add(a695, a696)?
        };
        let a730 = {
            let a727 = {
                let a697 = 1u64;
                let a698 = 1u64;
                nat_add(a697, a698)?
            };
            let a728 = {
                let a725 = {
                    let a699 = 1u64;
                    let a700 = 1u64;
                    nat_add(a699, a700)?
                };
                let a726 = {
                    let a723 = {
                        let a701 = 1u64;
                        let a702 = 1u64;
                        nat_add(a701, a702)?
                    };
                    let a724 = {
                        let a721 = {
                            let a703 = 1u64;
                            let a704 = 1u64;
                            nat_add(a703, a704)?
                        };
                        let a722 = {
                            let a719 = {
                                let a705 = 1u64;
                                let a706 = 1u64;
                                nat_add(a705, a706)?
                            };
                            let a720 = {
                                let a717 = {
                                    let a707 = 1u64;
                                    let a708 = 1u64;
                                    nat_add(a707, a708)?
                                };
                                let a718 = {
                                    let a715 = {
                                        let a709 = 1u64;
                                        let a710 = 1u64;
                                        nat_add(a709, a710)?
                                    };
                                    let a716 = {
                                        let a713 = {
                                            let a711 = 1u64;
                                            let a712 = 1u64;
                                            nat_add(a711, a712)?
                                        };
                                        let a714 = 1u64;
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
            nat_mul(a727, a728)?
        };
        nat_mul(a729, a730)?
    };
    nat_add(a731, a732)
}

pub fn f19(v0: u64) -> R<u64> {
    let a757 = f15(v0)?;
    let a758 = {
        let a733 = f14(v0)?;
        let a734 = 0u64;
        if nat_eq(a733, a734) {
            0u64
        } else {
            f4({
                let a755 = {
                    let a753 = {
                        let a735 = 1u64;
                        let a736 = 1u64;
                        nat_add(a735, a736)?
                    };
                    let a754 = {
                        let a751 = {
                            let a749 = {
                                let a737 = 1u64;
                                let a738 = 1u64;
                                nat_add(a737, a738)?
                            };
                            let a750 = {
                                let a747 = {
                                    let a745 = {
                                        let a739 = 1u64;
                                        let a740 = 1u64;
                                        nat_add(a739, a740)?
                                    };
                                    let a746 = {
                                        let a743 = {
                                            let a741 = 1u64;
                                            let a742 = 1u64;
                                            nat_add(a741, a742)?
                                        };
                                        let a744 = 1u64;
                                        nat_mul(a743, a744)?
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
                    };
                    nat_mul(a753, a754)?
                };
                let a756 = 1u64;
                nat_add(a755, a756)?
            })?
        }
    };
    nat_add(a757, a758)
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a759 = v1;
    let a760 = 0u64;
    if nat_eq(a759, a760) {
        Ok(v0)
    } else {
        let a785 = {
            let a783 = {
                let a761 = 1u64;
                let a762 = 1u64;
                nat_add(a761, a762)?
            };
            let a784 = {
                let a781 = {
                    let a763 = 1u64;
                    let a764 = 1u64;
                    nat_add(a763, a764)?
                };
                let a782 = {
                    let a779 = {
                        let a765 = 1u64;
                        let a766 = 1u64;
                        nat_add(a765, a766)?
                    };
                    let a780 = {
                        let a777 = {
                            let a767 = 1u64;
                            let a768 = 1u64;
                            nat_add(a767, a768)?
                        };
                        let a778 = {
                            let a775 = {
                                let a769 = 1u64;
                                let a770 = 1u64;
                                nat_add(a769, a770)?
                            };
                            let a776 = {
                                let a773 = {
                                    let a771 = 1u64;
                                    let a772 = 1u64;
                                    nat_add(a771, a772)?
                                };
                                let a774 = 1u64;
                                nat_mul(a773, a774)?
                            };
                            nat_mul(a775, a776)?
                        };
                        nat_mul(a777, a778)?
                    };
                    nat_mul(a779, a780)?
                };
                nat_mul(a781, a782)?
            };
            nat_mul(a783, a784)?
        };
        let a786 = v1;
        if nat_le(a785, a786) {
            let a787 = v0;
            let a788 = 0u64;
            if nat_eq(a787, a788) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f4(v1)?;
            f23({
                let a789 = v0;
                let a790 = v2;
                let a791 = 0u64;
                nat_quot(a789, a790, a791)
            }, {
                let a795 = 0u64;
                let a796 = {
                    let a792 = v0;
                    let a793 = v2;
                    let a794 = 0u64;
                    nat_rem(a792, a793, a794)
                };
                nat_lt(a795, a796)
            })
        }
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a819 = f4({
        let a817 = {
            let a815 = {
                let a797 = 1u64;
                let a798 = 1u64;
                nat_add(a797, a798)?
            };
            let a816 = {
                let a813 = {
                    let a811 = {
                        let a799 = 1u64;
                        let a800 = 1u64;
                        nat_add(a799, a800)?
                    };
                    let a812 = {
                        let a809 = {
                            let a801 = 1u64;
                            let a802 = 1u64;
                            nat_add(a801, a802)?
                        };
                        let a810 = {
                            let a807 = {
                                let a805 = {
                                    let a803 = 1u64;
                                    let a804 = 1u64;
                                    nat_add(a803, a804)?
                                };
                                let a806 = 1u64;
                                nat_mul(a805, a806)?
                            };
                            let a808 = 1u64;
                            nat_add(a807, a808)?
                        };
                        nat_mul(a809, a810)?
                    };
                    nat_mul(a811, a812)?
                };
                let a814 = 1u64;
                nat_add(a813, a814)?
            };
            nat_mul(a815, a816)?
        };
        let a818 = 1u64;
        nat_add(a817, a818)?
    })?;
    let a820 = v2;
    if nat_le(a819, a820) {
        f24(v0, {
            let a821 = v1;
            let a822 = 1u64;
            nat_add(a821, a822)?
        }, f20(v2, 1u64)?)
    } else {
        let v3: Adt0 = f25(v1, v2, {
            let a859 = {
                let a857 = {
                    let a823 = 1u64;
                    let a824 = 1u64;
                    nat_add(a823, a824)?
                };
                let a858 = {
                    let a855 = {
                        let a825 = 1u64;
                        let a826 = 1u64;
                        nat_add(a825, a826)?
                    };
                    let a856 = {
                        let a853 = {
                            let a827 = 1u64;
                            let a828 = 1u64;
                            nat_add(a827, a828)?
                        };
                        let a854 = {
                            let a851 = {
                                let a829 = 1u64;
                                let a830 = 1u64;
                                nat_add(a829, a830)?
                            };
                            let a852 = {
                                let a849 = {
                                    let a831 = 1u64;
                                    let a832 = 1u64;
                                    nat_add(a831, a832)?
                                };
                                let a850 = {
                                    let a847 = {
                                        let a833 = 1u64;
                                        let a834 = 1u64;
                                        nat_add(a833, a834)?
                                    };
                                    let a848 = {
                                        let a845 = {
                                            let a835 = 1u64;
                                            let a836 = 1u64;
                                            nat_add(a835, a836)?
                                        };
                                        let a846 = {
                                            let a843 = {
                                                let a837 = 1u64;
                                                let a838 = 1u64;
                                                nat_add(a837, a838)?
                                            };
                                            let a844 = {
                                                let a841 = {
                                                    let a839 = 1u64;
                                                    let a840 = 1u64;
                                                    nat_add(a839, a840)?
                                                };
                                                let a842 = 1u64;
                                                nat_mul(a841, a842)?
                                            };
                                            nat_mul(a843, a844)?
                                        };
                                        nat_mul(a845, a846)?
                                    };
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
            let a860 = 1u64;
            nat_add(a859, a860)?
        }, f4({
            let a879 = {
                let a861 = 1u64;
                let a862 = 1u64;
                nat_add(a861, a862)?
            };
            let a880 = {
                let a877 = {
                    let a875 = {
                        let a863 = 1u64;
                        let a864 = 1u64;
                        nat_add(a863, a864)?
                    };
                    let a876 = {
                        let a873 = {
                            let a865 = 1u64;
                            let a866 = 1u64;
                            nat_add(a865, a866)?
                        };
                        let a874 = {
                            let a871 = {
                                let a869 = {
                                    let a867 = 1u64;
                                    let a868 = 1u64;
                                    nat_add(a867, a868)?
                                };
                                let a870 = 1u64;
                                nat_mul(a869, a870)?
                            };
                            let a872 = 1u64;
                            nat_add(a871, a872)?
                        };
                        nat_mul(a873, a874)?
                    };
                    nat_mul(a875, a876)?
                };
                let a878 = 1u64;
                nat_add(a877, a878)?
            };
            nat_mul(a879, a880)?
        })?)?;
        f24(v0, match v3.clone() {
            Adt0::C0(h881, _, _, _) => {
                h881
            }
        }, match v3 {
            Adt0::C0(_, h882, _, _) => {
                h882
            }
        })
    }
}

pub fn f22(v0: u64) -> R<Adt0> {
    f25(f18(v0)?, f19(v0)?, 0u64, f4({
        let a903 = {
            let a901 = {
                let a883 = 1u64;
                let a884 = 1u64;
                nat_add(a883, a884)?
            };
            let a902 = {
                let a899 = {
                    let a897 = {
                        let a885 = 1u64;
                        let a886 = 1u64;
                        nat_add(a885, a886)?
                    };
                    let a898 = {
                        let a895 = {
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
                        let a896 = 1u64;
                        nat_add(a895, a896)?
                    };
                    nat_mul(a897, a898)?
                };
                let a900 = 1u64;
                nat_add(a899, a900)?
            };
            nat_mul(a901, a902)?
        };
        let a904 = 1u64;
        nat_add(a903, a904)?
    })?)
}

pub fn f23(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a912 = {
            let a909 = v0;
            let a910 = {
                let a907 = {
                    let a905 = 1u64;
                    let a906 = 1u64;
                    nat_add(a905, a906)?
                };
                let a908 = 1u64;
                nat_mul(a907, a908)?
            };
            let a911 = 0u64;
            nat_rem(a909, a910, a911)
        };
        let a913 = 0u64;
        nat_eq(a912, a913)
    } else {
        false
    } {
        let a914 = v0;
        let a915 = 1u64;
        nat_add(a914, a915)
    } else {
        Ok(v0)
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a954 = v1;
    let a955 = {
        let a952 = {
            let a950 = {
                let a916 = 1u64;
                let a917 = 1u64;
                nat_add(a916, a917)?
            };
            let a951 = {
                let a948 = {
                    let a918 = 1u64;
                    let a919 = 1u64;
                    nat_add(a918, a919)?
                };
                let a949 = {
                    let a946 = {
                        let a920 = 1u64;
                        let a921 = 1u64;
                        nat_add(a920, a921)?
                    };
                    let a947 = {
                        let a944 = {
                            let a922 = 1u64;
                            let a923 = 1u64;
                            nat_add(a922, a923)?
                        };
                        let a945 = {
                            let a942 = {
                                let a924 = 1u64;
                                let a925 = 1u64;
                                nat_add(a924, a925)?
                            };
                            let a943 = {
                                let a940 = {
                                    let a926 = 1u64;
                                    let a927 = 1u64;
                                    nat_add(a926, a927)?
                                };
                                let a941 = {
                                    let a938 = {
                                        let a928 = 1u64;
                                        let a929 = 1u64;
                                        nat_add(a928, a929)?
                                    };
                                    let a939 = {
                                        let a936 = {
                                            let a930 = 1u64;
                                            let a931 = 1u64;
                                            nat_add(a930, a931)?
                                        };
                                        let a937 = {
                                            let a934 = {
                                                let a932 = 1u64;
                                                let a933 = 1u64;
                                                nat_add(a932, a933)?
                                            };
                                            let a935 = 1u64;
                                            nat_mul(a934, a935)?
                                        };
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
        let a953 = 1u64;
        nat_add(a952, a953)?
    };
    if nat_lt(a954, a955) {
        f26(v0, {
            let a992 = {
                let a990 = {
                    let a956 = 1u64;
                    let a957 = 1u64;
                    nat_add(a956, a957)?
                };
                let a991 = {
                    let a988 = {
                        let a958 = 1u64;
                        let a959 = 1u64;
                        nat_add(a958, a959)?
                    };
                    let a989 = {
                        let a986 = {
                            let a960 = 1u64;
                            let a961 = 1u64;
                            nat_add(a960, a961)?
                        };
                        let a987 = {
                            let a984 = {
                                let a962 = 1u64;
                                let a963 = 1u64;
                                nat_add(a962, a963)?
                            };
                            let a985 = {
                                let a982 = {
                                    let a964 = 1u64;
                                    let a965 = 1u64;
                                    nat_add(a964, a965)?
                                };
                                let a983 = {
                                    let a980 = {
                                        let a966 = 1u64;
                                        let a967 = 1u64;
                                        nat_add(a966, a967)?
                                    };
                                    let a981 = {
                                        let a978 = {
                                            let a968 = 1u64;
                                            let a969 = 1u64;
                                            nat_add(a968, a969)?
                                        };
                                        let a979 = {
                                            let a976 = {
                                                let a970 = 1u64;
                                                let a971 = 1u64;
                                                nat_add(a970, a971)?
                                            };
                                            let a977 = {
                                                let a974 = {
                                                    let a972 = 1u64;
                                                    let a973 = 1u64;
                                                    nat_add(a972, a973)?
                                                };
                                                let a975 = 1u64;
                                                nat_mul(a974, a975)?
                                            };
                                            nat_mul(a976, a977)?
                                        };
                                        nat_mul(a978, a979)?
                                    };
                                    nat_mul(a980, a981)?
                                };
                                nat_mul(a982, a983)?
                            };
                            nat_mul(a984, a985)?
                        };
                        nat_mul(a986, a987)?
                    };
                    nat_mul(a988, a989)?
                };
                nat_mul(a990, a991)?
            };
            let a993 = 1u64;
            nat_add(a992, a993)?
        }, f20(v2, {
            let a1032 = {
                let a1030 = {
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
                let a1031 = 1u64;
                nat_add(a1030, a1031)?
            };
            let a1033 = v1;
            nat_sub(a1032, a1033)
        })?)
    } else {
        f26(v0, v1, v2)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h1034, _) = f27(Fn1::F28, {
        let a1053 = {
            let a1035 = 1u64;
            let a1036 = 1u64;
            nat_add(a1035, a1036)?
        };
        let a1054 = {
            let a1051 = {
                let a1037 = 1u64;
                let a1038 = 1u64;
                nat_add(a1037, a1038)?
            };
            let a1052 = {
                let a1049 = {
                    let a1039 = 1u64;
                    let a1040 = 1u64;
                    nat_add(a1039, a1040)?
                };
                let a1050 = {
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
                nat_mul(a1049, a1050)?
            };
            nat_mul(a1051, a1052)?
        };
        nat_mul(a1053, a1054)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h1034)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1067 = v2;
        let a1068 = {
            let a1065 = {
                let a1055 = 1u64;
                let a1056 = 1u64;
                nat_add(a1055, a1056)?
            };
            let a1066 = {
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
            nat_mul(a1065, a1066)?
        };
        let a1069 = 0u64;
        nat_rem(a1067, a1068, a1069)
    };
    let v4: u64 = {
        let a1082 = v2;
        let a1083 = {
            let a1080 = {
                let a1070 = 1u64;
                let a1071 = 1u64;
                nat_add(a1070, a1071)?
            };
            let a1081 = {
                let a1078 = {
                    let a1072 = 1u64;
                    let a1073 = 1u64;
                    nat_add(a1072, a1073)?
                };
                let a1079 = {
                    let a1076 = {
                        let a1074 = 1u64;
                        let a1075 = 1u64;
                        nat_add(a1074, a1075)?
                    };
                    let a1077 = 1u64;
                    nat_mul(a1076, a1077)?
                };
                nat_mul(a1078, a1079)?
            };
            nat_mul(a1080, a1081)?
        };
        let a1084 = 0u64;
        nat_quot(a1082, a1083, a1084)
    };
    let v5: bool = {
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
        let a1094 = v3;
        if nat_lt(a1093, a1094) {
            true
        } else {
            let a1103 = v3;
            let a1104 = {
                let a1101 = {
                    let a1095 = 1u64;
                    let a1096 = 1u64;
                    nat_add(a1095, a1096)?
                };
                let a1102 = {
                    let a1099 = {
                        let a1097 = 1u64;
                        let a1098 = 1u64;
                        nat_add(a1097, a1098)?
                    };
                    let a1100 = 1u64;
                    nat_mul(a1099, a1100)?
                };
                nat_mul(a1101, a1102)?
            };
            if nat_eq(a1103, a1104) {
                let a1112 = {
                    let a1109 = v4;
                    let a1110 = {
                        let a1107 = {
                            let a1105 = 1u64;
                            let a1106 = 1u64;
                            nat_add(a1105, a1106)?
                        };
                        let a1108 = 1u64;
                        nat_mul(a1107, a1108)?
                    };
                    let a1111 = 0u64;
                    nat_rem(a1109, a1110, a1111)
                };
                let a1113 = 1u64;
                nat_eq(a1112, a1113)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1114 = v4;
        let a1115 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1114, a1115)?
    };
    let v7: bool = {
        let a1144 = {
            let a1142 = {
                let a1118 = {
                    let a1116 = 1u64;
                    let a1117 = 1u64;
                    nat_add(a1116, a1117)?
                };
                let a1119 = 1u64;
                nat_mul(a1118, a1119)?
            };
            let a1143 = f4({
                let a1140 = {
                    let a1138 = {
                        let a1120 = 1u64;
                        let a1121 = 1u64;
                        nat_add(a1120, a1121)?
                    };
                    let a1139 = {
                        let a1136 = {
                            let a1134 = {
                                let a1122 = 1u64;
                                let a1123 = 1u64;
                                nat_add(a1122, a1123)?
                            };
                            let a1135 = {
                                let a1132 = {
                                    let a1130 = {
                                        let a1124 = 1u64;
                                        let a1125 = 1u64;
                                        nat_add(a1124, a1125)?
                                    };
                                    let a1131 = {
                                        let a1128 = {
                                            let a1126 = 1u64;
                                            let a1127 = 1u64;
                                            nat_add(a1126, a1127)?
                                        };
                                        let a1129 = 1u64;
                                        nat_mul(a1128, a1129)?
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
                    nat_mul(a1138, a1139)?
                };
                let a1141 = 1u64;
                nat_add(a1140, a1141)?
            })?;
            nat_mul(a1142, a1143)?
        };
        let a1145 = v6;
        nat_le(a1144, a1145)
    };
    let v8: u64 = if v7 {
        let a1150 = v6;
        let a1151 = {
            let a1148 = {
                let a1146 = 1u64;
                let a1147 = 1u64;
                nat_add(a1146, a1147)?
            };
            let a1149 = 1u64;
            nat_mul(a1148, a1149)?
        };
        let a1152 = 0u64;
        nat_quot(a1150, a1151, a1152)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1153 = v1;
        let a1154 = 1u64;
        nat_add(a1153, a1154)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1193 = v9;
        let a1194 = {
            let a1191 = {
                let a1189 = {
                    let a1155 = 1u64;
                    let a1156 = 1u64;
                    nat_add(a1155, a1156)?
                };
                let a1190 = {
                    let a1187 = {
                        let a1157 = 1u64;
                        let a1158 = 1u64;
                        nat_add(a1157, a1158)?
                    };
                    let a1188 = {
                        let a1185 = {
                            let a1159 = 1u64;
                            let a1160 = 1u64;
                            nat_add(a1159, a1160)?
                        };
                        let a1186 = {
                            let a1183 = {
                                let a1161 = 1u64;
                                let a1162 = 1u64;
                                nat_add(a1161, a1162)?
                            };
                            let a1184 = {
                                let a1181 = {
                                    let a1163 = 1u64;
                                    let a1164 = 1u64;
                                    nat_add(a1163, a1164)?
                                };
                                let a1182 = {
                                    let a1179 = {
                                        let a1165 = 1u64;
                                        let a1166 = 1u64;
                                        nat_add(a1165, a1166)?
                                    };
                                    let a1180 = {
                                        let a1177 = {
                                            let a1167 = 1u64;
                                            let a1168 = 1u64;
                                            nat_add(a1167, a1168)?
                                        };
                                        let a1178 = {
                                            let a1175 = {
                                                let a1169 = 1u64;
                                                let a1170 = 1u64;
                                                nat_add(a1169, a1170)?
                                            };
                                            let a1176 = {
                                                let a1173 = {
                                                    let a1171 = 1u64;
                                                    let a1172 = 1u64;
                                                    nat_add(a1171, a1172)?
                                                };
                                                let a1174 = 1u64;
                                                nat_mul(a1173, a1174)?
                                            };
                                            nat_mul(a1175, a1176)?
                                        };
                                        nat_mul(a1177, a1178)?
                                    };
                                    nat_mul(a1179, a1180)?
                                };
                                nat_mul(a1181, a1182)?
                            };
                            nat_mul(a1183, a1184)?
                        };
                        nat_mul(a1185, a1186)?
                    };
                    nat_mul(a1187, a1188)?
                };
                nat_mul(a1189, a1190)?
            };
            let a1192 = 1u64;
            nat_add(a1191, a1192)?
        };
        if if nat_eq(a1193, a1194) {
            let a1217 = v8;
            let a1218 = f4({
                let a1215 = {
                    let a1213 = {
                        let a1195 = 1u64;
                        let a1196 = 1u64;
                        nat_add(a1195, a1196)?
                    };
                    let a1214 = {
                        let a1211 = {
                            let a1209 = {
                                let a1197 = 1u64;
                                let a1198 = 1u64;
                                nat_add(a1197, a1198)?
                            };
                            let a1210 = {
                                let a1207 = {
                                    let a1205 = {
                                        let a1199 = 1u64;
                                        let a1200 = 1u64;
                                        nat_add(a1199, a1200)?
                                    };
                                    let a1206 = {
                                        let a1203 = {
                                            let a1201 = 1u64;
                                            let a1202 = 1u64;
                                            nat_add(a1201, a1202)?
                                        };
                                        let a1204 = 1u64;
                                        nat_mul(a1203, a1204)?
                                    };
                                    nat_mul(a1205, a1206)?
                                };
                                let a1208 = 1u64;
                                nat_add(a1207, a1208)?
                            };
                            nat_mul(a1209, a1210)?
                        };
                        let a1212 = 1u64;
                        nat_add(a1211, a1212)?
                    };
                    nat_mul(a1213, a1214)?
                };
                let a1216 = 1u64;
                nat_add(a1215, a1216)?
            })?;
            nat_lt(a1217, a1218)
        } else {
            false
        } {
            0u64
        } else {
            let a1255 = v9;
            let a1256 = {
                let a1253 = {
                    let a1219 = 1u64;
                    let a1220 = 1u64;
                    nat_add(a1219, a1220)?
                };
                let a1254 = {
                    let a1251 = {
                        let a1221 = 1u64;
                        let a1222 = 1u64;
                        nat_add(a1221, a1222)?
                    };
                    let a1252 = {
                        let a1249 = {
                            let a1223 = 1u64;
                            let a1224 = 1u64;
                            nat_add(a1223, a1224)?
                        };
                        let a1250 = {
                            let a1247 = {
                                let a1225 = 1u64;
                                let a1226 = 1u64;
                                nat_add(a1225, a1226)?
                            };
                            let a1248 = {
                                let a1245 = {
                                    let a1227 = 1u64;
                                    let a1228 = 1u64;
                                    nat_add(a1227, a1228)?
                                };
                                let a1246 = {
                                    let a1243 = {
                                        let a1229 = 1u64;
                                        let a1230 = 1u64;
                                        nat_add(a1229, a1230)?
                                    };
                                    let a1244 = {
                                        let a1241 = {
                                            let a1231 = 1u64;
                                            let a1232 = 1u64;
                                            nat_add(a1231, a1232)?
                                        };
                                        let a1242 = {
                                            let a1239 = {
                                                let a1233 = 1u64;
                                                let a1234 = 1u64;
                                                nat_add(a1233, a1234)?
                                            };
                                            let a1240 = {
                                                let a1237 = {
                                                    let a1235 = 1u64;
                                                    let a1236 = 1u64;
                                                    nat_add(a1235, a1236)?
                                                };
                                                let a1238 = 1u64;
                                                nat_mul(a1237, a1238)?
                                            };
                                            nat_mul(a1239, a1240)?
                                        };
                                        nat_mul(a1241, a1242)?
                                    };
                                    nat_mul(a1243, a1244)?
                                };
                                nat_mul(a1245, a1246)?
                            };
                            nat_mul(a1247, a1248)?
                        };
                        nat_mul(a1249, a1250)?
                    };
                    nat_mul(a1251, a1252)?
                };
                nat_mul(a1253, a1254)?
            };
            nat_sub(a1255, a1256)
        }
    };
    let a1257 = v8;
    let a1258 = 0u64;
    if nat_eq(a1257, a1258) {
        f12(v0)
    } else {
        let a1311 = {
            let a1309 = {
                let a1307 = {
                    let a1259 = 1u64;
                    let a1260 = 1u64;
                    nat_add(a1259, a1260)?
                };
                let a1308 = {
                    let a1305 = {
                        let a1303 = {
                            let a1261 = 1u64;
                            let a1262 = 1u64;
                            nat_add(a1261, a1262)?
                        };
                        let a1304 = {
                            let a1301 = {
                                let a1299 = {
                                    let a1263 = 1u64;
                                    let a1264 = 1u64;
                                    nat_add(a1263, a1264)?
                                };
                                let a1300 = {
                                    let a1297 = {
                                        let a1295 = {
                                            let a1265 = 1u64;
                                            let a1266 = 1u64;
                                            nat_add(a1265, a1266)?
                                        };
                                        let a1296 = {
                                            let a1293 = {
                                                let a1291 = {
                                                    let a1267 = 1u64;
                                                    let a1268 = 1u64;
                                                    nat_add(a1267, a1268)?
                                                };
                                                let a1292 = {
                                                    let a1289 = {
                                                        let a1287 = {
                                                            let a1269 = 1u64;
                                                            let a1270 = 1u64;
                                                            nat_add(a1269, a1270)?
                                                        };
                                                        let a1288 = {
                                                            let a1285 = {
                                                                let a1283 = {
                                                                    let a1271 = 1u64;
                                                                    let a1272 = 1u64;
                                                                    nat_add(a1271, a1272)?
                                                                };
                                                                let a1284 = {
                                                                    let a1281 = {
                                                                        let a1279 = {
                                                                            let a1273 = 1u64;
                                                                            let a1274 = 1u64;
                                                                            nat_add(a1273, a1274)?
                                                                        };
                                                                        let a1280 = {
                                                                            let a1277 = {
                                                                                let a1275 = 1u64;
                                                                                let a1276 = 1u64;
                                                                                nat_add(a1275, a1276)?
                                                                            };
                                                                            let a1278 = 1u64;
                                                                            nat_mul(a1277, a1278)?
                                                                        };
                                                                        nat_mul(a1279, a1280)?
                                                                    };
                                                                    let a1282 = 1u64;
                                                                    nat_add(a1281, a1282)?
                                                                };
                                                                nat_mul(a1283, a1284)?
                                                            };
                                                            let a1286 = 1u64;
                                                            nat_add(a1285, a1286)?
                                                        };
                                                        nat_mul(a1287, a1288)?
                                                    };
                                                    let a1290 = 1u64;
                                                    nat_add(a1289, a1290)?
                                                };
                                                nat_mul(a1291, a1292)?
                                            };
                                            let a1294 = 1u64;
                                            nat_add(a1293, a1294)?
                                        };
                                        nat_mul(a1295, a1296)?
                                    };
                                    let a1298 = 1u64;
                                    nat_add(a1297, a1298)?
                                };
                                nat_mul(a1299, a1300)?
                            };
                            let a1302 = 1u64;
                            nat_add(a1301, a1302)?
                        };
                        nat_mul(a1303, a1304)?
                    };
                    let a1306 = 1u64;
                    nat_add(a1305, a1306)?
                };
                nat_mul(a1307, a1308)?
            };
            let a1310 = 1u64;
            nat_add(a1309, a1310)?
        };
        let a1312 = v9;
        if nat_le(a1311, a1312) {
            f10(v0)
        } else {
            let a1364 = {
                let a1337 = f12(v0)?;
                let a1338 = {
                    let a1335 = v10;
                    let a1336 = f4({
                        let a1333 = {
                            let a1331 = {
                                let a1313 = 1u64;
                                let a1314 = 1u64;
                                nat_add(a1313, a1314)?
                            };
                            let a1332 = {
                                let a1329 = {
                                    let a1327 = {
                                        let a1315 = 1u64;
                                        let a1316 = 1u64;
                                        nat_add(a1315, a1316)?
                                    };
                                    let a1328 = {
                                        let a1325 = {
                                            let a1323 = {
                                                let a1317 = 1u64;
                                                let a1318 = 1u64;
                                                nat_add(a1317, a1318)?
                                            };
                                            let a1324 = {
                                                let a1321 = {
                                                    let a1319 = 1u64;
                                                    let a1320 = 1u64;
                                                    nat_add(a1319, a1320)?
                                                };
                                                let a1322 = 1u64;
                                                nat_mul(a1321, a1322)?
                                            };
                                            nat_mul(a1323, a1324)?
                                        };
                                        let a1326 = 1u64;
                                        nat_add(a1325, a1326)?
                                    };
                                    nat_mul(a1327, a1328)?
                                };
                                let a1330 = 1u64;
                                nat_add(a1329, a1330)?
                            };
                            nat_mul(a1331, a1332)?
                        };
                        let a1334 = 1u64;
                        nat_add(a1333, a1334)?
                    })?;
                    nat_mul(a1335, a1336)?
                };
                nat_add(a1337, a1338)?
            };
            let a1365 = {
                let a1361 = v8;
                let a1362 = f4({
                    let a1359 = {
                        let a1357 = {
                            let a1339 = 1u64;
                            let a1340 = 1u64;
                            nat_add(a1339, a1340)?
                        };
                        let a1358 = {
                            let a1355 = {
                                let a1353 = {
                                    let a1341 = 1u64;
                                    let a1342 = 1u64;
                                    nat_add(a1341, a1342)?
                                };
                                let a1354 = {
                                    let a1351 = {
                                        let a1349 = {
                                            let a1343 = 1u64;
                                            let a1344 = 1u64;
                                            nat_add(a1343, a1344)?
                                        };
                                        let a1350 = {
                                            let a1347 = {
                                                let a1345 = 1u64;
                                                let a1346 = 1u64;
                                                nat_add(a1345, a1346)?
                                            };
                                            let a1348 = 1u64;
                                            nat_mul(a1347, a1348)?
                                        };
                                        nat_mul(a1349, a1350)?
                                    };
                                    let a1352 = 1u64;
                                    nat_add(a1351, a1352)?
                                };
                                nat_mul(a1353, a1354)?
                            };
                            let a1356 = 1u64;
                            nat_add(a1355, a1356)?
                        };
                        nat_mul(a1357, a1358)?
                    };
                    let a1360 = 1u64;
                    nat_add(a1359, a1360)?
                })?;
                let a1363 = 0u64;
                nat_rem(a1361, a1362, a1363)
            };
            nat_add(a1364, a1365)
        }
    }
}

pub fn f27(v0: Fn1, p1370: u64, p1371: Adt0) -> R<(Adt0, bool)> {
    let m1369 = (p1370, p1371);
    {
        let mut m1369 = m1369;
        loop {
            let step = {
                let (v1, v2) = m1369;
                let m1366 = v1;
                if m1366 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1366 - 1;
                    let m1368 = {
                        let c1367: Fn1 = v0.clone();
                        c1367.apply(v2.clone())?
                    };
                    match m1368 {
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
                Ok(next) => { m1369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(v0: Adt0) -> R<Option<Adt0>> {
    f29(v0)
}

pub fn f29(v0: Adt0) -> R<Option<Adt0>> {
    let a1373 = 0u64;
    let a1374 = match v0.clone() {
        Adt0::C0(_, h1372, _, _) => {
            h1372
        }
    };
    if if if nat_lt(a1373, a1374) {
        let a1377 = match v0.clone() {
            Adt0::C0(_, h1375, _, _) => {
                h1375
            }
        };
        let a1378 = match v0.clone() {
            Adt0::C0(_, _, _, h1376) => {
                h1376
            }
        };
        nat_lt(a1377, a1378)
    } else {
        false
    } {
        let a1381 = match v0.clone() {
            Adt0::C0(_, _, h1379, _) => {
                h1379
            }
        };
        let a1382 = match v0.clone() {
            Adt0::C0(h1380, _, _, _) => {
                h1380
            }
        };
        nat_lt(a1381, a1382)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a1384 = match v0.clone() {
                Adt0::C0(h1383, _, _, _) => {
                    h1383
                }
            };
            let a1385 = 1u64;
            nat_sub(a1384, a1385)
        }, {
            let a1388 = match v0.clone() {
                Adt0::C0(_, h1386, _, _) => {
                    h1386
                }
            };
            let a1389 = match v0.clone() {
                Adt0::C0(_, h1387, _, _) => {
                    h1387
                }
            };
            nat_add(a1388, a1389)?
        }, match v0.clone() {
            Adt0::C0(_, _, h1390, _) => {
                h1390
            }
        }, match v0 {
            Adt0::C0(_, _, _, h1391) => {
                h1391
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}
