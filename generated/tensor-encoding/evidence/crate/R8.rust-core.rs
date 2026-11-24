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

pub fn f0(v0: u64, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1 = v0;
    let a2 = 6u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 3u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 1u64;
            if nat_lt(a5, a6) {
                let a7 = v0;
                let a8 = 0u64;
                if nat_eq(a7, a8) {
                    f1(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a9 = v0;
                let a10 = 2u64;
                if nat_lt(a9, a10) {
                    let a11 = v0;
                    let a12 = 1u64;
                    if nat_eq(a11, a12) {
                        f3(v1)
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a13 = v0;
                    let a14 = 2u64;
                    if nat_eq(a13, a14) {
                        f4(v1)
                    } else {
                        Ok(f2())
                    }
                }
            }
        } else {
            let a15 = v0;
            let a16 = 4u64;
            if nat_lt(a15, a16) {
                let a17 = v0;
                let a18 = 3u64;
                if nat_eq(a17, a18) {
                    f5(v1)
                } else {
                    Ok(f2())
                }
            } else {
                let a19 = v0;
                let a20 = 5u64;
                if nat_lt(a19, a20) {
                    let a21 = v0;
                    let a22 = 4u64;
                    if nat_eq(a21, a22) {
                        f6(v1)
                    } else {
                        Ok(f2())
                    }
                } else {
                    let a23 = v0;
                    let a24 = 5u64;
                    if nat_eq(a23, a24) {
                        f7(v1)
                    } else {
                        Ok(f2())
                    }
                }
            }
        }
    } else {
        let a25 = v0;
        let a26 = 7u64;
        if nat_lt(a25, a26) {
            let a27 = v0;
            let a28 = 6u64;
            if nat_eq(a27, a28) {
                f8(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a29 = v0;
            let a30 = 7u64;
            if nat_eq(a29, a30) {
                f9(v1)
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f1(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a31 = v0;
        let a32 = 8u64;
        let a33 = 0u64;
        nat_quot(a31, a32, a33)
    };
    let a34 = v1;
    let a35 = 1u64;
    if nat_lt(a34, a35) {
        let a36 = v1;
        let a37 = 0u64;
        if nat_eq(a36, a37) {
            f10({
                let a38 = v0;
                let a39 = 8u64;
                let a40 = 0u64;
                nat_rem(a38, a39, a40)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a41 = v1;
        let a42 = 1u64;
        if nat_eq(a41, a42) {
            Ok(f11({
                let a43 = v0;
                let a44 = 8u64;
                let a45 = 0u64;
                nat_rem(a43, a44, a45)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a46 = v0;
        let a47 = 8u64;
        let a48 = 0u64;
        nat_quot(a46, a47, a48)
    };
    let a49 = v1;
    let a50 = 4u64;
    if nat_lt(a49, a50) {
        let a51 = v1;
        let a52 = 2u64;
        if nat_lt(a51, a52) {
            let a53 = v1;
            let a54 = 1u64;
            if nat_lt(a53, a54) {
                let a55 = v1;
                let a56 = 0u64;
                if nat_eq(a55, a56) {
                    Ok(f12({
                        let a57 = v0;
                        let a58 = 8u64;
                        let a59 = 0u64;
                        nat_rem(a57, a58, a59)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a60 = v1;
                let a61 = 1u64;
                if nat_eq(a60, a61) {
                    f13({
                        let a62 = v0;
                        let a63 = 8u64;
                        let a64 = 0u64;
                        nat_rem(a62, a63, a64)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a65 = v1;
            let a66 = 3u64;
            if nat_lt(a65, a66) {
                let a67 = v1;
                let a68 = 2u64;
                if nat_eq(a67, a68) {
                    f14({
                        let a69 = v0;
                        let a70 = 8u64;
                        let a71 = 0u64;
                        nat_rem(a69, a70, a71)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a72 = v1;
                let a73 = 3u64;
                if nat_eq(a72, a73) {
                    Ok(f15({
                        let a74 = v0;
                        let a75 = 8u64;
                        let a76 = 0u64;
                        nat_rem(a74, a75, a76)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a77 = v1;
        let a78 = 6u64;
        if nat_lt(a77, a78) {
            let a79 = v1;
            let a80 = 5u64;
            if nat_lt(a79, a80) {
                let a81 = v1;
                let a82 = 4u64;
                if nat_eq(a81, a82) {
                    Ok(f16({
                        let a83 = v0;
                        let a84 = 8u64;
                        let a85 = 0u64;
                        nat_rem(a83, a84, a85)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a86 = v1;
                let a87 = 5u64;
                if nat_eq(a86, a87) {
                    f17({
                        let a88 = v0;
                        let a89 = 8u64;
                        let a90 = 0u64;
                        nat_rem(a88, a89, a90)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a91 = v1;
            let a92 = 7u64;
            if nat_lt(a91, a92) {
                let a93 = v1;
                let a94 = 6u64;
                if nat_eq(a93, a94) {
                    f18({
                        let a95 = v0;
                        let a96 = 8u64;
                        let a97 = 0u64;
                        nat_rem(a95, a96, a97)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a98 = v1;
                let a99 = 7u64;
                if nat_eq(a98, a99) {
                    Ok(f19({
                        let a100 = v0;
                        let a101 = 8u64;
                        let a102 = 0u64;
                        nat_rem(a100, a101, a102)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a103 = v0;
        let a104 = 8u64;
        let a105 = 0u64;
        nat_quot(a103, a104, a105)
    };
    let a106 = v1;
    let a107 = 1u64;
    if nat_lt(a106, a107) {
        let a108 = v1;
        let a109 = 0u64;
        if nat_eq(a108, a109) {
            f20({
                let a110 = v0;
                let a111 = 8u64;
                let a112 = 0u64;
                nat_rem(a110, a111, a112)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a113 = v1;
        let a114 = 1u64;
        if nat_eq(a113, a114) {
            Ok(f21({
                let a115 = v0;
                let a116 = 8u64;
                let a117 = 0u64;
                nat_rem(a115, a116, a117)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a118 = v0;
        let a119 = 8u64;
        let a120 = 0u64;
        nat_quot(a118, a119, a120)
    };
    let a121 = v1;
    let a122 = 4u64;
    if nat_lt(a121, a122) {
        let a123 = v1;
        let a124 = 2u64;
        if nat_lt(a123, a124) {
            let a125 = v1;
            let a126 = 1u64;
            if nat_lt(a125, a126) {
                let a127 = v1;
                let a128 = 0u64;
                if nat_eq(a127, a128) {
                    Ok(f22({
                        let a129 = v0;
                        let a130 = 8u64;
                        let a131 = 0u64;
                        nat_rem(a129, a130, a131)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a132 = v1;
                let a133 = 1u64;
                if nat_eq(a132, a133) {
                    f23({
                        let a134 = v0;
                        let a135 = 8u64;
                        let a136 = 0u64;
                        nat_rem(a134, a135, a136)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a137 = v1;
            let a138 = 3u64;
            if nat_lt(a137, a138) {
                let a139 = v1;
                let a140 = 2u64;
                if nat_eq(a139, a140) {
                    f24({
                        let a141 = v0;
                        let a142 = 8u64;
                        let a143 = 0u64;
                        nat_rem(a141, a142, a143)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a144 = v1;
                let a145 = 3u64;
                if nat_eq(a144, a145) {
                    Ok(f25({
                        let a146 = v0;
                        let a147 = 8u64;
                        let a148 = 0u64;
                        nat_rem(a146, a147, a148)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a149 = v1;
        let a150 = 6u64;
        if nat_lt(a149, a150) {
            let a151 = v1;
            let a152 = 5u64;
            if nat_lt(a151, a152) {
                let a153 = v1;
                let a154 = 4u64;
                if nat_eq(a153, a154) {
                    Ok(f26({
                        let a155 = v0;
                        let a156 = 8u64;
                        let a157 = 0u64;
                        nat_rem(a155, a156, a157)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a158 = v1;
                let a159 = 5u64;
                if nat_eq(a158, a159) {
                    Ok(f27({
                        let a160 = v0;
                        let a161 = 8u64;
                        let a162 = 0u64;
                        nat_rem(a160, a161, a162)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a163 = v1;
            let a164 = 7u64;
            if nat_lt(a163, a164) {
                let a165 = v1;
                let a166 = 6u64;
                if nat_eq(a165, a166) {
                    f28({
                        let a167 = v0;
                        let a168 = 8u64;
                        let a169 = 0u64;
                        nat_rem(a167, a168, a169)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a170 = v1;
                let a171 = 7u64;
                if nat_eq(a170, a171) {
                    f29({
                        let a172 = v0;
                        let a173 = 8u64;
                        let a174 = 0u64;
                        nat_rem(a172, a173, a174)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a175 = v0;
        let a176 = 8u64;
        let a177 = 0u64;
        nat_quot(a175, a176, a177)
    };
    let a178 = v1;
    let a179 = 1u64;
    if nat_lt(a178, a179) {
        let a180 = v1;
        let a181 = 0u64;
        if nat_eq(a180, a181) {
            f20({
                let a182 = v0;
                let a183 = 8u64;
                let a184 = 0u64;
                nat_rem(a182, a183, a184)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a185 = v1;
        let a186 = 1u64;
        if nat_eq(a185, a186) {
            Ok(f21({
                let a187 = v0;
                let a188 = 8u64;
                let a189 = 0u64;
                nat_rem(a187, a188, a189)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a190 = v0;
        let a191 = 8u64;
        let a192 = 0u64;
        nat_quot(a190, a191, a192)
    };
    let a193 = v1;
    let a194 = 4u64;
    if nat_lt(a193, a194) {
        let a195 = v1;
        let a196 = 2u64;
        if nat_lt(a195, a196) {
            let a197 = v1;
            let a198 = 1u64;
            if nat_lt(a197, a198) {
                let a199 = v1;
                let a200 = 0u64;
                if nat_eq(a199, a200) {
                    Ok(f12({
                        let a201 = v0;
                        let a202 = 8u64;
                        let a203 = 0u64;
                        nat_rem(a201, a202, a203)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a204 = v1;
                let a205 = 1u64;
                if nat_eq(a204, a205) {
                    f23({
                        let a206 = v0;
                        let a207 = 8u64;
                        let a208 = 0u64;
                        nat_rem(a206, a207, a208)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a209 = v1;
            let a210 = 3u64;
            if nat_lt(a209, a210) {
                let a211 = v1;
                let a212 = 2u64;
                if nat_eq(a211, a212) {
                    f24({
                        let a213 = v0;
                        let a214 = 8u64;
                        let a215 = 0u64;
                        nat_rem(a213, a214, a215)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a216 = v1;
                let a217 = 3u64;
                if nat_eq(a216, a217) {
                    Ok(f25({
                        let a218 = v0;
                        let a219 = 8u64;
                        let a220 = 0u64;
                        nat_rem(a218, a219, a220)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a221 = v1;
        let a222 = 6u64;
        if nat_lt(a221, a222) {
            let a223 = v1;
            let a224 = 5u64;
            if nat_lt(a223, a224) {
                let a225 = v1;
                let a226 = 4u64;
                if nat_eq(a225, a226) {
                    Ok(f26({
                        let a227 = v0;
                        let a228 = 8u64;
                        let a229 = 0u64;
                        nat_rem(a227, a228, a229)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a230 = v1;
                let a231 = 5u64;
                if nat_eq(a230, a231) {
                    Ok(f27({
                        let a232 = v0;
                        let a233 = 8u64;
                        let a234 = 0u64;
                        nat_rem(a232, a233, a234)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a235 = v1;
            let a236 = 7u64;
            if nat_lt(a235, a236) {
                let a237 = v1;
                let a238 = 6u64;
                if nat_eq(a237, a238) {
                    f30({
                        let a239 = v0;
                        let a240 = 8u64;
                        let a241 = 0u64;
                        nat_rem(a239, a240, a241)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a242 = v1;
                let a243 = 7u64;
                if nat_eq(a242, a243) {
                    f31({
                        let a244 = v0;
                        let a245 = 8u64;
                        let a246 = 0u64;
                        nat_rem(a244, a245, a246)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a247 = v0;
        let a248 = 8u64;
        let a249 = 0u64;
        nat_quot(a247, a248, a249)
    };
    let a250 = v1;
    let a251 = 1u64;
    if nat_lt(a250, a251) {
        let a252 = v1;
        let a253 = 0u64;
        if nat_eq(a252, a253) {
            f32({
                let a254 = v0;
                let a255 = 8u64;
                let a256 = 0u64;
                nat_rem(a254, a255, a256)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a257 = v1;
        let a258 = 1u64;
        if nat_eq(a257, a258) {
            Ok(f33({
                let a259 = v0;
                let a260 = 8u64;
                let a261 = 0u64;
                nat_rem(a259, a260, a261)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a262 = v0;
        let a263 = 8u64;
        let a264 = 0u64;
        nat_quot(a262, a263, a264)
    };
    let a265 = v1;
    let a266 = 4u64;
    if nat_lt(a265, a266) {
        let a267 = v1;
        let a268 = 2u64;
        if nat_lt(a267, a268) {
            let a269 = v1;
            let a270 = 1u64;
            if nat_lt(a269, a270) {
                let a271 = v1;
                let a272 = 0u64;
                if nat_eq(a271, a272) {
                    f34({
                        let a273 = v0;
                        let a274 = 8u64;
                        let a275 = 0u64;
                        nat_rem(a273, a274, a275)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a276 = v1;
                let a277 = 1u64;
                if nat_eq(a276, a277) {
                    f35({
                        let a278 = v0;
                        let a279 = 8u64;
                        let a280 = 0u64;
                        nat_rem(a278, a279, a280)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a281 = v1;
            let a282 = 3u64;
            if nat_lt(a281, a282) {
                let a283 = v1;
                let a284 = 2u64;
                if nat_eq(a283, a284) {
                    f36({
                        let a285 = v0;
                        let a286 = 8u64;
                        let a287 = 0u64;
                        nat_rem(a285, a286, a287)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a288 = v1;
                let a289 = 3u64;
                if nat_eq(a288, a289) {
                    f37({
                        let a290 = v0;
                        let a291 = 8u64;
                        let a292 = 0u64;
                        nat_rem(a290, a291, a292)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a293 = v1;
        let a294 = 6u64;
        if nat_lt(a293, a294) {
            let a295 = v1;
            let a296 = 5u64;
            if nat_lt(a295, a296) {
                let a297 = v1;
                let a298 = 4u64;
                if nat_eq(a297, a298) {
                    f38({
                        let a299 = v0;
                        let a300 = 8u64;
                        let a301 = 0u64;
                        nat_rem(a299, a300, a301)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a302 = v1;
                let a303 = 5u64;
                if nat_eq(a302, a303) {
                    f39({
                        let a304 = v0;
                        let a305 = 8u64;
                        let a306 = 0u64;
                        nat_rem(a304, a305, a306)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a307 = v1;
            let a308 = 7u64;
            if nat_lt(a307, a308) {
                let a309 = v1;
                let a310 = 6u64;
                if nat_eq(a309, a310) {
                    f40({
                        let a311 = v0;
                        let a312 = 8u64;
                        let a313 = 0u64;
                        nat_rem(a311, a312, a313)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a314 = v1;
                let a315 = 7u64;
                if nat_eq(a314, a315) {
                    f41({
                        let a316 = v0;
                        let a317 = 8u64;
                        let a318 = 0u64;
                        nat_rem(a316, a317, a318)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a319 = v0;
    let a320 = 4u64;
    if nat_lt(a319, a320) {
        let a321 = v0;
        let a322 = 2u64;
        if nat_lt(a321, a322) {
            let a323 = v0;
            let a324 = 1u64;
            if nat_lt(a323, a324) {
                let a325 = v0;
                let a326 = 0u64;
                if nat_eq(a325, a326) {
                    Ok(f42(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a327 = v0;
                let a328 = 1u64;
                if nat_eq(a327, a328) {
                    Ok(f42(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a329 = v0;
            let a330 = 3u64;
            if nat_lt(a329, a330) {
                let a331 = v0;
                let a332 = 2u64;
                if nat_eq(a331, a332) {
                    Ok(f42(0u64, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a333 = v0;
                let a334 = 3u64;
                if nat_eq(a333, a334) {
                    Ok(f42(0u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a335 = v0;
        let a336 = 6u64;
        if nat_lt(a335, a336) {
            let a337 = v0;
            let a338 = 5u64;
            if nat_lt(a337, a338) {
                let a339 = v0;
                let a340 = 4u64;
                if nat_eq(a339, a340) {
                    Ok(f47(0u64, f48(), 0u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a341 = v0;
                let a342 = 5u64;
                if nat_eq(a341, a342) {
                    Ok(f42(0u64, f49()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a343 = v0;
            let a344 = 7u64;
            if nat_lt(a343, a344) {
                let a345 = v0;
                let a346 = 6u64;
                if nat_eq(a345, a346) {
                    Ok(f50(2u64, f51(f49())?, f51(f49())?, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a347 = v0;
                let a348 = 7u64;
                if nat_eq(a347, a348) {
                    Ok(f50(0u64, f43(), f52(), f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a349 = v0;
    let a350 = 2u64;
    if nat_lt(a349, a350) {
        let a351 = v0;
        let a352 = 1u64;
        if nat_lt(a351, a352) {
            let a353 = v0;
            let a354 = 0u64;
            if nat_eq(a353, a354) {
                f53(0u64, f49(), 1u64, f49())
            } else {
                f2()
            }
        } else {
            let a355 = v0;
            let a356 = 1u64;
            if nat_eq(a355, a356) {
                f47(1u64, f49(), 256u64, 6u64)
            } else {
                f2()
            }
        }
    } else {
        let a357 = v0;
        let a358 = 3u64;
        if nat_lt(a357, a358) {
            let a359 = v0;
            let a360 = 2u64;
            if nat_eq(a359, a360) {
                f50(2u64, 1073741824u64, f43(), f43())
            } else {
                f2()
            }
        } else {
            let a361 = v0;
            let a362 = 4u64;
            if nat_lt(a361, a362) {
                let a363 = v0;
                let a364 = 3u64;
                if nat_eq(a363, a364) {
                    f50(3u64, f43(), 1132462080u64, f43())
                } else {
                    f2()
                }
            } else {
                let a365 = v0;
                let a366 = 4u64;
                if nat_eq(a365, a366) {
                    f54(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a367 = v0;
    let a368 = 4u64;
    if nat_lt(a367, a368) {
        let a369 = v0;
        let a370 = 2u64;
        if nat_lt(a369, a370) {
            let a371 = v0;
            let a372 = 1u64;
            if nat_lt(a371, a372) {
                let a373 = v0;
                let a374 = 0u64;
                if nat_eq(a373, a374) {
                    f42(0u64, f55())
                } else {
                    f2()
                }
            } else {
                let a375 = v0;
                let a376 = 1u64;
                if nat_eq(a375, a376) {
                    f42(0u64, f56())
                } else {
                    f2()
                }
            }
        } else {
            let a377 = v0;
            let a378 = 3u64;
            if nat_lt(a377, a378) {
                let a379 = v0;
                let a380 = 2u64;
                if nat_eq(a379, a380) {
                    f53(2u64, f44(), 32u64, f57())
                } else {
                    f2()
                }
            } else {
                let a381 = v0;
                let a382 = 3u64;
                if nat_eq(a381, a382) {
                    f42(0u64, f58())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a383 = v0;
        let a384 = 6u64;
        if nat_lt(a383, a384) {
            let a385 = v0;
            let a386 = 5u64;
            if nat_lt(a385, a386) {
                let a387 = v0;
                let a388 = 4u64;
                if nat_eq(a387, a388) {
                    f42(0u64, f59())
                } else {
                    f2()
                }
            } else {
                let a389 = v0;
                let a390 = 5u64;
                if nat_eq(a389, a390) {
                    f42(0u64, f60())
                } else {
                    f2()
                }
            }
        } else {
            let a391 = v0;
            let a392 = 7u64;
            if nat_lt(a391, a392) {
                let a393 = v0;
                let a394 = 6u64;
                if nat_eq(a393, a394) {
                    f42(0u64, f61())
                } else {
                    f2()
                }
            } else {
                let a395 = v0;
                let a396 = 7u64;
                if nat_eq(a395, a396) {
                    f53(0u64, f57(), f55(), f62())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a397 = v0;
    let a398 = 4u64;
    if nat_lt(a397, a398) {
        let a399 = v0;
        let a400 = 2u64;
        if nat_lt(a399, a400) {
            let a401 = v0;
            let a402 = 1u64;
            if nat_lt(a401, a402) {
                let a403 = v0;
                let a404 = 0u64;
                if nat_eq(a403, a404) {
                    Ok(f42(f51(f62())?, f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a405 = v0;
                let a406 = 1u64;
                if nat_eq(a405, a406) {
                    Ok(f50(2u64, f63(), f63(), f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a407 = v0;
            let a408 = 3u64;
            if nat_lt(a407, a408) {
                let a409 = v0;
                let a410 = 2u64;
                if nat_eq(a409, a410) {
                    Ok(f47(0u64, f48(), 0u64, 526u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a411 = v0;
                let a412 = 3u64;
                if nat_eq(a411, a412) {
                    Ok(f50(0u64, f43(), f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a413 = v0;
        let a414 = 6u64;
        if nat_lt(a413, a414) {
            let a415 = v0;
            let a416 = 5u64;
            if nat_lt(a415, a416) {
                let a417 = v0;
                let a418 = 4u64;
                if nat_eq(a417, a418) {
                    Ok(f50(4u64, f64(), 0u64, f64()))
                } else {
                    Ok(f2())
                }
            } else {
                let a419 = v0;
                let a420 = 5u64;
                if nat_eq(a419, a420) {
                    Ok(f50(2u64, f65(f62())?, f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a421 = v0;
            let a422 = 7u64;
            if nat_lt(a421, a422) {
                let a423 = v0;
                let a424 = 6u64;
                if nat_eq(a423, a424) {
                    Ok(f42(f64(), f66(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a425 = v0;
                let a426 = 7u64;
                if nat_eq(a425, a426) {
                    Ok(f50(4u64, f64(), 0u64, f67(f55())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a427 = v0;
    let a428 = 4u64;
    if nat_lt(a427, a428) {
        let a429 = v0;
        let a430 = 2u64;
        if nat_lt(a429, a430) {
            let a431 = v0;
            let a432 = 1u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 0u64;
                if nat_eq(a433, a434) {
                    Ok(f42(f63(), f68(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 1u64;
                if nat_eq(a435, a436) {
                    Ok(f47(2u64, f63(), 0u64, 531u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a437 = v0;
            let a438 = 3u64;
            if nat_lt(a437, a438) {
                let a439 = v0;
                let a440 = 2u64;
                if nat_eq(a439, a440) {
                    Ok(f54(535u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a441 = v0;
                let a442 = 3u64;
                if nat_eq(a441, a442) {
                    Ok(f50(6u64, f63(), 0u64, f68(f55())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a443 = v0;
        let a444 = 6u64;
        if nat_lt(a443, a444) {
            let a445 = v0;
            let a446 = 5u64;
            if nat_lt(a445, a446) {
                let a447 = v0;
                let a448 = 4u64;
                if nat_eq(a447, a448) {
                    Ok(f69(6u64, f61(), 0u64, 0u64, f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a449 = v0;
                let a450 = 5u64;
                if nat_eq(a449, a450) {
                    Ok(f53(0u64, f60(), f70(), f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a451 = v0;
            let a452 = 7u64;
            if nat_lt(a451, a452) {
                let a453 = v0;
                let a454 = 6u64;
                if nat_eq(a453, a454) {
                    Ok(f53(1u64, 1u64, f59(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a455 = v0;
                let a456 = 7u64;
                if nat_eq(a455, a456) {
                    Ok(f47(0u64, f71(), 0u64, 543u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a457 = v0;
    let a458 = 4u64;
    if nat_lt(a457, a458) {
        let a459 = v0;
        let a460 = 2u64;
        if nat_lt(a459, a460) {
            let a461 = v0;
            let a462 = 1u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 0u64;
                if nat_eq(a463, a464) {
                    f50(2u64, f64(), f63(), f72())
                } else {
                    f2()
                }
            } else {
                let a465 = v0;
                let a466 = 1u64;
                if nat_eq(a465, a466) {
                    f50(2u64, f72(), f63(), f72())
                } else {
                    f2()
                }
            }
        } else {
            let a467 = v0;
            let a468 = 3u64;
            if nat_lt(a467, a468) {
                let a469 = v0;
                let a470 = 2u64;
                if nat_eq(a469, a470) {
                    f47(0u64, f61(), 0u64, 541u64)
                } else {
                    f2()
                }
            } else {
                let a471 = v0;
                let a472 = 3u64;
                if nat_eq(a471, a472) {
                    f47(2u64, f72(), f73(), 541u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a473 = v0;
        let a474 = 6u64;
        if nat_lt(a473, a474) {
            let a475 = v0;
            let a476 = 5u64;
            if nat_lt(a475, a476) {
                let a477 = v0;
                let a478 = 4u64;
                if nat_eq(a477, a478) {
                    f54(543u64)
                } else {
                    f2()
                }
            } else {
                let a479 = v0;
                let a480 = 5u64;
                if nat_eq(a479, a480) {
                    f42(f72(), f73())
                } else {
                    f2()
                }
            }
        } else {
            let a481 = v0;
            let a482 = 7u64;
            if nat_lt(a481, a482) {
                let a483 = v0;
                let a484 = 6u64;
                if nat_eq(a483, a484) {
                    f42(f61(), f74())
                } else {
                    f2()
                }
            } else {
                let a485 = v0;
                let a486 = 7u64;
                if nat_eq(a485, a486) {
                    f53(0u64, f61(), 1u64, f61())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a487 = v0;
    let a488 = 4u64;
    if nat_lt(a487, a488) {
        let a489 = v0;
        let a490 = 2u64;
        if nat_lt(a489, a490) {
            let a491 = v0;
            let a492 = 1u64;
            if nat_lt(a491, a492) {
                let a493 = v0;
                let a494 = 0u64;
                if nat_eq(a493, a494) {
                    f53(0u64, f55(), 1u64, f55())
                } else {
                    f2()
                }
            } else {
                let a495 = v0;
                let a496 = 1u64;
                if nat_eq(a495, a496) {
                    f47(1u64, f61(), 8u64, 519u64)
                } else {
                    f2()
                }
            }
        } else {
            let a497 = v0;
            let a498 = 3u64;
            if nat_lt(a497, a498) {
                let a499 = v0;
                let a500 = 2u64;
                if nat_eq(a499, a500) {
                    f47(0u64, f71(), 0u64, 558u64)
                } else {
                    f2()
                }
            } else {
                let a501 = v0;
                let a502 = 3u64;
                if nat_eq(a501, a502) {
                    f47(0u64, f59(), 0u64, 558u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a503 = v0;
        let a504 = 6u64;
        if nat_lt(a503, a504) {
            let a505 = v0;
            let a506 = 5u64;
            if nat_lt(a505, a506) {
                let a507 = v0;
                let a508 = 4u64;
                if nat_eq(a507, a508) {
                    f69(6u64, f74(), 0u64, 0u64, f70())
                } else {
                    f2()
                }
            } else {
                let a509 = v0;
                let a510 = 5u64;
                if nat_eq(a509, a510) {
                    f53(3u64, f60(), f70(), f75())
                } else {
                    f2()
                }
            }
        } else {
            let a511 = v0;
            let a512 = 7u64;
            if nat_lt(a511, a512) {
                let a513 = v0;
                let a514 = 6u64;
                if nat_eq(a513, a514) {
                    f53(4u64, f75(), 2u64, f75())
                } else {
                    f2()
                }
            } else {
                let a515 = v0;
                let a516 = 7u64;
                if nat_eq(a515, a516) {
                    f47(0u64, f75(), 0u64, 554u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a517 = v0;
    let a518 = 4u64;
    if nat_lt(a517, a518) {
        let a519 = v0;
        let a520 = 2u64;
        if nat_lt(a519, a520) {
            let a521 = v0;
            let a522 = 1u64;
            if nat_lt(a521, a522) {
                let a523 = v0;
                let a524 = 0u64;
                if nat_eq(a523, a524) {
                    Ok(f53(1u64, f60(), f70(), f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a525 = v0;
                let a526 = 1u64;
                if nat_eq(a525, a526) {
                    Ok(f54(555u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a527 = v0;
            let a528 = 3u64;
            if nat_lt(a527, a528) {
                let a529 = v0;
                let a530 = 2u64;
                if nat_eq(a529, a530) {
                    Ok(f53(0u64, f60(), f70(), f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a531 = v0;
                let a532 = 3u64;
                if nat_eq(a531, a532) {
                    Ok(f53(2u64, f56(), 8u64, f75()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a533 = v0;
        let a534 = 6u64;
        if nat_lt(a533, a534) {
            let a535 = v0;
            let a536 = 5u64;
            if nat_lt(a535, a536) {
                let a537 = v0;
                let a538 = 4u64;
                if nat_eq(a537, a538) {
                    Ok(f53(0u64, f75(), f74(), f75()))
                } else {
                    Ok(f2())
                }
            } else {
                let a539 = v0;
                let a540 = 5u64;
                if nat_eq(a539, a540) {
                    Ok(f50(6u64, f68(f75())?, 0u64, f68(f75())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a541 = v0;
            let a542 = 7u64;
            if nat_lt(a541, a542) {
                let a543 = v0;
                let a544 = 6u64;
                if nat_eq(a543, a544) {
                    Ok(f42(f60(), f76(f56())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a545 = v0;
                let a546 = 7u64;
                if nat_eq(a545, a546) {
                    Ok(f53(0u64, f56(), 1u64, f56()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a547 = v0;
    let a548 = 4u64;
    if nat_lt(a547, a548) {
        let a549 = v0;
        let a550 = 2u64;
        if nat_lt(a549, a550) {
            let a551 = v0;
            let a552 = 1u64;
            if nat_lt(a551, a552) {
                let a553 = v0;
                let a554 = 0u64;
                if nat_eq(a553, a554) {
                    Ok(f47(1u64, f56(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a555 = v0;
                let a556 = 1u64;
                if nat_eq(a555, a556) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a557 = v0;
            let a558 = 3u64;
            if nat_lt(a557, a558) {
                let a559 = v0;
                let a560 = 2u64;
                if nat_eq(a559, a560) {
                    Ok(f47(2u64, f58(), f68(f55())?, 564u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a561 = v0;
                let a562 = 3u64;
                if nat_eq(a561, a562) {
                    Ok(f54(565u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a563 = v0;
        let a564 = 6u64;
        if nat_lt(a563, a564) {
            let a565 = v0;
            let a566 = 5u64;
            if nat_lt(a565, a566) {
                let a567 = v0;
                let a568 = 4u64;
                if nat_eq(a567, a568) {
                    Ok(f42(f68(f55())?, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a569 = v0;
                let a570 = 5u64;
                if nat_eq(a569, a570) {
                    Ok(f53(0u64, f55(), 1u64, f55()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a571 = v0;
            let a572 = 7u64;
            if nat_lt(a571, a572) {
                let a573 = v0;
                let a574 = 6u64;
                if nat_eq(a573, a574) {
                    Ok(f47(1u64, f55(), 32u64, 562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a575 = v0;
                let a576 = 7u64;
                if nat_eq(a575, a576) {
                    Ok(f42(0u64, f77(f44())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a577 = v0;
    let a578 = 1u64;
    if nat_lt(a577, a578) {
        let a579 = v0;
        let a580 = 0u64;
        if nat_eq(a579, a580) {
            f47(0u64, f58(), 0u64, 3109u64)
        } else {
            f2()
        }
    } else {
        let a581 = v0;
        let a582 = 2u64;
        if nat_lt(a581, a582) {
            let a583 = v0;
            let a584 = 1u64;
            if nat_eq(a583, a584) {
                f47(2u64, f58(), f78(), 3109u64)
            } else {
                f2()
            }
        } else {
            let a585 = v0;
            let a586 = 2u64;
            if nat_eq(a585, a586) {
                f54(1024u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a587 = v0;
    let a588 = 4u64;
    if nat_lt(a587, a588) {
        let a589 = v0;
        let a590 = 2u64;
        if nat_lt(a589, a590) {
            let a591 = v0;
            let a592 = 1u64;
            if nat_lt(a591, a592) {
                let a593 = v0;
                let a594 = 0u64;
                if nat_eq(a593, a594) {
                    Ok(f42(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a595 = v0;
                let a596 = 1u64;
                if nat_eq(a595, a596) {
                    Ok(f42(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a597 = v0;
            let a598 = 3u64;
            if nat_lt(a597, a598) {
                let a599 = v0;
                let a600 = 2u64;
                if nat_eq(a599, a600) {
                    Ok(f42(0u64, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a601 = v0;
                let a602 = 3u64;
                if nat_eq(a601, a602) {
                    Ok(f42(0u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a603 = v0;
        let a604 = 6u64;
        if nat_lt(a603, a604) {
            let a605 = v0;
            let a606 = 5u64;
            if nat_lt(a605, a606) {
                let a607 = v0;
                let a608 = 4u64;
                if nat_eq(a607, a608) {
                    Ok(f42(0u64, f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a609 = v0;
                let a610 = 5u64;
                if nat_eq(a609, a610) {
                    Ok(f50(2u64, f51(f49())?, f51(f49())?, f52()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a611 = v0;
            let a612 = 7u64;
            if nat_lt(a611, a612) {
                let a613 = v0;
                let a614 = 6u64;
                if nat_eq(a613, a614) {
                    Ok(f50(0u64, f43(), f52(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a615 = v0;
                let a616 = 7u64;
                if nat_eq(a615, a616) {
                    Ok(f53(0u64, f49(), 1u64, f49()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a617 = v0;
    let a618 = 2u64;
    if nat_lt(a617, a618) {
        let a619 = v0;
        let a620 = 1u64;
        if nat_lt(a619, a620) {
            let a621 = v0;
            let a622 = 0u64;
            if nat_eq(a621, a622) {
                f47(1u64, f49(), 256u64, 5u64)
            } else {
                f2()
            }
        } else {
            let a623 = v0;
            let a624 = 1u64;
            if nat_eq(a623, a624) {
                f47(0u64, f71(), 1u64, 11u64)
            } else {
                f2()
            }
        }
    } else {
        let a625 = v0;
        let a626 = 3u64;
        if nat_lt(a625, a626) {
            let a627 = v0;
            let a628 = 2u64;
            if nat_eq(a627, a628) {
                f50(2u64, 1073741824u64, f43(), f43())
            } else {
                f2()
            }
        } else {
            let a629 = v0;
            let a630 = 4u64;
            if nat_lt(a629, a630) {
                let a631 = v0;
                let a632 = 3u64;
                if nat_eq(a631, a632) {
                    f50(3u64, f43(), 1132462080u64, f43())
                } else {
                    f2()
                }
            } else {
                let a633 = v0;
                let a634 = 4u64;
                if nat_eq(a633, a634) {
                    f54(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a635 = v0;
    let a636 = 4u64;
    if nat_lt(a635, a636) {
        let a637 = v0;
        let a638 = 2u64;
        if nat_lt(a637, a638) {
            let a639 = v0;
            let a640 = 1u64;
            if nat_lt(a639, a640) {
                let a641 = v0;
                let a642 = 0u64;
                if nat_eq(a641, a642) {
                    f42(0u64, f55())
                } else {
                    f2()
                }
            } else {
                let a643 = v0;
                let a644 = 1u64;
                if nat_eq(a643, a644) {
                    f42(0u64, f56())
                } else {
                    f2()
                }
            }
        } else {
            let a645 = v0;
            let a646 = 3u64;
            if nat_lt(a645, a646) {
                let a647 = v0;
                let a648 = 2u64;
                if nat_eq(a647, a648) {
                    f53(2u64, f44(), 16u64, f57())
                } else {
                    f2()
                }
            } else {
                let a649 = v0;
                let a650 = 3u64;
                if nat_eq(a649, a650) {
                    f42(0u64, f58())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a651 = v0;
        let a652 = 6u64;
        if nat_lt(a651, a652) {
            let a653 = v0;
            let a654 = 5u64;
            if nat_lt(a653, a654) {
                let a655 = v0;
                let a656 = 4u64;
                if nat_eq(a655, a656) {
                    f42(0u64, f59())
                } else {
                    f2()
                }
            } else {
                let a657 = v0;
                let a658 = 5u64;
                if nat_eq(a657, a658) {
                    f42(0u64, f60())
                } else {
                    f2()
                }
            }
        } else {
            let a659 = v0;
            let a660 = 7u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 6u64;
                if nat_eq(a661, a662) {
                    f42(0u64, f61())
                } else {
                    f2()
                }
            } else {
                let a663 = v0;
                let a664 = 7u64;
                if nat_eq(a663, a664) {
                    f53(0u64, f57(), f55(), f62())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a665 = v0;
    let a666 = 4u64;
    if nat_lt(a665, a666) {
        let a667 = v0;
        let a668 = 2u64;
        if nat_lt(a667, a668) {
            let a669 = v0;
            let a670 = 1u64;
            if nat_lt(a669, a670) {
                let a671 = v0;
                let a672 = 0u64;
                if nat_eq(a671, a672) {
                    Ok(f42(f51(f62())?, f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a673 = v0;
                let a674 = 1u64;
                if nat_eq(a673, a674) {
                    Ok(f50(2u64, f63(), f63(), f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a675 = v0;
            let a676 = 3u64;
            if nat_lt(a675, a676) {
                let a677 = v0;
                let a678 = 2u64;
                if nat_eq(a677, a678) {
                    Ok(f47(0u64, f48(), 0u64, 527u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a679 = v0;
                let a680 = 3u64;
                if nat_eq(a679, a680) {
                    Ok(f50(0u64, f43(), f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a681 = v0;
        let a682 = 6u64;
        if nat_lt(a681, a682) {
            let a683 = v0;
            let a684 = 5u64;
            if nat_lt(a683, a684) {
                let a685 = v0;
                let a686 = 4u64;
                if nat_eq(a685, a686) {
                    Ok(f50(4u64, f64(), 0u64, f64()))
                } else {
                    Ok(f2())
                }
            } else {
                let a687 = v0;
                let a688 = 5u64;
                if nat_eq(a687, a688) {
                    Ok(f50(2u64, f65(f62())?, f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a689 = v0;
            let a690 = 7u64;
            if nat_lt(a689, a690) {
                let a691 = v0;
                let a692 = 6u64;
                if nat_eq(a691, a692) {
                    Ok(f54(529u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a693 = v0;
                let a694 = 7u64;
                if nat_eq(a693, a694) {
                    Ok(f50(2u64, 1048576000u64, f43(), f75()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a695 = v0;
    let a696 = 4u64;
    if nat_lt(a695, a696) {
        let a697 = v0;
        let a698 = 2u64;
        if nat_lt(a697, a698) {
            let a699 = v0;
            let a700 = 1u64;
            if nat_lt(a699, a700) {
                let a701 = v0;
                let a702 = 0u64;
                if nat_eq(a701, a702) {
                    Ok(f50(0u64, f75(), f64(), f64()))
                } else {
                    Ok(f2())
                }
            } else {
                let a703 = v0;
                let a704 = 1u64;
                if nat_eq(a703, a704) {
                    Ok(f42(f64(), f66(f55())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a705 = v0;
            let a706 = 3u64;
            if nat_lt(a705, a706) {
                let a707 = v0;
                let a708 = 2u64;
                if nat_eq(a707, a708) {
                    Ok(f50(4u64, f64(), 0u64, f67(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a709 = v0;
                let a710 = 3u64;
                if nat_eq(a709, a710) {
                    Ok(f42(f63(), f68(f55())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a711 = v0;
        let a712 = 6u64;
        if nat_lt(a711, a712) {
            let a713 = v0;
            let a714 = 5u64;
            if nat_lt(a713, a714) {
                let a715 = v0;
                let a716 = 4u64;
                if nat_eq(a715, a716) {
                    Ok(f47(2u64, f63(), 0u64, 534u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a717 = v0;
                let a718 = 5u64;
                if nat_eq(a717, a718) {
                    Ok(f54(538u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a719 = v0;
            let a720 = 7u64;
            if nat_lt(a719, a720) {
                let a721 = v0;
                let a722 = 6u64;
                if nat_eq(a721, a722) {
                    Ok(f50(6u64, f63(), 0u64, f68(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a723 = v0;
                let a724 = 7u64;
                if nat_eq(a723, a724) {
                    Ok(f69(6u64, f61(), 0u64, 0u64, f70()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a725 = v0;
    let a726 = 4u64;
    if nat_lt(a725, a726) {
        let a727 = v0;
        let a728 = 2u64;
        if nat_lt(a727, a728) {
            let a729 = v0;
            let a730 = 1u64;
            if nat_lt(a729, a730) {
                let a731 = v0;
                let a732 = 0u64;
                if nat_eq(a731, a732) {
                    f53(0u64, f60(), f70(), f60())
                } else {
                    f2()
                }
            } else {
                let a733 = v0;
                let a734 = 1u64;
                if nat_eq(a733, a734) {
                    f53(1u64, 1u64, f59(), f59())
                } else {
                    f2()
                }
            }
        } else {
            let a735 = v0;
            let a736 = 3u64;
            if nat_lt(a735, a736) {
                let a737 = v0;
                let a738 = 2u64;
                if nat_eq(a737, a738) {
                    f47(0u64, f71(), 0u64, 546u64)
                } else {
                    f2()
                }
            } else {
                let a739 = v0;
                let a740 = 3u64;
                if nat_eq(a739, a740) {
                    f50(2u64, f64(), f63(), f72())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a741 = v0;
        let a742 = 6u64;
        if nat_lt(a741, a742) {
            let a743 = v0;
            let a744 = 5u64;
            if nat_lt(a743, a744) {
                let a745 = v0;
                let a746 = 4u64;
                if nat_eq(a745, a746) {
                    f50(2u64, f72(), f63(), f72())
                } else {
                    f2()
                }
            } else {
                let a747 = v0;
                let a748 = 5u64;
                if nat_eq(a747, a748) {
                    f47(0u64, f61(), 0u64, 544u64)
                } else {
                    f2()
                }
            }
        } else {
            let a749 = v0;
            let a750 = 7u64;
            if nat_lt(a749, a750) {
                let a751 = v0;
                let a752 = 6u64;
                if nat_eq(a751, a752) {
                    f47(2u64, f72(), f73(), 544u64)
                } else {
                    f2()
                }
            } else {
                let a753 = v0;
                let a754 = 7u64;
                if nat_eq(a753, a754) {
                    f54(546u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a755 = v0;
    let a756 = 4u64;
    if nat_lt(a755, a756) {
        let a757 = v0;
        let a758 = 2u64;
        if nat_lt(a757, a758) {
            let a759 = v0;
            let a760 = 1u64;
            if nat_lt(a759, a760) {
                let a761 = v0;
                let a762 = 0u64;
                if nat_eq(a761, a762) {
                    f42(f72(), f73())
                } else {
                    f2()
                }
            } else {
                let a763 = v0;
                let a764 = 1u64;
                if nat_eq(a763, a764) {
                    f42(f61(), f74())
                } else {
                    f2()
                }
            }
        } else {
            let a765 = v0;
            let a766 = 3u64;
            if nat_lt(a765, a766) {
                let a767 = v0;
                let a768 = 2u64;
                if nat_eq(a767, a768) {
                    f53(0u64, f61(), 1u64, f61())
                } else {
                    f2()
                }
            } else {
                let a769 = v0;
                let a770 = 3u64;
                if nat_eq(a769, a770) {
                    f53(0u64, f55(), 1u64, f55())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a771 = v0;
        let a772 = 6u64;
        if nat_lt(a771, a772) {
            let a773 = v0;
            let a774 = 5u64;
            if nat_lt(a773, a774) {
                let a775 = v0;
                let a776 = 4u64;
                if nat_eq(a775, a776) {
                    f47(1u64, f61(), 8u64, 519u64)
                } else {
                    f2()
                }
            } else {
                let a777 = v0;
                let a778 = 5u64;
                if nat_eq(a777, a778) {
                    f47(0u64, f71(), 0u64, 561u64)
                } else {
                    f2()
                }
            }
        } else {
            let a779 = v0;
            let a780 = 7u64;
            if nat_lt(a779, a780) {
                let a781 = v0;
                let a782 = 6u64;
                if nat_eq(a781, a782) {
                    f47(0u64, f59(), 0u64, 561u64)
                } else {
                    f2()
                }
            } else {
                let a783 = v0;
                let a784 = 7u64;
                if nat_eq(a783, a784) {
                    f69(6u64, f74(), 0u64, 0u64, f70())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a785 = v0;
    let a786 = 4u64;
    if nat_lt(a785, a786) {
        let a787 = v0;
        let a788 = 2u64;
        if nat_lt(a787, a788) {
            let a789 = v0;
            let a790 = 1u64;
            if nat_lt(a789, a790) {
                let a791 = v0;
                let a792 = 0u64;
                if nat_eq(a791, a792) {
                    f53(3u64, f60(), f70(), f75())
                } else {
                    f2()
                }
            } else {
                let a793 = v0;
                let a794 = 1u64;
                if nat_eq(a793, a794) {
                    f53(4u64, f75(), 2u64, f75())
                } else {
                    f2()
                }
            }
        } else {
            let a795 = v0;
            let a796 = 3u64;
            if nat_lt(a795, a796) {
                let a797 = v0;
                let a798 = 2u64;
                if nat_eq(a797, a798) {
                    f47(0u64, f75(), 0u64, 557u64)
                } else {
                    f2()
                }
            } else {
                let a799 = v0;
                let a800 = 3u64;
                if nat_eq(a799, a800) {
                    f53(1u64, f60(), f70(), f60())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a801 = v0;
        let a802 = 6u64;
        if nat_lt(a801, a802) {
            let a803 = v0;
            let a804 = 5u64;
            if nat_lt(a803, a804) {
                let a805 = v0;
                let a806 = 4u64;
                if nat_eq(a805, a806) {
                    f54(558u64)
                } else {
                    f2()
                }
            } else {
                let a807 = v0;
                let a808 = 5u64;
                if nat_eq(a807, a808) {
                    f53(0u64, f60(), f70(), f60())
                } else {
                    f2()
                }
            }
        } else {
            let a809 = v0;
            let a810 = 7u64;
            if nat_lt(a809, a810) {
                let a811 = v0;
                let a812 = 6u64;
                if nat_eq(a811, a812) {
                    f53(2u64, f56(), 8u64, f75())
                } else {
                    f2()
                }
            } else {
                let a813 = v0;
                let a814 = 7u64;
                if nat_eq(a813, a814) {
                    f53(0u64, f75(), f74(), f75())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a815 = v0;
    let a816 = 4u64;
    if nat_lt(a815, a816) {
        let a817 = v0;
        let a818 = 2u64;
        if nat_lt(a817, a818) {
            let a819 = v0;
            let a820 = 1u64;
            if nat_lt(a819, a820) {
                let a821 = v0;
                let a822 = 0u64;
                if nat_eq(a821, a822) {
                    Ok(f50(6u64, f68(f75())?, 0u64, f68(f75())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a823 = v0;
                let a824 = 1u64;
                if nat_eq(a823, a824) {
                    Ok(f42(f60(), f76(f56())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a825 = v0;
            let a826 = 3u64;
            if nat_lt(a825, a826) {
                let a827 = v0;
                let a828 = 2u64;
                if nat_eq(a827, a828) {
                    Ok(f53(0u64, f56(), 1u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a829 = v0;
                let a830 = 3u64;
                if nat_eq(a829, a830) {
                    Ok(f47(1u64, f56(), 2u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a831 = v0;
        let a832 = 6u64;
        if nat_lt(a831, a832) {
            let a833 = v0;
            let a834 = 5u64;
            if nat_lt(a833, a834) {
                let a835 = v0;
                let a836 = 4u64;
                if nat_eq(a835, a836) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a837 = v0;
                let a838 = 5u64;
                if nat_eq(a837, a838) {
                    Ok(f47(2u64, f58(), f68(f55())?, 567u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a839 = v0;
            let a840 = 7u64;
            if nat_lt(a839, a840) {
                let a841 = v0;
                let a842 = 6u64;
                if nat_eq(a841, a842) {
                    Ok(f54(568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a843 = v0;
                let a844 = 7u64;
                if nat_eq(a843, a844) {
                    Ok(f42(f68(f55())?, f58()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a845 = v0;
    let a846 = 3u64;
    if nat_lt(a845, a846) {
        let a847 = v0;
        let a848 = 1u64;
        if nat_lt(a847, a848) {
            let a849 = v0;
            let a850 = 0u64;
            if nat_eq(a849, a850) {
                Ok(f53(0u64, f55(), 1u64, f55()))
            } else {
                Ok(f2())
            }
        } else {
            let a851 = v0;
            let a852 = 2u64;
            if nat_lt(a851, a852) {
                let a853 = v0;
                let a854 = 1u64;
                if nat_eq(a853, a854) {
                    Ok(f47(1u64, f55(), 16u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a855 = v0;
                let a856 = 2u64;
                if nat_eq(a855, a856) {
                    Ok(f42(0u64, f77(f44())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a857 = v0;
        let a858 = 4u64;
        if nat_lt(a857, a858) {
            let a859 = v0;
            let a860 = 3u64;
            if nat_eq(a859, a860) {
                Ok(f47(0u64, f58(), 0u64, 3099u64))
            } else {
                Ok(f2())
            }
        } else {
            let a861 = v0;
            let a862 = 5u64;
            if nat_lt(a861, a862) {
                let a863 = v0;
                let a864 = 4u64;
                if nat_eq(a863, a864) {
                    Ok(f47(2u64, f58(), f78(), 3099u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a865 = v0;
                let a866 = 5u64;
                if nat_eq(a865, a866) {
                    Ok(f54(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a867 = v0;
    let a868 = 4u64;
    if nat_lt(a867, a868) {
        let a869 = v0;
        let a870 = 2u64;
        if nat_lt(a869, a870) {
            let a871 = v0;
            let a872 = 1u64;
            if nat_lt(a871, a872) {
                let a873 = v0;
                let a874 = 0u64;
                if nat_eq(a873, a874) {
                    Ok(f50(6u64, f68(f75())?, 0u64, f68(f75())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a875 = v0;
                let a876 = 1u64;
                if nat_eq(a875, a876) {
                    Ok(f42(f60(), f76(f56())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a877 = v0;
            let a878 = 3u64;
            if nat_lt(a877, a878) {
                let a879 = v0;
                let a880 = 2u64;
                if nat_eq(a879, a880) {
                    Ok(f53(0u64, f56(), 1u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a881 = v0;
                let a882 = 3u64;
                if nat_eq(a881, a882) {
                    Ok(f47(1u64, f56(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a883 = v0;
        let a884 = 6u64;
        if nat_lt(a883, a884) {
            let a885 = v0;
            let a886 = 5u64;
            if nat_lt(a885, a886) {
                let a887 = v0;
                let a888 = 4u64;
                if nat_eq(a887, a888) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a889 = v0;
                let a890 = 5u64;
                if nat_eq(a889, a890) {
                    Ok(f47(2u64, f58(), f68(f55())?, 567u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a891 = v0;
            let a892 = 7u64;
            if nat_lt(a891, a892) {
                let a893 = v0;
                let a894 = 6u64;
                if nat_eq(a893, a894) {
                    Ok(f54(568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a895 = v0;
                let a896 = 7u64;
                if nat_eq(a895, a896) {
                    Ok(f42(f68(f55())?, f58()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a897 = v0;
    let a898 = 3u64;
    if nat_lt(a897, a898) {
        let a899 = v0;
        let a900 = 1u64;
        if nat_lt(a899, a900) {
            let a901 = v0;
            let a902 = 0u64;
            if nat_eq(a901, a902) {
                Ok(f53(0u64, f55(), 1u64, f55()))
            } else {
                Ok(f2())
            }
        } else {
            let a903 = v0;
            let a904 = 2u64;
            if nat_lt(a903, a904) {
                let a905 = v0;
                let a906 = 1u64;
                if nat_eq(a905, a906) {
                    Ok(f47(1u64, f55(), 32u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a907 = v0;
                let a908 = 2u64;
                if nat_eq(a907, a908) {
                    Ok(f42(0u64, f77(f44())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a909 = v0;
        let a910 = 4u64;
        if nat_lt(a909, a910) {
            let a911 = v0;
            let a912 = 3u64;
            if nat_eq(a911, a912) {
                Ok(f47(0u64, f58(), 0u64, 3091u64))
            } else {
                Ok(f2())
            }
        } else {
            let a913 = v0;
            let a914 = 5u64;
            if nat_lt(a913, a914) {
                let a915 = v0;
                let a916 = 4u64;
                if nat_eq(a915, a916) {
                    Ok(f47(2u64, f58(), f78(), 3091u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a917 = v0;
                let a918 = 5u64;
                if nat_eq(a917, a918) {
                    Ok(f54(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f32(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a919 = v0;
    let a920 = 4u64;
    if nat_lt(a919, a920) {
        let a921 = v0;
        let a922 = 2u64;
        if nat_lt(a921, a922) {
            let a923 = v0;
            let a924 = 1u64;
            if nat_lt(a923, a924) {
                let a925 = v0;
                let a926 = 0u64;
                if nat_eq(a925, a926) {
                    Ok(f53(3u64, 256u64, f79(), f80()))
                } else {
                    Ok(f2())
                }
            } else {
                let a927 = v0;
                let a928 = 1u64;
                if nat_eq(a927, a928) {
                    Ok(f53(3u64, f79(), 8u64, f45()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a929 = v0;
            let a930 = 3u64;
            if nat_lt(a929, a930) {
                let a931 = v0;
                let a932 = 2u64;
                if nat_eq(a931, a932) {
                    Ok(f42(0u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a933 = v0;
                let a934 = 3u64;
                if nat_eq(a933, a934) {
                    Ok(f42(0u64, f44()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a935 = v0;
        let a936 = 6u64;
        if nat_lt(a935, a936) {
            let a937 = v0;
            let a938 = 5u64;
            if nat_lt(a937, a938) {
                let a939 = v0;
                let a940 = 4u64;
                if nat_eq(a939, a940) {
                    Ok(f42(0u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a941 = v0;
                let a942 = 5u64;
                if nat_eq(a941, a942) {
                    Ok(f47(0u64, f48(), 0u64, 512u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a943 = v0;
            let a944 = 7u64;
            if nat_lt(a943, a944) {
                let a945 = v0;
                let a946 = 6u64;
                if nat_eq(a945, a946) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a947 = v0;
                let a948 = 7u64;
                if nat_eq(a947, a948) {
                    Ok(f50(2u64, f51(f55())?, f51(f55())?, f75()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a949 = v0;
    let a950 = 3u64;
    if nat_lt(a949, a950) {
        let a951 = v0;
        let a952 = 1u64;
        if nat_lt(a951, a952) {
            let a953 = v0;
            let a954 = 0u64;
            if nat_eq(a953, a954) {
                f50(0u64, f43(), f75(), f43())
            } else {
                f2()
            }
        } else {
            let a955 = v0;
            let a956 = 2u64;
            if nat_lt(a955, a956) {
                let a957 = v0;
                let a958 = 1u64;
                if nat_eq(a957, a958) {
                    f53(0u64, f55(), 1u64, f55())
                } else {
                    f2()
                }
            } else {
                let a959 = v0;
                let a960 = 2u64;
                if nat_eq(a959, a960) {
                    f47(1u64, f55(), 256u64, 7u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a961 = v0;
        let a962 = 4u64;
        if nat_lt(a961, a962) {
            let a963 = v0;
            let a964 = 3u64;
            if nat_eq(a963, a964) {
                f50(2u64, 1073741824u64, f43(), f43())
            } else {
                f2()
            }
        } else {
            let a965 = v0;
            let a966 = 5u64;
            if nat_lt(a965, a966) {
                let a967 = v0;
                let a968 = 4u64;
                if nat_eq(a967, a968) {
                    f50(3u64, f43(), 1132462080u64, f43())
                } else {
                    f2()
                }
            } else {
                let a969 = v0;
                let a970 = 5u64;
                if nat_eq(a969, a970) {
                    f54(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a971 = v0;
    let a972 = 4u64;
    if nat_lt(a971, a972) {
        let a973 = v0;
        let a974 = 2u64;
        if nat_lt(a973, a974) {
            let a975 = v0;
            let a976 = 1u64;
            if nat_lt(a975, a976) {
                let a977 = v0;
                let a978 = 0u64;
                if nat_eq(a977, a978) {
                    Ok(f53(2u64, f44(), f79(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a979 = v0;
                let a980 = 1u64;
                if nat_eq(a979, a980) {
                    Ok(f42(0u64, f58()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a981 = v0;
            let a982 = 3u64;
            if nat_lt(a981, a982) {
                let a983 = v0;
                let a984 = 2u64;
                if nat_eq(a983, a984) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a985 = v0;
                let a986 = 3u64;
                if nat_eq(a985, a986) {
                    Ok(f42(0u64, f81(f44())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a987 = v0;
        let a988 = 6u64;
        if nat_lt(a987, a988) {
            let a989 = v0;
            let a990 = 5u64;
            if nat_lt(a989, a990) {
                let a991 = v0;
                let a992 = 4u64;
                if nat_eq(a991, a992) {
                    Ok(f53(0u64, f57(), f55(), f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a993 = v0;
                let a994 = 5u64;
                if nat_eq(a993, a994) {
                    Ok(f50(2u64, f51(f62())?, f51(f62())?, f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a995 = v0;
            let a996 = 7u64;
            if nat_lt(a995, a996) {
                let a997 = v0;
                let a998 = 6u64;
                if nat_eq(a997, a998) {
                    Ok(f47(0u64, f48(), 0u64, 522u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a999 = v0;
                let a1000 = 7u64;
                if nat_eq(a999, a1000) {
                    Ok(f50(0u64, f43(), f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f35(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1001 = v0;
    let a1002 = 4u64;
    if nat_lt(a1001, a1002) {
        let a1003 = v0;
        let a1004 = 2u64;
        if nat_lt(a1003, a1004) {
            let a1005 = v0;
            let a1006 = 1u64;
            if nat_lt(a1005, a1006) {
                let a1007 = v0;
                let a1008 = 0u64;
                if nat_eq(a1007, a1008) {
                    Ok(f50(4u64, f64(), 0u64, f64()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1009 = v0;
                let a1010 = 1u64;
                if nat_eq(a1009, a1010) {
                    Ok(f50(2u64, f65(f62())?, f64(), f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1011 = v0;
            let a1012 = 3u64;
            if nat_lt(a1011, a1012) {
                let a1013 = v0;
                let a1014 = 2u64;
                if nat_eq(a1013, a1014) {
                    Ok(f42(f64(), f82(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1015 = v0;
                let a1016 = 3u64;
                if nat_eq(a1015, a1016) {
                    Ok(f42(f51(f62())?, f63()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1017 = v0;
        let a1018 = 6u64;
        if nat_lt(a1017, a1018) {
            let a1019 = v0;
            let a1020 = 5u64;
            if nat_lt(a1019, a1020) {
                let a1021 = v0;
                let a1022 = 4u64;
                if nat_eq(a1021, a1022) {
                    Ok(f47(2u64, f63(), 0u64, 526u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1023 = v0;
                let a1024 = 5u64;
                if nat_eq(a1023, a1024) {
                    Ok(f54(527u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1025 = v0;
            let a1026 = 7u64;
            if nat_lt(a1025, a1026) {
                let a1027 = v0;
                let a1028 = 6u64;
                if nat_eq(a1027, a1028) {
                    Ok(f50(6u64, f63(), 0u64, f63()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1029 = v0;
                let a1030 = 7u64;
                if nat_eq(a1029, a1030) {
                    Ok(f47(2u64, f58(), f63(), 529u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f36(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1031 = v0;
    let a1032 = 4u64;
    if nat_lt(a1031, a1032) {
        let a1033 = v0;
        let a1034 = 2u64;
        if nat_lt(a1033, a1034) {
            let a1035 = v0;
            let a1036 = 1u64;
            if nat_lt(a1035, a1036) {
                let a1037 = v0;
                let a1038 = 0u64;
                if nat_eq(a1037, a1038) {
                    Ok(f54(530u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1039 = v0;
                let a1040 = 1u64;
                if nat_eq(a1039, a1040) {
                    Ok(f42(f63(), f58()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1041 = v0;
            let a1042 = 3u64;
            if nat_lt(a1041, a1042) {
                let a1043 = v0;
                let a1044 = 2u64;
                if nat_eq(a1043, a1044) {
                    Ok(f42(f55(), f83(f55())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1045 = v0;
                let a1046 = 3u64;
                if nat_eq(a1045, a1046) {
                    Ok(f53(0u64, f55(), 1u64, f55()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1047 = v0;
        let a1048 = 6u64;
        if nat_lt(a1047, a1048) {
            let a1049 = v0;
            let a1050 = 5u64;
            if nat_lt(a1049, a1050) {
                let a1051 = v0;
                let a1052 = 4u64;
                if nat_eq(a1051, a1052) {
                    Ok(f47(1u64, f55(), f79(), 516u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1053 = v0;
                let a1054 = 5u64;
                if nat_eq(a1053, a1054) {
                    Ok(f47(2u64, f58(), f78(), 2586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1055 = v0;
            let a1056 = 7u64;
            if nat_lt(a1055, a1056) {
                let a1057 = v0;
                let a1058 = 6u64;
                if nat_eq(a1057, a1058) {
                    Ok(f42(1u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1059 = v0;
                let a1060 = 7u64;
                if nat_eq(a1059, a1060) {
                    Ok(f42(f55(), f84()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f37(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1061 = v0;
    let a1062 = 4u64;
    if nat_lt(a1061, a1062) {
        let a1063 = v0;
        let a1064 = 2u64;
        if nat_lt(a1063, a1064) {
            let a1065 = v0;
            let a1066 = 1u64;
            if nat_lt(a1065, a1066) {
                let a1067 = v0;
                let a1068 = 0u64;
                if nat_eq(a1067, a1068) {
                    Ok(f53(0u64, f57(), f55(), f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1069 = v0;
                let a1070 = 1u64;
                if nat_eq(a1069, a1070) {
                    Ok(f42(f51(f62())?, f63()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1071 = v0;
            let a1072 = 3u64;
            if nat_lt(a1071, a1072) {
                let a1073 = v0;
                let a1074 = 2u64;
                if nat_eq(a1073, a1074) {
                    Ok(f47(0u64, f84(), 0u64, 547u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1075 = v0;
                let a1076 = 3u64;
                if nat_eq(a1075, a1076) {
                    Ok(f53(1u64, f84(), 1u64, f49()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1077 = v0;
        let a1078 = 6u64;
        if nat_lt(a1077, a1078) {
            let a1079 = v0;
            let a1080 = 5u64;
            if nat_lt(a1079, a1080) {
                let a1081 = v0;
                let a1082 = 4u64;
                if nat_eq(a1081, a1082) {
                    Ok(f42(f83(f49())?, f85()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1083 = v0;
                let a1084 = 5u64;
                if nat_eq(a1083, a1084) {
                    Ok(f53(0u64, f57(), f85(), f62()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1085 = v0;
            let a1086 = 7u64;
            if nat_lt(a1085, a1086) {
                let a1087 = v0;
                let a1088 = 6u64;
                if nat_eq(a1087, a1088) {
                    Ok(f47(2u64, f63(), f51(f62())?, 544u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1089 = v0;
                let a1090 = 7u64;
                if nat_eq(a1089, a1090) {
                    Ok(f54(547u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f38(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1091 = v0;
    let a1092 = 4u64;
    if nat_lt(a1091, a1092) {
        let a1093 = v0;
        let a1094 = 2u64;
        if nat_lt(a1093, a1094) {
            let a1095 = v0;
            let a1096 = 1u64;
            if nat_lt(a1095, a1096) {
                let a1097 = v0;
                let a1098 = 0u64;
                if nat_eq(a1097, a1098) {
                    Ok(f42(f85(), f83(f84())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1099 = v0;
                let a1100 = 1u64;
                if nat_eq(a1099, a1100) {
                    Ok(f42(f49(), f84()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1101 = v0;
            let a1102 = 3u64;
            if nat_lt(a1101, a1102) {
                let a1103 = v0;
                let a1104 = 2u64;
                if nat_eq(a1103, a1104) {
                    Ok(f54(538u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1105 = v0;
                let a1106 = 3u64;
                if nat_eq(a1105, a1106) {
                    Ok(f42(f55(), f83(f84())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1107 = v0;
        let a1108 = 6u64;
        if nat_lt(a1107, a1108) {
            let a1109 = v0;
            let a1110 = 5u64;
            if nat_lt(a1109, a1110) {
                let a1111 = v0;
                let a1112 = 4u64;
                if nat_eq(a1111, a1112) {
                    Ok(f53(0u64, f55(), 1u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1113 = v0;
                let a1114 = 5u64;
                if nat_eq(a1113, a1114) {
                    Ok(f47(1u64, f55(), f79(), 535u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1115 = v0;
            let a1116 = 7u64;
            if nat_lt(a1115, a1116) {
                let a1117 = v0;
                let a1118 = 6u64;
                if nat_eq(a1117, a1118) {
                    Ok(f42(0u64, f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1119 = v0;
                let a1120 = 7u64;
                if nat_eq(a1119, a1120) {
                    Ok(f42(0u64, f86(f49())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f39(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1121 = v0;
    let a1122 = 4u64;
    if nat_lt(a1121, a1122) {
        let a1123 = v0;
        let a1124 = 2u64;
        if nat_lt(a1123, a1124) {
            let a1125 = v0;
            let a1126 = 1u64;
            if nat_lt(a1125, a1126) {
                let a1127 = v0;
                let a1128 = 0u64;
                if nat_eq(a1127, a1128) {
                    Ok(f42(0u64, f87(f49())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1129 = v0;
                let a1130 = 1u64;
                if nat_eq(a1129, a1130) {
                    Ok(f42(17u64, f49()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1131 = v0;
            let a1132 = 3u64;
            if nat_lt(a1131, a1132) {
                let a1133 = v0;
                let a1134 = 2u64;
                if nat_eq(a1133, a1134) {
                    Ok(f42(0u64, f86(f49())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1135 = v0;
                let a1136 = 3u64;
                if nat_eq(a1135, a1136) {
                    Ok(f42(0u64, f87(f49())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1137 = v0;
        let a1138 = 6u64;
        if nat_lt(a1137, a1138) {
            let a1139 = v0;
            let a1140 = 5u64;
            if nat_lt(a1139, a1140) {
                let a1141 = v0;
                let a1142 = 4u64;
                if nat_eq(a1141, a1142) {
                    Ok(f42(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1143 = v0;
                let a1144 = 5u64;
                if nat_eq(a1143, a1144) {
                    Ok(f42(f83(f55())?, f85()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1145 = v0;
            let a1146 = 7u64;
            if nat_lt(a1145, a1146) {
                let a1147 = v0;
                let a1148 = 6u64;
                if nat_eq(a1147, a1148) {
                    Ok(f53(0u64, f57(), f85(), f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1149 = v0;
                let a1150 = 7u64;
                if nat_eq(a1149, a1150) {
                    Ok(f42(f55(), f49()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f40(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1151 = v0;
    let a1152 = 4u64;
    if nat_lt(a1151, a1152) {
        let a1153 = v0;
        let a1154 = 2u64;
        if nat_lt(a1153, a1154) {
            let a1155 = v0;
            let a1156 = 1u64;
            if nat_lt(a1155, a1156) {
                let a1157 = v0;
                let a1158 = 0u64;
                if nat_eq(a1157, a1158) {
                    Ok(f47(0u64, f71(), 0u64, 568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1159 = v0;
                let a1160 = 1u64;
                if nat_eq(a1159, a1160) {
                    Ok(f53(0u64, f55(), 17u64, f88()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1161 = v0;
            let a1162 = 3u64;
            if nat_lt(a1161, a1162) {
                let a1163 = v0;
                let a1164 = 2u64;
                if nat_eq(a1163, a1164) {
                    Ok(f47(1u64, f85(), 8u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1165 = v0;
                let a1166 = 3u64;
                if nat_eq(a1165, a1166) {
                    Ok(f42(f55(), f88()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1167 = v0;
        let a1168 = 6u64;
        if nat_lt(a1167, a1168) {
            let a1169 = v0;
            let a1170 = 5u64;
            if nat_lt(a1169, a1170) {
                let a1171 = v0;
                let a1172 = 4u64;
                if nat_eq(a1171, a1172) {
                    Ok(f53(0u64, f55(), 17u64, f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1173 = v0;
                let a1174 = 5u64;
                if nat_eq(a1173, a1174) {
                    Ok(f53(0u64, f88(), 1u64, f75()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1175 = v0;
            let a1176 = 7u64;
            if nat_lt(a1175, a1176) {
                let a1177 = v0;
                let a1178 = 6u64;
                if nat_eq(a1177, a1178) {
                    Ok(f42(f86(f88())?, f86(f75())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1179 = v0;
                let a1180 = 7u64;
                if nat_eq(a1179, a1180) {
                    Ok(f42(f87(f88())?, f87(f75())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f41(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a1181 = v0;
    let a1182 = 3u64;
    if nat_lt(a1181, a1182) {
        let a1183 = v0;
        let a1184 = 1u64;
        if nat_lt(a1183, a1184) {
            let a1185 = v0;
            let a1186 = 0u64;
            if nat_eq(a1185, a1186) {
                Ok(f53(0u64, f49(), 1u64, f75()))
            } else {
                Ok(f2())
            }
        } else {
            let a1187 = v0;
            let a1188 = 2u64;
            if nat_lt(a1187, a1188) {
                let a1189 = v0;
                let a1190 = 1u64;
                if nat_eq(a1189, a1190) {
                    Ok(f50(2u64, f82(f85())?, f51(f62())?, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a1191 = v0;
                let a1192 = 2u64;
                if nat_eq(a1191, a1192) {
                    Ok(f50(0u64, f86(f49())?, f52(), f86(f75())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a1193 = v0;
        let a1194 = 5u64;
        if nat_lt(a1193, a1194) {
            let a1195 = v0;
            let a1196 = 4u64;
            if nat_lt(a1195, a1196) {
                let a1197 = v0;
                let a1198 = 3u64;
                if nat_eq(a1197, a1198) {
                    Ok(f50(0u64, f87(f49())?, f82(f85())?, f87(f75())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a1199 = v0;
                let a1200 = 4u64;
                if nat_eq(a1199, a1200) {
                    Ok(f53(0u64, f55(), 1u64, f55()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a1201 = v0;
            let a1202 = 6u64;
            if nat_lt(a1201, a1202) {
                let a1203 = v0;
                let a1204 = 5u64;
                if nat_eq(a1203, a1204) {
                    Ok(f47(1u64, f55(), f79(), 557u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a1205 = v0;
                let a1206 = 6u64;
                if nat_eq(a1205, a1206) {
                    Ok(f54(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f42(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f43() -> u64 {
    4294967348u64
}

pub fn f44() -> u64 {
    4294967301u64
}

pub fn f45() -> u64 {
    4294967302u64
}

pub fn f46() -> u64 {
    4294967347u64
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f48() -> u64 {
    4294967298u64
}

pub fn f49() -> u64 {
    4294967343u64
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f51(v0: u64) -> R<u64> {
    let a1210 = 12884901888u64;
    let a1211 = {
        let a1207 = v0;
        let a1208 = 4294967296u64;
        let a1209 = 0u64;
        nat_rem(a1207, a1208, a1209)
    };
    nat_add(a1210, a1211)
}

pub fn f52() -> u64 {
    4294967337u64
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f54(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f55() -> u64 {
    4294967303u64
}

pub fn f56() -> u64 {
    4294967304u64
}

pub fn f57() -> u64 {
    4294967354u64
}

pub fn f58() -> u64 {
    4294967315u64
}

pub fn f59() -> u64 {
    4294967313u64
}

pub fn f60() -> u64 {
    4294967314u64
}

pub fn f61() -> u64 {
    4294967305u64
}

pub fn f62() -> u64 {
    4294967307u64
}

pub fn f63() -> u64 {
    4294967308u64
}

pub fn f64() -> u64 {
    4294967309u64
}

pub fn f65(v0: u64) -> R<u64> {
    let a1215 = 17179869184u64;
    let a1216 = {
        let a1212 = v0;
        let a1213 = 4294967296u64;
        let a1214 = 0u64;
        nat_rem(a1212, a1213, a1214)
    };
    nat_add(a1215, a1216)
}

pub fn f66(v0: u64) -> R<u64> {
    let a1220 = 8589959168u64;
    let a1221 = {
        let a1217 = v0;
        let a1218 = 4294967296u64;
        let a1219 = 0u64;
        nat_rem(a1217, a1218, a1219)
    };
    nat_add(a1220, a1221)
}

pub fn f67(v0: u64) -> R<u64> {
    let a1225 = 8589967360u64;
    let a1226 = {
        let a1222 = v0;
        let a1223 = 4294967296u64;
        let a1224 = 0u64;
        nat_rem(a1222, a1223, a1224)
    };
    nat_add(a1225, a1226)
}

pub fn f68(v0: u64) -> R<u64> {
    let a1230 = 8589950976u64;
    let a1231 = {
        let a1227 = v0;
        let a1228 = 4294967296u64;
        let a1229 = 0u64;
        nat_rem(a1227, a1228, a1229)
    };
    nat_add(a1230, a1231)
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f70() -> u64 {
    4294967342u64
}

pub fn f71() -> u64 {
    4294967297u64
}

pub fn f72() -> u64 {
    4294967310u64
}

pub fn f73() -> u64 {
    4294967311u64
}

pub fn f74() -> u64 {
    4294967312u64
}

pub fn f75() -> u64 {
    4294967338u64
}

pub fn f76(v0: u64) -> R<u64> {
    let a1235 = 8589975552u64;
    let a1236 = {
        let a1232 = v0;
        let a1233 = 4294967296u64;
        let a1234 = 0u64;
        nat_rem(a1232, a1233, a1234)
    };
    nat_add(a1235, a1236)
}

pub fn f77(v0: u64) -> R<u64> {
    let a1240 = 8589984768u64;
    let a1241 = {
        let a1237 = v0;
        let a1238 = 4294967296u64;
        let a1239 = 0u64;
        nat_rem(a1237, a1238, a1239)
    };
    nat_add(a1240, a1241)
}

pub fn f78() -> u64 {
    4294967300u64
}

pub fn f79() -> u64 {
    4294967299u64
}

pub fn f80() -> u64 {
    4294967358u64
}

pub fn f81(v0: u64) -> R<u64> {
    let a1245 = 8589992960u64;
    let a1246 = {
        let a1242 = v0;
        let a1243 = 4294967296u64;
        let a1244 = 0u64;
        nat_rem(a1242, a1243, a1244)
    };
    nat_add(a1245, a1246)
}

pub fn f82(v0: u64) -> R<u64> {
    let a1250 = 8589950976u64;
    let a1251 = {
        let a1247 = v0;
        let a1248 = 4294967296u64;
        let a1249 = 0u64;
        nat_rem(a1247, a1248, a1249)
    };
    nat_add(a1250, a1251)
}

pub fn f83(v0: u64) -> R<u64> {
    let a1255 = 8589959168u64;
    let a1256 = {
        let a1252 = v0;
        let a1253 = 4294967296u64;
        let a1254 = 0u64;
        nat_rem(a1252, a1253, a1254)
    };
    nat_add(a1255, a1256)
}

pub fn f84() -> u64 {
    4294967329u64
}

pub fn f85() -> u64 {
    4294967353u64
}

pub fn f86(v0: u64) -> R<u64> {
    let a1260 = 8589967360u64;
    let a1261 = {
        let a1257 = v0;
        let a1258 = 4294967296u64;
        let a1259 = 0u64;
        nat_rem(a1257, a1258, a1259)
    };
    nat_add(a1260, a1261)
}

pub fn f87(v0: u64) -> R<u64> {
    let a1265 = 8589976064u64;
    let a1266 = {
        let a1262 = v0;
        let a1263 = 4294967296u64;
        let a1264 = 0u64;
        nat_rem(a1262, a1263, a1264)
    };
    nat_add(a1265, a1266)
}

pub fn f88() -> u64 {
    4294967306u64
}
