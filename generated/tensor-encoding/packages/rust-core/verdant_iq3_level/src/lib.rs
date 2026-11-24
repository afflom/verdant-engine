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
    let a155 = 3u64;
    if nat_lt(a154, a155) {
        let a156 = v1;
        let a157 = 1u64;
        if nat_lt(a156, a157) {
            let a158 = v1;
            let a159 = 0u64;
            if nat_eq(a158, a159) {
                f23({
                    let a160 = v0;
                    let a161 = 8u64;
                    let a162 = 0u64;
                    nat_rem(a160, a161, a162)
                })
            } else {
                Ok(f2())
            }
        } else {
            let a163 = v1;
            let a164 = 2u64;
            if nat_lt(a163, a164) {
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
            } else {
                let a170 = v1;
                let a171 = 2u64;
                if nat_eq(a170, a171) {
                    f25({
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
    } else {
        let a175 = v1;
        let a176 = 5u64;
        if nat_lt(a175, a176) {
            let a177 = v1;
            let a178 = 4u64;
            if nat_lt(a177, a178) {
                let a179 = v1;
                let a180 = 3u64;
                if nat_eq(a179, a180) {
                    f26({
                        let a181 = v0;
                        let a182 = 8u64;
                        let a183 = 0u64;
                        nat_rem(a181, a182, a183)
                    })
                } else {
                    Ok(f2())
                }
            } else {
                let a184 = v1;
                let a185 = 4u64;
                if nat_eq(a184, a185) {
                    f16({
                        let a186 = v0;
                        let a187 = 8u64;
                        let a188 = 0u64;
                        nat_rem(a186, a187, a188)
                    })
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a189 = v1;
            let a190 = 6u64;
            if nat_lt(a189, a190) {
                let a191 = v1;
                let a192 = 5u64;
                if nat_eq(a191, a192) {
                    Ok(f27({
                        let a193 = v0;
                        let a194 = 8u64;
                        let a195 = 0u64;
                        nat_rem(a193, a194, a195)
                    }))
                } else {
                    Ok(f2())
                }
            } else {
                let a196 = v1;
                let a197 = 6u64;
                if nat_eq(a196, a197) {
                    Ok(f18({
                        let a198 = v0;
                        let a199 = 8u64;
                        let a200 = 0u64;
                        nat_rem(a198, a199, a200)
                    }))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a201 = v0;
    let a202 = 4u64;
    if nat_lt(a201, a202) {
        let a203 = v0;
        let a204 = 2u64;
        if nat_lt(a203, a204) {
            let a205 = v0;
            let a206 = 1u64;
            if nat_lt(a205, a206) {
                let a207 = v0;
                let a208 = 0u64;
                if nat_eq(a207, a208) {
                    Ok(f28(3u64, f29(), 1097859072u64, f30()))
                } else {
                    Ok(f2())
                }
            } else {
                let a209 = v0;
                let a210 = 1u64;
                if nat_eq(a209, a210) {
                    Ok(f31(0u64, f32()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a211 = v0;
            let a212 = 3u64;
            if nat_lt(a211, a212) {
                let a213 = v0;
                let a214 = 2u64;
                if nat_eq(a213, a214) {
                    Ok(f31(0u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a215 = v0;
                let a216 = 3u64;
                if nat_eq(a215, a216) {
                    Ok(f31(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a217 = v0;
        let a218 = 6u64;
        if nat_lt(a217, a218) {
            let a219 = v0;
            let a220 = 5u64;
            if nat_lt(a219, a220) {
                let a221 = v0;
                let a222 = 4u64;
                if nat_eq(a221, a222) {
                    Ok(f31(0u64, f35(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a223 = v0;
                let a224 = 5u64;
                if nat_eq(a223, a224) {
                    Ok(f31(f36(), f37(f34())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a225 = v0;
            let a226 = 7u64;
            if nat_lt(a225, a226) {
                let a227 = v0;
                let a228 = 6u64;
                if nat_eq(a227, a228) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a229 = v0;
                let a230 = 7u64;
                if nat_eq(a229, a230) {
                    Ok(f39(1u64, f34(), 8u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a231 = v0;
    let a232 = 4u64;
    if nat_lt(a231, a232) {
        let a233 = v0;
        let a234 = 2u64;
        if nat_lt(a233, a234) {
            let a235 = v0;
            let a236 = 1u64;
            if nat_lt(a235, a236) {
                let a237 = v0;
                let a238 = 0u64;
                if nat_eq(a237, a238) {
                    f40(f33(), f41())
                } else {
                    f2()
                }
            } else {
                let a239 = v0;
                let a240 = 1u64;
                if nat_eq(a239, a240) {
                    f40(f42(), f43())
                } else {
                    f2()
                }
            }
        } else {
            let a241 = v0;
            let a242 = 3u64;
            if nat_lt(a241, a242) {
                let a243 = v0;
                let a244 = 2u64;
                if nat_eq(a243, a244) {
                    f28(1u64, f41(), f43(), f41())
                } else {
                    f2()
                }
            } else {
                let a245 = v0;
                let a246 = 3u64;
                if nat_eq(a245, a246) {
                    f28(2u64, f41(), 1045220557u64, f41())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a247 = v0;
        let a248 = 6u64;
        if nat_lt(a247, a248) {
            let a249 = v0;
            let a250 = 5u64;
            if nat_lt(a249, a250) {
                let a251 = v0;
                let a252 = 4u64;
                if nat_eq(a251, a252) {
                    f28(0u64, 1097859072u64, f41(), f41())
                } else {
                    f2()
                }
            } else {
                let a253 = v0;
                let a254 = 5u64;
                if nat_eq(a253, a254) {
                    f28(3u64, f41(), f29(), f44())
                } else {
                    f2()
                }
            }
        } else {
            let a255 = v0;
            let a256 = 7u64;
            if nat_lt(a255, a256) {
                let a257 = v0;
                let a258 = 6u64;
                if nat_eq(a257, a258) {
                    f28(3u64, 1065353216u64, f44(), f45())
                } else {
                    f2()
                }
            } else {
                let a259 = v0;
                let a260 = 7u64;
                if nat_eq(a259, a260) {
                    f31(0u64, f34())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a261 = v0;
    let a262 = 4u64;
    if nat_lt(a261, a262) {
        let a263 = v0;
        let a264 = 2u64;
        if nat_lt(a263, a264) {
            let a265 = v0;
            let a266 = 1u64;
            if nat_lt(a265, a266) {
                let a267 = v0;
                let a268 = 0u64;
                if nat_eq(a267, a268) {
                    Ok(f31(1042u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a269 = v0;
                let a270 = 1u64;
                if nat_eq(a269, a270) {
                    Ok(f47(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a271 = v0;
            let a272 = 3u64;
            if nat_lt(a271, a272) {
                let a273 = v0;
                let a274 = 2u64;
                if nat_eq(a273, a274) {
                    Ok(f31(f48(), f49(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a275 = v0;
                let a276 = 3u64;
                if nat_eq(a275, a276) {
                    Ok(f31(f50(), f51(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a277 = v0;
        let a278 = 6u64;
        if nat_lt(a277, a278) {
            let a279 = v0;
            let a280 = 5u64;
            if nat_lt(a279, a280) {
                let a281 = v0;
                let a282 = 4u64;
                if nat_eq(a281, a282) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a283 = v0;
                let a284 = 5u64;
                if nat_eq(a283, a284) {
                    Ok(f39(1u64, f34(), 8u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a285 = v0;
            let a286 = 7u64;
            if nat_lt(a285, a286) {
                let a287 = v0;
                let a288 = 6u64;
                if nat_eq(a287, a288) {
                    Ok(f31(0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a289 = v0;
                let a290 = 7u64;
                if nat_eq(a289, a290) {
                    Ok(f31(1049u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a291 = v0;
    let a292 = 4u64;
    if nat_lt(a291, a292) {
        let a293 = v0;
        let a294 = 2u64;
        if nat_lt(a293, a294) {
            let a295 = v0;
            let a296 = 1u64;
            if nat_lt(a295, a296) {
                let a297 = v0;
                let a298 = 0u64;
                if nat_eq(a297, a298) {
                    f47(2048u64)
                } else {
                    f2()
                }
            } else {
                let a299 = v0;
                let a300 = 1u64;
                if nat_eq(a299, a300) {
                    f39(2u64, 0u64, f54(), 1051u64)
                } else {
                    f2()
                }
            }
        } else {
            let a301 = v0;
            let a302 = 3u64;
            if nat_lt(a301, a302) {
                let a303 = v0;
                let a304 = 2u64;
                if nat_eq(a303, a304) {
                    f47(1062u64)
                } else {
                    f2()
                }
            } else {
                let a305 = v0;
                let a306 = 3u64;
                if nat_eq(a305, a306) {
                    f28(2u64, f55(), f55(), f43())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a307 = v0;
        let a308 = 6u64;
        if nat_lt(a307, a308) {
            let a309 = v0;
            let a310 = 5u64;
            if nat_lt(a309, a310) {
                let a311 = v0;
                let a312 = 4u64;
                if nat_eq(a311, a312) {
                    f28(2u64, f32(), f54(), f41())
                } else {
                    f2()
                }
            } else {
                let a313 = v0;
                let a314 = 5u64;
                if nat_eq(a313, a314) {
                    f39(2u64, f41(), f43(), 1055u64)
                } else {
                    f2()
                }
            }
        } else {
            let a315 = v0;
            let a316 = 7u64;
            if nat_lt(a315, a316) {
                let a317 = v0;
                let a318 = 6u64;
                if nat_eq(a317, a318) {
                    f47(1062u64)
                } else {
                    f2()
                }
            } else {
                let a319 = v0;
                let a320 = 7u64;
                if nat_eq(a319, a320) {
                    f28(3u64, f55(), f54(), f30())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a321 = v0;
    let a322 = 4u64;
    if nat_lt(a321, a322) {
        let a323 = v0;
        let a324 = 2u64;
        if nat_lt(a323, a324) {
            let a325 = v0;
            let a326 = 1u64;
            if nat_lt(a325, a326) {
                let a327 = v0;
                let a328 = 0u64;
                if nat_eq(a327, a328) {
                    Ok(f28(2u64, f30(), f55(), f32()))
                } else {
                    Ok(f2())
                }
            } else {
                let a329 = v0;
                let a330 = 1u64;
                if nat_eq(a329, a330) {
                    Ok(f31(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a331 = v0;
            let a332 = 3u64;
            if nat_lt(a331, a332) {
                let a333 = v0;
                let a334 = 2u64;
                if nat_eq(a333, a334) {
                    Ok(f31(f49(f34())?, f35(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a335 = v0;
                let a336 = 3u64;
                if nat_eq(a335, a336) {
                    Ok(f31(f51(f34())?, f37(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a337 = v0;
        let a338 = 6u64;
        if nat_lt(a337, a338) {
            let a339 = v0;
            let a340 = 5u64;
            if nat_lt(a339, a340) {
                let a341 = v0;
                let a342 = 4u64;
                if nat_eq(a341, a342) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a343 = v0;
                let a344 = 5u64;
                if nat_eq(a343, a344) {
                    Ok(f39(1u64, f34(), 8u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a345 = v0;
            let a346 = 7u64;
            if nat_lt(a345, a346) {
                let a347 = v0;
                let a348 = 6u64;
                if nat_eq(a347, a348) {
                    Ok(f38(0u64, f33(), 1u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a349 = v0;
                let a350 = 7u64;
                if nat_eq(a349, a350) {
                    Ok(f38(2u64, f42(), 2u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a351 = v0;
    let a352 = 1u64;
    if nat_lt(a351, a352) {
        let a353 = v0;
        let a354 = 0u64;
        if nat_eq(a353, a354) {
            f38(0u64, f43(), 1u64, f43())
        } else {
            f2()
        }
    } else {
        let a355 = v0;
        let a356 = 2u64;
        if nat_lt(a355, a356) {
            let a357 = v0;
            let a358 = 1u64;
            if nat_eq(a357, a358) {
                f39(1u64, f33(), f43(), 1032u64)
            } else {
                f2()
            }
        } else {
            let a359 = v0;
            let a360 = 2u64;
            if nat_eq(a359, a360) {
                f47(2560u64)
            } else {
                f2()
            }
        }
    }
}

pub fn f12(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a361 = v0;
    let a362 = 4u64;
    if nat_lt(a361, a362) {
        let a363 = v0;
        let a364 = 2u64;
        if nat_lt(a363, a364) {
            let a365 = v0;
            let a366 = 1u64;
            if nat_lt(a365, a366) {
                let a367 = v0;
                let a368 = 0u64;
                if nat_eq(a367, a368) {
                    Ok(f31(0u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a369 = v0;
                let a370 = 1u64;
                if nat_eq(a369, a370) {
                    Ok(f31(0u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a371 = v0;
            let a372 = 3u64;
            if nat_lt(a371, a372) {
                let a373 = v0;
                let a374 = 2u64;
                if nat_eq(a373, a374) {
                    Ok(f38(2u64, f34(), 4u64, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a375 = v0;
                let a376 = 3u64;
                if nat_eq(a375, a376) {
                    Ok(f38(0u64, f58(), f57(), f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a377 = v0;
        let a378 = 6u64;
        if nat_lt(a377, a378) {
            let a379 = v0;
            let a380 = 5u64;
            if nat_lt(a379, a380) {
                let a381 = v0;
                let a382 = 4u64;
                if nat_eq(a381, a382) {
                    Ok(f28(2u64, f44(), f60(f59())?, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a383 = v0;
                let a384 = 5u64;
                if nat_eq(a383, a384) {
                    Ok(f28(1u64, f43(), 1065353216u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a385 = v0;
            let a386 = 7u64;
            if nat_lt(a385, a386) {
                let a387 = v0;
                let a388 = 6u64;
                if nat_eq(a387, a388) {
                    Ok(f28(2u64, 1056964608u64, f43(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a389 = v0;
                let a390 = 7u64;
                if nat_eq(a389, a390) {
                    Ok(f28(0u64, f43(), 1262485504u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a391 = v0;
    let a392 = 4u64;
    if nat_lt(a391, a392) {
        let a393 = v0;
        let a394 = 2u64;
        if nat_lt(a393, a394) {
            let a395 = v0;
            let a396 = 1u64;
            if nat_lt(a395, a396) {
                let a397 = v0;
                let a398 = 0u64;
                if nat_eq(a397, a398) {
                    Ok(f61(2u64, f43(), 7u64, 0u64, f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a399 = v0;
                let a400 = 1u64;
                if nat_eq(a399, a400) {
                    Ok(f61(3u64, f56(), f57(), f62(), f56()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a401 = v0;
            let a402 = 3u64;
            if nat_lt(a401, a402) {
                let a403 = v0;
                let a404 = 2u64;
                if nat_eq(a403, a404) {
                    Ok(f38(0u64, f57(), 1u64, f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a405 = v0;
                let a406 = 3u64;
                if nat_eq(a405, a406) {
                    Ok(f39(1u64, f57(), 4u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a407 = v0;
        let a408 = 6u64;
        if nat_lt(a407, a408) {
            let a409 = v0;
            let a410 = 5u64;
            if nat_lt(a409, a410) {
                let a411 = v0;
                let a412 = 4u64;
                if nat_eq(a411, a412) {
                    Ok(f63(4u64, f56(), 0u64, 0u64, f64(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a413 = v0;
                let a414 = 5u64;
                if nat_eq(a413, a414) {
                    Ok(f38(0u64, f64(), 1u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a415 = v0;
            let a416 = 7u64;
            if nat_lt(a415, a416) {
                let a417 = v0;
                let a418 = 6u64;
                if nat_eq(a417, a418) {
                    Ok(f31(f65(f64())?, f66()))
                } else {
                    Ok(f2())
                }
            } else {
                let a419 = v0;
                let a420 = 7u64;
                if nat_eq(a419, a420) {
                    Ok(f38(0u64, f64(), 1u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f14(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a421 = v0;
    let a422 = 4u64;
    if nat_lt(a421, a422) {
        let a423 = v0;
        let a424 = 2u64;
        if nat_lt(a423, a424) {
            let a425 = v0;
            let a426 = 1u64;
            if nat_lt(a425, a426) {
                let a427 = v0;
                let a428 = 0u64;
                if nat_eq(a427, a428) {
                    Ok(f31(f65(f64())?, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a429 = v0;
                let a430 = 1u64;
                if nat_eq(a429, a430) {
                    Ok(f31(0u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a431 = v0;
            let a432 = 3u64;
            if nat_lt(a431, a432) {
                let a433 = v0;
                let a434 = 2u64;
                if nat_eq(a433, a434) {
                    Ok(f31(f65(f64())?, f68()))
                } else {
                    Ok(f2())
                }
            } else {
                let a435 = v0;
                let a436 = 3u64;
                if nat_eq(a435, a436) {
                    Ok(f38(0u64, f68(), 1u64, f68()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a437 = v0;
        let a438 = 6u64;
        if nat_lt(a437, a438) {
            let a439 = v0;
            let a440 = 5u64;
            if nat_lt(a439, a440) {
                let a441 = v0;
                let a442 = 4u64;
                if nat_eq(a441, a442) {
                    Ok(f38(0u64, f66(), f68(), f66()))
                } else {
                    Ok(f2())
                }
            } else {
                let a443 = v0;
                let a444 = 5u64;
                if nat_eq(a443, a444) {
                    Ok(f38(0u64, f67(), f68(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a445 = v0;
            let a446 = 7u64;
            if nat_lt(a445, a446) {
                let a447 = v0;
                let a448 = 6u64;
                if nat_eq(a447, a448) {
                    Ok(f31(f65(f66())?, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a449 = v0;
                let a450 = 7u64;
                if nat_eq(a449, a450) {
                    Ok(f38(0u64, f66(), 1u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a451 = v0;
    let a452 = 4u64;
    if nat_lt(a451, a452) {
        let a453 = v0;
        let a454 = 2u64;
        if nat_lt(a453, a454) {
            let a455 = v0;
            let a456 = 1u64;
            if nat_lt(a455, a456) {
                let a457 = v0;
                let a458 = 0u64;
                if nat_eq(a457, a458) {
                    Ok(f31(1u64, f50()))
                } else {
                    Ok(f2())
                }
            } else {
                let a459 = v0;
                let a460 = 1u64;
                if nat_eq(a459, a460) {
                    Ok(f39(0u64, f67(), f43(), 1586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a461 = v0;
            let a462 = 3u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 2u64;
                if nat_eq(a463, a464) {
                    Ok(f31(0u64, f50()))
                } else {
                    Ok(f2())
                }
            } else {
                let a465 = v0;
                let a466 = 3u64;
                if nat_eq(a465, a466) {
                    Ok(f31(2139095039u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a467 = v0;
        let a468 = 6u64;
        if nat_lt(a467, a468) {
            let a469 = v0;
            let a470 = 5u64;
            if nat_lt(a469, a470) {
                let a471 = v0;
                let a472 = 4u64;
                if nat_eq(a471, a472) {
                    Ok(f31(f66(), f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a473 = v0;
                let a474 = 5u64;
                if nat_eq(a473, a474) {
                    Ok(f31(f65(f70())?, f71()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a475 = v0;
            let a476 = 7u64;
            if nat_lt(a475, a476) {
                let a477 = v0;
                let a478 = 6u64;
                if nat_eq(a477, a478) {
                    Ok(f31(0u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a479 = v0;
                let a480 = 7u64;
                if nat_eq(a479, a480) {
                    Ok(f31(0u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a481 = v0;
    let a482 = 4u64;
    if nat_lt(a481, a482) {
        let a483 = v0;
        let a484 = 2u64;
        if nat_lt(a483, a484) {
            let a485 = v0;
            let a486 = 1u64;
            if nat_lt(a485, a486) {
                let a487 = v0;
                let a488 = 0u64;
                if nat_eq(a487, a488) {
                    Ok(f63(4u64, 0u64, f71(), 0u64, f43(), f73()))
                } else {
                    Ok(f2())
                }
            } else {
                let a489 = v0;
                let a490 = 1u64;
                if nat_eq(a489, a490) {
                    Ok(f38(0u64, f73(), f57(), f73()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a491 = v0;
            let a492 = 3u64;
            if nat_lt(a491, a492) {
                let a493 = v0;
                let a494 = 2u64;
                if nat_eq(a493, a494) {
                    Ok(f61(5u64, f74(f73())?, 0u64, 0u64, f75()))
                } else {
                    Ok(f2())
                }
            } else {
                let a495 = v0;
                let a496 = 3u64;
                if nat_eq(a495, a496) {
                    Ok(f40(f75(), f75()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a497 = v0;
        let a498 = 6u64;
        if nat_lt(a497, a498) {
            let a499 = v0;
            let a500 = 5u64;
            if nat_lt(a499, a500) {
                let a501 = v0;
                let a502 = 4u64;
                if nat_eq(a501, a502) {
                    Ok(f38(0u64, f58(), f57(), f59()))
                } else {
                    Ok(f2())
                }
            } else {
                let a503 = v0;
                let a504 = 5u64;
                if nat_eq(a503, a504) {
                    Ok(f28(2u64, f45(), f75(), f41()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a505 = v0;
            let a506 = 7u64;
            if nat_lt(a505, a506) {
                let a507 = v0;
                let a508 = 6u64;
                if nat_eq(a507, a508) {
                    Ok(f28(1u64, f41(), f60(f59())?, f41()))
                } else {
                    Ok(f2())
                }
            } else {
                let a509 = v0;
                let a510 = 7u64;
                if nat_eq(a509, a510) {
                    Ok(f28(2u64, f76(f59())?, f41(), f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a511 = v0;
    let a512 = 4u64;
    if nat_lt(a511, a512) {
        let a513 = v0;
        let a514 = 2u64;
        if nat_lt(a513, a514) {
            let a515 = v0;
            let a516 = 1u64;
            if nat_lt(a515, a516) {
                let a517 = v0;
                let a518 = 0u64;
                if nat_eq(a517, a518) {
                    f28(2u64, f43(), f41(), f43())
                } else {
                    f2()
                }
            } else {
                let a519 = v0;
                let a520 = 1u64;
                if nat_eq(a519, a520) {
                    f28(0u64, f72(), f43(), f72())
                } else {
                    f2()
                }
            }
        } else {
            let a521 = v0;
            let a522 = 3u64;
            if nat_lt(a521, a522) {
                let a523 = v0;
                let a524 = 2u64;
                if nat_eq(a523, a524) {
                    f38(0u64, f57(), 1u64, f57())
                } else {
                    f2()
                }
            } else {
                let a525 = v0;
                let a526 = 3u64;
                if nat_eq(a525, a526) {
                    f39(1u64, f57(), 4u64, 1568u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a527 = v0;
        let a528 = 6u64;
        if nat_lt(a527, a528) {
            let a529 = v0;
            let a530 = 5u64;
            if nat_lt(a529, a530) {
                let a531 = v0;
                let a532 = 4u64;
                if nat_eq(a531, a532) {
                    f39(2u64, f72(), f69(), 1582u64)
                } else {
                    f2()
                }
            } else {
                let a533 = v0;
                let a534 = 5u64;
                if nat_eq(a533, a534) {
                    f47(1584u64)
                } else {
                    f2()
                }
            }
        } else {
            let a535 = v0;
            let a536 = 7u64;
            if nat_lt(a535, a536) {
                let a537 = v0;
                let a538 = 6u64;
                if nat_eq(a537, a538) {
                    f31(f72(), f69())
                } else {
                    f2()
                }
            } else {
                let a539 = v0;
                let a540 = 7u64;
                if nat_eq(a539, a540) {
                    f31(f71(), f48())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a541 = v0;
    let a542 = 1u64;
    if nat_lt(a541, a542) {
        let a543 = v0;
        let a544 = 0u64;
        if nat_eq(a543, a544) {
            f38(0u64, f70(), 1u64, f70())
        } else {
            f2()
        }
    } else {
        let a545 = v0;
        let a546 = 2u64;
        if nat_lt(a545, a546) {
            let a547 = v0;
            let a548 = 1u64;
            if nat_eq(a547, a548) {
                f39(1u64, f70(), f67(), 1565u64)
            } else {
                f2()
            }
        } else {
            let a549 = v0;
            let a550 = 2u64;
            if nat_eq(a549, a550) {
                f77(f46())
            } else {
                f2()
            }
        }
    }
}

pub fn f19(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a551 = v0;
    let a552 = 4u64;
    if nat_lt(a551, a552) {
        let a553 = v0;
        let a554 = 2u64;
        if nat_lt(a553, a554) {
            let a555 = v0;
            let a556 = 1u64;
            if nat_lt(a555, a556) {
                let a557 = v0;
                let a558 = 0u64;
                if nat_eq(a557, a558) {
                    Ok(f28(3u64, f29(), 1084227584u64, f30()))
                } else {
                    Ok(f2())
                }
            } else {
                let a559 = v0;
                let a560 = 1u64;
                if nat_eq(a559, a560) {
                    Ok(f31(0u64, f32()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a561 = v0;
            let a562 = 3u64;
            if nat_lt(a561, a562) {
                let a563 = v0;
                let a564 = 2u64;
                if nat_eq(a563, a564) {
                    Ok(f31(0u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a565 = v0;
                let a566 = 3u64;
                if nat_eq(a565, a566) {
                    Ok(f31(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a567 = v0;
        let a568 = 6u64;
        if nat_lt(a567, a568) {
            let a569 = v0;
            let a570 = 5u64;
            if nat_lt(a569, a570) {
                let a571 = v0;
                let a572 = 4u64;
                if nat_eq(a571, a572) {
                    Ok(f31(0u64, f35(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a573 = v0;
                let a574 = 5u64;
                if nat_eq(a573, a574) {
                    Ok(f31(1u64, f37(f34())?))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a575 = v0;
            let a576 = 7u64;
            if nat_lt(a575, a576) {
                let a577 = v0;
                let a578 = 6u64;
                if nat_eq(a577, a578) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a579 = v0;
                let a580 = 7u64;
                if nat_eq(a579, a580) {
                    Ok(f39(1u64, f34(), 2u64, 1028u64))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a581 = v0;
    let a582 = 4u64;
    if nat_lt(a581, a582) {
        let a583 = v0;
        let a584 = 2u64;
        if nat_lt(a583, a584) {
            let a585 = v0;
            let a586 = 1u64;
            if nat_lt(a585, a586) {
                let a587 = v0;
                let a588 = 0u64;
                if nat_eq(a587, a588) {
                    f40(f33(), f41())
                } else {
                    f2()
                }
            } else {
                let a589 = v0;
                let a590 = 1u64;
                if nat_eq(a589, a590) {
                    f40(f42(), f43())
                } else {
                    f2()
                }
            }
        } else {
            let a591 = v0;
            let a592 = 3u64;
            if nat_lt(a591, a592) {
                let a593 = v0;
                let a594 = 2u64;
                if nat_eq(a593, a594) {
                    f28(1u64, f41(), f43(), f41())
                } else {
                    f2()
                }
            } else {
                let a595 = v0;
                let a596 = 3u64;
                if nat_eq(a595, a596) {
                    f28(2u64, f41(), 1036831949u64, f41())
                } else {
                    f2()
                }
            }
        }
    } else {
        let a597 = v0;
        let a598 = 6u64;
        if nat_lt(a597, a598) {
            let a599 = v0;
            let a600 = 5u64;
            if nat_lt(a599, a600) {
                let a601 = v0;
                let a602 = 4u64;
                if nat_eq(a601, a602) {
                    f28(0u64, 1084227584u64, f41(), f41())
                } else {
                    f2()
                }
            } else {
                let a603 = v0;
                let a604 = 5u64;
                if nat_eq(a603, a604) {
                    f28(3u64, f41(), f29(), f44())
                } else {
                    f2()
                }
            }
        } else {
            let a605 = v0;
            let a606 = 7u64;
            if nat_lt(a605, a606) {
                let a607 = v0;
                let a608 = 6u64;
                if nat_eq(a607, a608) {
                    f28(3u64, 1065353216u64, f44(), f45())
                } else {
                    f2()
                }
            } else {
                let a609 = v0;
                let a610 = 7u64;
                if nat_eq(a609, a610) {
                    f31(0u64, f34())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f21(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a611 = v0;
    let a612 = 4u64;
    if nat_lt(a611, a612) {
        let a613 = v0;
        let a614 = 2u64;
        if nat_lt(a613, a614) {
            let a615 = v0;
            let a616 = 1u64;
            if nat_lt(a615, a616) {
                let a617 = v0;
                let a618 = 0u64;
                if nat_eq(a617, a618) {
                    Ok(f31(1042u64, f46()))
                } else {
                    Ok(f2())
                }
            } else {
                let a619 = v0;
                let a620 = 1u64;
                if nat_eq(a619, a620) {
                    Ok(f47(1536u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a621 = v0;
            let a622 = 3u64;
            if nat_lt(a621, a622) {
                let a623 = v0;
                let a624 = 2u64;
                if nat_eq(a623, a624) {
                    Ok(f31(f48(), f49(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a625 = v0;
                let a626 = 3u64;
                if nat_eq(a625, a626) {
                    Ok(f31(f50(), f51(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a627 = v0;
        let a628 = 6u64;
        if nat_lt(a627, a628) {
            let a629 = v0;
            let a630 = 5u64;
            if nat_lt(a629, a630) {
                let a631 = v0;
                let a632 = 4u64;
                if nat_eq(a631, a632) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a633 = v0;
                let a634 = 5u64;
                if nat_eq(a633, a634) {
                    Ok(f39(1u64, f34(), 2u64, 1040u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a635 = v0;
            let a636 = 7u64;
            if nat_lt(a635, a636) {
                let a637 = v0;
                let a638 = 6u64;
                if nat_eq(a637, a638) {
                    Ok(f31(0u64, f52()))
                } else {
                    Ok(f2())
                }
            } else {
                let a639 = v0;
                let a640 = 7u64;
                if nat_eq(a639, a640) {
                    Ok(f31(1049u64, f53()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a641 = v0;
    let a642 = 4u64;
    if nat_lt(a641, a642) {
        let a643 = v0;
        let a644 = 2u64;
        if nat_lt(a643, a644) {
            let a645 = v0;
            let a646 = 1u64;
            if nat_lt(a645, a646) {
                let a647 = v0;
                let a648 = 0u64;
                if nat_eq(a647, a648) {
                    Ok(f28(2u64, f30(), f55(), f32()))
                } else {
                    Ok(f2())
                }
            } else {
                let a649 = v0;
                let a650 = 1u64;
                if nat_eq(a649, a650) {
                    Ok(f31(0u64, f34()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a651 = v0;
            let a652 = 3u64;
            if nat_lt(a651, a652) {
                let a653 = v0;
                let a654 = 2u64;
                if nat_eq(a653, a654) {
                    Ok(f31(f49(f34())?, f35(f34())?))
                } else {
                    Ok(f2())
                }
            } else {
                let a655 = v0;
                let a656 = 3u64;
                if nat_eq(a655, a656) {
                    Ok(f31(f51(f34())?, f37(f34())?))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a657 = v0;
        let a658 = 6u64;
        if nat_lt(a657, a658) {
            let a659 = v0;
            let a660 = 5u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 4u64;
                if nat_eq(a661, a662) {
                    Ok(f38(0u64, f34(), 1u64, f34()))
                } else {
                    Ok(f2())
                }
            } else {
                let a663 = v0;
                let a664 = 5u64;
                if nat_eq(a663, a664) {
                    Ok(f39(1u64, f34(), 2u64, 1058u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a665 = v0;
            let a666 = 7u64;
            if nat_lt(a665, a666) {
                let a667 = v0;
                let a668 = 6u64;
                if nat_eq(a667, a668) {
                    Ok(f38(0u64, f33(), 1u64, f33()))
                } else {
                    Ok(f2())
                }
            } else {
                let a669 = v0;
                let a670 = 7u64;
                if nat_eq(a669, a670) {
                    Ok(f38(2u64, f42(), 2u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f23(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a671 = v0;
    let a672 = 4u64;
    if nat_lt(a671, a672) {
        let a673 = v0;
        let a674 = 2u64;
        if nat_lt(a673, a674) {
            let a675 = v0;
            let a676 = 1u64;
            if nat_lt(a675, a676) {
                let a677 = v0;
                let a678 = 0u64;
                if nat_eq(a677, a678) {
                    Ok(f31(0u64, f56()))
                } else {
                    Ok(f2())
                }
            } else {
                let a679 = v0;
                let a680 = 1u64;
                if nat_eq(a679, a680) {
                    Ok(f31(0u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a681 = v0;
            let a682 = 3u64;
            if nat_lt(a681, a682) {
                let a683 = v0;
                let a684 = 2u64;
                if nat_eq(a683, a684) {
                    Ok(f38(2u64, f34(), 8u64, f58()))
                } else {
                    Ok(f2())
                }
            } else {
                let a685 = v0;
                let a686 = 3u64;
                if nat_eq(a685, a686) {
                    Ok(f38(0u64, f58(), f57(), f59()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a687 = v0;
        let a688 = 6u64;
        if nat_lt(a687, a688) {
            let a689 = v0;
            let a690 = 5u64;
            if nat_lt(a689, a690) {
                let a691 = v0;
                let a692 = 4u64;
                if nat_eq(a691, a692) {
                    Ok(f28(2u64, f44(), f60(f59())?, f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a693 = v0;
                let a694 = 5u64;
                if nat_eq(a693, a694) {
                    Ok(f28(1u64, f43(), 1065353216u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a695 = v0;
            let a696 = 7u64;
            if nat_lt(a695, a696) {
                let a697 = v0;
                let a698 = 6u64;
                if nat_eq(a697, a698) {
                    Ok(f28(2u64, 1056964608u64, f43(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a699 = v0;
                let a700 = 7u64;
                if nat_eq(a699, a700) {
                    Ok(f28(0u64, f43(), 1262485504u64, f43()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f24(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a701 = v0;
    let a702 = 4u64;
    if nat_lt(a701, a702) {
        let a703 = v0;
        let a704 = 2u64;
        if nat_lt(a703, a704) {
            let a705 = v0;
            let a706 = 1u64;
            if nat_lt(a705, a706) {
                let a707 = v0;
                let a708 = 0u64;
                if nat_eq(a707, a708) {
                    Ok(f61(2u64, f43(), 2u64, 0u64, f62()))
                } else {
                    Ok(f2())
                }
            } else {
                let a709 = v0;
                let a710 = 1u64;
                if nat_eq(a709, a710) {
                    Ok(f61(3u64, f56(), f57(), f62(), f56()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a711 = v0;
            let a712 = 3u64;
            if nat_lt(a711, a712) {
                let a713 = v0;
                let a714 = 2u64;
                if nat_eq(a713, a714) {
                    Ok(f38(0u64, f57(), 1u64, f57()))
                } else {
                    Ok(f2())
                }
            } else {
                let a715 = v0;
                let a716 = 3u64;
                if nat_eq(a715, a716) {
                    Ok(f39(1u64, f57(), 8u64, 1539u64))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a717 = v0;
        let a718 = 6u64;
        if nat_lt(a717, a718) {
            let a719 = v0;
            let a720 = 5u64;
            if nat_lt(a719, a720) {
                let a721 = v0;
                let a722 = 4u64;
                if nat_eq(a721, a722) {
                    Ok(f63(4u64, f56(), 0u64, 0u64, f64(), f43()))
                } else {
                    Ok(f2())
                }
            } else {
                let a723 = v0;
                let a724 = 5u64;
                if nat_eq(a723, a724) {
                    Ok(f38(0u64, f64(), 1u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a725 = v0;
            let a726 = 7u64;
            if nat_lt(a725, a726) {
                let a727 = v0;
                let a728 = 6u64;
                if nat_eq(a727, a728) {
                    Ok(f31(f65(f64())?, f66()))
                } else {
                    Ok(f2())
                }
            } else {
                let a729 = v0;
                let a730 = 7u64;
                if nat_eq(a729, a730) {
                    Ok(f38(0u64, f64(), 1u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a731 = v0;
    let a732 = 4u64;
    if nat_lt(a731, a732) {
        let a733 = v0;
        let a734 = 2u64;
        if nat_lt(a733, a734) {
            let a735 = v0;
            let a736 = 1u64;
            if nat_lt(a735, a736) {
                let a737 = v0;
                let a738 = 0u64;
                if nat_eq(a737, a738) {
                    Ok(f31(f65(f64())?, f67()))
                } else {
                    Ok(f2())
                }
            } else {
                let a739 = v0;
                let a740 = 1u64;
                if nat_eq(a739, a740) {
                    Ok(f31(0u64, f64()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a741 = v0;
            let a742 = 3u64;
            if nat_lt(a741, a742) {
                let a743 = v0;
                let a744 = 2u64;
                if nat_eq(a743, a744) {
                    Ok(f31(f65(f64())?, f68()))
                } else {
                    Ok(f2())
                }
            } else {
                let a745 = v0;
                let a746 = 3u64;
                if nat_eq(a745, a746) {
                    Ok(f38(0u64, f68(), 1u64, f68()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a747 = v0;
        let a748 = 6u64;
        if nat_lt(a747, a748) {
            let a749 = v0;
            let a750 = 5u64;
            if nat_lt(a749, a750) {
                let a751 = v0;
                let a752 = 4u64;
                if nat_eq(a751, a752) {
                    Ok(f38(0u64, f66(), f68(), f66()))
                } else {
                    Ok(f2())
                }
            } else {
                let a753 = v0;
                let a754 = 5u64;
                if nat_eq(a753, a754) {
                    Ok(f38(0u64, f67(), f68(), f67()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a755 = v0;
            let a756 = 7u64;
            if nat_lt(a755, a756) {
                let a757 = v0;
                let a758 = 6u64;
                if nat_eq(a757, a758) {
                    Ok(f31(f65(f66())?, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a759 = v0;
                let a760 = 7u64;
                if nat_eq(a759, a760) {
                    Ok(f38(3u64, f48(), 32768u64, f50()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a761 = v0;
    let a762 = 4u64;
    if nat_lt(a761, a762) {
        let a763 = v0;
        let a764 = 2u64;
        if nat_lt(a763, a764) {
            let a765 = v0;
            let a766 = 1u64;
            if nat_lt(a765, a766) {
                let a767 = v0;
                let a768 = 0u64;
                if nat_eq(a767, a768) {
                    Ok(f38(4u64, f48(), 32768u64, f48()))
                } else {
                    Ok(f2())
                }
            } else {
                let a769 = v0;
                let a770 = 1u64;
                if nat_eq(a769, a770) {
                    Ok(f39(0u64, f50(), 1u64, 1586u64))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a771 = v0;
            let a772 = 3u64;
            if nat_lt(a771, a772) {
                let a773 = v0;
                let a774 = 2u64;
                if nat_eq(a773, a774) {
                    Ok(f31(0u64, f50()))
                } else {
                    Ok(f2())
                }
            } else {
                let a775 = v0;
                let a776 = 3u64;
                if nat_eq(a775, a776) {
                    Ok(f31(2139095039u64, f69()))
                } else {
                    Ok(f2())
                }
            }
        }
    } else {
        let a777 = v0;
        let a778 = 6u64;
        if nat_lt(a777, a778) {
            let a779 = v0;
            let a780 = 5u64;
            if nat_lt(a779, a780) {
                let a781 = v0;
                let a782 = 4u64;
                if nat_eq(a781, a782) {
                    Ok(f31(f66(), f70()))
                } else {
                    Ok(f2())
                }
            } else {
                let a783 = v0;
                let a784 = 5u64;
                if nat_eq(a783, a784) {
                    Ok(f31(f65(f70())?, f71()))
                } else {
                    Ok(f2())
                }
            }
        } else {
            let a785 = v0;
            let a786 = 7u64;
            if nat_lt(a785, a786) {
                let a787 = v0;
                let a788 = 6u64;
                if nat_eq(a787, a788) {
                    Ok(f31(0u64, f72()))
                } else {
                    Ok(f2())
                }
            } else {
                let a789 = v0;
                let a790 = 7u64;
                if nat_eq(a789, a790) {
                    Ok(f31(0u64, f57()))
                } else {
                    Ok(f2())
                }
            }
        }
    }
}

pub fn f27(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a791 = v0;
    let a792 = 4u64;
    if nat_lt(a791, a792) {
        let a793 = v0;
        let a794 = 2u64;
        if nat_lt(a793, a794) {
            let a795 = v0;
            let a796 = 1u64;
            if nat_lt(a795, a796) {
                let a797 = v0;
                let a798 = 0u64;
                if nat_eq(a797, a798) {
                    f28(2u64, f43(), f41(), f43())
                } else {
                    f2()
                }
            } else {
                let a799 = v0;
                let a800 = 1u64;
                if nat_eq(a799, a800) {
                    f28(0u64, f72(), f43(), f72())
                } else {
                    f2()
                }
            }
        } else {
            let a801 = v0;
            let a802 = 3u64;
            if nat_lt(a801, a802) {
                let a803 = v0;
                let a804 = 2u64;
                if nat_eq(a803, a804) {
                    f38(0u64, f57(), 1u64, f57())
                } else {
                    f2()
                }
            } else {
                let a805 = v0;
                let a806 = 3u64;
                if nat_eq(a805, a806) {
                    f39(1u64, f57(), 8u64, 1568u64)
                } else {
                    f2()
                }
            }
        }
    } else {
        let a807 = v0;
        let a808 = 6u64;
        if nat_lt(a807, a808) {
            let a809 = v0;
            let a810 = 5u64;
            if nat_lt(a809, a810) {
                let a811 = v0;
                let a812 = 4u64;
                if nat_eq(a811, a812) {
                    f39(2u64, f72(), f69(), 1582u64)
                } else {
                    f2()
                }
            } else {
                let a813 = v0;
                let a814 = 5u64;
                if nat_eq(a813, a814) {
                    f47(1584u64)
                } else {
                    f2()
                }
            }
        } else {
            let a815 = v0;
            let a816 = 7u64;
            if nat_lt(a815, a816) {
                let a817 = v0;
                let a818 = 6u64;
                if nat_eq(a817, a818) {
                    f31(f72(), f69())
                } else {
                    f2()
                }
            } else {
                let a819 = v0;
                let a820 = 7u64;
                if nat_eq(a819, a820) {
                    f31(f71(), f48())
                } else {
                    f2()
                }
            }
        }
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f29() -> u64 {
    4294967315u64
}

pub fn f30() -> u64 {
    4294967316u64
}

pub fn f31(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f32() -> u64 {
    4294967317u64
}

pub fn f33() -> u64 {
    4294967318u64
}

pub fn f34() -> u64 {
    4294967323u64
}

pub fn f35(v0: u64) -> R<u64> {
    let a824 = 8589980672u64;
    let a825 = {
        let a821 = v0;
        let a822 = 4294967296u64;
        let a823 = 0u64;
        nat_rem(a821, a822, a823)
    };
    nat_add(a824, a825)
}

pub fn f36() -> u64 {
    4294967297u64
}

pub fn f37(v0: u64) -> R<u64> {
    let a829 = 8589982720u64;
    let a830 = {
        let a826 = v0;
        let a827 = 4294967296u64;
        let a828 = 0u64;
        nat_rem(a826, a827, a828)
    };
    nat_add(a829, a830)
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f40(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (6u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f41() -> u64 {
    4294967337u64
}

pub fn f42() -> u64 {
    4294967299u64
}

pub fn f43() -> u64 {
    4294967338u64
}

pub fn f44() -> u64 {
    4294967319u64
}

pub fn f45() -> u64 {
    4294967320u64
}

pub fn f46() -> u64 {
    4294967350u64
}

pub fn f47(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f48() -> u64 {
    4294967321u64
}

pub fn f49(v0: u64) -> R<u64> {
    let a834 = 8589976576u64;
    let a835 = {
        let a831 = v0;
        let a832 = 4294967296u64;
        let a833 = 0u64;
        nat_rem(a831, a832, a833)
    };
    nat_add(a834, a835)
}

pub fn f50() -> u64 {
    4294967322u64
}

pub fn f51(v0: u64) -> R<u64> {
    let a839 = 8589978624u64;
    let a840 = {
        let a836 = v0;
        let a837 = 4294967296u64;
        let a838 = 0u64;
        nat_rem(a836, a837, a838)
    };
    nat_add(a839, a840)
}

pub fn f52() -> u64 {
    4294967324u64
}

pub fn f53() -> u64 {
    4294967351u64
}

pub fn f54() -> u64 {
    4294967336u64
}

pub fn f55() -> u64 {
    4294967335u64
}

pub fn f56() -> u64 {
    4294967326u64
}

pub fn f57() -> u64 {
    4294967305u64
}

pub fn f58() -> u64 {
    4294967325u64
}

pub fn f59() -> u64 {
    4294967303u64
}

pub fn f60(v0: u64) -> R<u64> {
    let a844 = 8589950976u64;
    let a845 = {
        let a841 = v0;
        let a842 = 4294967296u64;
        let a843 = 0u64;
        nat_rem(a841, a842, a843)
    };
    nat_add(a844, a845)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f62() -> u64 {
    4294967331u64
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f64() -> u64 {
    4294967343u64
}

pub fn f65(v0: u64) -> R<u64> {
    let a849 = 21474836480u64;
    let a850 = {
        let a846 = v0;
        let a847 = 4294967296u64;
        let a848 = 0u64;
        nat_rem(a846, a847, a848)
    };
    nat_add(a849, a850)
}

pub fn f66() -> u64 {
    4294967327u64
}

pub fn f67() -> u64 {
    4294967328u64
}

pub fn f68() -> u64 {
    4294967339u64
}

pub fn f69() -> u64 {
    4294967334u64
}

pub fn f70() -> u64 {
    4294967329u64
}

pub fn f71() -> u64 {
    4294967353u64
}

pub fn f72() -> u64 {
    4294967333u64
}

pub fn f73() -> u64 {
    4294967330u64
}

pub fn f74(v0: u64) -> R<u64> {
    let a854 = 25769803776u64;
    let a855 = {
        let a851 = v0;
        let a852 = 4294967296u64;
        let a853 = 0u64;
        nat_rem(a851, a852, a853)
    };
    nat_add(a854, a855)
}

pub fn f75() -> u64 {
    4294967332u64
}

pub fn f76(v0: u64) -> R<u64> {
    let a859 = 8589967360u64;
    let a860 = {
        let a856 = v0;
        let a857 = 4294967296u64;
        let a858 = 0u64;
        nat_rem(a856, a857, a858)
    };
    nat_add(a859, a860)
}

pub fn f77(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (13u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
