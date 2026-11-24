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
    let a9 = 3u64;
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
            let a18 = 2u64;
            if nat_lt(a17, a18) {
                let a19 = v1;
                let a20 = 1u64;
                if nat_eq(a19, a20) {
                    Ok(f5({
                        let a21 = v0;
                        let a22 = 8u64;
                        let a23 = 0u64;
                        nat_rem(a21, a22, a23)
                    }))
                } else {
                    Ok(f3())
                }
            } else {
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
            }
        }
    } else {
        let a29 = v1;
        let a30 = 4u64;
        if nat_lt(a29, a30) {
            let a31 = v1;
            let a32 = 3u64;
            if nat_eq(a31, a32) {
                Ok(f7({
                    let a33 = v0;
                    let a34 = 8u64;
                    let a35 = 0u64;
                    nat_rem(a33, a34, a35)
                }))
            } else {
                Ok(f3())
            }
        } else {
            let a36 = v1;
            let a37 = 5u64;
            if nat_lt(a36, a37) {
                let a38 = v1;
                let a39 = 4u64;
                if nat_eq(a38, a39) {
                    f8({
                        let a40 = v0;
                        let a41 = 8u64;
                        let a42 = 0u64;
                        nat_rem(a40, a41, a42)
                    })
                } else {
                    Ok(f3())
                }
            } else {
                let a43 = v1;
                let a44 = 5u64;
                if nat_eq(a43, a44) {
                    Ok(f9({
                        let a45 = v0;
                        let a46 = 8u64;
                        let a47 = 0u64;
                        nat_rem(a45, a46, a47)
                    }))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a48 = v0;
        let a49 = 8u64;
        let a50 = 0u64;
        nat_quot(a48, a49, a50)
    };
    let a51 = v1;
    let a52 = 3u64;
    if nat_lt(a51, a52) {
        let a53 = v1;
        let a54 = 1u64;
        if nat_lt(a53, a54) {
            let a55 = v1;
            let a56 = 0u64;
            if nat_eq(a55, a56) {
                f10({
                    let a57 = v0;
                    let a58 = 8u64;
                    let a59 = 0u64;
                    nat_rem(a57, a58, a59)
                })
            } else {
                Ok(f3())
            }
        } else {
            let a60 = v1;
            let a61 = 2u64;
            if nat_lt(a60, a61) {
                let a62 = v1;
                let a63 = 1u64;
                if nat_eq(a62, a63) {
                    f11({
                        let a64 = v0;
                        let a65 = 8u64;
                        let a66 = 0u64;
                        nat_rem(a64, a65, a66)
                    })
                } else {
                    Ok(f3())
                }
            } else {
                let a67 = v1;
                let a68 = 2u64;
                if nat_eq(a67, a68) {
                    f12({
                        let a69 = v0;
                        let a70 = 8u64;
                        let a71 = 0u64;
                        nat_rem(a69, a70, a71)
                    })
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a72 = v1;
        let a73 = 5u64;
        if nat_lt(a72, a73) {
            let a74 = v1;
            let a75 = 4u64;
            if nat_lt(a74, a75) {
                let a76 = v1;
                let a77 = 3u64;
                if nat_eq(a76, a77) {
                    f13({
                        let a78 = v0;
                        let a79 = 8u64;
                        let a80 = 0u64;
                        nat_rem(a78, a79, a80)
                    })
                } else {
                    Ok(f3())
                }
            } else {
                let a81 = v1;
                let a82 = 4u64;
                if nat_eq(a81, a82) {
                    f14({
                        let a83 = v0;
                        let a84 = 8u64;
                        let a85 = 0u64;
                        nat_rem(a83, a84, a85)
                    })
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a86 = v1;
            let a87 = 6u64;
            if nat_lt(a86, a87) {
                let a88 = v1;
                let a89 = 5u64;
                if nat_eq(a88, a89) {
                    Ok(f15({
                        let a90 = v0;
                        let a91 = 8u64;
                        let a92 = 0u64;
                        nat_rem(a90, a91, a92)
                    }))
                } else {
                    Ok(f3())
                }
            } else {
                let a93 = v1;
                let a94 = 6u64;
                if nat_eq(a93, a94) {
                    Ok(f16({
                        let a95 = v0;
                        let a96 = 8u64;
                        let a97 = 0u64;
                        nat_rem(a95, a96, a97)
                    }))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f3() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f4(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a98 = v0;
    let a99 = 4u64;
    if nat_lt(a98, a99) {
        let a100 = v0;
        let a101 = 2u64;
        if nat_lt(a100, a101) {
            let a102 = v0;
            let a103 = 1u64;
            if nat_lt(a102, a103) {
                let a104 = v0;
                let a105 = 0u64;
                if nat_eq(a104, a105) {
                    Ok(f17(3u64, f18(), 1097859072u64, f19()))
                } else {
                    Ok(f3())
                }
            } else {
                let a106 = v0;
                let a107 = 1u64;
                if nat_eq(a106, a107) {
                    Ok(f20(0u64, f21()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a108 = v0;
            let a109 = 3u64;
            if nat_lt(a108, a109) {
                let a110 = v0;
                let a111 = 2u64;
                if nat_eq(a110, a111) {
                    Ok(f20(0u64, f22()))
                } else {
                    Ok(f3())
                }
            } else {
                let a112 = v0;
                let a113 = 3u64;
                if nat_eq(a112, a113) {
                    Ok(f20(0u64, f23()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a114 = v0;
        let a115 = 6u64;
        if nat_lt(a114, a115) {
            let a116 = v0;
            let a117 = 5u64;
            if nat_lt(a116, a117) {
                let a118 = v0;
                let a119 = 4u64;
                if nat_eq(a118, a119) {
                    Ok(f20(0u64, f24(f23())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a120 = v0;
                let a121 = 5u64;
                if nat_eq(a120, a121) {
                    Ok(f20(f25(), f26(f23())?))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a122 = v0;
            let a123 = 7u64;
            if nat_lt(a122, a123) {
                let a124 = v0;
                let a125 = 6u64;
                if nat_eq(a124, a125) {
                    Ok(f27(0u64, f23(), 1u64, f23()))
                } else {
                    Ok(f3())
                }
            } else {
                let a126 = v0;
                let a127 = 7u64;
                if nat_eq(a126, a127) {
                    Ok(f28(1u64, f23(), 8u64, 1028u64))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a128 = v0;
    let a129 = 4u64;
    if nat_lt(a128, a129) {
        let a130 = v0;
        let a131 = 2u64;
        if nat_lt(a130, a131) {
            let a132 = v0;
            let a133 = 1u64;
            if nat_lt(a132, a133) {
                let a134 = v0;
                let a135 = 0u64;
                if nat_eq(a134, a135) {
                    f29(f22(), f30())
                } else {
                    f3()
                }
            } else {
                let a136 = v0;
                let a137 = 1u64;
                if nat_eq(a136, a137) {
                    f29(f31(), f32())
                } else {
                    f3()
                }
            }
        } else {
            let a138 = v0;
            let a139 = 3u64;
            if nat_lt(a138, a139) {
                let a140 = v0;
                let a141 = 2u64;
                if nat_eq(a140, a141) {
                    f17(1u64, f30(), f32(), f30())
                } else {
                    f3()
                }
            } else {
                let a142 = v0;
                let a143 = 3u64;
                if nat_eq(a142, a143) {
                    f17(2u64, f30(), 1045220557u64, f30())
                } else {
                    f3()
                }
            }
        }
    } else {
        let a144 = v0;
        let a145 = 6u64;
        if nat_lt(a144, a145) {
            let a146 = v0;
            let a147 = 5u64;
            if nat_lt(a146, a147) {
                let a148 = v0;
                let a149 = 4u64;
                if nat_eq(a148, a149) {
                    f17(0u64, 1097859072u64, f30(), f30())
                } else {
                    f3()
                }
            } else {
                let a150 = v0;
                let a151 = 5u64;
                if nat_eq(a150, a151) {
                    f17(3u64, f30(), f18(), f33())
                } else {
                    f3()
                }
            }
        } else {
            let a152 = v0;
            let a153 = 7u64;
            if nat_lt(a152, a153) {
                let a154 = v0;
                let a155 = 6u64;
                if nat_eq(a154, a155) {
                    f17(3u64, 1065353216u64, f33(), f34())
                } else {
                    f3()
                }
            } else {
                let a156 = v0;
                let a157 = 7u64;
                if nat_eq(a156, a157) {
                    f20(0u64, f23())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a158 = v0;
    let a159 = 4u64;
    if nat_lt(a158, a159) {
        let a160 = v0;
        let a161 = 2u64;
        if nat_lt(a160, a161) {
            let a162 = v0;
            let a163 = 1u64;
            if nat_lt(a162, a163) {
                let a164 = v0;
                let a165 = 0u64;
                if nat_eq(a164, a165) {
                    Ok(f20(1042u64, f35()))
                } else {
                    Ok(f3())
                }
            } else {
                let a166 = v0;
                let a167 = 1u64;
                if nat_eq(a166, a167) {
                    Ok(f36(1536u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a168 = v0;
            let a169 = 3u64;
            if nat_lt(a168, a169) {
                let a170 = v0;
                let a171 = 2u64;
                if nat_eq(a170, a171) {
                    Ok(f20(f37(), f38(f23())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a172 = v0;
                let a173 = 3u64;
                if nat_eq(a172, a173) {
                    Ok(f20(f39(), f40(f23())?))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a174 = v0;
        let a175 = 6u64;
        if nat_lt(a174, a175) {
            let a176 = v0;
            let a177 = 5u64;
            if nat_lt(a176, a177) {
                let a178 = v0;
                let a179 = 4u64;
                if nat_eq(a178, a179) {
                    Ok(f27(0u64, f23(), 1u64, f23()))
                } else {
                    Ok(f3())
                }
            } else {
                let a180 = v0;
                let a181 = 5u64;
                if nat_eq(a180, a181) {
                    Ok(f28(1u64, f23(), 8u64, 1040u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a182 = v0;
            let a183 = 7u64;
            if nat_lt(a182, a183) {
                let a184 = v0;
                let a185 = 6u64;
                if nat_eq(a184, a185) {
                    Ok(f20(0u64, f41()))
                } else {
                    Ok(f3())
                }
            } else {
                let a186 = v0;
                let a187 = 7u64;
                if nat_eq(a186, a187) {
                    Ok(f20(1049u64, f42()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a188 = v0;
    let a189 = 4u64;
    if nat_lt(a188, a189) {
        let a190 = v0;
        let a191 = 2u64;
        if nat_lt(a190, a191) {
            let a192 = v0;
            let a193 = 1u64;
            if nat_lt(a192, a193) {
                let a194 = v0;
                let a195 = 0u64;
                if nat_eq(a194, a195) {
                    f36(2048u64)
                } else {
                    f3()
                }
            } else {
                let a196 = v0;
                let a197 = 1u64;
                if nat_eq(a196, a197) {
                    f28(2u64, 0u64, f43(), 1051u64)
                } else {
                    f3()
                }
            }
        } else {
            let a198 = v0;
            let a199 = 3u64;
            if nat_lt(a198, a199) {
                let a200 = v0;
                let a201 = 2u64;
                if nat_eq(a200, a201) {
                    f36(1062u64)
                } else {
                    f3()
                }
            } else {
                let a202 = v0;
                let a203 = 3u64;
                if nat_eq(a202, a203) {
                    f17(2u64, f44(), f44(), f32())
                } else {
                    f3()
                }
            }
        }
    } else {
        let a204 = v0;
        let a205 = 6u64;
        if nat_lt(a204, a205) {
            let a206 = v0;
            let a207 = 5u64;
            if nat_lt(a206, a207) {
                let a208 = v0;
                let a209 = 4u64;
                if nat_eq(a208, a209) {
                    f17(2u64, f21(), f43(), f30())
                } else {
                    f3()
                }
            } else {
                let a210 = v0;
                let a211 = 5u64;
                if nat_eq(a210, a211) {
                    f28(2u64, f30(), f32(), 1055u64)
                } else {
                    f3()
                }
            }
        } else {
            let a212 = v0;
            let a213 = 7u64;
            if nat_lt(a212, a213) {
                let a214 = v0;
                let a215 = 6u64;
                if nat_eq(a214, a215) {
                    f36(1062u64)
                } else {
                    f3()
                }
            } else {
                let a216 = v0;
                let a217 = 7u64;
                if nat_eq(a216, a217) {
                    f17(3u64, f44(), f43(), f19())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a218 = v0;
    let a219 = 4u64;
    if nat_lt(a218, a219) {
        let a220 = v0;
        let a221 = 2u64;
        if nat_lt(a220, a221) {
            let a222 = v0;
            let a223 = 1u64;
            if nat_lt(a222, a223) {
                let a224 = v0;
                let a225 = 0u64;
                if nat_eq(a224, a225) {
                    Ok(f17(2u64, f19(), f44(), f21()))
                } else {
                    Ok(f3())
                }
            } else {
                let a226 = v0;
                let a227 = 1u64;
                if nat_eq(a226, a227) {
                    Ok(f20(0u64, f23()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a228 = v0;
            let a229 = 3u64;
            if nat_lt(a228, a229) {
                let a230 = v0;
                let a231 = 2u64;
                if nat_eq(a230, a231) {
                    Ok(f20(f38(f23())?, f24(f23())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a232 = v0;
                let a233 = 3u64;
                if nat_eq(a232, a233) {
                    Ok(f20(f40(f23())?, f26(f23())?))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a234 = v0;
        let a235 = 6u64;
        if nat_lt(a234, a235) {
            let a236 = v0;
            let a237 = 5u64;
            if nat_lt(a236, a237) {
                let a238 = v0;
                let a239 = 4u64;
                if nat_eq(a238, a239) {
                    Ok(f27(0u64, f23(), 1u64, f23()))
                } else {
                    Ok(f3())
                }
            } else {
                let a240 = v0;
                let a241 = 5u64;
                if nat_eq(a240, a241) {
                    Ok(f28(1u64, f23(), 8u64, 1058u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a242 = v0;
            let a243 = 7u64;
            if nat_lt(a242, a243) {
                let a244 = v0;
                let a245 = 6u64;
                if nat_eq(a244, a245) {
                    Ok(f27(0u64, f22(), 1u64, f22()))
                } else {
                    Ok(f3())
                }
            } else {
                let a246 = v0;
                let a247 = 7u64;
                if nat_eq(a246, a247) {
                    Ok(f27(2u64, f31(), 2u64, f32()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a248 = v0;
    let a249 = 1u64;
    if nat_lt(a248, a249) {
        let a250 = v0;
        let a251 = 0u64;
        if nat_eq(a250, a251) {
            f27(0u64, f32(), 1u64, f32())
        } else {
            f3()
        }
    } else {
        let a252 = v0;
        let a253 = 2u64;
        if nat_lt(a252, a253) {
            let a254 = v0;
            let a255 = 1u64;
            if nat_eq(a254, a255) {
                f28(1u64, f22(), f32(), 1032u64)
            } else {
                f3()
            }
        } else {
            let a256 = v0;
            let a257 = 2u64;
            if nat_eq(a256, a257) {
                f36(2560u64)
            } else {
                f3()
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
                    Ok(f20(0u64, f45()))
                } else {
                    Ok(f3())
                }
            } else {
                let a266 = v0;
                let a267 = 1u64;
                if nat_eq(a266, a267) {
                    Ok(f20(0u64, f46()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a268 = v0;
            let a269 = 3u64;
            if nat_lt(a268, a269) {
                let a270 = v0;
                let a271 = 2u64;
                if nat_eq(a270, a271) {
                    Ok(f27(2u64, f23(), 4u64, f47()))
                } else {
                    Ok(f3())
                }
            } else {
                let a272 = v0;
                let a273 = 3u64;
                if nat_eq(a272, a273) {
                    Ok(f27(0u64, f47(), f46(), f48()))
                } else {
                    Ok(f3())
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
                    Ok(f17(2u64, f33(), f49(f48())?, f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a280 = v0;
                let a281 = 5u64;
                if nat_eq(a280, a281) {
                    Ok(f17(1u64, f32(), 1065353216u64, f32()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a282 = v0;
            let a283 = 7u64;
            if nat_lt(a282, a283) {
                let a284 = v0;
                let a285 = 6u64;
                if nat_eq(a284, a285) {
                    Ok(f17(2u64, 1056964608u64, f32(), f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a286 = v0;
                let a287 = 7u64;
                if nat_eq(a286, a287) {
                    Ok(f17(0u64, f32(), 1262485504u64, f32()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f50(2u64, f32(), 7u64, 0u64, f51()))
                } else {
                    Ok(f3())
                }
            } else {
                let a296 = v0;
                let a297 = 1u64;
                if nat_eq(a296, a297) {
                    Ok(f50(3u64, f45(), f46(), f51(), f45()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a298 = v0;
            let a299 = 3u64;
            if nat_lt(a298, a299) {
                let a300 = v0;
                let a301 = 2u64;
                if nat_eq(a300, a301) {
                    Ok(f27(0u64, f46(), 1u64, f46()))
                } else {
                    Ok(f3())
                }
            } else {
                let a302 = v0;
                let a303 = 3u64;
                if nat_eq(a302, a303) {
                    Ok(f28(1u64, f46(), 4u64, 1539u64))
                } else {
                    Ok(f3())
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
                    Ok(f52(4u64, f45(), 0u64, 0u64, f53(), f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a310 = v0;
                let a311 = 5u64;
                if nat_eq(a310, a311) {
                    Ok(f27(0u64, f53(), 1u64, f53()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a312 = v0;
            let a313 = 7u64;
            if nat_lt(a312, a313) {
                let a314 = v0;
                let a315 = 6u64;
                if nat_eq(a314, a315) {
                    Ok(f20(f54(f53())?, f55()))
                } else {
                    Ok(f3())
                }
            } else {
                let a316 = v0;
                let a317 = 7u64;
                if nat_eq(a316, a317) {
                    Ok(f27(0u64, f53(), 1u64, f53()))
                } else {
                    Ok(f3())
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
                    Ok(f20(f54(f53())?, f56()))
                } else {
                    Ok(f3())
                }
            } else {
                let a326 = v0;
                let a327 = 1u64;
                if nat_eq(a326, a327) {
                    Ok(f20(0u64, f53()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a328 = v0;
            let a329 = 3u64;
            if nat_lt(a328, a329) {
                let a330 = v0;
                let a331 = 2u64;
                if nat_eq(a330, a331) {
                    Ok(f20(f54(f53())?, f57()))
                } else {
                    Ok(f3())
                }
            } else {
                let a332 = v0;
                let a333 = 3u64;
                if nat_eq(a332, a333) {
                    Ok(f27(0u64, f57(), 1u64, f57()))
                } else {
                    Ok(f3())
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
                    Ok(f27(0u64, f55(), f57(), f55()))
                } else {
                    Ok(f3())
                }
            } else {
                let a340 = v0;
                let a341 = 5u64;
                if nat_eq(a340, a341) {
                    Ok(f27(0u64, f56(), f57(), f56()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a342 = v0;
            let a343 = 7u64;
            if nat_lt(a342, a343) {
                let a344 = v0;
                let a345 = 6u64;
                if nat_eq(a344, a345) {
                    Ok(f20(f54(f55())?, f37()))
                } else {
                    Ok(f3())
                }
            } else {
                let a346 = v0;
                let a347 = 7u64;
                if nat_eq(a346, a347) {
                    Ok(f27(0u64, f55(), 1u64, f32()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f20(1u64, f39()))
                } else {
                    Ok(f3())
                }
            } else {
                let a356 = v0;
                let a357 = 1u64;
                if nat_eq(a356, a357) {
                    Ok(f28(0u64, f56(), f32(), 1586u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a358 = v0;
            let a359 = 3u64;
            if nat_lt(a358, a359) {
                let a360 = v0;
                let a361 = 2u64;
                if nat_eq(a360, a361) {
                    Ok(f20(0u64, f39()))
                } else {
                    Ok(f3())
                }
            } else {
                let a362 = v0;
                let a363 = 3u64;
                if nat_eq(a362, a363) {
                    Ok(f20(2139095039u64, f58()))
                } else {
                    Ok(f3())
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
                    Ok(f20(f55(), f59()))
                } else {
                    Ok(f3())
                }
            } else {
                let a370 = v0;
                let a371 = 5u64;
                if nat_eq(a370, a371) {
                    Ok(f20(f54(f59())?, f60()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a372 = v0;
            let a373 = 7u64;
            if nat_lt(a372, a373) {
                let a374 = v0;
                let a375 = 6u64;
                if nat_eq(a374, a375) {
                    Ok(f20(0u64, f61()))
                } else {
                    Ok(f3())
                }
            } else {
                let a376 = v0;
                let a377 = 7u64;
                if nat_eq(a376, a377) {
                    Ok(f20(0u64, f46()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a378 = v0;
    let a379 = 4u64;
    if nat_lt(a378, a379) {
        let a380 = v0;
        let a381 = 2u64;
        if nat_lt(a380, a381) {
            let a382 = v0;
            let a383 = 1u64;
            if nat_lt(a382, a383) {
                let a384 = v0;
                let a385 = 0u64;
                if nat_eq(a384, a385) {
                    Ok(f52(4u64, 0u64, f60(), 0u64, f32(), f62()))
                } else {
                    Ok(f3())
                }
            } else {
                let a386 = v0;
                let a387 = 1u64;
                if nat_eq(a386, a387) {
                    Ok(f27(0u64, f62(), f46(), f62()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a388 = v0;
            let a389 = 3u64;
            if nat_lt(a388, a389) {
                let a390 = v0;
                let a391 = 2u64;
                if nat_eq(a390, a391) {
                    Ok(f50(5u64, f63(f62())?, 0u64, 0u64, f64()))
                } else {
                    Ok(f3())
                }
            } else {
                let a392 = v0;
                let a393 = 3u64;
                if nat_eq(a392, a393) {
                    Ok(f29(f64(), f64()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a394 = v0;
        let a395 = 6u64;
        if nat_lt(a394, a395) {
            let a396 = v0;
            let a397 = 5u64;
            if nat_lt(a396, a397) {
                let a398 = v0;
                let a399 = 4u64;
                if nat_eq(a398, a399) {
                    Ok(f27(0u64, f47(), f46(), f48()))
                } else {
                    Ok(f3())
                }
            } else {
                let a400 = v0;
                let a401 = 5u64;
                if nat_eq(a400, a401) {
                    Ok(f17(2u64, f34(), f64(), f30()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a402 = v0;
            let a403 = 7u64;
            if nat_lt(a402, a403) {
                let a404 = v0;
                let a405 = 6u64;
                if nat_eq(a404, a405) {
                    Ok(f17(1u64, f30(), f49(f48())?, f30()))
                } else {
                    Ok(f3())
                }
            } else {
                let a406 = v0;
                let a407 = 7u64;
                if nat_eq(a406, a407) {
                    Ok(f17(2u64, f65(f48())?, f30(), f32()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a408 = v0;
    let a409 = 4u64;
    if nat_lt(a408, a409) {
        let a410 = v0;
        let a411 = 2u64;
        if nat_lt(a410, a411) {
            let a412 = v0;
            let a413 = 1u64;
            if nat_lt(a412, a413) {
                let a414 = v0;
                let a415 = 0u64;
                if nat_eq(a414, a415) {
                    f17(2u64, f32(), f30(), f32())
                } else {
                    f3()
                }
            } else {
                let a416 = v0;
                let a417 = 1u64;
                if nat_eq(a416, a417) {
                    f17(0u64, f61(), f32(), f61())
                } else {
                    f3()
                }
            }
        } else {
            let a418 = v0;
            let a419 = 3u64;
            if nat_lt(a418, a419) {
                let a420 = v0;
                let a421 = 2u64;
                if nat_eq(a420, a421) {
                    f27(0u64, f46(), 1u64, f46())
                } else {
                    f3()
                }
            } else {
                let a422 = v0;
                let a423 = 3u64;
                if nat_eq(a422, a423) {
                    f28(1u64, f46(), 4u64, 1568u64)
                } else {
                    f3()
                }
            }
        }
    } else {
        let a424 = v0;
        let a425 = 6u64;
        if nat_lt(a424, a425) {
            let a426 = v0;
            let a427 = 5u64;
            if nat_lt(a426, a427) {
                let a428 = v0;
                let a429 = 4u64;
                if nat_eq(a428, a429) {
                    f28(2u64, f61(), f58(), 1582u64)
                } else {
                    f3()
                }
            } else {
                let a430 = v0;
                let a431 = 5u64;
                if nat_eq(a430, a431) {
                    f36(1584u64)
                } else {
                    f3()
                }
            }
        } else {
            let a432 = v0;
            let a433 = 7u64;
            if nat_lt(a432, a433) {
                let a434 = v0;
                let a435 = 6u64;
                if nat_eq(a434, a435) {
                    f20(f61(), f58())
                } else {
                    f3()
                }
            } else {
                let a436 = v0;
                let a437 = 7u64;
                if nat_eq(a436, a437) {
                    f20(f60(), f37())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a438 = v0;
    let a439 = 1u64;
    if nat_lt(a438, a439) {
        let a440 = v0;
        let a441 = 0u64;
        if nat_eq(a440, a441) {
            f27(0u64, f59(), 1u64, f59())
        } else {
            f3()
        }
    } else {
        let a442 = v0;
        let a443 = 2u64;
        if nat_lt(a442, a443) {
            let a444 = v0;
            let a445 = 1u64;
            if nat_eq(a444, a445) {
                f28(1u64, f59(), f56(), 1565u64)
            } else {
                f3()
            }
        } else {
            let a446 = v0;
            let a447 = 2u64;
            if nat_eq(a446, a447) {
                f66(f35())
            } else {
                f3()
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f18() -> u64 {
    4294967315u64
}

pub fn f19() -> u64 {
    4294967316u64
}

pub fn f20(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f21() -> u64 {
    4294967317u64
}

pub fn f22() -> u64 {
    4294967318u64
}

pub fn f23() -> u64 {
    4294967323u64
}

pub fn f24(v0: u64) -> R<u64> {
    let a451 = 8589980672u64;
    let a452 = {
        let a448 = v0;
        let a449 = 4294967296u64;
        let a450 = 0u64;
        nat_rem(a448, a449, a450)
    };
    nat_add(a451, a452)
}

pub fn f25() -> u64 {
    4294967297u64
}

pub fn f26(v0: u64) -> R<u64> {
    let a456 = 8589982720u64;
    let a457 = {
        let a453 = v0;
        let a454 = 4294967296u64;
        let a455 = 0u64;
        nat_rem(a453, a454, a455)
    };
    nat_add(a456, a457)
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f29(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f30() -> u64 {
    4294967337u64
}

pub fn f31() -> u64 {
    4294967299u64
}

pub fn f32() -> u64 {
    4294967338u64
}

pub fn f33() -> u64 {
    4294967319u64
}

pub fn f34() -> u64 {
    4294967320u64
}

pub fn f35() -> u64 {
    4294967350u64
}

pub fn f36(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f37() -> u64 {
    4294967321u64
}

pub fn f38(v0: u64) -> R<u64> {
    let a461 = 8589976576u64;
    let a462 = {
        let a458 = v0;
        let a459 = 4294967296u64;
        let a460 = 0u64;
        nat_rem(a458, a459, a460)
    };
    nat_add(a461, a462)
}

pub fn f39() -> u64 {
    4294967322u64
}

pub fn f40(v0: u64) -> R<u64> {
    let a466 = 8589978624u64;
    let a467 = {
        let a463 = v0;
        let a464 = 4294967296u64;
        let a465 = 0u64;
        nat_rem(a463, a464, a465)
    };
    nat_add(a466, a467)
}

pub fn f41() -> u64 {
    4294967324u64
}

pub fn f42() -> u64 {
    4294967351u64
}

pub fn f43() -> u64 {
    4294967336u64
}

pub fn f44() -> u64 {
    4294967335u64
}

pub fn f45() -> u64 {
    4294967326u64
}

pub fn f46() -> u64 {
    4294967305u64
}

pub fn f47() -> u64 {
    4294967325u64
}

pub fn f48() -> u64 {
    4294967303u64
}

pub fn f49(v0: u64) -> R<u64> {
    let a471 = 8589950976u64;
    let a472 = {
        let a468 = v0;
        let a469 = 4294967296u64;
        let a470 = 0u64;
        nat_rem(a468, a469, a470)
    };
    nat_add(a471, a472)
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f51() -> u64 {
    4294967331u64
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f53() -> u64 {
    4294967343u64
}

pub fn f54(v0: u64) -> R<u64> {
    let a476 = 21474836480u64;
    let a477 = {
        let a473 = v0;
        let a474 = 4294967296u64;
        let a475 = 0u64;
        nat_rem(a473, a474, a475)
    };
    nat_add(a476, a477)
}

pub fn f55() -> u64 {
    4294967327u64
}

pub fn f56() -> u64 {
    4294967328u64
}

pub fn f57() -> u64 {
    4294967339u64
}

pub fn f58() -> u64 {
    4294967334u64
}

pub fn f59() -> u64 {
    4294967329u64
}

pub fn f60() -> u64 {
    4294967353u64
}

pub fn f61() -> u64 {
    4294967333u64
}

pub fn f62() -> u64 {
    4294967330u64
}

pub fn f63(v0: u64) -> R<u64> {
    let a481 = 25769803776u64;
    let a482 = {
        let a478 = v0;
        let a479 = 4294967296u64;
        let a480 = 0u64;
        nat_rem(a478, a479, a480)
    };
    nat_add(a481, a482)
}

pub fn f64() -> u64 {
    4294967332u64
}

pub fn f65(v0: u64) -> R<u64> {
    let a486 = 8589967360u64;
    let a487 = {
        let a483 = v0;
        let a484 = 4294967296u64;
        let a485 = 0u64;
        nat_rem(a483, a484, a485)
    };
    nat_add(a486, a487)
}

pub fn f66(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}
