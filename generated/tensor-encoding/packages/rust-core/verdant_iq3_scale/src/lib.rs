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
                f18({
                    let a106 = v0;
                    let a107 = 8u64;
                    let a108 = 0u64;
                    nat_rem(a106, a107, a108)
                })
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
    let a113 = 2u64;
    if nat_lt(a112, a113) {
        let a114 = v1;
        let a115 = 1u64;
        if nat_lt(a114, a115) {
            let a116 = v1;
            let a117 = 0u64;
            if nat_eq(a116, a117) {
                Ok(f19({
                    let a118 = v0;
                    let a119 = 8u64;
                    let a120 = 0u64;
                    nat_rem(a118, a119, a120)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a121 = v1;
            let a122 = 1u64;
            if nat_eq(a121, a122) {
                f12({
                    let a123 = v0;
                    let a124 = 8u64;
                    let a125 = 0u64;
                    nat_rem(a123, a124, a125)
                })
            } else {
                Ok(f2())
            }
        }
    } else {
        let a126 = v1;
        let a127 = 3u64;
        if nat_lt(a126, a127) {
            let a128 = v1;
            let a129 = 2u64;
            if nat_eq(a128, a129) {
                Ok(f20({
                    let a130 = v0;
                    let a131 = 8u64;
                    let a132 = 0u64;
                    nat_rem(a130, a131, a132)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a133 = v1;
            let a134 = 3u64;
            if nat_eq(a133, a134) {
                Ok(f14({
                    let a135 = v0;
                    let a136 = 8u64;
                    let a137 = 0u64;
                    nat_rem(a135, a136, a137)
                }))
            } else {
                Ok(f2())
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a138 = v0;
    let a139 = 4u64;
    if nat_lt(a138, a139) {
        let a140 = v0;
        let a141 = 2u64;
        if nat_lt(a140, a141) {
            let a142 = v0;
            let a143 = 1u64;
            if nat_lt(a142, a143) {
                let a144 = v0;
                let a145 = 0u64;
                if nat_eq(a144, a145) {
                    Ok(f21(2u64, f22(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a146 = v0;
                let a147 = 1u64;
                if nat_eq(a146, a147) {
                    Ok(f23(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a148 = v0;
            let a149 = 3u64;
            if nat_lt(a148, a149) {
                let a150 = v0;
                let a151 = 2u64;
                if nat_eq(a150, a151) {
                    Ok(f24(6u64, f22(), 0u64, f22()))
                } else {
                    Ok(f2())
                }
            } else {
                let a152 = v0;
                let a153 = 3u64;
                if nat_eq(a152, a153) {
                    Ok(f25(0u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a154 = v0;
        let a155 = 6u64;
        if nat_lt(a154, a155) {
            let a156 = v0;
            let a157 = 5u64;
            if nat_lt(a156, a157) {
                let a158 = v0;
                let a159 = 4u64;
                if nat_eq(a158, a159) {
                    Ok(f27(1u64, 255u64, f28(f26())?, f28(f26())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a160 = v0;
                let a161 = 5u64;
                if nat_eq(a160, a161) {
                    Ok(f27(0u64, f26(), 1u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a162 = v0;
            let a163 = 7u64;
            if nat_lt(a162, a163) {
                let a164 = v0;
                let a165 = 6u64;
                if nat_eq(a164, a165) {
                    Ok(f21(1u64, f26(), 4u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a166 = v0;
                let a167 = 7u64;
                if nat_eq(a166, a167) {
                    Ok(f25(f22(), f29(f30())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a168 = v0;
    let a169 = 4u64;
    if nat_lt(a168, a169) {
        let a170 = v0;
        let a171 = 2u64;
        if nat_lt(a170, a171) {
            let a172 = v0;
            let a173 = 1u64;
            if nat_lt(a172, a173) {
                let a174 = v0;
                let a175 = 0u64;
                if nat_eq(a174, a175) {
                    f21(2u64, f31(), f22(), 3082u64)
                } else {
                    f2()
                }
            } else {
                let a176 = v0;
                let a177 = 1u64;
                if nat_eq(a176, a177) {
                    f23(3083u64)
                } else {
                    f2()
                }
            }
        } else {
            let a178 = v0;
            let a179 = 3u64;
            if nat_lt(a178, a179) {
                let a180 = v0;
                let a181 = 2u64;
                if nat_eq(a180, a181) {
                    f25(f22(), f31())
                } else {
                    f2()
                }
            } else {
                let a182 = v0;
                let a183 = 3u64;
                if nat_eq(a182, a183) {
                    f25(f32(), f33())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a184 = v0;
        let a185 = 6u64;
        if nat_lt(a184, a185) {
            let a186 = v0;
            let a187 = 5u64;
            if nat_lt(a186, a187) {
                let a188 = v0;
                let a189 = 4u64;
                if nat_eq(a188, a189) {
                    f21(0u64, f34(), 0u64, 3086u64)
                } else {
                    f2()
                }
            } else {
                let a190 = v0;
                let a191 = 5u64;
                if nat_eq(a190, a191) {
                    f25(f30(), f33())
                } else {
                    f2()
                }
            }
        } else {
            let a192 = v0;
            let a193 = 7u64;
            if nat_lt(a192, a193) {
                let a194 = v0;
                let a195 = 6u64;
                if nat_eq(a194, a195) {
                    f27(0u64, f32(), 1u64, f32())
                } else {
                    f2()
                }
            } else {
                let a196 = v0;
                let a197 = 7u64;
                if nat_eq(a196, a197) {
                    f25(0u64, f35())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a198 = v0;
    let a199 = 4u64;
    if nat_lt(a198, a199) {
        let a200 = v0;
        let a201 = 2u64;
        if nat_lt(a200, a201) {
            let a202 = v0;
            let a203 = 1u64;
            if nat_lt(a202, a203) {
                let a204 = v0;
                let a205 = 0u64;
                if nat_eq(a204, a205) {
                    Ok(f27(2u64, f33(), 8u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a206 = v0;
                let a207 = 1u64;
                if nat_eq(a206, a207) {
                    Ok(f27(0u64, f36(), f35(), f36()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a208 = v0;
            let a209 = 3u64;
            if nat_lt(a208, a209) {
                let a210 = v0;
                let a211 = 2u64;
                if nat_eq(a210, a211) {
                    Ok(f37(7u64, f36(), 0u64, f38(f35())?, f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a212 = v0;
                let a213 = 3u64;
                if nat_eq(a212, a213) {
                    Ok(f25(f41(), f42(f39())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a214 = v0;
        let a215 = 6u64;
        if nat_lt(a214, a215) {
            let a216 = v0;
            let a217 = 5u64;
            if nat_lt(a216, a217) {
                let a218 = v0;
                let a219 = 4u64;
                if nat_eq(a218, a219) {
                    Ok(f21(0u64, f34(), 1u64, 3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a220 = v0;
                let a221 = 5u64;
                if nat_eq(a220, a221) {
                    Ok(f27(2u64, f30(), 8u64, f36()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a222 = v0;
            let a223 = 7u64;
            if nat_lt(a222, a223) {
                let a224 = v0;
                let a225 = 6u64;
                if nat_eq(a224, a225) {
                    Ok(f27(0u64, f36(), f35(), f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a226 = v0;
                let a227 = 7u64;
                if nat_eq(a226, a227) {
                    Ok(f37(7u64, f36(), 1u64, f38(f35())?, f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a228 = v0;
    let a229 = 4u64;
    if nat_lt(a228, a229) {
        let a230 = v0;
        let a231 = 2u64;
        if nat_lt(a230, a231) {
            let a232 = v0;
            let a233 = 1u64;
            if nat_lt(a232, a233) {
                let a234 = v0;
                let a235 = 0u64;
                if nat_eq(a234, a235) {
                    Ok(f43(f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a236 = v0;
                let a237 = 1u64;
                if nat_eq(a236, a237) {
                    Ok(f27(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a238 = v0;
            let a239 = 3u64;
            if nat_lt(a238, a239) {
                let a240 = v0;
                let a241 = 2u64;
                if nat_eq(a240, a241) {
                    Ok(f21(1u64, f35(), 8u64, 3088u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a242 = v0;
                let a243 = 3u64;
                if nat_eq(a242, a243) {
                    Ok(f25(0u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a244 = v0;
        let a245 = 6u64;
        if nat_lt(a244, a245) {
            let a246 = v0;
            let a247 = 5u64;
            if nat_lt(a246, a247) {
                let a248 = v0;
                let a249 = 4u64;
                if nat_eq(a248, a249) {
                    Ok(f44(9u64, f33(), f26(), 0u64, f39(), f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a250 = v0;
                let a251 = 5u64;
                if nat_eq(a250, a251) {
                    Ok(f25(f28(f26())?, f41()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a252 = v0;
            let a253 = 7u64;
            if nat_lt(a252, a253) {
                let a254 = v0;
                let a255 = 6u64;
                if nat_eq(a254, a255) {
                    Ok(f21(0u64, f34(), 0u64, 3106u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a256 = v0;
                let a257 = 7u64;
                if nat_eq(a256, a257) {
                    Ok(f27(4u64, f41(), 128u64, f41()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a258 = v0;
    let a259 = 4u64;
    if nat_lt(a258, a259) {
        let a260 = v0;
        let a261 = 2u64;
        if nat_lt(a260, a261) {
            let a262 = v0;
            let a263 = 1u64;
            if nat_lt(a262, a263) {
                let a264 = v0;
                let a265 = 0u64;
                if nat_eq(a264, a265) {
                    Ok(f45(f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a266 = v0;
                let a267 = 1u64;
                if nat_eq(a266, a267) {
                    Ok(f23(3107u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a268 = v0;
            let a269 = 3u64;
            if nat_lt(a268, a269) {
                let a270 = v0;
                let a271 = 2u64;
                if nat_eq(a270, a271) {
                    Ok(f25(f41(), f42(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a272 = v0;
                let a273 = 3u64;
                if nat_eq(a272, a273) {
                    Ok(f27(0u64, f26(), 1u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a274 = v0;
        let a275 = 6u64;
        if nat_lt(a274, a275) {
            let a276 = v0;
            let a277 = 5u64;
            if nat_lt(a276, a277) {
                let a278 = v0;
                let a279 = 4u64;
                if nat_eq(a278, a279) {
                    Ok(f21(1u64, f26(), 4u64, 3100u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a280 = v0;
                let a281 = 5u64;
                if nat_eq(a280, a281) {
                    Ok(f27(0u64, f30(), 1u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a282 = v0;
            let a283 = 7u64;
            if nat_lt(a282, a283) {
                let a284 = v0;
                let a285 = 6u64;
                if nat_eq(a284, a285) {
                    Ok(f21(1u64, f30(), 8u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a286 = v0;
                let a287 = 7u64;
                if nat_eq(a286, a287) {
                    Ok(f23(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a288 = v0;
    let a289 = 4u64;
    if nat_lt(a288, a289) {
        let a290 = v0;
        let a291 = 2u64;
        if nat_lt(a290, a291) {
            let a292 = v0;
            let a293 = 1u64;
            if nat_lt(a292, a293) {
                let a294 = v0;
                let a295 = 0u64;
                if nat_eq(a294, a295) {
                    f21(0u64, f31(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a296 = v0;
                let a297 = 1u64;
                if nat_eq(a296, a297) {
                    f24(3u64, f31(), 1106771968u64, f22())
                } else {
                    f2()
                }
            }
        } else {
            let a298 = v0;
            let a299 = 3u64;
            if nat_lt(a298, a299) {
                let a300 = v0;
                let a301 = 2u64;
                if nat_eq(a300, a301) {
                    f25(1065458074u64, f46())
                } else {
                    f2()
                }
            } else {
                let a302 = v0;
                let a303 = 3u64;
                if nat_eq(a302, a303) {
                    f21(0u64, f34(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a304 = v0;
        let a305 = 6u64;
        if nat_lt(a304, a305) {
            let a306 = v0;
            let a307 = 5u64;
            if nat_lt(a306, a307) {
                let a308 = v0;
                let a309 = 4u64;
                if nat_eq(a308, a309) {
                    f25(1065630040u64, f46())
                } else {
                    f2()
                }
            } else {
                let a310 = v0;
                let a311 = 5u64;
                if nat_eq(a310, a311) {
                    f24(2u64, f22(), f46(), f47())
                } else {
                    f2()
                }
            }
        } else {
            let a312 = v0;
            let a313 = 7u64;
            if nat_lt(a312, a313) {
                let a314 = v0;
                let a315 = 6u64;
                if nat_eq(a314, a315) {
                    f48(f47(), f47())
                } else {
                    f2()
                }
            } else {
                let a316 = v0;
                let a317 = 7u64;
                if nat_eq(a316, a317) {
                    f27(4u64, f47(), 256u64, f41())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a318 = v0;
    let a319 = 4u64;
    if nat_lt(a318, a319) {
        let a320 = v0;
        let a321 = 2u64;
        if nat_lt(a320, a321) {
            let a322 = v0;
            let a323 = 1u64;
            if nat_lt(a322, a323) {
                let a324 = v0;
                let a325 = 0u64;
                if nat_eq(a324, a325) {
                    Ok(f25(0u64, f39()))
                } else {
                    Ok(f2())
                }
            } else {
                let a326 = v0;
                let a327 = 1u64;
                if nat_eq(a326, a327) {
                    Ok(f25(f41(), f42(f39())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a328 = v0;
            let a329 = 3u64;
            if nat_lt(a328, a329) {
                let a330 = v0;
                let a331 = 2u64;
                if nat_eq(a330, a331) {
                    Ok(f27(3u64, f47(), 256u64, f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a332 = v0;
                let a333 = 3u64;
                if nat_eq(a332, a333) {
                    Ok(f27(0u64, f39(), 1u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a334 = v0;
        let a335 = 6u64;
        if nat_lt(a334, a335) {
            let a336 = v0;
            let a337 = 5u64;
            if nat_lt(a336, a337) {
                let a338 = v0;
                let a339 = 4u64;
                if nat_eq(a338, a339) {
                    Ok(f25(f41(), f42(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a340 = v0;
                let a341 = 5u64;
                if nat_eq(a340, a341) {
                    Ok(f24(3u64, 1065353216u64, f22(), f49()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a342 = v0;
            let a343 = 7u64;
            if nat_lt(a342, a343) {
                let a344 = v0;
                let a345 = 6u64;
                if nat_eq(a344, a345) {
                    Ok(f25(0u64, f30()))
                } else {
                    Ok(f2())
                }
            } else {
                let a346 = v0;
                let a347 = 7u64;
                if nat_eq(a346, a347) {
                    Ok(f24(2u64, f49(), f29(f30())?, f50()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a348 = v0;
    let a349 = 4u64;
    if nat_lt(a348, a349) {
        let a350 = v0;
        let a351 = 2u64;
        if nat_lt(a350, a351) {
            let a352 = v0;
            let a353 = 1u64;
            if nat_lt(a352, a353) {
                let a354 = v0;
                let a355 = 0u64;
                if nat_eq(a354, a355) {
                    f24(1u64, f50(), 1065353216u64, f50())
                } else {
                    f2()
                }
            } else {
                let a356 = v0;
                let a357 = 1u64;
                if nat_eq(a356, a357) {
                    f24(2u64, 1056964608u64, f50(), f50())
                } else {
                    f2()
                }
            }
        } else {
            let a358 = v0;
            let a359 = 3u64;
            if nat_lt(a358, a359) {
                let a360 = v0;
                let a361 = 2u64;
                if nat_eq(a360, a361) {
                    f24(0u64, f50(), 1262485504u64, f50())
                } else {
                    f2()
                }
            } else {
                let a362 = v0;
                let a363 = 3u64;
                if nat_eq(a362, a363) {
                    f51(2u64, f50(), 15u64, 0u64, f41())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a364 = v0;
        let a365 = 6u64;
        if nat_lt(a364, a365) {
            let a366 = v0;
            let a367 = 5u64;
            if nat_lt(a366, a367) {
                let a368 = v0;
                let a369 = 4u64;
                if nat_eq(a368, a369) {
                    f44(8u64, f30(), 0u64, 0u64, f39(), f40())
                } else {
                    f2()
                }
            } else {
                let a370 = v0;
                let a371 = 5u64;
                if nat_eq(a370, a371) {
                    f43(f39(), f40(), f41())
                } else {
                    f2()
                }
            }
        } else {
            let a372 = v0;
            let a373 = 7u64;
            if nat_lt(a372, a373) {
                let a374 = v0;
                let a375 = 6u64;
                if nat_eq(a374, a375) {
                    f27(0u64, f30(), 1u64, f30())
                } else {
                    f2()
                }
            } else {
                let a376 = v0;
                let a377 = 7u64;
                if nat_eq(a376, a377) {
                    f21(1u64, f30(), 8u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a378 = v0;
    let a379 = 1u64;
    if nat_lt(a378, a379) {
        let a380 = v0;
        let a381 = 0u64;
        if nat_eq(a380, a381) {
            f52()
        } else {
            f2()
        }
    } else {
        let a382 = v0;
        let a383 = 1u64;
        if nat_eq(a382, a383) {
            f53()
        } else {
            f2()
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a384 = v0;
    let a385 = 4u64;
    if nat_lt(a384, a385) {
        let a386 = v0;
        let a387 = 2u64;
        if nat_lt(a386, a387) {
            let a388 = v0;
            let a389 = 1u64;
            if nat_lt(a388, a389) {
                let a390 = v0;
                let a391 = 0u64;
                if nat_eq(a390, a391) {
                    Ok(f21(2u64, f22(), 0u64, 3074u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a392 = v0;
                let a393 = 1u64;
                if nat_eq(a392, a393) {
                    Ok(f23(3079u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a394 = v0;
            let a395 = 3u64;
            if nat_lt(a394, a395) {
                let a396 = v0;
                let a397 = 2u64;
                if nat_eq(a396, a397) {
                    Ok(f24(6u64, f22(), 0u64, f22()))
                } else {
                    Ok(f2())
                }
            } else {
                let a398 = v0;
                let a399 = 3u64;
                if nat_eq(a398, a399) {
                    Ok(f25(0u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a400 = v0;
        let a401 = 6u64;
        if nat_lt(a400, a401) {
            let a402 = v0;
            let a403 = 5u64;
            if nat_lt(a402, a403) {
                let a404 = v0;
                let a405 = 4u64;
                if nat_eq(a404, a405) {
                    Ok(f27(1u64, 255u64, f28(f26())?, f28(f26())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a406 = v0;
                let a407 = 5u64;
                if nat_eq(a406, a407) {
                    Ok(f27(0u64, f26(), 1u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a408 = v0;
            let a409 = 7u64;
            if nat_lt(a408, a409) {
                let a410 = v0;
                let a411 = 6u64;
                if nat_eq(a410, a411) {
                    Ok(f21(1u64, f26(), 2u64, 3076u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a412 = v0;
                let a413 = 7u64;
                if nat_eq(a412, a413) {
                    Ok(f25(f22(), f29(f30())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a414 = v0;
    let a415 = 4u64;
    if nat_lt(a414, a415) {
        let a416 = v0;
        let a417 = 2u64;
        if nat_lt(a416, a417) {
            let a418 = v0;
            let a419 = 1u64;
            if nat_lt(a418, a419) {
                let a420 = v0;
                let a421 = 0u64;
                if nat_eq(a420, a421) {
                    Ok(f21(2u64, f31(), f22(), 3082u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a422 = v0;
                let a423 = 1u64;
                if nat_eq(a422, a423) {
                    Ok(f23(3083u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a424 = v0;
            let a425 = 3u64;
            if nat_lt(a424, a425) {
                let a426 = v0;
                let a427 = 2u64;
                if nat_eq(a426, a427) {
                    Ok(f25(f22(), f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a428 = v0;
                let a429 = 3u64;
                if nat_eq(a428, a429) {
                    Ok(f25(0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a430 = v0;
        let a431 = 6u64;
        if nat_lt(a430, a431) {
            let a432 = v0;
            let a433 = 5u64;
            if nat_lt(a432, a433) {
                let a434 = v0;
                let a435 = 4u64;
                if nat_eq(a434, a435) {
                    Ok(f27(2u64, f30(), 2u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a436 = v0;
                let a437 = 5u64;
                if nat_eq(a436, a437) {
                    Ok(f27(0u64, f36(), f35(), f36()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a438 = v0;
            let a439 = 7u64;
            if nat_lt(a438, a439) {
                let a440 = v0;
                let a441 = 6u64;
                if nat_eq(a440, a441) {
                    Ok(f37(7u64, f36(), 0u64, f38(f35())?, f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a442 = v0;
                let a443 = 7u64;
                if nat_eq(a442, a443) {
                    Ok(f25(f41(), f42(f39())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a444 = v0;
    let a445 = 4u64;
    if nat_lt(a444, a445) {
        let a446 = v0;
        let a447 = 2u64;
        if nat_lt(a446, a447) {
            let a448 = v0;
            let a449 = 1u64;
            if nat_lt(a448, a449) {
                let a450 = v0;
                let a451 = 0u64;
                if nat_eq(a450, a451) {
                    Ok(f37(7u64, f36(), 1u64, f38(f35())?, f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a452 = v0;
                let a453 = 1u64;
                if nat_eq(a452, a453) {
                    Ok(f43(f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a454 = v0;
            let a455 = 3u64;
            if nat_lt(a454, a455) {
                let a456 = v0;
                let a457 = 2u64;
                if nat_eq(a456, a457) {
                    Ok(f21(0u64, f34(), 0u64, 3095u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a458 = v0;
                let a459 = 3u64;
                if nat_eq(a458, a459) {
                    Ok(f27(4u64, f28(f35())?, 128u64, f41()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a460 = v0;
        let a461 = 6u64;
        if nat_lt(a460, a461) {
            let a462 = v0;
            let a463 = 5u64;
            if nat_lt(a462, a463) {
                let a464 = v0;
                let a465 = 4u64;
                if nat_eq(a464, a465) {
                    Ok(f25(1u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a466 = v0;
                let a467 = 5u64;
                if nat_eq(a466, a467) {
                    Ok(f43(f39(), f40(), f41()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a468 = v0;
            let a469 = 7u64;
            if nat_lt(a468, a469) {
                let a470 = v0;
                let a471 = 6u64;
                if nat_eq(a470, a471) {
                    Ok(f23(3097u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a472 = v0;
                let a473 = 7u64;
                if nat_eq(a472, a473) {
                    Ok(f27(0u64, f36(), 34u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a474 = v0;
    let a475 = 3u64;
    if nat_lt(a474, a475) {
        let a476 = v0;
        let a477 = 1u64;
        if nat_lt(a476, a477) {
            let a478 = v0;
            let a479 = 0u64;
            if nat_eq(a478, a479) {
                Ok(f25(f28(f35())?, f42(f39())?))
            } else {
                Ok(f2())
            }
        } else {
            let a480 = v0;
            let a481 = 2u64;
            if nat_lt(a480, a481) {
                let a482 = v0;
                let a483 = 1u64;
                if nat_eq(a482, a483) {
                    Ok(f27(0u64, f35(), 1u64, f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a484 = v0;
                let a485 = 2u64;
                if nat_eq(a484, a485) {
                    Ok(f21(1u64, f35(), 2u64, 3084u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a486 = v0;
        let a487 = 4u64;
        if nat_lt(a486, a487) {
            let a488 = v0;
            let a489 = 3u64;
            if nat_eq(a488, a489) {
                Ok(f27(0u64, f30(), 1u64, f30()))
            } else {
                Ok(f2())
            }
        } else {
            let a490 = v0;
            let a491 = 5u64;
            if nat_lt(a490, a491) {
                let a492 = v0;
                let a493 = 4u64;
                if nat_eq(a492, a493) {
                    Ok(f21(1u64, f30(), 16u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a494 = v0;
                let a495 = 5u64;
                if nat_eq(a494, a495) {
                    Ok(f23(3584u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a496 = v0;
    let a497 = 4u64;
    if nat_lt(a496, a497) {
        let a498 = v0;
        let a499 = 2u64;
        if nat_lt(a498, a499) {
            let a500 = v0;
            let a501 = 1u64;
            if nat_lt(a500, a501) {
                let a502 = v0;
                let a503 = 0u64;
                if nat_eq(a502, a503) {
                    f21(0u64, f31(), 0u64, 3609u64)
                } else {
                    f2()
                }
            } else {
                let a504 = v0;
                let a505 = 1u64;
                if nat_eq(a504, a505) {
                    f24(3u64, f31(), 1106771968u64, f22())
                } else {
                    f2()
                }
            }
        } else {
            let a506 = v0;
            let a507 = 3u64;
            if nat_lt(a506, a507) {
                let a508 = v0;
                let a509 = 2u64;
                if nat_eq(a508, a509) {
                    f25(1065353216u64, f46())
                } else {
                    f2()
                }
            } else {
                let a510 = v0;
                let a511 = 3u64;
                if nat_eq(a510, a511) {
                    f21(0u64, f34(), 1u64, 3589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a512 = v0;
        let a513 = 6u64;
        if nat_lt(a512, a513) {
            let a514 = v0;
            let a515 = 5u64;
            if nat_lt(a514, a515) {
                let a516 = v0;
                let a517 = 4u64;
                if nat_eq(a516, a517) {
                    f25(1065143501u64, f46())
                } else {
                    f2()
                }
            } else {
                let a518 = v0;
                let a519 = 5u64;
                if nat_eq(a518, a519) {
                    f24(2u64, f22(), f46(), f47())
                } else {
                    f2()
                }
            }
        } else {
            let a520 = v0;
            let a521 = 7u64;
            if nat_lt(a520, a521) {
                let a522 = v0;
                let a523 = 6u64;
                if nat_eq(a522, a523) {
                    f48(f47(), f47())
                } else {
                    f2()
                }
            } else {
                let a524 = v0;
                let a525 = 7u64;
                if nat_eq(a524, a525) {
                    f27(4u64, f47(), 256u64, f41())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a526 = v0;
    let a527 = 4u64;
    if nat_lt(a526, a527) {
        let a528 = v0;
        let a529 = 2u64;
        if nat_lt(a528, a529) {
            let a530 = v0;
            let a531 = 1u64;
            if nat_lt(a530, a531) {
                let a532 = v0;
                let a533 = 0u64;
                if nat_eq(a532, a533) {
                    f24(1u64, f50(), 1065353216u64, f50())
                } else {
                    f2()
                }
            } else {
                let a534 = v0;
                let a535 = 1u64;
                if nat_eq(a534, a535) {
                    f24(2u64, 1056964608u64, f50(), f50())
                } else {
                    f2()
                }
            }
        } else {
            let a536 = v0;
            let a537 = 3u64;
            if nat_lt(a536, a537) {
                let a538 = v0;
                let a539 = 2u64;
                if nat_eq(a538, a539) {
                    f24(0u64, f50(), 1262485504u64, f50())
                } else {
                    f2()
                }
            } else {
                let a540 = v0;
                let a541 = 3u64;
                if nat_eq(a540, a541) {
                    f51(2u64, f50(), 15u64, 0u64, f41())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a542 = v0;
        let a543 = 6u64;
        if nat_lt(a542, a543) {
            let a544 = v0;
            let a545 = 5u64;
            if nat_lt(a544, a545) {
                let a546 = v0;
                let a547 = 4u64;
                if nat_eq(a546, a547) {
                    f44(8u64, f30(), 0u64, 0u64, f39(), f40())
                } else {
                    f2()
                }
            } else {
                let a548 = v0;
                let a549 = 5u64;
                if nat_eq(a548, a549) {
                    f43(f39(), f40(), f41())
                } else {
                    f2()
                }
            }
        } else {
            let a550 = v0;
            let a551 = 7u64;
            if nat_lt(a550, a551) {
                let a552 = v0;
                let a553 = 6u64;
                if nat_eq(a552, a553) {
                    f27(0u64, f30(), 1u64, f30())
                } else {
                    f2()
                }
            } else {
                let a554 = v0;
                let a555 = 7u64;
                if nat_eq(a554, a555) {
                    f21(1u64, f30(), 16u64, 3599u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f22() -> u64 {
    4294967316u64
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f25(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f26() -> u64 {
    4294967304u64
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f28(v0: u64) -> R<u64> {
    let a559 = 8589975552u64;
    let a560 = {
        let a556 = v0;
        let a557 = 4294967296u64;
        let a558 = 0u64;
        nat_rem(a556, a557, a558)
    };
    nat_add(a559, a560)
}

pub fn f29(v0: u64) -> R<u64> {
    let a564 = 8589984768u64;
    let a565 = {
        let a561 = v0;
        let a562 = 4294967296u64;
        let a563 = 0u64;
        nat_rem(a561, a562, a563)
    };
    nat_add(a564, a565)
}

pub fn f30() -> u64 {
    4294967301u64
}

pub fn f31() -> u64 {
    4294967347u64
}

pub fn f32() -> u64 {
    4294967302u64
}

pub fn f33() -> u64 {
    4294967346u64
}

pub fn f34() -> u64 {
    4294967297u64
}

pub fn f35() -> u64 {
    4294967323u64
}

pub fn f36() -> u64 {
    4294967341u64
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f38(v0: u64) -> R<u64> {
    let a569 = 8589980672u64;
    let a570 = {
        let a566 = v0;
        let a567 = 4294967296u64;
        let a568 = 0u64;
        nat_rem(a566, a567, a568)
    };
    nat_add(a569, a570)
}

pub fn f39() -> u64 {
    4294967344u64
}

pub fn f40() -> u64 {
    4294967345u64
}

pub fn f41() -> u64 {
    4294967331u64
}

pub fn f42(v0: u64) -> R<u64> {
    let a574 = 30064771072u64;
    let a575 = {
        let a571 = v0;
        let a572 = 4294967296u64;
        let a573 = 0u64;
        nat_rem(a571, a572, a573)
    };
    nat_add(a574, a575)
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (14u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (15u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f46() -> u64 {
    4294967357u64
}

pub fn f47() -> u64 {
    4294967337u64
}

pub fn f48(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (7u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f49() -> u64 {
    4294967319u64
}

pub fn f50() -> u64 {
    4294967338u64
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f52() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (1u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f53() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (2u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
