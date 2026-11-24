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
    let a2 = 2u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 1u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 0u64;
            if nat_eq(a5, a6) {
                f1(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a7 = v0;
            let a8 = 1u64;
            if nat_eq(a7, a8) {
                f3(v1)
            } else {
                Ok(f2())
            }
        }
    } else {
        let a9 = v0;
        let a10 = 3u64;
        if nat_lt(a9, a10) {
            let a11 = v0;
            let a12 = 2u64;
            if nat_eq(a11, a12) {
                f4(v1)
            } else {
                Ok(f2())
            }
        } else {
            let a13 = v0;
            let a14 = 3u64;
            if nat_eq(a13, a14) {
                f5(v1)
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f1(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a15 = v0;
        let a16 = 8u64;
        let a17 = 0u64;
        nat_quot(a15, a16, a17)
    };
    let a18 = v1;
    let a19 = 2u64;
    if nat_lt(a18, a19) {
        let a20 = v1;
        let a21 = 1u64;
        if nat_lt(a20, a21) {
            let a22 = v1;
            let a23 = 0u64;
            if nat_eq(a22, a23) {
                f6({
                    let a24 = v0;
                    let a25 = 8u64;
                    let a26 = 0u64;
                    nat_rem(a24, a25, a26)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a27 = v1;
            let a28 = 1u64;
            if nat_eq(a27, a28) {
                f7({
                    let a29 = v0;
                    let a30 = 8u64;
                    let a31 = 0u64;
                    nat_rem(a29, a30, a31)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a32 = v1;
        let a33 = 3u64;
        if nat_lt(a32, a33) {
            let a34 = v1;
            let a35 = 2u64;
            if nat_eq(a34, a35) {
                f8({
                    let a36 = v0;
                    let a37 = 8u64;
                    let a38 = 0u64;
                    nat_rem(a36, a37, a38)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a39 = v1;
            let a40 = 3u64;
            if nat_eq(a39, a40) {
                Ok(f9({
                    let a41 = v0;
                    let a42 = 8u64;
                    let a43 = 0u64;
                    nat_rem(a41, a42, a43)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a44 = v0;
        let a45 = 8u64;
        let a46 = 0u64;
        nat_quot(a44, a45, a46)
    };
    let a47 = v1;
    let a48 = 1u64;
    if nat_lt(a47, a48) {
        let a49 = v1;
        let a50 = 0u64;
        if nat_eq(a49, a50) {
            f10({
                let a51 = v0;
                let a52 = 8u64;
                let a53 = 0u64;
                nat_rem(a51, a52, a53)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a54 = v1;
        let a55 = 2u64;
        if nat_lt(a54, a55) {
            let a56 = v1;
            let a57 = 1u64;
            if nat_eq(a56, a57) {
                f11({
                    let a58 = v0;
                    let a59 = 8u64;
                    let a60 = 0u64;
                    nat_rem(a58, a59, a60)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a61 = v1;
            let a62 = 2u64;
            if nat_eq(a61, a62) {
                Ok(f12({
                    let a63 = v0;
                    let a64 = 8u64;
                    let a65 = 0u64;
                    nat_rem(a63, a64, a65)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a66 = v0;
        let a67 = 8u64;
        let a68 = 0u64;
        nat_quot(a66, a67, a68)
    };
    let a69 = v1;
    let a70 = 2u64;
    if nat_lt(a69, a70) {
        let a71 = v1;
        let a72 = 1u64;
        if nat_lt(a71, a72) {
            let a73 = v1;
            let a74 = 0u64;
            if nat_eq(a73, a74) {
                f6({
                    let a75 = v0;
                    let a76 = 8u64;
                    let a77 = 0u64;
                    nat_rem(a75, a76, a77)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a78 = v1;
            let a79 = 1u64;
            if nat_eq(a78, a79) {
                f7({
                    let a80 = v0;
                    let a81 = 8u64;
                    let a82 = 0u64;
                    nat_rem(a80, a81, a82)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a83 = v1;
        let a84 = 3u64;
        if nat_lt(a83, a84) {
            let a85 = v1;
            let a86 = 2u64;
            if nat_eq(a85, a86) {
                f13({
                    let a87 = v0;
                    let a88 = 8u64;
                    let a89 = 0u64;
                    nat_rem(a87, a88, a89)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a90 = v1;
            let a91 = 3u64;
            if nat_eq(a90, a91) {
                Ok(f9({
                    let a92 = v0;
                    let a93 = 8u64;
                    let a94 = 0u64;
                    nat_rem(a92, a93, a94)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a95 = v0;
        let a96 = 8u64;
        let a97 = 0u64;
        nat_quot(a95, a96, a97)
    };
    let a98 = v1;
    let a99 = 1u64;
    if nat_lt(a98, a99) {
        let a100 = v1;
        let a101 = 0u64;
        if nat_eq(a100, a101) {
            f14({
                let a102 = v0;
                let a103 = 8u64;
                let a104 = 0u64;
                nat_rem(a102, a103, a104)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a105 = v1;
        let a106 = 2u64;
        if nat_lt(a105, a106) {
            let a107 = v1;
            let a108 = 1u64;
            if nat_eq(a107, a108) {
                f15({
                    let a109 = v0;
                    let a110 = 8u64;
                    let a111 = 0u64;
                    nat_rem(a109, a110, a111)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a112 = v1;
            let a113 = 2u64;
            if nat_eq(a112, a113) {
                Ok(f16({
                    let a114 = v0;
                    let a115 = 8u64;
                    let a116 = 0u64;
                    nat_rem(a114, a115, a116)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a117 = v0;
    let a118 = 4u64;
    if nat_lt(a117, a118) {
        let a119 = v0;
        let a120 = 2u64;
        if nat_lt(a119, a120) {
            let a121 = v0;
            let a122 = 1u64;
            if nat_lt(a121, a122) {
                let a123 = v0;
                let a124 = 0u64;
                if nat_eq(a123, a124) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a125 = v0;
                let a126 = 1u64;
                if nat_eq(a125, a126) {
                    Ok(f17(0u64, f19()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a127 = v0;
            let a128 = 3u64;
            if nat_lt(a127, a128) {
                let a129 = v0;
                let a130 = 2u64;
                if nat_eq(a129, a130) {
                    Ok(f17(0u64, f20()))
                } else {
                    Ok(f2())
                }
            } else {
                let a131 = v0;
                let a132 = 3u64;
                if nat_eq(a131, a132) {
                    Ok(f17(0u64, f21()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a133 = v0;
        let a134 = 6u64;
        if nat_lt(a133, a134) {
            let a135 = v0;
            let a136 = 5u64;
            if nat_lt(a135, a136) {
                let a137 = v0;
                let a138 = 4u64;
                if nat_eq(a137, a138) {
                    Ok(f22(0u64, f23(), 1u64, 2055u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a139 = v0;
                let a140 = 5u64;
                if nat_eq(a139, a140) {
                    Ok(f17(f24(f18())?, f25()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a141 = v0;
            let a142 = 7u64;
            if nat_lt(a141, a142) {
                let a143 = v0;
                let a144 = 6u64;
                if nat_eq(a143, a144) {
                    Ok(f26(2056u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a145 = v0;
                let a146 = 7u64;
                if nat_eq(a145, a146) {
                    Ok(f17(f27(f18())?, f25()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a147 = v0;
    let a148 = 4u64;
    if nat_lt(a147, a148) {
        let a149 = v0;
        let a150 = 2u64;
        if nat_lt(a149, a150) {
            let a151 = v0;
            let a152 = 1u64;
            if nat_lt(a151, a152) {
                let a153 = v0;
                let a154 = 0u64;
                if nat_eq(a153, a154) {
                    Ok(f17(0u64, f28()))
                } else {
                    Ok(f2())
                }
            } else {
                let a155 = v0;
                let a156 = 1u64;
                if nat_eq(a155, a156) {
                    Ok(f29(4u64, 0u64, f25(), 0u64, f30(), f31()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a157 = v0;
            let a158 = 3u64;
            if nat_lt(a157, a158) {
                let a159 = v0;
                let a160 = 2u64;
                if nat_eq(a159, a160) {
                    Ok(f32(0u64, f31(), f28(), f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a161 = v0;
                let a162 = 3u64;
                if nat_eq(a161, a162) {
                    Ok(f33(5u64, f34(f31())?, 0u64, 0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a163 = v0;
        let a164 = 6u64;
        if nat_lt(a163, a164) {
            let a165 = v0;
            let a166 = 5u64;
            if nat_lt(a165, a166) {
                let a167 = v0;
                let a168 = 4u64;
                if nat_eq(a167, a168) {
                    Ok(f36(f35(), f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a169 = v0;
                let a170 = 5u64;
                if nat_eq(a169, a170) {
                    Ok(f37(2u64, f38(f21())?, f39(f21())?, f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a171 = v0;
            let a172 = 7u64;
            if nat_lt(a171, a172) {
                let a173 = v0;
                let a174 = 6u64;
                if nat_eq(a173, a174) {
                    Ok(f37(2u64, f40(), f35(), f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a175 = v0;
                let a176 = 7u64;
                if nat_eq(a175, a176) {
                    Ok(f37(0u64, f19(), f40(), f19()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a177 = v0;
    let a178 = 4u64;
    if nat_lt(a177, a178) {
        let a179 = v0;
        let a180 = 2u64;
        if nat_lt(a179, a180) {
            let a181 = v0;
            let a182 = 1u64;
            if nat_lt(a181, a182) {
                let a183 = v0;
                let a184 = 0u64;
                if nat_eq(a183, a184) {
                    Ok(f37(2u64, f38(f21())?, f35(), f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a185 = v0;
                let a186 = 1u64;
                if nat_eq(a185, a186) {
                    Ok(f37(2u64, f40(), f35(), f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a187 = v0;
            let a188 = 3u64;
            if nat_lt(a187, a188) {
                let a189 = v0;
                let a190 = 2u64;
                if nat_eq(a189, a190) {
                    Ok(f37(0u64, f20(), f40(), f20()))
                } else {
                    Ok(f2())
                }
            } else {
                let a191 = v0;
                let a192 = 3u64;
                if nat_eq(a191, a192) {
                    Ok(f32(0u64, f28(), 1u64, f28()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a193 = v0;
        let a194 = 6u64;
        if nat_lt(a193, a194) {
            let a195 = v0;
            let a196 = 5u64;
            if nat_lt(a195, a196) {
                let a197 = v0;
                let a198 = 4u64;
                if nat_eq(a197, a198) {
                    Ok(f32(0u64, f21(), 1u64, f21()))
                } else {
                    Ok(f2())
                }
            } else {
                let a199 = v0;
                let a200 = 5u64;
                if nat_eq(a199, a200) {
                    Ok(f22(1u64, f28(), 4u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a201 = v0;
            let a202 = 7u64;
            if nat_lt(a201, a202) {
                let a203 = v0;
                let a204 = 6u64;
                if nat_eq(a203, a204) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a205 = v0;
                let a206 = 7u64;
                if nat_eq(a205, a206) {
                    Ok(f22(1u64, f18(), 8u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a207 = v0;
    let a208 = 0u64;
    if nat_eq(a207, a208) {
        f41(f42())
    } else {
        f2()
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a209 = v0;
    let a210 = 4u64;
    if nat_lt(a209, a210) {
        let a211 = v0;
        let a212 = 2u64;
        if nat_lt(a211, a212) {
            let a213 = v0;
            let a214 = 1u64;
            if nat_lt(a213, a214) {
                let a215 = v0;
                let a216 = 0u64;
                if nat_eq(a215, a216) {
                    Ok(f22(2u64, 0u64, f43(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a217 = v0;
                let a218 = 1u64;
                if nat_eq(a217, a218) {
                    Ok(f26(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a219 = v0;
            let a220 = 3u64;
            if nat_lt(a219, a220) {
                let a221 = v0;
                let a222 = 2u64;
                if nat_eq(a221, a222) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a223 = v0;
                let a224 = 3u64;
                if nat_eq(a223, a224) {
                    Ok(f22(0u64, f44(f18())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a225 = v0;
        let a226 = 6u64;
        if nat_lt(a225, a226) {
            let a227 = v0;
            let a228 = 5u64;
            if nat_lt(a227, a228) {
                let a229 = v0;
                let a230 = 4u64;
                if nat_eq(a229, a230) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a231 = v0;
                let a232 = 5u64;
                if nat_eq(a231, a232) {
                    Ok(f22(1u64, f18(), 8u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a233 = v0;
            let a234 = 7u64;
            if nat_lt(a233, a234) {
                let a235 = v0;
                let a236 = 6u64;
                if nat_eq(a235, a236) {
                    Ok(f26(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a237 = v0;
                let a238 = 7u64;
                if nat_eq(a237, a238) {
                    Ok(f37(3u64, 1065353216u64, f43(), f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a239 = v0;
    let a240 = 4u64;
    if nat_lt(a239, a240) {
        let a241 = v0;
        let a242 = 2u64;
        if nat_lt(a241, a242) {
            let a243 = v0;
            let a244 = 1u64;
            if nat_lt(a243, a244) {
                let a245 = v0;
                let a246 = 0u64;
                if nat_eq(a245, a246) {
                    Ok(f17(f43(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a247 = v0;
                let a248 = 1u64;
                if nat_eq(a247, a248) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a249 = v0;
            let a250 = 3u64;
            if nat_lt(a249, a250) {
                let a251 = v0;
                let a252 = 2u64;
                if nat_eq(a251, a252) {
                    Ok(f22(0u64, f47(), 0u64, 2572u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a253 = v0;
                let a254 = 3u64;
                if nat_eq(a253, a254) {
                    Ok(f22(0u64, f44(f18())?, 1u64, 2575u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a255 = v0;
        let a256 = 6u64;
        if nat_lt(a255, a256) {
            let a257 = v0;
            let a258 = 5u64;
            if nat_lt(a257, a258) {
                let a259 = v0;
                let a260 = 4u64;
                if nat_eq(a259, a260) {
                    Ok(f17(2574u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a261 = v0;
                let a262 = 5u64;
                if nat_eq(a261, a262) {
                    Ok(f26(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a263 = v0;
            let a264 = 7u64;
            if nat_lt(a263, a264) {
                let a265 = v0;
                let a266 = 6u64;
                if nat_eq(a265, a266) {
                    Ok(f17(f25(), f27(f18())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a267 = v0;
                let a268 = 7u64;
                if nat_eq(a267, a268) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a269 = v0;
    let a270 = 4u64;
    if nat_lt(a269, a270) {
        let a271 = v0;
        let a272 = 2u64;
        if nat_lt(a271, a272) {
            let a273 = v0;
            let a274 = 1u64;
            if nat_lt(a273, a274) {
                let a275 = v0;
                let a276 = 0u64;
                if nat_eq(a275, a276) {
                    f22(1u64, f18(), 8u64, 2570u64)
                } else {
                    f2()
                }
            } else {
                let a277 = v0;
                let a278 = 1u64;
                if nat_eq(a277, a278) {
                    f17(1u64, f23())
                } else {
                    f2()
                }
            }
        } else {
            let a279 = v0;
            let a280 = 3u64;
            if nat_lt(a279, a280) {
                let a281 = v0;
                let a282 = 2u64;
                if nat_eq(a281, a282) {
                    f17(2580u64, f42())
                } else {
                    f2()
                }
            } else {
                let a283 = v0;
                let a284 = 3u64;
                if nat_eq(a283, a284) {
                    f26(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a285 = v0;
        let a286 = 6u64;
        if nat_lt(a285, a286) {
            let a287 = v0;
            let a288 = 5u64;
            if nat_lt(a287, a288) {
                let a289 = v0;
                let a290 = 4u64;
                if nat_eq(a289, a290) {
                    f22(2u64, 0u64, f20(), 2582u64)
                } else {
                    f2()
                }
            } else {
                let a291 = v0;
                let a292 = 5u64;
                if nat_eq(a291, a292) {
                    f26(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a293 = v0;
            let a294 = 7u64;
            if nat_lt(a293, a294) {
                let a295 = v0;
                let a296 = 6u64;
                if nat_eq(a295, a296) {
                    f37(3u64, f19(), f20(), f43())
                } else {
                    f2()
                }
            } else {
                let a297 = v0;
                let a298 = 7u64;
                if nat_eq(a297, a298) {
                    f26(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a299 = v0;
    let a300 = 4u64;
    if nat_lt(a299, a300) {
        let a301 = v0;
        let a302 = 2u64;
        if nat_lt(a301, a302) {
            let a303 = v0;
            let a304 = 1u64;
            if nat_lt(a303, a304) {
                let a305 = v0;
                let a306 = 0u64;
                if nat_eq(a305, a306) {
                    Ok(f37(2u64, f38(f21())?, f35(), f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a307 = v0;
                let a308 = 1u64;
                if nat_eq(a307, a308) {
                    Ok(f37(2u64, f40(), f35(), f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a309 = v0;
            let a310 = 3u64;
            if nat_lt(a309, a310) {
                let a311 = v0;
                let a312 = 2u64;
                if nat_eq(a311, a312) {
                    Ok(f37(0u64, f20(), f40(), f20()))
                } else {
                    Ok(f2())
                }
            } else {
                let a313 = v0;
                let a314 = 3u64;
                if nat_eq(a313, a314) {
                    Ok(f32(0u64, f28(), 1u64, f28()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a315 = v0;
        let a316 = 6u64;
        if nat_lt(a315, a316) {
            let a317 = v0;
            let a318 = 5u64;
            if nat_lt(a317, a318) {
                let a319 = v0;
                let a320 = 4u64;
                if nat_eq(a319, a320) {
                    Ok(f32(0u64, f21(), 1u64, f21()))
                } else {
                    Ok(f2())
                }
            } else {
                let a321 = v0;
                let a322 = 5u64;
                if nat_eq(a321, a322) {
                    Ok(f22(1u64, f28(), 8u64, 2057u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a323 = v0;
            let a324 = 7u64;
            if nat_lt(a323, a324) {
                let a325 = v0;
                let a326 = 6u64;
                if nat_eq(a325, a326) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a327 = v0;
                let a328 = 7u64;
                if nat_eq(a327, a328) {
                    Ok(f22(1u64, f18(), 2u64, 2052u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a329 = v0;
    let a330 = 4u64;
    if nat_lt(a329, a330) {
        let a331 = v0;
        let a332 = 2u64;
        if nat_lt(a331, a332) {
            let a333 = v0;
            let a334 = 1u64;
            if nat_lt(a333, a334) {
                let a335 = v0;
                let a336 = 0u64;
                if nat_eq(a335, a336) {
                    Ok(f22(2u64, 0u64, f43(), 2562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a337 = v0;
                let a338 = 1u64;
                if nat_eq(a337, a338) {
                    Ok(f26(3072u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a339 = v0;
            let a340 = 3u64;
            if nat_lt(a339, a340) {
                let a341 = v0;
                let a342 = 2u64;
                if nat_eq(a341, a342) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a343 = v0;
                let a344 = 3u64;
                if nat_eq(a343, a344) {
                    Ok(f22(0u64, f44(f18())?, 0u64, 2567u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a345 = v0;
        let a346 = 6u64;
        if nat_lt(a345, a346) {
            let a347 = v0;
            let a348 = 5u64;
            if nat_lt(a347, a348) {
                let a349 = v0;
                let a350 = 4u64;
                if nat_eq(a349, a350) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a351 = v0;
                let a352 = 5u64;
                if nat_eq(a351, a352) {
                    Ok(f22(1u64, f18(), 2u64, 2563u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a353 = v0;
            let a354 = 7u64;
            if nat_lt(a353, a354) {
                let a355 = v0;
                let a356 = 6u64;
                if nat_eq(a355, a356) {
                    Ok(f26(3072u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a357 = v0;
                let a358 = 7u64;
                if nat_eq(a357, a358) {
                    Ok(f37(3u64, 1065353216u64, f43(), f45()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a359 = v0;
    let a360 = 4u64;
    if nat_lt(a359, a360) {
        let a361 = v0;
        let a362 = 2u64;
        if nat_lt(a361, a362) {
            let a363 = v0;
            let a364 = 1u64;
            if nat_lt(a363, a364) {
                let a365 = v0;
                let a366 = 0u64;
                if nat_eq(a365, a366) {
                    Ok(f17(f43(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a367 = v0;
                let a368 = 1u64;
                if nat_eq(a367, a368) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a369 = v0;
            let a370 = 3u64;
            if nat_lt(a369, a370) {
                let a371 = v0;
                let a372 = 2u64;
                if nat_eq(a371, a372) {
                    Ok(f22(0u64, f44(f18())?, 1u64, 2574u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a373 = v0;
                let a374 = 3u64;
                if nat_eq(a373, a374) {
                    Ok(f17(2573u64, f48()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a375 = v0;
        let a376 = 6u64;
        if nat_lt(a375, a376) {
            let a377 = v0;
            let a378 = 5u64;
            if nat_lt(a377, a378) {
                let a379 = v0;
                let a380 = 4u64;
                if nat_eq(a379, a380) {
                    Ok(f26(1536u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a381 = v0;
                let a382 = 5u64;
                if nat_eq(a381, a382) {
                    Ok(f17(f25(), f27(f18())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a383 = v0;
            let a384 = 7u64;
            if nat_lt(a383, a384) {
                let a385 = v0;
                let a386 = 6u64;
                if nat_eq(a385, a386) {
                    Ok(f32(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f2())
                }
            } else {
                let a387 = v0;
                let a388 = 7u64;
                if nat_eq(a387, a388) {
                    Ok(f22(1u64, f18(), 2u64, 2570u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a389 = v0;
    let a390 = 3u64;
    if nat_lt(a389, a390) {
        let a391 = v0;
        let a392 = 1u64;
        if nat_lt(a391, a392) {
            let a393 = v0;
            let a394 = 0u64;
            if nat_eq(a393, a394) {
                f17(1u64, f23())
            } else {
                f2()
            }
        } else {
            let a395 = v0;
            let a396 = 2u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 1u64;
                if nat_eq(a397, a398) {
                    f17(2579u64, f42())
                } else {
                    f2()
                }
            } else {
                let a399 = v0;
                let a400 = 2u64;
                if nat_eq(a399, a400) {
                    f26(2048u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a401 = v0;
        let a402 = 5u64;
        if nat_lt(a401, a402) {
            let a403 = v0;
            let a404 = 4u64;
            if nat_lt(a403, a404) {
                let a405 = v0;
                let a406 = 3u64;
                if nat_eq(a405, a406) {
                    f22(2u64, 0u64, f20(), 2581u64)
                } else {
                    f2()
                }
            } else {
                let a407 = v0;
                let a408 = 4u64;
                if nat_eq(a407, a408) {
                    f26(3072u64)
                } else {
                    f2()
                }
            }
        } else {
            let a409 = v0;
            let a410 = 6u64;
            if nat_lt(a409, a410) {
                let a411 = v0;
                let a412 = 5u64;
                if nat_eq(a411, a412) {
                    f37(3u64, f19(), f20(), f43())
                } else {
                    f2()
                }
            } else {
                let a413 = v0;
                let a414 = 6u64;
                if nat_eq(a413, a414) {
                    f26(3072u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f18() -> u64 {
    4294967323u64
}

pub fn f19() -> u64 {
    4294967335u64
}

pub fn f20() -> u64 {
    4294967336u64
}

pub fn f21() -> u64 {
    4294967303u64
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f23() -> u64 {
    4294967324u64
}

pub fn f24(v0: u64) -> R<u64> {
    let a418 = 8589976576u64;
    let a419 = {
        let a415 = v0;
        let a416 = 4294967296u64;
        let a417 = 0u64;
        nat_rem(a415, a416, a417)
    };
    nat_add(a418, a419)
}

pub fn f25() -> u64 {
    4294967321u64
}

pub fn f26(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f27(v0: u64) -> R<u64> {
    let a423 = 8589980672u64;
    let a424 = {
        let a420 = v0;
        let a421 = 4294967296u64;
        let a422 = 0u64;
        nat_rem(a420, a421, a422)
    };
    nat_add(a423, a424)
}

pub fn f28() -> u64 {
    4294967305u64
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f30() -> u64 {
    4294967338u64
}

pub fn f31() -> u64 {
    4294967330u64
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f34(v0: u64) -> R<u64> {
    let a428 = 25769803776u64;
    let a429 = {
        let a425 = v0;
        let a426 = 4294967296u64;
        let a427 = 0u64;
        nat_rem(a425, a426, a427)
    };
    nat_add(a428, a429)
}

pub fn f35() -> u64 {
    4294967332u64
}

pub fn f36(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f38(v0: u64) -> R<u64> {
    let a433 = 8589959168u64;
    let a434 = {
        let a430 = v0;
        let a431 = 4294967296u64;
        let a432 = 0u64;
        nat_rem(a430, a431, a432)
    };
    nat_add(a433, a434)
}

pub fn f39(v0: u64) -> R<u64> {
    let a438 = 8589950976u64;
    let a439 = {
        let a435 = v0;
        let a436 = 4294967296u64;
        let a437 = 0u64;
        nat_rem(a435, a436, a437)
    };
    nat_add(a438, a439)
}

pub fn f40() -> u64 {
    4294967337u64
}

pub fn f41(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f42() -> u64 {
    4294967351u64
}

pub fn f43() -> u64 {
    4294967316u64
}

pub fn f44(v0: u64) -> R<u64> {
    let a443 = 8589982720u64;
    let a444 = {
        let a440 = v0;
        let a441 = 4294967296u64;
        let a442 = 0u64;
        nat_rem(a440, a441, a442)
    };
    nat_add(a443, a444)
}

pub fn f45() -> u64 {
    4294967319u64
}

pub fn f46() -> u64 {
    4294967320u64
}

pub fn f47() -> u64 {
    4294967297u64
}

pub fn f48() -> u64 {
    4294967350u64
}
