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
}

#[derive(Clone)]
pub enum Adt1 {
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
    F35,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt1) -> R<Option<Adt1>> {
        match self {
            Fn1::F35 => f35(p0),
        }
    }
}

pub fn f0(v0: u64, v1: bool, v2: u64) -> R<Adt0> {
    let v3: u64 = f1(v0)?;
    if f2(v0)? {
        Ok(Adt0::C0(2143289344u64))
    } else {
        let a1 = v3;
        let a2 = 964689920u64;
        if nat_le(a1, a2) {
            Ok(Adt0::C0(v0))
        } else {
            let a3 = 1092616192u64;
            let a4 = v3;
            if nat_le(a3, a4) {
                Ok(Adt0::C0(f3(v0, 1065353216u64)?))
            } else {
                let a5 = v3;
                let a6 = 1056964608u64;
                if nat_le(a5, a6) {
                    Ok(Adt0::C0(f3(v0, f4(v3)?)?))
                } else {
                    if v1 {
                        Ok(Adt0::C0(f3(v0, f5(v2)?)?))
                    } else {
                        Ok(Adt0::C1(f6(f7(v3, v3)?)?))
                    }
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> R<u64> {
    let a31 = v0;
    let a32 = f8({
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
    })?;
    let a33 = 0u64;
    Ok(nat_rem(a31, a32, a33))
}

pub fn f2(v0: u64) -> R<bool> {
    let a76 = f9(v0)?;
    let a77 = {
        let a74 = {
            let a72 = {
                let a34 = 1u64;
                let a35 = 1u64;
                nat_add(a34, a35)?
            };
            let a73 = {
                let a70 = {
                    let a68 = {
                        let a36 = 1u64;
                        let a37 = 1u64;
                        nat_add(a36, a37)?
                    };
                    let a69 = {
                        let a66 = {
                            let a64 = {
                                let a38 = 1u64;
                                let a39 = 1u64;
                                nat_add(a38, a39)?
                            };
                            let a65 = {
                                let a62 = {
                                    let a60 = {
                                        let a40 = 1u64;
                                        let a41 = 1u64;
                                        nat_add(a40, a41)?
                                    };
                                    let a61 = {
                                        let a58 = {
                                            let a56 = {
                                                let a42 = 1u64;
                                                let a43 = 1u64;
                                                nat_add(a42, a43)?
                                            };
                                            let a57 = {
                                                let a54 = {
                                                    let a52 = {
                                                        let a44 = 1u64;
                                                        let a45 = 1u64;
                                                        nat_add(a44, a45)?
                                                    };
                                                    let a53 = {
                                                        let a50 = {
                                                            let a48 = {
                                                                let a46 = 1u64;
                                                                let a47 = 1u64;
                                                                nat_add(a46, a47)?
                                                            };
                                                            let a49 = 1u64;
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
            };
            nat_mul(a72, a73)?
        };
        let a75 = 1u64;
        nat_add(a74, a75)?
    };
    if nat_eq(a76, a77) {
        let a78 = 0u64;
        let a79 = f10(v0)?;
        Ok(nat_lt(a78, a79))
    } else {
        Ok(false)
    }
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    let a80 = f11(f12(v0)?)?;
    let a81 = v1;
    nat_add(a80, a81)
}

pub fn f4(v0: u64) -> R<u64> {
    let v1: u64 = f13(v0, v0)?;
    f7(v0, f13(f13(v0, v1)?, f14(3198855851u64, v1, f14(1040746633u64, v1, f14(3176992209u64, v1, f14(1018374052u64, v1, f14(3155244827u64, v1, f14(996895208u64, v1, 3133067698u64)?)?)?)?)?)?)?)
}

pub fn f5(v0: u64) -> R<u64> {
    f7(1065353216u64, f6(f15(f7(v0, v0)?, f7(1065353216u64, v0)?)?)?)
}

pub fn f6(v0: u64) -> R<u64> {
    let a128 = {
        let a106 = v0;
        let a107 = f8({
            let a104 = {
                let a102 = {
                    let a82 = 1u64;
                    let a83 = 1u64;
                    nat_add(a82, a83)?
                };
                let a103 = {
                    let a100 = {
                        let a98 = {
                            let a84 = 1u64;
                            let a85 = 1u64;
                            nat_add(a84, a85)?
                        };
                        let a99 = {
                            let a96 = {
                                let a94 = {
                                    let a86 = 1u64;
                                    let a87 = 1u64;
                                    nat_add(a86, a87)?
                                };
                                let a95 = {
                                    let a92 = {
                                        let a90 = {
                                            let a88 = 1u64;
                                            let a89 = 1u64;
                                            nat_add(a88, a89)?
                                        };
                                        let a91 = 1u64;
                                        nat_mul(a90, a91)?
                                    };
                                    let a93 = 1u64;
                                    nat_add(a92, a93)?
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
            };
            let a105 = 1u64;
            nat_add(a104, a105)?
        })?;
        nat_add(a106, a107)?
    };
    let a129 = f8({
        let a126 = {
            let a108 = 1u64;
            let a109 = 1u64;
            nat_add(a108, a109)?
        };
        let a127 = {
            let a124 = {
                let a110 = 1u64;
                let a111 = 1u64;
                nat_add(a110, a111)?
            };
            let a125 = {
                let a122 = {
                    let a112 = 1u64;
                    let a113 = 1u64;
                    nat_add(a112, a113)?
                };
                let a123 = {
                    let a120 = {
                        let a114 = 1u64;
                        let a115 = 1u64;
                        nat_add(a114, a115)?
                    };
                    let a121 = {
                        let a118 = {
                            let a116 = 1u64;
                            let a117 = 1u64;
                            nat_add(a116, a117)?
                        };
                        let a119 = 1u64;
                        nat_mul(a118, a119)?
                    };
                    nat_mul(a120, a121)?
                };
                nat_mul(a122, a123)?
            };
            nat_mul(a124, a125)?
        };
        nat_mul(a126, a127)?
    })?;
    let a130 = 0u64;
    Ok(nat_rem(a128, a129, a130))
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    if if f2(v0)? {
        true
    } else {
        f2(v1)?
    } {
        let a217 = {
            let a195 = {
                let a171 = {
                    let a169 = {
                        let a131 = 1u64;
                        let a132 = 1u64;
                        nat_add(a131, a132)?
                    };
                    let a170 = {
                        let a167 = {
                            let a165 = {
                                let a133 = 1u64;
                                let a134 = 1u64;
                                nat_add(a133, a134)?
                            };
                            let a166 = {
                                let a163 = {
                                    let a161 = {
                                        let a135 = 1u64;
                                        let a136 = 1u64;
                                        nat_add(a135, a136)?
                                    };
                                    let a162 = {
                                        let a159 = {
                                            let a157 = {
                                                let a137 = 1u64;
                                                let a138 = 1u64;
                                                nat_add(a137, a138)?
                                            };
                                            let a158 = {
                                                let a155 = {
                                                    let a153 = {
                                                        let a139 = 1u64;
                                                        let a140 = 1u64;
                                                        nat_add(a139, a140)?
                                                    };
                                                    let a154 = {
                                                        let a151 = {
                                                            let a149 = {
                                                                let a141 = 1u64;
                                                                let a142 = 1u64;
                                                                nat_add(a141, a142)?
                                                            };
                                                            let a150 = {
                                                                let a147 = {
                                                                    let a145 = {
                                                                        let a143 = 1u64;
                                                                        let a144 = 1u64;
                                                                        nat_add(a143, a144)?
                                                                    };
                                                                    let a146 = 1u64;
                                                                    nat_mul(a145, a146)?
                                                                };
                                                                let a148 = 1u64;
                                                                nat_add(a147, a148)?
                                                            };
                                                            nat_mul(a149, a150)?
                                                        };
                                                        let a152 = 1u64;
                                                        nat_add(a151, a152)?
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
            let a196 = f8({
                let a193 = {
                    let a191 = {
                        let a173 = 1u64;
                        let a174 = 1u64;
                        nat_add(a173, a174)?
                    };
                    let a192 = {
                        let a189 = {
                            let a187 = {
                                let a175 = 1u64;
                                let a176 = 1u64;
                                nat_add(a175, a176)?
                            };
                            let a188 = {
                                let a185 = {
                                    let a183 = {
                                        let a177 = 1u64;
                                        let a178 = 1u64;
                                        nat_add(a177, a178)?
                                    };
                                    let a184 = {
                                        let a181 = {
                                            let a179 = 1u64;
                                            let a180 = 1u64;
                                            nat_add(a179, a180)?
                                        };
                                        let a182 = 1u64;
                                        nat_mul(a181, a182)?
                                    };
                                    nat_mul(a183, a184)?
                                };
                                let a186 = 1u64;
                                nat_add(a185, a186)?
                            };
                            nat_mul(a187, a188)?
                        };
                        let a190 = 1u64;
                        nat_add(a189, a190)?
                    };
                    nat_mul(a191, a192)?
                };
                let a194 = 1u64;
                nat_add(a193, a194)?
            })?;
            nat_mul(a195, a196)?
        };
        let a218 = f8({
            let a215 = {
                let a197 = 1u64;
                let a198 = 1u64;
                nat_add(a197, a198)?
            };
            let a216 = {
                let a213 = {
                    let a211 = {
                        let a199 = 1u64;
                        let a200 = 1u64;
                        nat_add(a199, a200)?
                    };
                    let a212 = {
                        let a209 = {
                            let a207 = {
                                let a201 = 1u64;
                                let a202 = 1u64;
                                nat_add(a201, a202)?
                            };
                            let a208 = {
                                let a205 = {
                                    let a203 = 1u64;
                                    let a204 = 1u64;
                                    nat_add(a203, a204)?
                                };
                                let a206 = 1u64;
                                nat_mul(a205, a206)?
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
        })?;
        nat_add(a217, a218)
    } else {
        if if f16(v0)? {
            f16(v1)?
        } else {
            false
        } {
            let a219 = f12(v0)?;
            let a220 = f12(v1)?;
            if nat_eq(a219, a220) {
                Ok(v0)
            } else {
                let a307 = {
                    let a285 = {
                        let a261 = {
                            let a259 = {
                                let a221 = 1u64;
                                let a222 = 1u64;
                                nat_add(a221, a222)?
                            };
                            let a260 = {
                                let a257 = {
                                    let a255 = {
                                        let a223 = 1u64;
                                        let a224 = 1u64;
                                        nat_add(a223, a224)?
                                    };
                                    let a256 = {
                                        let a253 = {
                                            let a251 = {
                                                let a225 = 1u64;
                                                let a226 = 1u64;
                                                nat_add(a225, a226)?
                                            };
                                            let a252 = {
                                                let a249 = {
                                                    let a247 = {
                                                        let a227 = 1u64;
                                                        let a228 = 1u64;
                                                        nat_add(a227, a228)?
                                                    };
                                                    let a248 = {
                                                        let a245 = {
                                                            let a243 = {
                                                                let a229 = 1u64;
                                                                let a230 = 1u64;
                                                                nat_add(a229, a230)?
                                                            };
                                                            let a244 = {
                                                                let a241 = {
                                                                    let a239 = {
                                                                        let a231 = 1u64;
                                                                        let a232 = 1u64;
                                                                        nat_add(a231, a232)?
                                                                    };
                                                                    let a240 = {
                                                                        let a237 = {
                                                                            let a235 = {
                                                                                let a233 = 1u64;
                                                                                let a234 = 1u64;
                                                                                nat_add(a233, a234)?
                                                                            };
                                                                            let a236 = 1u64;
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
                                                    nat_mul(a247, a248)?
                                                };
                                                let a250 = 1u64;
                                                nat_add(a249, a250)?
                                            };
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
                    let a286 = f8({
                        let a283 = {
                            let a281 = {
                                let a263 = 1u64;
                                let a264 = 1u64;
                                nat_add(a263, a264)?
                            };
                            let a282 = {
                                let a279 = {
                                    let a277 = {
                                        let a265 = 1u64;
                                        let a266 = 1u64;
                                        nat_add(a265, a266)?
                                    };
                                    let a278 = {
                                        let a275 = {
                                            let a273 = {
                                                let a267 = 1u64;
                                                let a268 = 1u64;
                                                nat_add(a267, a268)?
                                            };
                                            let a274 = {
                                                let a271 = {
                                                    let a269 = 1u64;
                                                    let a270 = 1u64;
                                                    nat_add(a269, a270)?
                                                };
                                                let a272 = 1u64;
                                                nat_mul(a271, a272)?
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
                    })?;
                    nat_mul(a285, a286)?
                };
                let a308 = f8({
                    let a305 = {
                        let a287 = 1u64;
                        let a288 = 1u64;
                        nat_add(a287, a288)?
                    };
                    let a306 = {
                        let a303 = {
                            let a301 = {
                                let a289 = 1u64;
                                let a290 = 1u64;
                                nat_add(a289, a290)?
                            };
                            let a302 = {
                                let a299 = {
                                    let a297 = {
                                        let a291 = 1u64;
                                        let a292 = 1u64;
                                        nat_add(a291, a292)?
                                    };
                                    let a298 = {
                                        let a295 = {
                                            let a293 = 1u64;
                                            let a294 = 1u64;
                                            nat_add(a293, a294)?
                                        };
                                        let a296 = 1u64;
                                        nat_mul(a295, a296)?
                                    };
                                    nat_mul(a297, a298)?
                                };
                                let a300 = 1u64;
                                nat_add(a299, a300)?
                            };
                            nat_mul(a301, a302)?
                        };
                        let a304 = 1u64;
                        nat_add(a303, a304)?
                    };
                    nat_mul(a305, a306)?
                })?;
                nat_add(a307, a308)
            }
        } else {
            if f16(v0)? {
                Ok(v0)
            } else {
                if f16(v1)? {
                    Ok(v1)
                } else {
                    let a309 = f1(v1)?;
                    let a310 = f1(v0)?;
                    if nat_le(a309, a310) {
                        f17(v0, v1)
                    } else {
                        f17(v1, v0)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<u64> {
    f18(Fn0::F19, v0, 1u64)
}

pub fn f9(v0: u64) -> R<u64> {
    let a333 = f1(v0)?;
    let a334 = f8({
        let a331 = {
            let a329 = {
                let a311 = 1u64;
                let a312 = 1u64;
                nat_add(a311, a312)?
            };
            let a330 = {
                let a327 = {
                    let a325 = {
                        let a313 = 1u64;
                        let a314 = 1u64;
                        nat_add(a313, a314)?
                    };
                    let a326 = {
                        let a323 = {
                            let a321 = {
                                let a315 = 1u64;
                                let a316 = 1u64;
                                nat_add(a315, a316)?
                            };
                            let a322 = {
                                let a319 = {
                                    let a317 = 1u64;
                                    let a318 = 1u64;
                                    nat_add(a317, a318)?
                                };
                                let a320 = 1u64;
                                nat_mul(a319, a320)?
                            };
                            nat_mul(a321, a322)?
                        };
                        let a324 = 1u64;
                        nat_add(a323, a324)?
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
    })?;
    let a335 = 0u64;
    Ok(nat_quot(a333, a334, a335))
}

pub fn f10(v0: u64) -> R<u64> {
    let a358 = v0;
    let a359 = f8({
        let a356 = {
            let a354 = {
                let a336 = 1u64;
                let a337 = 1u64;
                nat_add(a336, a337)?
            };
            let a355 = {
                let a352 = {
                    let a350 = {
                        let a338 = 1u64;
                        let a339 = 1u64;
                        nat_add(a338, a339)?
                    };
                    let a351 = {
                        let a348 = {
                            let a346 = {
                                let a340 = 1u64;
                                let a341 = 1u64;
                                nat_add(a340, a341)?
                            };
                            let a347 = {
                                let a344 = {
                                    let a342 = 1u64;
                                    let a343 = 1u64;
                                    nat_add(a342, a343)?
                                };
                                let a345 = 1u64;
                                nat_mul(a344, a345)?
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
            };
            nat_mul(a354, a355)?
        };
        let a357 = 1u64;
        nat_add(a356, a357)?
    })?;
    let a360 = 0u64;
    Ok(nat_rem(a358, a359, a360))
}

pub fn f11(v0: u64) -> R<u64> {
    let a385 = v0;
    let a386 = f8({
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
    nat_mul(a385, a386)
}

pub fn f12(v0: u64) -> R<u64> {
    let a411 = v0;
    let a412 = f8({
        let a409 = {
            let a407 = {
                let a387 = 1u64;
                let a388 = 1u64;
                nat_add(a387, a388)?
            };
            let a408 = {
                let a405 = {
                    let a403 = {
                        let a389 = 1u64;
                        let a390 = 1u64;
                        nat_add(a389, a390)?
                    };
                    let a404 = {
                        let a401 = {
                            let a399 = {
                                let a391 = 1u64;
                                let a392 = 1u64;
                                nat_add(a391, a392)?
                            };
                            let a400 = {
                                let a397 = {
                                    let a395 = {
                                        let a393 = 1u64;
                                        let a394 = 1u64;
                                        nat_add(a393, a394)?
                                    };
                                    let a396 = 1u64;
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
    })?;
    let a413 = 0u64;
    Ok(nat_quot(a411, a412, a413))
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    if if if if f2(v0)? {
        true
    } else {
        f2(v1)?
    } {
        true
    } else {
        if f16(v0)? {
            f20(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f16(v1)? {
            f20(v0)?
        } else {
            false
        }
    } {
        let a500 = {
            let a478 = {
                let a454 = {
                    let a452 = {
                        let a414 = 1u64;
                        let a415 = 1u64;
                        nat_add(a414, a415)?
                    };
                    let a453 = {
                        let a450 = {
                            let a448 = {
                                let a416 = 1u64;
                                let a417 = 1u64;
                                nat_add(a416, a417)?
                            };
                            let a449 = {
                                let a446 = {
                                    let a444 = {
                                        let a418 = 1u64;
                                        let a419 = 1u64;
                                        nat_add(a418, a419)?
                                    };
                                    let a445 = {
                                        let a442 = {
                                            let a440 = {
                                                let a420 = 1u64;
                                                let a421 = 1u64;
                                                nat_add(a420, a421)?
                                            };
                                            let a441 = {
                                                let a438 = {
                                                    let a436 = {
                                                        let a422 = 1u64;
                                                        let a423 = 1u64;
                                                        nat_add(a422, a423)?
                                                    };
                                                    let a437 = {
                                                        let a434 = {
                                                            let a432 = {
                                                                let a424 = 1u64;
                                                                let a425 = 1u64;
                                                                nat_add(a424, a425)?
                                                            };
                                                            let a433 = {
                                                                let a430 = {
                                                                    let a428 = {
                                                                        let a426 = 1u64;
                                                                        let a427 = 1u64;
                                                                        nat_add(a426, a427)?
                                                                    };
                                                                    let a429 = 1u64;
                                                                    nat_mul(a428, a429)?
                                                                };
                                                                let a431 = 1u64;
                                                                nat_add(a430, a431)?
                                                            };
                                                            nat_mul(a432, a433)?
                                                        };
                                                        let a435 = 1u64;
                                                        nat_add(a434, a435)?
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
                            };
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
            let a479 = f8({
                let a476 = {
                    let a474 = {
                        let a456 = 1u64;
                        let a457 = 1u64;
                        nat_add(a456, a457)?
                    };
                    let a475 = {
                        let a472 = {
                            let a470 = {
                                let a458 = 1u64;
                                let a459 = 1u64;
                                nat_add(a458, a459)?
                            };
                            let a471 = {
                                let a468 = {
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
                                let a469 = 1u64;
                                nat_add(a468, a469)?
                            };
                            nat_mul(a470, a471)?
                        };
                        let a473 = 1u64;
                        nat_add(a472, a473)?
                    };
                    nat_mul(a474, a475)?
                };
                let a477 = 1u64;
                nat_add(a476, a477)?
            })?;
            nat_mul(a478, a479)?
        };
        let a501 = f8({
            let a498 = {
                let a480 = 1u64;
                let a481 = 1u64;
                nat_add(a480, a481)?
            };
            let a499 = {
                let a496 = {
                    let a494 = {
                        let a482 = 1u64;
                        let a483 = 1u64;
                        nat_add(a482, a483)?
                    };
                    let a495 = {
                        let a492 = {
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
                        let a493 = 1u64;
                        nat_add(a492, a493)?
                    };
                    nat_mul(a494, a495)?
                };
                let a497 = 1u64;
                nat_add(a496, a497)?
            };
            nat_mul(a498, a499)?
        })?;
        nat_add(a500, a501)
    } else {
        if if f16(v0)? {
            true
        } else {
            f16(v1)?
        } {
            f21(f22(v0, v1)?)
        } else {
            if if f20(v0)? {
                true
            } else {
                f20(v1)?
            } {
                f11(f22(v0, v1)?)
            } else {
                f23(v0, v1)
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f7(v0, f13(v1, v2)?)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    if if if if f2(v0)? {
        true
    } else {
        f2(v1)?
    } {
        true
    } else {
        if f16(v0)? {
            f16(v1)?
        } else {
            false
        }
    } {
        true
    } else {
        if f20(v0)? {
            f20(v1)?
        } else {
            false
        }
    } {
        let a588 = {
            let a566 = {
                let a542 = {
                    let a540 = {
                        let a502 = 1u64;
                        let a503 = 1u64;
                        nat_add(a502, a503)?
                    };
                    let a541 = {
                        let a538 = {
                            let a536 = {
                                let a504 = 1u64;
                                let a505 = 1u64;
                                nat_add(a504, a505)?
                            };
                            let a537 = {
                                let a534 = {
                                    let a532 = {
                                        let a506 = 1u64;
                                        let a507 = 1u64;
                                        nat_add(a506, a507)?
                                    };
                                    let a533 = {
                                        let a530 = {
                                            let a528 = {
                                                let a508 = 1u64;
                                                let a509 = 1u64;
                                                nat_add(a508, a509)?
                                            };
                                            let a529 = {
                                                let a526 = {
                                                    let a524 = {
                                                        let a510 = 1u64;
                                                        let a511 = 1u64;
                                                        nat_add(a510, a511)?
                                                    };
                                                    let a525 = {
                                                        let a522 = {
                                                            let a520 = {
                                                                let a512 = 1u64;
                                                                let a513 = 1u64;
                                                                nat_add(a512, a513)?
                                                            };
                                                            let a521 = {
                                                                let a518 = {
                                                                    let a516 = {
                                                                        let a514 = 1u64;
                                                                        let a515 = 1u64;
                                                                        nat_add(a514, a515)?
                                                                    };
                                                                    let a517 = 1u64;
                                                                    nat_mul(a516, a517)?
                                                                };
                                                                let a519 = 1u64;
                                                                nat_add(a518, a519)?
                                                            };
                                                            nat_mul(a520, a521)?
                                                        };
                                                        let a523 = 1u64;
                                                        nat_add(a522, a523)?
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
                            };
                            nat_mul(a536, a537)?
                        };
                        let a539 = 1u64;
                        nat_add(a538, a539)?
                    };
                    nat_mul(a540, a541)?
                };
                let a543 = 1u64;
                nat_add(a542, a543)?
            };
            let a567 = f8({
                let a564 = {
                    let a562 = {
                        let a544 = 1u64;
                        let a545 = 1u64;
                        nat_add(a544, a545)?
                    };
                    let a563 = {
                        let a560 = {
                            let a558 = {
                                let a546 = 1u64;
                                let a547 = 1u64;
                                nat_add(a546, a547)?
                            };
                            let a559 = {
                                let a556 = {
                                    let a554 = {
                                        let a548 = 1u64;
                                        let a549 = 1u64;
                                        nat_add(a548, a549)?
                                    };
                                    let a555 = {
                                        let a552 = {
                                            let a550 = 1u64;
                                            let a551 = 1u64;
                                            nat_add(a550, a551)?
                                        };
                                        let a553 = 1u64;
                                        nat_mul(a552, a553)?
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
            })?;
            nat_mul(a566, a567)?
        };
        let a589 = f8({
            let a586 = {
                let a568 = 1u64;
                let a569 = 1u64;
                nat_add(a568, a569)?
            };
            let a587 = {
                let a584 = {
                    let a582 = {
                        let a570 = 1u64;
                        let a571 = 1u64;
                        nat_add(a570, a571)?
                    };
                    let a583 = {
                        let a580 = {
                            let a578 = {
                                let a572 = 1u64;
                                let a573 = 1u64;
                                nat_add(a572, a573)?
                            };
                            let a579 = {
                                let a576 = {
                                    let a574 = 1u64;
                                    let a575 = 1u64;
                                    nat_add(a574, a575)?
                                };
                                let a577 = 1u64;
                                nat_mul(a576, a577)?
                            };
                            nat_mul(a578, a579)?
                        };
                        let a581 = 1u64;
                        nat_add(a580, a581)?
                    };
                    nat_mul(a582, a583)?
                };
                let a585 = 1u64;
                nat_add(a584, a585)?
            };
            nat_mul(a586, a587)?
        })?;
        nat_add(a588, a589)
    } else {
        if if f16(v0)? {
            true
        } else {
            f20(v1)?
        } {
            f21(f22(v0, v1)?)
        } else {
            if if f16(v1)? {
                true
            } else {
                f20(v0)?
            } {
                f11(f22(v0, v1)?)
            } else {
                f24(v0, v1)
            }
        }
    }
}

pub fn f16(v0: u64) -> R<bool> {
    let a632 = f9(v0)?;
    let a633 = {
        let a630 = {
            let a628 = {
                let a590 = 1u64;
                let a591 = 1u64;
                nat_add(a590, a591)?
            };
            let a629 = {
                let a626 = {
                    let a624 = {
                        let a592 = 1u64;
                        let a593 = 1u64;
                        nat_add(a592, a593)?
                    };
                    let a625 = {
                        let a622 = {
                            let a620 = {
                                let a594 = 1u64;
                                let a595 = 1u64;
                                nat_add(a594, a595)?
                            };
                            let a621 = {
                                let a618 = {
                                    let a616 = {
                                        let a596 = 1u64;
                                        let a597 = 1u64;
                                        nat_add(a596, a597)?
                                    };
                                    let a617 = {
                                        let a614 = {
                                            let a612 = {
                                                let a598 = 1u64;
                                                let a599 = 1u64;
                                                nat_add(a598, a599)?
                                            };
                                            let a613 = {
                                                let a610 = {
                                                    let a608 = {
                                                        let a600 = 1u64;
                                                        let a601 = 1u64;
                                                        nat_add(a600, a601)?
                                                    };
                                                    let a609 = {
                                                        let a606 = {
                                                            let a604 = {
                                                                let a602 = 1u64;
                                                                let a603 = 1u64;
                                                                nat_add(a602, a603)?
                                                            };
                                                            let a605 = 1u64;
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
    if nat_eq(a632, a633) {
        let a634 = f10(v0)?;
        let a635 = 0u64;
        Ok(nat_eq(a634, a635))
    } else {
        Ok(false)
    }
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f25(v0)?;
    let v3: u64 = {
        let a648 = f26(v0)?;
        let a649 = {
            let a646 = {
                let a636 = 1u64;
                let a637 = 1u64;
                nat_add(a636, a637)?
            };
            let a647 = {
                let a644 = {
                    let a638 = 1u64;
                    let a639 = 1u64;
                    nat_add(a638, a639)?
                };
                let a645 = {
                    let a642 = {
                        let a640 = 1u64;
                        let a641 = 1u64;
                        nat_add(a640, a641)?
                    };
                    let a643 = 1u64;
                    nat_mul(a642, a643)?
                };
                nat_mul(a644, a645)?
            };
            nat_mul(a646, a647)?
        };
        nat_mul(a648, a649)?
    };
    let v4: u64 = f27({
        let a662 = f26(v1)?;
        let a663 = {
            let a660 = {
                let a650 = 1u64;
                let a651 = 1u64;
                nat_add(a650, a651)?
            };
            let a661 = {
                let a658 = {
                    let a652 = 1u64;
                    let a653 = 1u64;
                    nat_add(a652, a653)?
                };
                let a659 = {
                    let a656 = {
                        let a654 = 1u64;
                        let a655 = 1u64;
                        nat_add(a654, a655)?
                    };
                    let a657 = 1u64;
                    nat_mul(a656, a657)?
                };
                nat_mul(a658, a659)?
            };
            nat_mul(a660, a661)?
        };
        nat_mul(a662, a663)?
    }, {
        let a664 = v2;
        let a665 = f25(v1)?;
        nat_sub(a664, a665)
    })?;
    let a666 = f12(v0)?;
    let a667 = f12(v1)?;
    if nat_eq(a666, a667) {
        f28(f12(v0)?, v2, {
            let a668 = v3;
            let a669 = v4;
            nat_add(a668, a669)?
        })
    } else {
        let a670 = v3;
        let a671 = v4;
        if nat_eq(a670, a671) {
            Ok(0u64)
        } else {
            f28(f12(v0)?, v2, {
                let a672 = v3;
                let a673 = v4;
                nat_sub(a672, a673)
            })
        }
    }
}

pub fn f18(v0: Fn0, p677: u64, p678: u64) -> R<u64> {
    let m676 = (p677, p678);
    {
        let mut m676 = m676;
        loop {
            let step = {
                let (v1, v2) = m676;
                let m674 = v1;
                if m674 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m674 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c675: Fn0 = v0.clone();
                        c675.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f19(v0: u64) -> R<u64> {
    let a679 = v0;
    let a680 = v0;
    nat_add(a679, a680)
}

pub fn f20(v0: u64) -> R<bool> {
    let a681 = f1(v0)?;
    let a682 = 0u64;
    Ok(nat_eq(a681, a682))
}

pub fn f21(v0: u64) -> R<u64> {
    let a749 = f11(v0)?;
    let a750 = {
        let a747 = {
            let a723 = {
                let a721 = {
                    let a683 = 1u64;
                    let a684 = 1u64;
                    nat_add(a683, a684)?
                };
                let a722 = {
                    let a719 = {
                        let a717 = {
                            let a685 = 1u64;
                            let a686 = 1u64;
                            nat_add(a685, a686)?
                        };
                        let a718 = {
                            let a715 = {
                                let a713 = {
                                    let a687 = 1u64;
                                    let a688 = 1u64;
                                    nat_add(a687, a688)?
                                };
                                let a714 = {
                                    let a711 = {
                                        let a709 = {
                                            let a689 = 1u64;
                                            let a690 = 1u64;
                                            nat_add(a689, a690)?
                                        };
                                        let a710 = {
                                            let a707 = {
                                                let a705 = {
                                                    let a691 = 1u64;
                                                    let a692 = 1u64;
                                                    nat_add(a691, a692)?
                                                };
                                                let a706 = {
                                                    let a703 = {
                                                        let a701 = {
                                                            let a693 = 1u64;
                                                            let a694 = 1u64;
                                                            nat_add(a693, a694)?
                                                        };
                                                        let a702 = {
                                                            let a699 = {
                                                                let a697 = {
                                                                    let a695 = 1u64;
                                                                    let a696 = 1u64;
                                                                    nat_add(a695, a696)?
                                                                };
                                                                let a698 = 1u64;
                                                                nat_mul(a697, a698)?
                                                            };
                                                            let a700 = 1u64;
                                                            nat_add(a699, a700)?
                                                        };
                                                        nat_mul(a701, a702)?
                                                    };
                                                    let a704 = 1u64;
                                                    nat_add(a703, a704)?
                                                };
                                                nat_mul(a705, a706)?
                                            };
                                            let a708 = 1u64;
                                            nat_add(a707, a708)?
                                        };
                                        nat_mul(a709, a710)?
                                    };
                                    let a712 = 1u64;
                                    nat_add(a711, a712)?
                                };
                                nat_mul(a713, a714)?
                            };
                            let a716 = 1u64;
                            nat_add(a715, a716)?
                        };
                        nat_mul(a717, a718)?
                    };
                    let a720 = 1u64;
                    nat_add(a719, a720)?
                };
                nat_mul(a721, a722)?
            };
            let a724 = 1u64;
            nat_add(a723, a724)?
        };
        let a748 = f8({
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
        })?;
        nat_mul(a747, a748)?
    };
    nat_add(a749, a750)
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let a757 = {
        let a751 = f12(v0)?;
        let a752 = f12(v1)?;
        nat_add(a751, a752)?
    };
    let a758 = {
        let a755 = {
            let a753 = 1u64;
            let a754 = 1u64;
            nat_add(a753, a754)?
        };
        let a756 = 1u64;
        nat_mul(a755, a756)?
    };
    let a759 = 0u64;
    Ok(nat_rem(a757, a758, a759))
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f29(v0)?;
    let v3: Adt1 = f29(v1)?;
    f28(f22(v0, v1)?, {
        let a814 = {
            let a762 = match v2.clone() {
                Adt1::C0(h760, _, _, _) => {
                    h760
                }
            };
            let a763 = match v3.clone() {
                Adt1::C0(h761, _, _, _) => {
                    h761
                }
            };
            nat_add(a762, a763)?
        };
        let a815 = {
            let a812 = {
                let a810 = {
                    let a764 = 1u64;
                    let a765 = 1u64;
                    nat_add(a764, a765)?
                };
                let a811 = {
                    let a808 = {
                        let a806 = {
                            let a766 = 1u64;
                            let a767 = 1u64;
                            nat_add(a766, a767)?
                        };
                        let a807 = {
                            let a804 = {
                                let a802 = {
                                    let a768 = 1u64;
                                    let a769 = 1u64;
                                    nat_add(a768, a769)?
                                };
                                let a803 = {
                                    let a800 = {
                                        let a798 = {
                                            let a770 = 1u64;
                                            let a771 = 1u64;
                                            nat_add(a770, a771)?
                                        };
                                        let a799 = {
                                            let a796 = {
                                                let a794 = {
                                                    let a772 = 1u64;
                                                    let a773 = 1u64;
                                                    nat_add(a772, a773)?
                                                };
                                                let a795 = {
                                                    let a792 = {
                                                        let a790 = {
                                                            let a774 = 1u64;
                                                            let a775 = 1u64;
                                                            nat_add(a774, a775)?
                                                        };
                                                        let a791 = {
                                                            let a788 = {
                                                                let a786 = {
                                                                    let a776 = 1u64;
                                                                    let a777 = 1u64;
                                                                    nat_add(a776, a777)?
                                                                };
                                                                let a787 = {
                                                                    let a784 = {
                                                                        let a778 = 1u64;
                                                                        let a779 = 1u64;
                                                                        nat_add(a778, a779)?
                                                                    };
                                                                    let a785 = {
                                                                        let a782 = {
                                                                            let a780 = 1u64;
                                                                            let a781 = 1u64;
                                                                            nat_add(a780, a781)?
                                                                        };
                                                                        let a783 = 1u64;
                                                                        nat_mul(a782, a783)?
                                                                    };
                                                                    nat_mul(a784, a785)?
                                                                };
                                                                nat_mul(a786, a787)?
                                                            };
                                                            let a789 = 1u64;
                                                            nat_add(a788, a789)?
                                                        };
                                                        nat_mul(a790, a791)?
                                                    };
                                                    let a793 = 1u64;
                                                    nat_add(a792, a793)?
                                                };
                                                nat_mul(a794, a795)?
                                            };
                                            let a797 = 1u64;
                                            nat_add(a796, a797)?
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
                };
                nat_mul(a810, a811)?
            };
            let a813 = 1u64;
            nat_add(a812, a813)?
        };
        nat_sub(a814, a815)
    }, f27({
        let a818 = match v2 {
            Adt1::C0(_, h816, _, _) => {
                h816
            }
        };
        let a819 = match v3 {
            Adt1::C0(_, h817, _, _) => {
                h817
            }
        };
        nat_mul(a818, a819)?
    }, {
        let a836 = {
            let a820 = 1u64;
            let a821 = 1u64;
            nat_add(a820, a821)?
        };
        let a837 = {
            let a834 = {
                let a822 = 1u64;
                let a823 = 1u64;
                nat_add(a822, a823)?
            };
            let a835 = {
                let a832 = {
                    let a830 = {
                        let a824 = 1u64;
                        let a825 = 1u64;
                        nat_add(a824, a825)?
                    };
                    let a831 = {
                        let a828 = {
                            let a826 = 1u64;
                            let a827 = 1u64;
                            nat_add(a826, a827)?
                        };
                        let a829 = 1u64;
                        nat_mul(a828, a829)?
                    };
                    nat_mul(a830, a831)?
                };
                let a833 = 1u64;
                nat_add(a832, a833)?
            };
            nat_mul(a834, a835)?
        };
        nat_mul(a836, a837)?
    })?)
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let v2: Adt1 = f29(v0)?;
    let v3: Adt1 = f29(v1)?;
    let v4: bool = {
        let a840 = match v2.clone() {
            Adt1::C0(_, h838, _, _) => {
                h838
            }
        };
        let a841 = match v3.clone() {
            Adt1::C0(_, h839, _, _) => {
                h839
            }
        };
        nat_lt(a840, a841)
    };
    let v5: u64 = {
        let a885 = match v2.clone() {
            Adt1::C0(_, h842, _, _) => {
                h842
            }
        };
        let a886 = f8(if v4 {
            let a863 = {
                let a861 = {
                    let a843 = 1u64;
                    let a844 = 1u64;
                    nat_add(a843, a844)?
                };
                let a862 = {
                    let a859 = {
                        let a857 = {
                            let a845 = 1u64;
                            let a846 = 1u64;
                            nat_add(a845, a846)?
                        };
                        let a858 = {
                            let a855 = {
                                let a847 = 1u64;
                                let a848 = 1u64;
                                nat_add(a847, a848)?
                            };
                            let a856 = {
                                let a853 = {
                                    let a851 = {
                                        let a849 = 1u64;
                                        let a850 = 1u64;
                                        nat_add(a849, a850)?
                                    };
                                    let a852 = 1u64;
                                    nat_mul(a851, a852)?
                                };
                                let a854 = 1u64;
                                nat_add(a853, a854)?
                            };
                            nat_mul(a855, a856)?
                        };
                        nat_mul(a857, a858)?
                    };
                    let a860 = 1u64;
                    nat_add(a859, a860)?
                };
                nat_mul(a861, a862)?
            };
            let a864 = 1u64;
            nat_add(a863, a864)?
        } else {
            let a883 = {
                let a865 = 1u64;
                let a866 = 1u64;
                nat_add(a865, a866)?
            };
            let a884 = {
                let a881 = {
                    let a879 = {
                        let a867 = 1u64;
                        let a868 = 1u64;
                        nat_add(a867, a868)?
                    };
                    let a880 = {
                        let a877 = {
                            let a869 = 1u64;
                            let a870 = 1u64;
                            nat_add(a869, a870)?
                        };
                        let a878 = {
                            let a875 = {
                                let a873 = {
                                    let a871 = 1u64;
                                    let a872 = 1u64;
                                    nat_add(a871, a872)?
                                };
                                let a874 = 1u64;
                                nat_mul(a873, a874)?
                            };
                            let a876 = 1u64;
                            nat_add(a875, a876)?
                        };
                        nat_mul(a877, a878)?
                    };
                    nat_mul(a879, a880)?
                };
                let a882 = 1u64;
                nat_add(a881, a882)?
            };
            nat_mul(a883, a884)?
        })?;
        nat_mul(a885, a886)?
    };
    let v6: u64 = {
        let a943 = {
            let a941 = {
                let a938 = match v2 {
                    Adt1::C0(h887, _, _, _) => {
                        h887
                    }
                };
                let a939 = {
                    let a936 = {
                        let a934 = {
                            let a888 = 1u64;
                            let a889 = 1u64;
                            nat_add(a888, a889)?
                        };
                        let a935 = {
                            let a932 = {
                                let a930 = {
                                    let a890 = 1u64;
                                    let a891 = 1u64;
                                    nat_add(a890, a891)?
                                };
                                let a931 = {
                                    let a928 = {
                                        let a926 = {
                                            let a892 = 1u64;
                                            let a893 = 1u64;
                                            nat_add(a892, a893)?
                                        };
                                        let a927 = {
                                            let a924 = {
                                                let a922 = {
                                                    let a894 = 1u64;
                                                    let a895 = 1u64;
                                                    nat_add(a894, a895)?
                                                };
                                                let a923 = {
                                                    let a920 = {
                                                        let a918 = {
                                                            let a896 = 1u64;
                                                            let a897 = 1u64;
                                                            nat_add(a896, a897)?
                                                        };
                                                        let a919 = {
                                                            let a916 = {
                                                                let a914 = {
                                                                    let a898 = 1u64;
                                                                    let a899 = 1u64;
                                                                    nat_add(a898, a899)?
                                                                };
                                                                let a915 = {
                                                                    let a912 = {
                                                                        let a910 = {
                                                                            let a900 = 1u64;
                                                                            let a901 = 1u64;
                                                                            nat_add(a900, a901)?
                                                                        };
                                                                        let a911 = {
                                                                            let a908 = {
                                                                                let a902 = 1u64;
                                                                                let a903 = 1u64;
                                                                                nat_add(a902, a903)?
                                                                            };
                                                                            let a909 = {
                                                                                let a906 = {
                                                                                    let a904 = 1u64;
                                                                                    let a905 = 1u64;
                                                                                    nat_add(a904, a905)?
                                                                                };
                                                                                let a907 = 1u64;
                                                                                nat_mul(a906, a907)?
                                                                            };
                                                                            nat_mul(a908, a909)?
                                                                        };
                                                                        nat_mul(a910, a911)?
                                                                    };
                                                                    let a913 = 1u64;
                                                                    nat_add(a912, a913)?
                                                                };
                                                                nat_mul(a914, a915)?
                                                            };
                                                            let a917 = 1u64;
                                                            nat_add(a916, a917)?
                                                        };
                                                        nat_mul(a918, a919)?
                                                    };
                                                    let a921 = 1u64;
                                                    nat_add(a920, a921)?
                                                };
                                                nat_mul(a922, a923)?
                                            };
                                            let a925 = 1u64;
                                            nat_add(a924, a925)?
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
                nat_add(a938, a939)?
            };
            let a942 = match v3.clone() {
                Adt1::C0(h940, _, _, _) => {
                    h940
                }
            };
            nat_sub(a941, a942)
        };
        let a944 = if v4 {
            1u64
        } else {
            0u64
        };
        nat_sub(a943, a944)
    };
    let v7: u64 = {
        let a946 = v5;
        let a947 = match v3.clone() {
            Adt1::C0(_, h945, _, _) => {
                h945
            }
        };
        let a948 = 0u64;
        nat_quot(a946, a947, a948)
    };
    let v8: bool = {
        let a953 = 0u64;
        let a954 = {
            let a950 = v5;
            let a951 = match v3 {
                Adt1::C0(_, h949, _, _) => {
                    h949
                }
            };
            let a952 = 0u64;
            nat_rem(a950, a951, a952)
        };
        nat_lt(a953, a954)
    };
    f28(f22(v0, v1)?, v6, f30(v7, v8)?)
}

pub fn f25(v0: u64) -> R<u64> {
    let a993 = {
        let a955 = f9(v0)?;
        let a956 = 0u64;
        if nat_eq(a955, a956) {
            1u64
        } else {
            f9(v0)?
        }
    };
    let a994 = {
        let a991 = {
            let a957 = 1u64;
            let a958 = 1u64;
            nat_add(a957, a958)?
        };
        let a992 = {
            let a989 = {
                let a959 = 1u64;
                let a960 = 1u64;
                nat_add(a959, a960)?
            };
            let a990 = {
                let a987 = {
                    let a961 = 1u64;
                    let a962 = 1u64;
                    nat_add(a961, a962)?
                };
                let a988 = {
                    let a985 = {
                        let a963 = 1u64;
                        let a964 = 1u64;
                        nat_add(a963, a964)?
                    };
                    let a986 = {
                        let a983 = {
                            let a965 = 1u64;
                            let a966 = 1u64;
                            nat_add(a965, a966)?
                        };
                        let a984 = {
                            let a981 = {
                                let a967 = 1u64;
                                let a968 = 1u64;
                                nat_add(a967, a968)?
                            };
                            let a982 = {
                                let a979 = {
                                    let a969 = 1u64;
                                    let a970 = 1u64;
                                    nat_add(a969, a970)?
                                };
                                let a980 = {
                                    let a977 = {
                                        let a971 = 1u64;
                                        let a972 = 1u64;
                                        nat_add(a971, a972)?
                                    };
                                    let a978 = {
                                        let a975 = {
                                            let a973 = 1u64;
                                            let a974 = 1u64;
                                            nat_add(a973, a974)?
                                        };
                                        let a976 = 1u64;
                                        nat_mul(a975, a976)?
                                    };
                                    nat_mul(a977, a978)?
                                };
                                nat_mul(a979, a980)?
                            };
                            nat_mul(a981, a982)?
                        };
                        nat_mul(a983, a984)?
                    };
                    nat_mul(a985, a986)?
                };
                nat_mul(a987, a988)?
            };
            nat_mul(a989, a990)?
        };
        nat_mul(a991, a992)?
    };
    nat_add(a993, a994)
}

pub fn f26(v0: u64) -> R<u64> {
    let a1019 = f10(v0)?;
    let a1020 = {
        let a995 = f9(v0)?;
        let a996 = 0u64;
        if nat_eq(a995, a996) {
            0u64
        } else {
            f8({
                let a1017 = {
                    let a1015 = {
                        let a997 = 1u64;
                        let a998 = 1u64;
                        nat_add(a997, a998)?
                    };
                    let a1016 = {
                        let a1013 = {
                            let a1011 = {
                                let a999 = 1u64;
                                let a1000 = 1u64;
                                nat_add(a999, a1000)?
                            };
                            let a1012 = {
                                let a1009 = {
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
                                let a1010 = 1u64;
                                nat_add(a1009, a1010)?
                            };
                            nat_mul(a1011, a1012)?
                        };
                        let a1014 = 1u64;
                        nat_add(a1013, a1014)?
                    };
                    nat_mul(a1015, a1016)?
                };
                let a1018 = 1u64;
                nat_add(a1017, a1018)?
            })?
        }
    };
    nat_add(a1019, a1020)
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let a1021 = v1;
    let a1022 = 0u64;
    if nat_eq(a1021, a1022) {
        Ok(v0)
    } else {
        let a1047 = {
            let a1045 = {
                let a1023 = 1u64;
                let a1024 = 1u64;
                nat_add(a1023, a1024)?
            };
            let a1046 = {
                let a1043 = {
                    let a1025 = 1u64;
                    let a1026 = 1u64;
                    nat_add(a1025, a1026)?
                };
                let a1044 = {
                    let a1041 = {
                        let a1027 = 1u64;
                        let a1028 = 1u64;
                        nat_add(a1027, a1028)?
                    };
                    let a1042 = {
                        let a1039 = {
                            let a1029 = 1u64;
                            let a1030 = 1u64;
                            nat_add(a1029, a1030)?
                        };
                        let a1040 = {
                            let a1037 = {
                                let a1031 = 1u64;
                                let a1032 = 1u64;
                                nat_add(a1031, a1032)?
                            };
                            let a1038 = {
                                let a1035 = {
                                    let a1033 = 1u64;
                                    let a1034 = 1u64;
                                    nat_add(a1033, a1034)?
                                };
                                let a1036 = 1u64;
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
        let a1048 = v1;
        if nat_le(a1047, a1048) {
            let a1049 = v0;
            let a1050 = 0u64;
            if nat_eq(a1049, a1050) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f8(v1)?;
            f30({
                let a1051 = v0;
                let a1052 = v2;
                let a1053 = 0u64;
                nat_quot(a1051, a1052, a1053)
            }, {
                let a1057 = 0u64;
                let a1058 = {
                    let a1054 = v0;
                    let a1055 = v2;
                    let a1056 = 0u64;
                    nat_rem(a1054, a1055, a1056)
                };
                nat_lt(a1057, a1058)
            })
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1081 = f8({
        let a1079 = {
            let a1077 = {
                let a1059 = 1u64;
                let a1060 = 1u64;
                nat_add(a1059, a1060)?
            };
            let a1078 = {
                let a1075 = {
                    let a1073 = {
                        let a1061 = 1u64;
                        let a1062 = 1u64;
                        nat_add(a1061, a1062)?
                    };
                    let a1074 = {
                        let a1071 = {
                            let a1063 = 1u64;
                            let a1064 = 1u64;
                            nat_add(a1063, a1064)?
                        };
                        let a1072 = {
                            let a1069 = {
                                let a1067 = {
                                    let a1065 = 1u64;
                                    let a1066 = 1u64;
                                    nat_add(a1065, a1066)?
                                };
                                let a1068 = 1u64;
                                nat_mul(a1067, a1068)?
                            };
                            let a1070 = 1u64;
                            nat_add(a1069, a1070)?
                        };
                        nat_mul(a1071, a1072)?
                    };
                    nat_mul(a1073, a1074)?
                };
                let a1076 = 1u64;
                nat_add(a1075, a1076)?
            };
            nat_mul(a1077, a1078)?
        };
        let a1080 = 1u64;
        nat_add(a1079, a1080)?
    })?;
    let a1082 = v2;
    if nat_le(a1081, a1082) {
        f31(v0, {
            let a1083 = v1;
            let a1084 = 1u64;
            nat_add(a1083, a1084)?
        }, f27(v2, 1u64)?)
    } else {
        let v3: Adt1 = f32(v1, v2, {
            let a1121 = {
                let a1119 = {
                    let a1085 = 1u64;
                    let a1086 = 1u64;
                    nat_add(a1085, a1086)?
                };
                let a1120 = {
                    let a1117 = {
                        let a1087 = 1u64;
                        let a1088 = 1u64;
                        nat_add(a1087, a1088)?
                    };
                    let a1118 = {
                        let a1115 = {
                            let a1089 = 1u64;
                            let a1090 = 1u64;
                            nat_add(a1089, a1090)?
                        };
                        let a1116 = {
                            let a1113 = {
                                let a1091 = 1u64;
                                let a1092 = 1u64;
                                nat_add(a1091, a1092)?
                            };
                            let a1114 = {
                                let a1111 = {
                                    let a1093 = 1u64;
                                    let a1094 = 1u64;
                                    nat_add(a1093, a1094)?
                                };
                                let a1112 = {
                                    let a1109 = {
                                        let a1095 = 1u64;
                                        let a1096 = 1u64;
                                        nat_add(a1095, a1096)?
                                    };
                                    let a1110 = {
                                        let a1107 = {
                                            let a1097 = 1u64;
                                            let a1098 = 1u64;
                                            nat_add(a1097, a1098)?
                                        };
                                        let a1108 = {
                                            let a1105 = {
                                                let a1099 = 1u64;
                                                let a1100 = 1u64;
                                                nat_add(a1099, a1100)?
                                            };
                                            let a1106 = {
                                                let a1103 = {
                                                    let a1101 = 1u64;
                                                    let a1102 = 1u64;
                                                    nat_add(a1101, a1102)?
                                                };
                                                let a1104 = 1u64;
                                                nat_mul(a1103, a1104)?
                                            };
                                            nat_mul(a1105, a1106)?
                                        };
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
            let a1122 = 1u64;
            nat_add(a1121, a1122)?
        }, f8({
            let a1141 = {
                let a1123 = 1u64;
                let a1124 = 1u64;
                nat_add(a1123, a1124)?
            };
            let a1142 = {
                let a1139 = {
                    let a1137 = {
                        let a1125 = 1u64;
                        let a1126 = 1u64;
                        nat_add(a1125, a1126)?
                    };
                    let a1138 = {
                        let a1135 = {
                            let a1127 = 1u64;
                            let a1128 = 1u64;
                            nat_add(a1127, a1128)?
                        };
                        let a1136 = {
                            let a1133 = {
                                let a1131 = {
                                    let a1129 = 1u64;
                                    let a1130 = 1u64;
                                    nat_add(a1129, a1130)?
                                };
                                let a1132 = 1u64;
                                nat_mul(a1131, a1132)?
                            };
                            let a1134 = 1u64;
                            nat_add(a1133, a1134)?
                        };
                        nat_mul(a1135, a1136)?
                    };
                    nat_mul(a1137, a1138)?
                };
                let a1140 = 1u64;
                nat_add(a1139, a1140)?
            };
            nat_mul(a1141, a1142)?
        })?)?;
        f31(v0, match v3.clone() {
            Adt1::C0(h1143, _, _, _) => {
                h1143
            }
        }, match v3 {
            Adt1::C0(_, h1144, _, _) => {
                h1144
            }
        })
    }
}

pub fn f29(v0: u64) -> R<Adt1> {
    f32(f25(v0)?, f26(v0)?, 0u64, f8({
        let a1165 = {
            let a1163 = {
                let a1145 = 1u64;
                let a1146 = 1u64;
                nat_add(a1145, a1146)?
            };
            let a1164 = {
                let a1161 = {
                    let a1159 = {
                        let a1147 = 1u64;
                        let a1148 = 1u64;
                        nat_add(a1147, a1148)?
                    };
                    let a1160 = {
                        let a1157 = {
                            let a1155 = {
                                let a1149 = 1u64;
                                let a1150 = 1u64;
                                nat_add(a1149, a1150)?
                            };
                            let a1156 = {
                                let a1153 = {
                                    let a1151 = 1u64;
                                    let a1152 = 1u64;
                                    nat_add(a1151, a1152)?
                                };
                                let a1154 = 1u64;
                                nat_mul(a1153, a1154)?
                            };
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
    })?)
}

pub fn f30(v0: u64, v1: bool) -> R<u64> {
    if if v1 {
        let a1174 = {
            let a1171 = v0;
            let a1172 = {
                let a1169 = {
                    let a1167 = 1u64;
                    let a1168 = 1u64;
                    nat_add(a1167, a1168)?
                };
                let a1170 = 1u64;
                nat_mul(a1169, a1170)?
            };
            let a1173 = 0u64;
            nat_rem(a1171, a1172, a1173)
        };
        let a1175 = 0u64;
        nat_eq(a1174, a1175)
    } else {
        false
    } {
        let a1176 = v0;
        let a1177 = 1u64;
        nat_add(a1176, a1177)
    } else {
        Ok(v0)
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1216 = v1;
    let a1217 = {
        let a1214 = {
            let a1212 = {
                let a1178 = 1u64;
                let a1179 = 1u64;
                nat_add(a1178, a1179)?
            };
            let a1213 = {
                let a1210 = {
                    let a1180 = 1u64;
                    let a1181 = 1u64;
                    nat_add(a1180, a1181)?
                };
                let a1211 = {
                    let a1208 = {
                        let a1182 = 1u64;
                        let a1183 = 1u64;
                        nat_add(a1182, a1183)?
                    };
                    let a1209 = {
                        let a1206 = {
                            let a1184 = 1u64;
                            let a1185 = 1u64;
                            nat_add(a1184, a1185)?
                        };
                        let a1207 = {
                            let a1204 = {
                                let a1186 = 1u64;
                                let a1187 = 1u64;
                                nat_add(a1186, a1187)?
                            };
                            let a1205 = {
                                let a1202 = {
                                    let a1188 = 1u64;
                                    let a1189 = 1u64;
                                    nat_add(a1188, a1189)?
                                };
                                let a1203 = {
                                    let a1200 = {
                                        let a1190 = 1u64;
                                        let a1191 = 1u64;
                                        nat_add(a1190, a1191)?
                                    };
                                    let a1201 = {
                                        let a1198 = {
                                            let a1192 = 1u64;
                                            let a1193 = 1u64;
                                            nat_add(a1192, a1193)?
                                        };
                                        let a1199 = {
                                            let a1196 = {
                                                let a1194 = 1u64;
                                                let a1195 = 1u64;
                                                nat_add(a1194, a1195)?
                                            };
                                            let a1197 = 1u64;
                                            nat_mul(a1196, a1197)?
                                        };
                                        nat_mul(a1198, a1199)?
                                    };
                                    nat_mul(a1200, a1201)?
                                };
                                nat_mul(a1202, a1203)?
                            };
                            nat_mul(a1204, a1205)?
                        };
                        nat_mul(a1206, a1207)?
                    };
                    nat_mul(a1208, a1209)?
                };
                nat_mul(a1210, a1211)?
            };
            nat_mul(a1212, a1213)?
        };
        let a1215 = 1u64;
        nat_add(a1214, a1215)?
    };
    if nat_lt(a1216, a1217) {
        f33(v0, {
            let a1254 = {
                let a1252 = {
                    let a1218 = 1u64;
                    let a1219 = 1u64;
                    nat_add(a1218, a1219)?
                };
                let a1253 = {
                    let a1250 = {
                        let a1220 = 1u64;
                        let a1221 = 1u64;
                        nat_add(a1220, a1221)?
                    };
                    let a1251 = {
                        let a1248 = {
                            let a1222 = 1u64;
                            let a1223 = 1u64;
                            nat_add(a1222, a1223)?
                        };
                        let a1249 = {
                            let a1246 = {
                                let a1224 = 1u64;
                                let a1225 = 1u64;
                                nat_add(a1224, a1225)?
                            };
                            let a1247 = {
                                let a1244 = {
                                    let a1226 = 1u64;
                                    let a1227 = 1u64;
                                    nat_add(a1226, a1227)?
                                };
                                let a1245 = {
                                    let a1242 = {
                                        let a1228 = 1u64;
                                        let a1229 = 1u64;
                                        nat_add(a1228, a1229)?
                                    };
                                    let a1243 = {
                                        let a1240 = {
                                            let a1230 = 1u64;
                                            let a1231 = 1u64;
                                            nat_add(a1230, a1231)?
                                        };
                                        let a1241 = {
                                            let a1238 = {
                                                let a1232 = 1u64;
                                                let a1233 = 1u64;
                                                nat_add(a1232, a1233)?
                                            };
                                            let a1239 = {
                                                let a1236 = {
                                                    let a1234 = 1u64;
                                                    let a1235 = 1u64;
                                                    nat_add(a1234, a1235)?
                                                };
                                                let a1237 = 1u64;
                                                nat_mul(a1236, a1237)?
                                            };
                                            nat_mul(a1238, a1239)?
                                        };
                                        nat_mul(a1240, a1241)?
                                    };
                                    nat_mul(a1242, a1243)?
                                };
                                nat_mul(a1244, a1245)?
                            };
                            nat_mul(a1246, a1247)?
                        };
                        nat_mul(a1248, a1249)?
                    };
                    nat_mul(a1250, a1251)?
                };
                nat_mul(a1252, a1253)?
            };
            let a1255 = 1u64;
            nat_add(a1254, a1255)?
        }, f27(v2, {
            let a1294 = {
                let a1292 = {
                    let a1290 = {
                        let a1256 = 1u64;
                        let a1257 = 1u64;
                        nat_add(a1256, a1257)?
                    };
                    let a1291 = {
                        let a1288 = {
                            let a1258 = 1u64;
                            let a1259 = 1u64;
                            nat_add(a1258, a1259)?
                        };
                        let a1289 = {
                            let a1286 = {
                                let a1260 = 1u64;
                                let a1261 = 1u64;
                                nat_add(a1260, a1261)?
                            };
                            let a1287 = {
                                let a1284 = {
                                    let a1262 = 1u64;
                                    let a1263 = 1u64;
                                    nat_add(a1262, a1263)?
                                };
                                let a1285 = {
                                    let a1282 = {
                                        let a1264 = 1u64;
                                        let a1265 = 1u64;
                                        nat_add(a1264, a1265)?
                                    };
                                    let a1283 = {
                                        let a1280 = {
                                            let a1266 = 1u64;
                                            let a1267 = 1u64;
                                            nat_add(a1266, a1267)?
                                        };
                                        let a1281 = {
                                            let a1278 = {
                                                let a1268 = 1u64;
                                                let a1269 = 1u64;
                                                nat_add(a1268, a1269)?
                                            };
                                            let a1279 = {
                                                let a1276 = {
                                                    let a1270 = 1u64;
                                                    let a1271 = 1u64;
                                                    nat_add(a1270, a1271)?
                                                };
                                                let a1277 = {
                                                    let a1274 = {
                                                        let a1272 = 1u64;
                                                        let a1273 = 1u64;
                                                        nat_add(a1272, a1273)?
                                                    };
                                                    let a1275 = 1u64;
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
                            nat_mul(a1286, a1287)?
                        };
                        nat_mul(a1288, a1289)?
                    };
                    nat_mul(a1290, a1291)?
                };
                let a1293 = 1u64;
                nat_add(a1292, a1293)?
            };
            let a1295 = v1;
            nat_sub(a1294, a1295)
        })?)
    } else {
        f33(v0, v1, v2)
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let (h1296, _) = f34(Fn1::F35, {
        let a1315 = {
            let a1297 = 1u64;
            let a1298 = 1u64;
            nat_add(a1297, a1298)?
        };
        let a1316 = {
            let a1313 = {
                let a1299 = 1u64;
                let a1300 = 1u64;
                nat_add(a1299, a1300)?
            };
            let a1314 = {
                let a1311 = {
                    let a1301 = 1u64;
                    let a1302 = 1u64;
                    nat_add(a1301, a1302)?
                };
                let a1312 = {
                    let a1309 = {
                        let a1303 = 1u64;
                        let a1304 = 1u64;
                        nat_add(a1303, a1304)?
                    };
                    let a1310 = {
                        let a1307 = {
                            let a1305 = 1u64;
                            let a1306 = 1u64;
                            nat_add(a1305, a1306)?
                        };
                        let a1308 = 1u64;
                        nat_mul(a1307, a1308)?
                    };
                    nat_mul(a1309, a1310)?
                };
                nat_mul(a1311, a1312)?
            };
            nat_mul(a1313, a1314)?
        };
        nat_mul(a1315, a1316)?
    }, Adt1::C0(v0, v1, v2, v3))?;
    Ok(h1296)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1329 = v2;
        let a1330 = {
            let a1327 = {
                let a1317 = 1u64;
                let a1318 = 1u64;
                nat_add(a1317, a1318)?
            };
            let a1328 = {
                let a1325 = {
                    let a1319 = 1u64;
                    let a1320 = 1u64;
                    nat_add(a1319, a1320)?
                };
                let a1326 = {
                    let a1323 = {
                        let a1321 = 1u64;
                        let a1322 = 1u64;
                        nat_add(a1321, a1322)?
                    };
                    let a1324 = 1u64;
                    nat_mul(a1323, a1324)?
                };
                nat_mul(a1325, a1326)?
            };
            nat_mul(a1327, a1328)?
        };
        let a1331 = 0u64;
        nat_rem(a1329, a1330, a1331)
    };
    let v4: u64 = {
        let a1344 = v2;
        let a1345 = {
            let a1342 = {
                let a1332 = 1u64;
                let a1333 = 1u64;
                nat_add(a1332, a1333)?
            };
            let a1343 = {
                let a1340 = {
                    let a1334 = 1u64;
                    let a1335 = 1u64;
                    nat_add(a1334, a1335)?
                };
                let a1341 = {
                    let a1338 = {
                        let a1336 = 1u64;
                        let a1337 = 1u64;
                        nat_add(a1336, a1337)?
                    };
                    let a1339 = 1u64;
                    nat_mul(a1338, a1339)?
                };
                nat_mul(a1340, a1341)?
            };
            nat_mul(a1342, a1343)?
        };
        let a1346 = 0u64;
        nat_quot(a1344, a1345, a1346)
    };
    let v5: bool = {
        let a1355 = {
            let a1353 = {
                let a1347 = 1u64;
                let a1348 = 1u64;
                nat_add(a1347, a1348)?
            };
            let a1354 = {
                let a1351 = {
                    let a1349 = 1u64;
                    let a1350 = 1u64;
                    nat_add(a1349, a1350)?
                };
                let a1352 = 1u64;
                nat_mul(a1351, a1352)?
            };
            nat_mul(a1353, a1354)?
        };
        let a1356 = v3;
        if nat_lt(a1355, a1356) {
            true
        } else {
            let a1365 = v3;
            let a1366 = {
                let a1363 = {
                    let a1357 = 1u64;
                    let a1358 = 1u64;
                    nat_add(a1357, a1358)?
                };
                let a1364 = {
                    let a1361 = {
                        let a1359 = 1u64;
                        let a1360 = 1u64;
                        nat_add(a1359, a1360)?
                    };
                    let a1362 = 1u64;
                    nat_mul(a1361, a1362)?
                };
                nat_mul(a1363, a1364)?
            };
            if nat_eq(a1365, a1366) {
                let a1374 = {
                    let a1371 = v4;
                    let a1372 = {
                        let a1369 = {
                            let a1367 = 1u64;
                            let a1368 = 1u64;
                            nat_add(a1367, a1368)?
                        };
                        let a1370 = 1u64;
                        nat_mul(a1369, a1370)?
                    };
                    let a1373 = 0u64;
                    nat_rem(a1371, a1372, a1373)
                };
                let a1375 = 1u64;
                nat_eq(a1374, a1375)
            } else {
                false
            }
        }
    };
    let v6: u64 = {
        let a1376 = v4;
        let a1377 = if v5 {
            1u64
        } else {
            0u64
        };
        nat_add(a1376, a1377)?
    };
    let v7: bool = {
        let a1406 = {
            let a1404 = {
                let a1380 = {
                    let a1378 = 1u64;
                    let a1379 = 1u64;
                    nat_add(a1378, a1379)?
                };
                let a1381 = 1u64;
                nat_mul(a1380, a1381)?
            };
            let a1405 = f8({
                let a1402 = {
                    let a1400 = {
                        let a1382 = 1u64;
                        let a1383 = 1u64;
                        nat_add(a1382, a1383)?
                    };
                    let a1401 = {
                        let a1398 = {
                            let a1396 = {
                                let a1384 = 1u64;
                                let a1385 = 1u64;
                                nat_add(a1384, a1385)?
                            };
                            let a1397 = {
                                let a1394 = {
                                    let a1392 = {
                                        let a1386 = 1u64;
                                        let a1387 = 1u64;
                                        nat_add(a1386, a1387)?
                                    };
                                    let a1393 = {
                                        let a1390 = {
                                            let a1388 = 1u64;
                                            let a1389 = 1u64;
                                            nat_add(a1388, a1389)?
                                        };
                                        let a1391 = 1u64;
                                        nat_mul(a1390, a1391)?
                                    };
                                    nat_mul(a1392, a1393)?
                                };
                                let a1395 = 1u64;
                                nat_add(a1394, a1395)?
                            };
                            nat_mul(a1396, a1397)?
                        };
                        let a1399 = 1u64;
                        nat_add(a1398, a1399)?
                    };
                    nat_mul(a1400, a1401)?
                };
                let a1403 = 1u64;
                nat_add(a1402, a1403)?
            })?;
            nat_mul(a1404, a1405)?
        };
        let a1407 = v6;
        nat_le(a1406, a1407)
    };
    let v8: u64 = if v7 {
        let a1412 = v6;
        let a1413 = {
            let a1410 = {
                let a1408 = 1u64;
                let a1409 = 1u64;
                nat_add(a1408, a1409)?
            };
            let a1411 = 1u64;
            nat_mul(a1410, a1411)?
        };
        let a1414 = 0u64;
        nat_quot(a1412, a1413, a1414)
    } else {
        v6
    };
    let v9: u64 = if v7 {
        let a1415 = v1;
        let a1416 = 1u64;
        nat_add(a1415, a1416)?
    } else {
        v1
    };
    let v10: u64 = {
        let a1455 = v9;
        let a1456 = {
            let a1453 = {
                let a1451 = {
                    let a1417 = 1u64;
                    let a1418 = 1u64;
                    nat_add(a1417, a1418)?
                };
                let a1452 = {
                    let a1449 = {
                        let a1419 = 1u64;
                        let a1420 = 1u64;
                        nat_add(a1419, a1420)?
                    };
                    let a1450 = {
                        let a1447 = {
                            let a1421 = 1u64;
                            let a1422 = 1u64;
                            nat_add(a1421, a1422)?
                        };
                        let a1448 = {
                            let a1445 = {
                                let a1423 = 1u64;
                                let a1424 = 1u64;
                                nat_add(a1423, a1424)?
                            };
                            let a1446 = {
                                let a1443 = {
                                    let a1425 = 1u64;
                                    let a1426 = 1u64;
                                    nat_add(a1425, a1426)?
                                };
                                let a1444 = {
                                    let a1441 = {
                                        let a1427 = 1u64;
                                        let a1428 = 1u64;
                                        nat_add(a1427, a1428)?
                                    };
                                    let a1442 = {
                                        let a1439 = {
                                            let a1429 = 1u64;
                                            let a1430 = 1u64;
                                            nat_add(a1429, a1430)?
                                        };
                                        let a1440 = {
                                            let a1437 = {
                                                let a1431 = 1u64;
                                                let a1432 = 1u64;
                                                nat_add(a1431, a1432)?
                                            };
                                            let a1438 = {
                                                let a1435 = {
                                                    let a1433 = 1u64;
                                                    let a1434 = 1u64;
                                                    nat_add(a1433, a1434)?
                                                };
                                                let a1436 = 1u64;
                                                nat_mul(a1435, a1436)?
                                            };
                                            nat_mul(a1437, a1438)?
                                        };
                                        nat_mul(a1439, a1440)?
                                    };
                                    nat_mul(a1441, a1442)?
                                };
                                nat_mul(a1443, a1444)?
                            };
                            nat_mul(a1445, a1446)?
                        };
                        nat_mul(a1447, a1448)?
                    };
                    nat_mul(a1449, a1450)?
                };
                nat_mul(a1451, a1452)?
            };
            let a1454 = 1u64;
            nat_add(a1453, a1454)?
        };
        if if nat_eq(a1455, a1456) {
            let a1479 = v8;
            let a1480 = f8({
                let a1477 = {
                    let a1475 = {
                        let a1457 = 1u64;
                        let a1458 = 1u64;
                        nat_add(a1457, a1458)?
                    };
                    let a1476 = {
                        let a1473 = {
                            let a1471 = {
                                let a1459 = 1u64;
                                let a1460 = 1u64;
                                nat_add(a1459, a1460)?
                            };
                            let a1472 = {
                                let a1469 = {
                                    let a1467 = {
                                        let a1461 = 1u64;
                                        let a1462 = 1u64;
                                        nat_add(a1461, a1462)?
                                    };
                                    let a1468 = {
                                        let a1465 = {
                                            let a1463 = 1u64;
                                            let a1464 = 1u64;
                                            nat_add(a1463, a1464)?
                                        };
                                        let a1466 = 1u64;
                                        nat_mul(a1465, a1466)?
                                    };
                                    nat_mul(a1467, a1468)?
                                };
                                let a1470 = 1u64;
                                nat_add(a1469, a1470)?
                            };
                            nat_mul(a1471, a1472)?
                        };
                        let a1474 = 1u64;
                        nat_add(a1473, a1474)?
                    };
                    nat_mul(a1475, a1476)?
                };
                let a1478 = 1u64;
                nat_add(a1477, a1478)?
            })?;
            nat_lt(a1479, a1480)
        } else {
            false
        } {
            0u64
        } else {
            let a1517 = v9;
            let a1518 = {
                let a1515 = {
                    let a1481 = 1u64;
                    let a1482 = 1u64;
                    nat_add(a1481, a1482)?
                };
                let a1516 = {
                    let a1513 = {
                        let a1483 = 1u64;
                        let a1484 = 1u64;
                        nat_add(a1483, a1484)?
                    };
                    let a1514 = {
                        let a1511 = {
                            let a1485 = 1u64;
                            let a1486 = 1u64;
                            nat_add(a1485, a1486)?
                        };
                        let a1512 = {
                            let a1509 = {
                                let a1487 = 1u64;
                                let a1488 = 1u64;
                                nat_add(a1487, a1488)?
                            };
                            let a1510 = {
                                let a1507 = {
                                    let a1489 = 1u64;
                                    let a1490 = 1u64;
                                    nat_add(a1489, a1490)?
                                };
                                let a1508 = {
                                    let a1505 = {
                                        let a1491 = 1u64;
                                        let a1492 = 1u64;
                                        nat_add(a1491, a1492)?
                                    };
                                    let a1506 = {
                                        let a1503 = {
                                            let a1493 = 1u64;
                                            let a1494 = 1u64;
                                            nat_add(a1493, a1494)?
                                        };
                                        let a1504 = {
                                            let a1501 = {
                                                let a1495 = 1u64;
                                                let a1496 = 1u64;
                                                nat_add(a1495, a1496)?
                                            };
                                            let a1502 = {
                                                let a1499 = {
                                                    let a1497 = 1u64;
                                                    let a1498 = 1u64;
                                                    nat_add(a1497, a1498)?
                                                };
                                                let a1500 = 1u64;
                                                nat_mul(a1499, a1500)?
                                            };
                                            nat_mul(a1501, a1502)?
                                        };
                                        nat_mul(a1503, a1504)?
                                    };
                                    nat_mul(a1505, a1506)?
                                };
                                nat_mul(a1507, a1508)?
                            };
                            nat_mul(a1509, a1510)?
                        };
                        nat_mul(a1511, a1512)?
                    };
                    nat_mul(a1513, a1514)?
                };
                nat_mul(a1515, a1516)?
            };
            nat_sub(a1517, a1518)
        }
    };
    let a1519 = v8;
    let a1520 = 0u64;
    if nat_eq(a1519, a1520) {
        f11(v0)
    } else {
        let a1573 = {
            let a1571 = {
                let a1569 = {
                    let a1521 = 1u64;
                    let a1522 = 1u64;
                    nat_add(a1521, a1522)?
                };
                let a1570 = {
                    let a1567 = {
                        let a1565 = {
                            let a1523 = 1u64;
                            let a1524 = 1u64;
                            nat_add(a1523, a1524)?
                        };
                        let a1566 = {
                            let a1563 = {
                                let a1561 = {
                                    let a1525 = 1u64;
                                    let a1526 = 1u64;
                                    nat_add(a1525, a1526)?
                                };
                                let a1562 = {
                                    let a1559 = {
                                        let a1557 = {
                                            let a1527 = 1u64;
                                            let a1528 = 1u64;
                                            nat_add(a1527, a1528)?
                                        };
                                        let a1558 = {
                                            let a1555 = {
                                                let a1553 = {
                                                    let a1529 = 1u64;
                                                    let a1530 = 1u64;
                                                    nat_add(a1529, a1530)?
                                                };
                                                let a1554 = {
                                                    let a1551 = {
                                                        let a1549 = {
                                                            let a1531 = 1u64;
                                                            let a1532 = 1u64;
                                                            nat_add(a1531, a1532)?
                                                        };
                                                        let a1550 = {
                                                            let a1547 = {
                                                                let a1545 = {
                                                                    let a1533 = 1u64;
                                                                    let a1534 = 1u64;
                                                                    nat_add(a1533, a1534)?
                                                                };
                                                                let a1546 = {
                                                                    let a1543 = {
                                                                        let a1541 = {
                                                                            let a1535 = 1u64;
                                                                            let a1536 = 1u64;
                                                                            nat_add(a1535, a1536)?
                                                                        };
                                                                        let a1542 = {
                                                                            let a1539 = {
                                                                                let a1537 = 1u64;
                                                                                let a1538 = 1u64;
                                                                                nat_add(a1537, a1538)?
                                                                            };
                                                                            let a1540 = 1u64;
                                                                            nat_mul(a1539, a1540)?
                                                                        };
                                                                        nat_mul(a1541, a1542)?
                                                                    };
                                                                    let a1544 = 1u64;
                                                                    nat_add(a1543, a1544)?
                                                                };
                                                                nat_mul(a1545, a1546)?
                                                            };
                                                            let a1548 = 1u64;
                                                            nat_add(a1547, a1548)?
                                                        };
                                                        nat_mul(a1549, a1550)?
                                                    };
                                                    let a1552 = 1u64;
                                                    nat_add(a1551, a1552)?
                                                };
                                                nat_mul(a1553, a1554)?
                                            };
                                            let a1556 = 1u64;
                                            nat_add(a1555, a1556)?
                                        };
                                        nat_mul(a1557, a1558)?
                                    };
                                    let a1560 = 1u64;
                                    nat_add(a1559, a1560)?
                                };
                                nat_mul(a1561, a1562)?
                            };
                            let a1564 = 1u64;
                            nat_add(a1563, a1564)?
                        };
                        nat_mul(a1565, a1566)?
                    };
                    let a1568 = 1u64;
                    nat_add(a1567, a1568)?
                };
                nat_mul(a1569, a1570)?
            };
            let a1572 = 1u64;
            nat_add(a1571, a1572)?
        };
        let a1574 = v9;
        if nat_le(a1573, a1574) {
            f21(v0)
        } else {
            let a1626 = {
                let a1599 = f11(v0)?;
                let a1600 = {
                    let a1597 = v10;
                    let a1598 = f8({
                        let a1595 = {
                            let a1593 = {
                                let a1575 = 1u64;
                                let a1576 = 1u64;
                                nat_add(a1575, a1576)?
                            };
                            let a1594 = {
                                let a1591 = {
                                    let a1589 = {
                                        let a1577 = 1u64;
                                        let a1578 = 1u64;
                                        nat_add(a1577, a1578)?
                                    };
                                    let a1590 = {
                                        let a1587 = {
                                            let a1585 = {
                                                let a1579 = 1u64;
                                                let a1580 = 1u64;
                                                nat_add(a1579, a1580)?
                                            };
                                            let a1586 = {
                                                let a1583 = {
                                                    let a1581 = 1u64;
                                                    let a1582 = 1u64;
                                                    nat_add(a1581, a1582)?
                                                };
                                                let a1584 = 1u64;
                                                nat_mul(a1583, a1584)?
                                            };
                                            nat_mul(a1585, a1586)?
                                        };
                                        let a1588 = 1u64;
                                        nat_add(a1587, a1588)?
                                    };
                                    nat_mul(a1589, a1590)?
                                };
                                let a1592 = 1u64;
                                nat_add(a1591, a1592)?
                            };
                            nat_mul(a1593, a1594)?
                        };
                        let a1596 = 1u64;
                        nat_add(a1595, a1596)?
                    })?;
                    nat_mul(a1597, a1598)?
                };
                nat_add(a1599, a1600)?
            };
            let a1627 = {
                let a1623 = v8;
                let a1624 = f8({
                    let a1621 = {
                        let a1619 = {
                            let a1601 = 1u64;
                            let a1602 = 1u64;
                            nat_add(a1601, a1602)?
                        };
                        let a1620 = {
                            let a1617 = {
                                let a1615 = {
                                    let a1603 = 1u64;
                                    let a1604 = 1u64;
                                    nat_add(a1603, a1604)?
                                };
                                let a1616 = {
                                    let a1613 = {
                                        let a1611 = {
                                            let a1605 = 1u64;
                                            let a1606 = 1u64;
                                            nat_add(a1605, a1606)?
                                        };
                                        let a1612 = {
                                            let a1609 = {
                                                let a1607 = 1u64;
                                                let a1608 = 1u64;
                                                nat_add(a1607, a1608)?
                                            };
                                            let a1610 = 1u64;
                                            nat_mul(a1609, a1610)?
                                        };
                                        nat_mul(a1611, a1612)?
                                    };
                                    let a1614 = 1u64;
                                    nat_add(a1613, a1614)?
                                };
                                nat_mul(a1615, a1616)?
                            };
                            let a1618 = 1u64;
                            nat_add(a1617, a1618)?
                        };
                        nat_mul(a1619, a1620)?
                    };
                    let a1622 = 1u64;
                    nat_add(a1621, a1622)?
                })?;
                let a1625 = 0u64;
                nat_rem(a1623, a1624, a1625)
            };
            nat_add(a1626, a1627)
        }
    }
}

pub fn f34(v0: Fn1, p1632: u64, p1633: Adt1) -> R<(Adt1, bool)> {
    let m1631 = (p1632, p1633);
    {
        let mut m1631 = m1631;
        loop {
            let step = {
                let (v1, v2) = m1631;
                let m1628 = v1;
                if m1628 == 0 {
                    Err::<(u64, Adt1), R<(Adt1, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1628 - 1;
                    let m1630 = {
                        let c1629: Fn1 = v0.clone();
                        c1629.apply(v2.clone())?
                    };
                    match m1630 {
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
                Ok(next) => { m1631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Adt1) -> R<Option<Adt1>> {
    f36(v0)
}

pub fn f36(v0: Adt1) -> R<Option<Adt1>> {
    let a1635 = 0u64;
    let a1636 = match v0.clone() {
        Adt1::C0(_, h1634, _, _) => {
            h1634
        }
    };
    if if if nat_lt(a1635, a1636) {
        let a1639 = match v0.clone() {
            Adt1::C0(_, h1637, _, _) => {
                h1637
            }
        };
        let a1640 = match v0.clone() {
            Adt1::C0(_, _, _, h1638) => {
                h1638
            }
        };
        nat_lt(a1639, a1640)
    } else {
        false
    } {
        let a1643 = match v0.clone() {
            Adt1::C0(_, _, h1641, _) => {
                h1641
            }
        };
        let a1644 = match v0.clone() {
            Adt1::C0(h1642, _, _, _) => {
                h1642
            }
        };
        nat_lt(a1643, a1644)
    } else {
        false
    } {
        Ok(Some(Adt1::C0({
            let a1646 = match v0.clone() {
                Adt1::C0(h1645, _, _, _) => {
                    h1645
                }
            };
            let a1647 = 1u64;
            nat_sub(a1646, a1647)
        }, {
            let a1650 = match v0.clone() {
                Adt1::C0(_, h1648, _, _) => {
                    h1648
                }
            };
            let a1651 = match v0.clone() {
                Adt1::C0(_, h1649, _, _) => {
                    h1649
                }
            };
            nat_add(a1650, a1651)?
        }, match v0.clone() {
            Adt1::C0(_, _, h1652, _) => {
                h1652
            }
        }, match v0 {
            Adt1::C0(_, _, _, h1653) => {
                h1653
            }
        })))
    } else {
        Ok(None::<Adt1>)
    }
}
