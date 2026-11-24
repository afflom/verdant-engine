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
                Ok(f5({
                    let a19 = v0;
                    let a20 = 8u64;
                    let a21 = 0u64;
                    nat_rem(a19, a20, a21)
                }))
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
            let a30 = 4u64;
            if nat_lt(a29, a30) {
                let a31 = v1;
                let a32 = 3u64;
                if nat_eq(a31, a32) {
                    f7({
                        let a33 = v0;
                        let a34 = 8u64;
                        let a35 = 0u64;
                        nat_rem(a33, a34, a35)
                    })
                } else {
                    Ok(f3())
                }
            } else {
                let a36 = v1;
                let a37 = 4u64;
                if nat_eq(a36, a37) {
                    f8({
                        let a38 = v0;
                        let a39 = 8u64;
                        let a40 = 0u64;
                        nat_rem(a38, a39, a40)
                    })
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let v1: u64 = {
        let a41 = v0;
        let a42 = 8u64;
        let a43 = 0u64;
        nat_quot(a41, a42, a43)
    };
    let a44 = v1;
    let a45 = 2u64;
    if nat_lt(a44, a45) {
        let a46 = v1;
        let a47 = 1u64;
        if nat_lt(a46, a47) {
            let a48 = v1;
            let a49 = 0u64;
            if nat_eq(a48, a49) {
                Ok(f9({
                    let a50 = v0;
                    let a51 = 8u64;
                    let a52 = 0u64;
                    nat_rem(a50, a51, a52)
                }))
            } else {
                Ok(f3())
            }
        } else {
            let a53 = v1;
            let a54 = 1u64;
            if nat_eq(a53, a54) {
                f10({
                    let a55 = v0;
                    let a56 = 8u64;
                    let a57 = 0u64;
                    nat_rem(a55, a56, a57)
                })
            } else {
                Ok(f3())
            }
        }
    } else {
        let a58 = v1;
        let a59 = 3u64;
        if nat_lt(a58, a59) {
            let a60 = v1;
            let a61 = 2u64;
            if nat_eq(a60, a61) {
                Ok(f11({
                    let a62 = v0;
                    let a63 = 8u64;
                    let a64 = 0u64;
                    nat_rem(a62, a63, a64)
                }))
            } else {
                Ok(f3())
            }
        } else {
            let a65 = v1;
            let a66 = 3u64;
            if nat_eq(a65, a66) {
                Ok(f12({
                    let a67 = v0;
                    let a68 = 8u64;
                    let a69 = 0u64;
                    nat_rem(a67, a68, a69)
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
    let a70 = v0;
    let a71 = 4u64;
    if nat_lt(a70, a71) {
        let a72 = v0;
        let a73 = 2u64;
        if nat_lt(a72, a73) {
            let a74 = v0;
            let a75 = 1u64;
            if nat_lt(a74, a75) {
                let a76 = v0;
                let a77 = 0u64;
                if nat_eq(a76, a77) {
                    Ok(f13(2u64, f14(), 0u64, 3074u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a78 = v0;
                let a79 = 1u64;
                if nat_eq(a78, a79) {
                    Ok(f15(3079u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a80 = v0;
            let a81 = 3u64;
            if nat_lt(a80, a81) {
                let a82 = v0;
                let a83 = 2u64;
                if nat_eq(a82, a83) {
                    Ok(f16(6u64, f14(), 0u64, f14()))
                } else {
                    Ok(f3())
                }
            } else {
                let a84 = v0;
                let a85 = 3u64;
                if nat_eq(a84, a85) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a86 = v0;
        let a87 = 6u64;
        if nat_lt(a86, a87) {
            let a88 = v0;
            let a89 = 5u64;
            if nat_lt(a88, a89) {
                let a90 = v0;
                let a91 = 4u64;
                if nat_eq(a90, a91) {
                    Ok(f19(1u64, 255u64, f20(f18())?, f20(f18())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a92 = v0;
                let a93 = 5u64;
                if nat_eq(a92, a93) {
                    Ok(f19(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a94 = v0;
            let a95 = 7u64;
            if nat_lt(a94, a95) {
                let a96 = v0;
                let a97 = 6u64;
                if nat_eq(a96, a97) {
                    Ok(f13(1u64, f18(), 4u64, 3076u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a98 = v0;
                let a99 = 7u64;
                if nat_eq(a98, a99) {
                    Ok(f17(f14(), f21(f22())?))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a100 = v0;
    let a101 = 4u64;
    if nat_lt(a100, a101) {
        let a102 = v0;
        let a103 = 2u64;
        if nat_lt(a102, a103) {
            let a104 = v0;
            let a105 = 1u64;
            if nat_lt(a104, a105) {
                let a106 = v0;
                let a107 = 0u64;
                if nat_eq(a106, a107) {
                    f13(2u64, f23(), f14(), 3082u64)
                } else {
                    f3()
                }
            } else {
                let a108 = v0;
                let a109 = 1u64;
                if nat_eq(a108, a109) {
                    f15(3083u64)
                } else {
                    f3()
                }
            }
        } else {
            let a110 = v0;
            let a111 = 3u64;
            if nat_lt(a110, a111) {
                let a112 = v0;
                let a113 = 2u64;
                if nat_eq(a112, a113) {
                    f17(f14(), f23())
                } else {
                    f3()
                }
            } else {
                let a114 = v0;
                let a115 = 3u64;
                if nat_eq(a114, a115) {
                    f17(f24(), f25())
                } else {
                    f3()
                }
            }
        }
    } else {
        let a116 = v0;
        let a117 = 6u64;
        if nat_lt(a116, a117) {
            let a118 = v0;
            let a119 = 5u64;
            if nat_lt(a118, a119) {
                let a120 = v0;
                let a121 = 4u64;
                if nat_eq(a120, a121) {
                    f13(0u64, f26(), 0u64, 3086u64)
                } else {
                    f3()
                }
            } else {
                let a122 = v0;
                let a123 = 5u64;
                if nat_eq(a122, a123) {
                    f17(f22(), f25())
                } else {
                    f3()
                }
            }
        } else {
            let a124 = v0;
            let a125 = 7u64;
            if nat_lt(a124, a125) {
                let a126 = v0;
                let a127 = 6u64;
                if nat_eq(a126, a127) {
                    f19(0u64, f24(), 1u64, f24())
                } else {
                    f3()
                }
            } else {
                let a128 = v0;
                let a129 = 7u64;
                if nat_eq(a128, a129) {
                    f17(0u64, f27())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a130 = v0;
    let a131 = 4u64;
    if nat_lt(a130, a131) {
        let a132 = v0;
        let a133 = 2u64;
        if nat_lt(a132, a133) {
            let a134 = v0;
            let a135 = 1u64;
            if nat_lt(a134, a135) {
                let a136 = v0;
                let a137 = 0u64;
                if nat_eq(a136, a137) {
                    Ok(f19(2u64, f25(), 8u64, f28()))
                } else {
                    Ok(f3())
                }
            } else {
                let a138 = v0;
                let a139 = 1u64;
                if nat_eq(a138, a139) {
                    Ok(f19(0u64, f28(), f27(), f28()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a140 = v0;
            let a141 = 3u64;
            if nat_lt(a140, a141) {
                let a142 = v0;
                let a143 = 2u64;
                if nat_eq(a142, a143) {
                    Ok(f29(7u64, f28(), 0u64, f30(f27())?, f31(), f32(), f33()))
                } else {
                    Ok(f3())
                }
            } else {
                let a144 = v0;
                let a145 = 3u64;
                if nat_eq(a144, a145) {
                    Ok(f17(f33(), f34(f31())?))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a146 = v0;
        let a147 = 6u64;
        if nat_lt(a146, a147) {
            let a148 = v0;
            let a149 = 5u64;
            if nat_lt(a148, a149) {
                let a150 = v0;
                let a151 = 4u64;
                if nat_eq(a150, a151) {
                    Ok(f13(0u64, f26(), 1u64, 3097u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a152 = v0;
                let a153 = 5u64;
                if nat_eq(a152, a153) {
                    Ok(f19(2u64, f22(), 8u64, f28()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a154 = v0;
            let a155 = 7u64;
            if nat_lt(a154, a155) {
                let a156 = v0;
                let a157 = 6u64;
                if nat_eq(a156, a157) {
                    Ok(f19(0u64, f28(), f27(), f28()))
                } else {
                    Ok(f3())
                }
            } else {
                let a158 = v0;
                let a159 = 7u64;
                if nat_eq(a158, a159) {
                    Ok(f29(7u64, f28(), 1u64, f30(f27())?, f31(), f32(), f33()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a160 = v0;
    let a161 = 4u64;
    if nat_lt(a160, a161) {
        let a162 = v0;
        let a163 = 2u64;
        if nat_lt(a162, a163) {
            let a164 = v0;
            let a165 = 1u64;
            if nat_lt(a164, a165) {
                let a166 = v0;
                let a167 = 0u64;
                if nat_eq(a166, a167) {
                    Ok(f35(f31(), f32(), f33()))
                } else {
                    Ok(f3())
                }
            } else {
                let a168 = v0;
                let a169 = 1u64;
                if nat_eq(a168, a169) {
                    Ok(f19(0u64, f27(), 1u64, f27()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a170 = v0;
            let a171 = 3u64;
            if nat_lt(a170, a171) {
                let a172 = v0;
                let a173 = 2u64;
                if nat_eq(a172, a173) {
                    Ok(f13(1u64, f27(), 8u64, 3088u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a174 = v0;
                let a175 = 3u64;
                if nat_eq(a174, a175) {
                    Ok(f17(0u64, f18()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a176 = v0;
        let a177 = 6u64;
        if nat_lt(a176, a177) {
            let a178 = v0;
            let a179 = 5u64;
            if nat_lt(a178, a179) {
                let a180 = v0;
                let a181 = 4u64;
                if nat_eq(a180, a181) {
                    Ok(f36(9u64, f25(), f18(), 0u64, f31(), f32()))
                } else {
                    Ok(f3())
                }
            } else {
                let a182 = v0;
                let a183 = 5u64;
                if nat_eq(a182, a183) {
                    Ok(f17(f20(f18())?, f33()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a184 = v0;
            let a185 = 7u64;
            if nat_lt(a184, a185) {
                let a186 = v0;
                let a187 = 6u64;
                if nat_eq(a186, a187) {
                    Ok(f13(0u64, f26(), 0u64, 3106u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a188 = v0;
                let a189 = 7u64;
                if nat_eq(a188, a189) {
                    Ok(f19(4u64, f33(), 128u64, f33()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a190 = v0;
    let a191 = 4u64;
    if nat_lt(a190, a191) {
        let a192 = v0;
        let a193 = 2u64;
        if nat_lt(a192, a193) {
            let a194 = v0;
            let a195 = 1u64;
            if nat_lt(a194, a195) {
                let a196 = v0;
                let a197 = 0u64;
                if nat_eq(a196, a197) {
                    Ok(f37(f31(), f32(), f33()))
                } else {
                    Ok(f3())
                }
            } else {
                let a198 = v0;
                let a199 = 1u64;
                if nat_eq(a198, a199) {
                    Ok(f15(3107u64))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a200 = v0;
            let a201 = 3u64;
            if nat_lt(a200, a201) {
                let a202 = v0;
                let a203 = 2u64;
                if nat_eq(a202, a203) {
                    Ok(f17(f33(), f34(f31())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a204 = v0;
                let a205 = 3u64;
                if nat_eq(a204, a205) {
                    Ok(f19(0u64, f18(), 1u64, f18()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a206 = v0;
        let a207 = 6u64;
        if nat_lt(a206, a207) {
            let a208 = v0;
            let a209 = 5u64;
            if nat_lt(a208, a209) {
                let a210 = v0;
                let a211 = 4u64;
                if nat_eq(a210, a211) {
                    Ok(f13(1u64, f18(), 4u64, 3100u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a212 = v0;
                let a213 = 5u64;
                if nat_eq(a212, a213) {
                    Ok(f19(0u64, f22(), 1u64, f22()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a214 = v0;
            let a215 = 7u64;
            if nat_lt(a214, a215) {
                let a216 = v0;
                let a217 = 6u64;
                if nat_eq(a216, a217) {
                    Ok(f13(1u64, f22(), 8u64, 512u64))
                } else {
                    Ok(f3())
                }
            } else {
                let a218 = v0;
                let a219 = 7u64;
                if nat_eq(a218, a219) {
                    Ok(f15(3584u64))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a220 = v0;
    let a221 = 4u64;
    if nat_lt(a220, a221) {
        let a222 = v0;
        let a223 = 2u64;
        if nat_lt(a222, a223) {
            let a224 = v0;
            let a225 = 1u64;
            if nat_lt(a224, a225) {
                let a226 = v0;
                let a227 = 0u64;
                if nat_eq(a226, a227) {
                    f13(0u64, f23(), 0u64, 3609u64)
                } else {
                    f3()
                }
            } else {
                let a228 = v0;
                let a229 = 1u64;
                if nat_eq(a228, a229) {
                    f16(3u64, f23(), 1106771968u64, f14())
                } else {
                    f3()
                }
            }
        } else {
            let a230 = v0;
            let a231 = 3u64;
            if nat_lt(a230, a231) {
                let a232 = v0;
                let a233 = 2u64;
                if nat_eq(a232, a233) {
                    f17(1065458074u64, f38())
                } else {
                    f3()
                }
            } else {
                let a234 = v0;
                let a235 = 3u64;
                if nat_eq(a234, a235) {
                    f13(0u64, f26(), 1u64, 3589u64)
                } else {
                    f3()
                }
            }
        }
    } else {
        let a236 = v0;
        let a237 = 6u64;
        if nat_lt(a236, a237) {
            let a238 = v0;
            let a239 = 5u64;
            if nat_lt(a238, a239) {
                let a240 = v0;
                let a241 = 4u64;
                if nat_eq(a240, a241) {
                    f17(1065630040u64, f38())
                } else {
                    f3()
                }
            } else {
                let a242 = v0;
                let a243 = 5u64;
                if nat_eq(a242, a243) {
                    f16(2u64, f14(), f38(), f39())
                } else {
                    f3()
                }
            }
        } else {
            let a244 = v0;
            let a245 = 7u64;
            if nat_lt(a244, a245) {
                let a246 = v0;
                let a247 = 6u64;
                if nat_eq(a246, a247) {
                    f40(f39(), f39())
                } else {
                    f3()
                }
            } else {
                let a248 = v0;
                let a249 = 7u64;
                if nat_eq(a248, a249) {
                    f19(4u64, f39(), 256u64, f33())
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    let a250 = v0;
    let a251 = 4u64;
    if nat_lt(a250, a251) {
        let a252 = v0;
        let a253 = 2u64;
        if nat_lt(a252, a253) {
            let a254 = v0;
            let a255 = 1u64;
            if nat_lt(a254, a255) {
                let a256 = v0;
                let a257 = 0u64;
                if nat_eq(a256, a257) {
                    Ok(f17(0u64, f31()))
                } else {
                    Ok(f3())
                }
            } else {
                let a258 = v0;
                let a259 = 1u64;
                if nat_eq(a258, a259) {
                    Ok(f17(f33(), f34(f31())?))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a260 = v0;
            let a261 = 3u64;
            if nat_lt(a260, a261) {
                let a262 = v0;
                let a263 = 2u64;
                if nat_eq(a262, a263) {
                    Ok(f19(3u64, f39(), 256u64, f33()))
                } else {
                    Ok(f3())
                }
            } else {
                let a264 = v0;
                let a265 = 3u64;
                if nat_eq(a264, a265) {
                    Ok(f19(0u64, f31(), 1u64, f31()))
                } else {
                    Ok(f3())
                }
            }
        }
    } else {
        let a266 = v0;
        let a267 = 6u64;
        if nat_lt(a266, a267) {
            let a268 = v0;
            let a269 = 5u64;
            if nat_lt(a268, a269) {
                let a270 = v0;
                let a271 = 4u64;
                if nat_eq(a270, a271) {
                    Ok(f17(f33(), f34(f31())?))
                } else {
                    Ok(f3())
                }
            } else {
                let a272 = v0;
                let a273 = 5u64;
                if nat_eq(a272, a273) {
                    Ok(f16(3u64, 1065353216u64, f14(), f41()))
                } else {
                    Ok(f3())
                }
            }
        } else {
            let a274 = v0;
            let a275 = 7u64;
            if nat_lt(a274, a275) {
                let a276 = v0;
                let a277 = 6u64;
                if nat_eq(a276, a277) {
                    Ok(f17(0u64, f22()))
                } else {
                    Ok(f3())
                }
            } else {
                let a278 = v0;
                let a279 = 7u64;
                if nat_eq(a278, a279) {
                    Ok(f16(2u64, f41(), f21(f22())?, f42()))
                } else {
                    Ok(f3())
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a280 = v0;
    let a281 = 4u64;
    if nat_lt(a280, a281) {
        let a282 = v0;
        let a283 = 2u64;
        if nat_lt(a282, a283) {
            let a284 = v0;
            let a285 = 1u64;
            if nat_lt(a284, a285) {
                let a286 = v0;
                let a287 = 0u64;
                if nat_eq(a286, a287) {
                    f16(1u64, f42(), 1065353216u64, f42())
                } else {
                    f3()
                }
            } else {
                let a288 = v0;
                let a289 = 1u64;
                if nat_eq(a288, a289) {
                    f16(2u64, 1056964608u64, f42(), f42())
                } else {
                    f3()
                }
            }
        } else {
            let a290 = v0;
            let a291 = 3u64;
            if nat_lt(a290, a291) {
                let a292 = v0;
                let a293 = 2u64;
                if nat_eq(a292, a293) {
                    f16(0u64, f42(), 1262485504u64, f42())
                } else {
                    f3()
                }
            } else {
                let a294 = v0;
                let a295 = 3u64;
                if nat_eq(a294, a295) {
                    f43(2u64, f42(), 15u64, 0u64, f33())
                } else {
                    f3()
                }
            }
        }
    } else {
        let a296 = v0;
        let a297 = 6u64;
        if nat_lt(a296, a297) {
            let a298 = v0;
            let a299 = 5u64;
            if nat_lt(a298, a299) {
                let a300 = v0;
                let a301 = 4u64;
                if nat_eq(a300, a301) {
                    f36(8u64, f22(), 0u64, 0u64, f31(), f32())
                } else {
                    f3()
                }
            } else {
                let a302 = v0;
                let a303 = 5u64;
                if nat_eq(a302, a303) {
                    f35(f31(), f32(), f33())
                } else {
                    f3()
                }
            }
        } else {
            let a304 = v0;
            let a305 = 7u64;
            if nat_lt(a304, a305) {
                let a306 = v0;
                let a307 = 6u64;
                if nat_eq(a306, a307) {
                    f19(0u64, f22(), 1u64, f22())
                } else {
                    f3()
                }
            } else {
                let a308 = v0;
                let a309 = 7u64;
                if nat_eq(a308, a309) {
                    f13(1u64, f22(), 8u64, 3599u64)
                } else {
                    f3()
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    let a310 = v0;
    let a311 = 1u64;
    if nat_lt(a310, a311) {
        let a312 = v0;
        let a313 = 0u64;
        if nat_eq(a312, a313) {
            f44()
        } else {
            f3()
        }
    } else {
        let a314 = v0;
        let a315 = 1u64;
        if nat_eq(a314, a315) {
            f45()
        } else {
            f3()
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (11u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f14() -> u64 {
    4294967316u64
}

pub fn f15(v0: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (12u64, (v0, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (5u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f17(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (3u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f18() -> u64 {
    4294967304u64
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (4u64, (v0, (v1, (v2, (v3, (0u64, (0u64, 0u64)))))))
}

pub fn f20(v0: u64) -> R<u64> {
    let a319 = 8589975552u64;
    let a320 = {
        let a316 = v0;
        let a317 = 4294967296u64;
        let a318 = 0u64;
        nat_rem(a316, a317, a318)
    };
    nat_add(a319, a320)
}

pub fn f21(v0: u64) -> R<u64> {
    let a324 = 8589984768u64;
    let a325 = {
        let a321 = v0;
        let a322 = 4294967296u64;
        let a323 = 0u64;
        nat_rem(a321, a322, a323)
    };
    nat_add(a324, a325)
}

pub fn f22() -> u64 {
    4294967301u64
}

pub fn f23() -> u64 {
    4294967347u64
}

pub fn f24() -> u64 {
    4294967302u64
}

pub fn f25() -> u64 {
    4294967346u64
}

pub fn f26() -> u64 {
    4294967297u64
}

pub fn f27() -> u64 {
    4294967323u64
}

pub fn f28() -> u64 {
    4294967341u64
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (10u64, (v0, (v1, (v2, (v3, (v4, (v5, v6)))))))
}

pub fn f30(v0: u64) -> R<u64> {
    let a329 = 8589980672u64;
    let a330 = {
        let a326 = v0;
        let a327 = 4294967296u64;
        let a328 = 0u64;
        nat_rem(a326, a327, a328)
    };
    nat_add(a329, a330)
}

pub fn f31() -> u64 {
    4294967344u64
}

pub fn f32() -> u64 {
    4294967345u64
}

pub fn f33() -> u64 {
    4294967331u64
}

pub fn f34(v0: u64) -> R<u64> {
    let a334 = 30064771072u64;
    let a335 = {
        let a331 = v0;
        let a332 = 4294967296u64;
        let a333 = 0u64;
        nat_rem(a331, a332, a333)
    };
    nat_add(a334, a335)
}

pub fn f35(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (14u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (9u64, (v0, (v1, (v2, (v3, (v4, (v5, 0u64)))))))
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (15u64, (v0, (v1, (v2, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f38() -> u64 {
    4294967357u64
}

pub fn f39() -> u64 {
    4294967337u64
}

pub fn f40(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (7u64, (v0, (v1, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f41() -> u64 {
    4294967319u64
}

pub fn f42() -> u64 {
    4294967338u64
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (8u64, (v0, (v1, (v2, (v3, (v4, (0u64, 0u64)))))))
}

pub fn f44() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (1u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn f45() -> (u64, (u64, (u64, (u64, (u64, (u64, (u64, u64))))))) {
    (2u64, (0u64, (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))))
}

pub fn run(p0: u64, p1: u64) -> R<(u64, (u64, (u64, (u64, (u64, (u64, (u64, u64)))))))> {
    f0(p0, p1)
}
