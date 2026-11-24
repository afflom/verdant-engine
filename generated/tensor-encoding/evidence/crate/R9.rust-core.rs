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
    let a19 = 3u64;
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
            let a28 = 2u64;
            if nat_lt(a27, a28) {
                let a29 = v1;
                let a30 = 1u64;
                if nat_eq(a29, a30) {
                    Ok(f7({
                        let a31 = v0;
                        let a32 = 8u64;
                        let a33 = 0u64;
                        nat_rem(a31, a32, a33)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
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
            }
        }
    } else {
        let a39 = v1;
        let a40 = 4u64;
        if nat_lt(a39, a40) {
            let a41 = v1;
            let a42 = 3u64;
            if nat_eq(a41, a42) {
                Ok(f9({
                    let a43 = v0;
                    let a44 = 8u64;
                    let a45 = 0u64;
                    nat_rem(a43, a44, a45)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a46 = v1;
            let a47 = 5u64;
            if nat_lt(a46, a47) {
                let a48 = v1;
                let a49 = 4u64;
                if nat_eq(a48, a49) {
                    f10({
                        let a50 = v0;
                        let a51 = 8u64;
                        let a52 = 0u64;
                        nat_rem(a50, a51, a52)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a53 = v1;
                let a54 = 5u64;
                if nat_eq(a53, a54) {
                    Ok(f11({
                        let a55 = v0;
                        let a56 = 8u64;
                        let a57 = 0u64;
                        nat_rem(a55, a56, a57)
                    }))
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
        let a58 = v0;
        let a59 = 8u64;
        let a60 = 0u64;
        nat_quot(a58, a59, a60)
    };
    let a61 = v1;
    let a62 = 3u64;
    if nat_lt(a61, a62) {
        let a63 = v1;
        let a64 = 1u64;
        if nat_lt(a63, a64) {
            let a65 = v1;
            let a66 = 0u64;
            if nat_eq(a65, a66) {
                f12({
                    let a67 = v0;
                    let a68 = 8u64;
                    let a69 = 0u64;
                    nat_rem(a67, a68, a69)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a70 = v1;
            let a71 = 2u64;
            if nat_lt(a70, a71) {
                let a72 = v1;
                let a73 = 1u64;
                if nat_eq(a72, a73) {
                    f13({
                        let a74 = v0;
                        let a75 = 8u64;
                        let a76 = 0u64;
                        nat_rem(a74, a75, a76)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a77 = v1;
                let a78 = 2u64;
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
            }
        }
    } else {
        let a82 = v1;
        let a83 = 5u64;
        if nat_lt(a82, a83) {
            let a84 = v1;
            let a85 = 4u64;
            if nat_lt(a84, a85) {
                let a86 = v1;
                let a87 = 3u64;
                if nat_eq(a86, a87) {
                    f15({
                        let a88 = v0;
                        let a89 = 8u64;
                        let a90 = 0u64;
                        nat_rem(a88, a89, a90)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a91 = v1;
                let a92 = 4u64;
                if nat_eq(a91, a92) {
                    f16({
                        let a93 = v0;
                        let a94 = 8u64;
                        let a95 = 0u64;
                        nat_rem(a93, a94, a95)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a96 = v1;
            let a97 = 6u64;
            if nat_lt(a96, a97) {
                let a98 = v1;
                let a99 = 5u64;
                if nat_eq(a98, a99) {
                    Ok(f17({
                        let a100 = v0;
                        let a101 = 8u64;
                        let a102 = 0u64;
                        nat_rem(a100, a101, a102)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a103 = v1;
                let a104 = 6u64;
                if nat_eq(a103, a104) {
                    Ok(f18({
                        let a105 = v0;
                        let a106 = 8u64;
                        let a107 = 0u64;
                        nat_rem(a105, a106, a107)
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
        let a108 = v0;
        let a109 = 8u64;
        let a110 = 0u64;
        nat_quot(a108, a109, a110)
    };
    let a111 = v1;
    let a112 = 3u64;
    if nat_lt(a111, a112) {
        let a113 = v1;
        let a114 = 1u64;
        if nat_lt(a113, a114) {
            let a115 = v1;
            let a116 = 0u64;
            if nat_eq(a115, a116) {
                f19({
                    let a117 = v0;
                    let a118 = 8u64;
                    let a119 = 0u64;
                    nat_rem(a117, a118, a119)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a120 = v1;
            let a121 = 2u64;
            if nat_lt(a120, a121) {
                let a122 = v1;
                let a123 = 1u64;
                if nat_eq(a122, a123) {
                    Ok(f20({
                        let a124 = v0;
                        let a125 = 8u64;
                        let a126 = 0u64;
                        nat_rem(a124, a125, a126)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a127 = v1;
                let a128 = 2u64;
                if nat_eq(a127, a128) {
                    f21({
                        let a129 = v0;
                        let a130 = 8u64;
                        let a131 = 0u64;
                        nat_rem(a129, a130, a131)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a132 = v1;
        let a133 = 4u64;
        if nat_lt(a132, a133) {
            let a134 = v1;
            let a135 = 3u64;
            if nat_eq(a134, a135) {
                Ok(f9({
                    let a136 = v0;
                    let a137 = 8u64;
                    let a138 = 0u64;
                    nat_rem(a136, a137, a138)
                }))
            } else {
                Ok(f2())
            }
        } else {
            let a139 = v1;
            let a140 = 5u64;
            if nat_lt(a139, a140) {
                let a141 = v1;
                let a142 = 4u64;
                if nat_eq(a141, a142) {
                    f22({
                        let a143 = v0;
                        let a144 = 8u64;
                        let a145 = 0u64;
                        nat_rem(a143, a144, a145)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a146 = v1;
                let a147 = 5u64;
                if nat_eq(a146, a147) {
                    Ok(f11({
                        let a148 = v0;
                        let a149 = 8u64;
                        let a150 = 0u64;
                        nat_rem(a148, a149, a150)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a151 = v0;
        let a152 = 8u64;
        let a153 = 0u64;
        nat_quot(a151, a152, a153)
    };
    let a154 = v1;
    let a155 = 4u64;
    if nat_lt(a154, a155) {
        let a156 = v1;
        let a157 = 2u64;
        if nat_lt(a156, a157) {
            let a158 = v1;
            let a159 = 1u64;
            if nat_lt(a158, a159) {
                let a160 = v1;
                let a161 = 0u64;
                if nat_eq(a160, a161) {
                    f23({
                        let a162 = v0;
                        let a163 = 8u64;
                        let a164 = 0u64;
                        nat_rem(a162, a163, a164)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a165 = v1;
                let a166 = 1u64;
                if nat_eq(a165, a166) {
                    f24({
                        let a167 = v0;
                        let a168 = 8u64;
                        let a169 = 0u64;
                        nat_rem(a167, a168, a169)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a170 = v1;
            let a171 = 3u64;
            if nat_lt(a170, a171) {
                let a172 = v1;
                let a173 = 2u64;
                if nat_eq(a172, a173) {
                    f25({
                        let a174 = v0;
                        let a175 = 8u64;
                        let a176 = 0u64;
                        nat_rem(a174, a175, a176)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a177 = v1;
                let a178 = 3u64;
                if nat_eq(a177, a178) {
                    f26({
                        let a179 = v0;
                        let a180 = 8u64;
                        let a181 = 0u64;
                        nat_rem(a179, a180, a181)
                    })
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a182 = v1;
        let a183 = 6u64;
        if nat_lt(a182, a183) {
            let a184 = v1;
            let a185 = 5u64;
            if nat_lt(a184, a185) {
                let a186 = v1;
                let a187 = 4u64;
                if nat_eq(a186, a187) {
                    f27({
                        let a188 = v0;
                        let a189 = 8u64;
                        let a190 = 0u64;
                        nat_rem(a188, a189, a190)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a191 = v1;
                let a192 = 5u64;
                if nat_eq(a191, a192) {
                    f28({
                        let a193 = v0;
                        let a194 = 8u64;
                        let a195 = 0u64;
                        nat_rem(a193, a194, a195)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a196 = v1;
            let a197 = 7u64;
            if nat_lt(a196, a197) {
                let a198 = v1;
                let a199 = 6u64;
                if nat_eq(a198, a199) {
                    Ok(f29({
                        let a200 = v0;
                        let a201 = 8u64;
                        let a202 = 0u64;
                        nat_rem(a200, a201, a202)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a203 = v1;
                let a204 = 7u64;
                if nat_eq(a203, a204) {
                    Ok(f30({
                        let a205 = v0;
                        let a206 = 8u64;
                        let a207 = 0u64;
                        nat_rem(a205, a206, a207)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
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
                    Ok(f31(3u64, f32(), 1097859072u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a216 = v0;
                let a217 = 1u64;
                if nat_eq(a216, a217) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a218 = v0;
            let a219 = 3u64;
            if nat_lt(a218, a219) {
                let a220 = v0;
                let a221 = 2u64;
                if nat_eq(a220, a221) {
                    Ok(f34(0u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a222 = v0;
                let a223 = 3u64;
                if nat_eq(a222, a223) {
                    Ok(f34(0u64, f37()))
                } else {
                    Ok(f2())
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
                    Ok(f34(0u64, f38(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a230 = v0;
                let a231 = 5u64;
                if nat_eq(a230, a231) {
                    Ok(f34(f39(), f40(f37())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a232 = v0;
            let a233 = 7u64;
            if nat_lt(a232, a233) {
                let a234 = v0;
                let a235 = 6u64;
                if nat_eq(a234, a235) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a236 = v0;
                let a237 = 7u64;
                if nat_eq(a236, a237) {
                    Ok(f42(1u64, f37(), 8u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a238 = v0;
    let a239 = 4u64;
    if nat_lt(a238, a239) {
        let a240 = v0;
        let a241 = 2u64;
        if nat_lt(a240, a241) {
            let a242 = v0;
            let a243 = 1u64;
            if nat_lt(a242, a243) {
                let a244 = v0;
                let a245 = 0u64;
                if nat_eq(a244, a245) {
                    f43(f36(), f44())
                } else {
                    f2()
                }
            } else {
                let a246 = v0;
                let a247 = 1u64;
                if nat_eq(a246, a247) {
                    f43(f45(), f46())
                } else {
                    f2()
                }
            }
        } else {
            let a248 = v0;
            let a249 = 3u64;
            if nat_lt(a248, a249) {
                let a250 = v0;
                let a251 = 2u64;
                if nat_eq(a250, a251) {
                    f31(1u64, f44(), f46(), f44())
                } else {
                    f2()
                }
            } else {
                let a252 = v0;
                let a253 = 3u64;
                if nat_eq(a252, a253) {
                    f31(2u64, f44(), 1045220557u64, f44())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a254 = v0;
        let a255 = 6u64;
        if nat_lt(a254, a255) {
            let a256 = v0;
            let a257 = 5u64;
            if nat_lt(a256, a257) {
                let a258 = v0;
                let a259 = 4u64;
                if nat_eq(a258, a259) {
                    f31(0u64, 1097859072u64, f44(), f44())
                } else {
                    f2()
                }
            } else {
                let a260 = v0;
                let a261 = 5u64;
                if nat_eq(a260, a261) {
                    f31(3u64, f44(), f32(), f47())
                } else {
                    f2()
                }
            }
        } else {
            let a262 = v0;
            let a263 = 7u64;
            if nat_lt(a262, a263) {
                let a264 = v0;
                let a265 = 6u64;
                if nat_eq(a264, a265) {
                    f31(3u64, 1065353216u64, f47(), f48())
                } else {
                    f2()
                }
            } else {
                let a266 = v0;
                let a267 = 7u64;
                if nat_eq(a266, a267) {
                    f34(0u64, f37())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a268 = v0;
    let a269 = 4u64;
    if nat_lt(a268, a269) {
        let a270 = v0;
        let a271 = 2u64;
        if nat_lt(a270, a271) {
            let a272 = v0;
            let a273 = 1u64;
            if nat_lt(a272, a273) {
                let a274 = v0;
                let a275 = 0u64;
                if nat_eq(a274, a275) {
                    Ok(f34(1042u64, f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a276 = v0;
                let a277 = 1u64;
                if nat_eq(a276, a277) {
                    Ok(f50(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a278 = v0;
            let a279 = 3u64;
            if nat_lt(a278, a279) {
                let a280 = v0;
                let a281 = 2u64;
                if nat_eq(a280, a281) {
                    Ok(f34(f51(), f52(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a282 = v0;
                let a283 = 3u64;
                if nat_eq(a282, a283) {
                    Ok(f34(f53(), f54(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a284 = v0;
        let a285 = 6u64;
        if nat_lt(a284, a285) {
            let a286 = v0;
            let a287 = 5u64;
            if nat_lt(a286, a287) {
                let a288 = v0;
                let a289 = 4u64;
                if nat_eq(a288, a289) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a290 = v0;
                let a291 = 5u64;
                if nat_eq(a290, a291) {
                    Ok(f42(1u64, f37(), 8u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a292 = v0;
            let a293 = 7u64;
            if nat_lt(a292, a293) {
                let a294 = v0;
                let a295 = 6u64;
                if nat_eq(a294, a295) {
                    Ok(f34(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a296 = v0;
                let a297 = 7u64;
                if nat_eq(a296, a297) {
                    Ok(f34(1049u64, f56()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a298 = v0;
    let a299 = 4u64;
    if nat_lt(a298, a299) {
        let a300 = v0;
        let a301 = 2u64;
        if nat_lt(a300, a301) {
            let a302 = v0;
            let a303 = 1u64;
            if nat_lt(a302, a303) {
                let a304 = v0;
                let a305 = 0u64;
                if nat_eq(a304, a305) {
                    f50(2048u64)
                } else {
                    f2()
                }
            } else {
                let a306 = v0;
                let a307 = 1u64;
                if nat_eq(a306, a307) {
                    f42(2u64, 0u64, f57(), 1051u64)
                } else {
                    f2()
                }
            }
        } else {
            let a308 = v0;
            let a309 = 3u64;
            if nat_lt(a308, a309) {
                let a310 = v0;
                let a311 = 2u64;
                if nat_eq(a310, a311) {
                    f50(1062u64)
                } else {
                    f2()
                }
            } else {
                let a312 = v0;
                let a313 = 3u64;
                if nat_eq(a312, a313) {
                    f31(2u64, f58(), f58(), f46())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a314 = v0;
        let a315 = 6u64;
        if nat_lt(a314, a315) {
            let a316 = v0;
            let a317 = 5u64;
            if nat_lt(a316, a317) {
                let a318 = v0;
                let a319 = 4u64;
                if nat_eq(a318, a319) {
                    f31(2u64, f35(), f57(), f44())
                } else {
                    f2()
                }
            } else {
                let a320 = v0;
                let a321 = 5u64;
                if nat_eq(a320, a321) {
                    f42(2u64, f44(), f46(), 1055u64)
                } else {
                    f2()
                }
            }
        } else {
            let a322 = v0;
            let a323 = 7u64;
            if nat_lt(a322, a323) {
                let a324 = v0;
                let a325 = 6u64;
                if nat_eq(a324, a325) {
                    f50(1062u64)
                } else {
                    f2()
                }
            } else {
                let a326 = v0;
                let a327 = 7u64;
                if nat_eq(a326, a327) {
                    f31(3u64, f58(), f57(), f33())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a328 = v0;
    let a329 = 4u64;
    if nat_lt(a328, a329) {
        let a330 = v0;
        let a331 = 2u64;
        if nat_lt(a330, a331) {
            let a332 = v0;
            let a333 = 1u64;
            if nat_lt(a332, a333) {
                let a334 = v0;
                let a335 = 0u64;
                if nat_eq(a334, a335) {
                    Ok(f31(2u64, f33(), f58(), f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a336 = v0;
                let a337 = 1u64;
                if nat_eq(a336, a337) {
                    Ok(f34(0u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a338 = v0;
            let a339 = 3u64;
            if nat_lt(a338, a339) {
                let a340 = v0;
                let a341 = 2u64;
                if nat_eq(a340, a341) {
                    Ok(f34(f52(f37())?, f38(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a342 = v0;
                let a343 = 3u64;
                if nat_eq(a342, a343) {
                    Ok(f34(f54(f37())?, f40(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a344 = v0;
        let a345 = 6u64;
        if nat_lt(a344, a345) {
            let a346 = v0;
            let a347 = 5u64;
            if nat_lt(a346, a347) {
                let a348 = v0;
                let a349 = 4u64;
                if nat_eq(a348, a349) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a350 = v0;
                let a351 = 5u64;
                if nat_eq(a350, a351) {
                    Ok(f42(1u64, f37(), 8u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a352 = v0;
            let a353 = 7u64;
            if nat_lt(a352, a353) {
                let a354 = v0;
                let a355 = 6u64;
                if nat_eq(a354, a355) {
                    Ok(f41(0u64, f36(), 1u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a356 = v0;
                let a357 = 7u64;
                if nat_eq(a356, a357) {
                    Ok(f41(2u64, f45(), 2u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a358 = v0;
    let a359 = 1u64;
    if nat_lt(a358, a359) {
        let a360 = v0;
        let a361 = 0u64;
        if nat_eq(a360, a361) {
            f41(0u64, f46(), 1u64, f46())
        } else {
            f2()
        }
    } else {
        let a362 = v0;
        let a363 = 2u64;
        if nat_lt(a362, a363) {
            let a364 = v0;
            let a365 = 1u64;
            if nat_eq(a364, a365) {
                f42(1u64, f36(), f46(), 1032u64)
            } else {
                f2()
            }
        } else {
            let a366 = v0;
            let a367 = 2u64;
            if nat_eq(a366, a367) {
                f50(2560u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a368 = v0;
    let a369 = 4u64;
    if nat_lt(a368, a369) {
        let a370 = v0;
        let a371 = 2u64;
        if nat_lt(a370, a371) {
            let a372 = v0;
            let a373 = 1u64;
            if nat_lt(a372, a373) {
                let a374 = v0;
                let a375 = 0u64;
                if nat_eq(a374, a375) {
                    Ok(f34(0u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a376 = v0;
                let a377 = 1u64;
                if nat_eq(a376, a377) {
                    Ok(f34(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a378 = v0;
            let a379 = 3u64;
            if nat_lt(a378, a379) {
                let a380 = v0;
                let a381 = 2u64;
                if nat_eq(a380, a381) {
                    Ok(f41(2u64, f37(), 4u64, f61()))
                } else {
                    Ok(f2())
                }
            } else {
                let a382 = v0;
                let a383 = 3u64;
                if nat_eq(a382, a383) {
                    Ok(f41(0u64, f61(), f60(), f62()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a384 = v0;
        let a385 = 6u64;
        if nat_lt(a384, a385) {
            let a386 = v0;
            let a387 = 5u64;
            if nat_lt(a386, a387) {
                let a388 = v0;
                let a389 = 4u64;
                if nat_eq(a388, a389) {
                    Ok(f31(2u64, f47(), f63(f62())?, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a390 = v0;
                let a391 = 5u64;
                if nat_eq(a390, a391) {
                    Ok(f31(1u64, f46(), 1065353216u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a392 = v0;
            let a393 = 7u64;
            if nat_lt(a392, a393) {
                let a394 = v0;
                let a395 = 6u64;
                if nat_eq(a394, a395) {
                    Ok(f31(2u64, 1056964608u64, f46(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a396 = v0;
                let a397 = 7u64;
                if nat_eq(a396, a397) {
                    Ok(f31(0u64, f46(), 1262485504u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a398 = v0;
    let a399 = 4u64;
    if nat_lt(a398, a399) {
        let a400 = v0;
        let a401 = 2u64;
        if nat_lt(a400, a401) {
            let a402 = v0;
            let a403 = 1u64;
            if nat_lt(a402, a403) {
                let a404 = v0;
                let a405 = 0u64;
                if nat_eq(a404, a405) {
                    Ok(f64(2u64, f46(), 7u64, 0u64, f65()))
                } else {
                    Ok(f2())
                }
            } else {
                let a406 = v0;
                let a407 = 1u64;
                if nat_eq(a406, a407) {
                    Ok(f64(3u64, f59(), f60(), f65(), f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a408 = v0;
            let a409 = 3u64;
            if nat_lt(a408, a409) {
                let a410 = v0;
                let a411 = 2u64;
                if nat_eq(a410, a411) {
                    Ok(f41(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a412 = v0;
                let a413 = 3u64;
                if nat_eq(a412, a413) {
                    Ok(f42(1u64, f60(), 4u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a414 = v0;
        let a415 = 6u64;
        if nat_lt(a414, a415) {
            let a416 = v0;
            let a417 = 5u64;
            if nat_lt(a416, a417) {
                let a418 = v0;
                let a419 = 4u64;
                if nat_eq(a418, a419) {
                    Ok(f66(4u64, f59(), 0u64, 0u64, f67(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a420 = v0;
                let a421 = 5u64;
                if nat_eq(a420, a421) {
                    Ok(f41(0u64, f67(), 1u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a422 = v0;
            let a423 = 7u64;
            if nat_lt(a422, a423) {
                let a424 = v0;
                let a425 = 6u64;
                if nat_eq(a424, a425) {
                    Ok(f34(f68(f67())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a426 = v0;
                let a427 = 7u64;
                if nat_eq(a426, a427) {
                    Ok(f41(0u64, f67(), 1u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a428 = v0;
    let a429 = 4u64;
    if nat_lt(a428, a429) {
        let a430 = v0;
        let a431 = 2u64;
        if nat_lt(a430, a431) {
            let a432 = v0;
            let a433 = 1u64;
            if nat_lt(a432, a433) {
                let a434 = v0;
                let a435 = 0u64;
                if nat_eq(a434, a435) {
                    Ok(f34(f68(f67())?, f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a436 = v0;
                let a437 = 1u64;
                if nat_eq(a436, a437) {
                    Ok(f34(0u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a438 = v0;
            let a439 = 3u64;
            if nat_lt(a438, a439) {
                let a440 = v0;
                let a441 = 2u64;
                if nat_eq(a440, a441) {
                    Ok(f34(f68(f67())?, f71()))
                } else {
                    Ok(f2())
                }
            } else {
                let a442 = v0;
                let a443 = 3u64;
                if nat_eq(a442, a443) {
                    Ok(f41(0u64, f71(), 1u64, f71()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a444 = v0;
        let a445 = 6u64;
        if nat_lt(a444, a445) {
            let a446 = v0;
            let a447 = 5u64;
            if nat_lt(a446, a447) {
                let a448 = v0;
                let a449 = 4u64;
                if nat_eq(a448, a449) {
                    Ok(f41(0u64, f69(), f71(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a450 = v0;
                let a451 = 5u64;
                if nat_eq(a450, a451) {
                    Ok(f41(0u64, f70(), f71(), f70()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a452 = v0;
            let a453 = 7u64;
            if nat_lt(a452, a453) {
                let a454 = v0;
                let a455 = 6u64;
                if nat_eq(a454, a455) {
                    Ok(f34(f68(f69())?, f51()))
                } else {
                    Ok(f2())
                }
            } else {
                let a456 = v0;
                let a457 = 7u64;
                if nat_eq(a456, a457) {
                    Ok(f41(0u64, f69(), 1u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a458 = v0;
    let a459 = 4u64;
    if nat_lt(a458, a459) {
        let a460 = v0;
        let a461 = 2u64;
        if nat_lt(a460, a461) {
            let a462 = v0;
            let a463 = 1u64;
            if nat_lt(a462, a463) {
                let a464 = v0;
                let a465 = 0u64;
                if nat_eq(a464, a465) {
                    Ok(f34(1u64, f53()))
                } else {
                    Ok(f2())
                }
            } else {
                let a466 = v0;
                let a467 = 1u64;
                if nat_eq(a466, a467) {
                    Ok(f42(0u64, f70(), f46(), 1586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a468 = v0;
            let a469 = 3u64;
            if nat_lt(a468, a469) {
                let a470 = v0;
                let a471 = 2u64;
                if nat_eq(a470, a471) {
                    Ok(f34(0u64, f53()))
                } else {
                    Ok(f2())
                }
            } else {
                let a472 = v0;
                let a473 = 3u64;
                if nat_eq(a472, a473) {
                    Ok(f34(2139095039u64, f72()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a474 = v0;
        let a475 = 6u64;
        if nat_lt(a474, a475) {
            let a476 = v0;
            let a477 = 5u64;
            if nat_lt(a476, a477) {
                let a478 = v0;
                let a479 = 4u64;
                if nat_eq(a478, a479) {
                    Ok(f34(f69(), f73()))
                } else {
                    Ok(f2())
                }
            } else {
                let a480 = v0;
                let a481 = 5u64;
                if nat_eq(a480, a481) {
                    Ok(f34(f68(f73())?, f74()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a482 = v0;
            let a483 = 7u64;
            if nat_lt(a482, a483) {
                let a484 = v0;
                let a485 = 6u64;
                if nat_eq(a484, a485) {
                    Ok(f34(0u64, f75()))
                } else {
                    Ok(f2())
                }
            } else {
                let a486 = v0;
                let a487 = 7u64;
                if nat_eq(a486, a487) {
                    Ok(f34(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a488 = v0;
    let a489 = 4u64;
    if nat_lt(a488, a489) {
        let a490 = v0;
        let a491 = 2u64;
        if nat_lt(a490, a491) {
            let a492 = v0;
            let a493 = 1u64;
            if nat_lt(a492, a493) {
                let a494 = v0;
                let a495 = 0u64;
                if nat_eq(a494, a495) {
                    Ok(f66(4u64, 0u64, f74(), 0u64, f46(), f76()))
                } else {
                    Ok(f2())
                }
            } else {
                let a496 = v0;
                let a497 = 1u64;
                if nat_eq(a496, a497) {
                    Ok(f41(0u64, f76(), f60(), f76()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a498 = v0;
            let a499 = 3u64;
            if nat_lt(a498, a499) {
                let a500 = v0;
                let a501 = 2u64;
                if nat_eq(a500, a501) {
                    Ok(f64(5u64, f77(f76())?, 0u64, 0u64, f78()))
                } else {
                    Ok(f2())
                }
            } else {
                let a502 = v0;
                let a503 = 3u64;
                if nat_eq(a502, a503) {
                    Ok(f43(f78(), f78()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a504 = v0;
        let a505 = 6u64;
        if nat_lt(a504, a505) {
            let a506 = v0;
            let a507 = 5u64;
            if nat_lt(a506, a507) {
                let a508 = v0;
                let a509 = 4u64;
                if nat_eq(a508, a509) {
                    Ok(f41(0u64, f61(), f60(), f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a510 = v0;
                let a511 = 5u64;
                if nat_eq(a510, a511) {
                    Ok(f31(2u64, f48(), f78(), f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a512 = v0;
            let a513 = 7u64;
            if nat_lt(a512, a513) {
                let a514 = v0;
                let a515 = 6u64;
                if nat_eq(a514, a515) {
                    Ok(f31(1u64, f44(), f63(f62())?, f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a516 = v0;
                let a517 = 7u64;
                if nat_eq(a516, a517) {
                    Ok(f31(2u64, f79(f62())?, f44(), f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a518 = v0;
    let a519 = 4u64;
    if nat_lt(a518, a519) {
        let a520 = v0;
        let a521 = 2u64;
        if nat_lt(a520, a521) {
            let a522 = v0;
            let a523 = 1u64;
            if nat_lt(a522, a523) {
                let a524 = v0;
                let a525 = 0u64;
                if nat_eq(a524, a525) {
                    f31(2u64, f46(), f44(), f46())
                } else {
                    f2()
                }
            } else {
                let a526 = v0;
                let a527 = 1u64;
                if nat_eq(a526, a527) {
                    f31(0u64, f75(), f46(), f75())
                } else {
                    f2()
                }
            }
        } else {
            let a528 = v0;
            let a529 = 3u64;
            if nat_lt(a528, a529) {
                let a530 = v0;
                let a531 = 2u64;
                if nat_eq(a530, a531) {
                    f41(0u64, f60(), 1u64, f60())
                } else {
                    f2()
                }
            } else {
                let a532 = v0;
                let a533 = 3u64;
                if nat_eq(a532, a533) {
                    f42(1u64, f60(), 4u64, 1568u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a534 = v0;
        let a535 = 6u64;
        if nat_lt(a534, a535) {
            let a536 = v0;
            let a537 = 5u64;
            if nat_lt(a536, a537) {
                let a538 = v0;
                let a539 = 4u64;
                if nat_eq(a538, a539) {
                    f42(2u64, f75(), f72(), 1582u64)
                } else {
                    f2()
                }
            } else {
                let a540 = v0;
                let a541 = 5u64;
                if nat_eq(a540, a541) {
                    f50(1584u64)
                } else {
                    f2()
                }
            }
        } else {
            let a542 = v0;
            let a543 = 7u64;
            if nat_lt(a542, a543) {
                let a544 = v0;
                let a545 = 6u64;
                if nat_eq(a544, a545) {
                    f34(f75(), f72())
                } else {
                    f2()
                }
            } else {
                let a546 = v0;
                let a547 = 7u64;
                if nat_eq(a546, a547) {
                    f34(f74(), f51())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a548 = v0;
    let a549 = 1u64;
    if nat_lt(a548, a549) {
        let a550 = v0;
        let a551 = 0u64;
        if nat_eq(a550, a551) {
            f41(0u64, f73(), 1u64, f73())
        } else {
            f2()
        }
    } else {
        let a552 = v0;
        let a553 = 2u64;
        if nat_lt(a552, a553) {
            let a554 = v0;
            let a555 = 1u64;
            if nat_eq(a554, a555) {
                f42(1u64, f73(), f70(), 1565u64)
            } else {
                f2()
            }
        } else {
            let a556 = v0;
            let a557 = 2u64;
            if nat_eq(a556, a557) {
                f80(f49())
            } else {
                f2()
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a558 = v0;
    let a559 = 4u64;
    if nat_lt(a558, a559) {
        let a560 = v0;
        let a561 = 2u64;
        if nat_lt(a560, a561) {
            let a562 = v0;
            let a563 = 1u64;
            if nat_lt(a562, a563) {
                let a564 = v0;
                let a565 = 0u64;
                if nat_eq(a564, a565) {
                    Ok(f31(3u64, f32(), 1084227584u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a566 = v0;
                let a567 = 1u64;
                if nat_eq(a566, a567) {
                    Ok(f34(0u64, f35()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a568 = v0;
            let a569 = 3u64;
            if nat_lt(a568, a569) {
                let a570 = v0;
                let a571 = 2u64;
                if nat_eq(a570, a571) {
                    Ok(f34(0u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a572 = v0;
                let a573 = 3u64;
                if nat_eq(a572, a573) {
                    Ok(f34(0u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a574 = v0;
        let a575 = 6u64;
        if nat_lt(a574, a575) {
            let a576 = v0;
            let a577 = 5u64;
            if nat_lt(a576, a577) {
                let a578 = v0;
                let a579 = 4u64;
                if nat_eq(a578, a579) {
                    Ok(f34(0u64, f38(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a580 = v0;
                let a581 = 5u64;
                if nat_eq(a580, a581) {
                    Ok(f34(1u64, f40(f37())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a582 = v0;
            let a583 = 7u64;
            if nat_lt(a582, a583) {
                let a584 = v0;
                let a585 = 6u64;
                if nat_eq(a584, a585) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a586 = v0;
                let a587 = 7u64;
                if nat_eq(a586, a587) {
                    Ok(f42(1u64, f37(), 2u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a588 = v0;
    let a589 = 4u64;
    if nat_lt(a588, a589) {
        let a590 = v0;
        let a591 = 2u64;
        if nat_lt(a590, a591) {
            let a592 = v0;
            let a593 = 1u64;
            if nat_lt(a592, a593) {
                let a594 = v0;
                let a595 = 0u64;
                if nat_eq(a594, a595) {
                    f43(f36(), f44())
                } else {
                    f2()
                }
            } else {
                let a596 = v0;
                let a597 = 1u64;
                if nat_eq(a596, a597) {
                    f43(f45(), f46())
                } else {
                    f2()
                }
            }
        } else {
            let a598 = v0;
            let a599 = 3u64;
            if nat_lt(a598, a599) {
                let a600 = v0;
                let a601 = 2u64;
                if nat_eq(a600, a601) {
                    f31(1u64, f44(), f46(), f44())
                } else {
                    f2()
                }
            } else {
                let a602 = v0;
                let a603 = 3u64;
                if nat_eq(a602, a603) {
                    f31(2u64, f44(), 1036831949u64, f44())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a604 = v0;
        let a605 = 6u64;
        if nat_lt(a604, a605) {
            let a606 = v0;
            let a607 = 5u64;
            if nat_lt(a606, a607) {
                let a608 = v0;
                let a609 = 4u64;
                if nat_eq(a608, a609) {
                    f31(0u64, 1084227584u64, f44(), f44())
                } else {
                    f2()
                }
            } else {
                let a610 = v0;
                let a611 = 5u64;
                if nat_eq(a610, a611) {
                    f31(3u64, f44(), f32(), f47())
                } else {
                    f2()
                }
            }
        } else {
            let a612 = v0;
            let a613 = 7u64;
            if nat_lt(a612, a613) {
                let a614 = v0;
                let a615 = 6u64;
                if nat_eq(a614, a615) {
                    f31(3u64, 1065353216u64, f47(), f48())
                } else {
                    f2()
                }
            } else {
                let a616 = v0;
                let a617 = 7u64;
                if nat_eq(a616, a617) {
                    f34(0u64, f37())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a618 = v0;
    let a619 = 4u64;
    if nat_lt(a618, a619) {
        let a620 = v0;
        let a621 = 2u64;
        if nat_lt(a620, a621) {
            let a622 = v0;
            let a623 = 1u64;
            if nat_lt(a622, a623) {
                let a624 = v0;
                let a625 = 0u64;
                if nat_eq(a624, a625) {
                    Ok(f34(1042u64, f49()))
                } else {
                    Ok(f2())
                }
            } else {
                let a626 = v0;
                let a627 = 1u64;
                if nat_eq(a626, a627) {
                    Ok(f50(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a628 = v0;
            let a629 = 3u64;
            if nat_lt(a628, a629) {
                let a630 = v0;
                let a631 = 2u64;
                if nat_eq(a630, a631) {
                    Ok(f34(f51(), f52(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a632 = v0;
                let a633 = 3u64;
                if nat_eq(a632, a633) {
                    Ok(f34(f53(), f54(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a634 = v0;
        let a635 = 6u64;
        if nat_lt(a634, a635) {
            let a636 = v0;
            let a637 = 5u64;
            if nat_lt(a636, a637) {
                let a638 = v0;
                let a639 = 4u64;
                if nat_eq(a638, a639) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a640 = v0;
                let a641 = 5u64;
                if nat_eq(a640, a641) {
                    Ok(f42(1u64, f37(), 2u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a642 = v0;
            let a643 = 7u64;
            if nat_lt(a642, a643) {
                let a644 = v0;
                let a645 = 6u64;
                if nat_eq(a644, a645) {
                    Ok(f34(0u64, f55()))
                } else {
                    Ok(f2())
                }
            } else {
                let a646 = v0;
                let a647 = 7u64;
                if nat_eq(a646, a647) {
                    Ok(f34(1049u64, f56()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a648 = v0;
    let a649 = 4u64;
    if nat_lt(a648, a649) {
        let a650 = v0;
        let a651 = 2u64;
        if nat_lt(a650, a651) {
            let a652 = v0;
            let a653 = 1u64;
            if nat_lt(a652, a653) {
                let a654 = v0;
                let a655 = 0u64;
                if nat_eq(a654, a655) {
                    Ok(f31(2u64, f33(), f58(), f35()))
                } else {
                    Ok(f2())
                }
            } else {
                let a656 = v0;
                let a657 = 1u64;
                if nat_eq(a656, a657) {
                    Ok(f34(0u64, f37()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a658 = v0;
            let a659 = 3u64;
            if nat_lt(a658, a659) {
                let a660 = v0;
                let a661 = 2u64;
                if nat_eq(a660, a661) {
                    Ok(f34(f52(f37())?, f38(f37())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a662 = v0;
                let a663 = 3u64;
                if nat_eq(a662, a663) {
                    Ok(f34(f54(f37())?, f40(f37())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a664 = v0;
        let a665 = 6u64;
        if nat_lt(a664, a665) {
            let a666 = v0;
            let a667 = 5u64;
            if nat_lt(a666, a667) {
                let a668 = v0;
                let a669 = 4u64;
                if nat_eq(a668, a669) {
                    Ok(f41(0u64, f37(), 1u64, f37()))
                } else {
                    Ok(f2())
                }
            } else {
                let a670 = v0;
                let a671 = 5u64;
                if nat_eq(a670, a671) {
                    Ok(f42(1u64, f37(), 2u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a672 = v0;
            let a673 = 7u64;
            if nat_lt(a672, a673) {
                let a674 = v0;
                let a675 = 6u64;
                if nat_eq(a674, a675) {
                    Ok(f41(0u64, f36(), 1u64, f36()))
                } else {
                    Ok(f2())
                }
            } else {
                let a676 = v0;
                let a677 = 7u64;
                if nat_eq(a676, a677) {
                    Ok(f41(2u64, f45(), 2u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a678 = v0;
    let a679 = 4u64;
    if nat_lt(a678, a679) {
        let a680 = v0;
        let a681 = 2u64;
        if nat_lt(a680, a681) {
            let a682 = v0;
            let a683 = 1u64;
            if nat_lt(a682, a683) {
                let a684 = v0;
                let a685 = 0u64;
                if nat_eq(a684, a685) {
                    Ok(f34(0u64, f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a686 = v0;
                let a687 = 1u64;
                if nat_eq(a686, a687) {
                    Ok(f34(0u64, f60()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a688 = v0;
            let a689 = 3u64;
            if nat_lt(a688, a689) {
                let a690 = v0;
                let a691 = 2u64;
                if nat_eq(a690, a691) {
                    Ok(f41(2u64, f37(), 8u64, f61()))
                } else {
                    Ok(f2())
                }
            } else {
                let a692 = v0;
                let a693 = 3u64;
                if nat_eq(a692, a693) {
                    Ok(f41(0u64, f61(), f60(), f62()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a694 = v0;
        let a695 = 6u64;
        if nat_lt(a694, a695) {
            let a696 = v0;
            let a697 = 5u64;
            if nat_lt(a696, a697) {
                let a698 = v0;
                let a699 = 4u64;
                if nat_eq(a698, a699) {
                    Ok(f31(2u64, f47(), f63(f62())?, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a700 = v0;
                let a701 = 5u64;
                if nat_eq(a700, a701) {
                    Ok(f31(1u64, f46(), 1065353216u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a702 = v0;
            let a703 = 7u64;
            if nat_lt(a702, a703) {
                let a704 = v0;
                let a705 = 6u64;
                if nat_eq(a704, a705) {
                    Ok(f31(2u64, 1056964608u64, f46(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a706 = v0;
                let a707 = 7u64;
                if nat_eq(a706, a707) {
                    Ok(f31(0u64, f46(), 1262485504u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a708 = v0;
    let a709 = 4u64;
    if nat_lt(a708, a709) {
        let a710 = v0;
        let a711 = 2u64;
        if nat_lt(a710, a711) {
            let a712 = v0;
            let a713 = 1u64;
            if nat_lt(a712, a713) {
                let a714 = v0;
                let a715 = 0u64;
                if nat_eq(a714, a715) {
                    Ok(f64(2u64, f46(), 2u64, 0u64, f65()))
                } else {
                    Ok(f2())
                }
            } else {
                let a716 = v0;
                let a717 = 1u64;
                if nat_eq(a716, a717) {
                    Ok(f64(3u64, f59(), f60(), f65(), f59()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a718 = v0;
            let a719 = 3u64;
            if nat_lt(a718, a719) {
                let a720 = v0;
                let a721 = 2u64;
                if nat_eq(a720, a721) {
                    Ok(f41(0u64, f60(), 1u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a722 = v0;
                let a723 = 3u64;
                if nat_eq(a722, a723) {
                    Ok(f42(1u64, f60(), 8u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a724 = v0;
        let a725 = 6u64;
        if nat_lt(a724, a725) {
            let a726 = v0;
            let a727 = 5u64;
            if nat_lt(a726, a727) {
                let a728 = v0;
                let a729 = 4u64;
                if nat_eq(a728, a729) {
                    Ok(f66(4u64, f59(), 0u64, 0u64, f67(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a730 = v0;
                let a731 = 5u64;
                if nat_eq(a730, a731) {
                    Ok(f41(0u64, f67(), 1u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a732 = v0;
            let a733 = 7u64;
            if nat_lt(a732, a733) {
                let a734 = v0;
                let a735 = 6u64;
                if nat_eq(a734, a735) {
                    Ok(f34(f68(f67())?, f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a736 = v0;
                let a737 = 7u64;
                if nat_eq(a736, a737) {
                    Ok(f41(0u64, f67(), 1u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a738 = v0;
    let a739 = 4u64;
    if nat_lt(a738, a739) {
        let a740 = v0;
        let a741 = 2u64;
        if nat_lt(a740, a741) {
            let a742 = v0;
            let a743 = 1u64;
            if nat_lt(a742, a743) {
                let a744 = v0;
                let a745 = 0u64;
                if nat_eq(a744, a745) {
                    Ok(f34(f68(f67())?, f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a746 = v0;
                let a747 = 1u64;
                if nat_eq(a746, a747) {
                    Ok(f42(0u64, f39(), 0u64, 1560u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a748 = v0;
            let a749 = 3u64;
            if nat_lt(a748, a749) {
                let a750 = v0;
                let a751 = 2u64;
                if nat_eq(a750, a751) {
                    Ok(f41(0u64, f67(), 6561u64, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a752 = v0;
                let a753 = 3u64;
                if nat_eq(a752, a753) {
                    Ok(f41(2u64, f68(f67())?, 65536u64, f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a754 = v0;
        let a755 = 6u64;
        if nat_lt(a754, a755) {
            let a756 = v0;
            let a757 = 5u64;
            if nat_lt(a756, a757) {
                let a758 = v0;
                let a759 = 4u64;
                if nat_eq(a758, a759) {
                    Ok(f41(0u64, f69(), f46(), f69()))
                } else {
                    Ok(f2())
                }
            } else {
                let a760 = v0;
                let a761 = 5u64;
                if nat_eq(a760, a761) {
                    Ok(f41(0u64, f67(), 1u64, f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a762 = v0;
            let a763 = 7u64;
            if nat_lt(a762, a763) {
                let a764 = v0;
                let a765 = 6u64;
                if nat_eq(a764, a765) {
                    Ok(f41(2u64, f68(f67())?, 65536u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a766 = v0;
                let a767 = 7u64;
                if nat_eq(a766, a767) {
                    Ok(f41(0u64, f70(), f46(), f70()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a768 = v0;
    let a769 = 4u64;
    if nat_lt(a768, a769) {
        let a770 = v0;
        let a771 = 2u64;
        if nat_lt(a770, a771) {
            let a772 = v0;
            let a773 = 1u64;
            if nat_lt(a772, a773) {
                let a774 = v0;
                let a775 = 0u64;
                if nat_eq(a774, a775) {
                    Ok(f34(0u64, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a776 = v0;
                let a777 = 1u64;
                if nat_eq(a776, a777) {
                    Ok(f34(f68(f67())?, f71()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a778 = v0;
            let a779 = 3u64;
            if nat_lt(a778, a779) {
                let a780 = v0;
                let a781 = 2u64;
                if nat_eq(a780, a781) {
                    Ok(f41(0u64, f71(), 1u64, f71()))
                } else {
                    Ok(f2())
                }
            } else {
                let a782 = v0;
                let a783 = 3u64;
                if nat_eq(a782, a783) {
                    Ok(f41(0u64, f69(), f71(), f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a784 = v0;
        let a785 = 6u64;
        if nat_lt(a784, a785) {
            let a786 = v0;
            let a787 = 5u64;
            if nat_lt(a786, a787) {
                let a788 = v0;
                let a789 = 4u64;
                if nat_eq(a788, a789) {
                    Ok(f41(0u64, f70(), f71(), f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a790 = v0;
                let a791 = 5u64;
                if nat_eq(a790, a791) {
                    Ok(f34(f68(f69())?, f51()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a792 = v0;
            let a793 = 7u64;
            if nat_lt(a792, a793) {
                let a794 = v0;
                let a795 = 6u64;
                if nat_eq(a794, a795) {
                    Ok(f41(3u64, f51(), 32768u64, f53()))
                } else {
                    Ok(f2())
                }
            } else {
                let a796 = v0;
                let a797 = 7u64;
                if nat_eq(a796, a797) {
                    Ok(f41(4u64, f51(), 32768u64, f51()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a798 = v0;
    let a799 = 4u64;
    if nat_lt(a798, a799) {
        let a800 = v0;
        let a801 = 2u64;
        if nat_lt(a800, a801) {
            let a802 = v0;
            let a803 = 1u64;
            if nat_lt(a802, a803) {
                let a804 = v0;
                let a805 = 0u64;
                if nat_eq(a804, a805) {
                    Ok(f42(0u64, f53(), 1u64, 1593u64))
                } else {
                    Ok(f2())
                }
            } else {
                let a806 = v0;
                let a807 = 1u64;
                if nat_eq(a806, a807) {
                    Ok(f34(0u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a808 = v0;
            let a809 = 3u64;
            if nat_lt(a808, a809) {
                let a810 = v0;
                let a811 = 2u64;
                if nat_eq(a810, a811) {
                    Ok(f34(2139095039u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a812 = v0;
                let a813 = 3u64;
                if nat_eq(a812, a813) {
                    Ok(f34(f69(), f73()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a814 = v0;
        let a815 = 6u64;
        if nat_lt(a814, a815) {
            let a816 = v0;
            let a817 = 5u64;
            if nat_lt(a816, a817) {
                let a818 = v0;
                let a819 = 4u64;
                if nat_eq(a818, a819) {
                    Ok(f34(f68(f73())?, f74()))
                } else {
                    Ok(f2())
                }
            } else {
                let a820 = v0;
                let a821 = 5u64;
                if nat_eq(a820, a821) {
                    Ok(f34(0u64, f75()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a822 = v0;
            let a823 = 7u64;
            if nat_lt(a822, a823) {
                let a824 = v0;
                let a825 = 6u64;
                if nat_eq(a824, a825) {
                    Ok(f34(0u64, f60()))
                } else {
                    Ok(f2())
                }
            } else {
                let a826 = v0;
                let a827 = 7u64;
                if nat_eq(a826, a827) {
                    Ok(f66(4u64, 0u64, f74(), 0u64, f46(), f76()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a828 = v0;
    let a829 = 4u64;
    if nat_lt(a828, a829) {
        let a830 = v0;
        let a831 = 2u64;
        if nat_lt(a830, a831) {
            let a832 = v0;
            let a833 = 1u64;
            if nat_lt(a832, a833) {
                let a834 = v0;
                let a835 = 0u64;
                if nat_eq(a834, a835) {
                    Ok(f41(0u64, f76(), f60(), f76()))
                } else {
                    Ok(f2())
                }
            } else {
                let a836 = v0;
                let a837 = 1u64;
                if nat_eq(a836, a837) {
                    Ok(f64(5u64, f77(f76())?, 0u64, 0u64, f78()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a838 = v0;
            let a839 = 3u64;
            if nat_lt(a838, a839) {
                let a840 = v0;
                let a841 = 2u64;
                if nat_eq(a840, a841) {
                    Ok(f43(f78(), f78()))
                } else {
                    Ok(f2())
                }
            } else {
                let a842 = v0;
                let a843 = 3u64;
                if nat_eq(a842, a843) {
                    Ok(f41(0u64, f61(), f60(), f62()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a844 = v0;
        let a845 = 6u64;
        if nat_lt(a844, a845) {
            let a846 = v0;
            let a847 = 5u64;
            if nat_lt(a846, a847) {
                let a848 = v0;
                let a849 = 4u64;
                if nat_eq(a848, a849) {
                    Ok(f31(2u64, f48(), f78(), f44()))
                } else {
                    Ok(f2())
                }
            } else {
                let a850 = v0;
                let a851 = 5u64;
                if nat_eq(a850, a851) {
                    Ok(f31(1u64, f44(), f63(f62())?, f44()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a852 = v0;
            let a853 = 7u64;
            if nat_lt(a852, a853) {
                let a854 = v0;
                let a855 = 6u64;
                if nat_eq(a854, a855) {
                    Ok(f31(2u64, f79(f62())?, f44(), f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a856 = v0;
                let a857 = 7u64;
                if nat_eq(a856, a857) {
                    Ok(f31(2u64, f46(), f44(), f46()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a858 = v0;
    let a859 = 4u64;
    if nat_lt(a858, a859) {
        let a860 = v0;
        let a861 = 2u64;
        if nat_lt(a860, a861) {
            let a862 = v0;
            let a863 = 1u64;
            if nat_lt(a862, a863) {
                let a864 = v0;
                let a865 = 0u64;
                if nat_eq(a864, a865) {
                    f31(0u64, f75(), f46(), f75())
                } else {
                    f2()
                }
            } else {
                let a866 = v0;
                let a867 = 1u64;
                if nat_eq(a866, a867) {
                    f41(0u64, f60(), 1u64, f60())
                } else {
                    f2()
                }
            }
        } else {
            let a868 = v0;
            let a869 = 3u64;
            if nat_lt(a868, a869) {
                let a870 = v0;
                let a871 = 2u64;
                if nat_eq(a870, a871) {
                    f42(1u64, f60(), 8u64, 1575u64)
                } else {
                    f2()
                }
            } else {
                let a872 = v0;
                let a873 = 3u64;
                if nat_eq(a872, a873) {
                    f42(2u64, f75(), f72(), 1589u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a874 = v0;
        let a875 = 6u64;
        if nat_lt(a874, a875) {
            let a876 = v0;
            let a877 = 5u64;
            if nat_lt(a876, a877) {
                let a878 = v0;
                let a879 = 4u64;
                if nat_eq(a878, a879) {
                    f50(1591u64)
                } else {
                    f2()
                }
            } else {
                let a880 = v0;
                let a881 = 5u64;
                if nat_eq(a880, a881) {
                    f34(f75(), f72())
                } else {
                    f2()
                }
            }
        } else {
            let a882 = v0;
            let a883 = 7u64;
            if nat_lt(a882, a883) {
                let a884 = v0;
                let a885 = 6u64;
                if nat_eq(a884, a885) {
                    f34(f74(), f51())
                } else {
                    f2()
                }
            } else {
                let a886 = v0;
                let a887 = 7u64;
                if nat_eq(a886, a887) {
                    f41(0u64, f73(), 1u64, f73())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a888 = v0;
    let a889 = 1u64;
    if nat_lt(a888, a889) {
        let a890 = v0;
        let a891 = 0u64;
        if nat_eq(a890, a891) {
            f42(1u64, f73(), f70(), 1572u64)
        } else {
            f2()
        }
    } else {
        let a892 = v0;
        let a893 = 1u64;
        if nat_eq(a892, a893) {
            f80(f49())
        } else {
            f2()
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f32() -> u64 {
    4294967315u64
}

pub fn f33() -> u64 {
    4294967316u64
}

pub fn f34(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f35() -> u64 {
    4294967317u64
}

pub fn f36() -> u64 {
    4294967318u64
}

pub fn f37() -> u64 {
    4294967323u64
}

pub fn f38(v0: u64) -> R<u64> {
    let a897 = 8589980672u64;
    let a898 = {
        let a894 = v0;
        let a895 = 4294967296u64;
        let a896 = 0u64;
        nat_rem(a894, a895, a896)
    };
    nat_add(a897, a898)
}

pub fn f39() -> u64 {
    4294967297u64
}

pub fn f40(v0: u64) -> R<u64> {
    let a902 = 8589982720u64;
    let a903 = {
        let a899 = v0;
        let a900 = 4294967296u64;
        let a901 = 0u64;
        nat_rem(a899, a900, a901)
    };
    nat_add(a902, a903)
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f43(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f44() -> u64 {
    4294967337u64
}

pub fn f45() -> u64 {
    4294967299u64
}

pub fn f46() -> u64 {
    4294967338u64
}

pub fn f47() -> u64 {
    4294967319u64
}

pub fn f48() -> u64 {
    4294967320u64
}

pub fn f49() -> u64 {
    4294967350u64
}

pub fn f50(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f51() -> u64 {
    4294967321u64
}

pub fn f52(v0: u64) -> R<u64> {
    let a907 = 8589976576u64;
    let a908 = {
        let a904 = v0;
        let a905 = 4294967296u64;
        let a906 = 0u64;
        nat_rem(a904, a905, a906)
    };
    nat_add(a907, a908)
}

pub fn f53() -> u64 {
    4294967322u64
}

pub fn f54(v0: u64) -> R<u64> {
    let a912 = 8589978624u64;
    let a913 = {
        let a909 = v0;
        let a910 = 4294967296u64;
        let a911 = 0u64;
        nat_rem(a909, a910, a911)
    };
    nat_add(a912, a913)
}

pub fn f55() -> u64 {
    4294967324u64
}

pub fn f56() -> u64 {
    4294967351u64
}

pub fn f57() -> u64 {
    4294967336u64
}

pub fn f58() -> u64 {
    4294967335u64
}

pub fn f59() -> u64 {
    4294967326u64
}

pub fn f60() -> u64 {
    4294967305u64
}

pub fn f61() -> u64 {
    4294967325u64
}

pub fn f62() -> u64 {
    4294967303u64
}

pub fn f63(v0: u64) -> R<u64> {
    let a917 = 8589950976u64;
    let a918 = {
        let a914 = v0;
        let a915 = 4294967296u64;
        let a916 = 0u64;
        nat_rem(a914, a915, a916)
    };
    nat_add(a917, a918)
}

pub fn f64(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f65() -> u64 {
    4294967331u64
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f67() -> u64 {
    4294967343u64
}

pub fn f68(v0: u64) -> R<u64> {
    let a922 = 21474836480u64;
    let a923 = {
        let a919 = v0;
        let a920 = 4294967296u64;
        let a921 = 0u64;
        nat_rem(a919, a920, a921)
    };
    nat_add(a922, a923)
}

pub fn f69() -> u64 {
    4294967327u64
}

pub fn f70() -> u64 {
    4294967328u64
}

pub fn f71() -> u64 {
    4294967339u64
}

pub fn f72() -> u64 {
    4294967334u64
}

pub fn f73() -> u64 {
    4294967329u64
}

pub fn f74() -> u64 {
    4294967353u64
}

pub fn f75() -> u64 {
    4294967333u64
}

pub fn f76() -> u64 {
    4294967330u64
}

pub fn f77(v0: u64) -> R<u64> {
    let a927 = 25769803776u64;
    let a928 = {
        let a924 = v0;
        let a925 = 4294967296u64;
        let a926 = 0u64;
        nat_rem(a924, a925, a926)
    };
    nat_add(a927, a928)
}

pub fn f78() -> u64 {
    4294967332u64
}

pub fn f79(v0: u64) -> R<u64> {
    let a932 = 8589967360u64;
    let a933 = {
        let a929 = v0;
        let a930 = 4294967296u64;
        let a931 = 0u64;
        nat_rem(a929, a930, a931)
    };
    nat_add(a932, a933)
}

pub fn f80(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}
