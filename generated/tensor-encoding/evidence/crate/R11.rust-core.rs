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
                Ok(f7({
                    let a29 = v0;
                    let a30 = 8u64;
                    let a31 = 0u64;
                    nat_rem(a29, a30, a31)
                }))
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
            let a40 = 4u64;
            if nat_lt(a39, a40) {
                let a41 = v1;
                let a42 = 3u64;
                if nat_eq(a41, a42) {
                    f9({
                        let a43 = v0;
                        let a44 = 8u64;
                        let a45 = 0u64;
                        nat_rem(a43, a44, a45)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a46 = v1;
                let a47 = 4u64;
                if nat_eq(a46, a47) {
                    f10({
                        let a48 = v0;
                        let a49 = 8u64;
                        let a50 = 0u64;
                        nat_rem(a48, a49, a50)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f2() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f3(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a51 = v0;
        let a52 = 8u64;
        let a53 = 0u64;
        nat_quot(a51, a52, a53)
    };
    let a54 = v1;
    let a55 = 2u64;
    if nat_lt(a54, a55) {
        let a56 = v1;
        let a57 = 1u64;
        if nat_lt(a56, a57) {
            let a58 = v1;
            let a59 = 0u64;
            if nat_eq(a58, a59) {
                Ok(f11({
                    let a60 = v0;
                    let a61 = 8u64;
                    let a62 = 0u64;
                    nat_rem(a60, a61, a62)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a63 = v1;
            let a64 = 1u64;
            if nat_eq(a63, a64) {
                f12({
                    let a65 = v0;
                    let a66 = 8u64;
                    let a67 = 0u64;
                    nat_rem(a65, a66, a67)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a68 = v1;
        let a69 = 3u64;
        if nat_lt(a68, a69) {
            let a70 = v1;
            let a71 = 2u64;
            if nat_eq(a70, a71) {
                Ok(f13({
                    let a72 = v0;
                    let a73 = 8u64;
                    let a74 = 0u64;
                    nat_rem(a72, a73, a74)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a75 = v1;
            let a76 = 3u64;
            if nat_eq(a75, a76) {
                Ok(f14({
                    let a77 = v0;
                    let a78 = 8u64;
                    let a79 = 0u64;
                    nat_rem(a77, a78, a79)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a80 = v0;
        let a81 = 8u64;
        let a82 = 0u64;
        nat_quot(a80, a81, a82)
    };
    let a83 = v1;
    let a84 = 2u64;
    if nat_lt(a83, a84) {
        let a85 = v1;
        let a86 = 1u64;
        if nat_lt(a85, a86) {
            let a87 = v1;
            let a88 = 0u64;
            if nat_eq(a87, a88) {
                f15({
                    let a89 = v0;
                    let a90 = 8u64;
                    let a91 = 0u64;
                    nat_rem(a89, a90, a91)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a92 = v1;
            let a93 = 1u64;
            if nat_eq(a92, a93) {
                f16({
                    let a94 = v0;
                    let a95 = 8u64;
                    let a96 = 0u64;
                    nat_rem(a94, a95, a96)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a97 = v1;
        let a98 = 3u64;
        if nat_lt(a97, a98) {
            let a99 = v1;
            let a100 = 2u64;
            if nat_eq(a99, a100) {
                f17({
                    let a101 = v0;
                    let a102 = 8u64;
                    let a103 = 0u64;
                    nat_rem(a101, a102, a103)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a104 = v1;
            let a105 = 3u64;
            if nat_eq(a104, a105) {
                Ok(f18({
                    let a106 = v0;
                    let a107 = 8u64;
                    let a108 = 0u64;
                    nat_rem(a106, a107, a108)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a109 = v0;
        let a110 = 8u64;
        let a111 = 0u64;
        nat_quot(a109, a110, a111)
    };
    let a112 = v1;
    let a113 = 1u64;
    if nat_lt(a112, a113) {
        let a114 = v1;
        let a115 = 0u64;
        if nat_eq(a114, a115) {
            f19({
                let a116 = v0;
                let a117 = 8u64;
                let a118 = 0u64;
                nat_rem(a116, a117, a118)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a119 = v1;
        let a120 = 2u64;
        if nat_lt(a119, a120) {
            let a121 = v1;
            let a122 = 1u64;
            if nat_eq(a121, a122) {
                f20({
                    let a123 = v0;
                    let a124 = 8u64;
                    let a125 = 0u64;
                    nat_rem(a123, a124, a125)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a126 = v1;
            let a127 = 2u64;
            if nat_eq(a126, a127) {
                Ok(f21({
                    let a128 = v0;
                    let a129 = 8u64;
                    let a130 = 0u64;
                    nat_rem(a128, a129, a130)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a131 = v0;
    let a132 = 4u64;
    if nat_lt(a131, a132) {
        let a133 = v0;
        let a134 = 2u64;
        if nat_lt(a133, a134) {
            let a135 = v0;
            let a136 = 1u64;
            if nat_lt(a135, a136) {
                let a137 = v0;
                let a138 = 0u64;
                if nat_eq(a137, a138) {
                    Ok(f22(2u64, f23(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a139 = v0;
                let a140 = 1u64;
                if nat_eq(a139, a140) {
                    Ok(f24(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a141 = v0;
            let a142 = 3u64;
            if nat_lt(a141, a142) {
                let a143 = v0;
                let a144 = 2u64;
                if nat_eq(a143, a144) {
                    Ok(f25(6u64, f23(), 0u64, f23()))
                } else {
                    Ok(f2())
                }
            } else {
                let a145 = v0;
                let a146 = 3u64;
                if nat_eq(a145, a146) {
                    Ok(f26(0u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a147 = v0;
        let a148 = 6u64;
        if nat_lt(a147, a148) {
            let a149 = v0;
            let a150 = 5u64;
            if nat_lt(a149, a150) {
                let a151 = v0;
                let a152 = 4u64;
                if nat_eq(a151, a152) {
                    Ok(f28(1u64, 255u64, f29(f27())?, f29(f27())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a153 = v0;
                let a154 = 5u64;
                if nat_eq(a153, a154) {
                    Ok(f28(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a155 = v0;
            let a156 = 7u64;
            if nat_lt(a155, a156) {
                let a157 = v0;
                let a158 = 6u64;
                if nat_eq(a157, a158) {
                    Ok(f22(1u64, f27(), 4u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a159 = v0;
                let a160 = 7u64;
                if nat_eq(a159, a160) {
                    Ok(f26(f23(), f30(f31())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a161 = v0;
    let a162 = 4u64;
    if nat_lt(a161, a162) {
        let a163 = v0;
        let a164 = 2u64;
        if nat_lt(a163, a164) {
            let a165 = v0;
            let a166 = 1u64;
            if nat_lt(a165, a166) {
                let a167 = v0;
                let a168 = 0u64;
                if nat_eq(a167, a168) {
                    f22(2u64, f32(), f23(), 3082u64)
                } else {
                    f2()
                }
            } else {
                let a169 = v0;
                let a170 = 1u64;
                if nat_eq(a169, a170) {
                    f24(3083u64)
                } else {
                    f2()
                }
            }
        } else {
            let a171 = v0;
            let a172 = 3u64;
            if nat_lt(a171, a172) {
                let a173 = v0;
                let a174 = 2u64;
                if nat_eq(a173, a174) {
                    f26(f23(), f32())
                } else {
                    f2()
                }
            } else {
                let a175 = v0;
                let a176 = 3u64;
                if nat_eq(a175, a176) {
                    f26(f33(), f34())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a177 = v0;
        let a178 = 6u64;
        if nat_lt(a177, a178) {
            let a179 = v0;
            let a180 = 5u64;
            if nat_lt(a179, a180) {
                let a181 = v0;
                let a182 = 4u64;
                if nat_eq(a181, a182) {
                    f22(0u64, f35(), 0u64, 3086u64)
                } else {
                    f2()
                }
            } else {
                let a183 = v0;
                let a184 = 5u64;
                if nat_eq(a183, a184) {
                    f26(f31(), f34())
                } else {
                    f2()
                }
            }
        } else {
            let a185 = v0;
            let a186 = 7u64;
            if nat_lt(a185, a186) {
                let a187 = v0;
                let a188 = 6u64;
                if nat_eq(a187, a188) {
                    f28(0u64, f33(), 1u64, f33())
                } else {
                    f2()
                }
            } else {
                let a189 = v0;
                let a190 = 7u64;
                if nat_eq(a189, a190) {
                    f26(0u64, f36())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a191 = v0;
    let a192 = 4u64;
    if nat_lt(a191, a192) {
        let a193 = v0;
        let a194 = 2u64;
        if nat_lt(a193, a194) {
            let a195 = v0;
            let a196 = 1u64;
            if nat_lt(a195, a196) {
                let a197 = v0;
                let a198 = 0u64;
                if nat_eq(a197, a198) {
                    Ok(f28(2u64, f34(), 8u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a199 = v0;
                let a200 = 1u64;
                if nat_eq(a199, a200) {
                    Ok(f28(0u64, f37(), f36(), f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a201 = v0;
            let a202 = 3u64;
            if nat_lt(a201, a202) {
                let a203 = v0;
                let a204 = 2u64;
                if nat_eq(a203, a204) {
                    Ok(f38(7u64, f37(), 0u64, f39(f36())?, f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a205 = v0;
                let a206 = 3u64;
                if nat_eq(a205, a206) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a207 = v0;
        let a208 = 6u64;
        if nat_lt(a207, a208) {
            let a209 = v0;
            let a210 = 5u64;
            if nat_lt(a209, a210) {
                let a211 = v0;
                let a212 = 4u64;
                if nat_eq(a211, a212) {
                    Ok(f22(0u64, f35(), 1u64, 3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a213 = v0;
                let a214 = 5u64;
                if nat_eq(a213, a214) {
                    Ok(f28(2u64, f31(), 8u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a215 = v0;
            let a216 = 7u64;
            if nat_lt(a215, a216) {
                let a217 = v0;
                let a218 = 6u64;
                if nat_eq(a217, a218) {
                    Ok(f28(0u64, f37(), f36(), f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a219 = v0;
                let a220 = 7u64;
                if nat_eq(a219, a220) {
                    Ok(f38(7u64, f37(), 1u64, f39(f36())?, f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a221 = v0;
    let a222 = 4u64;
    if nat_lt(a221, a222) {
        let a223 = v0;
        let a224 = 2u64;
        if nat_lt(a223, a224) {
            let a225 = v0;
            let a226 = 1u64;
            if nat_lt(a225, a226) {
                let a227 = v0;
                let a228 = 0u64;
                if nat_eq(a227, a228) {
                    Ok(f44(f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a229 = v0;
                let a230 = 1u64;
                if nat_eq(a229, a230) {
                    Ok(f28(0u64, f36(), 1u64, f36()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a231 = v0;
            let a232 = 3u64;
            if nat_lt(a231, a232) {
                let a233 = v0;
                let a234 = 2u64;
                if nat_eq(a233, a234) {
                    Ok(f22(1u64, f36(), 8u64, 3088u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a235 = v0;
                let a236 = 3u64;
                if nat_eq(a235, a236) {
                    Ok(f26(0u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a237 = v0;
        let a238 = 6u64;
        if nat_lt(a237, a238) {
            let a239 = v0;
            let a240 = 5u64;
            if nat_lt(a239, a240) {
                let a241 = v0;
                let a242 = 4u64;
                if nat_eq(a241, a242) {
                    Ok(f45(9u64, f34(), f27(), 0u64, f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a243 = v0;
                let a244 = 5u64;
                if nat_eq(a243, a244) {
                    Ok(f26(f29(f27())?, f42()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a245 = v0;
            let a246 = 7u64;
            if nat_lt(a245, a246) {
                let a247 = v0;
                let a248 = 6u64;
                if nat_eq(a247, a248) {
                    Ok(f22(0u64, f35(), 0u64, 3106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a249 = v0;
                let a250 = 7u64;
                if nat_eq(a249, a250) {
                    Ok(f28(4u64, f42(), 128u64, f42()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a251 = v0;
    let a252 = 4u64;
    if nat_lt(a251, a252) {
        let a253 = v0;
        let a254 = 2u64;
        if nat_lt(a253, a254) {
            let a255 = v0;
            let a256 = 1u64;
            if nat_lt(a255, a256) {
                let a257 = v0;
                let a258 = 0u64;
                if nat_eq(a257, a258) {
                    Ok(f46(f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a259 = v0;
                let a260 = 1u64;
                if nat_eq(a259, a260) {
                    Ok(f24(3107u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a261 = v0;
            let a262 = 3u64;
            if nat_lt(a261, a262) {
                let a263 = v0;
                let a264 = 2u64;
                if nat_eq(a263, a264) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a265 = v0;
                let a266 = 3u64;
                if nat_eq(a265, a266) {
                    Ok(f28(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a267 = v0;
        let a268 = 6u64;
        if nat_lt(a267, a268) {
            let a269 = v0;
            let a270 = 5u64;
            if nat_lt(a269, a270) {
                let a271 = v0;
                let a272 = 4u64;
                if nat_eq(a271, a272) {
                    Ok(f22(1u64, f27(), 4u64, 3100u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a273 = v0;
                let a274 = 5u64;
                if nat_eq(a273, a274) {
                    Ok(f28(0u64, f31(), 1u64, f31()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a275 = v0;
            let a276 = 7u64;
            if nat_lt(a275, a276) {
                let a277 = v0;
                let a278 = 6u64;
                if nat_eq(a277, a278) {
                    Ok(f22(1u64, f31(), 8u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a279 = v0;
                let a280 = 7u64;
                if nat_eq(a279, a280) {
                    Ok(f24(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a281 = v0;
    let a282 = 4u64;
    if nat_lt(a281, a282) {
        let a283 = v0;
        let a284 = 2u64;
        if nat_lt(a283, a284) {
            let a285 = v0;
            let a286 = 1u64;
            if nat_lt(a285, a286) {
                let a287 = v0;
                let a288 = 0u64;
                if nat_eq(a287, a288) {
                    f22(0u64, f32(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a289 = v0;
                let a290 = 1u64;
                if nat_eq(a289, a290) {
                    f25(3u64, f32(), 1106771968u64, f23())
                } else {
                    f2()
                }
            }
        } else {
            let a291 = v0;
            let a292 = 3u64;
            if nat_lt(a291, a292) {
                let a293 = v0;
                let a294 = 2u64;
                if nat_eq(a293, a294) {
                    f26(1065458074u64, f47())
                } else {
                    f2()
                }
            } else {
                let a295 = v0;
                let a296 = 3u64;
                if nat_eq(a295, a296) {
                    f22(0u64, f35(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a297 = v0;
        let a298 = 6u64;
        if nat_lt(a297, a298) {
            let a299 = v0;
            let a300 = 5u64;
            if nat_lt(a299, a300) {
                let a301 = v0;
                let a302 = 4u64;
                if nat_eq(a301, a302) {
                    f26(1065630040u64, f47())
                } else {
                    f2()
                }
            } else {
                let a303 = v0;
                let a304 = 5u64;
                if nat_eq(a303, a304) {
                    f25(2u64, f23(), f47(), f48())
                } else {
                    f2()
                }
            }
        } else {
            let a305 = v0;
            let a306 = 7u64;
            if nat_lt(a305, a306) {
                let a307 = v0;
                let a308 = 6u64;
                if nat_eq(a307, a308) {
                    f49(f48(), f48())
                } else {
                    f2()
                }
            } else {
                let a309 = v0;
                let a310 = 7u64;
                if nat_eq(a309, a310) {
                    f28(4u64, f48(), 256u64, f42())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a311 = v0;
    let a312 = 4u64;
    if nat_lt(a311, a312) {
        let a313 = v0;
        let a314 = 2u64;
        if nat_lt(a313, a314) {
            let a315 = v0;
            let a316 = 1u64;
            if nat_lt(a315, a316) {
                let a317 = v0;
                let a318 = 0u64;
                if nat_eq(a317, a318) {
                    Ok(f26(0u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a319 = v0;
                let a320 = 1u64;
                if nat_eq(a319, a320) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a321 = v0;
            let a322 = 3u64;
            if nat_lt(a321, a322) {
                let a323 = v0;
                let a324 = 2u64;
                if nat_eq(a323, a324) {
                    Ok(f28(3u64, f48(), 256u64, f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a325 = v0;
                let a326 = 3u64;
                if nat_eq(a325, a326) {
                    Ok(f28(0u64, f40(), 1u64, f40()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a327 = v0;
        let a328 = 6u64;
        if nat_lt(a327, a328) {
            let a329 = v0;
            let a330 = 5u64;
            if nat_lt(a329, a330) {
                let a331 = v0;
                let a332 = 4u64;
                if nat_eq(a331, a332) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a333 = v0;
                let a334 = 5u64;
                if nat_eq(a333, a334) {
                    Ok(f25(3u64, 1065353216u64, f23(), f50()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a335 = v0;
            let a336 = 7u64;
            if nat_lt(a335, a336) {
                let a337 = v0;
                let a338 = 6u64;
                if nat_eq(a337, a338) {
                    Ok(f26(0u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a339 = v0;
                let a340 = 7u64;
                if nat_eq(a339, a340) {
                    Ok(f25(2u64, f50(), f30(f31())?, f51()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a341 = v0;
    let a342 = 4u64;
    if nat_lt(a341, a342) {
        let a343 = v0;
        let a344 = 2u64;
        if nat_lt(a343, a344) {
            let a345 = v0;
            let a346 = 1u64;
            if nat_lt(a345, a346) {
                let a347 = v0;
                let a348 = 0u64;
                if nat_eq(a347, a348) {
                    f25(1u64, f51(), 1065353216u64, f51())
                } else {
                    f2()
                }
            } else {
                let a349 = v0;
                let a350 = 1u64;
                if nat_eq(a349, a350) {
                    f25(2u64, 1056964608u64, f51(), f51())
                } else {
                    f2()
                }
            }
        } else {
            let a351 = v0;
            let a352 = 3u64;
            if nat_lt(a351, a352) {
                let a353 = v0;
                let a354 = 2u64;
                if nat_eq(a353, a354) {
                    f25(0u64, f51(), 1262485504u64, f51())
                } else {
                    f2()
                }
            } else {
                let a355 = v0;
                let a356 = 3u64;
                if nat_eq(a355, a356) {
                    f52(2u64, f51(), 15u64, 0u64, f42())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a357 = v0;
        let a358 = 6u64;
        if nat_lt(a357, a358) {
            let a359 = v0;
            let a360 = 5u64;
            if nat_lt(a359, a360) {
                let a361 = v0;
                let a362 = 4u64;
                if nat_eq(a361, a362) {
                    f45(8u64, f31(), 0u64, 0u64, f40(), f41())
                } else {
                    f2()
                }
            } else {
                let a363 = v0;
                let a364 = 5u64;
                if nat_eq(a363, a364) {
                    f44(f40(), f41(), f42())
                } else {
                    f2()
                }
            }
        } else {
            let a365 = v0;
            let a366 = 7u64;
            if nat_lt(a365, a366) {
                let a367 = v0;
                let a368 = 6u64;
                if nat_eq(a367, a368) {
                    f28(0u64, f31(), 1u64, f31())
                } else {
                    f2()
                }
            } else {
                let a369 = v0;
                let a370 = 7u64;
                if nat_eq(a369, a370) {
                    f22(1u64, f31(), 8u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a371 = v0;
    let a372 = 1u64;
    if nat_lt(a371, a372) {
        let a373 = v0;
        let a374 = 0u64;
        if nat_eq(a373, a374) {
            f53()
        } else {
            f2()
        }
    } else {
        let a375 = v0;
        let a376 = 1u64;
        if nat_eq(a375, a376) {
            f54()
        } else {
            f2()
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a377 = v0;
    let a378 = 4u64;
    if nat_lt(a377, a378) {
        let a379 = v0;
        let a380 = 2u64;
        if nat_lt(a379, a380) {
            let a381 = v0;
            let a382 = 1u64;
            if nat_lt(a381, a382) {
                let a383 = v0;
                let a384 = 0u64;
                if nat_eq(a383, a384) {
                    Ok(f22(2u64, f23(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a385 = v0;
                let a386 = 1u64;
                if nat_eq(a385, a386) {
                    Ok(f24(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a387 = v0;
            let a388 = 3u64;
            if nat_lt(a387, a388) {
                let a389 = v0;
                let a390 = 2u64;
                if nat_eq(a389, a390) {
                    Ok(f25(6u64, f23(), 0u64, f23()))
                } else {
                    Ok(f2())
                }
            } else {
                let a391 = v0;
                let a392 = 3u64;
                if nat_eq(a391, a392) {
                    Ok(f26(0u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a393 = v0;
        let a394 = 6u64;
        if nat_lt(a393, a394) {
            let a395 = v0;
            let a396 = 5u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 4u64;
                if nat_eq(a397, a398) {
                    Ok(f28(1u64, 255u64, f29(f27())?, f29(f27())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a399 = v0;
                let a400 = 5u64;
                if nat_eq(a399, a400) {
                    Ok(f28(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a401 = v0;
            let a402 = 7u64;
            if nat_lt(a401, a402) {
                let a403 = v0;
                let a404 = 6u64;
                if nat_eq(a403, a404) {
                    Ok(f22(1u64, f27(), 2u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a405 = v0;
                let a406 = 7u64;
                if nat_eq(a405, a406) {
                    Ok(f26(f23(), f30(f31())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a407 = v0;
    let a408 = 4u64;
    if nat_lt(a407, a408) {
        let a409 = v0;
        let a410 = 2u64;
        if nat_lt(a409, a410) {
            let a411 = v0;
            let a412 = 1u64;
            if nat_lt(a411, a412) {
                let a413 = v0;
                let a414 = 0u64;
                if nat_eq(a413, a414) {
                    Ok(f22(2u64, f32(), f23(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a415 = v0;
                let a416 = 1u64;
                if nat_eq(a415, a416) {
                    Ok(f24(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a417 = v0;
            let a418 = 3u64;
            if nat_lt(a417, a418) {
                let a419 = v0;
                let a420 = 2u64;
                if nat_eq(a419, a420) {
                    Ok(f26(f23(), f32()))
                } else {
                    Ok(f2())
                }
            } else {
                let a421 = v0;
                let a422 = 3u64;
                if nat_eq(a421, a422) {
                    Ok(f26(0u64, f36()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a423 = v0;
        let a424 = 6u64;
        if nat_lt(a423, a424) {
            let a425 = v0;
            let a426 = 5u64;
            if nat_lt(a425, a426) {
                let a427 = v0;
                let a428 = 4u64;
                if nat_eq(a427, a428) {
                    Ok(f28(2u64, f31(), 2u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a429 = v0;
                let a430 = 5u64;
                if nat_eq(a429, a430) {
                    Ok(f28(0u64, f37(), f36(), f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a431 = v0;
            let a432 = 7u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 6u64;
                if nat_eq(a433, a434) {
                    Ok(f38(7u64, f37(), 0u64, f39(f36())?, f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 7u64;
                if nat_eq(a435, a436) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a437 = v0;
    let a438 = 4u64;
    if nat_lt(a437, a438) {
        let a439 = v0;
        let a440 = 2u64;
        if nat_lt(a439, a440) {
            let a441 = v0;
            let a442 = 1u64;
            if nat_lt(a441, a442) {
                let a443 = v0;
                let a444 = 0u64;
                if nat_eq(a443, a444) {
                    Ok(f38(7u64, f37(), 1u64, f39(f36())?, f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a445 = v0;
                let a446 = 1u64;
                if nat_eq(a445, a446) {
                    Ok(f44(f40(), f41(), f42()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a447 = v0;
            let a448 = 3u64;
            if nat_lt(a447, a448) {
                let a449 = v0;
                let a450 = 2u64;
                if nat_eq(a449, a450) {
                    Ok(f28(0u64, f37(), 34u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a451 = v0;
                let a452 = 3u64;
                if nat_eq(a451, a452) {
                    Ok(f26(f29(f36())?, f43(f40())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a453 = v0;
        let a454 = 6u64;
        if nat_lt(a453, a454) {
            let a455 = v0;
            let a456 = 5u64;
            if nat_lt(a455, a456) {
                let a457 = v0;
                let a458 = 4u64;
                if nat_eq(a457, a458) {
                    Ok(f28(0u64, f36(), 1u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a459 = v0;
                let a460 = 5u64;
                if nat_eq(a459, a460) {
                    Ok(f22(1u64, f36(), 2u64, 3084u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a461 = v0;
            let a462 = 7u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 6u64;
                if nat_eq(a463, a464) {
                    Ok(f28(0u64, f31(), 1u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a465 = v0;
                let a466 = 7u64;
                if nat_eq(a465, a466) {
                    Ok(f22(1u64, f31(), 16u64, 512u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a467 = v0;
    let a468 = 0u64;
    if nat_eq(a467, a468) {
        f24(3584u64)
    } else {
        f2()
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a469 = v0;
    let a470 = 4u64;
    if nat_lt(a469, a470) {
        let a471 = v0;
        let a472 = 2u64;
        if nat_lt(a471, a472) {
            let a473 = v0;
            let a474 = 1u64;
            if nat_lt(a473, a474) {
                let a475 = v0;
                let a476 = 0u64;
                if nat_eq(a475, a476) {
                    Ok(f22(0u64, f32(), 0u64, 3607u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a477 = v0;
                let a478 = 1u64;
                if nat_eq(a477, a478) {
                    Ok(f25(3u64, f32(), 1106771968u64, f23()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a479 = v0;
            let a480 = 3u64;
            if nat_lt(a479, a480) {
                let a481 = v0;
                let a482 = 2u64;
                if nat_eq(a481, a482) {
                    Ok(f26(1065143501u64, f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a483 = v0;
                let a484 = 3u64;
                if nat_eq(a483, a484) {
                    Ok(f25(2u64, f23(), f47(), f48()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a485 = v0;
        let a486 = 6u64;
        if nat_lt(a485, a486) {
            let a487 = v0;
            let a488 = 5u64;
            if nat_lt(a487, a488) {
                let a489 = v0;
                let a490 = 4u64;
                if nat_eq(a489, a490) {
                    Ok(f49(f48(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a491 = v0;
                let a492 = 5u64;
                if nat_eq(a491, a492) {
                    Ok(f28(4u64, f48(), 256u64, f42()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a493 = v0;
            let a494 = 7u64;
            if nat_lt(a493, a494) {
                let a495 = v0;
                let a496 = 6u64;
                if nat_eq(a495, a496) {
                    Ok(f26(0u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a497 = v0;
                let a498 = 7u64;
                if nat_eq(a497, a498) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a499 = v0;
    let a500 = 4u64;
    if nat_lt(a499, a500) {
        let a501 = v0;
        let a502 = 2u64;
        if nat_lt(a501, a502) {
            let a503 = v0;
            let a504 = 1u64;
            if nat_lt(a503, a504) {
                let a505 = v0;
                let a506 = 0u64;
                if nat_eq(a505, a506) {
                    Ok(f28(3u64, f48(), 256u64, f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a507 = v0;
                let a508 = 1u64;
                if nat_eq(a507, a508) {
                    Ok(f28(0u64, f40(), 1u64, f40()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a509 = v0;
            let a510 = 3u64;
            if nat_lt(a509, a510) {
                let a511 = v0;
                let a512 = 2u64;
                if nat_eq(a511, a512) {
                    Ok(f26(f42(), f43(f40())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a513 = v0;
                let a514 = 3u64;
                if nat_eq(a513, a514) {
                    Ok(f25(3u64, 1065353216u64, f23(), f50()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a515 = v0;
        let a516 = 6u64;
        if nat_lt(a515, a516) {
            let a517 = v0;
            let a518 = 5u64;
            if nat_lt(a517, a518) {
                let a519 = v0;
                let a520 = 4u64;
                if nat_eq(a519, a520) {
                    Ok(f26(0u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a521 = v0;
                let a522 = 5u64;
                if nat_eq(a521, a522) {
                    Ok(f25(2u64, f50(), f30(f31())?, f51()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a523 = v0;
            let a524 = 7u64;
            if nat_lt(a523, a524) {
                let a525 = v0;
                let a526 = 6u64;
                if nat_eq(a525, a526) {
                    Ok(f25(1u64, f51(), 1065353216u64, f51()))
                } else {
                    Ok(f2())
                }
            } else {
                let a527 = v0;
                let a528 = 7u64;
                if nat_eq(a527, a528) {
                    Ok(f25(2u64, 1056964608u64, f51(), f51()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a529 = v0;
    let a530 = 4u64;
    if nat_lt(a529, a530) {
        let a531 = v0;
        let a532 = 2u64;
        if nat_lt(a531, a532) {
            let a533 = v0;
            let a534 = 1u64;
            if nat_lt(a533, a534) {
                let a535 = v0;
                let a536 = 0u64;
                if nat_eq(a535, a536) {
                    f25(0u64, f51(), 1262485504u64, f51())
                } else {
                    f2()
                }
            } else {
                let a537 = v0;
                let a538 = 1u64;
                if nat_eq(a537, a538) {
                    f52(2u64, f51(), 15u64, 0u64, f42())
                } else {
                    f2()
                }
            }
        } else {
            let a539 = v0;
            let a540 = 3u64;
            if nat_lt(a539, a540) {
                let a541 = v0;
                let a542 = 2u64;
                if nat_eq(a541, a542) {
                    f45(8u64, f31(), 0u64, 0u64, f40(), f41())
                } else {
                    f2()
                }
            } else {
                let a543 = v0;
                let a544 = 3u64;
                if nat_eq(a543, a544) {
                    f44(f40(), f41(), f42())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a545 = v0;
        let a546 = 6u64;
        if nat_lt(a545, a546) {
            let a547 = v0;
            let a548 = 5u64;
            if nat_lt(a547, a548) {
                let a549 = v0;
                let a550 = 4u64;
                if nat_eq(a549, a550) {
                    f28(0u64, f31(), 1u64, f31())
                } else {
                    f2()
                }
            } else {
                let a551 = v0;
                let a552 = 5u64;
                if nat_eq(a551, a552) {
                    f22(1u64, f31(), 16u64, 3597u64)
                } else {
                    f2()
                }
            }
        } else {
            let a553 = v0;
            let a554 = 7u64;
            if nat_lt(a553, a554) {
                let a555 = v0;
                let a556 = 6u64;
                if nat_eq(a555, a556) {
                    f53()
                } else {
                    f2()
                }
            } else {
                let a557 = v0;
                let a558 = 7u64;
                if nat_eq(a557, a558) {
                    f54()
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f23() -> u64 {
    4294967316u64
}

pub fn f24(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f26(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f27() -> u64 {
    4294967304u64
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f29(v0: u64) -> R<u64> {
    let a562 = 8589975552u64;
    let a563 = {
        let a559 = v0;
        let a560 = 4294967296u64;
        let a561 = 0u64;
        nat_rem(a559, a560, a561)
    };
    nat_add(a562, a563)
}

pub fn f30(v0: u64) -> R<u64> {
    let a567 = 8589984768u64;
    let a568 = {
        let a564 = v0;
        let a565 = 4294967296u64;
        let a566 = 0u64;
        nat_rem(a564, a565, a566)
    };
    nat_add(a567, a568)
}

pub fn f31() -> u64 {
    4294967301u64
}

pub fn f32() -> u64 {
    4294967347u64
}

pub fn f33() -> u64 {
    4294967302u64
}

pub fn f34() -> u64 {
    4294967346u64
}

pub fn f35() -> u64 {
    4294967297u64
}

pub fn f36() -> u64 {
    4294967323u64
}

pub fn f37() -> u64 {
    4294967341u64
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f39(v0: u64) -> R<u64> {
    let a572 = 8589980672u64;
    let a573 = {
        let a569 = v0;
        let a570 = 4294967296u64;
        let a571 = 0u64;
        nat_rem(a569, a570, a571)
    };
    nat_add(a572, a573)
}

pub fn f40() -> u64 {
    4294967344u64
}

pub fn f41() -> u64 {
    4294967345u64
}

pub fn f42() -> u64 {
    4294967331u64
}

pub fn f43(v0: u64) -> R<u64> {
    let a577 = 30064771072u64;
    let a578 = {
        let a574 = v0;
        let a575 = 4294967296u64;
        let a576 = 0u64;
        nat_rem(a574, a575, a576)
    };
    nat_add(a577, a578)
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (14u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (15u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f47() -> u64 {
    4294967357u64
}

pub fn f48() -> u64 {
    4294967337u64
}

pub fn f49(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (7u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f50() -> u64 {
    4294967319u64
}

pub fn f51() -> u64 {
    4294967338u64
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f53() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (1u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f54() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (2u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}
