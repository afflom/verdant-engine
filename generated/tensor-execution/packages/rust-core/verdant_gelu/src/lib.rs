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
    C2(u64),
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
    F27,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F27 => f27(p0),
        }
    }
}

pub fn f0(v0: bool, v1: u64, v2: bool, v3: u64) -> R<Adt0> {
    if v0 {
        if v2 {
            Ok(Adt0::C0(f1(v1, v3)?))
        } else {
            Ok(Adt0::C2(f1(1071242019u64, v1)?))
        }
    } else {
        if v2 {
            Ok(Adt0::C0(f1(f1(1056964608u64, v1)?, f2(1065353216u64, v3)?)?))
        } else {
            let v4: u64 = f1(f1(1027024659u64, v1)?, v1)?;
            Ok(Adt0::C1(f1(f1(1061962282u64, v1)?, f2(1065353216u64, v4)?)?))
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    if if if if f3(v0)? {
        true
    } else {
        f3(v1)?
    } {
        true
    } else {
        if f4(v0)? {
            f5(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f4(v1)? {
            f5(v0)?
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
        if if f4(v0)? {
            true
        } else {
            f4(v1)?
        } {
            f7(f8(v0, v1)?)
        } else {
            if if f5(v0)? {
                true
            } else {
                f5(v1)?
            } {
                f9(f8(v0, v1)?)
            } else {
                f10(v0, v1)
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if f3(v0)? {
        true
    } else {
        f3(v1)?
    } {
        let a175 = {
            let a153 = {
                let a129 = {
                    let a127 = {
                        let a89 = 1u64;
                        let a90 = 1u64;
                        nat_add(a89, a90)?
                    };
                    let a128 = {
                        let a125 = {
                            let a123 = {
                                let a91 = 1u64;
                                let a92 = 1u64;
                                nat_add(a91, a92)?
                            };
                            let a124 = {
                                let a121 = {
                                    let a119 = {
                                        let a93 = 1u64;
                                        let a94 = 1u64;
                                        nat_add(a93, a94)?
                                    };
                                    let a120 = {
                                        let a117 = {
                                            let a115 = {
                                                let a95 = 1u64;
                                                let a96 = 1u64;
                                                nat_add(a95, a96)?
                                            };
                                            let a116 = {
                                                let a113 = {
                                                    let a111 = {
                                                        let a97 = 1u64;
                                                        let a98 = 1u64;
                                                        nat_add(a97, a98)?
                                                    };
                                                    let a112 = {
                                                        let a109 = {
                                                            let a107 = {
                                                                let a99 = 1u64;
                                                                let a100 = 1u64;
                                                                nat_add(a99, a100)?
                                                            };
                                                            let a108 = {
                                                                let a105 = {
                                                                    let a103 = {
                                                                        let a101 = 1u64;
                                                                        let a102 = 1u64;
                                                                        nat_add(a101, a102)?
                                                                    };
                                                                    let a104 = 1u64;
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
                                            };
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
            let a154 = f6({
                let a151 = {
                    let a149 = {
                        let a131 = 1u64;
                        let a132 = 1u64;
                        nat_add(a131, a132)?
                    };
                    let a150 = {
                        let a147 = {
                            let a145 = {
                                let a133 = 1u64;
                                let a134 = 1u64;
                                nat_add(a133, a134)?
                            };
                            let a146 = {
                                let a143 = {
                                    let a141 = {
                                        let a135 = 1u64;
                                        let a136 = 1u64;
                                        nat_add(a135, a136)?
                                    };
                                    let a142 = {
                                        let a139 = {
                                            let a137 = 1u64;
                                            let a138 = 1u64;
                                            nat_add(a137, a138)?
                                        };
                                        let a140 = 1u64;
                                        nat_mul(a139, a140)?
                                    };
                                    nat_mul(a141, a142)?
                                };
                                let a144 = 1u64;
                                nat_add(a143, a144)?
                            };
                            nat_mul(a145, a146)?
                        };
                        let a148 = 1u64;
                        nat_add(a147, a148)?
                    };
                    nat_mul(a149, a150)?
                };
                let a152 = 1u64;
                nat_add(a151, a152)?
            })?;
            nat_mul(a153, a154)?
        };
        let a176 = f6({
            let a173 = {
                let a155 = 1u64;
                let a156 = 1u64;
                nat_add(a155, a156)?
            };
            let a174 = {
                let a171 = {
                    let a169 = {
                        let a157 = 1u64;
                        let a158 = 1u64;
                        nat_add(a157, a158)?
                    };
                    let a170 = {
                        let a167 = {
                            let a165 = {
                                let a159 = 1u64;
                                let a160 = 1u64;
                                nat_add(a159, a160)?
                            };
                            let a166 = {
                                let a163 = {
                                    let a161 = 1u64;
                                    let a162 = 1u64;
                                    nat_add(a161, a162)?
                                };
                                let a164 = 1u64;
                                nat_mul(a163, a164)?
                            };
                            nat_mul(a165, a166)?
                        };
                        let a168 = 1u64;
                        nat_add(a167, a168)?
                    };
                    nat_mul(a169, a170)?
                };
                let a172 = 1u64;
                nat_add(a171, a172)?
            };
            nat_mul(a173, a174)?
        })?;
        nat_add(a175, a176)
    } else {
        if if f4(v0)? {
            f4(v1)?
        } else {
            false
        } {
            let a177 = f11(v0)?;
            let a178 = f11(v1)?;
            if nat_eq(a177, a178) {
                Ok(v0)
            } else {
                let a265 = {
                    let a243 = {
                        let a219 = {
                            let a217 = {
                                let a179 = 1u64;
                                let a180 = 1u64;
                                nat_add(a179, a180)?
                            };
                            let a218 = {
                                let a215 = {
                                    let a213 = {
                                        let a181 = 1u64;
                                        let a182 = 1u64;
                                        nat_add(a181, a182)?
                                    };
                                    let a214 = {
                                        let a211 = {
                                            let a209 = {
                                                let a183 = 1u64;
                                                let a184 = 1u64;
                                                nat_add(a183, a184)?
                                            };
                                            let a210 = {
                                                let a207 = {
                                                    let a205 = {
                                                        let a185 = 1u64;
                                                        let a186 = 1u64;
                                                        nat_add(a185, a186)?
                                                    };
                                                    let a206 = {
                                                        let a203 = {
                                                            let a201 = {
                                                                let a187 = 1u64;
                                                                let a188 = 1u64;
                                                                nat_add(a187, a188)?
                                                            };
                                                            let a202 = {
                                                                let a199 = {
                                                                    let a197 = {
                                                                        let a189 = 1u64;
                                                                        let a190 = 1u64;
                                                                        nat_add(a189, a190)?
                                                                    };
                                                                    let a198 = {
                                                                        let a195 = {
                                                                            let a193 = {
                                                                                let a191 = 1u64;
                                                                                let a192 = 1u64;
                                                                                nat_add(a191, a192)?
                                                                            };
                                                                            let a194 = 1u64;
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
                                                    };
                                                    nat_mul(a205, a206)?
                                                };
                                                let a208 = 1u64;
                                                nat_add(a207, a208)?
                                            };
                                            nat_mul(a209, a210)?
                                        };
                                        let a212 = 1u64;
                                        nat_add(a211, a212)?
                                    };
                                    nat_mul(a213, a214)?
                                };
                                let a216 = 1u64;
                                nat_add(a215, a216)?
                            };
                            nat_mul(a217, a218)?
                        };
                        let a220 = 1u64;
                        nat_add(a219, a220)?
                    };
                    let a244 = f6({
                        let a241 = {
                            let a239 = {
                                let a221 = 1u64;
                                let a222 = 1u64;
                                nat_add(a221, a222)?
                            };
                            let a240 = {
                                let a237 = {
                                    let a235 = {
                                        let a223 = 1u64;
                                        let a224 = 1u64;
                                        nat_add(a223, a224)?
                                    };
                                    let a236 = {
                                        let a233 = {
                                            let a231 = {
                                                let a225 = 1u64;
                                                let a226 = 1u64;
                                                nat_add(a225, a226)?
                                            };
                                            let a232 = {
                                                let a229 = {
                                                    let a227 = 1u64;
                                                    let a228 = 1u64;
                                                    nat_add(a227, a228)?
                                                };
                                                let a230 = 1u64;
                                                nat_mul(a229, a230)?
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
                    })?;
                    nat_mul(a243, a244)?
                };
                let a266 = f6({
                    let a263 = {
                        let a245 = 1u64;
                        let a246 = 1u64;
                        nat_add(a245, a246)?
                    };
                    let a264 = {
                        let a261 = {
                            let a259 = {
                                let a247 = 1u64;
                                let a248 = 1u64;
                                nat_add(a247, a248)?
                            };
                            let a260 = {
                                let a257 = {
                                    let a255 = {
                                        let a249 = 1u64;
                                        let a250 = 1u64;
                                        nat_add(a249, a250)?
                                    };
                                    let a256 = {
                                        let a253 = {
                                            let a251 = 1u64;
                                            let a252 = 1u64;
                                            nat_add(a251, a252)?
                                        };
                                        let a254 = 1u64;
                                        nat_mul(a253, a254)?
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
                })?;
                nat_add(a265, a266)
            }
        } else {
            if f4(v0)? {
                Ok(v0)
            } else {
                if f4(v1)? {
                    Ok(v1)
                } else {
                    let a267 = f12(v1)?;
                    let a268 = f12(v0)?;
                    if nat_le(a267, a268) {
                        f13(v0, v1)
                    } else {
                        f13(v1, v0)
                    }
                }
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

pub fn f4(v0: u64) -> R<bool> {
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

pub fn f5(v0: u64) -> R<bool> {
    let a361 = f12(v0)?;
    let a362 = 0u64;
    Ok(nat_eq(a361, a362))
}

pub fn f6(v0: u64) -> R<u64> {
    f16(Fn0::F17, v0, 1u64)
}

pub fn f7(v0: u64) -> R<u64> {
    let a429 = f9(v0)?;
    let a430 = {
        let a427 = {
            let a403 = {
                let a401 = {
                    let a363 = 1u64;
                    let a364 = 1u64;
                    nat_add(a363, a364)?
                };
                let a402 = {
                    let a399 = {
                        let a397 = {
                            let a365 = 1u64;
                            let a366 = 1u64;
                            nat_add(a365, a366)?
                        };
                        let a398 = {
                            let a395 = {
                                let a393 = {
                                    let a367 = 1u64;
                                    let a368 = 1u64;
                                    nat_add(a367, a368)?
                                };
                                let a394 = {
                                    let a391 = {
                                        let a389 = {
                                            let a369 = 1u64;
                                            let a370 = 1u64;
                                            nat_add(a369, a370)?
                                        };
                                        let a390 = {
                                            let a387 = {
                                                let a385 = {
                                                    let a371 = 1u64;
                                                    let a372 = 1u64;
                                                    nat_add(a371, a372)?
                                                };
                                                let a386 = {
                                                    let a383 = {
                                                        let a381 = {
                                                            let a373 = 1u64;
                                                            let a374 = 1u64;
                                                            nat_add(a373, a374)?
                                                        };
                                                        let a382 = {
                                                            let a379 = {
                                                                let a377 = {
                                                                    let a375 = 1u64;
                                                                    let a376 = 1u64;
                                                                    nat_add(a375, a376)?
                                                                };
                                                                let a378 = 1u64;
                                                                nat_mul(a377, a378)?
                                                            };
                                                            let a380 = 1u64;
                                                            nat_add(a379, a380)?
                                                        };
                                                        nat_mul(a381, a382)?
                                                    };
                                                    let a384 = 1u64;
                                                    nat_add(a383, a384)?
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
        let a428 = f6({
            let a425 = {
                let a423 = {
                    let a405 = 1u64;
                    let a406 = 1u64;
                    nat_add(a405, a406)?
                };
                let a424 = {
                    let a421 = {
                        let a419 = {
                            let a407 = 1u64;
                            let a408 = 1u64;
                            nat_add(a407, a408)?
                        };
                        let a420 = {
                            let a417 = {
                                let a415 = {
                                    let a409 = 1u64;
                                    let a410 = 1u64;
                                    nat_add(a409, a410)?
                                };
                                let a416 = {
                                    let a413 = {
                                        let a411 = 1u64;
                                        let a412 = 1u64;
                                        nat_add(a411, a412)?
                                    };
                                    let a414 = 1u64;
                                    nat_mul(a413, a414)?
                                };
                                nat_mul(a415, a416)?
                            };
                            let a418 = 1u64;
                            nat_add(a417, a418)?
                        };
                        nat_mul(a419, a420)?
                    };
                    let a422 = 1u64;
                    nat_add(a421, a422)?
                };
                nat_mul(a423, a424)?
            };
            let a426 = 1u64;
            nat_add(a425, a426)?
        })?;
        nat_mul(a427, a428)?
    };
    nat_add(a429, a430)
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a437 = {
        let a431 = f11(v0)?;
        let a432 = f11(v1)?;
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
    let a439 = 0u64;
    Ok(nat_rem(a437, a438, a439))
}

pub fn f9(v0: u64) -> R<u64> {
    let a464 = v0;
    let a465 = f6({
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
    })?;
    nat_mul(a464, a465)
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f18(v0)?;
    let v3: Adt1 = f18(v1)?;
    f19(f8(v0, v1)?, {
        let a520 = {
            let a468 = match v2.clone() {
                Adt1::C0(h466, _, _, _) => {
                    h466
                }
            };
            let a469 = match v3.clone() {
                Adt1::C0(h467, _, _, _) => {
                    h467
                }
            };
            nat_add(a468, a469)?
        };
        let a521 = {
            let a518 = {
                let a516 = {
                    let a470 = 1u64;
                    let a471 = 1u64;
                    nat_add(a470, a471)?
                };
                let a517 = {
                    let a514 = {
                        let a512 = {
                            let a472 = 1u64;
                            let a473 = 1u64;
                            nat_add(a472, a473)?
                        };
                        let a513 = {
                            let a510 = {
                                let a508 = {
                                    let a474 = 1u64;
                                    let a475 = 1u64;
                                    nat_add(a474, a475)?
                                };
                                let a509 = {
                                    let a506 = {
                                        let a504 = {
                                            let a476 = 1u64;
                                            let a477 = 1u64;
                                            nat_add(a476, a477)?
                                        };
                                        let a505 = {
                                            let a502 = {
                                                let a500 = {
                                                    let a478 = 1u64;
                                                    let a479 = 1u64;
                                                    nat_add(a478, a479)?
                                                };
                                                let a501 = {
                                                    let a498 = {
                                                        let a496 = {
                                                            let a480 = 1u64;
                                                            let a481 = 1u64;
                                                            nat_add(a480, a481)?
                                                        };
                                                        let a497 = {
                                                            let a494 = {
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
                                                            let a495 = 1u64;
                                                            nat_add(a494, a495)?
                                                        };
                                                        nat_mul(a496, a497)?
                                                    };
                                                    let a499 = 1u64;
                                                    nat_add(a498, a499)?
                                                };
                                                nat_mul(a500, a501)?
                                            };
                                            let a503 = 1u64;
                                            nat_add(a502, a503)?
                                        };
                                        nat_mul(a504, a505)?
                                    };
                                    let a507 = 1u64;
                                    nat_add(a506, a507)?
                                };
                                nat_mul(a508, a509)?
                            };
                            let a511 = 1u64;
                            nat_add(a510, a511)?
                        };
                        nat_mul(a512, a513)?
                    };
                    let a515 = 1u64;
                    nat_add(a514, a515)?
                };
                nat_mul(a516, a517)?
            };
            let a519 = 1u64;
            nat_add(a518, a519)?
        };
        nat_sub(a520, a521)
    }, f20({
        let a524 = match v2 {
            Adt1::C0(_, h522, _, _) => {
                h522
            }
        };
        let a525 = match v3 {
            Adt1::C0(_, h523, _, _) => {
                h523
            }
        };
        nat_mul(a524, a525)?
    }, {
        let a542 = {
            let a526 = 1u64;
            let a527 = 1u64;
            nat_add(a526, a527)?
        };
        let a543 = {
            let a540 = {
                let a528 = 1u64;
                let a529 = 1u64;
                nat_add(a528, a529)?
            };
            let a541 = {
                let a538 = {
                    let a536 = {
                        let a530 = 1u64;
                        let a531 = 1u64;
                        nat_add(a530, a531)?
                    };
                    let a537 = {
                        let a534 = {
                            let a532 = 1u64;
                            let a533 = 1u64;
                            nat_add(a532, a533)?
                        };
                        let a535 = 1u64;
                        nat_mul(a534, a535)?
                    };
                    nat_mul(a536, a537)?
                };
                let a539 = 1u64;
                nat_add(a538, a539)?
            };
            nat_mul(a540, a541)?
        };
        nat_mul(a542, a543)?
    })?)
}

pub fn f11(v0: u64) -> R<u64> {
    let a568 = v0;
    let a569 = f6({
        let a566 = {
            let a564 = {
                let a544 = 1u64;
                let a545 = 1u64;
                nat_add(a544, a545)?
            };
            let a565 = {
                let a562 = {
                    let a560 = {
                        let a546 = 1u64;
                        let a547 = 1u64;
                        nat_add(a546, a547)?
                    };
                    let a561 = {
                        let a558 = {
                            let a556 = {
                                let a548 = 1u64;
                                let a549 = 1u64;
                                nat_add(a548, a549)?
                            };
                            let a557 = {
                                let a554 = {
                                    let a552 = {
                                        let a550 = 1u64;
                                        let a551 = 1u64;
                                        nat_add(a550, a551)?
                                    };
                                    let a553 = 1u64;
                                    nat_mul(a552, a553)?
                                };
                                let a555 = 1u64;
                                nat_add(a554, a555)?
                            };
                            nat_mul(a556, a557)?
                        };
                        let a559 = 1u64;
                        nat_add(a558, a559)?
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
    })?;
    let a570 = 0u64;
    Ok(nat_quot(a568, a569, a570))
}

pub fn f12(v0: u64) -> R<u64> {
    let a595 = v0;
    let a596 = f6({
        let a593 = {
            let a591 = {
                let a571 = 1u64;
                let a572 = 1u64;
                nat_add(a571, a572)?
            };
            let a592 = {
                let a589 = {
                    let a587 = {
                        let a573 = 1u64;
                        let a574 = 1u64;
                        nat_add(a573, a574)?
                    };
                    let a588 = {
                        let a585 = {
                            let a583 = {
                                let a575 = 1u64;
                                let a576 = 1u64;
                                nat_add(a575, a576)?
                            };
                            let a584 = {
                                let a581 = {
                                    let a579 = {
                                        let a577 = 1u64;
                                        let a578 = 1u64;
                                        nat_add(a577, a578)?
                                    };
                                    let a580 = 1u64;
                                    nat_mul(a579, a580)?
                                };
                                let a582 = 1u64;
                                nat_add(a581, a582)?
                            };
                            nat_mul(a583, a584)?
                        };
                        let a586 = 1u64;
                        nat_add(a585, a586)?
                    };
                    nat_mul(a587, a588)?
                };
                let a590 = 1u64;
                nat_add(a589, a590)?
            };
            nat_mul(a591, a592)?
        };
        let a594 = 1u64;
        nat_add(a593, a594)?
    })?;
    let a597 = 0u64;
    Ok(nat_rem(a595, a596, a597))
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f21(v0)?;
    let v3: u64 = {
        let a610 = f22(v0)?;
        let a611 = {
            let a608 = {
                let a598 = 1u64;
                let a599 = 1u64;
                nat_add(a598, a599)?
            };
            let a609 = {
                let a606 = {
                    let a600 = 1u64;
                    let a601 = 1u64;
                    nat_add(a600, a601)?
                };
                let a607 = {
                    let a604 = {
                        let a602 = 1u64;
                        let a603 = 1u64;
                        nat_add(a602, a603)?
                    };
                    let a605 = 1u64;
                    nat_mul(a604, a605)?
                };
                nat_mul(a606, a607)?
            };
            nat_mul(a608, a609)?
        };
        nat_mul(a610, a611)?
    };
    let v4: u64 = f20({
        let a624 = f22(v1)?;
        let a625 = {
            let a622 = {
                let a612 = 1u64;
                let a613 = 1u64;
                nat_add(a612, a613)?
            };
            let a623 = {
                let a620 = {
                    let a614 = 1u64;
                    let a615 = 1u64;
                    nat_add(a614, a615)?
                };
                let a621 = {
                    let a618 = {
                        let a616 = 1u64;
                        let a617 = 1u64;
                        nat_add(a616, a617)?
                    };
                    let a619 = 1u64;
                    nat_mul(a618, a619)?
                };
                nat_mul(a620, a621)?
            };
            nat_mul(a622, a623)?
        };
        nat_mul(a624, a625)?
    }, {
        let a626 = v2;
        let a627 = f21(v1)?;
        nat_sub(a626, a627)
    })?;
    let a628 = f11(v0)?;
    let a629 = f11(v1)?;
    if nat_eq(a628, a629) {
        f19(f11(v0)?, v2, {
            let a630 = v3;
            let a631 = v4;
            nat_add(a630, a631)?
        })
    } else {
        let a632 = v3;
        let a633 = v4;
        if nat_eq(a632, a633) {
            Ok(0u64)
        } else {
            f19(f11(v0)?, v2, {
                let a634 = v3;
                let a635 = v4;
                nat_sub(a634, a635)
            })
        }
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a658 = f12(v0)?;
    let a659 = f6({
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
    let a684 = f6({
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

pub fn f18(v0: u64) -> R<Adt1> {
    f23(f21(v0)?, f22(v0)?, 0u64, f6({
        let a713 = {
            let a711 = {
                let a693 = 1u64;
                let a694 = 1u64;
                nat_add(a693, a694)?
            };
            let a712 = {
                let a709 = {
                    let a707 = {
                        let a695 = 1u64;
                        let a696 = 1u64;
                        nat_add(a695, a696)?
                    };
                    let a708 = {
                        let a705 = {
                            let a703 = {
                                let a697 = 1u64;
                                let a698 = 1u64;
                                nat_add(a697, a698)?
                            };
                            let a704 = {
                                let a701 = {
                                    let a699 = 1u64;
                                    let a700 = 1u64;
                                    nat_add(a699, a700)?
                                };
                                let a702 = 1u64;
                                nat_mul(a701, a702)?
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
            nat_mul(a711, a712)?
        };
        let a714 = 1u64;
        nat_add(a713, a714)?
    })?)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a737 = f6({
        let a735 = {
            let a733 = {
                let a715 = 1u64;
                let a716 = 1u64;
                nat_add(a715, a716)?
            };
            let a734 = {
                let a731 = {
                    let a729 = {
                        let a717 = 1u64;
                        let a718 = 1u64;
                        nat_add(a717, a718)?
                    };
                    let a730 = {
                        let a727 = {
                            let a719 = 1u64;
                            let a720 = 1u64;
                            nat_add(a719, a720)?
                        };
                        let a728 = {
                            let a725 = {
                                let a723 = {
                                    let a721 = 1u64;
                                    let a722 = 1u64;
                                    nat_add(a721, a722)?
                                };
                                let a724 = 1u64;
                                nat_mul(a723, a724)?
                            };
                            let a726 = 1u64;
                            nat_add(a725, a726)?
                        };
                        nat_mul(a727, a728)?
                    };
                    nat_mul(a729, a730)?
                };
                let a732 = 1u64;
                nat_add(a731, a732)?
            };
            nat_mul(a733, a734)?
        };
        let a736 = 1u64;
        nat_add(a735, a736)?
    })?;
    let a738 = v2;
    if nat_le(a737, a738) {
        f24(v0, {
            let a739 = v1;
            let a740 = 1u64;
            nat_add(a739, a740)?
        }, f20(v2, 1u64)?)
    } else {
        let v3: Adt1 = f23(v1, v2, {
            let a777 = {
                let a775 = {
                    let a741 = 1u64;
                    let a742 = 1u64;
                    nat_add(a741, a742)?
                };
                let a776 = {
                    let a773 = {
                        let a743 = 1u64;
                        let a744 = 1u64;
                        nat_add(a743, a744)?
                    };
                    let a774 = {
                        let a771 = {
                            let a745 = 1u64;
                            let a746 = 1u64;
                            nat_add(a745, a746)?
                        };
                        let a772 = {
                            let a769 = {
                                let a747 = 1u64;
                                let a748 = 1u64;
                                nat_add(a747, a748)?
                            };
                            let a770 = {
                                let a767 = {
                                    let a749 = 1u64;
                                    let a750 = 1u64;
                                    nat_add(a749, a750)?
                                };
                                let a768 = {
                                    let a765 = {
                                        let a751 = 1u64;
                                        let a752 = 1u64;
                                        nat_add(a751, a752)?
                                    };
                                    let a766 = {
                                        let a763 = {
                                            let a753 = 1u64;
                                            let a754 = 1u64;
                                            nat_add(a753, a754)?
                                        };
                                        let a764 = {
                                            let a761 = {
                                                let a755 = 1u64;
                                                let a756 = 1u64;
                                                nat_add(a755, a756)?
                                            };
                                            let a762 = {
                                                let a759 = {
                                                    let a757 = 1u64;
                                                    let a758 = 1u64;
                                                    nat_add(a757, a758)?
                                                };
                                                let a760 = 1u64;
                                                nat_mul(a759, a760)?
                                            };
                                            nat_mul(a761, a762)?
                                        };
                                        nat_mul(a763, a764)?
                                    };
                                    nat_mul(a765, a766)?
                                };
                                nat_mul(a767, a768)?
                            };
                            nat_mul(a769, a770)?
                        };
                        nat_mul(a771, a772)?
                    };
                    nat_mul(a773, a774)?
                };
                nat_mul(a775, a776)?
            };
            let a778 = 1u64;
            nat_add(a777, a778)?
        }, f6({
            let a797 = {
                let a779 = 1u64;
                let a780 = 1u64;
                nat_add(a779, a780)?
            };
            let a798 = {
                let a795 = {
                    let a793 = {
                        let a781 = 1u64;
                        let a782 = 1u64;
                        nat_add(a781, a782)?
                    };
                    let a794 = {
                        let a791 = {
                            let a783 = 1u64;
                            let a784 = 1u64;
                            nat_add(a783, a784)?
                        };
                        let a792 = {
                            let a789 = {
                                let a787 = {
                                    let a785 = 1u64;
                                    let a786 = 1u64;
                                    nat_add(a785, a786)?
                                };
                                let a788 = 1u64;
                                nat_mul(a787, a788)?
                            };
                            let a790 = 1u64;
                            nat_add(a789, a790)?
                        };
                        nat_mul(a791, a792)?
                    };
                    nat_mul(a793, a794)?
                };
                let a796 = 1u64;
                nat_add(a795, a796)?
            };
            nat_mul(a797, a798)?
        })?)?;
        f24(v0, match v3.clone() {
            Adt1::C0(h799, _, _, _) => {
                h799
            }
        }, match v3 {
            Adt1::C0(_, h800, _, _) => {
                h800
            }
        })
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a801 = v1;
    let a802 = 0u64;
    if nat_eq(a801, a802) {
        Ok(v0)
    } else {
        let a827 = {
            let a825 = {
                let a803 = 1u64;
                let a804 = 1u64;
                nat_add(a803, a804)?
            };
            let a826 = {
                let a823 = {
                    let a805 = 1u64;
                    let a806 = 1u64;
                    nat_add(a805, a806)?
                };
                let a824 = {
                    let a821 = {
                        let a807 = 1u64;
                        let a808 = 1u64;
                        nat_add(a807, a808)?
                    };
                    let a822 = {
                        let a819 = {
                            let a809 = 1u64;
                            let a810 = 1u64;
                            nat_add(a809, a810)?
                        };
                        let a820 = {
                            let a817 = {
                                let a811 = 1u64;
                                let a812 = 1u64;
                                nat_add(a811, a812)?
                            };
                            let a818 = {
                                let a815 = {
                                    let a813 = 1u64;
                                    let a814 = 1u64;
                                    nat_add(a813, a814)?
                                };
                                let a816 = 1u64;
                                nat_mul(a815, a816)?
                            };
                            nat_mul(a817, a818)?
                        };
                        nat_mul(a819, a820)?
                    };
                    nat_mul(a821, a822)?
                };
                nat_mul(a823, a824)?
            };
            nat_mul(a825, a826)?
        };
        let a828 = v1;
        if nat_le(a827, a828) {
            let a829 = v0;
            let a830 = 0u64;
            if nat_eq(a829, a830) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f6(v1)?;
            f25({
                let a831 = v0;
                let a832 = v2;
                let a833 = 0u64;
                nat_quot(a831, a832, a833)
            }, {
                let a837 = 0u64;
                let a838 = {
                    let a834 = v0;
                    let a835 = v2;
                    let a836 = 0u64;
                    nat_rem(a834, a835, a836)
                };
                nat_lt(a837, a838)
            })
        }
    }
}

pub fn f21(v0: u64) -> R<u64> {
    let a877 = {
        let a839 = f14(v0)?;
        let a840 = 0u64;
        if nat_eq(a839, a840) {
            1u64
        } else {
            f14(v0)?
        }
    };
    let a878 = {
        let a875 = {
            let a841 = 1u64;
            let a842 = 1u64;
            nat_add(a841, a842)?
        };
        let a876 = {
            let a873 = {
                let a843 = 1u64;
                let a844 = 1u64;
                nat_add(a843, a844)?
            };
            let a874 = {
                let a871 = {
                    let a845 = 1u64;
                    let a846 = 1u64;
                    nat_add(a845, a846)?
                };
                let a872 = {
                    let a869 = {
                        let a847 = 1u64;
                        let a848 = 1u64;
                        nat_add(a847, a848)?
                    };
                    let a870 = {
                        let a867 = {
                            let a849 = 1u64;
                            let a850 = 1u64;
                            nat_add(a849, a850)?
                        };
                        let a868 = {
                            let a865 = {
                                let a851 = 1u64;
                                let a852 = 1u64;
                                nat_add(a851, a852)?
                            };
                            let a866 = {
                                let a863 = {
                                    let a853 = 1u64;
                                    let a854 = 1u64;
                                    nat_add(a853, a854)?
                                };
                                let a864 = {
                                    let a861 = {
                                        let a855 = 1u64;
                                        let a856 = 1u64;
                                        nat_add(a855, a856)?
                                    };
                                    let a862 = {
                                        let a859 = {
                                            let a857 = 1u64;
                                            let a858 = 1u64;
                                            nat_add(a857, a858)?
                                        };
                                        let a860 = 1u64;
                                        nat_mul(a859, a860)?
                                    };
                                    nat_mul(a861, a862)?
                                };
                                nat_mul(a863, a864)?
                            };
                            nat_mul(a865, a866)?
                        };
                        nat_mul(a867, a868)?
                    };
                    nat_mul(a869, a870)?
                };
                nat_mul(a871, a872)?
            };
            nat_mul(a873, a874)?
        };
        nat_mul(a875, a876)?
    };
    nat_add(a877, a878)
}

pub fn f22(v0: u64) -> R<u64> {
    let a903 = f15(v0)?;
    let a904 = {
        let a879 = f14(v0)?;
        let a880 = 0u64;
        if nat_eq(a879, a880) {
            0u64
        } else {
            f6({
                let a901 = {
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
                                    let a891 = {
                                        let a885 = 1u64;
                                        let a886 = 1u64;
                                        nat_add(a885, a886)?
                                    };
                                    let a892 = {
                                        let a889 = {
                                            let a887 = 1u64;
                                            let a888 = 1u64;
                                            nat_add(a887, a888)?
                                        };
                                        let a890 = 1u64;
                                        nat_mul(a889, a890)?
                                    };
                                    nat_mul(a891, a892)?
                                };
                                let a894 = 1u64;
                                nat_add(a893, a894)?
                            };
                            nat_mul(a895, a896)?
                        };
                        let a898 = 1u64;
                        nat_add(a897, a898)?
                    };
                    nat_mul(a899, a900)?
                };
                let a902 = 1u64;
                nat_add(a901, a902)?
            })?
        }
    };
    nat_add(a903, a904)
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h905, _) = f26(Fn1::F27, {
        let a924 = {
            let a906 = 1u64;
            let a907 = 1u64;
            nat_add(a906, a907)?
        };
        let a925 = {
            let a922 = {
                let a908 = 1u64;
                let a909 = 1u64;
                nat_add(a908, a909)?
            };
            let a923 = {
                let a920 = {
                    let a910 = 1u64;
                    let a911 = 1u64;
                    nat_add(a910, a911)?
                };
                let a921 = {
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
                nat_mul(a920, a921)?
            };
            nat_mul(a922, a923)?
        };
        nat_mul(a924, a925)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h905)
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a964 = v1;
    let a965 = {
        let a962 = {
            let a960 = {
                let a926 = 1u64;
                let a927 = 1u64;
                nat_add(a926, a927)?
            };
            let a961 = {
                let a958 = {
                    let a928 = 1u64;
                    let a929 = 1u64;
                    nat_add(a928, a929)?
                };
                let a959 = {
                    let a956 = {
                        let a930 = 1u64;
                        let a931 = 1u64;
                        nat_add(a930, a931)?
                    };
                    let a957 = {
                        let a954 = {
                            let a932 = 1u64;
                            let a933 = 1u64;
                            nat_add(a932, a933)?
                        };
                        let a955 = {
                            let a952 = {
                                let a934 = 1u64;
                                let a935 = 1u64;
                                nat_add(a934, a935)?
                            };
                            let a953 = {
                                let a950 = {
                                    let a936 = 1u64;
                                    let a937 = 1u64;
                                    nat_add(a936, a937)?
                                };
                                let a951 = {
                                    let a948 = {
                                        let a938 = 1u64;
                                        let a939 = 1u64;
                                        nat_add(a938, a939)?
                                    };
                                    let a949 = {
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
        let a963 = 1u64;
        nat_add(a962, a963)?
    };
    if nat_lt(a964, a965) {
        f28(v0, {
            let a1002 = {
                let a1000 = {
                    let a966 = 1u64;
                    let a967 = 1u64;
                    nat_add(a966, a967)?
                };
                let a1001 = {
                    let a998 = {
                        let a968 = 1u64;
                        let a969 = 1u64;
                        nat_add(a968, a969)?
                    };
                    let a999 = {
                        let a996 = {
                            let a970 = 1u64;
                            let a971 = 1u64;
                            nat_add(a970, a971)?
                        };
                        let a997 = {
                            let a994 = {
                                let a972 = 1u64;
                                let a973 = 1u64;
                                nat_add(a972, a973)?
                            };
                            let a995 = {
                                let a992 = {
                                    let a974 = 1u64;
                                    let a975 = 1u64;
                                    nat_add(a974, a975)?
                                };
                                let a993 = {
                                    let a990 = {
                                        let a976 = 1u64;
                                        let a977 = 1u64;
                                        nat_add(a976, a977)?
                                    };
                                    let a991 = {
                                        let a988 = {
                                            let a978 = 1u64;
                                            let a979 = 1u64;
                                            nat_add(a978, a979)?
                                        };
                                        let a989 = {
                                            let a986 = {
                                                let a980 = 1u64;
                                                let a981 = 1u64;
                                                nat_add(a980, a981)?
                                            };
                                            let a987 = {
                                                let a984 = {
                                                    let a982 = 1u64;
                                                    let a983 = 1u64;
                                                    nat_add(a982, a983)?
                                                };
                                                let a985 = 1u64;
                                                nat_mul(a984, a985)?
                                            };
                                            nat_mul(a986, a987)?
                                        };
                                        nat_mul(a988, a989)?
                                    };
                                    nat_mul(a990, a991)?
                                };
                                nat_mul(a992, a993)?
                            };
                            nat_mul(a994, a995)?
                        };
                        nat_mul(a996, a997)?
                    };
                    nat_mul(a998, a999)?
                };
                nat_mul(a1000, a1001)?
            };
            let a1003 = 1u64;
            nat_add(a1002, a1003)?
        }, f20(v2, {
            let a1042 = {
                let a1040 = {
                    let a1038 = {
                        let a1004 = 1u64;
                        let a1005 = 1u64;
                        nat_add(a1004, a1005)?
                    };
                    let a1039 = {
                        let a1036 = {
                            let a1006 = 1u64;
                            let a1007 = 1u64;
                            nat_add(a1006, a1007)?
                        };
                        let a1037 = {
                            let a1034 = {
                                let a1008 = 1u64;
                                let a1009 = 1u64;
                                nat_add(a1008, a1009)?
                            };
                            let a1035 = {
                                let a1032 = {
                                    let a1010 = 1u64;
                                    let a1011 = 1u64;
                                    nat_add(a1010, a1011)?
                                };
                                let a1033 = {
                                    let a1030 = {
                                        let a1012 = 1u64;
                                        let a1013 = 1u64;
                                        nat_add(a1012, a1013)?
                                    };
                                    let a1031 = {
                                        let a1028 = {
                                            let a1014 = 1u64;
                                            let a1015 = 1u64;
                                            nat_add(a1014, a1015)?
                                        };
                                        let a1029 = {
                                            let a1026 = {
                                                let a1016 = 1u64;
                                                let a1017 = 1u64;
                                                nat_add(a1016, a1017)?
                                            };
                                            let a1027 = {
                                                let a1024 = {
                                                    let a1018 = 1u64;
                                                    let a1019 = 1u64;
                                                    nat_add(a1018, a1019)?
                                                };
                                                let a1025 = {
                                                    let a1022 = {
                                                        let a1020 = 1u64;
                                                        let a1021 = 1u64;
                                                        nat_add(a1020, a1021)?
                                                    };
                                                    let a1023 = 1u64;
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
                                nat_mul(a1032, a1033)?
                            };
                            nat_mul(a1034, a1035)?
                        };
                        nat_mul(a1036, a1037)?
                    };
                    nat_mul(a1038, a1039)?
                };
                let a1041 = 1u64;
                nat_add(a1040, a1041)?
            };
            let a1043 = v1;
            nat_sub(a1042, a1043)
        })?)
    } else {
        f28(v0, v1, v2)
    }
}

pub fn f25(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1051 = {
            let a1048 = v0;
            let a1049 = {
                let a1046 = {
                    let a1044 = 1u64;
                    let a1045 = 1u64;
                    nat_add(a1044, a1045)?
                };
                let a1047 = 1u64;
                nat_mul(a1046, a1047)?
            };
            let a1050 = 0u64;
            nat_rem(a1048, a1049, a1050)
        };
        let a1052 = 0u64;
        nat_eq(a1051, a1052)
    } else {
        false
    } {
        let a1053 = v0;
        let a1054 = 1u64;
        nat_add(a1053, a1054)
    } else {
        Ok(v0)
    }
}

pub fn f26(v0: Fn1, p1059: u64, p1060: Adt1) -> R<(Adt1, bool)> {
    let m1058 = (p1059, p1060);
    {
        let mut m1058 = m1058;
        loop {
            let step = {
                let (v1, v2) = m1058;
                let m1055 = v1;
                if m1055 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1055 - 1;
                    let m1057 = {
                        let c1056: Fn1 = v0.clone();
                        c1056.apply(v2.clone())?
                    };
                    match m1057 {
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
                Ok(next) => { m1058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f27(v0: Adt1) -> R<Option<Adt1>> {
    f29(v0)
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1073 = v2;
        let a1074 = {
            let a1071 = {
                let a1061 = 1u64;
                let a1062 = 1u64;
                nat_add(a1061, a1062)?
            };
            let a1072 = {
                let a1069 = {
                    let a1063 = 1u64;
                    let a1064 = 1u64;
                    nat_add(a1063, a1064)?
                };
                let a1070 = {
                    let a1067 = {
                        let a1065 = 1u64;
                        let a1066 = 1u64;
                        nat_add(a1065, a1066)?
                    };
                    let a1068 = 1u64;
                    nat_mul(a1067, a1068)?
                };
                nat_mul(a1069, a1070)?
            };
            nat_mul(a1071, a1072)?
        };
        let a1075 = 0u64;
        nat_rem(a1073, a1074, a1075)
    };
    let v4: u64 = {
        let a1088 = v2;
        let a1089 = {
            let a1086 = {
                let a1076 = 1u64;
                let a1077 = 1u64;
                nat_add(a1076, a1077)?
            };
            let a1087 = {
                let a1084 = {
                    let a1078 = 1u64;
                    let a1079 = 1u64;
                    nat_add(a1078, a1079)?
                };
                let a1085 = {
                    let a1082 = {
                        let a1080 = 1u64;
                        let a1081 = 1u64;
                        nat_add(a1080, a1081)?
                    };
                    let a1083 = 1u64;
                    nat_mul(a1082, a1083)?
                };
                nat_mul(a1084, a1085)?
            };
            nat_mul(a1086, a1087)?
        };
        let a1090 = 0u64;
        nat_quot(a1088, a1089, a1090)
    };
    let v5: bool = {
        let a1099 = {
            let a1097 = {
                let a1091 = 1u64;
                let a1092 = 1u64;
                nat_add(a1091, a1092)?
            };
            let a1098 = {
                let a1095 = {
                    let a1093 = 1u64;
                    let a1094 = 1u64;
                    nat_add(a1093, a1094)?
                };
                let a1096 = 1u64;
                nat_mul(a1095, a1096)?
            };
            nat_mul(a1097, a1098)?
        };
        let a1100 = v3;
        if nat_lt(a1099, a1100) {
            true
        } else {
            let a1109 = v3;
            let a1110 = {
                let a1107 = {
                    let a1101 = 1u64;
                    let a1102 = 1u64;
                    nat_add(a1101, a1102)?
                };
                let a1108 = {
                    let a1105 = {
                        let a1103 = 1u64;
                        let a1104 = 1u64;
                        nat_add(a1103, a1104)?
                    };
                    let a1106 = 1u64;
                    nat_mul(a1105, a1106)?
                };
                nat_mul(a1107, a1108)?
            };
            if nat_eq(a1109, a1110) {
                let a1118 = {
                    let a1115 = v4;
                    let a1116 = {
                        let a1113 = {
                            let a1111 = 1u64;
                            let a1112 = 1u64;
                            nat_add(a1111, a1112)?
                        };
                        let a1114 = 1u64;
                        nat_mul(a1113, a1114)?
                    };
                    let a1117 = 0u64;
                    nat_rem(a1115, a1116, a1117)
                };
                let a1119 = 1u64;
                nat_eq(a1118, a1119)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1120 = v4;
        let a1121 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1120, a1121)?
    };
    let v7: bool = {
        let a1150 = {
            let a1148 = {
                let a1124 = {
                    let a1122 = 1u64;
                    let a1123 = 1u64;
                    nat_add(a1122, a1123)?
                };
                let a1125 = 1u64;
                nat_mul(a1124, a1125)?
            };
            let a1149 = f6({
                let a1146 = {
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
                };
                let a1147 = 1u64;
                nat_add(a1146, a1147)?
            })?;
            nat_mul(a1148, a1149)?
        };
        let a1151 = v6;
        nat_le(a1150, a1151)
    };
    let v8: u64 = if v7 {
        let a1156 = v6;
        let a1157 = {
            let a1154 = {
                let a1152 = 1u64;
                let a1153 = 1u64;
                nat_add(a1152, a1153)?
            };
            let a1155 = 1u64;
            nat_mul(a1154, a1155)?
        };
        let a1158 = 0u64;
        nat_quot(a1156, a1157, a1158)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1159 = v1;
        let a1160 = 1u64;
        nat_add(a1159, a1160)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1199 = v9;
        let a1200 = {
            let a1197 = {
                let a1195 = {
                    let a1161 = 1u64;
                    let a1162 = 1u64;
                    nat_add(a1161, a1162)?
                };
                let a1196 = {
                    let a1193 = {
                        let a1163 = 1u64;
                        let a1164 = 1u64;
                        nat_add(a1163, a1164)?
                    };
                    let a1194 = {
                        let a1191 = {
                            let a1165 = 1u64;
                            let a1166 = 1u64;
                            nat_add(a1165, a1166)?
                        };
                        let a1192 = {
                            let a1189 = {
                                let a1167 = 1u64;
                                let a1168 = 1u64;
                                nat_add(a1167, a1168)?
                            };
                            let a1190 = {
                                let a1187 = {
                                    let a1169 = 1u64;
                                    let a1170 = 1u64;
                                    nat_add(a1169, a1170)?
                                };
                                let a1188 = {
                                    let a1185 = {
                                        let a1171 = 1u64;
                                        let a1172 = 1u64;
                                        nat_add(a1171, a1172)?
                                    };
                                    let a1186 = {
                                        let a1183 = {
                                            let a1173 = 1u64;
                                            let a1174 = 1u64;
                                            nat_add(a1173, a1174)?
                                        };
                                        let a1184 = {
                                            let a1181 = {
                                                let a1175 = 1u64;
                                                let a1176 = 1u64;
                                                nat_add(a1175, a1176)?
                                            };
                                            let a1182 = {
                                                let a1179 = {
                                                    let a1177 = 1u64;
                                                    let a1178 = 1u64;
                                                    nat_add(a1177, a1178)?
                                                };
                                                let a1180 = 1u64;
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
                        nat_mul(a1191, a1192)?
                    };
                    nat_mul(a1193, a1194)?
                };
                nat_mul(a1195, a1196)?
            };
            let a1198 = 1u64;
            nat_add(a1197, a1198)?
        };
        if if nat_eq(a1199, a1200) {
            let a1223 = v8;
            let a1224 = f6({
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
            })?;
            nat_lt(a1223, a1224)
        } else {
            false
        } {
            0u64
        } else {
            let a1261 = v9;
            let a1262 = {
                let a1259 = {
                    let a1225 = 1u64;
                    let a1226 = 1u64;
                    nat_add(a1225, a1226)?
                };
                let a1260 = {
                    let a1257 = {
                        let a1227 = 1u64;
                        let a1228 = 1u64;
                        nat_add(a1227, a1228)?
                    };
                    let a1258 = {
                        let a1255 = {
                            let a1229 = 1u64;
                            let a1230 = 1u64;
                            nat_add(a1229, a1230)?
                        };
                        let a1256 = {
                            let a1253 = {
                                let a1231 = 1u64;
                                let a1232 = 1u64;
                                nat_add(a1231, a1232)?
                            };
                            let a1254 = {
                                let a1251 = {
                                    let a1233 = 1u64;
                                    let a1234 = 1u64;
                                    nat_add(a1233, a1234)?
                                };
                                let a1252 = {
                                    let a1249 = {
                                        let a1235 = 1u64;
                                        let a1236 = 1u64;
                                        nat_add(a1235, a1236)?
                                    };
                                    let a1250 = {
                                        let a1247 = {
                                            let a1237 = 1u64;
                                            let a1238 = 1u64;
                                            nat_add(a1237, a1238)?
                                        };
                                        let a1248 = {
                                            let a1245 = {
                                                let a1239 = 1u64;
                                                let a1240 = 1u64;
                                                nat_add(a1239, a1240)?
                                            };
                                            let a1246 = {
                                                let a1243 = {
                                                    let a1241 = 1u64;
                                                    let a1242 = 1u64;
                                                    nat_add(a1241, a1242)?
                                                };
                                                let a1244 = 1u64;
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
                        nat_mul(a1255, a1256)?
                    };
                    nat_mul(a1257, a1258)?
                };
                nat_mul(a1259, a1260)?
            };
            nat_sub(a1261, a1262)
        }
    };
    let a1263 = v8;
    let a1264 = 0u64;
    if nat_eq(a1263, a1264) {
        f9(v0)
    } else {
        let a1317 = {
            let a1315 = {
                let a1313 = {
                    let a1265 = 1u64;
                    let a1266 = 1u64;
                    nat_add(a1265, a1266)?
                };
                let a1314 = {
                    let a1311 = {
                        let a1309 = {
                            let a1267 = 1u64;
                            let a1268 = 1u64;
                            nat_add(a1267, a1268)?
                        };
                        let a1310 = {
                            let a1307 = {
                                let a1305 = {
                                    let a1269 = 1u64;
                                    let a1270 = 1u64;
                                    nat_add(a1269, a1270)?
                                };
                                let a1306 = {
                                    let a1303 = {
                                        let a1301 = {
                                            let a1271 = 1u64;
                                            let a1272 = 1u64;
                                            nat_add(a1271, a1272)?
                                        };
                                        let a1302 = {
                                            let a1299 = {
                                                let a1297 = {
                                                    let a1273 = 1u64;
                                                    let a1274 = 1u64;
                                                    nat_add(a1273, a1274)?
                                                };
                                                let a1298 = {
                                                    let a1295 = {
                                                        let a1293 = {
                                                            let a1275 = 1u64;
                                                            let a1276 = 1u64;
                                                            nat_add(a1275, a1276)?
                                                        };
                                                        let a1294 = {
                                                            let a1291 = {
                                                                let a1289 = {
                                                                    let a1277 = 1u64;
                                                                    let a1278 = 1u64;
                                                                    nat_add(a1277, a1278)?
                                                                };
                                                                let a1290 = {
                                                                    let a1287 = {
                                                                        let a1285 = {
                                                                            let a1279 = 1u64;
                                                                            let a1280 = 1u64;
                                                                            nat_add(a1279, a1280)?
                                                                        };
                                                                        let a1286 = {
                                                                            let a1283 = {
                                                                                let a1281 = 1u64;
                                                                                let a1282 = 1u64;
                                                                                nat_add(a1281, a1282)?
                                                                            };
                                                                            let a1284 = 1u64;
                                                                            nat_mul(a1283, a1284)?
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
                                                        };
                                                        nat_mul(a1293, a1294)?
                                                    };
                                                    let a1296 = 1u64;
                                                    nat_add(a1295, a1296)?
                                                };
                                                nat_mul(a1297, a1298)?
                                            };
                                            let a1300 = 1u64;
                                            nat_add(a1299, a1300)?
                                        };
                                        nat_mul(a1301, a1302)?
                                    };
                                    let a1304 = 1u64;
                                    nat_add(a1303, a1304)?
                                };
                                nat_mul(a1305, a1306)?
                            };
                            let a1308 = 1u64;
                            nat_add(a1307, a1308)?
                        };
                        nat_mul(a1309, a1310)?
                    };
                    let a1312 = 1u64;
                    nat_add(a1311, a1312)?
                };
                nat_mul(a1313, a1314)?
            };
            let a1316 = 1u64;
            nat_add(a1315, a1316)?
        };
        let a1318 = v9;
        if nat_le(a1317, a1318) {
            f7(v0)
        } else {
            let a1370 = {
                let a1343 = f9(v0)?;
                let a1344 = {
                    let a1341 = v10;
                    let a1342 = f6({
                        let a1339 = {
                            let a1337 = {
                                let a1319 = 1u64;
                                let a1320 = 1u64;
                                nat_add(a1319, a1320)?
                            };
                            let a1338 = {
                                let a1335 = {
                                    let a1333 = {
                                        let a1321 = 1u64;
                                        let a1322 = 1u64;
                                        nat_add(a1321, a1322)?
                                    };
                                    let a1334 = {
                                        let a1331 = {
                                            let a1329 = {
                                                let a1323 = 1u64;
                                                let a1324 = 1u64;
                                                nat_add(a1323, a1324)?
                                            };
                                            let a1330 = {
                                                let a1327 = {
                                                    let a1325 = 1u64;
                                                    let a1326 = 1u64;
                                                    nat_add(a1325, a1326)?
                                                };
                                                let a1328 = 1u64;
                                                nat_mul(a1327, a1328)?
                                            };
                                            nat_mul(a1329, a1330)?
                                        };
                                        let a1332 = 1u64;
                                        nat_add(a1331, a1332)?
                                    };
                                    nat_mul(a1333, a1334)?
                                };
                                let a1336 = 1u64;
                                nat_add(a1335, a1336)?
                            };
                            nat_mul(a1337, a1338)?
                        };
                        let a1340 = 1u64;
                        nat_add(a1339, a1340)?
                    })?;
                    nat_mul(a1341, a1342)?
                };
                nat_add(a1343, a1344)?
            };
            let a1371 = {
                let a1367 = v8;
                let a1368 = f6({
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
                })?;
                let a1369 = 0u64;
                nat_rem(a1367, a1368, a1369)
            };
            nat_add(a1370, a1371)
        }
    }
}

pub fn f29(v0: Adt1) -> R<Option<Adt1>> {
    let a1373 = 0u64;
    let a1374 = match v0.clone() {
        Adt1::C0(_, h1372, _, _) => {
            h1372
        }
    };
    if if if nat_lt(a1373, a1374) {
        let a1377 = match v0.clone() {
            Adt1::C0(_, h1375, _, _) => {
                h1375
            }
        };
        let a1378 = match v0.clone() {
            Adt1::C0(_, _, _, h1376) => {
                h1376
            }
        };
        nat_lt(a1377, a1378)
    } else {
        false
    } {
        let a1381 = match v0.clone() {
            Adt1::C0(_, _, h1379, _) => {
                h1379
            }
        };
        let a1382 = match v0.clone() {
            Adt1::C0(h1380, _, _, _) => {
                h1380
            }
        };
        nat_lt(a1381, a1382)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1384 = match v0.clone() {
                Adt1::C0(h1383, _, _, _) => {
                    h1383
                }
            };
            let a1385 = 1u64;
            nat_sub(a1384, a1385)
        }, {
            let a1388 = match v0.clone() {
                Adt1::C0(_, h1386, _, _) => {
                    h1386
                }
            };
            let a1389 = match v0.clone() {
                Adt1::C0(_, h1387, _, _) => {
                    h1387
                }
            };
            nat_add(a1388, a1389)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1390, _) => {
                h1390
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1391) => {
                h1391
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}

pub fn run(p0: bool, p1: u64, p2: bool, p3: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3)
}
