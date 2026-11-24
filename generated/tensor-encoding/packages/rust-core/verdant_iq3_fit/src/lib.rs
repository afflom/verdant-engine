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
    let a106 = 3u64;
    if nat_lt(a105, a106) {
        let a107 = v1;
        let a108 = 1u64;
        if nat_lt(a107, a108) {
            let a109 = v1;
            let a110 = 0u64;
            if nat_eq(a109, a110) {
                Ok(f18({
                    let a111 = v0;
                    let a112 = 8u64;
                    let a113 = 0u64;
                    nat_rem(a111, a112, a113)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a114 = v1;
            let a115 = 2u64;
            if nat_lt(a114, a115) {
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
            } else {
                let a121 = v1;
                let a122 = 2u64;
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
            }
        }
    } else {
        let a126 = v1;
        let a127 = 4u64;
        if nat_lt(a126, a127) {
            let a128 = v1;
            let a129 = 3u64;
            if nat_eq(a128, a129) {
                f21({
                    let a130 = v0;
                    let a131 = 8u64;
                    let a132 = 0u64;
                    nat_rem(a130, a131, a132)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a133 = v1;
            let a134 = 5u64;
            if nat_lt(a133, a134) {
                let a135 = v1;
                let a136 = 4u64;
                if nat_eq(a135, a136) {
                    f22({
                        let a137 = v0;
                        let a138 = 8u64;
                        let a139 = 0u64;
                        nat_rem(a137, a138, a139)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a140 = v1;
                let a141 = 5u64;
                if nat_eq(a140, a141) {
                    Ok(f23({
                        let a142 = v0;
                        let a143 = 8u64;
                        let a144 = 0u64;
                        nat_rem(a142, a143, a144)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a145 = v0;
    let a146 = 4u64;
    if nat_lt(a145, a146) {
        let a147 = v0;
        let a148 = 2u64;
        if nat_lt(a147, a148) {
            let a149 = v0;
            let a150 = 1u64;
            if nat_lt(a149, a150) {
                let a151 = v0;
                let a152 = 0u64;
                if nat_eq(a151, a152) {
                    Ok(f24(0u64, f25()))
                } else {
                    Ok(f2())
                }
            } else {
                let a153 = v0;
                let a154 = 1u64;
                if nat_eq(a153, a154) {
                    Ok(f24(0u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a155 = v0;
            let a156 = 3u64;
            if nat_lt(a155, a156) {
                let a157 = v0;
                let a158 = 2u64;
                if nat_eq(a157, a158) {
                    Ok(f24(0u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a159 = v0;
                let a160 = 3u64;
                if nat_eq(a159, a160) {
                    Ok(f24(0u64, f28()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a161 = v0;
        let a162 = 6u64;
        if nat_lt(a161, a162) {
            let a163 = v0;
            let a164 = 5u64;
            if nat_lt(a163, a164) {
                let a165 = v0;
                let a166 = 4u64;
                if nat_eq(a165, a166) {
                    Ok(f29(0u64, f30(), 0u64, 512u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a167 = v0;
                let a168 = 5u64;
                if nat_eq(a167, a168) {
                    Ok(f24(0u64, f31()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a169 = v0;
            let a170 = 7u64;
            if nat_lt(a169, a170) {
                let a171 = v0;
                let a172 = 6u64;
                if nat_eq(a171, a172) {
                    Ok(f32(2u64, f33(f31())?, f33(f31())?, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a173 = v0;
                let a174 = 7u64;
                if nat_eq(a173, a174) {
                    Ok(f32(0u64, f25(), f34(), f25()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a175 = v0;
    let a176 = 2u64;
    if nat_lt(a175, a176) {
        let a177 = v0;
        let a178 = 1u64;
        if nat_lt(a177, a178) {
            let a179 = v0;
            let a180 = 0u64;
            if nat_eq(a179, a180) {
                f35(0u64, f31(), 1u64, f31())
            } else {
                f2()
            }
        } else {
            let a181 = v0;
            let a182 = 1u64;
            if nat_eq(a181, a182) {
                f29(1u64, f31(), 256u64, 6u64)
            } else {
                f2()
            }
        }
    } else {
        let a183 = v0;
        let a184 = 3u64;
        if nat_lt(a183, a184) {
            let a185 = v0;
            let a186 = 2u64;
            if nat_eq(a185, a186) {
                f32(2u64, 1073741824u64, f25(), f25())
            } else {
                f2()
            }
        } else {
            let a187 = v0;
            let a188 = 4u64;
            if nat_lt(a187, a188) {
                let a189 = v0;
                let a190 = 3u64;
                if nat_eq(a189, a190) {
                    f32(3u64, f25(), 1132462080u64, f25())
                } else {
                    f2()
                }
            } else {
                let a191 = v0;
                let a192 = 4u64;
                if nat_eq(a191, a192) {
                    f36(512u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a193 = v0;
    let a194 = 4u64;
    if nat_lt(a193, a194) {
        let a195 = v0;
        let a196 = 2u64;
        if nat_lt(a195, a196) {
            let a197 = v0;
            let a198 = 1u64;
            if nat_lt(a197, a198) {
                let a199 = v0;
                let a200 = 0u64;
                if nat_eq(a199, a200) {
                    f24(0u64, f37())
                } else {
                    f2()
                }
            } else {
                let a201 = v0;
                let a202 = 1u64;
                if nat_eq(a201, a202) {
                    f24(0u64, f38())
                } else {
                    f2()
                }
            }
        } else {
            let a203 = v0;
            let a204 = 3u64;
            if nat_lt(a203, a204) {
                let a205 = v0;
                let a206 = 2u64;
                if nat_eq(a205, a206) {
                    f35(2u64, f26(), 32u64, f39())
                } else {
                    f2()
                }
            } else {
                let a207 = v0;
                let a208 = 3u64;
                if nat_eq(a207, a208) {
                    f24(0u64, f40())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a209 = v0;
        let a210 = 6u64;
        if nat_lt(a209, a210) {
            let a211 = v0;
            let a212 = 5u64;
            if nat_lt(a211, a212) {
                let a213 = v0;
                let a214 = 4u64;
                if nat_eq(a213, a214) {
                    f24(0u64, f41())
                } else {
                    f2()
                }
            } else {
                let a215 = v0;
                let a216 = 5u64;
                if nat_eq(a215, a216) {
                    f24(0u64, f42())
                } else {
                    f2()
                }
            }
        } else {
            let a217 = v0;
            let a218 = 7u64;
            if nat_lt(a217, a218) {
                let a219 = v0;
                let a220 = 6u64;
                if nat_eq(a219, a220) {
                    f24(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a221 = v0;
                let a222 = 7u64;
                if nat_eq(a221, a222) {
                    f35(0u64, f39(), f37(), f44())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a223 = v0;
    let a224 = 4u64;
    if nat_lt(a223, a224) {
        let a225 = v0;
        let a226 = 2u64;
        if nat_lt(a225, a226) {
            let a227 = v0;
            let a228 = 1u64;
            if nat_lt(a227, a228) {
                let a229 = v0;
                let a230 = 0u64;
                if nat_eq(a229, a230) {
                    Ok(f24(f33(f44())?, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a231 = v0;
                let a232 = 1u64;
                if nat_eq(a231, a232) {
                    Ok(f32(2u64, f45(), f45(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a233 = v0;
            let a234 = 3u64;
            if nat_lt(a233, a234) {
                let a235 = v0;
                let a236 = 2u64;
                if nat_eq(a235, a236) {
                    Ok(f29(0u64, f30(), 0u64, 526u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a237 = v0;
                let a238 = 3u64;
                if nat_eq(a237, a238) {
                    Ok(f32(0u64, f25(), f46(), f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a239 = v0;
        let a240 = 6u64;
        if nat_lt(a239, a240) {
            let a241 = v0;
            let a242 = 5u64;
            if nat_lt(a241, a242) {
                let a243 = v0;
                let a244 = 4u64;
                if nat_eq(a243, a244) {
                    Ok(f32(4u64, f46(), 0u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a245 = v0;
                let a246 = 5u64;
                if nat_eq(a245, a246) {
                    Ok(f32(2u64, f47(f44())?, f46(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a247 = v0;
            let a248 = 7u64;
            if nat_lt(a247, a248) {
                let a249 = v0;
                let a250 = 6u64;
                if nat_eq(a249, a250) {
                    Ok(f24(f46(), f48(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a251 = v0;
                let a252 = 7u64;
                if nat_eq(a251, a252) {
                    Ok(f32(4u64, f46(), 0u64, f49(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a253 = v0;
    let a254 = 4u64;
    if nat_lt(a253, a254) {
        let a255 = v0;
        let a256 = 2u64;
        if nat_lt(a255, a256) {
            let a257 = v0;
            let a258 = 1u64;
            if nat_lt(a257, a258) {
                let a259 = v0;
                let a260 = 0u64;
                if nat_eq(a259, a260) {
                    Ok(f24(f45(), f50(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a261 = v0;
                let a262 = 1u64;
                if nat_eq(a261, a262) {
                    Ok(f29(2u64, f45(), 0u64, 531u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a263 = v0;
            let a264 = 3u64;
            if nat_lt(a263, a264) {
                let a265 = v0;
                let a266 = 2u64;
                if nat_eq(a265, a266) {
                    Ok(f36(535u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a267 = v0;
                let a268 = 3u64;
                if nat_eq(a267, a268) {
                    Ok(f32(6u64, f45(), 0u64, f50(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a269 = v0;
        let a270 = 6u64;
        if nat_lt(a269, a270) {
            let a271 = v0;
            let a272 = 5u64;
            if nat_lt(a271, a272) {
                let a273 = v0;
                let a274 = 4u64;
                if nat_eq(a273, a274) {
                    Ok(f51(6u64, f43(), 0u64, 0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a275 = v0;
                let a276 = 5u64;
                if nat_eq(a275, a276) {
                    Ok(f35(0u64, f42(), f52(), f42()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a277 = v0;
            let a278 = 7u64;
            if nat_lt(a277, a278) {
                let a279 = v0;
                let a280 = 6u64;
                if nat_eq(a279, a280) {
                    Ok(f35(1u64, 1u64, f41(), f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a281 = v0;
                let a282 = 7u64;
                if nat_eq(a281, a282) {
                    Ok(f29(0u64, f53(), 0u64, 543u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a283 = v0;
    let a284 = 4u64;
    if nat_lt(a283, a284) {
        let a285 = v0;
        let a286 = 2u64;
        if nat_lt(a285, a286) {
            let a287 = v0;
            let a288 = 1u64;
            if nat_lt(a287, a288) {
                let a289 = v0;
                let a290 = 0u64;
                if nat_eq(a289, a290) {
                    f32(2u64, f46(), f45(), f54())
                } else {
                    f2()
                }
            } else {
                let a291 = v0;
                let a292 = 1u64;
                if nat_eq(a291, a292) {
                    f32(2u64, f54(), f45(), f54())
                } else {
                    f2()
                }
            }
        } else {
            let a293 = v0;
            let a294 = 3u64;
            if nat_lt(a293, a294) {
                let a295 = v0;
                let a296 = 2u64;
                if nat_eq(a295, a296) {
                    f29(0u64, f43(), 0u64, 541u64)
                } else {
                    f2()
                }
            } else {
                let a297 = v0;
                let a298 = 3u64;
                if nat_eq(a297, a298) {
                    f29(2u64, f54(), f55(), 541u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a299 = v0;
        let a300 = 6u64;
        if nat_lt(a299, a300) {
            let a301 = v0;
            let a302 = 5u64;
            if nat_lt(a301, a302) {
                let a303 = v0;
                let a304 = 4u64;
                if nat_eq(a303, a304) {
                    f36(543u64)
                } else {
                    f2()
                }
            } else {
                let a305 = v0;
                let a306 = 5u64;
                if nat_eq(a305, a306) {
                    f24(f54(), f55())
                } else {
                    f2()
                }
            }
        } else {
            let a307 = v0;
            let a308 = 7u64;
            if nat_lt(a307, a308) {
                let a309 = v0;
                let a310 = 6u64;
                if nat_eq(a309, a310) {
                    f24(f43(), f56())
                } else {
                    f2()
                }
            } else {
                let a311 = v0;
                let a312 = 7u64;
                if nat_eq(a311, a312) {
                    f35(0u64, f43(), 1u64, f43())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a313 = v0;
    let a314 = 4u64;
    if nat_lt(a313, a314) {
        let a315 = v0;
        let a316 = 2u64;
        if nat_lt(a315, a316) {
            let a317 = v0;
            let a318 = 1u64;
            if nat_lt(a317, a318) {
                let a319 = v0;
                let a320 = 0u64;
                if nat_eq(a319, a320) {
                    f35(0u64, f37(), 1u64, f37())
                } else {
                    f2()
                }
            } else {
                let a321 = v0;
                let a322 = 1u64;
                if nat_eq(a321, a322) {
                    f29(1u64, f43(), 8u64, 519u64)
                } else {
                    f2()
                }
            }
        } else {
            let a323 = v0;
            let a324 = 3u64;
            if nat_lt(a323, a324) {
                let a325 = v0;
                let a326 = 2u64;
                if nat_eq(a325, a326) {
                    f29(0u64, f53(), 0u64, 558u64)
                } else {
                    f2()
                }
            } else {
                let a327 = v0;
                let a328 = 3u64;
                if nat_eq(a327, a328) {
                    f29(0u64, f41(), 0u64, 558u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a329 = v0;
        let a330 = 6u64;
        if nat_lt(a329, a330) {
            let a331 = v0;
            let a332 = 5u64;
            if nat_lt(a331, a332) {
                let a333 = v0;
                let a334 = 4u64;
                if nat_eq(a333, a334) {
                    f51(6u64, f56(), 0u64, 0u64, f52())
                } else {
                    f2()
                }
            } else {
                let a335 = v0;
                let a336 = 5u64;
                if nat_eq(a335, a336) {
                    f35(3u64, f42(), f52(), f57())
                } else {
                    f2()
                }
            }
        } else {
            let a337 = v0;
            let a338 = 7u64;
            if nat_lt(a337, a338) {
                let a339 = v0;
                let a340 = 6u64;
                if nat_eq(a339, a340) {
                    f35(4u64, f57(), 2u64, f57())
                } else {
                    f2()
                }
            } else {
                let a341 = v0;
                let a342 = 7u64;
                if nat_eq(a341, a342) {
                    f29(0u64, f57(), 0u64, 554u64)
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a343 = v0;
    let a344 = 4u64;
    if nat_lt(a343, a344) {
        let a345 = v0;
        let a346 = 2u64;
        if nat_lt(a345, a346) {
            let a347 = v0;
            let a348 = 1u64;
            if nat_lt(a347, a348) {
                let a349 = v0;
                let a350 = 0u64;
                if nat_eq(a349, a350) {
                    Ok(f35(1u64, f42(), f52(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a351 = v0;
                let a352 = 1u64;
                if nat_eq(a351, a352) {
                    Ok(f36(555u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a353 = v0;
            let a354 = 3u64;
            if nat_lt(a353, a354) {
                let a355 = v0;
                let a356 = 2u64;
                if nat_eq(a355, a356) {
                    Ok(f35(0u64, f42(), f52(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a357 = v0;
                let a358 = 3u64;
                if nat_eq(a357, a358) {
                    Ok(f35(2u64, f38(), 8u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a359 = v0;
        let a360 = 6u64;
        if nat_lt(a359, a360) {
            let a361 = v0;
            let a362 = 5u64;
            if nat_lt(a361, a362) {
                let a363 = v0;
                let a364 = 4u64;
                if nat_eq(a363, a364) {
                    Ok(f35(0u64, f57(), f56(), f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a365 = v0;
                let a366 = 5u64;
                if nat_eq(a365, a366) {
                    Ok(f32(6u64, f50(f57())?, 0u64, f50(f57())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a367 = v0;
            let a368 = 7u64;
            if nat_lt(a367, a368) {
                let a369 = v0;
                let a370 = 6u64;
                if nat_eq(a369, a370) {
                    Ok(f24(f42(), f58(f38())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a371 = v0;
                let a372 = 7u64;
                if nat_eq(a371, a372) {
                    Ok(f35(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a373 = v0;
    let a374 = 4u64;
    if nat_lt(a373, a374) {
        let a375 = v0;
        let a376 = 2u64;
        if nat_lt(a375, a376) {
            let a377 = v0;
            let a378 = 1u64;
            if nat_lt(a377, a378) {
                let a379 = v0;
                let a380 = 0u64;
                if nat_eq(a379, a380) {
                    Ok(f29(1u64, f38(), 4u64, 516u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a381 = v0;
                let a382 = 1u64;
                if nat_eq(a381, a382) {
                    Ok(f24(0u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a383 = v0;
            let a384 = 3u64;
            if nat_lt(a383, a384) {
                let a385 = v0;
                let a386 = 2u64;
                if nat_eq(a385, a386) {
                    Ok(f29(2u64, f40(), f50(f37())?, 564u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a387 = v0;
                let a388 = 3u64;
                if nat_eq(a387, a388) {
                    Ok(f36(565u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a389 = v0;
        let a390 = 6u64;
        if nat_lt(a389, a390) {
            let a391 = v0;
            let a392 = 5u64;
            if nat_lt(a391, a392) {
                let a393 = v0;
                let a394 = 4u64;
                if nat_eq(a393, a394) {
                    Ok(f24(f50(f37())?, f40()))
                } else {
                    Ok(f2())
                }
            } else {
                let a395 = v0;
                let a396 = 5u64;
                if nat_eq(a395, a396) {
                    Ok(f35(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a397 = v0;
            let a398 = 7u64;
            if nat_lt(a397, a398) {
                let a399 = v0;
                let a400 = 6u64;
                if nat_eq(a399, a400) {
                    Ok(f29(1u64, f37(), 32u64, 562u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a401 = v0;
                let a402 = 7u64;
                if nat_eq(a401, a402) {
                    Ok(f24(0u64, f59(f26())?))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a403 = v0;
    let a404 = 1u64;
    if nat_lt(a403, a404) {
        let a405 = v0;
        let a406 = 0u64;
        if nat_eq(a405, a406) {
            f29(0u64, f40(), 0u64, 3109u64)
        } else {
            f2()
        }
    } else {
        let a407 = v0;
        let a408 = 2u64;
        if nat_lt(a407, a408) {
            let a409 = v0;
            let a410 = 1u64;
            if nat_eq(a409, a410) {
                f29(2u64, f40(), f60(), 3109u64)
            } else {
                f2()
            }
        } else {
            let a411 = v0;
            let a412 = 2u64;
            if nat_eq(a411, a412) {
                f36(1024u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a413 = v0;
    let a414 = 4u64;
    if nat_lt(a413, a414) {
        let a415 = v0;
        let a416 = 2u64;
        if nat_lt(a415, a416) {
            let a417 = v0;
            let a418 = 1u64;
            if nat_lt(a417, a418) {
                let a419 = v0;
                let a420 = 0u64;
                if nat_eq(a419, a420) {
                    Ok(f24(0u64, f25()))
                } else {
                    Ok(f2())
                }
            } else {
                let a421 = v0;
                let a422 = 1u64;
                if nat_eq(a421, a422) {
                    Ok(f24(0u64, f26()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a423 = v0;
            let a424 = 3u64;
            if nat_lt(a423, a424) {
                let a425 = v0;
                let a426 = 2u64;
                if nat_eq(a425, a426) {
                    Ok(f24(0u64, f27()))
                } else {
                    Ok(f2())
                }
            } else {
                let a427 = v0;
                let a428 = 3u64;
                if nat_eq(a427, a428) {
                    Ok(f24(0u64, f28()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a429 = v0;
        let a430 = 6u64;
        if nat_lt(a429, a430) {
            let a431 = v0;
            let a432 = 5u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 4u64;
                if nat_eq(a433, a434) {
                    Ok(f24(0u64, f31()))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 5u64;
                if nat_eq(a435, a436) {
                    Ok(f32(2u64, f33(f31())?, f33(f31())?, f34()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a437 = v0;
            let a438 = 7u64;
            if nat_lt(a437, a438) {
                let a439 = v0;
                let a440 = 6u64;
                if nat_eq(a439, a440) {
                    Ok(f32(0u64, f25(), f34(), f25()))
                } else {
                    Ok(f2())
                }
            } else {
                let a441 = v0;
                let a442 = 7u64;
                if nat_eq(a441, a442) {
                    Ok(f35(0u64, f31(), 1u64, f31()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a443 = v0;
    let a444 = 2u64;
    if nat_lt(a443, a444) {
        let a445 = v0;
        let a446 = 1u64;
        if nat_lt(a445, a446) {
            let a447 = v0;
            let a448 = 0u64;
            if nat_eq(a447, a448) {
                f29(1u64, f31(), 256u64, 5u64)
            } else {
                f2()
            }
        } else {
            let a449 = v0;
            let a450 = 1u64;
            if nat_eq(a449, a450) {
                f32(2u64, 1073741824u64, f25(), f25())
            } else {
                f2()
            }
        }
    } else {
        let a451 = v0;
        let a452 = 3u64;
        if nat_lt(a451, a452) {
            let a453 = v0;
            let a454 = 2u64;
            if nat_eq(a453, a454) {
                f32(3u64, f25(), 1132462080u64, f25())
            } else {
                f2()
            }
        } else {
            let a455 = v0;
            let a456 = 3u64;
            if nat_eq(a455, a456) {
                f36(512u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
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
                    f24(0u64, f37())
                } else {
                    f2()
                }
            } else {
                let a465 = v0;
                let a466 = 1u64;
                if nat_eq(a465, a466) {
                    f24(0u64, f38())
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
                    f35(2u64, f26(), 16u64, f39())
                } else {
                    f2()
                }
            } else {
                let a471 = v0;
                let a472 = 3u64;
                if nat_eq(a471, a472) {
                    f24(0u64, f40())
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
                    f24(0u64, f41())
                } else {
                    f2()
                }
            } else {
                let a479 = v0;
                let a480 = 5u64;
                if nat_eq(a479, a480) {
                    f24(0u64, f42())
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
                    f24(0u64, f43())
                } else {
                    f2()
                }
            } else {
                let a485 = v0;
                let a486 = 7u64;
                if nat_eq(a485, a486) {
                    f35(0u64, f39(), f37(), f44())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f24(f33(f44())?, f45()))
                } else {
                    Ok(f2())
                }
            } else {
                let a495 = v0;
                let a496 = 1u64;
                if nat_eq(a495, a496) {
                    Ok(f32(2u64, f45(), f45(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a497 = v0;
            let a498 = 3u64;
            if nat_lt(a497, a498) {
                let a499 = v0;
                let a500 = 2u64;
                if nat_eq(a499, a500) {
                    Ok(f29(0u64, f30(), 0u64, 527u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a501 = v0;
                let a502 = 3u64;
                if nat_eq(a501, a502) {
                    Ok(f32(0u64, f25(), f46(), f46()))
                } else {
                    Ok(f2())
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
                    Ok(f32(4u64, f46(), 0u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a509 = v0;
                let a510 = 5u64;
                if nat_eq(a509, a510) {
                    Ok(f32(2u64, f47(f44())?, f46(), f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a511 = v0;
            let a512 = 7u64;
            if nat_lt(a511, a512) {
                let a513 = v0;
                let a514 = 6u64;
                if nat_eq(a513, a514) {
                    Ok(f36(529u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a515 = v0;
                let a516 = 7u64;
                if nat_eq(a515, a516) {
                    Ok(f32(2u64, 1048576000u64, f25(), f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f32(0u64, f57(), f46(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a525 = v0;
                let a526 = 1u64;
                if nat_eq(a525, a526) {
                    Ok(f24(f46(), f48(f37())?))
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
                    Ok(f32(4u64, f46(), 0u64, f49(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a531 = v0;
                let a532 = 3u64;
                if nat_eq(a531, a532) {
                    Ok(f24(f45(), f50(f37())?))
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
                    Ok(f29(2u64, f45(), 0u64, 534u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a539 = v0;
                let a540 = 5u64;
                if nat_eq(a539, a540) {
                    Ok(f36(538u64))
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
                    Ok(f32(6u64, f45(), 0u64, f50(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a545 = v0;
                let a546 = 7u64;
                if nat_eq(a545, a546) {
                    Ok(f51(6u64, f43(), 0u64, 0u64, f52()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f35(0u64, f42(), f52(), f42()))
                } else {
                    Ok(f2())
                }
            } else {
                let a555 = v0;
                let a556 = 1u64;
                if nat_eq(a555, a556) {
                    Ok(f35(1u64, 1u64, f41(), f41()))
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
                    Ok(f35(0u64, f43(), 1u64, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a561 = v0;
                let a562 = 3u64;
                if nat_eq(a561, a562) {
                    Ok(f35(0u64, f37(), 1u64, f37()))
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
                    Ok(f29(1u64, f43(), 8u64, 519u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a569 = v0;
                let a570 = 5u64;
                if nat_eq(a569, a570) {
                    Ok(f24(f42(), f58(f38())?))
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
                    Ok(f35(0u64, f38(), 1u64, f38()))
                } else {
                    Ok(f2())
                }
            } else {
                let a575 = v0;
                let a576 = 7u64;
                if nat_eq(a575, a576) {
                    Ok(f29(1u64, f38(), 2u64, 516u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a577 = v0;
    let a578 = 4u64;
    if nat_lt(a577, a578) {
        let a579 = v0;
        let a580 = 2u64;
        if nat_lt(a579, a580) {
            let a581 = v0;
            let a582 = 1u64;
            if nat_lt(a581, a582) {
                let a583 = v0;
                let a584 = 0u64;
                if nat_eq(a583, a584) {
                    Ok(f24(0u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a585 = v0;
                let a586 = 1u64;
                if nat_eq(a585, a586) {
                    Ok(f29(2u64, f40(), f50(f37())?, 547u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a587 = v0;
            let a588 = 3u64;
            if nat_lt(a587, a588) {
                let a589 = v0;
                let a590 = 2u64;
                if nat_eq(a589, a590) {
                    Ok(f36(548u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a591 = v0;
                let a592 = 3u64;
                if nat_eq(a591, a592) {
                    Ok(f24(f50(f37())?, f40()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a593 = v0;
        let a594 = 6u64;
        if nat_lt(a593, a594) {
            let a595 = v0;
            let a596 = 5u64;
            if nat_lt(a595, a596) {
                let a597 = v0;
                let a598 = 4u64;
                if nat_eq(a597, a598) {
                    Ok(f35(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a599 = v0;
                let a600 = 5u64;
                if nat_eq(a599, a600) {
                    Ok(f29(1u64, f37(), 16u64, 545u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a601 = v0;
            let a602 = 7u64;
            if nat_lt(a601, a602) {
                let a603 = v0;
                let a604 = 6u64;
                if nat_eq(a603, a604) {
                    Ok(f24(0u64, f59(f26())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a605 = v0;
                let a606 = 7u64;
                if nat_eq(a605, a606) {
                    Ok(f29(0u64, f40(), 0u64, 3094u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a607 = v0;
    let a608 = 1u64;
    if nat_lt(a607, a608) {
        let a609 = v0;
        let a610 = 0u64;
        if nat_eq(a609, a610) {
            f29(2u64, f40(), f60(), 3094u64)
        } else {
            f2()
        }
    } else {
        let a611 = v0;
        let a612 = 1u64;
        if nat_eq(a611, a612) {
            f36(1024u64)
        } else {
            f2()
        }
    }
}

pub fn f24(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f25() -> u64 {
    4294967348u64
}

pub fn f26() -> u64 {
    4294967301u64
}

pub fn f27() -> u64 {
    4294967302u64
}

pub fn f28() -> u64 {
    4294967347u64
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f30() -> u64 {
    4294967298u64
}

pub fn f31() -> u64 {
    4294967343u64
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f33(v0: u64) -> R<u64> {
    let a616 = 12884901888u64;
    let a617 = {
        let a613 = v0;
        let a614 = 4294967296u64;
        let a615 = 0u64;
        nat_rem(a613, a614, a615)
    };
    nat_add(a616, a617)
}

pub fn f34() -> u64 {
    4294967337u64
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f36(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f37() -> u64 {
    4294967303u64
}

pub fn f38() -> u64 {
    4294967304u64
}

pub fn f39() -> u64 {
    4294967354u64
}

pub fn f40() -> u64 {
    4294967315u64
}

pub fn f41() -> u64 {
    4294967313u64
}

pub fn f42() -> u64 {
    4294967314u64
}

pub fn f43() -> u64 {
    4294967305u64
}

pub fn f44() -> u64 {
    4294967307u64
}

pub fn f45() -> u64 {
    4294967308u64
}

pub fn f46() -> u64 {
    4294967309u64
}

pub fn f47(v0: u64) -> R<u64> {
    let a621 = 17179869184u64;
    let a622 = {
        let a618 = v0;
        let a619 = 4294967296u64;
        let a620 = 0u64;
        nat_rem(a618, a619, a620)
    };
    nat_add(a621, a622)
}

pub fn f48(v0: u64) -> R<u64> {
    let a626 = 8589959168u64;
    let a627 = {
        let a623 = v0;
        let a624 = 4294967296u64;
        let a625 = 0u64;
        nat_rem(a623, a624, a625)
    };
    nat_add(a626, a627)
}

pub fn f49(v0: u64) -> R<u64> {
    let a631 = 8589967360u64;
    let a632 = {
        let a628 = v0;
        let a629 = 4294967296u64;
        let a630 = 0u64;
        nat_rem(a628, a629, a630)
    };
    nat_add(a631, a632)
}

pub fn f50(v0: u64) -> R<u64> {
    let a636 = 8589950976u64;
    let a637 = {
        let a633 = v0;
        let a634 = 4294967296u64;
        let a635 = 0u64;
        nat_rem(a633, a634, a635)
    };
    nat_add(a636, a637)
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f52() -> u64 {
    4294967342u64
}

pub fn f53() -> u64 {
    4294967297u64
}

pub fn f54() -> u64 {
    4294967310u64
}

pub fn f55() -> u64 {
    4294967311u64
}

pub fn f56() -> u64 {
    4294967312u64
}

pub fn f57() -> u64 {
    4294967338u64
}

pub fn f58(v0: u64) -> R<u64> {
    let a641 = 8589975552u64;
    let a642 = {
        let a638 = v0;
        let a639 = 4294967296u64;
        let a640 = 0u64;
        nat_rem(a638, a639, a640)
    };
    nat_add(a641, a642)
}

pub fn f59(v0: u64) -> R<u64> {
    let a646 = 8589984768u64;
    let a647 = {
        let a643 = v0;
        let a644 = 4294967296u64;
        let a645 = 0u64;
        nat_rem(a643, a644, a645)
    };
    nat_add(a646, a647)
}

pub fn f60() -> u64 {
    4294967300u64
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
