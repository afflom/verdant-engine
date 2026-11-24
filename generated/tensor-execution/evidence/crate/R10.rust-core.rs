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
    F21,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F21 => f21(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F31,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F31 => f31(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 2u64;
    if if nat_lt(a1, a2) {
        let a3 = v1;
        let a4 = 4294967296u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        Ok(Some({
            let a5 = v0;
            let a6 = 0u64;
            if nat_eq(a5, a6) {
                f1(v1)?
            } else {
                f2(v1, f1(v1)?)?
            }
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64) -> R<u64> {
    let v1: u64 = f3(f4(v0, 1077936128u64)?, 1086324736u64)?;
    if f5(1065353216u64, v1)? {
        Ok(1065353216u64)
    } else {
        if f5(0u64, v1)? {
            Ok(v1)
        } else {
            Ok(0u64)
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    if if if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        true
    } else {
        if f7(v0)? {
            f8(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f7(v1)? {
            f8(v0)?
        } else {
            false
        }
    } {
        let a93 = {
            let a71 = {
                let a47 = {
                    let a45 = {
                        let a7 = 1u64;
                        let a8 = 1u64;
                        nat_add(a7, a8)?
                    };
                    let a46 = {
                        let a43 = {
                            let a41 = {
                                let a9 = 1u64;
                                let a10 = 1u64;
                                nat_add(a9, a10)?
                            };
                            let a42 = {
                                let a39 = {
                                    let a37 = {
                                        let a11 = 1u64;
                                        let a12 = 1u64;
                                        nat_add(a11, a12)?
                                    };
                                    let a38 = {
                                        let a35 = {
                                            let a33 = {
                                                let a13 = 1u64;
                                                let a14 = 1u64;
                                                nat_add(a13, a14)?
                                            };
                                            let a34 = {
                                                let a31 = {
                                                    let a29 = {
                                                        let a15 = 1u64;
                                                        let a16 = 1u64;
                                                        nat_add(a15, a16)?
                                                    };
                                                    let a30 = {
                                                        let a27 = {
                                                            let a25 = {
                                                                let a17 = 1u64;
                                                                let a18 = 1u64;
                                                                nat_add(a17, a18)?
                                                            };
                                                            let a26 = {
                                                                let a23 = {
                                                                    let a21 = {
                                                                        let a19 = 1u64;
                                                                        let a20 = 1u64;
                                                                        nat_add(a19, a20)?
                                                                    };
                                                                    let a22 = 1u64;
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
                    nat_mul(a45, a46)?
                };
                let a48 = 1u64;
                nat_add(a47, a48)?
            };
            let a72 = f9({
                let a69 = {
                    let a67 = {
                        let a49 = 1u64;
                        let a50 = 1u64;
                        nat_add(a49, a50)?
                    };
                    let a68 = {
                        let a65 = {
                            let a63 = {
                                let a51 = 1u64;
                                let a52 = 1u64;
                                nat_add(a51, a52)?
                            };
                            let a64 = {
                                let a61 = {
                                    let a59 = {
                                        let a53 = 1u64;
                                        let a54 = 1u64;
                                        nat_add(a53, a54)?
                                    };
                                    let a60 = {
                                        let a57 = {
                                            let a55 = 1u64;
                                            let a56 = 1u64;
                                            nat_add(a55, a56)?
                                        };
                                        let a58 = 1u64;
                                        nat_mul(a57, a58)?
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
                    };
                    nat_mul(a67, a68)?
                };
                let a70 = 1u64;
                nat_add(a69, a70)?
            })?;
            nat_mul(a71, a72)?
        };
        let a94 = f9({
            let a91 = {
                let a73 = 1u64;
                let a74 = 1u64;
                nat_add(a73, a74)?
            };
            let a92 = {
                let a89 = {
                    let a87 = {
                        let a75 = 1u64;
                        let a76 = 1u64;
                        nat_add(a75, a76)?
                    };
                    let a88 = {
                        let a85 = {
                            let a83 = {
                                let a77 = 1u64;
                                let a78 = 1u64;
                                nat_add(a77, a78)?
                            };
                            let a84 = {
                                let a81 = {
                                    let a79 = 1u64;
                                    let a80 = 1u64;
                                    nat_add(a79, a80)?
                                };
                                let a82 = 1u64;
                                nat_mul(a81, a82)?
                            };
                            nat_mul(a83, a84)?
                        };
                        let a86 = 1u64;
                        nat_add(a85, a86)?
                    };
                    nat_mul(a87, a88)?
                };
                let a90 = 1u64;
                nat_add(a89, a90)?
            };
            nat_mul(a91, a92)?
        })?;
        nat_add(a93, a94)
    } else {
        if if f7(v0)? {
            true
        } else {
            f7(v1)?
        } {
            f10(f11(v0, v1)?)
        } else {
            if if f8(v0)? {
                true
            } else {
                f8(v1)?
            } {
                f12(f11(v0, v1)?)
            } else {
                f13(v0, v1)
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    if if if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        true
    } else {
        if f7(v0)? {
            f7(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f8(v0)? {
            f8(v1)?
        } else {
            false
        }
    } {
        let a181 = {
            let a159 = {
                let a135 = {
                    let a133 = {
                        let a95 = 1u64;
                        let a96 = 1u64;
                        nat_add(a95, a96)?
                    };
                    let a134 = {
                        let a131 = {
                            let a129 = {
                                let a97 = 1u64;
                                let a98 = 1u64;
                                nat_add(a97, a98)?
                            };
                            let a130 = {
                                let a127 = {
                                    let a125 = {
                                        let a99 = 1u64;
                                        let a100 = 1u64;
                                        nat_add(a99, a100)?
                                    };
                                    let a126 = {
                                        let a123 = {
                                            let a121 = {
                                                let a101 = 1u64;
                                                let a102 = 1u64;
                                                nat_add(a101, a102)?
                                            };
                                            let a122 = {
                                                let a119 = {
                                                    let a117 = {
                                                        let a103 = 1u64;
                                                        let a104 = 1u64;
                                                        nat_add(a103, a104)?
                                                    };
                                                    let a118 = {
                                                        let a115 = {
                                                            let a113 = {
                                                                let a105 = 1u64;
                                                                let a106 = 1u64;
                                                                nat_add(a105, a106)?
                                                            };
                                                            let a114 = {
                                                                let a111 = {
                                                                    let a109 = {
                                                                        let a107 = 1u64;
                                                                        let a108 = 1u64;
                                                                        nat_add(a107, a108)?
                                                                    };
                                                                    let a110 = 1u64;
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
                    nat_mul(a133, a134)?
                };
                let a136 = 1u64;
                nat_add(a135, a136)?
            };
            let a160 = f9({
                let a157 = {
                    let a155 = {
                        let a137 = 1u64;
                        let a138 = 1u64;
                        nat_add(a137, a138)?
                    };
                    let a156 = {
                        let a153 = {
                            let a151 = {
                                let a139 = 1u64;
                                let a140 = 1u64;
                                nat_add(a139, a140)?
                            };
                            let a152 = {
                                let a149 = {
                                    let a147 = {
                                        let a141 = 1u64;
                                        let a142 = 1u64;
                                        nat_add(a141, a142)?
                                    };
                                    let a148 = {
                                        let a145 = {
                                            let a143 = 1u64;
                                            let a144 = 1u64;
                                            nat_add(a143, a144)?
                                        };
                                        let a146 = 1u64;
                                        nat_mul(a145, a146)?
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
                    };
                    nat_mul(a155, a156)?
                };
                let a158 = 1u64;
                nat_add(a157, a158)?
            })?;
            nat_mul(a159, a160)?
        };
        let a182 = f9({
            let a179 = {
                let a161 = 1u64;
                let a162 = 1u64;
                nat_add(a161, a162)?
            };
            let a180 = {
                let a177 = {
                    let a175 = {
                        let a163 = 1u64;
                        let a164 = 1u64;
                        nat_add(a163, a164)?
                    };
                    let a176 = {
                        let a173 = {
                            let a171 = {
                                let a165 = 1u64;
                                let a166 = 1u64;
                                nat_add(a165, a166)?
                            };
                            let a172 = {
                                let a169 = {
                                    let a167 = 1u64;
                                    let a168 = 1u64;
                                    nat_add(a167, a168)?
                                };
                                let a170 = 1u64;
                                nat_mul(a169, a170)?
                            };
                            nat_mul(a171, a172)?
                        };
                        let a174 = 1u64;
                        nat_add(a173, a174)?
                    };
                    nat_mul(a175, a176)?
                };
                let a178 = 1u64;
                nat_add(a177, a178)?
            };
            nat_mul(a179, a180)?
        })?;
        nat_add(a181, a182)
    } else {
        if if f7(v0)? {
            true
        } else {
            f8(v1)?
        } {
            f10(f11(v0, v1)?)
        } else {
            if if f7(v1)? {
                true
            } else {
                f8(v0)?
            } {
                f12(f11(v0, v1)?)
            } else {
                f14(v0, v1)
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        let a269 = {
            let a247 = {
                let a223 = {
                    let a221 = {
                        let a183 = 1u64;
                        let a184 = 1u64;
                        nat_add(a183, a184)?
                    };
                    let a222 = {
                        let a219 = {
                            let a217 = {
                                let a185 = 1u64;
                                let a186 = 1u64;
                                nat_add(a185, a186)?
                            };
                            let a218 = {
                                let a215 = {
                                    let a213 = {
                                        let a187 = 1u64;
                                        let a188 = 1u64;
                                        nat_add(a187, a188)?
                                    };
                                    let a214 = {
                                        let a211 = {
                                            let a209 = {
                                                let a189 = 1u64;
                                                let a190 = 1u64;
                                                nat_add(a189, a190)?
                                            };
                                            let a210 = {
                                                let a207 = {
                                                    let a205 = {
                                                        let a191 = 1u64;
                                                        let a192 = 1u64;
                                                        nat_add(a191, a192)?
                                                    };
                                                    let a206 = {
                                                        let a203 = {
                                                            let a201 = {
                                                                let a193 = 1u64;
                                                                let a194 = 1u64;
                                                                nat_add(a193, a194)?
                                                            };
                                                            let a202 = {
                                                                let a199 = {
                                                                    let a197 = {
                                                                        let a195 = 1u64;
                                                                        let a196 = 1u64;
                                                                        nat_add(a195, a196)?
                                                                    };
                                                                    let a198 = 1u64;
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
                    nat_mul(a221, a222)?
                };
                let a224 = 1u64;
                nat_add(a223, a224)?
            };
            let a248 = f9({
                let a245 = {
                    let a243 = {
                        let a225 = 1u64;
                        let a226 = 1u64;
                        nat_add(a225, a226)?
                    };
                    let a244 = {
                        let a241 = {
                            let a239 = {
                                let a227 = 1u64;
                                let a228 = 1u64;
                                nat_add(a227, a228)?
                            };
                            let a240 = {
                                let a237 = {
                                    let a235 = {
                                        let a229 = 1u64;
                                        let a230 = 1u64;
                                        nat_add(a229, a230)?
                                    };
                                    let a236 = {
                                        let a233 = {
                                            let a231 = 1u64;
                                            let a232 = 1u64;
                                            nat_add(a231, a232)?
                                        };
                                        let a234 = 1u64;
                                        nat_mul(a233, a234)?
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
            })?;
            nat_mul(a247, a248)?
        };
        let a270 = f9({
            let a267 = {
                let a249 = 1u64;
                let a250 = 1u64;
                nat_add(a249, a250)?
            };
            let a268 = {
                let a265 = {
                    let a263 = {
                        let a251 = 1u64;
                        let a252 = 1u64;
                        nat_add(a251, a252)?
                    };
                    let a264 = {
                        let a261 = {
                            let a259 = {
                                let a253 = 1u64;
                                let a254 = 1u64;
                                nat_add(a253, a254)?
                            };
                            let a260 = {
                                let a257 = {
                                    let a255 = 1u64;
                                    let a256 = 1u64;
                                    nat_add(a255, a256)?
                                };
                                let a258 = 1u64;
                                nat_mul(a257, a258)?
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
        })?;
        nat_add(a269, a270)
    } else {
        if if f7(v0)? {
            f7(v1)?
        } else {
            false
        } {
            let a271 = f15(v0)?;
            let a272 = f15(v1)?;
            if nat_eq(a271, a272) {
                Ok(v0)
            } else {
                let a359 = {
                    let a337 = {
                        let a313 = {
                            let a311 = {
                                let a273 = 1u64;
                                let a274 = 1u64;
                                nat_add(a273, a274)?
                            };
                            let a312 = {
                                let a309 = {
                                    let a307 = {
                                        let a275 = 1u64;
                                        let a276 = 1u64;
                                        nat_add(a275, a276)?
                                    };
                                    let a308 = {
                                        let a305 = {
                                            let a303 = {
                                                let a277 = 1u64;
                                                let a278 = 1u64;
                                                nat_add(a277, a278)?
                                            };
                                            let a304 = {
                                                let a301 = {
                                                    let a299 = {
                                                        let a279 = 1u64;
                                                        let a280 = 1u64;
                                                        nat_add(a279, a280)?
                                                    };
                                                    let a300 = {
                                                        let a297 = {
                                                            let a295 = {
                                                                let a281 = 1u64;
                                                                let a282 = 1u64;
                                                                nat_add(a281, a282)?
                                                            };
                                                            let a296 = {
                                                                let a293 = {
                                                                    let a291 = {
                                                                        let a283 = 1u64;
                                                                        let a284 = 1u64;
                                                                        nat_add(a283, a284)?
                                                                    };
                                                                    let a292 = {
                                                                        let a289 = {
                                                                            let a287 = {
                                                                                let a285 = 1u64;
                                                                                let a286 = 1u64;
                                                                                nat_add(a285, a286)?
                                                                            };
                                                                            let a288 = 1u64;
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
                            nat_mul(a311, a312)?
                        };
                        let a314 = 1u64;
                        nat_add(a313, a314)?
                    };
                    let a338 = f9({
                        let a335 = {
                            let a333 = {
                                let a315 = 1u64;
                                let a316 = 1u64;
                                nat_add(a315, a316)?
                            };
                            let a334 = {
                                let a331 = {
                                    let a329 = {
                                        let a317 = 1u64;
                                        let a318 = 1u64;
                                        nat_add(a317, a318)?
                                    };
                                    let a330 = {
                                        let a327 = {
                                            let a325 = {
                                                let a319 = 1u64;
                                                let a320 = 1u64;
                                                nat_add(a319, a320)?
                                            };
                                            let a326 = {
                                                let a323 = {
                                                    let a321 = 1u64;
                                                    let a322 = 1u64;
                                                    nat_add(a321, a322)?
                                                };
                                                let a324 = 1u64;
                                                nat_mul(a323, a324)?
                                            };
                                            nat_mul(a325, a326)?
                                        };
                                        let a328 = 1u64;
                                        nat_add(a327, a328)?
                                    };
                                    nat_mul(a329, a330)?
                                };
                                let a332 = 1u64;
                                nat_add(a331, a332)?
                            };
                            nat_mul(a333, a334)?
                        };
                        let a336 = 1u64;
                        nat_add(a335, a336)?
                    })?;
                    nat_mul(a337, a338)?
                };
                let a360 = f9({
                    let a357 = {
                        let a339 = 1u64;
                        let a340 = 1u64;
                        nat_add(a339, a340)?
                    };
                    let a358 = {
                        let a355 = {
                            let a353 = {
                                let a341 = 1u64;
                                let a342 = 1u64;
                                nat_add(a341, a342)?
                            };
                            let a354 = {
                                let a351 = {
                                    let a349 = {
                                        let a343 = 1u64;
                                        let a344 = 1u64;
                                        nat_add(a343, a344)?
                                    };
                                    let a350 = {
                                        let a347 = {
                                            let a345 = 1u64;
                                            let a346 = 1u64;
                                            nat_add(a345, a346)?
                                        };
                                        let a348 = 1u64;
                                        nat_mul(a347, a348)?
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
                    nat_mul(a357, a358)?
                })?;
                nat_add(a359, a360)
            }
        } else {
            if f7(v0)? {
                Ok(v0)
            } else {
                if f7(v1)? {
                    Ok(v1)
                } else {
                    let a361 = f16(v1)?;
                    let a362 = f16(v0)?;
                    if nat_le(a361, a362) {
                        f17(v0, v1)
                    } else {
                        f17(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<bool> {
    if if f6(v0)? {
        true
    } else {
        f6(v1)?
    } {
        Ok(false)
    } else {
        if if f8(v0)? {
            f8(v1)?
        } else {
            false
        } {
            Ok(false)
        } else {
            let a363 = f15(v0)?;
            let a364 = f15(v1)?;
            if nat_eq(a363, a364) {
                let a365 = f15(v0)?;
                let a366 = 0u64;
                if nat_eq(a365, a366) {
                    let a367 = f16(v0)?;
                    let a368 = f16(v1)?;
                    Ok(nat_lt(a367, a368))
                } else {
                    let a369 = f16(v1)?;
                    let a370 = f16(v0)?;
                    Ok(nat_lt(a369, a370))
                }
            } else {
                let a371 = f15(v0)?;
                let a372 = 1u64;
                Ok(nat_eq(a371, a372))
            }
        }
    }
}

pub fn f6(v0: u64) -> R<bool> {
    let a415 = f18(v0)?;
    let a416 = {
        let a413 = {
            let a411 = {
                let a373 = 1u64;
                let a374 = 1u64;
                nat_add(a373, a374)?
            };
            let a412 = {
                let a409 = {
                    let a407 = {
                        let a375 = 1u64;
                        let a376 = 1u64;
                        nat_add(a375, a376)?
                    };
                    let a408 = {
                        let a405 = {
                            let a403 = {
                                let a377 = 1u64;
                                let a378 = 1u64;
                                nat_add(a377, a378)?
                            };
                            let a404 = {
                                let a401 = {
                                    let a399 = {
                                        let a379 = 1u64;
                                        let a380 = 1u64;
                                        nat_add(a379, a380)?
                                    };
                                    let a400 = {
                                        let a397 = {
                                            let a395 = {
                                                let a381 = 1u64;
                                                let a382 = 1u64;
                                                nat_add(a381, a382)?
                                            };
                                            let a396 = {
                                                let a393 = {
                                                    let a391 = {
                                                        let a383 = 1u64;
                                                        let a384 = 1u64;
                                                        nat_add(a383, a384)?
                                                    };
                                                    let a392 = {
                                                        let a389 = {
                                                            let a387 = {
                                                                let a385 = 1u64;
                                                                let a386 = 1u64;
                                                                nat_add(a385, a386)?
                                                            };
                                                            let a388 = 1u64;
                                                            nat_mul(a387, a388)?
                                                        };
                                                        let a390 = 1u64;
                                                        nat_add(a389, a390)?
                                                    };
                                                    nat_mul(a391, a392)?
                                                };
                                                let a394 = 1u64;
                                                nat_add(a393, a394)?
                                            };
                                            nat_mul(a395, a396)?
                                        };
                                        let a398 = 1u64;
                                        nat_add(a397, a398)?
                                    };
                                    nat_mul(a399, a400)?
                                };
                                let a402 = 1u64;
                                nat_add(a401, a402)?
                            };
                            nat_mul(a403, a404)?
                        };
                        let a406 = 1u64;
                        nat_add(a405, a406)?
                    };
                    nat_mul(a407, a408)?
                };
                let a410 = 1u64;
                nat_add(a409, a410)?
            };
            nat_mul(a411, a412)?
        };
        let a414 = 1u64;
        nat_add(a413, a414)?
    };
    if nat_eq(a415, a416) {
        let a417 = 0u64;
        let a418 = f19(v0)?;
        Ok(nat_lt(a417, a418))
    } else {
        Ok(false)
    }
}

pub fn f7(v0: u64) -> R<bool> {
    let a461 = f18(v0)?;
    let a462 = {
        let a459 = {
            let a457 = {
                let a419 = 1u64;
                let a420 = 1u64;
                nat_add(a419, a420)?
            };
            let a458 = {
                let a455 = {
                    let a453 = {
                        let a421 = 1u64;
                        let a422 = 1u64;
                        nat_add(a421, a422)?
                    };
                    let a454 = {
                        let a451 = {
                            let a449 = {
                                let a423 = 1u64;
                                let a424 = 1u64;
                                nat_add(a423, a424)?
                            };
                            let a450 = {
                                let a447 = {
                                    let a445 = {
                                        let a425 = 1u64;
                                        let a426 = 1u64;
                                        nat_add(a425, a426)?
                                    };
                                    let a446 = {
                                        let a443 = {
                                            let a441 = {
                                                let a427 = 1u64;
                                                let a428 = 1u64;
                                                nat_add(a427, a428)?
                                            };
                                            let a442 = {
                                                let a439 = {
                                                    let a437 = {
                                                        let a429 = 1u64;
                                                        let a430 = 1u64;
                                                        nat_add(a429, a430)?
                                                    };
                                                    let a438 = {
                                                        let a435 = {
                                                            let a433 = {
                                                                let a431 = 1u64;
                                                                let a432 = 1u64;
                                                                nat_add(a431, a432)?
                                                            };
                                                            let a434 = 1u64;
                                                            nat_mul(a433, a434)?
                                                        };
                                                        let a436 = 1u64;
                                                        nat_add(a435, a436)?
                                                    };
                                                    nat_mul(a437, a438)?
                                                };
                                                let a440 = 1u64;
                                                nat_add(a439, a440)?
                                            };
                                            nat_mul(a441, a442)?
                                        };
                                        let a444 = 1u64;
                                        nat_add(a443, a444)?
                                    };
                                    nat_mul(a445, a446)?
                                };
                                let a448 = 1u64;
                                nat_add(a447, a448)?
                            };
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
    if nat_eq(a461, a462) {
        let a463 = f19(v0)?;
        let a464 = 0u64;
        Ok(nat_eq(a463, a464))
    } else {
        Ok(false)
    }
}

pub fn f8(v0: u64) -> R<bool> {
    let a465 = f16(v0)?;
    let a466 = 0u64;
    Ok(nat_eq(a465, a466))
}

pub fn f9(v0: u64) -> R<u64> {
    f20(Fn0::F21, v0, 1u64)
}

pub fn f10(v0: u64) -> R<u64> {
    let a533 = f12(v0)?;
    let a534 = {
        let a531 = {
            let a507 = {
                let a505 = {
                    let a467 = 1u64;
                    let a468 = 1u64;
                    nat_add(a467, a468)?
                };
                let a506 = {
                    let a503 = {
                        let a501 = {
                            let a469 = 1u64;
                            let a470 = 1u64;
                            nat_add(a469, a470)?
                        };
                        let a502 = {
                            let a499 = {
                                let a497 = {
                                    let a471 = 1u64;
                                    let a472 = 1u64;
                                    nat_add(a471, a472)?
                                };
                                let a498 = {
                                    let a495 = {
                                        let a493 = {
                                            let a473 = 1u64;
                                            let a474 = 1u64;
                                            nat_add(a473, a474)?
                                        };
                                        let a494 = {
                                            let a491 = {
                                                let a489 = {
                                                    let a475 = 1u64;
                                                    let a476 = 1u64;
                                                    nat_add(a475, a476)?
                                                };
                                                let a490 = {
                                                    let a487 = {
                                                        let a485 = {
                                                            let a477 = 1u64;
                                                            let a478 = 1u64;
                                                            nat_add(a477, a478)?
                                                        };
                                                        let a486 = {
                                                            let a483 = {
                                                                let a481 = {
                                                                    let a479 = 1u64;
                                                                    let a480 = 1u64;
                                                                    nat_add(a479, a480)?
                                                                };
                                                                let a482 = 1u64;
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
                                nat_mul(a497, a498)?
                            };
                            let a500 = 1u64;
                            nat_add(a499, a500)?
                        };
                        nat_mul(a501, a502)?
                    };
                    let a504 = 1u64;
                    nat_add(a503, a504)?
                };
                nat_mul(a505, a506)?
            };
            let a508 = 1u64;
            nat_add(a507, a508)?
        };
        let a532 = f9({
            let a529 = {
                let a527 = {
                    let a509 = 1u64;
                    let a510 = 1u64;
                    nat_add(a509, a510)?
                };
                let a528 = {
                    let a525 = {
                        let a523 = {
                            let a511 = 1u64;
                            let a512 = 1u64;
                            nat_add(a511, a512)?
                        };
                        let a524 = {
                            let a521 = {
                                let a519 = {
                                    let a513 = 1u64;
                                    let a514 = 1u64;
                                    nat_add(a513, a514)?
                                };
                                let a520 = {
                                    let a517 = {
                                        let a515 = 1u64;
                                        let a516 = 1u64;
                                        nat_add(a515, a516)?
                                    };
                                    let a518 = 1u64;
                                    nat_mul(a517, a518)?
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
        })?;
        nat_mul(a531, a532)?
    };
    nat_add(a533, a534)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a541 = {
        let a535 = f15(v0)?;
        let a536 = f15(v1)?;
        nat_add(a535, a536)?
    };
    let a542 = {
        let a539 = {
            let a537 = 1u64;
            let a538 = 1u64;
            nat_add(a537, a538)?
        };
        let a540 = 1u64;
        nat_mul(a539, a540)?
    };
    let a543 = 0u64;
    Ok(nat_rem(a541, a542, a543))
}

pub fn f12(v0: u64) -> R<u64> {
    let a568 = v0;
    let a569 = f9({
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
    nat_mul(a568, a569)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f22(v0)?;
    let v3: Adt0 = f22(v1)?;
    f23(f11(v0, v1)?, {
        let a624 = {
            let a572 = match v2.clone() {
                Adt0::C0(h570, _, _, _) => {
                    h570
                }
            };
            let a573 = match v3.clone() {
                Adt0::C0(h571, _, _, _) => {
                    h571
                }
            };
            nat_add(a572, a573)?
        };
        let a625 = {
            let a622 = {
                let a620 = {
                    let a574 = 1u64;
                    let a575 = 1u64;
                    nat_add(a574, a575)?
                };
                let a621 = {
                    let a618 = {
                        let a616 = {
                            let a576 = 1u64;
                            let a577 = 1u64;
                            nat_add(a576, a577)?
                        };
                        let a617 = {
                            let a614 = {
                                let a612 = {
                                    let a578 = 1u64;
                                    let a579 = 1u64;
                                    nat_add(a578, a579)?
                                };
                                let a613 = {
                                    let a610 = {
                                        let a608 = {
                                            let a580 = 1u64;
                                            let a581 = 1u64;
                                            nat_add(a580, a581)?
                                        };
                                        let a609 = {
                                            let a606 = {
                                                let a604 = {
                                                    let a582 = 1u64;
                                                    let a583 = 1u64;
                                                    nat_add(a582, a583)?
                                                };
                                                let a605 = {
                                                    let a602 = {
                                                        let a600 = {
                                                            let a584 = 1u64;
                                                            let a585 = 1u64;
                                                            nat_add(a584, a585)?
                                                        };
                                                        let a601 = {
                                                            let a598 = {
                                                                let a596 = {
                                                                    let a586 = 1u64;
                                                                    let a587 = 1u64;
                                                                    nat_add(a586, a587)?
                                                                };
                                                                let a597 = {
                                                                    let a594 = {
                                                                        let a588 = 1u64;
                                                                        let a589 = 1u64;
                                                                        nat_add(a588, a589)?
                                                                    };
                                                                    let a595 = {
                                                                        let a592 = {
                                                                            let a590 = 1u64;
                                                                            let a591 = 1u64;
                                                                            nat_add(a590, a591)?
                                                                        };
                                                                        let a593 = 1u64;
                                                                        nat_mul(a592, a593)?
                                                                    };
                                                                    nat_mul(a594, a595)?
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
                                nat_mul(a612, a613)?
                            };
                            let a615 = 1u64;
                            nat_add(a614, a615)?
                        };
                        nat_mul(a616, a617)?
                    };
                    let a619 = 1u64;
                    nat_add(a618, a619)?
                };
                nat_mul(a620, a621)?
            };
            let a623 = 1u64;
            nat_add(a622, a623)?
        };
        nat_sub(a624, a625)
    }, f24({
        let a628 = match v2 {
            Adt0::C0(_, h626, _, _) => {
                h626
            }
        };
        let a629 = match v3 {
            Adt0::C0(_, h627, _, _) => {
                h627
            }
        };
        nat_mul(a628, a629)?
    }, {
        let a646 = {
            let a630 = 1u64;
            let a631 = 1u64;
            nat_add(a630, a631)?
        };
        let a647 = {
            let a644 = {
                let a632 = 1u64;
                let a633 = 1u64;
                nat_add(a632, a633)?
            };
            let a645 = {
                let a642 = {
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
                let a643 = 1u64;
                nat_add(a642, a643)?
            };
            nat_mul(a644, a645)?
        };
        nat_mul(a646, a647)?
    })?)
}

pub fn f14(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt0 = f22(v0)?;
    let v3: Adt0 = f22(v1)?;
    let v4: bool = {
        let a650 = match v2.clone() {
            Adt0::C0(_, h648, _, _) => {
                h648
            }
        };
        let a651 = match v3.clone() {
            Adt0::C0(_, h649, _, _) => {
                h649
            }
        };
        nat_lt(a650, a651)
    };
    let v5: u64 = {
        let a695 = match v2.clone() {
            Adt0::C0(_, h652, _, _) => {
                h652
            }
        };
        let a696 = f9(if v4 {
            let a673 = {
                let a671 = {
                    let a653 = 1u64;
                    let a654 = 1u64;
                    nat_add(a653, a654)?
                };
                let a672 = {
                    let a669 = {
                        let a667 = {
                            let a655 = 1u64;
                            let a656 = 1u64;
                            nat_add(a655, a656)?
                        };
                        let a668 = {
                            let a665 = {
                                let a657 = 1u64;
                                let a658 = 1u64;
                                nat_add(a657, a658)?
                            };
                            let a666 = {
                                let a663 = {
                                    let a661 = {
                                        let a659 = 1u64;
                                        let a660 = 1u64;
                                        nat_add(a659, a660)?
                                    };
                                    let a662 = 1u64;
                                    nat_mul(a661, a662)?
                                };
                                let a664 = 1u64;
                                nat_add(a663, a664)?
                            };
                            nat_mul(a665, a666)?
                        };
                        nat_mul(a667, a668)?
                    };
                    let a670 = 1u64;
                    nat_add(a669, a670)?
                };
                nat_mul(a671, a672)?
            };
            let a674 = 1u64;
            nat_add(a673, a674)?
        } else {
            let a693 = {
                let a675 = 1u64;
                let a676 = 1u64;
                nat_add(a675, a676)?
            };
            let a694 = {
                let a691 = {
                    let a689 = {
                        let a677 = 1u64;
                        let a678 = 1u64;
                        nat_add(a677, a678)?
                    };
                    let a690 = {
                        let a687 = {
                            let a679 = 1u64;
                            let a680 = 1u64;
                            nat_add(a679, a680)?
                        };
                        let a688 = {
                            let a685 = {
                                let a683 = {
                                    let a681 = 1u64;
                                    let a682 = 1u64;
                                    nat_add(a681, a682)?
                                };
                                let a684 = 1u64;
                                nat_mul(a683, a684)?
                            };
                            let a686 = 1u64;
                            nat_add(a685, a686)?
                        };
                        nat_mul(a687, a688)?
                    };
                    nat_mul(a689, a690)?
                };
                let a692 = 1u64;
                nat_add(a691, a692)?
            };
            nat_mul(a693, a694)?
        })?;
        nat_mul(a695, a696)?
    };
    let v6: u64 = {
        let a753 = {
            let a751 = {
                let a748 = match v2 {
                    Adt0::C0(h697, _, _, _) => {
                        h697
                    }
                };
                let a749 = {
                    let a746 = {
                        let a744 = {
                            let a698 = 1u64;
                            let a699 = 1u64;
                            nat_add(a698, a699)?
                        };
                        let a745 = {
                            let a742 = {
                                let a740 = {
                                    let a700 = 1u64;
                                    let a701 = 1u64;
                                    nat_add(a700, a701)?
                                };
                                let a741 = {
                                    let a738 = {
                                        let a736 = {
                                            let a702 = 1u64;
                                            let a703 = 1u64;
                                            nat_add(a702, a703)?
                                        };
                                        let a737 = {
                                            let a734 = {
                                                let a732 = {
                                                    let a704 = 1u64;
                                                    let a705 = 1u64;
                                                    nat_add(a704, a705)?
                                                };
                                                let a733 = {
                                                    let a730 = {
                                                        let a728 = {
                                                            let a706 = 1u64;
                                                            let a707 = 1u64;
                                                            nat_add(a706, a707)?
                                                        };
                                                        let a729 = {
                                                            let a726 = {
                                                                let a724 = {
                                                                    let a708 = 1u64;
                                                                    let a709 = 1u64;
                                                                    nat_add(a708, a709)?
                                                                };
                                                                let a725 = {
                                                                    let a722 = {
                                                                        let a720 = {
                                                                            let a710 = 1u64;
                                                                            let a711 = 1u64;
                                                                            nat_add(a710, a711)?
                                                                        };
                                                                        let a721 = {
                                                                            let a718 = {
                                                                                let a712 = 1u64;
                                                                                let a713 = 1u64;
                                                                                nat_add(a712, a713)?
                                                                            };
                                                                            let a719 = {
                                                                                let a716 = {
                                                                                    let a714 = 1u64;
                                                                                    let a715 = 1u64;
                                                                                    nat_add(a714, a715)?
                                                                                };
                                                                                let a717 = 1u64;
                                                                                nat_mul(a716, a717)?
                                                                            };
                                                                            nat_mul(a718, a719)?
                                                                        };
                                                                        nat_mul(a720, a721)?
                                                                    };
                                                                    let a723 = 1u64;
                                                                    nat_add(a722, a723)?
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
                                        };
                                        nat_mul(a736, a737)?
                                    };
                                    let a739 = 1u64;
                                    nat_add(a738, a739)?
                                };
                                nat_mul(a740, a741)?
                            };
                            let a743 = 1u64;
                            nat_add(a742, a743)?
                        };
                        nat_mul(a744, a745)?
                    };
                    let a747 = 1u64;
                    nat_add(a746, a747)?
                };
                nat_add(a748, a749)?
            };
            let a752 = match v3.clone() {
                Adt0::C0(h750, _, _, _) => {
                    h750
                }
            };
            nat_sub(a751, a752)
        };
        let a754 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a753, a754)
    };
    let v7: u64 = {
        let a756 = v5;
        let a757 = match v3.clone() {
            Adt0::C0(_, h755, _, _) => {
                h755
            }
        };
        let a758 = 0u64;
        nat_quot(a756, a757, a758)
    };
    let v8: bool = {
        let a763 = 0u64;
        let a764 = {
            let a760 = v5;
            let a761 = match v3 {
                Adt0::C0(_, h759, _, _) => {
                    h759
                }
            };
            let a762 = 0u64;
            nat_rem(a760, a761, a762)
        };
        nat_lt(a763, a764)
    };
    f23(f11(v0, v1)?, v6, f25(v7, v8)?)
}

pub fn f15(v0: u64) -> R<u64> {
    let a789 = v0;
    let a790 = f9({
        let a787 = {
            let a785 = {
                let a765 = 1u64;
                let a766 = 1u64;
                nat_add(a765, a766)?
            };
            let a786 = {
                let a783 = {
                    let a781 = {
                        let a767 = 1u64;
                        let a768 = 1u64;
                        nat_add(a767, a768)?
                    };
                    let a782 = {
                        let a779 = {
                            let a777 = {
                                let a769 = 1u64;
                                let a770 = 1u64;
                                nat_add(a769, a770)?
                            };
                            let a778 = {
                                let a775 = {
                                    let a773 = {
                                        let a771 = 1u64;
                                        let a772 = 1u64;
                                        nat_add(a771, a772)?
                                    };
                                    let a774 = 1u64;
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
    })?;
    let a791 = 0u64;
    Ok(nat_quot(a789, a790, a791))
}

pub fn f16(v0: u64) -> R<u64> {
    let a816 = v0;
    let a817 = f9({
        let a814 = {
            let a812 = {
                let a792 = 1u64;
                let a793 = 1u64;
                nat_add(a792, a793)?
            };
            let a813 = {
                let a810 = {
                    let a808 = {
                        let a794 = 1u64;
                        let a795 = 1u64;
                        nat_add(a794, a795)?
                    };
                    let a809 = {
                        let a806 = {
                            let a804 = {
                                let a796 = 1u64;
                                let a797 = 1u64;
                                nat_add(a796, a797)?
                            };
                            let a805 = {
                                let a802 = {
                                    let a800 = {
                                        let a798 = 1u64;
                                        let a799 = 1u64;
                                        nat_add(a798, a799)?
                                    };
                                    let a801 = 1u64;
                                    nat_mul(a800, a801)?
                                };
                                let a803 = 1u64;
                                nat_add(a802, a803)?
                            };
                            nat_mul(a804, a805)?
                        };
                        let a807 = 1u64;
                        nat_add(a806, a807)?
                    };
                    nat_mul(a808, a809)?
                };
                let a811 = 1u64;
                nat_add(a810, a811)?
            };
            nat_mul(a812, a813)?
        };
        let a815 = 1u64;
        nat_add(a814, a815)?
    })?;
    let a818 = 0u64;
    Ok(nat_rem(a816, a817, a818))
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f26(v0)?;
    let v3: u64 = {
        let a831 = f27(v0)?;
        let a832 = {
            let a829 = {
                let a819 = 1u64;
                let a820 = 1u64;
                nat_add(a819, a820)?
            };
            let a830 = {
                let a827 = {
                    let a821 = 1u64;
                    let a822 = 1u64;
                    nat_add(a821, a822)?
                };
                let a828 = {
                    let a825 = {
                        let a823 = 1u64;
                        let a824 = 1u64;
                        nat_add(a823, a824)?
                    };
                    let a826 = 1u64;
                    nat_mul(a825, a826)?
                };
                nat_mul(a827, a828)?
            };
            nat_mul(a829, a830)?
        };
        nat_mul(a831, a832)?
    };
    let v4: u64 = f24({
        let a845 = f27(v1)?;
        let a846 = {
            let a843 = {
                let a833 = 1u64;
                let a834 = 1u64;
                nat_add(a833, a834)?
            };
            let a844 = {
                let a841 = {
                    let a835 = 1u64;
                    let a836 = 1u64;
                    nat_add(a835, a836)?
                };
                let a842 = {
                    let a839 = {
                        let a837 = 1u64;
                        let a838 = 1u64;
                        nat_add(a837, a838)?
                    };
                    let a840 = 1u64;
                    nat_mul(a839, a840)?
                };
                nat_mul(a841, a842)?
            };
            nat_mul(a843, a844)?
        };
        nat_mul(a845, a846)?
    }, {
        let a847 = v2;
        let a848 = f26(v1)?;
        nat_sub(a847, a848)
    })?;
    let a849 = f15(v0)?;
    let a850 = f15(v1)?;
    if nat_eq(a849, a850) {
        f23(f15(v0)?, v2, {
            let a851 = v3;
            let a852 = v4;
            nat_add(a851, a852)?
        })
    } else {
        let a853 = v3;
        let a854 = v4;
        if nat_eq(a853, a854) {
            Ok(0u64)
        } else {
            f23(f15(v0)?, v2, {
                let a855 = v3;
                let a856 = v4;
                nat_sub(a855, a856)
            })
        }
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a879 = f16(v0)?;
    let a880 = f9({
        let a877 = {
            let a875 = {
                let a857 = 1u64;
                let a858 = 1u64;
                nat_add(a857, a858)?
            };
            let a876 = {
                let a873 = {
                    let a871 = {
                        let a859 = 1u64;
                        let a860 = 1u64;
                        nat_add(a859, a860)?
                    };
                    let a872 = {
                        let a869 = {
                            let a867 = {
                                let a861 = 1u64;
                                let a862 = 1u64;
                                nat_add(a861, a862)?
                            };
                            let a868 = {
                                let a865 = {
                                    let a863 = 1u64;
                                    let a864 = 1u64;
                                    nat_add(a863, a864)?
                                };
                                let a866 = 1u64;
                                nat_mul(a865, a866)?
                            };
                            nat_mul(a867, a868)?
                        };
                        let a870 = 1u64;
                        nat_add(a869, a870)?
                    };
                    nat_mul(a871, a872)?
                };
                let a874 = 1u64;
                nat_add(a873, a874)?
            };
            nat_mul(a875, a876)?
        };
        let a878 = 1u64;
        nat_add(a877, a878)?
    })?;
    let a881 = 0u64;
    Ok(nat_quot(a879, a880, a881))
}

pub fn f19(v0: u64) -> R<u64> {
    let a904 = v0;
    let a905 = f9({
        let a902 = {
            let a900 = {
                let a882 = 1u64;
                let a883 = 1u64;
                nat_add(a882, a883)?
            };
            let a901 = {
                let a898 = {
                    let a896 = {
                        let a884 = 1u64;
                        let a885 = 1u64;
                        nat_add(a884, a885)?
                    };
                    let a897 = {
                        let a894 = {
                            let a892 = {
                                let a886 = 1u64;
                                let a887 = 1u64;
                                nat_add(a886, a887)?
                            };
                            let a893 = {
                                let a890 = {
                                    let a888 = 1u64;
                                    let a889 = 1u64;
                                    nat_add(a888, a889)?
                                };
                                let a891 = 1u64;
                                nat_mul(a890, a891)?
                            };
                            nat_mul(a892, a893)?
                        };
                        let a895 = 1u64;
                        nat_add(a894, a895)?
                    };
                    nat_mul(a896, a897)?
                };
                let a899 = 1u64;
                nat_add(a898, a899)?
            };
            nat_mul(a900, a901)?
        };
        let a903 = 1u64;
        nat_add(a902, a903)?
    })?;
    let a906 = 0u64;
    Ok(nat_rem(a904, a905, a906))
}

pub fn f20(v0: Fn0, p910: u64, p911: u64) -> R<u64> {
    let m909 = (p910, p911);
    {
        let mut m909 = m909;
        loop {
            let step = {
                let (v1, v2) = m909;
                let m907 = v1;
                if m907 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m907 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c908: Fn0 = v0.clone();
                        c908.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21(v0: u64) -> R<u64> {
    let a912 = v0;
    let a913 = v0;
    nat_add(a912, a913)
}

pub fn f22(v0: u64) -> R<Adt0> {
    f28(f26(v0)?, f27(v0)?, 0u64, f9({
        let a934 = {
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
                        let a927 = 1u64;
                        nat_add(a926, a927)?
                    };
                    nat_mul(a928, a929)?
                };
                let a931 = 1u64;
                nat_add(a930, a931)?
            };
            nat_mul(a932, a933)?
        };
        let a935 = 1u64;
        nat_add(a934, a935)?
    })?)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a958 = f9({
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
                            let a940 = 1u64;
                            let a941 = 1u64;
                            nat_add(a940, a941)?
                        };
                        let a949 = {
                            let a946 = {
                                let a944 = {
                                    let a942 = 1u64;
                                    let a943 = 1u64;
                                    nat_add(a942, a943)?
                                };
                                let a945 = 1u64;
                                nat_mul(a944, a945)?
                            };
                            let a947 = 1u64;
                            nat_add(a946, a947)?
                        };
                        nat_mul(a948, a949)?
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
    })?;
    let a959 = v2;
    if nat_le(a958, a959) {
        f29(v0, {
            let a960 = v1;
            let a961 = 1u64;
            nat_add(a960, a961)?
        }, f24(v2, 1u64)?)
    } else {
        let v3: Adt0 = f28(v1, v2, {
            let a998 = {
                let a996 = {
                    let a962 = 1u64;
                    let a963 = 1u64;
                    nat_add(a962, a963)?
                };
                let a997 = {
                    let a994 = {
                        let a964 = 1u64;
                        let a965 = 1u64;
                        nat_add(a964, a965)?
                    };
                    let a995 = {
                        let a992 = {
                            let a966 = 1u64;
                            let a967 = 1u64;
                            nat_add(a966, a967)?
                        };
                        let a993 = {
                            let a990 = {
                                let a968 = 1u64;
                                let a969 = 1u64;
                                nat_add(a968, a969)?
                            };
                            let a991 = {
                                let a988 = {
                                    let a970 = 1u64;
                                    let a971 = 1u64;
                                    nat_add(a970, a971)?
                                };
                                let a989 = {
                                    let a986 = {
                                        let a972 = 1u64;
                                        let a973 = 1u64;
                                        nat_add(a972, a973)?
                                    };
                                    let a987 = {
                                        let a984 = {
                                            let a974 = 1u64;
                                            let a975 = 1u64;
                                            nat_add(a974, a975)?
                                        };
                                        let a985 = {
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
            let a999 = 1u64;
            nat_add(a998, a999)?
        }, f9({
            let a1018 = {
                let a1000 = 1u64;
                let a1001 = 1u64;
                nat_add(a1000, a1001)?
            };
            let a1019 = {
                let a1016 = {
                    let a1014 = {
                        let a1002 = 1u64;
                        let a1003 = 1u64;
                        nat_add(a1002, a1003)?
                    };
                    let a1015 = {
                        let a1012 = {
                            let a1004 = 1u64;
                            let a1005 = 1u64;
                            nat_add(a1004, a1005)?
                        };
                        let a1013 = {
                            let a1010 = {
                                let a1008 = {
                                    let a1006 = 1u64;
                                    let a1007 = 1u64;
                                    nat_add(a1006, a1007)?
                                };
                                let a1009 = 1u64;
                                nat_mul(a1008, a1009)?
                            };
                            let a1011 = 1u64;
                            nat_add(a1010, a1011)?
                        };
                        nat_mul(a1012, a1013)?
                    };
                    nat_mul(a1014, a1015)?
                };
                let a1017 = 1u64;
                nat_add(a1016, a1017)?
            };
            nat_mul(a1018, a1019)?
        })?)?;
        f29(v0, match v3.clone() {
            Adt0::C0(h1020, _, _, _) => {
                h1020
            }
        }, match v3 {
            Adt0::C0(_, h1021, _, _) => {
                h1021
            }
        })
    }
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a1022 = v1;
    let a1023 = 0u64;
    if nat_eq(a1022, a1023) {
        Ok(v0)
    } else {
        let a1048 = {
            let a1046 = {
                let a1024 = 1u64;
                let a1025 = 1u64;
                nat_add(a1024, a1025)?
            };
            let a1047 = {
                let a1044 = {
                    let a1026 = 1u64;
                    let a1027 = 1u64;
                    nat_add(a1026, a1027)?
                };
                let a1045 = {
                    let a1042 = {
                        let a1028 = 1u64;
                        let a1029 = 1u64;
                        nat_add(a1028, a1029)?
                    };
                    let a1043 = {
                        let a1040 = {
                            let a1030 = 1u64;
                            let a1031 = 1u64;
                            nat_add(a1030, a1031)?
                        };
                        let a1041 = {
                            let a1038 = {
                                let a1032 = 1u64;
                                let a1033 = 1u64;
                                nat_add(a1032, a1033)?
                            };
                            let a1039 = {
                                let a1036 = {
                                    let a1034 = 1u64;
                                    let a1035 = 1u64;
                                    nat_add(a1034, a1035)?
                                };
                                let a1037 = 1u64;
                                nat_mul(a1036, a1037)?
                            };
                            nat_mul(a1038, a1039)?
                        };
                        nat_mul(a1040, a1041)?
                    };
                    nat_mul(a1042, a1043)?
                };
                nat_mul(a1044, a1045)?
            };
            nat_mul(a1046, a1047)?
        };
        let a1049 = v1;
        if nat_le(a1048, a1049) {
            let a1050 = v0;
            let a1051 = 0u64;
            if nat_eq(a1050, a1051) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f9(v1)?;
            f25({
                let a1052 = v0;
                let a1053 = v2;
                let a1054 = 0u64;
                nat_quot(a1052, a1053, a1054)
            }, {
                let a1058 = 0u64;
                let a1059 = {
                    let a1055 = v0;
                    let a1056 = v2;
                    let a1057 = 0u64;
                    nat_rem(a1055, a1056, a1057)
                };
                nat_lt(a1058, a1059)
            })
        }
    }
}

pub fn f25(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1067 = {
            let a1064 = v0;
            let a1065 = {
                let a1062 = {
                    let a1060 = 1u64;
                    let a1061 = 1u64;
                    nat_add(a1060, a1061)?
                };
                let a1063 = 1u64;
                nat_mul(a1062, a1063)?
            };
            let a1066 = 0u64;
            nat_rem(a1064, a1065, a1066)
        };
        let a1068 = 0u64;
        nat_eq(a1067, a1068)
    } else {
        false
    } {
        let a1069 = v0;
        let a1070 = 1u64;
        nat_add(a1069, a1070)
    } else {
        Ok(v0)
    }
}

pub fn f26(v0: u64) -> R<u64> {
    let a1109 = {
        let a1071 = f18(v0)?;
        let a1072 = 0u64;
        if nat_eq(a1071, a1072) {
            1u64
        } else {
            f18(v0)?
        }
    };
    let a1110 = {
        let a1107 = {
            let a1073 = 1u64;
            let a1074 = 1u64;
            nat_add(a1073, a1074)?
        };
        let a1108 = {
            let a1105 = {
                let a1075 = 1u64;
                let a1076 = 1u64;
                nat_add(a1075, a1076)?
            };
            let a1106 = {
                let a1103 = {
                    let a1077 = 1u64;
                    let a1078 = 1u64;
                    nat_add(a1077, a1078)?
                };
                let a1104 = {
                    let a1101 = {
                        let a1079 = 1u64;
                        let a1080 = 1u64;
                        nat_add(a1079, a1080)?
                    };
                    let a1102 = {
                        let a1099 = {
                            let a1081 = 1u64;
                            let a1082 = 1u64;
                            nat_add(a1081, a1082)?
                        };
                        let a1100 = {
                            let a1097 = {
                                let a1083 = 1u64;
                                let a1084 = 1u64;
                                nat_add(a1083, a1084)?
                            };
                            let a1098 = {
                                let a1095 = {
                                    let a1085 = 1u64;
                                    let a1086 = 1u64;
                                    nat_add(a1085, a1086)?
                                };
                                let a1096 = {
                                    let a1093 = {
                                        let a1087 = 1u64;
                                        let a1088 = 1u64;
                                        nat_add(a1087, a1088)?
                                    };
                                    let a1094 = {
                                        let a1091 = {
                                            let a1089 = 1u64;
                                            let a1090 = 1u64;
                                            nat_add(a1089, a1090)?
                                        };
                                        let a1092 = 1u64;
                                        nat_mul(a1091, a1092)?
                                    };
                                    nat_mul(a1093, a1094)?
                                };
                                nat_mul(a1095, a1096)?
                            };
                            nat_mul(a1097, a1098)?
                        };
                        nat_mul(a1099, a1100)?
                    };
                    nat_mul(a1101, a1102)?
                };
                nat_mul(a1103, a1104)?
            };
            nat_mul(a1105, a1106)?
        };
        nat_mul(a1107, a1108)?
    };
    nat_add(a1109, a1110)
}

pub fn f27(v0: u64) -> R<u64> {
    let a1135 = f19(v0)?;
    let a1136 = {
        let a1111 = f18(v0)?;
        let a1112 = 0u64;
        if nat_eq(a1111, a1112) {
            0u64
        } else {
            f9({
                let a1133 = {
                    let a1131 = {
                        let a1113 = 1u64;
                        let a1114 = 1u64;
                        nat_add(a1113, a1114)?
                    };
                    let a1132 = {
                        let a1129 = {
                            let a1127 = {
                                let a1115 = 1u64;
                                let a1116 = 1u64;
                                nat_add(a1115, a1116)?
                            };
                            let a1128 = {
                                let a1125 = {
                                    let a1123 = {
                                        let a1117 = 1u64;
                                        let a1118 = 1u64;
                                        nat_add(a1117, a1118)?
                                    };
                                    let a1124 = {
                                        let a1121 = {
                                            let a1119 = 1u64;
                                            let a1120 = 1u64;
                                            nat_add(a1119, a1120)?
                                        };
                                        let a1122 = 1u64;
                                        nat_mul(a1121, a1122)?
                                    };
                                    nat_mul(a1123, a1124)?
                                };
                                let a1126 = 1u64;
                                nat_add(a1125, a1126)?
                            };
                            nat_mul(a1127, a1128)?
                        };
                        let a1130 = 1u64;
                        nat_add(a1129, a1130)?
                    };
                    nat_mul(a1131, a1132)?
                };
                let a1134 = 1u64;
                nat_add(a1133, a1134)?
            })?
        }
    };
    nat_add(a1135, a1136)
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h1137, _) = f30(Fn1::F31, {
        let a1156 = {
            let a1138 = 1u64;
            let a1139 = 1u64;
            nat_add(a1138, a1139)?
        };
        let a1157 = {
            let a1154 = {
                let a1140 = 1u64;
                let a1141 = 1u64;
                nat_add(a1140, a1141)?
            };
            let a1155 = {
                let a1152 = {
                    let a1142 = 1u64;
                    let a1143 = 1u64;
                    nat_add(a1142, a1143)?
                };
                let a1153 = {
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
                nat_mul(a1152, a1153)?
            };
            nat_mul(a1154, a1155)?
        };
        nat_mul(a1156, a1157)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h1137)
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1196 = v1;
    let a1197 = {
        let a1194 = {
            let a1192 = {
                let a1158 = 1u64;
                let a1159 = 1u64;
                nat_add(a1158, a1159)?
            };
            let a1193 = {
                let a1190 = {
                    let a1160 = 1u64;
                    let a1161 = 1u64;
                    nat_add(a1160, a1161)?
                };
                let a1191 = {
                    let a1188 = {
                        let a1162 = 1u64;
                        let a1163 = 1u64;
                        nat_add(a1162, a1163)?
                    };
                    let a1189 = {
                        let a1186 = {
                            let a1164 = 1u64;
                            let a1165 = 1u64;
                            nat_add(a1164, a1165)?
                        };
                        let a1187 = {
                            let a1184 = {
                                let a1166 = 1u64;
                                let a1167 = 1u64;
                                nat_add(a1166, a1167)?
                            };
                            let a1185 = {
                                let a1182 = {
                                    let a1168 = 1u64;
                                    let a1169 = 1u64;
                                    nat_add(a1168, a1169)?
                                };
                                let a1183 = {
                                    let a1180 = {
                                        let a1170 = 1u64;
                                        let a1171 = 1u64;
                                        nat_add(a1170, a1171)?
                                    };
                                    let a1181 = {
                                        let a1178 = {
                                            let a1172 = 1u64;
                                            let a1173 = 1u64;
                                            nat_add(a1172, a1173)?
                                        };
                                        let a1179 = {
                                            let a1176 = {
                                                let a1174 = 1u64;
                                                let a1175 = 1u64;
                                                nat_add(a1174, a1175)?
                                            };
                                            let a1177 = 1u64;
                                            nat_mul(a1176, a1177)?
                                        };
                                        nat_mul(a1178, a1179)?
                                    };
                                    nat_mul(a1180, a1181)?
                                };
                                nat_mul(a1182, a1183)?
                            };
                            nat_mul(a1184, a1185)?
                        };
                        nat_mul(a1186, a1187)?
                    };
                    nat_mul(a1188, a1189)?
                };
                nat_mul(a1190, a1191)?
            };
            nat_mul(a1192, a1193)?
        };
        let a1195 = 1u64;
        nat_add(a1194, a1195)?
    };
    if nat_lt(a1196, a1197) {
        f32(v0, {
            let a1234 = {
                let a1232 = {
                    let a1198 = 1u64;
                    let a1199 = 1u64;
                    nat_add(a1198, a1199)?
                };
                let a1233 = {
                    let a1230 = {
                        let a1200 = 1u64;
                        let a1201 = 1u64;
                        nat_add(a1200, a1201)?
                    };
                    let a1231 = {
                        let a1228 = {
                            let a1202 = 1u64;
                            let a1203 = 1u64;
                            nat_add(a1202, a1203)?
                        };
                        let a1229 = {
                            let a1226 = {
                                let a1204 = 1u64;
                                let a1205 = 1u64;
                                nat_add(a1204, a1205)?
                            };
                            let a1227 = {
                                let a1224 = {
                                    let a1206 = 1u64;
                                    let a1207 = 1u64;
                                    nat_add(a1206, a1207)?
                                };
                                let a1225 = {
                                    let a1222 = {
                                        let a1208 = 1u64;
                                        let a1209 = 1u64;
                                        nat_add(a1208, a1209)?
                                    };
                                    let a1223 = {
                                        let a1220 = {
                                            let a1210 = 1u64;
                                            let a1211 = 1u64;
                                            nat_add(a1210, a1211)?
                                        };
                                        let a1221 = {
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
                                        nat_mul(a1220, a1221)?
                                    };
                                    nat_mul(a1222, a1223)?
                                };
                                nat_mul(a1224, a1225)?
                            };
                            nat_mul(a1226, a1227)?
                        };
                        nat_mul(a1228, a1229)?
                    };
                    nat_mul(a1230, a1231)?
                };
                nat_mul(a1232, a1233)?
            };
            let a1235 = 1u64;
            nat_add(a1234, a1235)?
        }, f24(v2, {
            let a1274 = {
                let a1272 = {
                    let a1270 = {
                        let a1236 = 1u64;
                        let a1237 = 1u64;
                        nat_add(a1236, a1237)?
                    };
                    let a1271 = {
                        let a1268 = {
                            let a1238 = 1u64;
                            let a1239 = 1u64;
                            nat_add(a1238, a1239)?
                        };
                        let a1269 = {
                            let a1266 = {
                                let a1240 = 1u64;
                                let a1241 = 1u64;
                                nat_add(a1240, a1241)?
                            };
                            let a1267 = {
                                let a1264 = {
                                    let a1242 = 1u64;
                                    let a1243 = 1u64;
                                    nat_add(a1242, a1243)?
                                };
                                let a1265 = {
                                    let a1262 = {
                                        let a1244 = 1u64;
                                        let a1245 = 1u64;
                                        nat_add(a1244, a1245)?
                                    };
                                    let a1263 = {
                                        let a1260 = {
                                            let a1246 = 1u64;
                                            let a1247 = 1u64;
                                            nat_add(a1246, a1247)?
                                        };
                                        let a1261 = {
                                            let a1258 = {
                                                let a1248 = 1u64;
                                                let a1249 = 1u64;
                                                nat_add(a1248, a1249)?
                                            };
                                            let a1259 = {
                                                let a1256 = {
                                                    let a1250 = 1u64;
                                                    let a1251 = 1u64;
                                                    nat_add(a1250, a1251)?
                                                };
                                                let a1257 = {
                                                    let a1254 = {
                                                        let a1252 = 1u64;
                                                        let a1253 = 1u64;
                                                        nat_add(a1252, a1253)?
                                                    };
                                                    let a1255 = 1u64;
                                                    nat_mul(a1254, a1255)?
                                                };
                                                nat_mul(a1256, a1257)?
                                            };
                                            nat_mul(a1258, a1259)?
                                        };
                                        nat_mul(a1260, a1261)?
                                    };
                                    nat_mul(a1262, a1263)?
                                };
                                nat_mul(a1264, a1265)?
                            };
                            nat_mul(a1266, a1267)?
                        };
                        nat_mul(a1268, a1269)?
                    };
                    nat_mul(a1270, a1271)?
                };
                let a1273 = 1u64;
                nat_add(a1272, a1273)?
            };
            let a1275 = v1;
            nat_sub(a1274, a1275)
        })?)
    } else {
        f32(v0, v1, v2)
    }
}

pub fn f30(v0: Fn1, p1280: u64, p1281: Adt0) -> R<(Adt0, bool)> {
    let m1279 = (p1280, p1281);
    {
        let mut m1279 = m1279;
        loop {
            let step = {
                let (v1, v2) = m1279;
                let m1276 = v1;
                if m1276 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1276 - 1;
                    let m1278 = {
                        let c1277: Fn1 = v0.clone();
                        c1277.apply(v2.clone())?
                    };
                    match m1278 {
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
                Ok(next) => { m1279 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Adt0) -> R<Option<Adt0>> {
    f33(v0)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1294 = v2;
        let a1295 = {
            let a1292 = {
                let a1282 = 1u64;
                let a1283 = 1u64;
                nat_add(a1282, a1283)?
            };
            let a1293 = {
                let a1290 = {
                    let a1284 = 1u64;
                    let a1285 = 1u64;
                    nat_add(a1284, a1285)?
                };
                let a1291 = {
                    let a1288 = {
                        let a1286 = 1u64;
                        let a1287 = 1u64;
                        nat_add(a1286, a1287)?
                    };
                    let a1289 = 1u64;
                    nat_mul(a1288, a1289)?
                };
                nat_mul(a1290, a1291)?
            };
            nat_mul(a1292, a1293)?
        };
        let a1296 = 0u64;
        nat_rem(a1294, a1295, a1296)
    };
    let v4: u64 = {
        let a1309 = v2;
        let a1310 = {
            let a1307 = {
                let a1297 = 1u64;
                let a1298 = 1u64;
                nat_add(a1297, a1298)?
            };
            let a1308 = {
                let a1305 = {
                    let a1299 = 1u64;
                    let a1300 = 1u64;
                    nat_add(a1299, a1300)?
                };
                let a1306 = {
                    let a1303 = {
                        let a1301 = 1u64;
                        let a1302 = 1u64;
                        nat_add(a1301, a1302)?
                    };
                    let a1304 = 1u64;
                    nat_mul(a1303, a1304)?
                };
                nat_mul(a1305, a1306)?
            };
            nat_mul(a1307, a1308)?
        };
        let a1311 = 0u64;
        nat_quot(a1309, a1310, a1311)
    };
    let v5: bool = {
        let a1320 = {
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
        let a1321 = v3;
        if nat_lt(a1320, a1321) {
            true
        } else {
            let a1330 = v3;
            let a1331 = {
                let a1328 = {
                    let a1322 = 1u64;
                    let a1323 = 1u64;
                    nat_add(a1322, a1323)?
                };
                let a1329 = {
                    let a1326 = {
                        let a1324 = 1u64;
                        let a1325 = 1u64;
                        nat_add(a1324, a1325)?
                    };
                    let a1327 = 1u64;
                    nat_mul(a1326, a1327)?
                };
                nat_mul(a1328, a1329)?
            };
            if nat_eq(a1330, a1331) {
                let a1339 = {
                    let a1336 = v4;
                    let a1337 = {
                        let a1334 = {
                            let a1332 = 1u64;
                            let a1333 = 1u64;
                            nat_add(a1332, a1333)?
                        };
                        let a1335 = 1u64;
                        nat_mul(a1334, a1335)?
                    };
                    let a1338 = 0u64;
                    nat_rem(a1336, a1337, a1338)
                };
                let a1340 = 1u64;
                nat_eq(a1339, a1340)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1341 = v4;
        let a1342 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1341, a1342)?
    };
    let v7: bool = {
        let a1371 = {
            let a1369 = {
                let a1345 = {
                    let a1343 = 1u64;
                    let a1344 = 1u64;
                    nat_add(a1343, a1344)?
                };
                let a1346 = 1u64;
                nat_mul(a1345, a1346)?
            };
            let a1370 = f9({
                let a1367 = {
                    let a1365 = {
                        let a1347 = 1u64;
                        let a1348 = 1u64;
                        nat_add(a1347, a1348)?
                    };
                    let a1366 = {
                        let a1363 = {
                            let a1361 = {
                                let a1349 = 1u64;
                                let a1350 = 1u64;
                                nat_add(a1349, a1350)?
                            };
                            let a1362 = {
                                let a1359 = {
                                    let a1357 = {
                                        let a1351 = 1u64;
                                        let a1352 = 1u64;
                                        nat_add(a1351, a1352)?
                                    };
                                    let a1358 = {
                                        let a1355 = {
                                            let a1353 = 1u64;
                                            let a1354 = 1u64;
                                            nat_add(a1353, a1354)?
                                        };
                                        let a1356 = 1u64;
                                        nat_mul(a1355, a1356)?
                                    };
                                    nat_mul(a1357, a1358)?
                                };
                                let a1360 = 1u64;
                                nat_add(a1359, a1360)?
                            };
                            nat_mul(a1361, a1362)?
                        };
                        let a1364 = 1u64;
                        nat_add(a1363, a1364)?
                    };
                    nat_mul(a1365, a1366)?
                };
                let a1368 = 1u64;
                nat_add(a1367, a1368)?
            })?;
            nat_mul(a1369, a1370)?
        };
        let a1372 = v6;
        nat_le(a1371, a1372)
    };
    let v8: u64 = if v7 {
        let a1377 = v6;
        let a1378 = {
            let a1375 = {
                let a1373 = 1u64;
                let a1374 = 1u64;
                nat_add(a1373, a1374)?
            };
            let a1376 = 1u64;
            nat_mul(a1375, a1376)?
        };
        let a1379 = 0u64;
        nat_quot(a1377, a1378, a1379)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1380 = v1;
        let a1381 = 1u64;
        nat_add(a1380, a1381)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1420 = v9;
        let a1421 = {
            let a1418 = {
                let a1416 = {
                    let a1382 = 1u64;
                    let a1383 = 1u64;
                    nat_add(a1382, a1383)?
                };
                let a1417 = {
                    let a1414 = {
                        let a1384 = 1u64;
                        let a1385 = 1u64;
                        nat_add(a1384, a1385)?
                    };
                    let a1415 = {
                        let a1412 = {
                            let a1386 = 1u64;
                            let a1387 = 1u64;
                            nat_add(a1386, a1387)?
                        };
                        let a1413 = {
                            let a1410 = {
                                let a1388 = 1u64;
                                let a1389 = 1u64;
                                nat_add(a1388, a1389)?
                            };
                            let a1411 = {
                                let a1408 = {
                                    let a1390 = 1u64;
                                    let a1391 = 1u64;
                                    nat_add(a1390, a1391)?
                                };
                                let a1409 = {
                                    let a1406 = {
                                        let a1392 = 1u64;
                                        let a1393 = 1u64;
                                        nat_add(a1392, a1393)?
                                    };
                                    let a1407 = {
                                        let a1404 = {
                                            let a1394 = 1u64;
                                            let a1395 = 1u64;
                                            nat_add(a1394, a1395)?
                                        };
                                        let a1405 = {
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
                    nat_mul(a1414, a1415)?
                };
                nat_mul(a1416, a1417)?
            };
            let a1419 = 1u64;
            nat_add(a1418, a1419)?
        };
        if if nat_eq(a1420, a1421) {
            let a1444 = v8;
            let a1445 = f9({
                let a1442 = {
                    let a1440 = {
                        let a1422 = 1u64;
                        let a1423 = 1u64;
                        nat_add(a1422, a1423)?
                    };
                    let a1441 = {
                        let a1438 = {
                            let a1436 = {
                                let a1424 = 1u64;
                                let a1425 = 1u64;
                                nat_add(a1424, a1425)?
                            };
                            let a1437 = {
                                let a1434 = {
                                    let a1432 = {
                                        let a1426 = 1u64;
                                        let a1427 = 1u64;
                                        nat_add(a1426, a1427)?
                                    };
                                    let a1433 = {
                                        let a1430 = {
                                            let a1428 = 1u64;
                                            let a1429 = 1u64;
                                            nat_add(a1428, a1429)?
                                        };
                                        let a1431 = 1u64;
                                        nat_mul(a1430, a1431)?
                                    };
                                    nat_mul(a1432, a1433)?
                                };
                                let a1435 = 1u64;
                                nat_add(a1434, a1435)?
                            };
                            nat_mul(a1436, a1437)?
                        };
                        let a1439 = 1u64;
                        nat_add(a1438, a1439)?
                    };
                    nat_mul(a1440, a1441)?
                };
                let a1443 = 1u64;
                nat_add(a1442, a1443)?
            })?;
            nat_lt(a1444, a1445)
        } else {
            false
        } {
            0u64
        } else {
            let a1482 = v9;
            let a1483 = {
                let a1480 = {
                    let a1446 = 1u64;
                    let a1447 = 1u64;
                    nat_add(a1446, a1447)?
                };
                let a1481 = {
                    let a1478 = {
                        let a1448 = 1u64;
                        let a1449 = 1u64;
                        nat_add(a1448, a1449)?
                    };
                    let a1479 = {
                        let a1476 = {
                            let a1450 = 1u64;
                            let a1451 = 1u64;
                            nat_add(a1450, a1451)?
                        };
                        let a1477 = {
                            let a1474 = {
                                let a1452 = 1u64;
                                let a1453 = 1u64;
                                nat_add(a1452, a1453)?
                            };
                            let a1475 = {
                                let a1472 = {
                                    let a1454 = 1u64;
                                    let a1455 = 1u64;
                                    nat_add(a1454, a1455)?
                                };
                                let a1473 = {
                                    let a1470 = {
                                        let a1456 = 1u64;
                                        let a1457 = 1u64;
                                        nat_add(a1456, a1457)?
                                    };
                                    let a1471 = {
                                        let a1468 = {
                                            let a1458 = 1u64;
                                            let a1459 = 1u64;
                                            nat_add(a1458, a1459)?
                                        };
                                        let a1469 = {
                                            let a1466 = {
                                                let a1460 = 1u64;
                                                let a1461 = 1u64;
                                                nat_add(a1460, a1461)?
                                            };
                                            let a1467 = {
                                                let a1464 = {
                                                    let a1462 = 1u64;
                                                    let a1463 = 1u64;
                                                    nat_add(a1462, a1463)?
                                                };
                                                let a1465 = 1u64;
                                                nat_mul(a1464, a1465)?
                                            };
                                            nat_mul(a1466, a1467)?
                                        };
                                        nat_mul(a1468, a1469)?
                                    };
                                    nat_mul(a1470, a1471)?
                                };
                                nat_mul(a1472, a1473)?
                            };
                            nat_mul(a1474, a1475)?
                        };
                        nat_mul(a1476, a1477)?
                    };
                    nat_mul(a1478, a1479)?
                };
                nat_mul(a1480, a1481)?
            };
            nat_sub(a1482, a1483)
        }
    };
    let a1484 = v8;
    let a1485 = 0u64;
    if nat_eq(a1484, a1485) {
        f12(v0)
    } else {
        let a1538 = {
            let a1536 = {
                let a1534 = {
                    let a1486 = 1u64;
                    let a1487 = 1u64;
                    nat_add(a1486, a1487)?
                };
                let a1535 = {
                    let a1532 = {
                        let a1530 = {
                            let a1488 = 1u64;
                            let a1489 = 1u64;
                            nat_add(a1488, a1489)?
                        };
                        let a1531 = {
                            let a1528 = {
                                let a1526 = {
                                    let a1490 = 1u64;
                                    let a1491 = 1u64;
                                    nat_add(a1490, a1491)?
                                };
                                let a1527 = {
                                    let a1524 = {
                                        let a1522 = {
                                            let a1492 = 1u64;
                                            let a1493 = 1u64;
                                            nat_add(a1492, a1493)?
                                        };
                                        let a1523 = {
                                            let a1520 = {
                                                let a1518 = {
                                                    let a1494 = 1u64;
                                                    let a1495 = 1u64;
                                                    nat_add(a1494, a1495)?
                                                };
                                                let a1519 = {
                                                    let a1516 = {
                                                        let a1514 = {
                                                            let a1496 = 1u64;
                                                            let a1497 = 1u64;
                                                            nat_add(a1496, a1497)?
                                                        };
                                                        let a1515 = {
                                                            let a1512 = {
                                                                let a1510 = {
                                                                    let a1498 = 1u64;
                                                                    let a1499 = 1u64;
                                                                    nat_add(a1498, a1499)?
                                                                };
                                                                let a1511 = {
                                                                    let a1508 = {
                                                                        let a1506 = {
                                                                            let a1500 = 1u64;
                                                                            let a1501 = 1u64;
                                                                            nat_add(a1500, a1501)?
                                                                        };
                                                                        let a1507 = {
                                                                            let a1504 = {
                                                                                let a1502 = 1u64;
                                                                                let a1503 = 1u64;
                                                                                nat_add(a1502, a1503)?
                                                                            };
                                                                            let a1505 = 1u64;
                                                                            nat_mul(a1504, a1505)?
                                                                        };
                                                                        nat_mul(a1506, a1507)?
                                                                    };
                                                                    let a1509 = 1u64;
                                                                    nat_add(a1508, a1509)?
                                                                };
                                                                nat_mul(a1510, a1511)?
                                                            };
                                                            let a1513 = 1u64;
                                                            nat_add(a1512, a1513)?
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
        let a1539 = v9;
        if nat_le(a1538, a1539) {
            f10(v0)
        } else {
            let a1591 = {
                let a1564 = f12(v0)?;
                let a1565 = {
                    let a1562 = v10;
                    let a1563 = f9({
                        let a1560 = {
                            let a1558 = {
                                let a1540 = 1u64;
                                let a1541 = 1u64;
                                nat_add(a1540, a1541)?
                            };
                            let a1559 = {
                                let a1556 = {
                                    let a1554 = {
                                        let a1542 = 1u64;
                                        let a1543 = 1u64;
                                        nat_add(a1542, a1543)?
                                    };
                                    let a1555 = {
                                        let a1552 = {
                                            let a1550 = {
                                                let a1544 = 1u64;
                                                let a1545 = 1u64;
                                                nat_add(a1544, a1545)?
                                            };
                                            let a1551 = {
                                                let a1548 = {
                                                    let a1546 = 1u64;
                                                    let a1547 = 1u64;
                                                    nat_add(a1546, a1547)?
                                                };
                                                let a1549 = 1u64;
                                                nat_mul(a1548, a1549)?
                                            };
                                            nat_mul(a1550, a1551)?
                                        };
                                        let a1553 = 1u64;
                                        nat_add(a1552, a1553)?
                                    };
                                    nat_mul(a1554, a1555)?
                                };
                                let a1557 = 1u64;
                                nat_add(a1556, a1557)?
                            };
                            nat_mul(a1558, a1559)?
                        };
                        let a1561 = 1u64;
                        nat_add(a1560, a1561)?
                    })?;
                    nat_mul(a1562, a1563)?
                };
                nat_add(a1564, a1565)?
            };
            let a1592 = {
                let a1588 = v8;
                let a1589 = f9({
                    let a1586 = {
                        let a1584 = {
                            let a1566 = 1u64;
                            let a1567 = 1u64;
                            nat_add(a1566, a1567)?
                        };
                        let a1585 = {
                            let a1582 = {
                                let a1580 = {
                                    let a1568 = 1u64;
                                    let a1569 = 1u64;
                                    nat_add(a1568, a1569)?
                                };
                                let a1581 = {
                                    let a1578 = {
                                        let a1576 = {
                                            let a1570 = 1u64;
                                            let a1571 = 1u64;
                                            nat_add(a1570, a1571)?
                                        };
                                        let a1577 = {
                                            let a1574 = {
                                                let a1572 = 1u64;
                                                let a1573 = 1u64;
                                                nat_add(a1572, a1573)?
                                            };
                                            let a1575 = 1u64;
                                            nat_mul(a1574, a1575)?
                                        };
                                        nat_mul(a1576, a1577)?
                                    };
                                    let a1579 = 1u64;
                                    nat_add(a1578, a1579)?
                                };
                                nat_mul(a1580, a1581)?
                            };
                            let a1583 = 1u64;
                            nat_add(a1582, a1583)?
                        };
                        nat_mul(a1584, a1585)?
                    };
                    let a1587 = 1u64;
                    nat_add(a1586, a1587)?
                })?;
                let a1590 = 0u64;
                nat_rem(a1588, a1589, a1590)
            };
            nat_add(a1591, a1592)
        }
    }
}

pub fn f33(v0: Adt0) -> R<Option<Adt0>> {
    let a1594 = 0u64;
    let a1595 = match v0.clone() {
        Adt0::C0(_, h1593, _, _) => {
            h1593
        }
    };
    if if if nat_lt(a1594, a1595) {
        let a1598 = match v0.clone() {
            Adt0::C0(_, h1596, _, _) => {
                h1596
            }
        };
        let a1599 = match v0.clone() {
            Adt0::C0(_, _, _, h1597) => {
                h1597
            }
        };
        nat_lt(a1598, a1599)
    } else {
        false
    } {
        let a1602 = match v0.clone() {
            Adt0::C0(_, _, h1600, _) => {
                h1600
            }
        };
        let a1603 = match v0.clone() {
            Adt0::C0(h1601, _, _, _) => {
                h1601
            }
        };
        nat_lt(a1602, a1603)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a1605 = match v0.clone() {
                Adt0::C0(h1604, _, _, _) => {
                    h1604
                }
            };
            let a1606 = 1u64;
            nat_sub(a1605, a1606)
        }, {
            let a1609 = match v0.clone() {
                Adt0::C0(_, h1607, _, _) => {
                    h1607
                }
            };
            let a1610 = match v0.clone() {
                Adt0::C0(_, h1608, _, _) => {
                    h1608
                }
            };
            nat_add(a1609, a1610)?
        }, match v0.clone() {
            Adt0::C0(_, _, h1611, _) => {
                h1611
            }
        }, match v0 {
            Adt0::C0(_, _, _, h1612) => {
                h1612
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}
