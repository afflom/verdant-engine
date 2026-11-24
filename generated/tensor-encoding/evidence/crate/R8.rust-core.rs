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
    let a9 = 1u64;
    if nat_lt(a8, a9) {
        let a10 = v1;
        let a11 = 0u64;
        if nat_eq(a10, a11) {
            f4({
                let a12 = v0;
                let a13 = 8u64;
                let a14 = 0u64;
                nat_rem(a12, a13, a14)
            })
        } else {
            Ok(f3())
        }
    } else {
        let a15 = v1;
        let a16 = 1u64;
        if nat_eq(a15, a16) {
            Ok(f5({
                let a17 = v0;
                let a18 = 8u64;
                let a19 = 0u64;
                nat_rem(a17, a18, a19)
            }))
        } else {
            Ok(f3())
        }
    }
}

pub fn f2(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a20 = v0;
        let a21 = 8u64;
        let a22 = 0u64;
        nat_quot(a20, a21, a22)
    };
    let a23 = v1;
    let a24 = 4u64;
    if nat_lt(a23, a24) {
        let a25 = v1;
        let a26 = 2u64;
        if nat_lt(a25, a26) {
            let a27 = v1;
            let a28 = 1u64;
            if nat_lt(a27, a28) {
                let a29 = v1;
                let a30 = 0u64;
                if nat_eq(a29, a30) {
                    Ok(f6({
                        let a31 = v0;
                        let a32 = 8u64;
                        let a33 = 0u64;
                        nat_rem(a31, a32, a33)
                    }))
                } else {
                    Ok(f3())
                }
            } else {
                let a34 = v1;
                let a35 = 1u64;
                if nat_eq(a34, a35) {
                    f7({
                        let a36 = v0;
                        let a37 = 8u64;
                        let a38 = 0u64;
                        nat_rem(a36, a37, a38)
                    })
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a39 = v1;
            let a40 = 3u64;
            if nat_lt(a39, a40) {
                let a41 = v1;
                let a42 = 2u64;
                if nat_eq(a41, a42) {
                    f8({
                        let a43 = v0;
                        let a44 = 8u64;
                        let a45 = 0u64;
                        nat_rem(a43, a44, a45)
                    })
                } else {
                    Ok(f3())
                }
            } else {
                let a46 = v1;
                let a47 = 3u64;
                if nat_eq(a46, a47) {
                    Ok(f9({
                        let a48 = v0;
                        let a49 = 8u64;
                        let a50 = 0u64;
                        nat_rem(a48, a49, a50)
                    }))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a51 = v1;
        let a52 = 6u64;
        if nat_lt(a51, a52) {
            let a53 = v1;
            let a54 = 5u64;
            if nat_lt(a53, a54) {
                let a55 = v1;
                let a56 = 4u64;
                if nat_eq(a55, a56) {
                    Ok(f10({
                        let a57 = v0;
                        let a58 = 8u64;
                        let a59 = 0u64;
                        nat_rem(a57, a58, a59)
                    }))
                } else {
                    Ok(f3())
                }
            } else {
                let a60 = v1;
                let a61 = 5u64;
                if nat_eq(a60, a61) {
                    f11({
                        let a62 = v0;
                        let a63 = 8u64;
                        let a64 = 0u64;
                        nat_rem(a62, a63, a64)
                    })
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a65 = v1;
            let a66 = 7u64;
            if nat_lt(a65, a66) {
                let a67 = v1;
                let a68 = 6u64;
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
            } else {
                let a72 = v1;
                let a73 = 7u64;
                if nat_eq(a72, a73) {
                    Ok(f13({
                        let a74 = v0;
                        let a75 = 8u64;
                        let a76 = 0u64;
                        nat_rem(a74, a75, a76)
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
    let a77 = v0;
    let a78 = 4u64;
    if nat_lt(a77, a78) {
        let a79 = v0;
        let a80 = 2u64;
        if nat_lt(a79, a80) {
            let a81 = v0;
            let a82 = 1u64;
            if nat_lt(a81, a82) {
                let a83 = v0;
                let a84 = 0u64;
                if nat_eq(a83, a84) {
                    Ok(f14(0u64, f15()))
                } else {
                    Ok(f3())
                }
            } else {
                let a85 = v0;
                let a86 = 1u64;
                if nat_eq(a85, a86) {
                    Ok(f14(0u64, f16()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a87 = v0;
            let a88 = 3u64;
            if nat_lt(a87, a88) {
                let a89 = v0;
                let a90 = 2u64;
                if nat_eq(a89, a90) {
                    Ok(f14(0u64, f17()))
                } else {
                    Ok(f3())
                }
            } else {
                let a91 = v0;
                let a92 = 3u64;
                if nat_eq(a91, a92) {
                    Ok(f14(0u64, f18()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a93 = v0;
        let a94 = 6u64;
        if nat_lt(a93, a94) {
            let a95 = v0;
            let a96 = 5u64;
            if nat_lt(a95, a96) {
                let a97 = v0;
                let a98 = 4u64;
                if nat_eq(a97, a98) {
                    Ok(f19(0u64, f20(), 0u64, 512u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a99 = v0;
                let a100 = 5u64;
                if nat_eq(a99, a100) {
                    Ok(f14(0u64, f21()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a101 = v0;
            let a102 = 7u64;
            if nat_lt(a101, a102) {
                let a103 = v0;
                let a104 = 6u64;
                if nat_eq(a103, a104) {
                    Ok(f22(2u64, f23(f21())?, f23(f21())?, f24()))
                } else {
                    Ok(f3())
                }
            } else {
                let a105 = v0;
                let a106 = 7u64;
                if nat_eq(a105, a106) {
                    Ok(f22(0u64, f15(), f24(), f15()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a107 = v0;
    let a108 = 2u64;
    if nat_lt(a107, a108) {
        let a109 = v0;
        let a110 = 1u64;
        if nat_lt(a109, a110) {
            let a111 = v0;
            let a112 = 0u64;
            if nat_eq(a111, a112) {
                f25(0u64, f21(), 1u64, f21())
            } else {
                f3()
            }
        } else {
            let a113 = v0;
            let a114 = 1u64;
            if nat_eq(a113, a114) {
                f19(1u64, f21(), 256u64, 6u64)
            } else {
                f3()
            }
        }
    } else {
        let a115 = v0;
        let a116 = 3u64;
        if nat_lt(a115, a116) {
            let a117 = v0;
            let a118 = 2u64;
            if nat_eq(a117, a118) {
                f22(2u64, 1073741824u64, f15(), f15())
            } else {
                f3()
            }
        } else {
            let a119 = v0;
            let a120 = 4u64;
            if nat_lt(a119, a120) {
                let a121 = v0;
                let a122 = 3u64;
                if nat_eq(a121, a122) {
                    f22(3u64, f15(), 1132462080u64, f15())
                } else {
                    f3()
                }
            } else {
                let a123 = v0;
                let a124 = 4u64;
                if nat_eq(a123, a124) {
                    f26(512u64)
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a125 = v0;
    let a126 = 4u64;
    if nat_lt(a125, a126) {
        let a127 = v0;
        let a128 = 2u64;
        if nat_lt(a127, a128) {
            let a129 = v0;
            let a130 = 1u64;
            if nat_lt(a129, a130) {
                let a131 = v0;
                let a132 = 0u64;
                if nat_eq(a131, a132) {
                    f14(0u64, f27())
                } else {
                    f3()
                }
            } else {
                let a133 = v0;
                let a134 = 1u64;
                if nat_eq(a133, a134) {
                    f14(0u64, f28())
                } else {
                    f3()
                }
            }
        } else {
            let a135 = v0;
            let a136 = 3u64;
            if nat_lt(a135, a136) {
                let a137 = v0;
                let a138 = 2u64;
                if nat_eq(a137, a138) {
                    f25(2u64, f16(), 32u64, f29())
                } else {
                    f3()
                }
            } else {
                let a139 = v0;
                let a140 = 3u64;
                if nat_eq(a139, a140) {
                    f14(0u64, f30())
                } else {
                    f3()
                }
            }
        }
    } else {
        let a141 = v0;
        let a142 = 6u64;
        if nat_lt(a141, a142) {
            let a143 = v0;
            let a144 = 5u64;
            if nat_lt(a143, a144) {
                let a145 = v0;
                let a146 = 4u64;
                if nat_eq(a145, a146) {
                    f14(0u64, f31())
                } else {
                    f3()
                }
            } else {
                let a147 = v0;
                let a148 = 5u64;
                if nat_eq(a147, a148) {
                    f14(0u64, f32())
                } else {
                    f3()
                }
            }
        } else {
            let a149 = v0;
            let a150 = 7u64;
            if nat_lt(a149, a150) {
                let a151 = v0;
                let a152 = 6u64;
                if nat_eq(a151, a152) {
                    f14(0u64, f33())
                } else {
                    f3()
                }
            } else {
                let a153 = v0;
                let a154 = 7u64;
                if nat_eq(a153, a154) {
                    f25(0u64, f29(), f27(), f34())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a155 = v0;
    let a156 = 4u64;
    if nat_lt(a155, a156) {
        let a157 = v0;
        let a158 = 2u64;
        if nat_lt(a157, a158) {
            let a159 = v0;
            let a160 = 1u64;
            if nat_lt(a159, a160) {
                let a161 = v0;
                let a162 = 0u64;
                if nat_eq(a161, a162) {
                    Ok(f14(f23(f34())?, f35()))
                } else {
                    Ok(f3())
                }
            } else {
                let a163 = v0;
                let a164 = 1u64;
                if nat_eq(a163, a164) {
                    Ok(f22(2u64, f35(), f35(), f36()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a165 = v0;
            let a166 = 3u64;
            if nat_lt(a165, a166) {
                let a167 = v0;
                let a168 = 2u64;
                if nat_eq(a167, a168) {
                    Ok(f19(0u64, f20(), 0u64, 526u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a169 = v0;
                let a170 = 3u64;
                if nat_eq(a169, a170) {
                    Ok(f22(0u64, f15(), f36(), f36()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a171 = v0;
        let a172 = 6u64;
        if nat_lt(a171, a172) {
            let a173 = v0;
            let a174 = 5u64;
            if nat_lt(a173, a174) {
                let a175 = v0;
                let a176 = 4u64;
                if nat_eq(a175, a176) {
                    Ok(f22(4u64, f36(), 0u64, f36()))
                } else {
                    Ok(f3())
                }
            } else {
                let a177 = v0;
                let a178 = 5u64;
                if nat_eq(a177, a178) {
                    Ok(f22(2u64, f37(f34())?, f36(), f36()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a179 = v0;
            let a180 = 7u64;
            if nat_lt(a179, a180) {
                let a181 = v0;
                let a182 = 6u64;
                if nat_eq(a181, a182) {
                    Ok(f14(f36(), f38(f27())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a183 = v0;
                let a184 = 7u64;
                if nat_eq(a183, a184) {
                    Ok(f22(4u64, f36(), 0u64, f39(f27())?))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a185 = v0;
    let a186 = 4u64;
    if nat_lt(a185, a186) {
        let a187 = v0;
        let a188 = 2u64;
        if nat_lt(a187, a188) {
            let a189 = v0;
            let a190 = 1u64;
            if nat_lt(a189, a190) {
                let a191 = v0;
                let a192 = 0u64;
                if nat_eq(a191, a192) {
                    Ok(f14(f35(), f40(f27())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a193 = v0;
                let a194 = 1u64;
                if nat_eq(a193, a194) {
                    Ok(f19(2u64, f35(), 0u64, 531u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a195 = v0;
            let a196 = 3u64;
            if nat_lt(a195, a196) {
                let a197 = v0;
                let a198 = 2u64;
                if nat_eq(a197, a198) {
                    Ok(f26(535u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a199 = v0;
                let a200 = 3u64;
                if nat_eq(a199, a200) {
                    Ok(f22(6u64, f35(), 0u64, f40(f27())?))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a201 = v0;
        let a202 = 6u64;
        if nat_lt(a201, a202) {
            let a203 = v0;
            let a204 = 5u64;
            if nat_lt(a203, a204) {
                let a205 = v0;
                let a206 = 4u64;
                if nat_eq(a205, a206) {
                    Ok(f41(6u64, f33(), 0u64, 0u64, f42()))
                } else {
                    Ok(f3())
                }
            } else {
                let a207 = v0;
                let a208 = 5u64;
                if nat_eq(a207, a208) {
                    Ok(f25(0u64, f32(), f42(), f32()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a209 = v0;
            let a210 = 7u64;
            if nat_lt(a209, a210) {
                let a211 = v0;
                let a212 = 6u64;
                if nat_eq(a211, a212) {
                    Ok(f25(1u64, 1u64, f31(), f31()))
                } else {
                    Ok(f3())
                }
            } else {
                let a213 = v0;
                let a214 = 7u64;
                if nat_eq(a213, a214) {
                    Ok(f19(0u64, f43(), 0u64, 543u64))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a215 = v0;
    let a216 = 4u64;
    if nat_lt(a215, a216) {
        let a217 = v0;
        let a218 = 2u64;
        if nat_lt(a217, a218) {
            let a219 = v0;
            let a220 = 1u64;
            if nat_lt(a219, a220) {
                let a221 = v0;
                let a222 = 0u64;
                if nat_eq(a221, a222) {
                    f22(2u64, f36(), f35(), f44())
                } else {
                    f3()
                }
            } else {
                let a223 = v0;
                let a224 = 1u64;
                if nat_eq(a223, a224) {
                    f22(2u64, f44(), f35(), f44())
                } else {
                    f3()
                }
            }
        } else {
            let a225 = v0;
            let a226 = 3u64;
            if nat_lt(a225, a226) {
                let a227 = v0;
                let a228 = 2u64;
                if nat_eq(a227, a228) {
                    f19(0u64, f33(), 0u64, 541u64)
                } else {
                    f3()
                }
            } else {
                let a229 = v0;
                let a230 = 3u64;
                if nat_eq(a229, a230) {
                    f19(2u64, f44(), f45(), 541u64)
                } else {
                    f3()
                }
            }
        }
    } else {
        let a231 = v0;
        let a232 = 6u64;
        if nat_lt(a231, a232) {
            let a233 = v0;
            let a234 = 5u64;
            if nat_lt(a233, a234) {
                let a235 = v0;
                let a236 = 4u64;
                if nat_eq(a235, a236) {
                    f26(543u64)
                } else {
                    f3()
                }
            } else {
                let a237 = v0;
                let a238 = 5u64;
                if nat_eq(a237, a238) {
                    f14(f44(), f45())
                } else {
                    f3()
                }
            }
        } else {
            let a239 = v0;
            let a240 = 7u64;
            if nat_lt(a239, a240) {
                let a241 = v0;
                let a242 = 6u64;
                if nat_eq(a241, a242) {
                    f14(f33(), f46())
                } else {
                    f3()
                }
            } else {
                let a243 = v0;
                let a244 = 7u64;
                if nat_eq(a243, a244) {
                    f25(0u64, f33(), 1u64, f33())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a245 = v0;
    let a246 = 4u64;
    if nat_lt(a245, a246) {
        let a247 = v0;
        let a248 = 2u64;
        if nat_lt(a247, a248) {
            let a249 = v0;
            let a250 = 1u64;
            if nat_lt(a249, a250) {
                let a251 = v0;
                let a252 = 0u64;
                if nat_eq(a251, a252) {
                    f25(0u64, f27(), 1u64, f27())
                } else {
                    f3()
                }
            } else {
                let a253 = v0;
                let a254 = 1u64;
                if nat_eq(a253, a254) {
                    f19(1u64, f33(), 8u64, 519u64)
                } else {
                    f3()
                }
            }
        } else {
            let a255 = v0;
            let a256 = 3u64;
            if nat_lt(a255, a256) {
                let a257 = v0;
                let a258 = 2u64;
                if nat_eq(a257, a258) {
                    f19(0u64, f43(), 0u64, 558u64)
                } else {
                    f3()
                }
            } else {
                let a259 = v0;
                let a260 = 3u64;
                if nat_eq(a259, a260) {
                    f19(0u64, f31(), 0u64, 558u64)
                } else {
                    f3()
                }
            }
        }
    } else {
        let a261 = v0;
        let a262 = 6u64;
        if nat_lt(a261, a262) {
            let a263 = v0;
            let a264 = 5u64;
            if nat_lt(a263, a264) {
                let a265 = v0;
                let a266 = 4u64;
                if nat_eq(a265, a266) {
                    f41(6u64, f46(), 0u64, 0u64, f42())
                } else {
                    f3()
                }
            } else {
                let a267 = v0;
                let a268 = 5u64;
                if nat_eq(a267, a268) {
                    f25(3u64, f32(), f42(), f47())
                } else {
                    f3()
                }
            }
        } else {
            let a269 = v0;
            let a270 = 7u64;
            if nat_lt(a269, a270) {
                let a271 = v0;
                let a272 = 6u64;
                if nat_eq(a271, a272) {
                    f25(4u64, f47(), 2u64, f47())
                } else {
                    f3()
                }
            } else {
                let a273 = v0;
                let a274 = 7u64;
                if nat_eq(a273, a274) {
                    f19(0u64, f47(), 0u64, 554u64)
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a275 = v0;
    let a276 = 4u64;
    if nat_lt(a275, a276) {
        let a277 = v0;
        let a278 = 2u64;
        if nat_lt(a277, a278) {
            let a279 = v0;
            let a280 = 1u64;
            if nat_lt(a279, a280) {
                let a281 = v0;
                let a282 = 0u64;
                if nat_eq(a281, a282) {
                    Ok(f25(1u64, f32(), f42(), f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a283 = v0;
                let a284 = 1u64;
                if nat_eq(a283, a284) {
                    Ok(f26(555u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a285 = v0;
            let a286 = 3u64;
            if nat_lt(a285, a286) {
                let a287 = v0;
                let a288 = 2u64;
                if nat_eq(a287, a288) {
                    Ok(f25(0u64, f32(), f42(), f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a289 = v0;
                let a290 = 3u64;
                if nat_eq(a289, a290) {
                    Ok(f25(2u64, f28(), 8u64, f47()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a291 = v0;
        let a292 = 6u64;
        if nat_lt(a291, a292) {
            let a293 = v0;
            let a294 = 5u64;
            if nat_lt(a293, a294) {
                let a295 = v0;
                let a296 = 4u64;
                if nat_eq(a295, a296) {
                    Ok(f25(0u64, f47(), f46(), f47()))
                } else {
                    Ok(f3())
                }
            } else {
                let a297 = v0;
                let a298 = 5u64;
                if nat_eq(a297, a298) {
                    Ok(f22(6u64, f40(f47())?, 0u64, f40(f47())?))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a299 = v0;
            let a300 = 7u64;
            if nat_lt(a299, a300) {
                let a301 = v0;
                let a302 = 6u64;
                if nat_eq(a301, a302) {
                    Ok(f14(f32(), f48(f28())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a303 = v0;
                let a304 = 7u64;
                if nat_eq(a303, a304) {
                    Ok(f25(0u64, f28(), 1u64, f28()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a305 = v0;
    let a306 = 4u64;
    if nat_lt(a305, a306) {
        let a307 = v0;
        let a308 = 2u64;
        if nat_lt(a307, a308) {
            let a309 = v0;
            let a310 = 1u64;
            if nat_lt(a309, a310) {
                let a311 = v0;
                let a312 = 0u64;
                if nat_eq(a311, a312) {
                    Ok(f19(1u64, f28(), 4u64, 516u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a313 = v0;
                let a314 = 1u64;
                if nat_eq(a313, a314) {
                    Ok(f14(0u64, f27()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a315 = v0;
            let a316 = 3u64;
            if nat_lt(a315, a316) {
                let a317 = v0;
                let a318 = 2u64;
                if nat_eq(a317, a318) {
                    Ok(f19(2u64, f30(), f40(f27())?, 564u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a319 = v0;
                let a320 = 3u64;
                if nat_eq(a319, a320) {
                    Ok(f26(565u64))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a321 = v0;
        let a322 = 6u64;
        if nat_lt(a321, a322) {
            let a323 = v0;
            let a324 = 5u64;
            if nat_lt(a323, a324) {
                let a325 = v0;
                let a326 = 4u64;
                if nat_eq(a325, a326) {
                    Ok(f14(f40(f27())?, f30()))
                } else {
                    Ok(f3())
                }
            } else {
                let a327 = v0;
                let a328 = 5u64;
                if nat_eq(a327, a328) {
                    Ok(f25(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a329 = v0;
            let a330 = 7u64;
            if nat_lt(a329, a330) {
                let a331 = v0;
                let a332 = 6u64;
                if nat_eq(a331, a332) {
                    Ok(f19(1u64, f27(), 32u64, 562u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a333 = v0;
                let a334 = 7u64;
                if nat_eq(a333, a334) {
                    Ok(f14(0u64, f49(f16())?))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a335 = v0;
    let a336 = 1u64;
    if nat_lt(a335, a336) {
        let a337 = v0;
        let a338 = 0u64;
        if nat_eq(a337, a338) {
            f19(0u64, f30(), 0u64, 3109u64)
        } else {
            f3()
        }
    } else {
        let a339 = v0;
        let a340 = 2u64;
        if nat_lt(a339, a340) {
            let a341 = v0;
            let a342 = 1u64;
            if nat_eq(a341, a342) {
                f19(2u64, f30(), f50(), 3109u64)
            } else {
                f3()
            }
        } else {
            let a343 = v0;
            let a344 = 2u64;
            if nat_eq(a343, a344) {
                f26(1024u64)
            } else {
                f3()
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f15() -> u64 {
    4294967348u64
}

pub fn f16() -> u64 {
    4294967301u64
}

pub fn f17() -> u64 {
    4294967302u64
}

pub fn f18() -> u64 {
    4294967347u64
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f20() -> u64 {
    4294967298u64
}

pub fn f21() -> u64 {
    4294967343u64
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f23(v0: u64) -> R<u64> {
    let a348 = 12884901888u64;
    let a349 = {
        let a345 = v0;
        let a346 = 4294967296u64;
        let a347 = 0u64;
        nat_rem(a345, a346, a347)
    };
    nat_add(a348, a349)
}

pub fn f24() -> u64 {
    4294967337u64
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f26(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f27() -> u64 {
    4294967303u64
}

pub fn f28() -> u64 {
    4294967304u64
}

pub fn f29() -> u64 {
    4294967354u64
}

pub fn f30() -> u64 {
    4294967315u64
}

pub fn f31() -> u64 {
    4294967313u64
}

pub fn f32() -> u64 {
    4294967314u64
}

pub fn f33() -> u64 {
    4294967305u64
}

pub fn f34() -> u64 {
    4294967307u64
}

pub fn f35() -> u64 {
    4294967308u64
}

pub fn f36() -> u64 {
    4294967309u64
}

pub fn f37(v0: u64) -> R<u64> {
    let a353 = 17179869184u64;
    let a354 = {
        let a350 = v0;
        let a351 = 4294967296u64;
        let a352 = 0u64;
        nat_rem(a350, a351, a352)
    };
    nat_add(a353, a354)
}

pub fn f38(v0: u64) -> R<u64> {
    let a358 = 8589959168u64;
    let a359 = {
        let a355 = v0;
        let a356 = 4294967296u64;
        let a357 = 0u64;
        nat_rem(a355, a356, a357)
    };
    nat_add(a358, a359)
}

pub fn f39(v0: u64) -> R<u64> {
    let a363 = 8589967360u64;
    let a364 = {
        let a360 = v0;
        let a361 = 4294967296u64;
        let a362 = 0u64;
        nat_rem(a360, a361, a362)
    };
    nat_add(a363, a364)
}

pub fn f40(v0: u64) -> R<u64> {
    let a368 = 8589950976u64;
    let a369 = {
        let a365 = v0;
        let a366 = 4294967296u64;
        let a367 = 0u64;
        nat_rem(a365, a366, a367)
    };
    nat_add(a368, a369)
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f42() -> u64 {
    4294967342u64
}

pub fn f43() -> u64 {
    4294967297u64
}

pub fn f44() -> u64 {
    4294967310u64
}

pub fn f45() -> u64 {
    4294967311u64
}

pub fn f46() -> u64 {
    4294967312u64
}

pub fn f47() -> u64 {
    4294967338u64
}

pub fn f48(v0: u64) -> R<u64> {
    let a373 = 8589975552u64;
    let a374 = {
        let a370 = v0;
        let a371 = 4294967296u64;
        let a372 = 0u64;
        nat_rem(a370, a371, a372)
    };
    nat_add(a373, a374)
}

pub fn f49(v0: u64) -> R<u64> {
    let a378 = 8589984768u64;
    let a379 = {
        let a375 = v0;
        let a376 = 4294967296u64;
        let a377 = 0u64;
        nat_rem(a375, a376, a377)
    };
    nat_add(a378, a379)
}

pub fn f50() -> u64 {
    4294967300u64
}
