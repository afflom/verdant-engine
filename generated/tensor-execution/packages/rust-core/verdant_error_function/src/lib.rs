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
    C2(u64, u64, u64),
}

pub fn f0(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt0> {
    let v10: u64 = {
        let a1 = v3;
        let a2 = 2147483648u64;
        let a3 = 0u64;
        nat_rem(a1, a2, a3)
    };
    let v11: u64 = {
        let a7 = {
            let a4 = v10;
            let a5 = 8192u64;
            let a6 = 0u64;
            nat_quot(a4, a5, a6)
        };
        let a8 = 8192u64;
        nat_mul(a7, a8)?
    };
    let v12: u64 = {
        let a9 = v11;
        let a10 = 2147483648u64;
        nat_add(a9, a10)?
    };
    let a11 = v2;
    let a12 = 1u64;
    if nat_le(a11, a12) {
        let a13 = v2;
        let a14 = 0u64;
        if if nat_eq(a13, a14) {
            v0
        } else {
            false
        } {
            Ok(Adt0::C1(2u64, v1, 1060439283u64, 0u64, 1u64))
        } else {
            let a15 = 2139095040u64;
            let a16 = v10;
            if nat_lt(a15, a16) {
                Ok(Adt0::C0(2143289344u64))
            } else {
                let a17 = v10;
                let a18 = 830472192u64;
                if nat_lt(a17, a18) {
                    Ok(f1(10u64, v10, v7, v8))
                } else {
                    let a19 = v10;
                    let a20 = 1062731776u64;
                    if nat_lt(a19, a20) {
                        Ok(f2(20u64, v10, v4, v5, v6, v7))
                    } else {
                        let a21 = v10;
                        let a22 = 1067450368u64;
                        if nat_lt(a21, a22) {
                            Ok(f3(50u64, v10, v4, v5, v6, v7, v8))
                        } else {
                            let a23 = v10;
                            let a24 = 1077336941u64;
                            if nat_lt(a23, a24) {
                                Ok(f4(90u64, v10, v4, v5, v6))
                            } else {
                                let a25 = v10;
                                let a26 = 1086324736u64;
                                if nat_lt(a25, a26) {
                                    Ok(f5(140u64, v10, v4, v5, v6))
                                } else {
                                    let v13: u64 = {
                                        let a27 = v3;
                                        let a28 = 2147483648u64;
                                        if nat_lt(a27, a28) {
                                            1065353216u64
                                        } else {
                                            let a29 = 1065353216u64;
                                            let a30 = 2147483648u64;
                                            nat_add(a29, a30)?
                                        }
                                    };
                                    if v0 {
                                        Ok(Adt0::C1(0u64, 1065353216u64, v13, 5u64, 241u64))
                                    } else {
                                        Ok(Adt0::C0(v13))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a31 = v2;
        let a32 = 230u64;
        if nat_eq(a31, a32) {
            let v14: u64 = {
                let a33 = v3;
                let a34 = 2147483648u64;
                if nat_lt(a33, a34) {
                    v9
                } else {
                    let a35 = v9;
                    let a36 = 2147483648u64;
                    nat_add(a35, a36)?
                }
            };
            if v0 {
                Ok(Adt0::C1(0u64, 1065353216u64, v14, 5u64, 241u64))
            } else {
                Ok(Adt0::C0(v14))
            }
        } else {
            let a37 = 10u64;
            let a38 = v2;
            if if nat_le(a37, a38) {
                let a39 = v2;
                let a40 = 14u64;
                nat_lt(a39, a40)
            } else {
                false
            } {
                Ok(f1(v2, v10, v7, v8))
            } else {
                let a41 = 20u64;
                let a42 = v2;
                if if nat_le(a41, a42) {
                    let a43 = v2;
                    let a44 = 42u64;
                    nat_lt(a43, a44)
                } else {
                    false
                } {
                    Ok(f2(v2, v10, v4, v5, v6, v7))
                } else {
                    let a45 = 50u64;
                    let a46 = v2;
                    if if nat_le(a45, a46) {
                        let a47 = v2;
                        let a48 = 79u64;
                        nat_lt(a47, a48)
                    } else {
                        false
                    } {
                        Ok(f3(v2, v10, v4, v5, v6, v7, v8))
                    } else {
                        let a49 = 90u64;
                        let a50 = v2;
                        if if nat_le(a49, a50) {
                            let a51 = v2;
                            let a52 = 123u64;
                            nat_lt(a51, a52)
                        } else {
                            false
                        } {
                            Ok(f4(v2, v10, v4, v5, v6))
                        } else {
                            let a53 = 140u64;
                            let a54 = v2;
                            if if nat_le(a53, a54) {
                                let a55 = v2;
                                let a56 = 169u64;
                                nat_lt(a55, a56)
                            } else {
                                false
                            } {
                                Ok(f5(v2, v10, v4, v5, v6))
                            } else {
                                let a57 = 200u64;
                                let a58 = v2;
                                if if nat_le(a57, a58) {
                                    let a59 = v2;
                                    let a60 = 211u64;
                                    nat_lt(a59, a60)
                                } else {
                                    false
                                } {
                                    Ok(f6(v2, v10, v11, v12, v4, v5, v7, v8))
                                } else {
                                    let a61 = 241u64;
                                    let a62 = v2;
                                    if if nat_le(a61, a62) {
                                        let a63 = v2;
                                        let a64 = 243u64;
                                        nat_lt(a63, a64)
                                    } else {
                                        false
                                    } {
                                        Ok(f7(v2, v1, v7, v8))
                                    } else {
                                        let a65 = v2;
                                        let a66 = 250u64;
                                        if nat_eq(a65, a66) {
                                            Ok(Adt0::C0(v9))
                                        } else {
                                            Ok(Adt0::C0(2143289344u64))
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
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> Adt0 {
    let a67 = v0;
    let a68 = 12u64;
    if nat_lt(a67, a68) {
        let a69 = v0;
        let a70 = 11u64;
        if nat_lt(a69, a70) {
            Adt0::C1(2u64, 1090519040u64, v1, 4u64, 11u64)
        } else {
            Adt0::C1(2u64, 1065579988u64, v1, 5u64, 12u64)
        }
    } else {
        let a71 = v0;
        let a72 = 13u64;
        if nat_lt(a71, a72) {
            Adt0::C1(0u64, v2, v3, 4u64, 13u64)
        } else {
            Adt0::C1(2u64, 1040187392u64, v2, 6u64, 230u64)
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> Adt0 {
    let a73 = v0;
    let a74 = 31u64;
    if nat_lt(a73, a74) {
        let a75 = v0;
        let a76 = 25u64;
        if nat_lt(a75, a76) {
            let a77 = v0;
            let a78 = 22u64;
            if nat_lt(a77, a78) {
                let a79 = v0;
                let a80 = 21u64;
                if nat_lt(a79, a80) {
                    Adt0::C1(2u64, v1, v1, 1u64, 21u64)
                } else {
                    Adt0::C1(2u64, v2, 3083294385u64, 2u64, 22u64)
                }
            } else {
                let a81 = v0;
                let a82 = 23u64;
                if nat_lt(a81, a82) {
                    Adt0::C1(0u64, v3, 3149730953u64, 2u64, 23u64)
                } else {
                    let a83 = v0;
                    let a84 = 24u64;
                    if nat_lt(a83, a84) {
                        Adt0::C1(2u64, v2, v3, 2u64, 24u64)
                    } else {
                        Adt0::C1(0u64, v3, 3169407631u64, 2u64, 25u64)
                    }
                }
            }
        } else {
            let a85 = v0;
            let a86 = 28u64;
            if nat_lt(a85, a86) {
                let a87 = v0;
                let a88 = 26u64;
                if nat_lt(a87, a88) {
                    Adt0::C1(2u64, v2, v3, 2u64, 26u64)
                } else {
                    let a89 = v0;
                    let a90 = 27u64;
                    if nat_lt(a89, a90) {
                        Adt0::C1(0u64, v3, 3198577643u64, 2u64, 27u64)
                    } else {
                        Adt0::C1(2u64, v2, v3, 2u64, 28u64)
                    }
                }
            } else {
                let a91 = v0;
                let a92 = 29u64;
                if nat_lt(a91, a92) {
                    Adt0::C1(0u64, v3, 1040414164u64, 2u64, 29u64)
                } else {
                    let a93 = v0;
                    let a94 = 30u64;
                    if nat_lt(a93, a94) {
                        Adt0::C1(2u64, v2, 3062161946u64, 3u64, 30u64)
                    } else {
                        Adt0::C1(0u64, v4, 957017673u64, 3u64, 31u64)
                    }
                }
            }
        }
    } else {
        let a95 = v0;
        let a96 = 36u64;
        if nat_lt(a95, a96) {
            let a97 = v0;
            let a98 = 33u64;
            if nat_lt(a97, a98) {
                let a99 = v0;
                let a100 = 32u64;
                if nat_lt(a99, a100) {
                    Adt0::C1(2u64, v2, v4, 3u64, 32u64)
                } else {
                    Adt0::C1(0u64, v4, 1000767766u64, 3u64, 33u64)
                }
            } else {
                let a101 = v0;
                let a102 = 34u64;
                if nat_lt(a101, a102) {
                    Adt0::C1(2u64, v2, v4, 3u64, 34u64)
                } else {
                    let a103 = v0;
                    let a104 = 35u64;
                    if nat_lt(a103, a104) {
                        Adt0::C1(0u64, v4, 1032137315u64, 3u64, 35u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 36u64)
                    }
                }
            }
        } else {
            let a105 = v0;
            let a106 = 39u64;
            if nat_lt(a105, a106) {
                let a107 = v0;
                let a108 = 37u64;
                if nat_lt(a107, a108) {
                    Adt0::C1(0u64, v4, 1053539278u64, 3u64, 37u64)
                } else {
                    let a109 = v0;
                    let a110 = 38u64;
                    if nat_lt(a109, a110) {
                        Adt0::C1(2u64, v2, v4, 3u64, 38u64)
                    } else {
                        Adt0::C1(0u64, v4, 1065353216u64, 3u64, 39u64)
                    }
                }
            } else {
                let a111 = v0;
                let a112 = 40u64;
                if nat_lt(a111, a112) {
                    Adt0::C1(3u64, v3, v4, 4u64, 40u64)
                } else {
                    let a113 = v0;
                    let a114 = 41u64;
                    if nat_lt(a113, a114) {
                        Adt0::C1(2u64, v1, v5, 4u64, 41u64)
                    } else {
                        Adt0::C1(0u64, v1, v5, 6u64, 230u64)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt0 {
    let a115 = v0;
    let a116 = 64u64;
    if nat_lt(a115, a116) {
        let a117 = v0;
        let a118 = 57u64;
        if nat_lt(a117, a118) {
            let a119 = v0;
            let a120 = 53u64;
            if nat_lt(a119, a120) {
                let a121 = v0;
                let a122 = 51u64;
                if nat_lt(a121, a122) {
                    Adt0::C1(1u64, v1, 1065353216u64, 1u64, 51u64)
                } else {
                    let a123 = v0;
                    let a124 = 52u64;
                    if nat_lt(a123, a124) {
                        Adt0::C1(2u64, v2, 3138255296u64, 2u64, 52u64)
                    } else {
                        Adt0::C1(0u64, v3, 1024545203u64, 2u64, 53u64)
                    }
                }
            } else {
                let a125 = v0;
                let a126 = 55u64;
                if nat_lt(a125, a126) {
                    let a127 = v0;
                    let a128 = 54u64;
                    if nat_lt(a127, a128) {
                        Adt0::C1(2u64, v2, v3, 2u64, 54u64)
                    } else {
                        Adt0::C1(0u64, v3, 3185777858u64, 2u64, 55u64)
                    }
                } else {
                    let a129 = v0;
                    let a130 = 56u64;
                    if nat_lt(a129, a130) {
                        Adt0::C1(2u64, v2, v3, 2u64, 56u64)
                    } else {
                        Adt0::C1(0u64, v3, 1050869332u64, 2u64, 57u64)
                    }
                }
            }
        } else {
            let a131 = v0;
            let a132 = 60u64;
            if nat_lt(a131, a132) {
                let a133 = v0;
                let a134 = 58u64;
                if nat_lt(a133, a134) {
                    Adt0::C1(2u64, v2, v3, 2u64, 58u64)
                } else {
                    let a135 = v0;
                    let a136 = 59u64;
                    if nat_lt(a135, a136) {
                        Adt0::C1(0u64, v3, 3200160264u64, 2u64, 59u64)
                    } else {
                        Adt0::C1(2u64, v2, v3, 2u64, 60u64)
                    }
                }
            } else {
                let a137 = v0;
                let a138 = 62u64;
                if nat_lt(a137, a138) {
                    let a139 = v0;
                    let a140 = 61u64;
                    if nat_lt(a139, a140) {
                        Adt0::C1(0u64, v3, 1054107653u64, 2u64, 61u64)
                    } else {
                        Adt0::C1(2u64, v2, v3, 2u64, 62u64)
                    }
                } else {
                    let a141 = v0;
                    let a142 = 63u64;
                    if nat_lt(a141, a142) {
                        Adt0::C1(0u64, v3, 3139096006u64, 2u64, 63u64)
                    } else {
                        Adt0::C1(2u64, v2, 1011112611u64, 3u64, 64u64)
                    }
                }
            }
        }
    } else {
        let a143 = v0;
        let a144 = 71u64;
        if nat_lt(a143, a144) {
            let a145 = v0;
            let a146 = 67u64;
            if nat_lt(a145, a146) {
                let a147 = v0;
                let a148 = 65u64;
                if nat_lt(a147, a148) {
                    Adt0::C1(0u64, v4, 1012887059u64, 3u64, 65u64)
                } else {
                    let a149 = v0;
                    let a150 = 66u64;
                    if nat_lt(a149, a150) {
                        Adt0::C1(2u64, v2, v4, 3u64, 66u64)
                    } else {
                        Adt0::C1(0u64, v4, 1040265991u64, 3u64, 67u64)
                    }
                }
            } else {
                let a151 = v0;
                let a152 = 69u64;
                if nat_lt(a151, a152) {
                    let a153 = v0;
                    let a154 = 68u64;
                    if nat_lt(a153, a154) {
                        Adt0::C1(2u64, v2, v4, 3u64, 68u64)
                    } else {
                        Adt0::C1(0u64, v4, 1033050855u64, 3u64, 69u64)
                    }
                } else {
                    let a155 = v0;
                    let a156 = 70u64;
                    if nat_lt(a155, a156) {
                        Adt0::C1(2u64, v2, v4, 3u64, 70u64)
                    } else {
                        Adt0::C1(0u64, v4, 1057642373u64, 3u64, 71u64)
                    }
                }
            }
        } else {
            let a157 = v0;
            let a158 = 75u64;
            if nat_lt(a157, a158) {
                let a159 = v0;
                let a160 = 73u64;
                if nat_lt(a159, a160) {
                    let a161 = v0;
                    let a162 = 72u64;
                    if nat_lt(a161, a162) {
                        Adt0::C1(2u64, v2, v4, 3u64, 72u64)
                    } else {
                        Adt0::C1(0u64, v4, 1037693745u64, 3u64, 73u64)
                    }
                } else {
                    let a163 = v0;
                    let a164 = 74u64;
                    if nat_lt(a163, a164) {
                        Adt0::C1(2u64, v2, v4, 3u64, 74u64)
                    } else {
                        Adt0::C1(0u64, v4, 1065353216u64, 3u64, 75u64)
                    }
                }
            } else {
                let a165 = v0;
                let a166 = 77u64;
                if nat_lt(a165, a166) {
                    let a167 = v0;
                    let a168 = 76u64;
                    if nat_lt(a167, a168) {
                        Adt0::C1(3u64, v3, v4, 4u64, 76u64)
                    } else {
                        Adt0::C1(1u64, 1065353216u64, 1062753803u64, 5u64, 77u64)
                    }
                } else {
                    let a169 = v0;
                    let a170 = 78u64;
                    if nat_lt(a169, a170) {
                        Adt0::C1(1u64, v6, v5, 5u64, 78u64)
                    } else {
                        Adt0::C1(1u64, 1065353216u64, v6, 6u64, 230u64)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt0 {
    let a171 = v0;
    let a172 = 106u64;
    if nat_lt(a171, a172) {
        let a173 = v0;
        let a174 = 98u64;
        if nat_lt(a173, a174) {
            let a175 = v0;
            let a176 = 94u64;
            if nat_lt(a175, a176) {
                let a177 = v0;
                let a178 = 92u64;
                if nat_lt(a177, a178) {
                    let a179 = v0;
                    let a180 = 91u64;
                    if nat_lt(a179, a180) {
                        Adt0::C1(2u64, v1, v1, 1u64, 91u64)
                    } else {
                        Adt0::C1(3u64, 1065353216u64, v2, 1u64, 92u64)
                    }
                } else {
                    let a181 = v0;
                    let a182 = 93u64;
                    if nat_lt(a181, a182) {
                        Adt0::C1(2u64, v2, 3239905150u64, 2u64, 93u64)
                    } else {
                        Adt0::C1(0u64, v3, 3265434411u64, 2u64, 94u64)
                    }
                }
            } else {
                let a183 = v0;
                let a184 = 96u64;
                if nat_lt(a183, a184) {
                    let a185 = v0;
                    let a186 = 95u64;
                    if nat_lt(a185, a186) {
                        Adt0::C1(2u64, v2, v3, 2u64, 95u64)
                    } else {
                        Adt0::C1(0u64, v3, 3275266791u64, 2u64, 96u64)
                    }
                } else {
                    let a187 = v0;
                    let a188 = 97u64;
                    if nat_lt(a187, a188) {
                        Adt0::C1(2u64, v2, v3, 2u64, 97u64)
                    } else {
                        Adt0::C1(0u64, v3, 3273811340u64, 2u64, 98u64)
                    }
                }
            }
        } else {
            let a189 = v0;
            let a190 = 102u64;
            if nat_lt(a189, a190) {
                let a191 = v0;
                let a192 = 100u64;
                if nat_lt(a191, a192) {
                    let a193 = v0;
                    let a194 = 99u64;
                    if nat_lt(a193, a194) {
                        Adt0::C1(2u64, v2, v3, 2u64, 99u64)
                    } else {
                        Adt0::C1(0u64, v3, 3262742615u64, 2u64, 100u64)
                    }
                } else {
                    let a195 = v0;
                    let a196 = 101u64;
                    if nat_lt(a195, a196) {
                        Adt0::C1(2u64, v2, v3, 2u64, 101u64)
                    } else {
                        Adt0::C1(0u64, v3, 3240685602u64, 2u64, 102u64)
                    }
                }
            } else {
                let a197 = v0;
                let a198 = 104u64;
                if nat_lt(a197, a198) {
                    let a199 = v0;
                    let a200 = 103u64;
                    if nat_lt(a199, a200) {
                        Adt0::C1(2u64, v2, v3, 2u64, 103u64)
                    } else {
                        Adt0::C1(0u64, v3, 3207700663u64, 2u64, 104u64)
                    }
                } else {
                    let a201 = v0;
                    let a202 = 105u64;
                    if nat_lt(a201, a202) {
                        Adt0::C1(2u64, v2, v3, 2u64, 105u64)
                    } else {
                        Adt0::C1(0u64, v3, 3156320403u64, 2u64, 106u64)
                    }
                }
            }
        }
    } else {
        let a203 = v0;
        let a204 = 114u64;
        if nat_lt(a203, a204) {
            let a205 = v0;
            let a206 = 110u64;
            if nat_lt(a205, a206) {
                let a207 = v0;
                let a208 = 108u64;
                if nat_lt(a207, a208) {
                    let a209 = v0;
                    let a210 = 107u64;
                    if nat_lt(a209, a210) {
                        Adt0::C1(2u64, v2, 3178725271u64, 3u64, 107u64)
                    } else {
                        Adt0::C1(0u64, v4, 1087520636u64, 3u64, 108u64)
                    }
                } else {
                    let a211 = v0;
                    let a212 = 109u64;
                    if nat_lt(a211, a212) {
                        Adt0::C1(2u64, v2, v4, 3u64, 109u64)
                    } else {
                        Adt0::C1(0u64, v4, 1121535263u64, 3u64, 110u64)
                    }
                }
            } else {
                let a213 = v0;
                let a214 = 112u64;
                if nat_lt(a213, a214) {
                    let a215 = v0;
                    let a216 = 111u64;
                    if nat_lt(a215, a216) {
                        Adt0::C1(2u64, v2, v4, 3u64, 111u64)
                    } else {
                        Adt0::C1(0u64, v4, 1138131211u64, 3u64, 112u64)
                    }
                } else {
                    let a217 = v0;
                    let a218 = 113u64;
                    if nat_lt(a217, a218) {
                        Adt0::C1(2u64, v2, v4, 3u64, 113u64)
                    } else {
                        Adt0::C1(0u64, v4, 1143036105u64, 3u64, 114u64)
                    }
                }
            }
        } else {
            let a219 = v0;
            let a220 = 118u64;
            if nat_lt(a219, a220) {
                let a221 = v0;
                let a222 = 116u64;
                if nat_lt(a221, a222) {
                    let a223 = v0;
                    let a224 = 115u64;
                    if nat_lt(a223, a224) {
                        Adt0::C1(2u64, v2, v4, 3u64, 115u64)
                    } else {
                        Adt0::C1(0u64, v4, 1138313327u64, 3u64, 116u64)
                    }
                } else {
                    let a225 = v0;
                    let a226 = 117u64;
                    if nat_lt(a225, a226) {
                        Adt0::C1(2u64, v2, v4, 3u64, 117u64)
                    } else {
                        Adt0::C1(0u64, v4, 1124706403u64, 3u64, 118u64)
                    }
                }
            } else {
                let a227 = v0;
                let a228 = 120u64;
                if nat_lt(a227, a228) {
                    let a229 = v0;
                    let a230 = 119u64;
                    if nat_lt(a229, a230) {
                        Adt0::C1(2u64, v2, v4, 3u64, 119u64)
                    } else {
                        Adt0::C1(0u64, v4, 1100821966u64, 3u64, 120u64)
                    }
                } else {
                    let a231 = v0;
                    let a232 = 121u64;
                    if nat_lt(a231, a232) {
                        Adt0::C1(2u64, v2, v4, 3u64, 121u64)
                    } else {
                        let a233 = v0;
                        let a234 = 122u64;
                        if nat_lt(a233, a234) {
                            Adt0::C1(0u64, v4, 1065353216u64, 3u64, 122u64)
                        } else {
                            Adt0::C1(3u64, v3, v4, 4u64, 200u64)
                        }
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt0 {
    let a235 = v0;
    let a236 = 154u64;
    if nat_lt(a235, a236) {
        let a237 = v0;
        let a238 = 147u64;
        if nat_lt(a237, a238) {
            let a239 = v0;
            let a240 = 143u64;
            if nat_lt(a239, a240) {
                let a241 = v0;
                let a242 = 141u64;
                if nat_lt(a241, a242) {
                    Adt0::C1(2u64, v1, v1, 1u64, 141u64)
                } else {
                    let a243 = v0;
                    let a244 = 142u64;
                    if nat_lt(a243, a244) {
                        Adt0::C1(3u64, 1065353216u64, v2, 1u64, 142u64)
                    } else {
                        Adt0::C1(2u64, v2, 3287401077u64, 2u64, 143u64)
                    }
                }
            } else {
                let a245 = v0;
                let a246 = 145u64;
                if nat_lt(a245, a246) {
                    let a247 = v0;
                    let a248 = 144u64;
                    if nat_lt(a247, a248) {
                        Adt0::C1(0u64, v3, 3296731915u64, 2u64, 144u64)
                    } else {
                        Adt0::C1(2u64, v2, v3, 2u64, 145u64)
                    }
                } else {
                    let a249 = v0;
                    let a250 = 146u64;
                    if nat_lt(a249, a250) {
                        Adt0::C1(0u64, v3, 3290391617u64, 2u64, 146u64)
                    } else {
                        Adt0::C1(2u64, v2, v3, 2u64, 147u64)
                    }
                }
            }
        } else {
            let a251 = v0;
            let a252 = 150u64;
            if nat_lt(a251, a252) {
                let a253 = v0;
                let a254 = 148u64;
                if nat_lt(a253, a254) {
                    Adt0::C1(0u64, v3, 3273695978u64, 2u64, 148u64)
                } else {
                    let a255 = v0;
                    let a256 = 149u64;
                    if nat_lt(a255, a256) {
                        Adt0::C1(2u64, v2, v3, 2u64, 149u64)
                    } else {
                        Adt0::C1(0u64, v3, 3247312971u64, 2u64, 150u64)
                    }
                }
            } else {
                let a257 = v0;
                let a258 = 152u64;
                if nat_lt(a257, a258) {
                    let a259 = v0;
                    let a260 = 151u64;
                    if nat_lt(a259, a260) {
                        Adt0::C1(2u64, v2, v3, 2u64, 151u64)
                    } else {
                        Adt0::C1(0u64, v3, 3209469396u64, 2u64, 152u64)
                    }
                } else {
                    let a261 = v0;
                    let a262 = 153u64;
                    if nat_lt(a261, a262) {
                        Adt0::C1(2u64, v2, v3, 2u64, 153u64)
                    } else {
                        Adt0::C1(0u64, v3, 3156320402u64, 2u64, 154u64)
                    }
                }
            }
        }
    } else {
        let a263 = v0;
        let a264 = 161u64;
        if nat_lt(a263, a264) {
            let a265 = v0;
            let a266 = 157u64;
            if nat_lt(a265, a266) {
                let a267 = v0;
                let a268 = 155u64;
                if nat_lt(a267, a268) {
                    Adt0::C1(2u64, v2, 3249768210u64, 3u64, 155u64)
                } else {
                    let a269 = v0;
                    let a270 = 156u64;
                    if nat_lt(a269, a270) {
                        Adt0::C1(0u64, v4, 1139622823u64, 3u64, 156u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 157u64)
                    }
                }
            } else {
                let a271 = v0;
                let a272 = 159u64;
                if nat_lt(a271, a272) {
                    let a273 = v0;
                    let a274 = 158u64;
                    if nat_lt(a273, a274) {
                        Adt0::C1(0u64, v4, 1159696590u64, 3u64, 158u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 159u64)
                    }
                } else {
                    let a275 = v0;
                    let a276 = 160u64;
                    if nat_lt(a275, a276) {
                        Adt0::C1(0u64, v4, 1162345915u64, 3u64, 160u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 161u64)
                    }
                }
            }
        } else {
            let a277 = v0;
            let a278 = 165u64;
            if nat_lt(a277, a278) {
                let a279 = v0;
                let a280 = 163u64;
                if nat_lt(a279, a280) {
                    let a281 = v0;
                    let a282 = 162u64;
                    if nat_lt(a281, a282) {
                        Adt0::C1(0u64, v4, 1153439577u64, 3u64, 162u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 163u64)
                    }
                } else {
                    let a283 = v0;
                    let a284 = 164u64;
                    if nat_lt(a283, a284) {
                        Adt0::C1(0u64, v4, 1134749041u64, 3u64, 164u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 165u64)
                    }
                }
            } else {
                let a285 = v0;
                let a286 = 167u64;
                if nat_lt(a285, a286) {
                    let a287 = v0;
                    let a288 = 166u64;
                    if nat_lt(a287, a288) {
                        Adt0::C1(0u64, v4, 1106424921u64, 3u64, 166u64)
                    } else {
                        Adt0::C1(2u64, v2, v4, 3u64, 167u64)
                    }
                } else {
                    let a289 = v0;
                    let a290 = 168u64;
                    if nat_lt(a289, a290) {
                        Adt0::C1(0u64, v4, 1065353216u64, 3u64, 168u64)
                    } else {
                        Adt0::C1(3u64, v3, v4, 4u64, 200u64)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> Adt0 {
    let a291 = v0;
    let a292 = 205u64;
    if nat_lt(a291, a292) {
        let a293 = v0;
        let a294 = 202u64;
        if nat_lt(a293, a294) {
            let a295 = v0;
            let a296 = 201u64;
            if nat_lt(a295, a296) {
                Adt0::C1(2u64, v3, v2, 5u64, 201u64)
            } else {
                Adt0::C1(1u64, v7, 1058013184u64, 5u64, 202u64)
            }
        } else {
            let a297 = v0;
            let a298 = 203u64;
            if nat_lt(a297, a298) {
                Adt0::C2(v7, 5u64, 203u64)
            } else {
                let a299 = v0;
                let a300 = 204u64;
                if nat_lt(a299, a300) {
                    Adt0::C1(1u64, v2, v1, 1u64, 204u64)
                } else {
                    Adt0::C1(0u64, v2, v1, 2u64, 205u64)
                }
            }
        }
    } else {
        let a301 = v0;
        let a302 = 208u64;
        if nat_lt(a301, a302) {
            let a303 = v0;
            let a304 = 206u64;
            if nat_lt(a303, a304) {
                Adt0::C1(2u64, v4, v5, 1u64, 206u64)
            } else {
                let a305 = v0;
                let a306 = 207u64;
                if nat_lt(a305, a306) {
                    Adt0::C1(0u64, v4, v6, 1u64, 207u64)
                } else {
                    Adt0::C2(v4, 1u64, 208u64)
                }
            }
        } else {
            let a307 = v0;
            let a308 = 209u64;
            if nat_lt(a307, a308) {
                Adt0::C1(2u64, v7, v4, 5u64, 209u64)
            } else {
                let a309 = v0;
                let a310 = 210u64;
                if nat_lt(a309, a310) {
                    Adt0::C1(3u64, v7, v1, 5u64, 210u64)
                } else {
                    Adt0::C1(1u64, 1065353216u64, v7, 6u64, 230u64)
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64) -> Adt0 {
    let a311 = v0;
    let a312 = 242u64;
    if nat_lt(a311, a312) {
        Adt0::C1(2u64, 1056964608u64, v1, 4u64, 242u64)
    } else {
        Adt0::C1(2u64, v2, v3, 6u64, 250u64)
    }
}

pub fn run(p0: bool, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
}
