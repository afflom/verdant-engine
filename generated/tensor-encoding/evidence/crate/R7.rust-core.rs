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
    let a2 = 16u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 6u64;
        if nat_lt(a3, a4) {
            let a5 = v0;
            let a6 = 3u64;
            if nat_lt(a5, a6) {
                let a7 = v0;
                let a8 = 1u64;
                if nat_lt(a7, a8) {
                    let a9 = v0;
                    let a10 = 0u64;
                    if nat_eq(a9, a10) {
                        Ok(f1(v1, v2, v3, v4))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a11 = v0;
                    let a12 = 2u64;
                    if nat_lt(a11, a12) {
                        let a13 = v0;
                        let a14 = 1u64;
                        let _ = nat_eq(a13, a14);
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    } else {
                        let a15 = v0;
                        let a16 = 2u64;
                        if nat_eq(a15, a16) {
                            Ok(f2(v2, v3))
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                }
            } else {
                let a17 = v0;
                let a18 = 4u64;
                if nat_lt(a17, a18) {
                    let a19 = v0;
                    let a20 = 3u64;
                    if nat_eq(a19, a20) {
                        f3(v2, v3, v4)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a21 = v0;
                    let a22 = 5u64;
                    if nat_lt(a21, a22) {
                        let a23 = v0;
                        let a24 = 4u64;
                        if nat_eq(a23, a24) {
                            f4(v1, v2, v3)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a25 = v0;
                        let a26 = 5u64;
                        if nat_eq(a25, a26) {
                            f5(v1, v2)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                }
            }
        } else {
            let a27 = v0;
            let a28 = 9u64;
            if nat_lt(a27, a28) {
                let a29 = v0;
                let a30 = 7u64;
                if nat_lt(a29, a30) {
                    let a31 = v0;
                    let a32 = 6u64;
                    if nat_eq(a31, a32) {
                        Ok(f6(v2))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a33 = v0;
                    let a34 = 8u64;
                    if nat_lt(a33, a34) {
                        let a35 = v0;
                        let a36 = 7u64;
                        if nat_eq(a35, a36) {
                            f7(v1, v2, v3, v4)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a37 = v0;
                        let a38 = 8u64;
                        if nat_eq(a37, a38) {
                            f8(v1, v2)
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    }
                }
            } else {
                let a39 = v0;
                let a40 = 10u64;
                if nat_lt(a39, a40) {
                    let a41 = v0;
                    let a42 = 9u64;
                    if nat_eq(a41, a42) {
                        f9(v1, v2, v3)
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a43 = v0;
                    let a44 = 11u64;
                    if nat_lt(a43, a44) {
                        let a45 = v0;
                        let a46 = 10u64;
                        if nat_eq(a45, a46) {
                            let v5: u64 = {
                                let a47 = v2;
                                let a48 = 4294967296u64;
                                let a49 = 0u64;
                                nat_quot(a47, a48, a49)
                            };
                            let v6: u64 = {
                                let a50 = v2;
                                let a51 = 4294967296u64;
                                let a52 = 0u64;
                                nat_rem(a50, a51, a52)
                            };
                            let a53 = v5;
                            let a54 = 2u64;
                            if nat_eq(a53, a54) {
                                Ok(f10(v5, {
                                    let a55 = v6;
                                    let a56 = 256u64;
                                    let a57 = 0u64;
                                    nat_quot(a55, a56, a57)
                                }, {
                                    let a58 = v6;
                                    let a59 = 256u64;
                                    let a60 = 0u64;
                                    nat_rem(a58, a59, a60)
                                }))
                            } else {
                                let a61 = v5;
                                let a62 = 2u64;
                                if nat_lt(a61, a62) {
                                    Ok(f10(v5, v6, 0u64))
                                } else {
                                    Ok(f10(v5, 0u64, v6))
                                }
                            }
                        } else {
                            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                        }
                    } else {
                        let a63 = v0;
                        let a64 = 11u64;
                        if nat_eq(a63, a64) {
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
            let a65 = v0;
            let a66 = 16u64;
            nat_sub(a65, a66)
        }, v2, v3, v4)
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    let a67 = v1;
    let a68 = 256u64;
    if if if nat_eq(a67, a68) {
        let a69 = v2;
        let a70 = 0u64;
        if nat_eq(a69, a70) {
            true
        } else {
            let a71 = v2;
            let a72 = 256u64;
            nat_eq(a71, a72)
        }
    } else {
        false
    } {
        let a73 = v3;
        let a74 = if v0 {
            98u64
        } else {
            110u64
        };
        nat_eq(a73, a74)
    } else {
        false
    } {
        f14(8u64, 32u64, if v0 {
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
    f11(f15({
        let a75 = v0;
        let a76 = 8388608u64;
        let a77 = 0u64;
        nat_rem(a75, a76, a77)
    }, v1, 4194304u64))
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11(f16(v0, v2, 1u64, 8u64, {
        let a78 = v1;
        let a79 = 0u64;
        if nat_eq(a78, a79) {
            1u64
        } else {
            let a80 = v1;
            let a81 = 1u64;
            if nat_eq(a80, a81) {
                8u64
            } else {
                let a82 = v1;
                let a83 = 2u64;
                if nat_eq(a82, a83) {
                    64u64
                } else {
                    let a84 = v1;
                    let a85 = 3u64;
                    if nat_eq(a84, a85) {
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
    Ok(f17({
        let a86 = if v0 {
            0u64
        } else {
            4097u64
        };
        let a87 = v1;
        nat_add(a86, a87)?
    }, {
        let a90 = if v0 {
            15496u64
        } else {
            16520u64
        };
        let a91 = {
            let a88 = 4u64;
            let a89 = v2;
            nat_mul(a88, a89)?
        };
        nat_add(a90, a91)?
    }))
}

pub fn f5(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    Ok(f11({
        let a97 = 1u64;
        let a98 = {
            let a95 = 2u64;
            let a96 = {
                let a92 = v1;
                let a93 = if v0 {
                    8u64
                } else {
                    2u64
                };
                let a94 = 0u64;
                nat_quot(a92, a93, a94)
            };
            nat_mul(a95, a96)?
        };
        nat_add(a97, a98)?
    }))
}

pub fn f6(v0: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    f11(f18(v0))
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    let a99 = v2;
    let a100 = 0u64;
    if nat_eq(a99, a100) {
        Ok(f10({
            let a101 = 2u64;
            let a102 = v1;
            nat_add(a101, a102)?
        }, 0u64, {
            let a103 = v3;
            let a104 = 256u64;
            let a105 = 0u64;
            nat_rem(a103, a104, a105)
        }))
    } else {
        if v0 {
            Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
        } else {
            Ok(f10({
                let a109 = 66u64;
                let a110 = {
                    let a106 = v1;
                    let a107 = 8u64;
                    let a108 = 0u64;
                    nat_quot(a106, a107, a108)
                };
                nat_add(a109, a110)?
            }, {
                let a111 = v1;
                let a112 = 8u64;
                let a113 = 0u64;
                nat_rem(a111, a112, a113)
            }, {
                let a117 = {
                    let a114 = v3;
                    let a115 = 256u64;
                    let a116 = 0u64;
                    nat_quot(a114, a115, a116)
                };
                let a118 = 2u64;
                let a119 = 0u64;
                nat_rem(a117, a118, a119)
            }))
        }
    }
}

pub fn f8(v0: bool, v1: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f17({
            let a122 = 69u64;
            let a123 = {
                let a120 = 4u64;
                let a121 = v1;
                nat_mul(a120, a121)?
            };
            nat_add(a122, a123)?
        }, 4u64))
    } else {
        Ok(f17({
            let a127 = 106u64;
            let a128 = {
                let a124 = v1;
                let a125 = 2u64;
                let a126 = 0u64;
                nat_quot(a124, a125, a126)
            };
            nat_add(a127, a128)?
        }, {
            let a132 = 4u64;
            let a133 = {
                let a129 = v1;
                let a130 = 2u64;
                let a131 = 0u64;
                nat_rem(a129, a130, a131)
            };
            nat_mul(a132, a133)?
        }))
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
    if v0 {
        Ok(f17({
            let a136 = 66u64;
            let a137 = {
                let a134 = 4u64;
                let a135 = v1;
                nat_mul(a134, a135)?
            };
            nat_add(a136, a137)?
        }, {
            let a138 = 7u64;
            let a139 = v2;
            nat_mul(a138, a139)?
        }))
    } else {
        Ok(f17({
            let a144 = {
                let a142 = 74u64;
                let a143 = {
                    let a140 = 4u64;
                    let a141 = v1;
                    nat_mul(a140, a141)?
                };
                nat_add(a142, a143)?
            };
            let a145 = v2;
            nat_add(a144, a145)?
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
    let a146 = v0;
    let a147 = 3u64;
    if nat_lt(a146, a147) {
        let a148 = v0;
        let a149 = 1u64;
        if nat_lt(a148, a149) {
            let a150 = v0;
            let a151 = 0u64;
            if nat_eq(a150, a151) {
                let a152 = v1;
                let a153 = v2;
                nat_add(a152, a153)
            } else {
                Ok(0u64)
            }
        } else {
            let a154 = v0;
            let a155 = 2u64;
            if nat_lt(a154, a155) {
                let a156 = v0;
                let a157 = 1u64;
                if nat_eq(a156, a157) {
                    let a158 = v1;
                    let a159 = v2;
                    Ok(nat_sub(a158, a159))
                } else {
                    Ok(0u64)
                }
            } else {
                let a160 = v0;
                let a161 = 2u64;
                if nat_eq(a160, a161) {
                    let a162 = v1;
                    let a163 = v2;
                    nat_mul(a162, a163)
                } else {
                    Ok(0u64)
                }
            }
        }
    } else {
        let a164 = v0;
        let a165 = 5u64;
        if nat_lt(a164, a165) {
            let a166 = v0;
            let a167 = 4u64;
            if nat_lt(a166, a167) {
                let a168 = v0;
                let a169 = 3u64;
                if nat_eq(a168, a169) {
                    let a170 = v1;
                    let a171 = v2;
                    let a172 = 0u64;
                    Ok(nat_quot(a170, a171, a172))
                } else {
                    Ok(0u64)
                }
            } else {
                let a173 = v0;
                let a174 = 4u64;
                if nat_eq(a173, a174) {
                    let a175 = v1;
                    let a176 = v2;
                    let a177 = 0u64;
                    Ok(nat_rem(a175, a176, a177))
                } else {
                    Ok(0u64)
                }
            }
        } else {
            let a178 = v0;
            let a179 = 6u64;
            if nat_lt(a178, a179) {
                let a180 = v0;
                let a181 = 5u64;
                if nat_eq(a180, a181) {
                    let a182 = v1;
                    let a183 = v2;
                    if nat_eq(a182, a183) {
                        Ok(1u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(0u64)
                }
            } else {
                let a184 = v0;
                let a185 = 6u64;
                if nat_eq(a184, a185) {
                    let a186 = v1;
                    let a187 = v2;
                    if nat_lt(a186, a187) {
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

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> R<(u64, (u64, (u64, (u64, (u64, u64)))))> {
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
                    let a196 = v1;
                    let a197 = 256u64;
                    if if if nat_eq(a196, a197) {
                        let a198 = v2;
                        let a199 = 0u64;
                        if nat_eq(a198, a199) {
                            true
                        } else {
                            let a200 = v2;
                            let a201 = 256u64;
                            nat_eq(a200, a201)
                        }
                    } else {
                        false
                    } {
                        let a202 = v3;
                        let a203 = 82u64;
                        nat_eq(a202, a203)
                    } else {
                        false
                    } {
                        Ok(f14(16u64, 16u64, 9u64, 841731191u64, 45740u64))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a204 = v0;
                let a205 = 1u64;
                let _ = nat_eq(a204, a205);
                Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
            }
        } else {
            let a206 = v0;
            let a207 = 3u64;
            if nat_lt(a206, a207) {
                let a208 = v0;
                let a209 = 2u64;
                if nat_eq(a208, a209) {
                    Ok(f2(v1, v2))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a210 = v0;
                let a211 = 3u64;
                if nat_eq(a210, a211) {
                    Ok(f11({
                        let a244 = v1;
                        let a245 = {
                            let a242 = v3;
                            let a243 = {
                                let a212 = v2;
                                let a213 = 4u64;
                                if nat_lt(a212, a213) {
                                    let a214 = v2;
                                    let a215 = 2u64;
                                    if nat_lt(a214, a215) {
                                        let a216 = v2;
                                        let a217 = 1u64;
                                        if nat_lt(a216, a217) {
                                            let a218 = v2;
                                            let a219 = 0u64;
                                            if nat_eq(a218, a219) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a220 = v2;
                                            let a221 = 1u64;
                                            if nat_eq(a220, a221) {
                                                3u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a222 = v2;
                                        let a223 = 3u64;
                                        if nat_lt(a222, a223) {
                                            let a224 = v2;
                                            let a225 = 2u64;
                                            if nat_eq(a224, a225) {
                                                9u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a226 = v2;
                                            let a227 = 3u64;
                                            if nat_eq(a226, a227) {
                                                27u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                } else {
                                    let a228 = v2;
                                    let a229 = 6u64;
                                    if nat_lt(a228, a229) {
                                        let a230 = v2;
                                        let a231 = 5u64;
                                        if nat_lt(a230, a231) {
                                            let a232 = v2;
                                            let a233 = 4u64;
                                            if nat_eq(a232, a233) {
                                                81u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a234 = v2;
                                            let a235 = 5u64;
                                            if nat_eq(a234, a235) {
                                                243u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    } else {
                                        let a236 = v2;
                                        let a237 = 7u64;
                                        if nat_lt(a236, a237) {
                                            let a238 = v2;
                                            let a239 = 6u64;
                                            if nat_eq(a238, a239) {
                                                729u64
                                            } else {
                                                0u64
                                            }
                                        } else {
                                            let a240 = v2;
                                            let a241 = 7u64;
                                            if nat_eq(a240, a241) {
                                                2187u64
                                            } else {
                                                0u64
                                            }
                                        }
                                    }
                                }
                            };
                            nat_mul(a242, a243)?
                        };
                        nat_add(a244, a245)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        }
    } else {
        let a246 = v0;
        let a247 = 6u64;
        if nat_lt(a246, a247) {
            let a248 = v0;
            let a249 = 5u64;
            if nat_lt(a248, a249) {
                let a250 = v0;
                let a251 = 4u64;
                if nat_eq(a250, a251) {
                    Ok(f17(v1, {
                        let a254 = 7304u64;
                        let a255 = {
                            let a252 = v2;
                            let a253 = 8u64;
                            nat_mul(a252, a253)?
                        };
                        nat_add(a254, a255)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a256 = v0;
                let a257 = 5u64;
                if nat_eq(a256, a257) {
                    Ok(f11({
                        let a263 = 1u64;
                        let a264 = {
                            let a261 = 2u64;
                            let a262 = {
                                let a258 = v1;
                                let a259 = 17u64;
                                let a260 = 0u64;
                                nat_quot(a258, a259, a260)
                            };
                            nat_mul(a261, a262)?
                        };
                        nat_add(a263, a264)?
                    }))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            }
        } else {
            let a265 = v0;
            let a266 = 7u64;
            if nat_lt(a265, a266) {
                let a267 = v0;
                let a268 = 6u64;
                if nat_eq(a267, a268) {
                    Ok(f6(v1))
                } else {
                    Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                }
            } else {
                let a269 = v0;
                let a270 = 8u64;
                if nat_lt(a269, a270) {
                    let a271 = v0;
                    let a272 = 7u64;
                    if nat_eq(a271, a272) {
                        let a273 = v2;
                        let a274 = 0u64;
                        if nat_eq(a273, a274) {
                            Ok(f10({
                                let a275 = 2u64;
                                let a276 = v1;
                                nat_add(a275, a276)?
                            }, 0u64, {
                                let a277 = v3;
                                let a278 = 256u64;
                                let a279 = 0u64;
                                nat_rem(a277, a278, a279)
                            }))
                        } else {
                            Ok(f10({
                                let a283 = 66u64;
                                let a284 = {
                                    let a280 = v1;
                                    let a281 = 4u64;
                                    let a282 = 0u64;
                                    nat_quot(a280, a281, a282)
                                };
                                nat_add(a283, a284)?
                            }, {
                                let a288 = 2u64;
                                let a289 = {
                                    let a285 = v1;
                                    let a286 = 4u64;
                                    let a287 = 0u64;
                                    nat_rem(a285, a286, a287)
                                };
                                nat_mul(a288, a289)?
                            }, {
                                let a290 = v3;
                                let a291 = 256u64;
                                let a292 = 0u64;
                                nat_quot(a290, a291, a292)
                            }))
                        }
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                } else {
                    let a293 = v0;
                    let a294 = 8u64;
                    if nat_eq(a293, a294) {
                        Ok(f17({
                            let a298 = 74u64;
                            let a299 = {
                                let a295 = v1;
                                let a296 = 2u64;
                                let a297 = 0u64;
                                nat_quot(a295, a296, a297)
                            };
                            nat_add(a298, a299)?
                        }, {
                            let a303 = 4u64;
                            let a304 = {
                                let a300 = v1;
                                let a301 = 2u64;
                                let a302 = 0u64;
                                nat_rem(a300, a301, a302)
                            };
                            nat_mul(a303, a304)?
                        }))
                    } else {
                        Ok((0u64, (0u64, (0u64, (0u64, (0u64, 0u64))))))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (6u64, (v0, (v1, (v2, (v3, v4)))))
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> u64 {
    let v3: u64 = {
        let a305 = v0;
        let a306 = v2;
        nat_sub(a305, a306)
    };
    let a307 = v3;
    let a308 = v1;
    if nat_lt(a307, a308) {
        v3
    } else {
        v1
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a317 = v0;
    let a318 = {
        let a315 = {
            let a312 = {
                let a309 = v1;
                let a310 = v2;
                let a311 = 0u64;
                nat_quot(a309, a310, a311)
            };
            let a313 = v3;
            let a314 = 0u64;
            nat_rem(a312, a313, a314)
        };
        let a316 = v4;
        nat_mul(a315, a316)?
    };
    nat_add(a317, a318)
}

pub fn f17(v0: u64, v1: u64) -> (u64, (u64, (u64, (u64, (u64, u64))))) {
    (7u64, (v0, (v1, (0u64, (0u64, 0u64)))))
}

pub fn f18(v0: u64) -> u64 {
    let a319 = v0;
    let a320 = 0u64;
    if nat_eq(a319, a320) {
        1u64
    } else {
        let a321 = v0;
        let a322 = 1u64;
        if nat_eq(a321, a322) {
            2u64
        } else {
            let a323 = v0;
            let a324 = 2u64;
            if nat_eq(a323, a324) {
                4u64
            } else {
                let a325 = v0;
                let a326 = 3u64;
                if nat_eq(a325, a326) {
                    8u64
                } else {
                    let a327 = v0;
                    let a328 = 4u64;
                    if nat_eq(a327, a328) {
                        16u64
                    } else {
                        let a329 = v0;
                        let a330 = 5u64;
                        if nat_eq(a329, a330) {
                            32u64
                        } else {
                            let a331 = v0;
                            let a332 = 6u64;
                            if nat_eq(a331, a332) {
                                64u64
                            } else {
                                let a333 = v0;
                                let a334 = 7u64;
                                if nat_eq(a333, a334) {
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
