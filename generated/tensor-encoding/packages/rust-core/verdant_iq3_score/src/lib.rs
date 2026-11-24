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
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        f1(v1)
    } else {
        let a3 = v0;
        let a4 = 1u64;
        if nat_eq(a3, a4) {
            f2(v1)
        } else {
            Ok(f3())
        }
    }
}

pub fn f1(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a5 = v0;
        let a6 = 8u64;
        let a7 = 0u64;
        nat_quot(a5, a6, a7)
    };
    let a8 = v1;
    let a9 = 2u64;
    if nat_lt(a8, a9) {
        let a10 = v1;
        let a11 = 1u64;
        if nat_lt(a10, a11) {
            let a12 = v1;
            let a13 = 0u64;
            if nat_eq(a12, a13) {
                f4({
                    let a14 = v0;
                    let a15 = 8u64;
                    let a16 = 0u64;
                    nat_rem(a14, a15, a16)
                })
            } else {
                Ok(f3())
            }
        } else {
            let a17 = v1;
            let a18 = 1u64;
            if nat_eq(a17, a18) {
                f5({
                    let a19 = v0;
                    let a20 = 8u64;
                    let a21 = 0u64;
                    nat_rem(a19, a20, a21)
                })
            } else {
                Ok(f3())
            }
        }
    } else {
        let a22 = v1;
        let a23 = 3u64;
        if nat_lt(a22, a23) {
            let a24 = v1;
            let a25 = 2u64;
            if nat_eq(a24, a25) {
                f6({
                    let a26 = v0;
                    let a27 = 8u64;
                    let a28 = 0u64;
                    nat_rem(a26, a27, a28)
                })
            } else {
                Ok(f3())
            }
        } else {
            let a29 = v1;
            let a30 = 3u64;
            if nat_eq(a29, a30) {
                Ok(f7({
                    let a31 = v0;
                    let a32 = 8u64;
                    let a33 = 0u64;
                    nat_rem(a31, a32, a33)
                }))
            } else {
                Ok(f3())
            }
        }
    }
}

pub fn f2(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a34 = v0;
        let a35 = 8u64;
        let a36 = 0u64;
        nat_quot(a34, a35, a36)
    };
    let a37 = v1;
    let a38 = 1u64;
    if nat_lt(a37, a38) {
        let a39 = v1;
        let a40 = 0u64;
        if nat_eq(a39, a40) {
            f8({
                let a41 = v0;
                let a42 = 8u64;
                let a43 = 0u64;
                nat_rem(a41, a42, a43)
            })
        } else {
            Ok(f3())
        }
    } else {
        let a44 = v1;
        let a45 = 2u64;
        if nat_lt(a44, a45) {
            let a46 = v1;
            let a47 = 1u64;
            if nat_eq(a46, a47) {
                f9({
                    let a48 = v0;
                    let a49 = 8u64;
                    let a50 = 0u64;
                    nat_rem(a48, a49, a50)
                })
            } else {
                Ok(f3())
            }
        } else {
            let a51 = v1;
            let a52 = 2u64;
            if nat_eq(a51, a52) {
                Ok(f10({
                    let a53 = v0;
                    let a54 = 8u64;
                    let a55 = 0u64;
                    nat_rem(a53, a54, a55)
                }))
            } else {
                Ok(f3())
            }
        }
    }
}

pub fn f3() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a56 = v0;
    let a57 = 4u64;
    if nat_lt(a56, a57) {
        let a58 = v0;
        let a59 = 2u64;
        if nat_lt(a58, a59) {
            let a60 = v0;
            let a61 = 1u64;
            if nat_lt(a60, a61) {
                let a62 = v0;
                let a63 = 0u64;
                if nat_eq(a62, a63) {
                    Ok(f11(0u64, f12()))
                } else {
                    Ok(f3())
                }
            } else {
                let a64 = v0;
                let a65 = 1u64;
                if nat_eq(a64, a65) {
                    Ok(f11(0u64, f13()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a66 = v0;
            let a67 = 3u64;
            if nat_lt(a66, a67) {
                let a68 = v0;
                let a69 = 2u64;
                if nat_eq(a68, a69) {
                    Ok(f11(0u64, f14()))
                } else {
                    Ok(f3())
                }
            } else {
                let a70 = v0;
                let a71 = 3u64;
                if nat_eq(a70, a71) {
                    Ok(f11(0u64, f15()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a72 = v0;
        let a73 = 6u64;
        if nat_lt(a72, a73) {
            let a74 = v0;
            let a75 = 5u64;
            if nat_lt(a74, a75) {
                let a76 = v0;
                let a77 = 4u64;
                if nat_eq(a76, a77) {
                    Ok(f16(0u64, f17(), 1u64, 2055u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a78 = v0;
                let a79 = 5u64;
                if nat_eq(a78, a79) {
                    Ok(f11(f18(f12())?, f19()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a80 = v0;
            let a81 = 7u64;
            if nat_lt(a80, a81) {
                let a82 = v0;
                let a83 = 6u64;
                if nat_eq(a82, a83) {
                    Ok(f20(2056u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a84 = v0;
                let a85 = 7u64;
                if nat_eq(a84, a85) {
                    Ok(f11(f21(f12())?, f19()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a86 = v0;
    let a87 = 4u64;
    if nat_lt(a86, a87) {
        let a88 = v0;
        let a89 = 2u64;
        if nat_lt(a88, a89) {
            let a90 = v0;
            let a91 = 1u64;
            if nat_lt(a90, a91) {
                let a92 = v0;
                let a93 = 0u64;
                if nat_eq(a92, a93) {
                    Ok(f11(0u64, f22()))
                } else {
                    Ok(f3())
                }
            } else {
                let a94 = v0;
                let a95 = 1u64;
                if nat_eq(a94, a95) {
                    Ok(f23(4u64, 0u64, f19(), 0u64, f24(), f25()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a96 = v0;
            let a97 = 3u64;
            if nat_lt(a96, a97) {
                let a98 = v0;
                let a99 = 2u64;
                if nat_eq(a98, a99) {
                    Ok(f26(0u64, f25(), f22(), f25()))
                } else {
                    Ok(f3())
                }
            } else {
                let a100 = v0;
                let a101 = 3u64;
                if nat_eq(a100, a101) {
                    Ok(f27(5u64, f28(f25())?, 0u64, 0u64, f29()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a102 = v0;
        let a103 = 6u64;
        if nat_lt(a102, a103) {
            let a104 = v0;
            let a105 = 5u64;
            if nat_lt(a104, a105) {
                let a106 = v0;
                let a107 = 4u64;
                if nat_eq(a106, a107) {
                    Ok(f30(f29(), f29()))
                } else {
                    Ok(f3())
                }
            } else {
                let a108 = v0;
                let a109 = 5u64;
                if nat_eq(a108, a109) {
                    Ok(f31(2u64, f32(f15())?, f33(f15())?, f34()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a110 = v0;
            let a111 = 7u64;
            if nat_lt(a110, a111) {
                let a112 = v0;
                let a113 = 6u64;
                if nat_eq(a112, a113) {
                    Ok(f31(2u64, f34(), f29(), f34()))
                } else {
                    Ok(f3())
                }
            } else {
                let a114 = v0;
                let a115 = 7u64;
                if nat_eq(a114, a115) {
                    Ok(f31(0u64, f13(), f34(), f13()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a116 = v0;
    let a117 = 4u64;
    if nat_lt(a116, a117) {
        let a118 = v0;
        let a119 = 2u64;
        if nat_lt(a118, a119) {
            let a120 = v0;
            let a121 = 1u64;
            if nat_lt(a120, a121) {
                let a122 = v0;
                let a123 = 0u64;
                if nat_eq(a122, a123) {
                    Ok(f31(2u64, f32(f15())?, f29(), f34()))
                } else {
                    Ok(f3())
                }
            } else {
                let a124 = v0;
                let a125 = 1u64;
                if nat_eq(a124, a125) {
                    Ok(f31(2u64, f34(), f29(), f34()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a126 = v0;
            let a127 = 3u64;
            if nat_lt(a126, a127) {
                let a128 = v0;
                let a129 = 2u64;
                if nat_eq(a128, a129) {
                    Ok(f31(0u64, f14(), f34(), f14()))
                } else {
                    Ok(f3())
                }
            } else {
                let a130 = v0;
                let a131 = 3u64;
                if nat_eq(a130, a131) {
                    Ok(f26(0u64, f22(), 1u64, f22()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a132 = v0;
        let a133 = 6u64;
        if nat_lt(a132, a133) {
            let a134 = v0;
            let a135 = 5u64;
            if nat_lt(a134, a135) {
                let a136 = v0;
                let a137 = 4u64;
                if nat_eq(a136, a137) {
                    Ok(f26(0u64, f15(), 1u64, f15()))
                } else {
                    Ok(f3())
                }
            } else {
                let a138 = v0;
                let a139 = 5u64;
                if nat_eq(a138, a139) {
                    Ok(f16(1u64, f22(), 4u64, 2057u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a140 = v0;
            let a141 = 7u64;
            if nat_lt(a140, a141) {
                let a142 = v0;
                let a143 = 6u64;
                if nat_eq(a142, a143) {
                    Ok(f26(0u64, f12(), 1u64, f12()))
                } else {
                    Ok(f3())
                }
            } else {
                let a144 = v0;
                let a145 = 7u64;
                if nat_eq(a144, a145) {
                    Ok(f16(1u64, f12(), 8u64, 2052u64))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a146 = v0;
    let a147 = 0u64;
    if nat_eq(a146, a147) {
        f35(f36())
    } else {
        f3()
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a148 = v0;
    let a149 = 4u64;
    if nat_lt(a148, a149) {
        let a150 = v0;
        let a151 = 2u64;
        if nat_lt(a150, a151) {
            let a152 = v0;
            let a153 = 1u64;
            if nat_lt(a152, a153) {
                let a154 = v0;
                let a155 = 0u64;
                if nat_eq(a154, a155) {
                    Ok(f16(2u64, 0u64, f37(), 2562u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a156 = v0;
                let a157 = 1u64;
                if nat_eq(a156, a157) {
                    Ok(f20(3072u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a158 = v0;
            let a159 = 3u64;
            if nat_lt(a158, a159) {
                let a160 = v0;
                let a161 = 2u64;
                if nat_eq(a160, a161) {
                    Ok(f11(0u64, f12()))
                } else {
                    Ok(f3())
                }
            } else {
                let a162 = v0;
                let a163 = 3u64;
                if nat_eq(a162, a163) {
                    Ok(f16(0u64, f38(f12())?, 0u64, 2567u64))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a164 = v0;
        let a165 = 6u64;
        if nat_lt(a164, a165) {
            let a166 = v0;
            let a167 = 5u64;
            if nat_lt(a166, a167) {
                let a168 = v0;
                let a169 = 4u64;
                if nat_eq(a168, a169) {
                    Ok(f26(0u64, f12(), 1u64, f12()))
                } else {
                    Ok(f3())
                }
            } else {
                let a170 = v0;
                let a171 = 5u64;
                if nat_eq(a170, a171) {
                    Ok(f16(1u64, f12(), 8u64, 2563u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a172 = v0;
            let a173 = 7u64;
            if nat_lt(a172, a173) {
                let a174 = v0;
                let a175 = 6u64;
                if nat_eq(a174, a175) {
                    Ok(f20(3072u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a176 = v0;
                let a177 = 7u64;
                if nat_eq(a176, a177) {
                    Ok(f31(3u64, 1065353216u64, f37(), f39()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a178 = v0;
    let a179 = 4u64;
    if nat_lt(a178, a179) {
        let a180 = v0;
        let a181 = 2u64;
        if nat_lt(a180, a181) {
            let a182 = v0;
            let a183 = 1u64;
            if nat_lt(a182, a183) {
                let a184 = v0;
                let a185 = 0u64;
                if nat_eq(a184, a185) {
                    Ok(f11(f37(), f40()))
                } else {
                    Ok(f3())
                }
            } else {
                let a186 = v0;
                let a187 = 1u64;
                if nat_eq(a186, a187) {
                    Ok(f11(0u64, f12()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a188 = v0;
            let a189 = 3u64;
            if nat_lt(a188, a189) {
                let a190 = v0;
                let a191 = 2u64;
                if nat_eq(a190, a191) {
                    Ok(f16(0u64, f41(), 0u64, 2572u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a192 = v0;
                let a193 = 3u64;
                if nat_eq(a192, a193) {
                    Ok(f16(0u64, f38(f12())?, 1u64, 2575u64))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a194 = v0;
        let a195 = 6u64;
        if nat_lt(a194, a195) {
            let a196 = v0;
            let a197 = 5u64;
            if nat_lt(a196, a197) {
                let a198 = v0;
                let a199 = 4u64;
                if nat_eq(a198, a199) {
                    Ok(f11(2574u64, f42()))
                } else {
                    Ok(f3())
                }
            } else {
                let a200 = v0;
                let a201 = 5u64;
                if nat_eq(a200, a201) {
                    Ok(f20(1536u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a202 = v0;
            let a203 = 7u64;
            if nat_lt(a202, a203) {
                let a204 = v0;
                let a205 = 6u64;
                if nat_eq(a204, a205) {
                    Ok(f11(f19(), f21(f12())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a206 = v0;
                let a207 = 7u64;
                if nat_eq(a206, a207) {
                    Ok(f26(0u64, f12(), 1u64, f12()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a208 = v0;
    let a209 = 4u64;
    if nat_lt(a208, a209) {
        let a210 = v0;
        let a211 = 2u64;
        if nat_lt(a210, a211) {
            let a212 = v0;
            let a213 = 1u64;
            if nat_lt(a212, a213) {
                let a214 = v0;
                let a215 = 0u64;
                if nat_eq(a214, a215) {
                    f16(1u64, f12(), 8u64, 2570u64)
                } else {
                    f3()
                }
            } else {
                let a216 = v0;
                let a217 = 1u64;
                if nat_eq(a216, a217) {
                    f11(1u64, f17())
                } else {
                    f3()
                }
            }
        } else {
            let a218 = v0;
            let a219 = 3u64;
            if nat_lt(a218, a219) {
                let a220 = v0;
                let a221 = 2u64;
                if nat_eq(a220, a221) {
                    f11(2580u64, f36())
                } else {
                    f3()
                }
            } else {
                let a222 = v0;
                let a223 = 3u64;
                if nat_eq(a222, a223) {
                    f20(2048u64)
                } else {
                    f3()
                }
            }
        }
    } else {
        let a224 = v0;
        let a225 = 6u64;
        if nat_lt(a224, a225) {
            let a226 = v0;
            let a227 = 5u64;
            if nat_lt(a226, a227) {
                let a228 = v0;
                let a229 = 4u64;
                if nat_eq(a228, a229) {
                    f16(2u64, 0u64, f14(), 2582u64)
                } else {
                    f3()
                }
            } else {
                let a230 = v0;
                let a231 = 5u64;
                if nat_eq(a230, a231) {
                    f20(3072u64)
                } else {
                    f3()
                }
            }
        } else {
            let a232 = v0;
            let a233 = 7u64;
            if nat_lt(a232, a233) {
                let a234 = v0;
                let a235 = 6u64;
                if nat_eq(a234, a235) {
                    f31(3u64, f13(), f14(), f37())
                } else {
                    f3()
                }
            } else {
                let a236 = v0;
                let a237 = 7u64;
                if nat_eq(a236, a237) {
                    f20(3072u64)
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f12() -> u64 {
    4294967323u64
}

pub fn f13() -> u64 {
    4294967335u64
}

pub fn f14() -> u64 {
    4294967336u64
}

pub fn f15() -> u64 {
    4294967303u64
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f17() -> u64 {
    4294967324u64
}

pub fn f18(v0: u64) -> R<u64> {
    let a241 = 8589976576u64;
    let a242 = {
        let a238 = v0;
        let a239 = 4294967296u64;
        let a240 = 0u64;
        nat_rem(a238, a239, a240)
    };
    nat_add(a241, a242)
}

pub fn f19() -> u64 {
    4294967321u64
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f21(v0: u64) -> R<u64> {
    let a246 = 8589980672u64;
    let a247 = {
        let a243 = v0;
        let a244 = 4294967296u64;
        let a245 = 0u64;
        nat_rem(a243, a244, a245)
    };
    nat_add(a246, a247)
}

pub fn f22() -> u64 {
    4294967305u64
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f24() -> u64 {
    4294967338u64
}

pub fn f25() -> u64 {
    4294967330u64
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f28(v0: u64) -> R<u64> {
    let a251 = 25769803776u64;
    let a252 = {
        let a248 = v0;
        let a249 = 4294967296u64;
        let a250 = 0u64;
        nat_rem(a248, a249, a250)
    };
    nat_add(a251, a252)
}

pub fn f29() -> u64 {
    4294967332u64
}

pub fn f30(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f32(v0: u64) -> R<u64> {
    let a256 = 8589959168u64;
    let a257 = {
        let a253 = v0;
        let a254 = 4294967296u64;
        let a255 = 0u64;
        nat_rem(a253, a254, a255)
    };
    nat_add(a256, a257)
}

pub fn f33(v0: u64) -> R<u64> {
    let a261 = 8589950976u64;
    let a262 = {
        let a258 = v0;
        let a259 = 4294967296u64;
        let a260 = 0u64;
        nat_rem(a258, a259, a260)
    };
    nat_add(a261, a262)
}

pub fn f34() -> u64 {
    4294967337u64
}

pub fn f35(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f36() -> u64 {
    4294967351u64
}

pub fn f37() -> u64 {
    4294967316u64
}

pub fn f38(v0: u64) -> R<u64> {
    let a266 = 8589982720u64;
    let a267 = {
        let a263 = v0;
        let a264 = 4294967296u64;
        let a265 = 0u64;
        nat_rem(a263, a264, a265)
    };
    nat_add(a266, a267)
}

pub fn f39() -> u64 {
    4294967319u64
}

pub fn f40() -> u64 {
    4294967320u64
}

pub fn f41() -> u64 {
    4294967297u64
}

pub fn f42() -> u64 {
    4294967350u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
