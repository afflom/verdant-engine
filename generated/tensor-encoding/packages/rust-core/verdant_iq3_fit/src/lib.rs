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
    let a19 = 1u64;
    if nat_lt(a18, a19) {
        let a20 = v1;
        let a21 = 0u64;
        if nat_eq(a20, a21) {
            f6({
                let a22 = v0;
                let a23 = 8u64;
                let a24 = 0u64;
                nat_rem(a22, a23, a24)
            })
        } else {
            Ok(f2())
        }
    } else {
        let a25 = v1;
        let a26 = 1u64;
        if nat_eq(a25, a26) {
            Ok(f7({
                let a27 = v0;
                let a28 = 8u64;
                let a29 = 0u64;
                nat_rem(a27, a28, a29)
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
        let a30 = v0;
        let a31 = 8u64;
        let a32 = 0u64;
        nat_quot(a30, a31, a32)
    };
    let a33 = v1;
    let a34 = 4u64;
    if nat_lt(a33, a34) {
        let a35 = v1;
        let a36 = 2u64;
        if nat_lt(a35, a36) {
            let a37 = v1;
            let a38 = 1u64;
            if nat_lt(a37, a38) {
                let a39 = v1;
                let a40 = 0u64;
                if nat_eq(a39, a40) {
                    Ok(f8({
                        let a41 = v0;
                        let a42 = 8u64;
                        let a43 = 0u64;
                        nat_rem(a41, a42, a43)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a44 = v1;
                let a45 = 1u64;
                if nat_eq(a44, a45) {
                    f9({
                        let a46 = v0;
                        let a47 = 8u64;
                        let a48 = 0u64;
                        nat_rem(a46, a47, a48)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a49 = v1;
            let a50 = 3u64;
            if nat_lt(a49, a50) {
                let a51 = v1;
                let a52 = 2u64;
                if nat_eq(a51, a52) {
                    f10({
                        let a53 = v0;
                        let a54 = 8u64;
                        let a55 = 0u64;
                        nat_rem(a53, a54, a55)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a56 = v1;
                let a57 = 3u64;
                if nat_eq(a56, a57) {
                    Ok(f11({
                        let a58 = v0;
                        let a59 = 8u64;
                        let a60 = 0u64;
                        nat_rem(a58, a59, a60)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a61 = v1;
        let a62 = 6u64;
        if nat_lt(a61, a62) {
            let a63 = v1;
            let a64 = 5u64;
            if nat_lt(a63, a64) {
                let a65 = v1;
                let a66 = 4u64;
                if nat_eq(a65, a66) {
                    Ok(f12({
                        let a67 = v0;
                        let a68 = 8u64;
                        let a69 = 0u64;
                        nat_rem(a67, a68, a69)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a70 = v1;
                let a71 = 5u64;
                if nat_eq(a70, a71) {
                    f13({
                        let a72 = v0;
                        let a73 = 8u64;
                        let a74 = 0u64;
                        nat_rem(a72, a73, a74)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a75 = v1;
            let a76 = 7u64;
            if nat_lt(a75, a76) {
                let a77 = v1;
                let a78 = 6u64;
                if nat_eq(a77, a78) {
                    f14({
                        let a79 = v0;
                        let a80 = 8u64;
                        let a81 = 0u64;
                        nat_rem(a79, a80, a81)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a82 = v1;
                let a83 = 7u64;
                if nat_eq(a82, a83) {
                    Ok(f15({
                        let a84 = v0;
                        let a85 = 8u64;
                        let a86 = 0u64;
                        nat_rem(a84, a85, a86)
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
        let a87 = v0;
        let a88 = 8u64;
        let a89 = 0u64;
        nat_quot(a87, a88, a89)
    };
    let a90 = v1;
    let a91 = 1u64;
    if nat_lt(a90, a91) {
        let a92 = v1;
        let a93 = 0u64;
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
    } else {
        let a97 = v1;
        let a98 = 1u64;
        if nat_eq(a97, a98) {
            Ok(f17({
                let a99 = v0;
                let a100 = 8u64;
                let a101 = 0u64;
                nat_rem(a99, a100, a101)
            }))
        } else {
            Ok(f2())
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a102 = v0;
        let a103 = 8u64;
        let a104 = 0u64;
        nat_quot(a102, a103, a104)
    };
    let a105 = v1;
    let a106 = 4u64;
    if nat_lt(a105, a106) {
        let a107 = v1;
        let a108 = 2u64;
        if nat_lt(a107, a108) {
            let a109 = v1;
            let a110 = 1u64;
            if nat_lt(a109, a110) {
                let a111 = v1;
                let a112 = 0u64;
                if nat_eq(a111, a112) {
                    Ok(f18({
                        let a113 = v0;
                        let a114 = 8u64;
                        let a115 = 0u64;
                        nat_rem(a113, a114, a115)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a116 = v1;
                let a117 = 1u64;
                if nat_eq(a116, a117) {
                    f19({
                        let a118 = v0;
                        let a119 = 8u64;
                        let a120 = 0u64;
                        nat_rem(a118, a119, a120)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a121 = v1;
            let a122 = 3u64;
            if nat_lt(a121, a122) {
                let a123 = v1;
                let a124 = 2u64;
                if nat_eq(a123, a124) {
                    f20({
                        let a125 = v0;
                        let a126 = 8u64;
                        let a127 = 0u64;
                        nat_rem(a125, a126, a127)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a128 = v1;
                let a129 = 3u64;
                if nat_eq(a128, a129) {
                    Ok(f21({
                        let a130 = v0;
                        let a131 = 8u64;
                        let a132 = 0u64;
                        nat_rem(a130, a131, a132)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a133 = v1;
        let a134 = 6u64;
        if nat_lt(a133, a134) {
            let a135 = v1;
            let a136 = 5u64;
            if nat_lt(a135, a136) {
                let a137 = v1;
                let a138 = 4u64;
                if nat_eq(a137, a138) {
                    Ok(f22({
                        let a139 = v0;
                        let a140 = 8u64;
                        let a141 = 0u64;
                        nat_rem(a139, a140, a141)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a142 = v1;
                let a143 = 5u64;
                if nat_eq(a142, a143) {
                    Ok(f23({
                        let a144 = v0;
                        let a145 = 8u64;
                        let a146 = 0u64;
                        nat_rem(a144, a145, a146)
                    }))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a147 = v1;
            let a148 = 7u64;
            if nat_lt(a147, a148) {
                let a149 = v1;
                let a150 = 6u64;
                if nat_eq(a149, a150) {
                    f24({
                        let a151 = v0;
                        let a152 = 8u64;
                        let a153 = 0u64;
                        nat_rem(a151, a152, a153)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a154 = v1;
                let a155 = 7u64;
                if nat_eq(a154, a155) {
                    f25({
                        let a156 = v0;
                        let a157 = 8u64;
                        let a158 = 0u64;
                        nat_rem(a156, a157, a158)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a159 = v0;
    let a160 = 4u64;
    if nat_lt(a159, a160) {
        let a161 = v0;
        let a162 = 2u64;
        if nat_lt(a161, a162) {
            let a163 = v0;
            let a164 = 1u64;
            if nat_lt(a163, a164) {
                let a165 = v0;
                let a166 = 0u64;
                if nat_eq(a165, a166) {
                    Ok(f26(0u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a167 = v0;
                let a168 = 1u64;
                if nat_eq(a167, a168) {
                    Ok(f26(0u64, f28()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a169 = v0;
            let a170 = 3u64;
            if nat_lt(a169, a170) {
                let a171 = v0;
                let a172 = 2u64;
                if nat_eq(a171, a172) {
                    Ok(f26(0u64, f29()))
                } else {
                    Ok(f2())
                }
            } else {
                let a173 = v0;
                let a174 = 3u64;
                if nat_eq(a173, a174) {
                    Ok(f26(0u64, f30()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a175 = v0;
        let a176 = 6u64;
        if nat_lt(a175, a176) {
            let a177 = v0;
            let a178 = 5u64;
            if nat_lt(a177, a178) {
                let a179 = v0;
                let a180 = 4u64;
                if nat_eq(a179, a180) {
                    Ok(f31(0u64, f32(), 0u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a181 = v0;
                let a182 = 5u64;
                if nat_eq(a181, a182) {
                    Ok(f26(0u64, f33()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a183 = v0;
            let a184 = 7u64;
            if nat_lt(a183, a184) {
                let a185 = v0;
                let a186 = 6u64;
                if nat_eq(a185, a186) {
                    Ok(f34(2u64, f35(f33())?, f35(f33())?, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a187 = v0;
                let a188 = 7u64;
                if nat_eq(a187, a188) {
                    Ok(f34(0u64, f27(), f36(), f27()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a189 = v0;
    let a190 = 2u64;
    if nat_lt(a189, a190) {
        let a191 = v0;
        let a192 = 1u64;
        if nat_lt(a191, a192) {
            let a193 = v0;
            let a194 = 0u64;
            if nat_eq(a193, a194) {
                f37(0u64, f33(), 1u64, f33())
            } else {
                f2()
            }
        } else {
            let a195 = v0;
            let a196 = 1u64;
            if nat_eq(a195, a196) {
                f31(1u64, f33(), 256u64, 6u64)
            } else {
                f2()
            }
        }
    } else {
        let a197 = v0;
        let a198 = 3u64;
        if nat_lt(a197, a198) {
            let a199 = v0;
            let a200 = 2u64;
            if nat_eq(a199, a200) {
                f34(2u64, 1073741824u64, f27(), f27())
            } else {
                f2()
            }
        } else {
            let a201 = v0;
            let a202 = 4u64;
            if nat_lt(a201, a202) {
                let a203 = v0;
                let a204 = 3u64;
                if nat_eq(a203, a204) {
                    f34(3u64, f27(), 1132462080u64, f27())
                } else {
                    f2()
                }
            } else {
                let a205 = v0;
                let a206 = 4u64;
                if nat_eq(a205, a206) {
                    f38(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a207 = v0;
    let a208 = 4u64;
    if nat_lt(a207, a208) {
        let a209 = v0;
        let a210 = 2u64;
        if nat_lt(a209, a210) {
            let a211 = v0;
            let a212 = 1u64;
            if nat_lt(a211, a212) {
                let a213 = v0;
                let a214 = 0u64;
                if nat_eq(a213, a214) {
                    f26(0u64, f39())
                } else {
                    f2()
                }
            } else {
                let a215 = v0;
                let a216 = 1u64;
                if nat_eq(a215, a216) {
                    f26(0u64, f40())
                } else {
                    f2()
                }
            }
        } else {
            let a217 = v0;
            let a218 = 3u64;
            if nat_lt(a217, a218) {
                let a219 = v0;
                let a220 = 2u64;
                if nat_eq(a219, a220) {
                    f37(2u64, f28(), 32u64, f41())
                } else {
                    f2()
                }
            } else {
                let a221 = v0;
                let a222 = 3u64;
                if nat_eq(a221, a222) {
                    f26(0u64, f42())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a223 = v0;
        let a224 = 6u64;
        if nat_lt(a223, a224) {
            let a225 = v0;
            let a226 = 5u64;
            if nat_lt(a225, a226) {
                let a227 = v0;
                let a228 = 4u64;
                if nat_eq(a227, a228) {
                    f26(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a229 = v0;
                let a230 = 5u64;
                if nat_eq(a229, a230) {
                    f26(0u64, f44())
                } else {
                    f2()
                }
            }
        } else {
            let a231 = v0;
            let a232 = 7u64;
            if nat_lt(a231, a232) {
                let a233 = v0;
                let a234 = 6u64;
                if nat_eq(a233, a234) {
                    f26(0u64, f45())
                } else {
                    f2()
                }
            } else {
                let a235 = v0;
                let a236 = 7u64;
                if nat_eq(a235, a236) {
                    f37(0u64, f41(), f39(), f46())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a237 = v0;
    let a238 = 4u64;
    if nat_lt(a237, a238) {
        let a239 = v0;
        let a240 = 2u64;
        if nat_lt(a239, a240) {
            let a241 = v0;
            let a242 = 1u64;
            if nat_lt(a241, a242) {
                let a243 = v0;
                let a244 = 0u64;
                if nat_eq(a243, a244) {
                    Ok(f26(f35(f46())?, f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a245 = v0;
                let a246 = 1u64;
                if nat_eq(a245, a246) {
                    Ok(f34(2u64, f47(), f47(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a247 = v0;
            let a248 = 3u64;
            if nat_lt(a247, a248) {
                let a249 = v0;
                let a250 = 2u64;
                if nat_eq(a249, a250) {
                    Ok(f31(0u64, f32(), 0u64, 526u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a251 = v0;
                let a252 = 3u64;
                if nat_eq(a251, a252) {
                    Ok(f34(0u64, f27(), f48(), f48()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a253 = v0;
        let a254 = 6u64;
        if nat_lt(a253, a254) {
            let a255 = v0;
            let a256 = 5u64;
            if nat_lt(a255, a256) {
                let a257 = v0;
                let a258 = 4u64;
                if nat_eq(a257, a258) {
                    Ok(f34(4u64, f48(), 0u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a259 = v0;
                let a260 = 5u64;
                if nat_eq(a259, a260) {
                    Ok(f34(2u64, f49(f46())?, f48(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a261 = v0;
            let a262 = 7u64;
            if nat_lt(a261, a262) {
                let a263 = v0;
                let a264 = 6u64;
                if nat_eq(a263, a264) {
                    Ok(f26(f48(), f50(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a265 = v0;
                let a266 = 7u64;
                if nat_eq(a265, a266) {
                    Ok(f34(4u64, f48(), 0u64, f51(f39())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a267 = v0;
    let a268 = 4u64;
    if nat_lt(a267, a268) {
        let a269 = v0;
        let a270 = 2u64;
        if nat_lt(a269, a270) {
            let a271 = v0;
            let a272 = 1u64;
            if nat_lt(a271, a272) {
                let a273 = v0;
                let a274 = 0u64;
                if nat_eq(a273, a274) {
                    Ok(f26(f47(), f52(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a275 = v0;
                let a276 = 1u64;
                if nat_eq(a275, a276) {
                    Ok(f31(2u64, f47(), 0u64, 531u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a277 = v0;
            let a278 = 3u64;
            if nat_lt(a277, a278) {
                let a279 = v0;
                let a280 = 2u64;
                if nat_eq(a279, a280) {
                    Ok(f38(535u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a281 = v0;
                let a282 = 3u64;
                if nat_eq(a281, a282) {
                    Ok(f34(6u64, f47(), 0u64, f52(f39())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a283 = v0;
        let a284 = 6u64;
        if nat_lt(a283, a284) {
            let a285 = v0;
            let a286 = 5u64;
            if nat_lt(a285, a286) {
                let a287 = v0;
                let a288 = 4u64;
                if nat_eq(a287, a288) {
                    Ok(f53(6u64, f45(), 0u64, 0u64, f54()))
                } else {
                    Ok(f2())
                }
            } else {
                let a289 = v0;
                let a290 = 5u64;
                if nat_eq(a289, a290) {
                    Ok(f37(0u64, f44(), f54(), f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a291 = v0;
            let a292 = 7u64;
            if nat_lt(a291, a292) {
                let a293 = v0;
                let a294 = 6u64;
                if nat_eq(a293, a294) {
                    Ok(f37(1u64, 1u64, f43(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a295 = v0;
                let a296 = 7u64;
                if nat_eq(a295, a296) {
                    Ok(f31(0u64, f55(), 0u64, 543u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a297 = v0;
    let a298 = 4u64;
    if nat_lt(a297, a298) {
        let a299 = v0;
        let a300 = 2u64;
        if nat_lt(a299, a300) {
            let a301 = v0;
            let a302 = 1u64;
            if nat_lt(a301, a302) {
                let a303 = v0;
                let a304 = 0u64;
                if nat_eq(a303, a304) {
                    f34(2u64, f48(), f47(), f56())
                } else {
                    f2()
                }
            } else {
                let a305 = v0;
                let a306 = 1u64;
                if nat_eq(a305, a306) {
                    f34(2u64, f56(), f47(), f56())
                } else {
                    f2()
                }
            }
        } else {
            let a307 = v0;
            let a308 = 3u64;
            if nat_lt(a307, a308) {
                let a309 = v0;
                let a310 = 2u64;
                if nat_eq(a309, a310) {
                    f31(0u64, f45(), 0u64, 541u64)
                } else {
                    f2()
                }
            } else {
                let a311 = v0;
                let a312 = 3u64;
                if nat_eq(a311, a312) {
                    f31(2u64, f56(), f57(), 541u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a313 = v0;
        let a314 = 6u64;
        if nat_lt(a313, a314) {
            let a315 = v0;
            let a316 = 5u64;
            if nat_lt(a315, a316) {
                let a317 = v0;
                let a318 = 4u64;
                if nat_eq(a317, a318) {
                    f38(543u64)
                } else {
                    f2()
                }
            } else {
                let a319 = v0;
                let a320 = 5u64;
                if nat_eq(a319, a320) {
                    f26(f56(), f57())
                } else {
                    f2()
                }
            }
        } else {
            let a321 = v0;
            let a322 = 7u64;
            if nat_lt(a321, a322) {
                let a323 = v0;
                let a324 = 6u64;
                if nat_eq(a323, a324) {
                    f26(f45(), f58())
                } else {
                    f2()
                }
            } else {
                let a325 = v0;
                let a326 = 7u64;
                if nat_eq(a325, a326) {
                    f37(0u64, f45(), 1u64, f45())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a327 = v0;
    let a328 = 4u64;
    if nat_lt(a327, a328) {
        let a329 = v0;
        let a330 = 2u64;
        if nat_lt(a329, a330) {
            let a331 = v0;
            let a332 = 1u64;
            if nat_lt(a331, a332) {
                let a333 = v0;
                let a334 = 0u64;
                if nat_eq(a333, a334) {
                    f37(0u64, f39(), 1u64, f39())
                } else {
                    f2()
                }
            } else {
                let a335 = v0;
                let a336 = 1u64;
                if nat_eq(a335, a336) {
                    f31(1u64, f45(), 8u64, 519u64)
                } else {
                    f2()
                }
            }
        } else {
            let a337 = v0;
            let a338 = 3u64;
            if nat_lt(a337, a338) {
                let a339 = v0;
                let a340 = 2u64;
                if nat_eq(a339, a340) {
                    f31(0u64, f55(), 0u64, 558u64)
                } else {
                    f2()
                }
            } else {
                let a341 = v0;
                let a342 = 3u64;
                if nat_eq(a341, a342) {
                    f31(0u64, f43(), 0u64, 558u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a343 = v0;
        let a344 = 6u64;
        if nat_lt(a343, a344) {
            let a345 = v0;
            let a346 = 5u64;
            if nat_lt(a345, a346) {
                let a347 = v0;
                let a348 = 4u64;
                if nat_eq(a347, a348) {
                    f53(6u64, f58(), 0u64, 0u64, f54())
                } else {
                    f2()
                }
            } else {
                let a349 = v0;
                let a350 = 5u64;
                if nat_eq(a349, a350) {
                    f37(3u64, f44(), f54(), f59())
                } else {
                    f2()
                }
            }
        } else {
            let a351 = v0;
            let a352 = 7u64;
            if nat_lt(a351, a352) {
                let a353 = v0;
                let a354 = 6u64;
                if nat_eq(a353, a354) {
                    f37(4u64, f59(), 2u64, f59())
                } else {
                    f2()
                }
            } else {
                let a355 = v0;
                let a356 = 7u64;
                if nat_eq(a355, a356) {
                    f31(0u64, f59(), 0u64, 554u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a357 = v0;
    let a358 = 4u64;
    if nat_lt(a357, a358) {
        let a359 = v0;
        let a360 = 2u64;
        if nat_lt(a359, a360) {
            let a361 = v0;
            let a362 = 1u64;
            if nat_lt(a361, a362) {
                let a363 = v0;
                let a364 = 0u64;
                if nat_eq(a363, a364) {
                    Ok(f37(1u64, f44(), f54(), f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a365 = v0;
                let a366 = 1u64;
                if nat_eq(a365, a366) {
                    Ok(f38(555u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a367 = v0;
            let a368 = 3u64;
            if nat_lt(a367, a368) {
                let a369 = v0;
                let a370 = 2u64;
                if nat_eq(a369, a370) {
                    Ok(f37(0u64, f44(), f54(), f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a371 = v0;
                let a372 = 3u64;
                if nat_eq(a371, a372) {
                    Ok(f37(2u64, f40(), 8u64, f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a373 = v0;
        let a374 = 6u64;
        if nat_lt(a373, a374) {
            let a375 = v0;
            let a376 = 5u64;
            if nat_lt(a375, a376) {
                let a377 = v0;
                let a378 = 4u64;
                if nat_eq(a377, a378) {
                    Ok(f37(0u64, f59(), f58(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a379 = v0;
                let a380 = 5u64;
                if nat_eq(a379, a380) {
                    Ok(f34(6u64, f52(f59())?, 0u64, f52(f59())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a381 = v0;
            let a382 = 7u64;
            if nat_lt(a381, a382) {
                let a383 = v0;
                let a384 = 6u64;
                if nat_eq(a383, a384) {
                    Ok(f26(f44(), f60(f40())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a385 = v0;
                let a386 = 7u64;
                if nat_eq(a385, a386) {
                    Ok(f37(0u64, f40(), 1u64, f40()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a387 = v0;
    let a388 = 4u64;
    if nat_lt(a387, a388) {
        let a389 = v0;
        let a390 = 2u64;
        if nat_lt(a389, a390) {
            let a391 = v0;
            let a392 = 1u64;
            if nat_lt(a391, a392) {
                let a393 = v0;
                let a394 = 0u64;
                if nat_eq(a393, a394) {
                    Ok(f31(1u64, f40(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a395 = v0;
                let a396 = 1u64;
                if nat_eq(a395, a396) {
                    Ok(f26(0u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a397 = v0;
            let a398 = 3u64;
            if nat_lt(a397, a398) {
                let a399 = v0;
                let a400 = 2u64;
                if nat_eq(a399, a400) {
                    Ok(f31(2u64, f42(), f52(f39())?, 564u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a401 = v0;
                let a402 = 3u64;
                if nat_eq(a401, a402) {
                    Ok(f38(565u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a403 = v0;
        let a404 = 6u64;
        if nat_lt(a403, a404) {
            let a405 = v0;
            let a406 = 5u64;
            if nat_lt(a405, a406) {
                let a407 = v0;
                let a408 = 4u64;
                if nat_eq(a407, a408) {
                    Ok(f26(f52(f39())?, f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a409 = v0;
                let a410 = 5u64;
                if nat_eq(a409, a410) {
                    Ok(f37(0u64, f39(), 1u64, f39()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a411 = v0;
            let a412 = 7u64;
            if nat_lt(a411, a412) {
                let a413 = v0;
                let a414 = 6u64;
                if nat_eq(a413, a414) {
                    Ok(f31(1u64, f39(), 32u64, 562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a415 = v0;
                let a416 = 7u64;
                if nat_eq(a415, a416) {
                    Ok(f26(0u64, f61(f28())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a417 = v0;
    let a418 = 1u64;
    if nat_lt(a417, a418) {
        let a419 = v0;
        let a420 = 0u64;
        if nat_eq(a419, a420) {
            f31(0u64, f42(), 0u64, 3109u64)
        } else {
            f2()
        }
    } else {
        let a421 = v0;
        let a422 = 2u64;
        if nat_lt(a421, a422) {
            let a423 = v0;
            let a424 = 1u64;
            if nat_eq(a423, a424) {
                f31(2u64, f42(), f62(), 3109u64)
            } else {
                f2()
            }
        } else {
            let a425 = v0;
            let a426 = 2u64;
            if nat_eq(a425, a426) {
                f38(1024u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f26(0u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 1u64;
                if nat_eq(a435, a436) {
                    Ok(f26(0u64, f28()))
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
                    Ok(f26(0u64, f29()))
                } else {
                    Ok(f2())
                }
            } else {
                let a441 = v0;
                let a442 = 3u64;
                if nat_eq(a441, a442) {
                    Ok(f26(0u64, f30()))
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
                    Ok(f26(0u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a449 = v0;
                let a450 = 5u64;
                if nat_eq(a449, a450) {
                    Ok(f34(2u64, f35(f33())?, f35(f33())?, f36()))
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
                    Ok(f34(0u64, f27(), f36(), f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a455 = v0;
                let a456 = 7u64;
                if nat_eq(a455, a456) {
                    Ok(f37(0u64, f33(), 1u64, f33()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a457 = v0;
    let a458 = 2u64;
    if nat_lt(a457, a458) {
        let a459 = v0;
        let a460 = 1u64;
        if nat_lt(a459, a460) {
            let a461 = v0;
            let a462 = 0u64;
            if nat_eq(a461, a462) {
                f31(1u64, f33(), 256u64, 5u64)
            } else {
                f2()
            }
        } else {
            let a463 = v0;
            let a464 = 1u64;
            if nat_eq(a463, a464) {
                f31(0u64, f55(), 1u64, 11u64)
            } else {
                f2()
            }
        }
    } else {
        let a465 = v0;
        let a466 = 3u64;
        if nat_lt(a465, a466) {
            let a467 = v0;
            let a468 = 2u64;
            if nat_eq(a467, a468) {
                f34(2u64, 1073741824u64, f27(), f27())
            } else {
                f2()
            }
        } else {
            let a469 = v0;
            let a470 = 4u64;
            if nat_lt(a469, a470) {
                let a471 = v0;
                let a472 = 3u64;
                if nat_eq(a471, a472) {
                    f34(3u64, f27(), 1132462080u64, f27())
                } else {
                    f2()
                }
            } else {
                let a473 = v0;
                let a474 = 4u64;
                if nat_eq(a473, a474) {
                    f38(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a475 = v0;
    let a476 = 4u64;
    if nat_lt(a475, a476) {
        let a477 = v0;
        let a478 = 2u64;
        if nat_lt(a477, a478) {
            let a479 = v0;
            let a480 = 1u64;
            if nat_lt(a479, a480) {
                let a481 = v0;
                let a482 = 0u64;
                if nat_eq(a481, a482) {
                    f26(0u64, f39())
                } else {
                    f2()
                }
            } else {
                let a483 = v0;
                let a484 = 1u64;
                if nat_eq(a483, a484) {
                    f26(0u64, f40())
                } else {
                    f2()
                }
            }
        } else {
            let a485 = v0;
            let a486 = 3u64;
            if nat_lt(a485, a486) {
                let a487 = v0;
                let a488 = 2u64;
                if nat_eq(a487, a488) {
                    f37(2u64, f28(), 16u64, f41())
                } else {
                    f2()
                }
            } else {
                let a489 = v0;
                let a490 = 3u64;
                if nat_eq(a489, a490) {
                    f26(0u64, f42())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a491 = v0;
        let a492 = 6u64;
        if nat_lt(a491, a492) {
            let a493 = v0;
            let a494 = 5u64;
            if nat_lt(a493, a494) {
                let a495 = v0;
                let a496 = 4u64;
                if nat_eq(a495, a496) {
                    f26(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a497 = v0;
                let a498 = 5u64;
                if nat_eq(a497, a498) {
                    f26(0u64, f44())
                } else {
                    f2()
                }
            }
        } else {
            let a499 = v0;
            let a500 = 7u64;
            if nat_lt(a499, a500) {
                let a501 = v0;
                let a502 = 6u64;
                if nat_eq(a501, a502) {
                    f26(0u64, f45())
                } else {
                    f2()
                }
            } else {
                let a503 = v0;
                let a504 = 7u64;
                if nat_eq(a503, a504) {
                    f37(0u64, f41(), f39(), f46())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a505 = v0;
    let a506 = 4u64;
    if nat_lt(a505, a506) {
        let a507 = v0;
        let a508 = 2u64;
        if nat_lt(a507, a508) {
            let a509 = v0;
            let a510 = 1u64;
            if nat_lt(a509, a510) {
                let a511 = v0;
                let a512 = 0u64;
                if nat_eq(a511, a512) {
                    Ok(f26(f35(f46())?, f47()))
                } else {
                    Ok(f2())
                }
            } else {
                let a513 = v0;
                let a514 = 1u64;
                if nat_eq(a513, a514) {
                    Ok(f34(2u64, f47(), f47(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a515 = v0;
            let a516 = 3u64;
            if nat_lt(a515, a516) {
                let a517 = v0;
                let a518 = 2u64;
                if nat_eq(a517, a518) {
                    Ok(f31(0u64, f32(), 0u64, 527u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a519 = v0;
                let a520 = 3u64;
                if nat_eq(a519, a520) {
                    Ok(f34(0u64, f27(), f48(), f48()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a521 = v0;
        let a522 = 6u64;
        if nat_lt(a521, a522) {
            let a523 = v0;
            let a524 = 5u64;
            if nat_lt(a523, a524) {
                let a525 = v0;
                let a526 = 4u64;
                if nat_eq(a525, a526) {
                    Ok(f34(4u64, f48(), 0u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a527 = v0;
                let a528 = 5u64;
                if nat_eq(a527, a528) {
                    Ok(f34(2u64, f49(f46())?, f48(), f48()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a529 = v0;
            let a530 = 7u64;
            if nat_lt(a529, a530) {
                let a531 = v0;
                let a532 = 6u64;
                if nat_eq(a531, a532) {
                    Ok(f38(529u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a533 = v0;
                let a534 = 7u64;
                if nat_eq(a533, a534) {
                    Ok(f34(2u64, 1048576000u64, f27(), f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a535 = v0;
    let a536 = 4u64;
    if nat_lt(a535, a536) {
        let a537 = v0;
        let a538 = 2u64;
        if nat_lt(a537, a538) {
            let a539 = v0;
            let a540 = 1u64;
            if nat_lt(a539, a540) {
                let a541 = v0;
                let a542 = 0u64;
                if nat_eq(a541, a542) {
                    Ok(f34(0u64, f59(), f48(), f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a543 = v0;
                let a544 = 1u64;
                if nat_eq(a543, a544) {
                    Ok(f26(f48(), f50(f39())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a545 = v0;
            let a546 = 3u64;
            if nat_lt(a545, a546) {
                let a547 = v0;
                let a548 = 2u64;
                if nat_eq(a547, a548) {
                    Ok(f34(4u64, f48(), 0u64, f51(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a549 = v0;
                let a550 = 3u64;
                if nat_eq(a549, a550) {
                    Ok(f26(f47(), f52(f39())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a551 = v0;
        let a552 = 6u64;
        if nat_lt(a551, a552) {
            let a553 = v0;
            let a554 = 5u64;
            if nat_lt(a553, a554) {
                let a555 = v0;
                let a556 = 4u64;
                if nat_eq(a555, a556) {
                    Ok(f31(2u64, f47(), 0u64, 534u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a557 = v0;
                let a558 = 5u64;
                if nat_eq(a557, a558) {
                    Ok(f38(538u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a559 = v0;
            let a560 = 7u64;
            if nat_lt(a559, a560) {
                let a561 = v0;
                let a562 = 6u64;
                if nat_eq(a561, a562) {
                    Ok(f34(6u64, f47(), 0u64, f52(f39())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a563 = v0;
                let a564 = 7u64;
                if nat_eq(a563, a564) {
                    Ok(f53(6u64, f45(), 0u64, 0u64, f54()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a565 = v0;
    let a566 = 4u64;
    if nat_lt(a565, a566) {
        let a567 = v0;
        let a568 = 2u64;
        if nat_lt(a567, a568) {
            let a569 = v0;
            let a570 = 1u64;
            if nat_lt(a569, a570) {
                let a571 = v0;
                let a572 = 0u64;
                if nat_eq(a571, a572) {
                    f37(0u64, f44(), f54(), f44())
                } else {
                    f2()
                }
            } else {
                let a573 = v0;
                let a574 = 1u64;
                if nat_eq(a573, a574) {
                    f37(1u64, 1u64, f43(), f43())
                } else {
                    f2()
                }
            }
        } else {
            let a575 = v0;
            let a576 = 3u64;
            if nat_lt(a575, a576) {
                let a577 = v0;
                let a578 = 2u64;
                if nat_eq(a577, a578) {
                    f31(0u64, f55(), 0u64, 546u64)
                } else {
                    f2()
                }
            } else {
                let a579 = v0;
                let a580 = 3u64;
                if nat_eq(a579, a580) {
                    f34(2u64, f48(), f47(), f56())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a581 = v0;
        let a582 = 6u64;
        if nat_lt(a581, a582) {
            let a583 = v0;
            let a584 = 5u64;
            if nat_lt(a583, a584) {
                let a585 = v0;
                let a586 = 4u64;
                if nat_eq(a585, a586) {
                    f34(2u64, f56(), f47(), f56())
                } else {
                    f2()
                }
            } else {
                let a587 = v0;
                let a588 = 5u64;
                if nat_eq(a587, a588) {
                    f31(0u64, f45(), 0u64, 544u64)
                } else {
                    f2()
                }
            }
        } else {
            let a589 = v0;
            let a590 = 7u64;
            if nat_lt(a589, a590) {
                let a591 = v0;
                let a592 = 6u64;
                if nat_eq(a591, a592) {
                    f31(2u64, f56(), f57(), 544u64)
                } else {
                    f2()
                }
            } else {
                let a593 = v0;
                let a594 = 7u64;
                if nat_eq(a593, a594) {
                    f38(546u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a595 = v0;
    let a596 = 4u64;
    if nat_lt(a595, a596) {
        let a597 = v0;
        let a598 = 2u64;
        if nat_lt(a597, a598) {
            let a599 = v0;
            let a600 = 1u64;
            if nat_lt(a599, a600) {
                let a601 = v0;
                let a602 = 0u64;
                if nat_eq(a601, a602) {
                    f26(f56(), f57())
                } else {
                    f2()
                }
            } else {
                let a603 = v0;
                let a604 = 1u64;
                if nat_eq(a603, a604) {
                    f26(f45(), f58())
                } else {
                    f2()
                }
            }
        } else {
            let a605 = v0;
            let a606 = 3u64;
            if nat_lt(a605, a606) {
                let a607 = v0;
                let a608 = 2u64;
                if nat_eq(a607, a608) {
                    f37(0u64, f45(), 1u64, f45())
                } else {
                    f2()
                }
            } else {
                let a609 = v0;
                let a610 = 3u64;
                if nat_eq(a609, a610) {
                    f37(0u64, f39(), 1u64, f39())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a611 = v0;
        let a612 = 6u64;
        if nat_lt(a611, a612) {
            let a613 = v0;
            let a614 = 5u64;
            if nat_lt(a613, a614) {
                let a615 = v0;
                let a616 = 4u64;
                if nat_eq(a615, a616) {
                    f31(1u64, f45(), 8u64, 519u64)
                } else {
                    f2()
                }
            } else {
                let a617 = v0;
                let a618 = 5u64;
                if nat_eq(a617, a618) {
                    f31(0u64, f55(), 0u64, 561u64)
                } else {
                    f2()
                }
            }
        } else {
            let a619 = v0;
            let a620 = 7u64;
            if nat_lt(a619, a620) {
                let a621 = v0;
                let a622 = 6u64;
                if nat_eq(a621, a622) {
                    f31(0u64, f43(), 0u64, 561u64)
                } else {
                    f2()
                }
            } else {
                let a623 = v0;
                let a624 = 7u64;
                if nat_eq(a623, a624) {
                    f53(6u64, f58(), 0u64, 0u64, f54())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a625 = v0;
    let a626 = 4u64;
    if nat_lt(a625, a626) {
        let a627 = v0;
        let a628 = 2u64;
        if nat_lt(a627, a628) {
            let a629 = v0;
            let a630 = 1u64;
            if nat_lt(a629, a630) {
                let a631 = v0;
                let a632 = 0u64;
                if nat_eq(a631, a632) {
                    f37(3u64, f44(), f54(), f59())
                } else {
                    f2()
                }
            } else {
                let a633 = v0;
                let a634 = 1u64;
                if nat_eq(a633, a634) {
                    f37(4u64, f59(), 2u64, f59())
                } else {
                    f2()
                }
            }
        } else {
            let a635 = v0;
            let a636 = 3u64;
            if nat_lt(a635, a636) {
                let a637 = v0;
                let a638 = 2u64;
                if nat_eq(a637, a638) {
                    f31(0u64, f59(), 0u64, 557u64)
                } else {
                    f2()
                }
            } else {
                let a639 = v0;
                let a640 = 3u64;
                if nat_eq(a639, a640) {
                    f37(1u64, f44(), f54(), f44())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a641 = v0;
        let a642 = 6u64;
        if nat_lt(a641, a642) {
            let a643 = v0;
            let a644 = 5u64;
            if nat_lt(a643, a644) {
                let a645 = v0;
                let a646 = 4u64;
                if nat_eq(a645, a646) {
                    f38(558u64)
                } else {
                    f2()
                }
            } else {
                let a647 = v0;
                let a648 = 5u64;
                if nat_eq(a647, a648) {
                    f37(0u64, f44(), f54(), f44())
                } else {
                    f2()
                }
            }
        } else {
            let a649 = v0;
            let a650 = 7u64;
            if nat_lt(a649, a650) {
                let a651 = v0;
                let a652 = 6u64;
                if nat_eq(a651, a652) {
                    f37(2u64, f40(), 8u64, f59())
                } else {
                    f2()
                }
            } else {
                let a653 = v0;
                let a654 = 7u64;
                if nat_eq(a653, a654) {
                    f37(0u64, f59(), f58(), f59())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a655 = v0;
    let a656 = 4u64;
    if nat_lt(a655, a656) {
        let a657 = v0;
        let a658 = 2u64;
        if nat_lt(a657, a658) {
            let a659 = v0;
            let a660 = 1u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 0u64;
                if nat_eq(a661, a662) {
                    Ok(f34(6u64, f52(f59())?, 0u64, f52(f59())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a663 = v0;
                let a664 = 1u64;
                if nat_eq(a663, a664) {
                    Ok(f26(f44(), f60(f40())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a665 = v0;
            let a666 = 3u64;
            if nat_lt(a665, a666) {
                let a667 = v0;
                let a668 = 2u64;
                if nat_eq(a667, a668) {
                    Ok(f37(0u64, f40(), 1u64, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a669 = v0;
                let a670 = 3u64;
                if nat_eq(a669, a670) {
                    Ok(f31(1u64, f40(), 2u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a671 = v0;
        let a672 = 6u64;
        if nat_lt(a671, a672) {
            let a673 = v0;
            let a674 = 5u64;
            if nat_lt(a673, a674) {
                let a675 = v0;
                let a676 = 4u64;
                if nat_eq(a675, a676) {
                    Ok(f26(0u64, f39()))
                } else {
                    Ok(f2())
                }
            } else {
                let a677 = v0;
                let a678 = 5u64;
                if nat_eq(a677, a678) {
                    Ok(f31(2u64, f42(), f52(f39())?, 567u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a679 = v0;
            let a680 = 7u64;
            if nat_lt(a679, a680) {
                let a681 = v0;
                let a682 = 6u64;
                if nat_eq(a681, a682) {
                    Ok(f38(568u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a683 = v0;
                let a684 = 7u64;
                if nat_eq(a683, a684) {
                    Ok(f26(f52(f39())?, f42()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a685 = v0;
    let a686 = 3u64;
    if nat_lt(a685, a686) {
        let a687 = v0;
        let a688 = 1u64;
        if nat_lt(a687, a688) {
            let a689 = v0;
            let a690 = 0u64;
            if nat_eq(a689, a690) {
                Ok(f37(0u64, f39(), 1u64, f39()))
            } else {
                Ok(f2())
            }
        } else {
            let a691 = v0;
            let a692 = 2u64;
            if nat_lt(a691, a692) {
                let a693 = v0;
                let a694 = 1u64;
                if nat_eq(a693, a694) {
                    Ok(f31(1u64, f39(), 16u64, 565u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a695 = v0;
                let a696 = 2u64;
                if nat_eq(a695, a696) {
                    Ok(f26(0u64, f61(f28())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a697 = v0;
        let a698 = 4u64;
        if nat_lt(a697, a698) {
            let a699 = v0;
            let a700 = 3u64;
            if nat_eq(a699, a700) {
                Ok(f31(0u64, f42(), 0u64, 3099u64))
            } else {
                Ok(f2())
            }
        } else {
            let a701 = v0;
            let a702 = 5u64;
            if nat_lt(a701, a702) {
                let a703 = v0;
                let a704 = 4u64;
                if nat_eq(a703, a704) {
                    Ok(f31(2u64, f42(), f62(), 3099u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a705 = v0;
                let a706 = 5u64;
                if nat_eq(a705, a706) {
                    Ok(f38(1024u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f27() -> u64 {
    4294967348u64
}

pub fn f28() -> u64 {
    4294967301u64
}

pub fn f29() -> u64 {
    4294967302u64
}

pub fn f30() -> u64 {
    4294967347u64
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f32() -> u64 {
    4294967298u64
}

pub fn f33() -> u64 {
    4294967343u64
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f35(v0: u64) -> R<u64> {
    let a710 = 12884901888u64;
    let a711 = {
        let a707 = v0;
        let a708 = 4294967296u64;
        let a709 = 0u64;
        nat_rem(a707, a708, a709)
    };
    nat_add(a710, a711)
}

pub fn f36() -> u64 {
    4294967337u64
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f38(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f39() -> u64 {
    4294967303u64
}

pub fn f40() -> u64 {
    4294967304u64
}

pub fn f41() -> u64 {
    4294967354u64
}

pub fn f42() -> u64 {
    4294967315u64
}

pub fn f43() -> u64 {
    4294967313u64
}

pub fn f44() -> u64 {
    4294967314u64
}

pub fn f45() -> u64 {
    4294967305u64
}

pub fn f46() -> u64 {
    4294967307u64
}

pub fn f47() -> u64 {
    4294967308u64
}

pub fn f48() -> u64 {
    4294967309u64
}

pub fn f49(v0: u64) -> R<u64> {
    let a715 = 17179869184u64;
    let a716 = {
        let a712 = v0;
        let a713 = 4294967296u64;
        let a714 = 0u64;
        nat_rem(a712, a713, a714)
    };
    nat_add(a715, a716)
}

pub fn f50(v0: u64) -> R<u64> {
    let a720 = 8589959168u64;
    let a721 = {
        let a717 = v0;
        let a718 = 4294967296u64;
        let a719 = 0u64;
        nat_rem(a717, a718, a719)
    };
    nat_add(a720, a721)
}

pub fn f51(v0: u64) -> R<u64> {
    let a725 = 8589967360u64;
    let a726 = {
        let a722 = v0;
        let a723 = 4294967296u64;
        let a724 = 0u64;
        nat_rem(a722, a723, a724)
    };
    nat_add(a725, a726)
}

pub fn f52(v0: u64) -> R<u64> {
    let a730 = 8589950976u64;
    let a731 = {
        let a727 = v0;
        let a728 = 4294967296u64;
        let a729 = 0u64;
        nat_rem(a727, a728, a729)
    };
    nat_add(a730, a731)
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f54() -> u64 {
    4294967342u64
}

pub fn f55() -> u64 {
    4294967297u64
}

pub fn f56() -> u64 {
    4294967310u64
}

pub fn f57() -> u64 {
    4294967311u64
}

pub fn f58() -> u64 {
    4294967312u64
}

pub fn f59() -> u64 {
    4294967338u64
}

pub fn f60(v0: u64) -> R<u64> {
    let a735 = 8589975552u64;
    let a736 = {
        let a732 = v0;
        let a733 = 4294967296u64;
        let a734 = 0u64;
        nat_rem(a732, a733, a734)
    };
    nat_add(a735, a736)
}

pub fn f61(v0: u64) -> R<u64> {
    let a740 = 8589984768u64;
    let a741 = {
        let a737 = v0;
        let a738 = 4294967296u64;
        let a739 = 0u64;
        nat_rem(a737, a738, a739)
    };
    nat_add(a740, a741)
}

pub fn f62() -> u64 {
    4294967300u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
