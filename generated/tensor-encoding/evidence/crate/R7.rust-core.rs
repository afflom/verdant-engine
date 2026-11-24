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

pub fn f0(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a1 = v0;
    let a2 = 32u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 16u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 6u64;
            if nat_lt(a5, a6) {
                let a7 = v0;
                let a8 = 3u64;
                if nat_lt(a7, a8) {
                    let a9 = v0;
                    let a10 = 1u64;
                    if nat_lt(a9, a10) {
                        let a11 = v0;
                        let a12 = 0u64;
                        if nat_eq(a11, a12) {
                            Ok(f1(v1, v2, v3, v4))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a13 = v0;
                        let a14 = 2u64;
                        if nat_lt(a13, a14) {
                            let a15 = v0;
                            let a16 = 1u64;
                            let _ = nat_eq(a15, a16);
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        } else {
                            let a17 = v0;
                            let a18 = 2u64;
                            if nat_eq(a17, a18) {
                                Ok(f2(v2, v3))
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        }
                    }
                } else {
                    let a19 = v0;
                    let a20 = 4u64;
                    if nat_lt(a19, a20) {
                        let a21 = v0;
                        let a22 = 3u64;
                        if nat_eq(a21, a22) {
                            f3(v2, v3, v4)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a23 = v0;
                        let a24 = 5u64;
                        if nat_lt(a23, a24) {
                            let a25 = v0;
                            let a26 = 4u64;
                            if nat_eq(a25, a26) {
                                f4(v1, v2, v3)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a27 = v0;
                            let a28 = 5u64;
                            if nat_eq(a27, a28) {
                                f5(v1, v2)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        }
                    }
                }
            } else {
                let a29 = v0;
                let a30 = 9u64;
                if nat_lt(a29, a30) {
                    let a31 = v0;
                    let a32 = 7u64;
                    if nat_lt(a31, a32) {
                        let a33 = v0;
                        let a34 = 6u64;
                        if nat_eq(a33, a34) {
                            Ok(f6(v2))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a35 = v0;
                        let a36 = 8u64;
                        if nat_lt(a35, a36) {
                            let a37 = v0;
                            let a38 = 7u64;
                            if nat_eq(a37, a38) {
                                f7(v1, v2, v3, v4)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a39 = v0;
                            let a40 = 8u64;
                            if nat_eq(a39, a40) {
                                f8(v1, v2)
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        }
                    }
                } else {
                    let a41 = v0;
                    let a42 = 10u64;
                    if nat_lt(a41, a42) {
                        let a43 = v0;
                        let a44 = 9u64;
                        if nat_eq(a43, a44) {
                            f9(v1, v2, v3)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a45 = v0;
                        let a46 = 11u64;
                        if nat_lt(a45, a46) {
                            let a47 = v0;
                            let a48 = 10u64;
                            if nat_eq(a47, a48) {
                                let v5: u64 = {
                                    let a49 = v2;
                                    let a50 = 4294967296u64;
                                    let a51 = 0u64;
                                    nat_quot(a49, a50, a51)
                                };
                                let v6: u64 = {
                                    let a52 = v2;
                                    let a53 = 4294967296u64;
                                    let a54 = 0u64;
                                    nat_rem(a52, a53, a54)
                                };
                                let a55 = v5;
                                let a56 = 2u64;
                                if nat_eq(a55, a56) {
                                    Ok(f10(v5, {
                                        let a57 = v6;
                                        let a58 = 256u64;
                                        let a59 = 0u64;
                                        nat_quot(a57, a58, a59)
                                    }, {
                                        let a60 = v6;
                                        let a61 = 256u64;
                                        let a62 = 0u64;
                                        nat_rem(a60, a61, a62)
                                    }))
                                } else {
                                    let a63 = v5;
                                    let a64 = 2u64;
                                    if nat_lt(a63, a64) {
                                        Ok(f10(v5, v6, 0u64))
                                    } else {
                                        Ok(f10(v5, 0u64, v6))
                                    }
                                }
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        } else {
                            let a65 = v0;
                            let a66 = 11u64;
                            if nat_eq(a65, a66) {
                                Ok(f11(f12(v2, v3, v4)?))
                            } else {
                                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                            }
                        }
                    }
                }
            }
        } else {
            f13({
                let a67 = v0;
                let a68 = 16u64;
                nat_sub(a67, a68)
            }, v1, v2, v3, v4)
        }
    } else {
        f14({
            let a69 = v0;
            let a70 = 32u64;
            nat_sub(a69, a70)
        }, v2, v3, v4)
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    let a71 = v1;
    let a72 = 256u64;
    if if if nat_eq(a71, a72) {
        let a73 = v2;
        let a74 = 0u64;
        if nat_eq(a73, a74) {
            true
        } else {
            let a75 = v2;
            let a76 = 256u64;
            nat_eq(a75, a76)
        }
    } else {
        false
    } {
        let a77 = v3;
        let a78 = if v0 {
            98u64
        } else {
            110u64
        };
        nat_eq(a77, a78)
    } else {
        false
    } {
        f15(8u64, 32u64, if v0 {
            15u64
        } else {
            9u64
        }, if v0 {
            841731191u64
        } else {
            0u64
        }, if v0 {
            19241u64
        } else {
            25245u64
        })
    } else {
        (0u64, (0u64, (0u64, (0u64, (0u64, 0u64)))))
    }
}

pub fn f2(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f16({
        let a79 = v0;
        let a80 = 8388608u64;
        let a81 = 0u64;
        nat_rem(a79, a80, a81)
    }, v1, 4194304u64))
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11(f17(v0, v2, 1u64, 8u64, {
        let a82 = v1;
        let a83 = 0u64;
        if nat_eq(a82, a83) {
            1u64
        } else {
            let a84 = v1;
            let a85 = 1u64;
            if nat_eq(a84, a85) {
                8u64
            } else {
                let a86 = v1;
                let a87 = 2u64;
                if nat_eq(a86, a87) {
                    64u64
                } else {
                    let a88 = v1;
                    let a89 = 3u64;
                    if nat_eq(a88, a89) {
                        512u64
                    } else {
                        0u64
                    }
                }
            }
        }
    })?))
}

pub fn f4(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f18({
        let a90 = if v0 {
            0u64
        } else {
            4097u64
        };
        let a91 = v1;
        nat_add(a90, a91)?
    }, {
        let a94 = if v0 {
            15496u64
        } else {
            16520u64
        };
        let a95 = {
            let a92 = 4u64;
            let a93 = v2;
            nat_mul(a92, a93)?
        };
        nat_add(a94, a95)?
    }))
}

pub fn f5(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11({
        let a101 = 1u64;
        let a102 = {
            let a99 = 2u64;
            let a100 = {
                let a96 = v1;
                let a97 = if v0 {
                    8u64
                } else {
                    2u64
                };
                let a98 = 0u64;
                nat_quot(a96, a97, a98)
            };
            nat_mul(a99, a100)?
        };
        nat_add(a101, a102)?
    }))
}

pub fn f6(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f19(v0))
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a103 = v2;
    let a104 = 0u64;
    if nat_eq(a103, a104) {
        Ok(f10({
            let a105 = 2u64;
            let a106 = v1;
            nat_add(a105, a106)?
        }, 0u64, {
            let a107 = v3;
            let a108 = 256u64;
            let a109 = 0u64;
            nat_rem(a107, a108, a109)
        }))
    } else {
        if v0 {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        } else {
            Ok(f10({
                let a113 = 66u64;
                let a114 = {
                    let a110 = v1;
                    let a111 = 8u64;
                    let a112 = 0u64;
                    nat_quot(a110, a111, a112)
                };
                nat_add(a113, a114)?
            }, {
                let a115 = v1;
                let a116 = 8u64;
                let a117 = 0u64;
                nat_rem(a115, a116, a117)
            }, {
                let a121 = {
                    let a118 = v3;
                    let a119 = 256u64;
                    let a120 = 0u64;
                    nat_quot(a118, a119, a120)
                };
                let a122 = 2u64;
                let a123 = 0u64;
                nat_rem(a121, a122, a123)
            }))
        }
    }
}

pub fn f8(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f18({
            let a126 = 69u64;
            let a127 = {
                let a124 = 4u64;
                let a125 = v1;
                nat_mul(a124, a125)?
            };
            nat_add(a126, a127)?
        }, 4u64))
    } else {
        Ok(f18({
            let a131 = 106u64;
            let a132 = {
                let a128 = v1;
                let a129 = 2u64;
                let a130 = 0u64;
                nat_quot(a128, a129, a130)
            };
            nat_add(a131, a132)?
        }, {
            let a136 = 4u64;
            let a137 = {
                let a133 = v1;
                let a134 = 2u64;
                let a135 = 0u64;
                nat_rem(a133, a134, a135)
            };
            nat_mul(a136, a137)?
        }))
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f18({
            let a140 = 66u64;
            let a141 = {
                let a138 = 4u64;
                let a139 = v1;
                nat_mul(a138, a139)?
            };
            nat_add(a140, a141)?
        }, {
            let a142 = 7u64;
            let a143 = v2;
            nat_mul(a142, a143)?
        }))
    } else {
        Ok(f18({
            let a148 = {
                let a146 = 74u64;
                let a147 = {
                    let a144 = 4u64;
                    let a145 = v1;
                    nat_mul(a144, a145)?
                };
                nat_add(a146, a147)?
            };
            let a149 = v2;
            nat_add(a148, a149)?
        }, 0u64))
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (8u64, (v0, (v1, (v2, (0u64, 0u64)))))
}

pub fn f11(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (1u64, (v0, (0u64, (0u64, (0u64, 0u64)))))
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a150 = v0;
    let a151 = 3u64;
    if nat_lt(a150, a151) {
        let a152 = v0;
        let a153 = 1u64;
        if nat_lt(a152, a153) {
            let a154 = v0;
            let a155 = 0u64;
            if nat_eq(a154, a155) {
                let a156 = v1;
                let a157 = v2;
                nat_add(a156, a157)
            } else {
                Ok(0u64)
            }
        } else {
            let a158 = v0;
            let a159 = 2u64;
            if nat_lt(a158, a159) {
                let a160 = v0;
                let a161 = 1u64;
                if nat_eq(a160, a161) {
                    let a162 = v1;
                    let a163 = v2;
                    Ok(nat_sub(a162, a163))
                } else {
                    Ok(0u64)
                }
            } else {
                let a164 = v0;
                let a165 = 2u64;
                if nat_eq(a164, a165) {
                    let a166 = v1;
                    let a167 = v2;
                    nat_mul(a166, a167)
                } else {
                    Ok(0u64)
                }
            }
        }
    } else {
        let a168 = v0;
        let a169 = 5u64;
        if nat_lt(a168, a169) {
            let a170 = v0;
            let a171 = 4u64;
            if nat_lt(a170, a171) {
                let a172 = v0;
                let a173 = 3u64;
                if nat_eq(a172, a173) {
                    let a174 = v1;
                    let a175 = v2;
                    let a176 = 0u64;
                    Ok(nat_quot(a174, a175, a176))
                } else {
                    Ok(0u64)
                }
            } else {
                let a177 = v0;
                let a178 = 4u64;
                if nat_eq(a177, a178) {
                    let a179 = v1;
                    let a180 = v2;
                    let a181 = 0u64;
                    Ok(nat_rem(a179, a180, a181))
                } else {
                    Ok(0u64)
                }
            }
        } else {
            let a182 = v0;
            let a183 = 6u64;
            if nat_lt(a182, a183) {
                let a184 = v0;
                let a185 = 5u64;
                if nat_eq(a184, a185) {
                    let a186 = v1;
                    let a187 = v2;
                    if nat_eq(a186, a187) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a188 = v0;
                let a189 = 6u64;
                if nat_eq(a188, a189) {
                    let a190 = v1;
                    let a191 = v2;
                    if nat_lt(a190, a191) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            }
        }
    }
}

pub fn f13(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a192 = v0;
    let a193 = 4u64;
    if nat_lt(a192, a193) {
        let a194 = v0;
        let a195 = 2u64;
        if nat_lt(a194, a195) {
            let a196 = v0;
            let a197 = 1u64;
            if nat_lt(a196, a197) {
                let a198 = v0;
                let a199 = 0u64;
                if nat_eq(a198, a199) {
                    if v1 {
                        let a200 = v2;
                        let a201 = 256u64;
                        if if if nat_eq(a200, a201) {
                            let a202 = v3;
                            let a203 = 256u64;
                            nat_eq(a202, a203)
                        } else {
                            false
                        } {
                            let a204 = v4;
                            let a205 = 74u64;
                            nat_eq(a204, a205)
                        } else {
                            false
                        } {
                            Ok(f15(16u64, 16u64, 9u64, 646978941u64, 182233u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a206 = v2;
                        let a207 = 256u64;
                        if if if nat_eq(a206, a207) {
                            let a208 = v3;
                            let a209 = 0u64;
                            if nat_eq(a208, a209) {
                                true
                            } else {
                                let a210 = v3;
                                let a211 = 256u64;
                                nat_eq(a210, a211)
                            }
                        } else {
                            false
                        } {
                            let a212 = v4;
                            let a213 = 82u64;
                            nat_eq(a212, a213)
                        } else {
                            false
                        } {
                            Ok(f15(16u64, 16u64, 9u64, 841731191u64, 45740u64))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a214 = v0;
                let a215 = 1u64;
                let _ = nat_eq(a214, a215);
                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
            }
        } else {
            let a216 = v0;
            let a217 = 3u64;
            if nat_lt(a216, a217) {
                let a218 = v0;
                let a219 = 2u64;
                if nat_eq(a218, a219) {
                    Ok(f2(v2, v3))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a220 = v0;
                let a221 = 3u64;
                if nat_eq(a220, a221) {
                    Ok(f11({
                        let a254 = v2;
                        let a255 = {
                            let a252 = v4;
                            let a253 = {
                                let a222 = v3;
                                let a223 = 4u64;
                                if nat_lt(a222, a223) {
                                    let a224 = v3;
                                    let a225 = 2u64;
                                    if nat_lt(a224, a225) {
                                        let a226 = v3;
                                        let a227 = 1u64;
                                        if nat_lt(a226, a227) {
                                            let a228 = v3;
                                            let a229 = 0u64;
                                            if nat_eq(a228, a229) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a230 = v3;
                                            let a231 = 1u64;
                                            if nat_eq(a230, a231) {
                                                3u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a232 = v3;
                                        let a233 = 3u64;
                                        if nat_lt(a232, a233) {
                                            let a234 = v3;
                                            let a235 = 2u64;
                                            if nat_eq(a234, a235) {
                                                9u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a236 = v3;
                                            let a237 = 3u64;
                                            if nat_eq(a236, a237) {
                                                27u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                } else {
                                    let a238 = v3;
                                    let a239 = 6u64;
                                    if nat_lt(a238, a239) {
                                        let a240 = v3;
                                        let a241 = 5u64;
                                        if nat_lt(a240, a241) {
                                            let a242 = v3;
                                            let a243 = 4u64;
                                            if nat_eq(a242, a243) {
                                                81u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a244 = v3;
                                            let a245 = 5u64;
                                            if nat_eq(a244, a245) {
                                                243u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a246 = v3;
                                        let a247 = 7u64;
                                        if nat_lt(a246, a247) {
                                            let a248 = v3;
                                            let a249 = 6u64;
                                            if nat_eq(a248, a249) {
                                                729u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a250 = v3;
                                            let a251 = 7u64;
                                            if nat_eq(a250, a251) {
                                                2187u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                }
                            };
                            nat_mul(a252, a253)?
                        };
                        nat_add(a254, a255)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        }
    } else {
        let a256 = v0;
        let a257 = 6u64;
        if nat_lt(a256, a257) {
            let a258 = v0;
            let a259 = 5u64;
            if nat_lt(a258, a259) {
                let a260 = v0;
                let a261 = 4u64;
                if nat_eq(a260, a261) {
                    Ok(f18(v2, {
                        let a264 = if v1 {
                            3208u64
                        } else {
                            7304u64
                        };
                        let a265 = {
                            let a262 = v3;
                            let a263 = 8u64;
                            nat_mul(a262, a263)?
                        };
                        nat_add(a264, a265)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a266 = v0;
                let a267 = 5u64;
                if nat_eq(a266, a267) {
                    Ok(f11({
                        let a273 = 1u64;
                        let a274 = {
                            let a271 = 2u64;
                            let a272 = {
                                let a268 = v2;
                                let a269 = 17u64;
                                let a270 = 0u64;
                                nat_quot(a268, a269, a270)
                            };
                            nat_mul(a271, a272)?
                        };
                        nat_add(a273, a274)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        } else {
            let a275 = v0;
            let a276 = 7u64;
            if nat_lt(a275, a276) {
                let a277 = v0;
                let a278 = 6u64;
                if nat_eq(a277, a278) {
                    Ok(f6(v2))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a279 = v0;
                let a280 = 8u64;
                if nat_lt(a279, a280) {
                    let a281 = v0;
                    let a282 = 7u64;
                    if nat_eq(a281, a282) {
                        if v1 {
                            let a283 = v3;
                            let a284 = 0u64;
                            if nat_eq(a283, a284) {
                                Ok(f10({
                                    let a287 = 2u64;
                                    let a288 = {
                                        let a285 = v2;
                                        let a286 = 2u64;
                                        nat_mul(a285, a286)?
                                    };
                                    nat_add(a287, a288)?
                                }, 0u64, {
                                    let a289 = v4;
                                    let a290 = 256u64;
                                    let a291 = 0u64;
                                    nat_rem(a289, a290, a291)
                                }))
                            } else {
                                Ok(f10({
                                    let a294 = 3u64;
                                    let a295 = {
                                        let a292 = v2;
                                        let a293 = 2u64;
                                        nat_mul(a292, a293)?
                                    };
                                    nat_add(a294, a295)?
                                }, 0u64, {
                                    let a296 = v4;
                                    let a297 = 256u64;
                                    let a298 = 0u64;
                                    nat_quot(a296, a297, a298)
                                }))
                            }
                        } else {
                            let a299 = v3;
                            let a300 = 0u64;
                            if nat_eq(a299, a300) {
                                Ok(f10({
                                    let a301 = 2u64;
                                    let a302 = v2;
                                    nat_add(a301, a302)?
                                }, 0u64, {
                                    let a303 = v4;
                                    let a304 = 256u64;
                                    let a305 = 0u64;
                                    nat_rem(a303, a304, a305)
                                }))
                            } else {
                                Ok(f10({
                                    let a309 = 66u64;
                                    let a310 = {
                                        let a306 = v2;
                                        let a307 = 4u64;
                                        let a308 = 0u64;
                                        nat_quot(a306, a307, a308)
                                    };
                                    nat_add(a309, a310)?
                                }, {
                                    let a314 = 2u64;
                                    let a315 = {
                                        let a311 = v2;
                                        let a312 = 4u64;
                                        let a313 = 0u64;
                                        nat_rem(a311, a312, a313)
                                    };
                                    nat_mul(a314, a315)?
                                }, {
                                    let a316 = v4;
                                    let a317 = 256u64;
                                    let a318 = 0u64;
                                    nat_quot(a316, a317, a318)
                                }))
                            }
                        }
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a319 = v0;
                    let a320 = 8u64;
                    if nat_eq(a319, a320) {
                        Ok(f18({
                            let a324 = if v1 {
                                66u64
                            } else {
                                74u64
                            };
                            let a325 = {
                                let a321 = v2;
                                let a322 = 2u64;
                                let a323 = 0u64;
                                nat_quot(a321, a322, a323)
                            };
                            nat_add(a324, a325)?
                        }, {
                            let a329 = 4u64;
                            let a330 = {
                                let a326 = v2;
                                let a327 = 2u64;
                                let a328 = 0u64;
                                nat_rem(a326, a327, a328)
                            };
                            nat_mul(a329, a330)?
                        }))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a331 = v0;
    let a332 = 0u64;
    if nat_eq(a331, a332) {
        let a333 = v1;
        let a334 = 256u64;
        if if if nat_eq(a333, a334) {
            let a335 = v2;
            let a336 = 256u64;
            nat_eq(a335, a336)
        } else {
            false
        } {
            let a337 = v3;
            let a338 = 66u64;
            nat_eq(a337, a338)
        } else {
            false
        } {
            Ok(f15(8u64, 32u64, 6u64, 646978941u64, 113413u64))
        } else {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        }
    } else {
        let a339 = v0;
        let a340 = 4u64;
        if nat_eq(a339, a340) {
            Ok(f18(v1, {
                let a343 = 1160u64;
                let a344 = {
                    let a341 = v2;
                    let a342 = 8u64;
                    nat_mul(a341, a342)?
                };
                nat_add(a343, a344)?
            }))
        } else {
            let a345 = v0;
            let a346 = 7u64;
            if nat_eq(a345, a346) {
                Ok(f10({
                    let a351 = {
                        let a349 = 2u64;
                        let a350 = {
                            let a347 = v1;
                            let a348 = 8u64;
                            nat_mul(a347, a348)?
                        };
                        nat_add(a349, a350)?
                    };
                    let a352 = v2;
                    nat_add(a351, a352)?
                }, 0u64, v3))
            } else {
                let a353 = v0;
                let a354 = 8u64;
                if nat_eq(a353, a354) {
                    Ok(f18({
                        let a357 = 9u64;
                        let a358 = {
                            let a355 = v1;
                            let a356 = 8u64;
                            nat_mul(a355, a356)?
                        };
                        nat_add(a357, a358)?
                    }, 4u64))
                } else {
                    let a359 = v0;
                    let a360 = 9u64;
                    if nat_eq(a359, a360) {
                        Ok(f18({
                            let a363 = 6u64;
                            let a364 = {
                                let a361 = v1;
                                let a362 = 8u64;
                                nat_mul(a361, a362)?
                            };
                            nat_add(a363, a364)?
                        }, {
                            let a365 = v2;
                            let a366 = 7u64;
                            nat_mul(a365, a366)?
                        }))
                    } else {
                        f13(v0, true, v1, v2, v3)
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (6u64, (v0, (v1, (v2, (v3, v4)))))
}

pub fn f16(v0: u64, v1: u64, v2: u64) -> u64 {
    let v3: u64 = {
        let a367 = v0;
        let a368 = v2;
        nat_sub(a367, a368)
    };
    let a369 = v3;
    let a370 = v1;
    if nat_lt(a369, a370) {
        v3
    } else {
        v1
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a379 = v0;
    let a380 = {
        let a377 = {
            let a374 = {
                let a371 = v1;
                let a372 = v2;
                let a373 = 0u64;
                nat_quot(a371, a372, a373)
            };
            let a375 = v3;
            let a376 = 0u64;
            nat_rem(a374, a375, a376)
        };
        let a378 = v4;
        nat_mul(a377, a378)?
    };
    nat_add(a379, a380)
}

pub fn f18(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (7u64, (v0, (v1, (0u64, (0u64, 0u64)))))
}

pub fn f19(v0: u64) -> u64 {
    let a381 = v0;
    let a382 = 0u64;
    if nat_eq(a381, a382) {
        1u64
    } else {
        let a383 = v0;
        let a384 = 1u64;
        if nat_eq(a383, a384) {
            2u64
        } else {
            let a385 = v0;
            let a386 = 2u64;
            if nat_eq(a385, a386) {
                4u64
            } else {
                let a387 = v0;
                let a388 = 3u64;
                if nat_eq(a387, a388) {
                    8u64
                } else {
                    let a389 = v0;
                    let a390 = 4u64;
                    if nat_eq(a389, a390) {
                        16u64
                    } else {
                        let a391 = v0;
                        let a392 = 5u64;
                        if nat_eq(a391, a392) {
                            32u64
                        } else {
                            let a393 = v0;
                            let a394 = 6u64;
                            if nat_eq(a393, a394) {
                                64u64
                            } else {
                                let a395 = v0;
                                let a396 = 7u64;
                                if nat_eq(a395, a396) {
                                    128u64
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
