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
    C1(u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F8,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F8 => f8(p0),
        }
    }
}

pub fn f0(v0: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 838860800u64;
    if nat_le(a1, a2) {
        Ok(Adt0::C0(1065353216u64))
    } else {
        let a3 = 1125515264u64;
        let a4 = v0;
        if nat_le(a3, a4) {
            Ok(Adt0::C0(0u64))
        } else {
            let v1: u64 = f1(v0)?;
            let v2: u64 = f2(v0)?;
            let v3: u64 = {
                let a5 = 102u64;
                let a6 = v1;
                if nat_le(a5, a6) {
                    let a9 = v2;
                    let a10 = f3({
                        let a7 = v1;
                        let a8 = 102u64;
                        nat_sub(a7, a8)
                    })?;
                    nat_mul(a9, a10)?
                } else {
                    f4(v2, f3({
                        let a11 = 102u64;
                        let a12 = v1;
                        nat_sub(a11, a12)
                    })?)?
                }
            };
            Ok(Adt0::C1({
                let a13 = v3;
                let a14 = 281474976710656u64;
                let a15 = 0u64;
                nat_quot(a13, a14, a15)
            }, {
                let a16 = v3;
                let a17 = 281474976710656u64;
                let a18 = 0u64;
                nat_rem(a16, a17, a18)
            }, 195103586505167u64, 16u64, 281474976710656u64))
        }
    }
}

pub fn f1(v0: u64) -> R<u64> {
    let a41 = f5(v0)?;
    let a42 = f3({
        let a39 = {
            let a37 = {
                let a19 = 1u64;
                let a20 = 1u64;
                nat_add(a19, a20)?
            };
            let a38 = {
                let a35 = {
                    let a33 = {
                        let a21 = 1u64;
                        let a22 = 1u64;
                        nat_add(a21, a22)?
                    };
                    let a34 = {
                        let a31 = {
                            let a29 = {
                                let a23 = 1u64;
                                let a24 = 1u64;
                                nat_add(a23, a24)?
                            };
                            let a30 = {
                                let a27 = {
                                    let a25 = 1u64;
                                    let a26 = 1u64;
                                    nat_add(a25, a26)?
                                };
                                let a28 = 1u64;
                                nat_mul(a27, a28)?
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
    })?;
    let a43 = 0u64;
    Ok(nat_quot(a41, a42, a43))
}

pub fn f2(v0: u64) -> R<u64> {
    let a68 = f6(v0)?;
    let a69 = {
        let a44 = f1(v0)?;
        let a45 = 0u64;
        if nat_eq(a44, a45) {
            0u64
        } else {
            f3({
                let a66 = {
                    let a64 = {
                        let a46 = 1u64;
                        let a47 = 1u64;
                        nat_add(a46, a47)?
                    };
                    let a65 = {
                        let a62 = {
                            let a60 = {
                                let a48 = 1u64;
                                let a49 = 1u64;
                                nat_add(a48, a49)?
                            };
                            let a61 = {
                                let a58 = {
                                    let a56 = {
                                        let a50 = 1u64;
                                        let a51 = 1u64;
                                        nat_add(a50, a51)?
                                    };
                                    let a57 = {
                                        let a54 = {
                                            let a52 = 1u64;
                                            let a53 = 1u64;
                                            nat_add(a52, a53)?
                                        };
                                        let a55 = 1u64;
                                        nat_mul(a54, a55)?
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
            })?
        }
    };
    nat_add(a68, a69)
}

pub fn f3(v0: u64) -> R<u64> {
    f7(Fn0::F8, v0, 1u64)
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a70 = v0;
        let a71 = v1;
        let a72 = 0u64;
        nat_quot(a70, a71, a72)
    };
    let v3: u64 = {
        let a76 = {
            let a73 = v0;
            let a74 = v1;
            let a75 = 0u64;
            nat_rem(a73, a74, a75)
        };
        let a77 = 2u64;
        nat_mul(a76, a77)?
    };
    let a87 = v2;
    let a88 = {
        let a78 = v1;
        let a79 = v3;
        if if nat_lt(a78, a79) {
            true
        } else {
            let a80 = v3;
            let a81 = v1;
            if nat_eq(a80, a81) {
                let a85 = {
                    let a82 = v2;
                    let a83 = 2u64;
                    let a84 = 0u64;
                    nat_rem(a82, a83, a84)
                };
                let a86 = 1u64;
                nat_eq(a85, a86)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a87, a88)
}

pub fn f5(v0: u64) -> R<u64> {
    let a113 = v0;
    let a114 = f3({
        let a111 = {
            let a109 = {
                let a89 = 1u64;
                let a90 = 1u64;
                nat_add(a89, a90)?
            };
            let a110 = {
                let a107 = {
                    let a105 = {
                        let a91 = 1u64;
                        let a92 = 1u64;
                        nat_add(a91, a92)?
                    };
                    let a106 = {
                        let a103 = {
                            let a101 = {
                                let a93 = 1u64;
                                let a94 = 1u64;
                                nat_add(a93, a94)?
                            };
                            let a102 = {
                                let a99 = {
                                    let a97 = {
                                        let a95 = 1u64;
                                        let a96 = 1u64;
                                        nat_add(a95, a96)?
                                    };
                                    let a98 = 1u64;
                                    nat_mul(a97, a98)?
                                };
                                let a100 = 1u64;
                                nat_add(a99, a100)?
                            };
                            nat_mul(a101, a102)?
                        };
                        let a104 = 1u64;
                        nat_add(a103, a104)?
                    };
                    nat_mul(a105, a106)?
                };
                let a108 = 1u64;
                nat_add(a107, a108)?
            };
            nat_mul(a109, a110)?
        };
        let a112 = 1u64;
        nat_add(a111, a112)?
    })?;
    let a115 = 0u64;
    Ok(nat_rem(a113, a114, a115))
}

pub fn f6(v0: u64) -> R<u64> {
    let a138 = v0;
    let a139 = f3({
        let a136 = {
            let a134 = {
                let a116 = 1u64;
                let a117 = 1u64;
                nat_add(a116, a117)?
            };
            let a135 = {
                let a132 = {
                    let a130 = {
                        let a118 = 1u64;
                        let a119 = 1u64;
                        nat_add(a118, a119)?
                    };
                    let a131 = {
                        let a128 = {
                            let a126 = {
                                let a120 = 1u64;
                                let a121 = 1u64;
                                nat_add(a120, a121)?
                            };
                            let a127 = {
                                let a124 = {
                                    let a122 = 1u64;
                                    let a123 = 1u64;
                                    nat_add(a122, a123)?
                                };
                                let a125 = 1u64;
                                nat_mul(a124, a125)?
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
    })?;
    let a140 = 0u64;
    Ok(nat_rem(a138, a139, a140))
}

pub fn f7(v0: Fn0, p144: u64, p145: u64) -> R<u64> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v1, v2) = m143;
                let m141 = v1;
                if m141 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m141 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c142: Fn0 = v0.clone();
                        c142.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f8(v0: u64) -> R<u64> {
    let a146 = v0;
    let a147 = v0;
    nat_add(a146, a147)
}
