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

#[derive(Clone)]
pub enum Adt0 {
    C0(u64, u64, u64, u64),
    C1(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(u64, u64, u64, u64),
    C2(u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4) => {
            f1(v1, v2, v3, v4)
        }
        Adt0::C1(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) => {
            f2(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
        }
        Adt0::C2(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) => {
            f3(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
        }
        Adt0::C3(v27, v28, v29, v30) => {
            f4(v27, v28, v29, v30)
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let a2 = f5(v0);
    let a3 = 0u64;
    if nat_eq(a2, a3) {
        Ok(Adt1::C0(0u64))
    } else {
        let a4 = v1;
        let a5 = f6(v0);
        if if nat_eq(a4, a5) {
            let a6 = 4u64;
            let a7 = v3;
            if nat_le(a6, a7) {
                let a14 = {
                    let a12 = {
                        let a10 = {
                            let a8 = f5(v0);
                            let a9 = 1u64;
                            nat_sub(a8, a9)
                        };
                        let a11 = v3;
                        nat_mul(a10, a11)?
                    };
                    let a13 = 4u64;
                    nat_add(a12, a13)?
                };
                let a15 = v2;
                nat_le(a14, a15)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt1::C1(f5(v0), f6(v0), f7(v0), f8(v0)))
        } else {
            Ok(Adt1::C0(1u64))
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, _: u64, _: u64, _: u64, _: u64, v8: u64, v9: u64) -> R<Adt1> {
    let a16 = v0;
    let a17 = 2u64;
    if nat_eq(a16, a17) {
        let a18 = v1;
        let a19 = 0u64;
        if nat_eq(a18, a19) {
            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
        } else {
            let a20 = v1;
            let a21 = 1u64;
            if nat_eq(a20, a21) {
                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
            } else {
                Ok(Adt1::C0(0u64))
            }
        }
    } else {
        let a22 = v0;
        let a23 = 3u64;
        if nat_eq(a22, a23) {
            let a24 = v1;
            let a25 = 0u64;
            if nat_eq(a24, a25) {
                Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
            } else {
                let a26 = v1;
                let a27 = 1u64;
                if nat_eq(a26, a27) {
                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                } else {
                    let a28 = v1;
                    let a29 = 2u64;
                    if nat_eq(a28, a29) {
                        Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                    } else {
                        let a30 = v1;
                        let a31 = 3u64;
                        if nat_eq(a30, a31) {
                            Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                        } else {
                            Ok(Adt1::C0(0u64))
                        }
                    }
                }
            }
        } else {
            let a32 = v0;
            let a33 = 6u64;
            if nat_eq(a32, a33) {
                let a34 = v1;
                let a35 = 0u64;
                if nat_eq(a34, a35) {
                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                } else {
                    let a36 = v1;
                    let a37 = 1u64;
                    if nat_eq(a36, a37) {
                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                    } else {
                        Ok(Adt1::C0(0u64))
                    }
                }
            } else {
                let a38 = v0;
                let a39 = 7u64;
                if nat_eq(a38, a39) {
                    let a40 = v1;
                    let a41 = 0u64;
                    if nat_eq(a40, a41) {
                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                    } else {
                        let a42 = v1;
                        let a43 = 1u64;
                        if nat_eq(a42, a43) {
                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                        } else {
                            let a44 = v1;
                            let a45 = 2u64;
                            if nat_eq(a44, a45) {
                                Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                            } else {
                                let a46 = v1;
                                let a47 = 3u64;
                                if nat_eq(a46, a47) {
                                    Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                } else {
                                    Ok(Adt1::C0(0u64))
                                }
                            }
                        }
                    }
                } else {
                    if f9(v0) {
                        let a48 = v1;
                        let a49 = 0u64;
                        if nat_eq(a48, a49) {
                            Ok(Adt1::C2(0u64, {
                                let a50 = v0;
                                let a51 = 15u64;
                                if nat_eq(a50, a51) {
                                    4u64
                                } else {
                                    2u64
                                }
                            }, 0u64, 0u64))
                        } else {
                            let a52 = v1;
                            let a53 = 1u64;
                            if if nat_eq(a52, a53) {
                                let a54 = v0;
                                let a55 = 15u64;
                                !nat_eq(a54, a55)
                            } else {
                                false
                            } {
                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                            } else {
                                Ok(Adt1::C0(0u64))
                            }
                        }
                    } else {
                        let a56 = v0;
                        let a57 = 10u64;
                        if nat_eq(a56, a57) {
                            let a58 = v1;
                            let a59 = 0u64;
                            if nat_eq(a58, a59) {
                                Ok(Adt1::C2(0u64, 2u64, 80u64, 0u64))
                            } else {
                                let a60 = v1;
                                let a61 = 1u64;
                                if nat_eq(a60, a61) {
                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                } else {
                                    let a62 = v1;
                                    let a63 = 2u64;
                                    if nat_eq(a62, a63) {
                                        Ok(Adt1::C2(0u64, 2u64, 82u64, 1u64))
                                    } else {
                                        let a64 = v1;
                                        let a65 = 3u64;
                                        if nat_eq(a64, a65) {
                                            Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                        } else {
                                            Ok(Adt1::C0(0u64))
                                        }
                                    }
                                }
                            }
                        } else {
                            let a66 = v0;
                            let a67 = 11u64;
                            if nat_eq(a66, a67) {
                                let a68 = v1;
                                let a69 = 0u64;
                                if nat_eq(a68, a69) {
                                    Ok(Adt1::C2(0u64, 2u64, 108u64, 0u64))
                                } else {
                                    let a70 = v1;
                                    let a71 = 1u64;
                                    if nat_eq(a70, a71) {
                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                    } else {
                                        Ok(Adt1::C0(0u64))
                                    }
                                }
                            } else {
                                let a72 = v0;
                                let a73 = 12u64;
                                if nat_eq(a72, a73) {
                                    let a74 = v1;
                                    let a75 = 0u64;
                                    if nat_eq(a74, a75) {
                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                    } else {
                                        let a76 = v1;
                                        let a77 = 1u64;
                                        if nat_eq(a76, a77) {
                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                        } else {
                                            let a78 = v1;
                                            let a79 = 2u64;
                                            if nat_eq(a78, a79) {
                                                Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                                            } else {
                                                let a80 = v1;
                                                let a81 = 3u64;
                                                if nat_eq(a80, a81) {
                                                    Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                                } else {
                                                    Ok(Adt1::C0(0u64))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let a82 = v0;
                                    let a83 = 13u64;
                                    if nat_eq(a82, a83) {
                                        let a84 = v1;
                                        let a85 = 0u64;
                                        if nat_eq(a84, a85) {
                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                        } else {
                                            let a86 = v1;
                                            let a87 = 1u64;
                                            if nat_eq(a86, a87) {
                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                            } else {
                                                let a88 = v1;
                                                let a89 = 2u64;
                                                if nat_eq(a88, a89) {
                                                    Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                                                } else {
                                                    let a90 = v1;
                                                    let a91 = 3u64;
                                                    if nat_eq(a90, a91) {
                                                        Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let a92 = v0;
                                        let a93 = 14u64;
                                        if nat_eq(a92, a93) {
                                            let a94 = v1;
                                            let a95 = 0u64;
                                            if nat_eq(a94, a95) {
                                                Ok(Adt1::C2(0u64, 2u64, 208u64, 0u64))
                                            } else {
                                                let a96 = v1;
                                                let a97 = 1u64;
                                                if nat_eq(a96, a97) {
                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                } else {
                                                    Ok(Adt1::C0(0u64))
                                                }
                                            }
                                        } else {
                                            let a98 = v0;
                                            let a99 = 16u64;
                                            if nat_eq(a98, a99) {
                                                let a100 = v1;
                                                let a101 = 0u64;
                                                if nat_eq(a100, a101) {
                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                } else {
                                                    let a102 = v1;
                                                    let a103 = 1u64;
                                                    if nat_eq(a102, a103) {
                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            } else {
                                                let a104 = v0;
                                                let a105 = 17u64;
                                                if nat_eq(a104, a105) {
                                                    let a106 = v1;
                                                    let a107 = 0u64;
                                                    if nat_eq(a106, a107) {
                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                    } else {
                                                        let a108 = v1;
                                                        let a109 = 1u64;
                                                        if nat_eq(a108, a109) {
                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                        } else {
                                                            Ok(Adt1::C0(0u64))
                                                        }
                                                    }
                                                } else {
                                                    let a110 = v0;
                                                    let a111 = 18u64;
                                                    if nat_eq(a110, a111) {
                                                        let a112 = v1;
                                                        let a113 = 0u64;
                                                        if nat_eq(a112, a113) {
                                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                        } else {
                                                            let a114 = v1;
                                                            let a115 = 1u64;
                                                            if nat_eq(a114, a115) {
                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                            } else {
                                                                Ok(Adt1::C0(0u64))
                                                            }
                                                        }
                                                    } else {
                                                        let a116 = v0;
                                                        let a117 = 19u64;
                                                        if nat_eq(a116, a117) {
                                                            let a118 = v1;
                                                            let a119 = 0u64;
                                                            if nat_eq(a118, a119) {
                                                                Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                            } else {
                                                                let a120 = v1;
                                                                let a121 = 1u64;
                                                                if nat_eq(a120, a121) {
                                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                } else {
                                                                    Ok(Adt1::C0(0u64))
                                                                }
                                                            }
                                                        } else {
                                                            let a122 = v0;
                                                            let a123 = 20u64;
                                                            if nat_eq(a122, a123) {
                                                                let a124 = v1;
                                                                let a125 = 0u64;
                                                                if nat_eq(a124, a125) {
                                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                } else {
                                                                    let a126 = v1;
                                                                    let a127 = 1u64;
                                                                    if nat_eq(a126, a127) {
                                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                    } else {
                                                                        Ok(Adt1::C0(0u64))
                                                                    }
                                                                }
                                                            } else {
                                                                let a128 = v0;
                                                                let a129 = 21u64;
                                                                if nat_eq(a128, a129) {
                                                                    let a130 = v1;
                                                                    let a131 = 0u64;
                                                                    if nat_eq(a130, a131) {
                                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                    } else {
                                                                        let a132 = v1;
                                                                        let a133 = 1u64;
                                                                        if nat_eq(a132, a133) {
                                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                        } else {
                                                                            Ok(Adt1::C0(0u64))
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a134 = v0;
                                                                    let a135 = 22u64;
                                                                    if nat_eq(a134, a135) {
                                                                        let a136 = v1;
                                                                        let a137 = 0u64;
                                                                        if nat_eq(a136, a137) {
                                                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                        } else {
                                                                            let a138 = v1;
                                                                            let a139 = 1u64;
                                                                            if nat_eq(a138, a139) {
                                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                            } else {
                                                                                Ok(Adt1::C0(0u64))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let a140 = v0;
                                                                        let a141 = 23u64;
                                                                        if nat_eq(a140, a141) {
                                                                            let a142 = v1;
                                                                            let a143 = 0u64;
                                                                            if nat_eq(a142, a143) {
                                                                                Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                            } else {
                                                                                let a144 = v1;
                                                                                let a145 = 1u64;
                                                                                if nat_eq(a144, a145) {
                                                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                } else {
                                                                                    Ok(Adt1::C0(0u64))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            let a146 = v0;
                                                                            let a147 = 29u64;
                                                                            if nat_eq(a146, a147) {
                                                                                let a148 = v1;
                                                                                let a149 = 0u64;
                                                                                if nat_eq(a148, a149) {
                                                                                    Ok(Adt1::C2(0u64, 4u64, 48u64, 6u64))
                                                                                } else {
                                                                                    let a150 = v1;
                                                                                    let a151 = 1u64;
                                                                                    if nat_eq(a150, a151) {
                                                                                        Ok(Adt1::C2(0u64, 4u64, 52u64, 7u64))
                                                                                    } else {
                                                                                        let a152 = v1;
                                                                                        let a153 = 2u64;
                                                                                        if nat_eq(a152, a153) {
                                                                                            Ok(Adt1::C3(10u64, 1u64, {
                                                                                                let a188 = {
                                                                                                    let a168 = {
                                                                                                        let a157 = {
                                                                                                            let a154 = v8;
                                                                                                            let a155 = 4096u64;
                                                                                                            let a156 = 0u64;
                                                                                                            nat_quot(a154, a155, a156)
                                                                                                        };
                                                                                                        let a158 = 16u64;
                                                                                                        let a159 = 0u64;
                                                                                                        nat_rem(a157, a158, a159)
                                                                                                    };
                                                                                                    let a169 = {
                                                                                                        let a166 = {
                                                                                                            let a163 = {
                                                                                                                let a160 = v8;
                                                                                                                let a161 = 268435456u64;
                                                                                                                let a162 = 0u64;
                                                                                                                nat_quot(a160, a161, a162)
                                                                                                            };
                                                                                                            let a164 = 16u64;
                                                                                                            let a165 = 0u64;
                                                                                                            nat_rem(a163, a164, a165)
                                                                                                        };
                                                                                                        let a167 = 16u64;
                                                                                                        nat_mul(a166, a167)?
                                                                                                    };
                                                                                                    nat_add(a168, a169)?
                                                                                                };
                                                                                                let a189 = {
                                                                                                    let a186 = {
                                                                                                        let a176 = {
                                                                                                            let a173 = {
                                                                                                                let a170 = v9;
                                                                                                                let a171 = 4096u64;
                                                                                                                let a172 = 0u64;
                                                                                                                nat_quot(a170, a171, a172)
                                                                                                            };
                                                                                                            let a174 = 16u64;
                                                                                                            let a175 = 0u64;
                                                                                                            nat_rem(a173, a174, a175)
                                                                                                        };
                                                                                                        let a177 = 256u64;
                                                                                                        nat_mul(a176, a177)?
                                                                                                    };
                                                                                                    let a187 = {
                                                                                                        let a184 = {
                                                                                                            let a181 = {
                                                                                                                let a178 = v9;
                                                                                                                let a179 = 268435456u64;
                                                                                                                let a180 = 0u64;
                                                                                                                nat_quot(a178, a179, a180)
                                                                                                            };
                                                                                                            let a182 = 16u64;
                                                                                                            let a183 = 0u64;
                                                                                                            nat_rem(a181, a182, a183)
                                                                                                        };
                                                                                                        let a185 = 4096u64;
                                                                                                        nat_mul(a184, a185)?
                                                                                                    };
                                                                                                    nat_add(a186, a187)?
                                                                                                };
                                                                                                nat_add(a188, a189)?
                                                                                            }, 0u64))
                                                                                        } else {
                                                                                            Ok(Adt1::C0(0u64))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a190 = v0;
                                                                                let a191 = 34u64;
                                                                                if nat_eq(a190, a191) {
                                                                                    let a192 = v1;
                                                                                    let a193 = 0u64;
                                                                                    if nat_eq(a192, a193) {
                                                                                        Ok(Adt1::C2(0u64, 2u64, 52u64, 0u64))
                                                                                    } else {
                                                                                        let a194 = v1;
                                                                                        let a195 = 1u64;
                                                                                        if nat_eq(a194, a195) {
                                                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                        } else {
                                                                                            Ok(Adt1::C0(0u64))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    let a196 = v0;
                                                                                    let a197 = 35u64;
                                                                                    if nat_eq(a196, a197) {
                                                                                        let a198 = v1;
                                                                                        let a199 = 0u64;
                                                                                        if nat_eq(a198, a199) {
                                                                                            Ok(Adt1::C2(0u64, 2u64, 64u64, 0u64))
                                                                                        } else {
                                                                                            let a200 = v1;
                                                                                            let a201 = 1u64;
                                                                                            if nat_eq(a200, a201) {
                                                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                            } else {
                                                                                                Ok(Adt1::C0(0u64))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let a202 = v0;
                                                                                        let a203 = 39u64;
                                                                                        if nat_eq(a202, a203) {
                                                                                            let a204 = v1;
                                                                                            let a205 = 0u64;
                                                                                            if nat_eq(a204, a205) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, 0u64, 0u64))
                                                                                            } else {
                                                                                                let a206 = v1;
                                                                                                let a207 = 1u64;
                                                                                                if nat_eq(a206, a207) {
                                                                                                    Ok(Adt1::C3(9u64, 1u64, v2, 0u64))
                                                                                                } else {
                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            let a208 = v0;
                                                                                            let a209 = 41u64;
                                                                                            if nat_eq(a208, a209) {
                                                                                                let a210 = v1;
                                                                                                let a211 = 0u64;
                                                                                                if nat_eq(a210, a211) {
                                                                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                                                } else {
                                                                                                    let a212 = v1;
                                                                                                    let a213 = 1u64;
                                                                                                    if nat_eq(a212, a213) {
                                                                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C0(0u64))
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                let a214 = v0;
                                                                                                let a215 = 42u64;
                                                                                                if nat_eq(a214, a215) {
                                                                                                    let a216 = v1;
                                                                                                    let a217 = 0u64;
                                                                                                    if nat_eq(a216, a217) {
                                                                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                                                    } else {
                                                                                                        let a218 = v1;
                                                                                                        let a219 = 1u64;
                                                                                                        if nat_eq(a218, a219) {
                                                                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                                        } else {
                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<Adt1> {
    let a220 = v1;
    let a221 = f5(v0);
    if if nat_lt(a220, a221) {
        let a222 = 4u64;
        let a223 = v2;
        nat_le(a222, a223)
    } else {
        false
    } {
        let a224 = v0;
        let a225 = 0u64;
        if nat_eq(a224, a225) {
            let a226 = v3;
            let a227 = 0u64;
            if nat_eq(a226, a227) {
                Ok(Adt1::C2(0u64, 4u64, 0u64, 2u64))
            } else {
                let a228 = v3;
                let a229 = 1u64;
                if nat_eq(a228, a229) {
                    Ok(Adt1::C4({
                        let a230 = v1;
                        let a231 = v2;
                        nat_mul(a230, a231)?
                    }, v6))
                } else {
                    Ok(Adt1::C0(0u64))
                }
            }
        } else {
            let a232 = v0;
            let a233 = 26u64;
            if nat_eq(a232, a233) {
                let a234 = v3;
                let a235 = 0u64;
                if nat_eq(a234, a235) {
                    Ok(Adt1::C2(0u64, 4u64, 0u64, 2u64))
                } else {
                    let a236 = v3;
                    let a237 = 1u64;
                    if nat_eq(a236, a237) {
                        Ok(Adt1::C4({
                            let a238 = v1;
                            let a239 = v2;
                            nat_mul(a238, a239)?
                        }, v6))
                    } else {
                        Ok(Adt1::C0(0u64))
                    }
                }
            } else {
                let a240 = v0;
                let a241 = 1u64;
                if nat_eq(a240, a241) {
                    let a242 = v3;
                    let a243 = 0u64;
                    if nat_eq(a242, a243) {
                        Ok(Adt1::C2(0u64, 2u64, 0u64, 2u64))
                    } else {
                        let a244 = v3;
                        let a245 = 1u64;
                        if nat_eq(a244, a245) {
                            Ok(Adt1::C3(10u64, 1u64, v6, 2u64))
                        } else {
                            let a246 = v3;
                            let a247 = 2u64;
                            if nat_eq(a246, a247) {
                                Ok(Adt1::C4({
                                    let a248 = v1;
                                    let a249 = v2;
                                    nat_mul(a248, a249)?
                                }, v6))
                            } else {
                                Ok(Adt1::C0(0u64))
                            }
                        }
                    }
                } else {
                    let a250 = v0;
                    let a251 = 30u64;
                    if nat_eq(a250, a251) {
                        let a252 = v3;
                        let a253 = 0u64;
                        if nat_eq(a252, a253) {
                            Ok(Adt1::C2(0u64, 2u64, 0u64, 2u64))
                        } else {
                            let a254 = v3;
                            let a255 = 1u64;
                            if nat_eq(a254, a255) {
                                Ok(Adt1::C3(10u64, 30u64, v6, 2u64))
                            } else {
                                let a256 = v3;
                                let a257 = 2u64;
                                if nat_eq(a256, a257) {
                                    Ok(Adt1::C4({
                                        let a258 = v1;
                                        let a259 = v2;
                                        nat_mul(a258, a259)?
                                    }, v6))
                                } else {
                                    Ok(Adt1::C0(0u64))
                                }
                            }
                        }
                    } else {
                        let a260 = v0;
                        let a261 = 43u64;
                        if nat_eq(a260, a261) {
                            let a262 = v3;
                            let a263 = 0u64;
                            if nat_eq(a262, a263) {
                                Ok(Adt1::C2(0u64, 1u64, 0u64, 2u64))
                            } else {
                                let a264 = v3;
                                let a265 = 1u64;
                                if nat_eq(a264, a265) {
                                    Ok(Adt1::C3(9u64, 3u64, v6, 2u64))
                                } else {
                                    let a266 = v3;
                                    let a267 = 2u64;
                                    if nat_eq(a266, a267) {
                                        Ok(Adt1::C4({
                                            let a268 = v1;
                                            let a269 = v2;
                                            nat_mul(a268, a269)?
                                        }, v6))
                                    } else {
                                        Ok(Adt1::C0(0u64))
                                    }
                                }
                            }
                        } else {
                            let a270 = v0;
                            let a271 = 2u64;
                            if nat_eq(a270, a271) {
                                let a272 = v3;
                                let a273 = 0u64;
                                if nat_eq(a272, a273) {
                                    Ok(Adt1::C2(0u64, 1u64, {
                                        let a277 = 2u64;
                                        let a278 = {
                                            let a274 = v1;
                                            let a275 = 16u64;
                                            let a276 = 0u64;
                                            nat_rem(a274, a275, a276)
                                        };
                                        nat_add(a277, a278)?
                                    }, 3u64))
                                } else {
                                    let a279 = v3;
                                    let a280 = 1u64;
                                    if nat_eq(a279, a280) {
                                        Ok(Adt1::C3(8u64, {
                                            let a289 = {
                                                let a286 = v7;
                                                let a287 = {
                                                    let a284 = {
                                                        let a281 = v1;
                                                        let a282 = 16u64;
                                                        let a283 = 0u64;
                                                        nat_quot(a281, a282, a283)
                                                    };
                                                    let a285 = 0u64;
                                                    if nat_eq(a284, a285) {
                                                        1u64
                                                    } else {
                                                        16u64
                                                    }
                                                };
                                                let a288 = 0u64;
                                                nat_quot(a286, a287, a288)
                                            };
                                            let a290 = 16u64;
                                            let a291 = 0u64;
                                            nat_rem(a289, a290, a291)
                                        }, 8u64, 2u64))
                                    } else {
                                        let a292 = v3;
                                        let a293 = 2u64;
                                        if nat_eq(a292, a293) {
                                            f4(2u64, v6, v4, 2u64)
                                        } else {
                                            let a294 = v3;
                                            let a295 = 3u64;
                                            if nat_eq(a294, a295) {
                                                Ok(Adt1::C4({
                                                    let a296 = v1;
                                                    let a297 = v2;
                                                    nat_mul(a296, a297)?
                                                }, v6))
                                            } else {
                                                Ok(Adt1::C0(0u64))
                                            }
                                        }
                                    }
                                }
                            } else {
                                let a298 = v0;
                                let a299 = 3u64;
                                if nat_eq(a298, a299) {
                                    let a300 = v3;
                                    let a301 = 0u64;
                                    if nat_eq(a300, a301) {
                                        Ok(Adt1::C2(0u64, 1u64, {
                                            let a305 = 4u64;
                                            let a306 = {
                                                let a302 = v1;
                                                let a303 = 16u64;
                                                let a304 = 0u64;
                                                nat_rem(a302, a303, a304)
                                            };
                                            nat_add(a305, a306)?
                                        }, 3u64))
                                    } else {
                                        let a307 = v3;
                                        let a308 = 1u64;
                                        if nat_eq(a307, a308) {
                                            Ok(Adt1::C3(8u64, {
                                                let a317 = {
                                                    let a314 = v7;
                                                    let a315 = {
                                                        let a312 = {
                                                            let a309 = v1;
                                                            let a310 = 16u64;
                                                            let a311 = 0u64;
                                                            nat_quot(a309, a310, a311)
                                                        };
                                                        let a313 = 0u64;
                                                        if nat_eq(a312, a313) {
                                                            1u64
                                                        } else {
                                                            16u64
                                                        }
                                                    };
                                                    let a316 = 0u64;
                                                    nat_quot(a314, a315, a316)
                                                };
                                                let a318 = 16u64;
                                                let a319 = 0u64;
                                                nat_rem(a317, a318, a319)
                                            }, 0u64, 2u64))
                                        } else {
                                            let a320 = v3;
                                            let a321 = 2u64;
                                            if nat_eq(a320, a321) {
                                                f4(2u64, v6, v4, 2u64)
                                            } else {
                                                let a322 = v3;
                                                let a323 = 3u64;
                                                if nat_eq(a322, a323) {
                                                    f4(0u64, v6, v5, 2u64)
                                                } else {
                                                    let a324 = v3;
                                                    let a325 = 4u64;
                                                    if nat_eq(a324, a325) {
                                                        Ok(Adt1::C4({
                                                            let a326 = v1;
                                                            let a327 = v2;
                                                            nat_mul(a326, a327)?
                                                        }, v6))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let a328 = v0;
                                    let a329 = 6u64;
                                    if nat_eq(a328, a329) {
                                        let a330 = v3;
                                        let a331 = 0u64;
                                        if nat_eq(a330, a331) {
                                            Ok(Adt1::C2(0u64, 1u64, {
                                                let a335 = 6u64;
                                                let a336 = {
                                                    let a332 = v1;
                                                    let a333 = 16u64;
                                                    let a334 = 0u64;
                                                    nat_rem(a332, a333, a334)
                                                };
                                                nat_add(a335, a336)?
                                            }, 3u64))
                                        } else {
                                            let a337 = v3;
                                            let a338 = 1u64;
                                            if nat_eq(a337, a338) {
                                                Ok(Adt1::C2(0u64, 1u64, {
                                                    let a342 = 2u64;
                                                    let a343 = {
                                                        let a339 = v1;
                                                        let a340 = 8u64;
                                                        let a341 = 0u64;
                                                        nat_quot(a339, a340, a341)
                                                    };
                                                    nat_add(a342, a343)?
                                                }, 4u64))
                                            } else {
                                                let a344 = v3;
                                                let a345 = 2u64;
                                                if nat_eq(a344, a345) {
                                                    Ok(Adt1::C3(8u64, {
                                                        let a368 = {
                                                            let a354 = {
                                                                let a351 = v7;
                                                                let a352 = {
                                                                    let a349 = {
                                                                        let a346 = v1;
                                                                        let a347 = 16u64;
                                                                        let a348 = 0u64;
                                                                        nat_quot(a346, a347, a348)
                                                                    };
                                                                    let a350 = 0u64;
                                                                    if nat_eq(a349, a350) {
                                                                        1u64
                                                                    } else {
                                                                        16u64
                                                                    }
                                                                };
                                                                let a353 = 0u64;
                                                                nat_quot(a351, a352, a353)
                                                            };
                                                            let a355 = 16u64;
                                                            let a356 = 0u64;
                                                            nat_rem(a354, a355, a356)
                                                        };
                                                        let a369 = {
                                                            let a366 = {
                                                                let a363 = {
                                                                    let a360 = v8;
                                                                    let a361 = f10({
                                                                        let a357 = v1;
                                                                        let a358 = 8u64;
                                                                        let a359 = 0u64;
                                                                        nat_rem(a357, a358, a359)
                                                                    });
                                                                    let a362 = 0u64;
                                                                    nat_quot(a360, a361, a362)
                                                                };
                                                                let a364 = 2u64;
                                                                let a365 = 0u64;
                                                                nat_rem(a363, a364, a365)
                                                            };
                                                            let a367 = 16u64;
                                                            nat_mul(a366, a367)?
                                                        };
                                                        nat_add(a368, a369)?
                                                    }, 16u64, 2u64))
                                                } else {
                                                    let a370 = v3;
                                                    let a371 = 3u64;
                                                    if nat_eq(a370, a371) {
                                                        f4(2u64, v6, v4, 2u64)
                                                    } else {
                                                        let a372 = v3;
                                                        let a373 = 4u64;
                                                        if nat_eq(a372, a373) {
                                                            Ok(Adt1::C4({
                                                                let a374 = v1;
                                                                let a375 = v2;
                                                                nat_mul(a374, a375)?
                                                            }, v6))
                                                        } else {
                                                            Ok(Adt1::C0(0u64))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let a376 = v0;
                                        let a377 = 7u64;
                                        if nat_eq(a376, a377) {
                                            let a378 = v3;
                                            let a379 = 0u64;
                                            if nat_eq(a378, a379) {
                                                Ok(Adt1::C2(0u64, 1u64, {
                                                    let a383 = 8u64;
                                                    let a384 = {
                                                        let a380 = v1;
                                                        let a381 = 16u64;
                                                        let a382 = 0u64;
                                                        nat_rem(a380, a381, a382)
                                                    };
                                                    nat_add(a383, a384)?
                                                }, 3u64))
                                            } else {
                                                let a385 = v3;
                                                let a386 = 1u64;
                                                if nat_eq(a385, a386) {
                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                        let a390 = 4u64;
                                                        let a391 = {
                                                            let a387 = v1;
                                                            let a388 = 8u64;
                                                            let a389 = 0u64;
                                                            nat_quot(a387, a388, a389)
                                                        };
                                                        nat_add(a390, a391)?
                                                    }, 4u64))
                                                } else {
                                                    let a392 = v3;
                                                    let a393 = 2u64;
                                                    if nat_eq(a392, a393) {
                                                        Ok(Adt1::C3(8u64, {
                                                            let a416 = {
                                                                let a402 = {
                                                                    let a399 = v7;
                                                                    let a400 = {
                                                                        let a397 = {
                                                                            let a394 = v1;
                                                                            let a395 = 16u64;
                                                                            let a396 = 0u64;
                                                                            nat_quot(a394, a395, a396)
                                                                        };
                                                                        let a398 = 0u64;
                                                                        if nat_eq(a397, a398) {
                                                                            1u64
                                                                        } else {
                                                                            16u64
                                                                        }
                                                                    };
                                                                    let a401 = 0u64;
                                                                    nat_quot(a399, a400, a401)
                                                                };
                                                                let a403 = 16u64;
                                                                let a404 = 0u64;
                                                                nat_rem(a402, a403, a404)
                                                            };
                                                            let a417 = {
                                                                let a414 = {
                                                                    let a411 = {
                                                                        let a408 = v8;
                                                                        let a409 = f10({
                                                                            let a405 = v1;
                                                                            let a406 = 8u64;
                                                                            let a407 = 0u64;
                                                                            nat_rem(a405, a406, a407)
                                                                        });
                                                                        let a410 = 0u64;
                                                                        nat_quot(a408, a409, a410)
                                                                    };
                                                                    let a412 = 2u64;
                                                                    let a413 = 0u64;
                                                                    nat_rem(a411, a412, a413)
                                                                };
                                                                let a415 = 16u64;
                                                                nat_mul(a414, a415)?
                                                            };
                                                            nat_add(a416, a417)?
                                                        }, 0u64, 2u64))
                                                    } else {
                                                        let a418 = v3;
                                                        let a419 = 3u64;
                                                        if nat_eq(a418, a419) {
                                                            f4(2u64, v6, v4, 2u64)
                                                        } else {
                                                            let a420 = v3;
                                                            let a421 = 4u64;
                                                            if nat_eq(a420, a421) {
                                                                f4(0u64, v6, v5, 2u64)
                                                            } else {
                                                                let a422 = v3;
                                                                let a423 = 5u64;
                                                                if nat_eq(a422, a423) {
                                                                    Ok(Adt1::C4({
                                                                        let a424 = v1;
                                                                        let a425 = v2;
                                                                        nat_mul(a424, a425)?
                                                                    }, v6))
                                                                } else {
                                                                    Ok(Adt1::C0(0u64))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if f9(v0) {
                                                let a426 = v3;
                                                let a427 = 0u64;
                                                if nat_eq(a426, a427) {
                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                        let a430 = {
                                                            let a428 = v0;
                                                            let a429 = 8u64;
                                                            if nat_eq(a428, a429) {
                                                                2u64
                                                            } else {
                                                                4u64
                                                            }
                                                        };
                                                        let a431 = v1;
                                                        nat_add(a430, a431)?
                                                    }, 3u64))
                                                } else {
                                                    let a432 = v3;
                                                    let a433 = 1u64;
                                                    if nat_eq(a432, a433) {
                                                        Ok(Adt1::C3(8u64, v7, {
                                                            let a434 = v7;
                                                            let a435 = 128u64;
                                                            if nat_lt(a434, a435) {
                                                                0u64
                                                            } else {
                                                                256u64
                                                            }
                                                        }, 2u64))
                                                    } else {
                                                        let a436 = v3;
                                                        let a437 = 2u64;
                                                        if nat_eq(a436, a437) {
                                                            f4(2u64, v6, v4, 2u64)
                                                        } else {
                                                            let a438 = v3;
                                                            let a439 = 3u64;
                                                            if nat_eq(a438, a439) {
                                                                Ok(Adt1::C4({
                                                                    let a440 = v1;
                                                                    let a441 = v2;
                                                                    nat_mul(a440, a441)?
                                                                }, v6))
                                                            } else {
                                                                Ok(Adt1::C0(0u64))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                let a442 = v0;
                                                let a443 = 41u64;
                                                if nat_eq(a442, a443) {
                                                    let a444 = v3;
                                                    let a445 = 0u64;
                                                    if nat_eq(a444, a445) {
                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                            let a449 = 2u64;
                                                            let a450 = {
                                                                let a446 = v1;
                                                                let a447 = 8u64;
                                                                let a448 = 0u64;
                                                                nat_quot(a446, a447, a448)
                                                            };
                                                            nat_add(a449, a450)?
                                                        }, 3u64))
                                                    } else {
                                                        let a451 = v3;
                                                        let a452 = 1u64;
                                                        if nat_eq(a451, a452) {
                                                            Ok(Adt1::C3(6u64, v4, 0u64, 2u64))
                                                        } else {
                                                            let a453 = v3;
                                                            let a454 = 2u64;
                                                            if nat_eq(a453, a454) {
                                                                Ok(Adt1::C4({
                                                                    let a455 = v1;
                                                                    let a456 = v2;
                                                                    nat_mul(a455, a456)?
                                                                }, {
                                                                    let a466 = {
                                                                        let a463 = {
                                                                            let a460 = v7;
                                                                            let a461 = f10({
                                                                                let a457 = v1;
                                                                                let a458 = 8u64;
                                                                                let a459 = 0u64;
                                                                                nat_rem(a457, a458, a459)
                                                                            });
                                                                            let a462 = 0u64;
                                                                            nat_quot(a460, a461, a462)
                                                                        };
                                                                        let a464 = 2u64;
                                                                        let a465 = 0u64;
                                                                        nat_rem(a463, a464, a465)
                                                                    };
                                                                    let a467 = 1u64;
                                                                    if nat_eq(a466, a467) {
                                                                        v4
                                                                    } else {
                                                                        v6
                                                                    }
                                                                }))
                                                            } else {
                                                                Ok(Adt1::C0(0u64))
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    let a468 = v0;
                                                    let a469 = 42u64;
                                                    if nat_eq(a468, a469) {
                                                        let a470 = v3;
                                                        let a471 = 0u64;
                                                        if nat_eq(a470, a471) {
                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                let a475 = 2u64;
                                                                let a476 = {
                                                                    let a472 = v1;
                                                                    let a473 = 4u64;
                                                                    let a474 = 0u64;
                                                                    nat_quot(a472, a473, a474)
                                                                };
                                                                nat_add(a475, a476)?
                                                            }, 3u64))
                                                        } else {
                                                            let a477 = v3;
                                                            let a478 = 1u64;
                                                            if nat_eq(a477, a478) {
                                                                Ok(Adt1::C3(8u64, {
                                                                    let a487 = {
                                                                        let a484 = v7;
                                                                        let a485 = f10({
                                                                            let a482 = 2u64;
                                                                            let a483 = {
                                                                                let a479 = v1;
                                                                                let a480 = 4u64;
                                                                                let a481 = 0u64;
                                                                                nat_rem(a479, a480, a481)
                                                                            };
                                                                            nat_mul(a482, a483)?
                                                                        });
                                                                        let a486 = 0u64;
                                                                        nat_quot(a484, a485, a486)
                                                                    };
                                                                    let a488 = 4u64;
                                                                    let a489 = 0u64;
                                                                    nat_rem(a487, a488, a489)
                                                                }, 1u64, 2u64))
                                                            } else {
                                                                let a490 = v3;
                                                                let a491 = 2u64;
                                                                if nat_eq(a490, a491) {
                                                                    f4(2u64, v6, v4, 2u64)
                                                                } else {
                                                                    let a492 = v3;
                                                                    let a493 = 3u64;
                                                                    if nat_eq(a492, a493) {
                                                                        Ok(Adt1::C4({
                                                                            let a494 = v1;
                                                                            let a495 = v2;
                                                                            nat_mul(a494, a495)?
                                                                        }, v6))
                                                                    } else {
                                                                        Ok(Adt1::C0(0u64))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a496 = v0;
                                                        let a497 = 39u64;
                                                        if nat_eq(a496, a497) {
                                                            let a498 = v3;
                                                            let a499 = 0u64;
                                                            if nat_eq(a498, a499) {
                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                    let a503 = 1u64;
                                                                    let a504 = {
                                                                        let a500 = v1;
                                                                        let a501 = 16u64;
                                                                        let a502 = 0u64;
                                                                        nat_rem(a500, a501, a502)
                                                                    };
                                                                    nat_add(a503, a504)?
                                                                }, 3u64))
                                                            } else {
                                                                let a505 = v3;
                                                                let a506 = 1u64;
                                                                if nat_eq(a505, a506) {
                                                                    Ok(Adt1::C3(9u64, 6u64, {
                                                                        let a515 = {
                                                                            let a512 = v7;
                                                                            let a513 = {
                                                                                let a510 = {
                                                                                    let a507 = v1;
                                                                                    let a508 = 16u64;
                                                                                    let a509 = 0u64;
                                                                                    nat_quot(a507, a508, a509)
                                                                                };
                                                                                let a511 = 0u64;
                                                                                if nat_eq(a510, a511) {
                                                                                    1u64
                                                                                } else {
                                                                                    16u64
                                                                                }
                                                                            };
                                                                            let a514 = 0u64;
                                                                            nat_quot(a512, a513, a514)
                                                                        };
                                                                        let a516 = 16u64;
                                                                        let a517 = 0u64;
                                                                        nat_rem(a515, a516, a517)
                                                                    }, 2u64))
                                                                } else {
                                                                    let a518 = v3;
                                                                    let a519 = 2u64;
                                                                    if nat_eq(a518, a519) {
                                                                        f4(2u64, v6, v4, 2u64)
                                                                    } else {
                                                                        let a520 = v3;
                                                                        let a521 = 3u64;
                                                                        if nat_eq(a520, a521) {
                                                                            Ok(Adt1::C4({
                                                                                let a522 = v1;
                                                                                let a523 = v2;
                                                                                nat_mul(a522, a523)?
                                                                            }, v6))
                                                                        } else {
                                                                            Ok(Adt1::C0(0u64))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            let a524 = v0;
                                                            let a525 = 40u64;
                                                            if nat_eq(a524, a525) {
                                                                let a526 = v3;
                                                                let a527 = 0u64;
                                                                if nat_eq(a526, a527) {
                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                        let a528 = v1;
                                                                        let a529 = 16u64;
                                                                        let a530 = 0u64;
                                                                        nat_quot(a528, a529, a530)
                                                                    }, 3u64))
                                                                } else {
                                                                    let a531 = v3;
                                                                    let a532 = 1u64;
                                                                    if nat_eq(a531, a532) {
                                                                        Ok(Adt1::C3(9u64, 2u64, v7, 2u64))
                                                                    } else {
                                                                        let a533 = v3;
                                                                        let a534 = 2u64;
                                                                        if nat_eq(a533, a534) {
                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                let a545 = {
                                                                                    let a540 = 4u64;
                                                                                    let a541 = {
                                                                                        let a538 = {
                                                                                            let a535 = v1;
                                                                                            let a536 = 16u64;
                                                                                            let a537 = 0u64;
                                                                                            nat_quot(a535, a536, a537)
                                                                                        };
                                                                                        let a539 = 8u64;
                                                                                        nat_mul(a538, a539)?
                                                                                    };
                                                                                    nat_add(a540, a541)?
                                                                                };
                                                                                let a546 = {
                                                                                    let a542 = v1;
                                                                                    let a543 = 8u64;
                                                                                    let a544 = 0u64;
                                                                                    nat_rem(a542, a543, a544)
                                                                                };
                                                                                nat_add(a545, a546)?
                                                                            }, 3u64))
                                                                        } else {
                                                                            let a547 = v3;
                                                                            let a548 = 3u64;
                                                                            if nat_eq(a547, a548) {
                                                                                Ok(Adt1::C3(9u64, 6u64, {
                                                                                    let a560 = {
                                                                                        let a557 = v7;
                                                                                        let a558 = {
                                                                                            let a555 = {
                                                                                                let a552 = {
                                                                                                    let a549 = v1;
                                                                                                    let a550 = 16u64;
                                                                                                    let a551 = 0u64;
                                                                                                    nat_rem(a549, a550, a551)
                                                                                                };
                                                                                                let a553 = 8u64;
                                                                                                let a554 = 0u64;
                                                                                                nat_quot(a552, a553, a554)
                                                                                            };
                                                                                            let a556 = 0u64;
                                                                                            if nat_eq(a555, a556) {
                                                                                                1u64
                                                                                            } else {
                                                                                                16u64
                                                                                            }
                                                                                        };
                                                                                        let a559 = 0u64;
                                                                                        nat_quot(a557, a558, a559)
                                                                                    };
                                                                                    let a561 = 16u64;
                                                                                    let a562 = 0u64;
                                                                                    nat_rem(a560, a561, a562)
                                                                                }, 3u64))
                                                                            } else {
                                                                                let a563 = v3;
                                                                                let a564 = 4u64;
                                                                                if nat_eq(a563, a564) {
                                                                                    f4(2u64, v7, v6, 2u64)
                                                                                } else {
                                                                                    let a565 = v3;
                                                                                    let a566 = 5u64;
                                                                                    if nat_eq(a565, a566) {
                                                                                        Ok(Adt1::C4({
                                                                                            let a567 = v1;
                                                                                            let a568 = v2;
                                                                                            nat_mul(a567, a568)?
                                                                                        }, v6))
                                                                                    } else {
                                                                                        Ok(Adt1::C0(0u64))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                let a569 = v0;
                                                                let a570 = 10u64;
                                                                if nat_eq(a569, a570) {
                                                                    let a571 = v3;
                                                                    let a572 = 0u64;
                                                                    if nat_eq(a571, a572) {
                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                            let a573 = v1;
                                                                            let a574 = 16u64;
                                                                            let a575 = 0u64;
                                                                            nat_quot(a573, a574, a575)
                                                                        }, 3u64))
                                                                    } else {
                                                                        let a576 = v3;
                                                                        let a577 = 1u64;
                                                                        if nat_eq(a576, a577) {
                                                                            Ok(Adt1::C3(8u64, {
                                                                                let a578 = v7;
                                                                                let a579 = 16u64;
                                                                                let a580 = 0u64;
                                                                                nat_rem(a578, a579, a580)
                                                                            }, 0u64, 2u64))
                                                                        } else {
                                                                            let a581 = v3;
                                                                            let a582 = 2u64;
                                                                            if nat_eq(a581, a582) {
                                                                                f4(2u64, v4, v6, 2u64)
                                                                            } else {
                                                                                let a583 = v3;
                                                                                let a584 = 3u64;
                                                                                if nat_eq(a583, a584) {
                                                                                    Ok(Adt1::C3(8u64, {
                                                                                        let a585 = v7;
                                                                                        let a586 = 16u64;
                                                                                        let a587 = 0u64;
                                                                                        nat_quot(a585, a586, a587)
                                                                                    }, 0u64, 4u64))
                                                                                } else {
                                                                                    let a588 = v3;
                                                                                    let a589 = 4u64;
                                                                                    if nat_eq(a588, a589) {
                                                                                        f4(2u64, v5, v8, 4u64)
                                                                                    } else {
                                                                                        let a590 = v3;
                                                                                        let a591 = 5u64;
                                                                                        if nat_eq(a590, a591) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a602 = 16u64;
                                                                                                let a603 = {
                                                                                                    let a600 = {
                                                                                                        let a595 = 32u64;
                                                                                                        let a596 = {
                                                                                                            let a592 = v1;
                                                                                                            let a593 = 128u64;
                                                                                                            let a594 = 0u64;
                                                                                                            nat_quot(a592, a593, a594)
                                                                                                        };
                                                                                                        nat_mul(a595, a596)?
                                                                                                    };
                                                                                                    let a601 = {
                                                                                                        let a597 = v1;
                                                                                                        let a598 = 32u64;
                                                                                                        let a599 = 0u64;
                                                                                                        nat_rem(a597, a598, a599)
                                                                                                    };
                                                                                                    nat_add(a600, a601)?
                                                                                                };
                                                                                                nat_add(a602, a603)?
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a604 = v3;
                                                                                            let a605 = 6u64;
                                                                                            if nat_eq(a604, a605) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a617 = {
                                                                                                        let a614 = v7;
                                                                                                        let a615 = f10({
                                                                                                            let a612 = 2u64;
                                                                                                            let a613 = {
                                                                                                                let a609 = {
                                                                                                                    let a606 = v1;
                                                                                                                    let a607 = 128u64;
                                                                                                                    let a608 = 0u64;
                                                                                                                    nat_rem(a606, a607, a608)
                                                                                                                };
                                                                                                                let a610 = 32u64;
                                                                                                                let a611 = 0u64;
                                                                                                                nat_quot(a609, a610, a611)
                                                                                                            };
                                                                                                            nat_mul(a612, a613)?
                                                                                                        });
                                                                                                        let a616 = 0u64;
                                                                                                        nat_quot(a614, a615, a616)
                                                                                                    };
                                                                                                    let a618 = 4u64;
                                                                                                    let a619 = 0u64;
                                                                                                    nat_rem(a617, a618, a619)
                                                                                                }, 0u64, 3u64))
                                                                                            } else {
                                                                                                let a620 = v3;
                                                                                                let a621 = 7u64;
                                                                                                if nat_eq(a620, a621) {
                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                } else {
                                                                                                    let a622 = v3;
                                                                                                    let a623 = 8u64;
                                                                                                    if nat_eq(a622, a623) {
                                                                                                        f4(1u64, v6, v8, 2u64)
                                                                                                    } else {
                                                                                                        let a624 = v3;
                                                                                                        let a625 = 9u64;
                                                                                                        if nat_eq(a624, a625) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a626 = v1;
                                                                                                                let a627 = v2;
                                                                                                                nat_mul(a626, a627)?
                                                                                                            }, v6))
                                                                                                        } else {
                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a628 = v0;
                                                                    let a629 = 11u64;
                                                                    if nat_eq(a628, a629) {
                                                                        let a630 = v3;
                                                                        let a631 = 0u64;
                                                                        if nat_eq(a630, a631) {
                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                let a638 = 96u64;
                                                                                let a639 = {
                                                                                    let a635 = {
                                                                                        let a632 = v1;
                                                                                        let a633 = 16u64;
                                                                                        let a634 = 0u64;
                                                                                        nat_quot(a632, a633, a634)
                                                                                    };
                                                                                    let a636 = 8u64;
                                                                                    let a637 = 0u64;
                                                                                    nat_rem(a635, a636, a637)
                                                                                };
                                                                                nat_add(a638, a639)?
                                                                            }, 3u64))
                                                                        } else {
                                                                            let a640 = v3;
                                                                            let a641 = 1u64;
                                                                            if nat_eq(a640, a641) {
                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                    let a648 = 104u64;
                                                                                    let a649 = {
                                                                                        let a645 = {
                                                                                            let a642 = v1;
                                                                                            let a643 = 16u64;
                                                                                            let a644 = 0u64;
                                                                                            nat_quot(a642, a643, a644)
                                                                                        };
                                                                                        let a646 = 4u64;
                                                                                        let a647 = 0u64;
                                                                                        nat_rem(a645, a646, a647)
                                                                                    };
                                                                                    nat_add(a648, a649)?
                                                                                }, 4u64))
                                                                            } else {
                                                                                let a650 = v3;
                                                                                let a651 = 2u64;
                                                                                if nat_eq(a650, a651) {
                                                                                    Ok(Adt1::C3(8u64, {
                                                                                        let a682 = {
                                                                                            let a663 = {
                                                                                                let a660 = v7;
                                                                                                let a661 = {
                                                                                                    let a658 = {
                                                                                                        let a655 = {
                                                                                                            let a652 = v1;
                                                                                                            let a653 = 16u64;
                                                                                                            let a654 = 0u64;
                                                                                                            nat_quot(a652, a653, a654)
                                                                                                        };
                                                                                                        let a656 = 8u64;
                                                                                                        let a657 = 0u64;
                                                                                                        nat_quot(a655, a656, a657)
                                                                                                    };
                                                                                                    let a659 = 0u64;
                                                                                                    if nat_eq(a658, a659) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        16u64
                                                                                                    }
                                                                                                };
                                                                                                let a662 = 0u64;
                                                                                                nat_quot(a660, a661, a662)
                                                                                            };
                                                                                            let a664 = 16u64;
                                                                                            let a665 = 0u64;
                                                                                            nat_rem(a663, a664, a665)
                                                                                        };
                                                                                        let a683 = {
                                                                                            let a680 = {
                                                                                                let a677 = {
                                                                                                    let a674 = v8;
                                                                                                    let a675 = f10({
                                                                                                        let a672 = 2u64;
                                                                                                        let a673 = {
                                                                                                            let a669 = {
                                                                                                                let a666 = v1;
                                                                                                                let a667 = 16u64;
                                                                                                                let a668 = 0u64;
                                                                                                                nat_quot(a666, a667, a668)
                                                                                                            };
                                                                                                            let a670 = 4u64;
                                                                                                            let a671 = 0u64;
                                                                                                            nat_quot(a669, a670, a671)
                                                                                                        };
                                                                                                        nat_mul(a672, a673)?
                                                                                                    });
                                                                                                    let a676 = 0u64;
                                                                                                    nat_quot(a674, a675, a676)
                                                                                                };
                                                                                                let a678 = 4u64;
                                                                                                let a679 = 0u64;
                                                                                                nat_rem(a677, a678, a679)
                                                                                            };
                                                                                            let a681 = 16u64;
                                                                                            nat_mul(a680, a681)?
                                                                                        };
                                                                                        nat_add(a682, a683)?
                                                                                    }, 32u64, 2u64))
                                                                                } else {
                                                                                    let a684 = v3;
                                                                                    let a685 = 3u64;
                                                                                    if nat_eq(a684, a685) {
                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                    } else {
                                                                                        let a686 = v3;
                                                                                        let a687 = 4u64;
                                                                                        if nat_eq(a686, a687) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a698 = 32u64;
                                                                                                let a699 = {
                                                                                                    let a696 = {
                                                                                                        let a691 = 32u64;
                                                                                                        let a692 = {
                                                                                                            let a688 = v1;
                                                                                                            let a689 = 128u64;
                                                                                                            let a690 = 0u64;
                                                                                                            nat_quot(a688, a689, a690)
                                                                                                        };
                                                                                                        nat_mul(a691, a692)?
                                                                                                    };
                                                                                                    let a697 = {
                                                                                                        let a693 = v1;
                                                                                                        let a694 = 32u64;
                                                                                                        let a695 = 0u64;
                                                                                                        nat_rem(a693, a694, a695)
                                                                                                    };
                                                                                                    nat_add(a696, a697)?
                                                                                                };
                                                                                                nat_add(a698, a699)?
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a700 = v3;
                                                                                            let a701 = 5u64;
                                                                                            if nat_eq(a700, a701) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                    let a702 = v1;
                                                                                                    let a703 = 32u64;
                                                                                                    let a704 = 0u64;
                                                                                                    nat_rem(a702, a703, a704)
                                                                                                }, 4u64))
                                                                                            } else {
                                                                                                let a705 = v3;
                                                                                                let a706 = 6u64;
                                                                                                if nat_eq(a705, a706) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a718 = {
                                                                                                            let a715 = v7;
                                                                                                            let a716 = f10({
                                                                                                                let a713 = 2u64;
                                                                                                                let a714 = {
                                                                                                                    let a710 = {
                                                                                                                        let a707 = v1;
                                                                                                                        let a708 = 128u64;
                                                                                                                        let a709 = 0u64;
                                                                                                                        nat_rem(a707, a708, a709)
                                                                                                                    };
                                                                                                                    let a711 = 32u64;
                                                                                                                    let a712 = 0u64;
                                                                                                                    nat_quot(a710, a711, a712)
                                                                                                                };
                                                                                                                nat_mul(a713, a714)?
                                                                                                            });
                                                                                                            let a717 = 0u64;
                                                                                                            nat_quot(a715, a716, a717)
                                                                                                        };
                                                                                                        let a719 = 4u64;
                                                                                                        let a720 = 0u64;
                                                                                                        nat_rem(a718, a719, a720)
                                                                                                    }, {
                                                                                                        let a730 = {
                                                                                                            let a727 = {
                                                                                                                let a724 = v8;
                                                                                                                let a725 = f10({
                                                                                                                    let a721 = v1;
                                                                                                                    let a722 = 32u64;
                                                                                                                    let a723 = 0u64;
                                                                                                                    nat_quot(a721, a722, a723)
                                                                                                                });
                                                                                                                let a726 = 0u64;
                                                                                                                nat_quot(a724, a725, a726)
                                                                                                            };
                                                                                                            let a728 = 2u64;
                                                                                                            let a729 = 0u64;
                                                                                                            nat_rem(a727, a728, a729)
                                                                                                        };
                                                                                                        let a731 = 1u64;
                                                                                                        if nat_eq(a730, a731) {
                                                                                                            0u64
                                                                                                        } else {
                                                                                                            4u64
                                                                                                        }
                                                                                                    }, 3u64))
                                                                                                } else {
                                                                                                    let a732 = v3;
                                                                                                    let a733 = 7u64;
                                                                                                    if nat_eq(a732, a733) {
                                                                                                        f4(2u64, v6, v7, 2u64)
                                                                                                    } else {
                                                                                                        let a734 = v3;
                                                                                                        let a735 = 8u64;
                                                                                                        if nat_eq(a734, a735) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a736 = v1;
                                                                                                                let a737 = v2;
                                                                                                                nat_mul(a736, a737)?
                                                                                                            }, v6))
                                                                                                        } else {
                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let a738 = v0;
                                                                        let a739 = 12u64;
                                                                        if nat_eq(a738, a739) {
                                                                            let a740 = v3;
                                                                            let a741 = 0u64;
                                                                            if nat_eq(a740, a741) {
                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                    let a755 = 4u64;
                                                                                    let a756 = {
                                                                                        let a745 = {
                                                                                            let a742 = v1;
                                                                                            let a743 = 32u64;
                                                                                            let a744 = 0u64;
                                                                                            nat_quot(a742, a743, a744)
                                                                                        };
                                                                                        let a746 = 4u64;
                                                                                        if nat_lt(a745, a746) {
                                                                                            let a747 = v1;
                                                                                            let a748 = 32u64;
                                                                                            let a749 = 0u64;
                                                                                            nat_quot(a747, a748, a749)
                                                                                        } else {
                                                                                            let a753 = {
                                                                                                let a750 = v1;
                                                                                                let a751 = 32u64;
                                                                                                let a752 = 0u64;
                                                                                                nat_quot(a750, a751, a752)
                                                                                            };
                                                                                            let a754 = 4u64;
                                                                                            nat_sub(a753, a754)
                                                                                        }
                                                                                    };
                                                                                    nat_add(a755, a756)?
                                                                                }, 3u64))
                                                                            } else {
                                                                                let a757 = v3;
                                                                                let a758 = 1u64;
                                                                                if nat_eq(a757, a758) {
                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                        let a772 = 4u64;
                                                                                        let a773 = {
                                                                                            let a762 = {
                                                                                                let a759 = v1;
                                                                                                let a760 = 32u64;
                                                                                                let a761 = 0u64;
                                                                                                nat_quot(a759, a760, a761)
                                                                                            };
                                                                                            let a763 = 4u64;
                                                                                            if nat_lt(a762, a763) {
                                                                                                let a767 = {
                                                                                                    let a764 = v1;
                                                                                                    let a765 = 32u64;
                                                                                                    let a766 = 0u64;
                                                                                                    nat_quot(a764, a765, a766)
                                                                                                };
                                                                                                let a768 = 4u64;
                                                                                                nat_add(a767, a768)?
                                                                                            } else {
                                                                                                let a769 = v1;
                                                                                                let a770 = 32u64;
                                                                                                let a771 = 0u64;
                                                                                                nat_quot(a769, a770, a771)
                                                                                            }
                                                                                        };
                                                                                        nat_add(a772, a773)?
                                                                                    }, 4u64))
                                                                                } else {
                                                                                    let a774 = v3;
                                                                                    let a775 = 2u64;
                                                                                    if nat_eq(a774, a775) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a789 = 4u64;
                                                                                            let a790 = {
                                                                                                let a779 = {
                                                                                                    let a776 = v1;
                                                                                                    let a777 = 32u64;
                                                                                                    let a778 = 0u64;
                                                                                                    nat_quot(a776, a777, a778)
                                                                                                };
                                                                                                let a780 = 4u64;
                                                                                                if nat_lt(a779, a780) {
                                                                                                    let a781 = v1;
                                                                                                    let a782 = 32u64;
                                                                                                    let a783 = 0u64;
                                                                                                    nat_quot(a781, a782, a783)
                                                                                                } else {
                                                                                                    let a787 = {
                                                                                                        let a784 = v1;
                                                                                                        let a785 = 32u64;
                                                                                                        let a786 = 0u64;
                                                                                                        nat_quot(a784, a785, a786)
                                                                                                    };
                                                                                                    let a788 = 4u64;
                                                                                                    nat_add(a787, a788)?
                                                                                                }
                                                                                            };
                                                                                            nat_add(a789, a790)?
                                                                                        }, 5u64))
                                                                                    } else {
                                                                                        let a791 = v3;
                                                                                        let a792 = 3u64;
                                                                                        if nat_eq(a791, a792) {
                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                let a796 = {
                                                                                                    let a793 = v1;
                                                                                                    let a794 = 32u64;
                                                                                                    let a795 = 0u64;
                                                                                                    nat_quot(a793, a794, a795)
                                                                                                };
                                                                                                let a797 = 4u64;
                                                                                                if nat_lt(a796, a797) {
                                                                                                    let a798 = v7;
                                                                                                    let a799 = 64u64;
                                                                                                    let a800 = 0u64;
                                                                                                    nat_rem(a798, a799, a800)
                                                                                                } else {
                                                                                                    let a809 = {
                                                                                                        let a801 = v9;
                                                                                                        let a802 = 16u64;
                                                                                                        let a803 = 0u64;
                                                                                                        nat_rem(a801, a802, a803)
                                                                                                    };
                                                                                                    let a810 = {
                                                                                                        let a807 = {
                                                                                                            let a804 = v7;
                                                                                                            let a805 = 64u64;
                                                                                                            let a806 = 0u64;
                                                                                                            nat_quot(a804, a805, a806)
                                                                                                        };
                                                                                                        let a808 = 16u64;
                                                                                                        nat_mul(a807, a808)?
                                                                                                    };
                                                                                                    nat_add(a809, a810)?
                                                                                                }
                                                                                            }, 0u64, 2u64))
                                                                                        } else {
                                                                                            let a811 = v3;
                                                                                            let a812 = 4u64;
                                                                                            if nat_eq(a811, a812) {
                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                            } else {
                                                                                                let a813 = v3;
                                                                                                let a814 = 5u64;
                                                                                                if nat_eq(a813, a814) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a818 = {
                                                                                                            let a815 = v1;
                                                                                                            let a816 = 32u64;
                                                                                                            let a817 = 0u64;
                                                                                                            nat_quot(a815, a816, a817)
                                                                                                        };
                                                                                                        let a819 = 4u64;
                                                                                                        if nat_lt(a818, a819) {
                                                                                                            let a820 = v8;
                                                                                                            let a821 = 64u64;
                                                                                                            let a822 = 0u64;
                                                                                                            nat_rem(a820, a821, a822)
                                                                                                        } else {
                                                                                                            let a831 = {
                                                                                                                let a823 = v9;
                                                                                                                let a824 = 16u64;
                                                                                                                let a825 = 0u64;
                                                                                                                nat_quot(a823, a824, a825)
                                                                                                            };
                                                                                                            let a832 = {
                                                                                                                let a829 = {
                                                                                                                    let a826 = v8;
                                                                                                                    let a827 = 64u64;
                                                                                                                    let a828 = 0u64;
                                                                                                                    nat_quot(a826, a827, a828)
                                                                                                                };
                                                                                                                let a830 = 16u64;
                                                                                                                nat_mul(a829, a830)?
                                                                                                            };
                                                                                                            nat_add(a831, a832)?
                                                                                                        }
                                                                                                    }, 0u64, 6u64))
                                                                                                } else {
                                                                                                    let a833 = v3;
                                                                                                    let a834 = 6u64;
                                                                                                    if nat_eq(a833, a834) {
                                                                                                        f4(2u64, v5, v10, 6u64)
                                                                                                    } else {
                                                                                                        let a835 = v3;
                                                                                                        let a836 = 7u64;
                                                                                                        if nat_eq(a835, a836) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a850 = {
                                                                                                                    let a845 = 16u64;
                                                                                                                    let a846 = {
                                                                                                                        let a843 = {
                                                                                                                            let a840 = {
                                                                                                                                let a837 = v1;
                                                                                                                                let a838 = 32u64;
                                                                                                                                let a839 = 0u64;
                                                                                                                                nat_quot(a837, a838, a839)
                                                                                                                            };
                                                                                                                            let a841 = 2u64;
                                                                                                                            let a842 = 0u64;
                                                                                                                            nat_quot(a840, a841, a842)
                                                                                                                        };
                                                                                                                        let a844 = 32u64;
                                                                                                                        nat_mul(a843, a844)?
                                                                                                                    };
                                                                                                                    nat_add(a845, a846)?
                                                                                                                };
                                                                                                                let a851 = {
                                                                                                                    let a847 = v1;
                                                                                                                    let a848 = 32u64;
                                                                                                                    let a849 = 0u64;
                                                                                                                    nat_rem(a847, a848, a849)
                                                                                                                };
                                                                                                                nat_add(a850, a851)?
                                                                                                            }, 3u64))
                                                                                                        } else {
                                                                                                            let a852 = v3;
                                                                                                            let a853 = 8u64;
                                                                                                            if nat_eq(a852, a853) {
                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                    let a865 = {
                                                                                                                        let a862 = v7;
                                                                                                                        let a863 = {
                                                                                                                            let a860 = {
                                                                                                                                let a857 = {
                                                                                                                                    let a854 = v1;
                                                                                                                                    let a855 = 32u64;
                                                                                                                                    let a856 = 0u64;
                                                                                                                                    nat_quot(a854, a855, a856)
                                                                                                                                };
                                                                                                                                let a858 = 2u64;
                                                                                                                                let a859 = 0u64;
                                                                                                                                nat_rem(a857, a858, a859)
                                                                                                                            };
                                                                                                                            let a861 = 0u64;
                                                                                                                            if nat_eq(a860, a861) {
                                                                                                                                1u64
                                                                                                                            } else {
                                                                                                                                16u64
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a864 = 0u64;
                                                                                                                        nat_quot(a862, a863, a864)
                                                                                                                    };
                                                                                                                    let a866 = 16u64;
                                                                                                                    let a867 = 0u64;
                                                                                                                    nat_rem(a865, a866, a867)
                                                                                                                }, 0u64, 3u64))
                                                                                                            } else {
                                                                                                                let a868 = v3;
                                                                                                                let a869 = 9u64;
                                                                                                                if nat_eq(a868, a869) {
                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                } else {
                                                                                                                    let a870 = v3;
                                                                                                                    let a871 = 10u64;
                                                                                                                    if nat_eq(a870, a871) {
                                                                                                                        f4(1u64, v6, v10, 2u64)
                                                                                                                    } else {
                                                                                                                        let a872 = v3;
                                                                                                                        let a873 = 11u64;
                                                                                                                        if nat_eq(a872, a873) {
                                                                                                                            Ok(Adt1::C4({
                                                                                                                                let a874 = v1;
                                                                                                                                let a875 = v2;
                                                                                                                                nat_mul(a874, a875)?
                                                                                                                            }, v6))
                                                                                                                        } else {
                                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            let a876 = v0;
                                                                            let a877 = 13u64;
                                                                            if nat_eq(a876, a877) {
                                                                                let a878 = v3;
                                                                                let a879 = 0u64;
                                                                                if nat_eq(a878, a879) {
                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                        let a893 = 4u64;
                                                                                        let a894 = {
                                                                                            let a883 = {
                                                                                                let a880 = v1;
                                                                                                let a881 = 32u64;
                                                                                                let a882 = 0u64;
                                                                                                nat_quot(a880, a881, a882)
                                                                                            };
                                                                                            let a884 = 4u64;
                                                                                            if nat_lt(a883, a884) {
                                                                                                let a885 = v1;
                                                                                                let a886 = 32u64;
                                                                                                let a887 = 0u64;
                                                                                                nat_quot(a885, a886, a887)
                                                                                            } else {
                                                                                                let a891 = {
                                                                                                    let a888 = v1;
                                                                                                    let a889 = 32u64;
                                                                                                    let a890 = 0u64;
                                                                                                    nat_quot(a888, a889, a890)
                                                                                                };
                                                                                                let a892 = 4u64;
                                                                                                nat_sub(a891, a892)
                                                                                            }
                                                                                        };
                                                                                        nat_add(a893, a894)?
                                                                                    }, 3u64))
                                                                                } else {
                                                                                    let a895 = v3;
                                                                                    let a896 = 1u64;
                                                                                    if nat_eq(a895, a896) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a910 = 4u64;
                                                                                            let a911 = {
                                                                                                let a900 = {
                                                                                                    let a897 = v1;
                                                                                                    let a898 = 32u64;
                                                                                                    let a899 = 0u64;
                                                                                                    nat_quot(a897, a898, a899)
                                                                                                };
                                                                                                let a901 = 4u64;
                                                                                                if nat_lt(a900, a901) {
                                                                                                    let a905 = {
                                                                                                        let a902 = v1;
                                                                                                        let a903 = 32u64;
                                                                                                        let a904 = 0u64;
                                                                                                        nat_quot(a902, a903, a904)
                                                                                                    };
                                                                                                    let a906 = 4u64;
                                                                                                    nat_add(a905, a906)?
                                                                                                } else {
                                                                                                    let a907 = v1;
                                                                                                    let a908 = 32u64;
                                                                                                    let a909 = 0u64;
                                                                                                    nat_quot(a907, a908, a909)
                                                                                                }
                                                                                            };
                                                                                            nat_add(a910, a911)?
                                                                                        }, 4u64))
                                                                                    } else {
                                                                                        let a912 = v3;
                                                                                        let a913 = 2u64;
                                                                                        if nat_eq(a912, a913) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a927 = 4u64;
                                                                                                let a928 = {
                                                                                                    let a917 = {
                                                                                                        let a914 = v1;
                                                                                                        let a915 = 32u64;
                                                                                                        let a916 = 0u64;
                                                                                                        nat_quot(a914, a915, a916)
                                                                                                    };
                                                                                                    let a918 = 4u64;
                                                                                                    if nat_lt(a917, a918) {
                                                                                                        let a919 = v1;
                                                                                                        let a920 = 32u64;
                                                                                                        let a921 = 0u64;
                                                                                                        nat_quot(a919, a920, a921)
                                                                                                    } else {
                                                                                                        let a925 = {
                                                                                                            let a922 = v1;
                                                                                                            let a923 = 32u64;
                                                                                                            let a924 = 0u64;
                                                                                                            nat_quot(a922, a923, a924)
                                                                                                        };
                                                                                                        let a926 = 4u64;
                                                                                                        nat_add(a925, a926)?
                                                                                                    }
                                                                                                };
                                                                                                nat_add(a927, a928)?
                                                                                            }, 5u64))
                                                                                        } else {
                                                                                            let a929 = v3;
                                                                                            let a930 = 3u64;
                                                                                            if nat_eq(a929, a930) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a934 = {
                                                                                                        let a931 = v1;
                                                                                                        let a932 = 32u64;
                                                                                                        let a933 = 0u64;
                                                                                                        nat_quot(a931, a932, a933)
                                                                                                    };
                                                                                                    let a935 = 4u64;
                                                                                                    if nat_lt(a934, a935) {
                                                                                                        let a936 = v7;
                                                                                                        let a937 = 64u64;
                                                                                                        let a938 = 0u64;
                                                                                                        nat_rem(a936, a937, a938)
                                                                                                    } else {
                                                                                                        let a947 = {
                                                                                                            let a939 = v9;
                                                                                                            let a940 = 16u64;
                                                                                                            let a941 = 0u64;
                                                                                                            nat_rem(a939, a940, a941)
                                                                                                        };
                                                                                                        let a948 = {
                                                                                                            let a945 = {
                                                                                                                let a942 = v7;
                                                                                                                let a943 = 64u64;
                                                                                                                let a944 = 0u64;
                                                                                                                nat_quot(a942, a943, a944)
                                                                                                            };
                                                                                                            let a946 = 16u64;
                                                                                                            nat_mul(a945, a946)?
                                                                                                        };
                                                                                                        nat_add(a947, a948)?
                                                                                                    }
                                                                                                }, 0u64, 2u64))
                                                                                            } else {
                                                                                                let a949 = v3;
                                                                                                let a950 = 4u64;
                                                                                                if nat_eq(a949, a950) {
                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                } else {
                                                                                                    let a951 = v3;
                                                                                                    let a952 = 5u64;
                                                                                                    if nat_eq(a951, a952) {
                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                            let a956 = {
                                                                                                                let a953 = v1;
                                                                                                                let a954 = 32u64;
                                                                                                                let a955 = 0u64;
                                                                                                                nat_quot(a953, a954, a955)
                                                                                                            };
                                                                                                            let a957 = 4u64;
                                                                                                            if nat_lt(a956, a957) {
                                                                                                                let a958 = v8;
                                                                                                                let a959 = 64u64;
                                                                                                                let a960 = 0u64;
                                                                                                                nat_rem(a958, a959, a960)
                                                                                                            } else {
                                                                                                                let a969 = {
                                                                                                                    let a961 = v9;
                                                                                                                    let a962 = 16u64;
                                                                                                                    let a963 = 0u64;
                                                                                                                    nat_quot(a961, a962, a963)
                                                                                                                };
                                                                                                                let a970 = {
                                                                                                                    let a967 = {
                                                                                                                        let a964 = v8;
                                                                                                                        let a965 = 64u64;
                                                                                                                        let a966 = 0u64;
                                                                                                                        nat_quot(a964, a965, a966)
                                                                                                                    };
                                                                                                                    let a968 = 16u64;
                                                                                                                    nat_mul(a967, a968)?
                                                                                                                };
                                                                                                                nat_add(a969, a970)?
                                                                                                            }
                                                                                                        }, 0u64, 6u64))
                                                                                                    } else {
                                                                                                        let a971 = v3;
                                                                                                        let a972 = 6u64;
                                                                                                        if nat_eq(a971, a972) {
                                                                                                            f4(2u64, v5, v10, 6u64)
                                                                                                        } else {
                                                                                                            let a973 = v3;
                                                                                                            let a974 = 7u64;
                                                                                                            if nat_eq(a973, a974) {
                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                    let a988 = {
                                                                                                                        let a983 = 48u64;
                                                                                                                        let a984 = {
                                                                                                                            let a981 = {
                                                                                                                                let a978 = {
                                                                                                                                    let a975 = v1;
                                                                                                                                    let a976 = 32u64;
                                                                                                                                    let a977 = 0u64;
                                                                                                                                    nat_quot(a975, a976, a977)
                                                                                                                                };
                                                                                                                                let a979 = 2u64;
                                                                                                                                let a980 = 0u64;
                                                                                                                                nat_quot(a978, a979, a980)
                                                                                                                            };
                                                                                                                            let a982 = 32u64;
                                                                                                                            nat_mul(a981, a982)?
                                                                                                                        };
                                                                                                                        nat_add(a983, a984)?
                                                                                                                    };
                                                                                                                    let a989 = {
                                                                                                                        let a985 = v1;
                                                                                                                        let a986 = 32u64;
                                                                                                                        let a987 = 0u64;
                                                                                                                        nat_rem(a985, a986, a987)
                                                                                                                    };
                                                                                                                    nat_add(a988, a989)?
                                                                                                                }, 3u64))
                                                                                                            } else {
                                                                                                                let a990 = v3;
                                                                                                                let a991 = 8u64;
                                                                                                                if nat_eq(a990, a991) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a995 = 16u64;
                                                                                                                        let a996 = {
                                                                                                                            let a992 = v1;
                                                                                                                            let a993 = 32u64;
                                                                                                                            let a994 = 0u64;
                                                                                                                            nat_rem(a992, a993, a994)
                                                                                                                        };
                                                                                                                        nat_add(a995, a996)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a997 = v3;
                                                                                                                    let a998 = 9u64;
                                                                                                                    if nat_eq(a997, a998) {
                                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                                            let a1024 = {
                                                                                                                                let a1010 = {
                                                                                                                                    let a1007 = v7;
                                                                                                                                    let a1008 = {
                                                                                                                                        let a1005 = {
                                                                                                                                            let a1002 = {
                                                                                                                                                let a999 = v1;
                                                                                                                                                let a1000 = 32u64;
                                                                                                                                                let a1001 = 0u64;
                                                                                                                                                nat_quot(a999, a1000, a1001)
                                                                                                                                            };
                                                                                                                                            let a1003 = 2u64;
                                                                                                                                            let a1004 = 0u64;
                                                                                                                                            nat_rem(a1002, a1003, a1004)
                                                                                                                                        };
                                                                                                                                        let a1006 = 0u64;
                                                                                                                                        if nat_eq(a1005, a1006) {
                                                                                                                                            1u64
                                                                                                                                        } else {
                                                                                                                                            16u64
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    let a1009 = 0u64;
                                                                                                                                    nat_quot(a1007, a1008, a1009)
                                                                                                                                };
                                                                                                                                let a1011 = 16u64;
                                                                                                                                let a1012 = 0u64;
                                                                                                                                nat_rem(a1010, a1011, a1012)
                                                                                                                            };
                                                                                                                            let a1025 = {
                                                                                                                                let a1022 = {
                                                                                                                                    let a1019 = {
                                                                                                                                        let a1016 = v8;
                                                                                                                                        let a1017 = f10({
                                                                                                                                            let a1013 = v1;
                                                                                                                                            let a1014 = 32u64;
                                                                                                                                            let a1015 = 0u64;
                                                                                                                                            nat_quot(a1013, a1014, a1015)
                                                                                                                                        });
                                                                                                                                        let a1018 = 0u64;
                                                                                                                                        nat_quot(a1016, a1017, a1018)
                                                                                                                                    };
                                                                                                                                    let a1020 = 2u64;
                                                                                                                                    let a1021 = 0u64;
                                                                                                                                    nat_rem(a1019, a1020, a1021)
                                                                                                                                };
                                                                                                                                let a1023 = 16u64;
                                                                                                                                nat_mul(a1022, a1023)?
                                                                                                                            };
                                                                                                                            nat_add(a1024, a1025)?
                                                                                                                        }, 0u64, 3u64))
                                                                                                                    } else {
                                                                                                                        let a1026 = v3;
                                                                                                                        let a1027 = 10u64;
                                                                                                                        if nat_eq(a1026, a1027) {
                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1028 = v3;
                                                                                                                            let a1029 = 11u64;
                                                                                                                            if nat_eq(a1028, a1029) {
                                                                                                                                f4(1u64, v6, v10, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1030 = v3;
                                                                                                                                let a1031 = 12u64;
                                                                                                                                if nat_eq(a1030, a1031) {
                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                        let a1032 = v1;
                                                                                                                                        let a1033 = v2;
                                                                                                                                        nat_mul(a1032, a1033)?
                                                                                                                                    }, v6))
                                                                                                                                } else {
                                                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a1034 = v0;
                                                                                let a1035 = 14u64;
                                                                                if nat_eq(a1034, a1035) {
                                                                                    let a1036 = v3;
                                                                                    let a1037 = 0u64;
                                                                                    if nat_eq(a1036, a1037) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a1061 = {
                                                                                                let a1043 = 192u64;
                                                                                                let a1044 = {
                                                                                                    let a1041 = 8u64;
                                                                                                    let a1042 = {
                                                                                                        let a1038 = v1;
                                                                                                        let a1039 = 128u64;
                                                                                                        let a1040 = 0u64;
                                                                                                        nat_quot(a1038, a1039, a1040)
                                                                                                    };
                                                                                                    nat_mul(a1041, a1042)?
                                                                                                };
                                                                                                nat_add(a1043, a1044)?
                                                                                            };
                                                                                            let a1062 = {
                                                                                                let a1059 = {
                                                                                                    let a1051 = 2u64;
                                                                                                    let a1052 = {
                                                                                                        let a1048 = {
                                                                                                            let a1045 = v1;
                                                                                                            let a1046 = 128u64;
                                                                                                            let a1047 = 0u64;
                                                                                                            nat_rem(a1045, a1046, a1047)
                                                                                                        };
                                                                                                        let a1049 = 32u64;
                                                                                                        let a1050 = 0u64;
                                                                                                        nat_quot(a1048, a1049, a1050)
                                                                                                    };
                                                                                                    nat_mul(a1051, a1052)?
                                                                                                };
                                                                                                let a1060 = {
                                                                                                    let a1056 = {
                                                                                                        let a1053 = v1;
                                                                                                        let a1054 = 32u64;
                                                                                                        let a1055 = 0u64;
                                                                                                        nat_rem(a1053, a1054, a1055)
                                                                                                    };
                                                                                                    let a1057 = 16u64;
                                                                                                    let a1058 = 0u64;
                                                                                                    nat_quot(a1056, a1057, a1058)
                                                                                                };
                                                                                                nat_add(a1059, a1060)?
                                                                                            };
                                                                                            nat_add(a1061, a1062)?
                                                                                        }, 3u64))
                                                                                    } else {
                                                                                        let a1063 = v3;
                                                                                        let a1064 = 1u64;
                                                                                        if nat_eq(a1063, a1064) {
                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                let a1065 = v7;
                                                                                                let a1066 = 128u64;
                                                                                                if nat_lt(a1065, a1066) {
                                                                                                    0u64
                                                                                                } else {
                                                                                                    256u64
                                                                                                }
                                                                                            }, 2u64))
                                                                                        } else {
                                                                                            let a1067 = v3;
                                                                                            let a1068 = 2u64;
                                                                                            if nat_eq(a1067, a1068) {
                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                            } else {
                                                                                                let a1069 = v3;
                                                                                                let a1070 = 3u64;
                                                                                                if nat_eq(a1069, a1070) {
                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                        let a1092 = {
                                                                                                            let a1074 = 64u64;
                                                                                                            let a1075 = {
                                                                                                                let a1071 = v1;
                                                                                                                let a1072 = 128u64;
                                                                                                                let a1073 = 0u64;
                                                                                                                nat_quot(a1071, a1072, a1073)
                                                                                                            };
                                                                                                            nat_mul(a1074, a1075)?
                                                                                                        };
                                                                                                        let a1093 = {
                                                                                                            let a1090 = {
                                                                                                                let a1085 = 32u64;
                                                                                                                let a1086 = {
                                                                                                                    let a1082 = {
                                                                                                                        let a1079 = {
                                                                                                                            let a1076 = v1;
                                                                                                                            let a1077 = 128u64;
                                                                                                                            let a1078 = 0u64;
                                                                                                                            nat_rem(a1076, a1077, a1078)
                                                                                                                        };
                                                                                                                        let a1080 = 32u64;
                                                                                                                        let a1081 = 0u64;
                                                                                                                        nat_quot(a1079, a1080, a1081)
                                                                                                                    };
                                                                                                                    let a1083 = 2u64;
                                                                                                                    let a1084 = 0u64;
                                                                                                                    nat_rem(a1082, a1083, a1084)
                                                                                                                };
                                                                                                                nat_mul(a1085, a1086)?
                                                                                                            };
                                                                                                            let a1091 = {
                                                                                                                let a1087 = v1;
                                                                                                                let a1088 = 32u64;
                                                                                                                let a1089 = 0u64;
                                                                                                                nat_rem(a1087, a1088, a1089)
                                                                                                            };
                                                                                                            nat_add(a1090, a1091)?
                                                                                                        };
                                                                                                        nat_add(a1092, a1093)?
                                                                                                    }, 3u64))
                                                                                                } else {
                                                                                                    let a1094 = v3;
                                                                                                    let a1095 = 4u64;
                                                                                                    if nat_eq(a1094, a1095) {
                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                            let a1106 = 128u64;
                                                                                                            let a1107 = {
                                                                                                                let a1104 = {
                                                                                                                    let a1099 = 32u64;
                                                                                                                    let a1100 = {
                                                                                                                        let a1096 = v1;
                                                                                                                        let a1097 = 128u64;
                                                                                                                        let a1098 = 0u64;
                                                                                                                        nat_quot(a1096, a1097, a1098)
                                                                                                                    };
                                                                                                                    nat_mul(a1099, a1100)?
                                                                                                                };
                                                                                                                let a1105 = {
                                                                                                                    let a1101 = v1;
                                                                                                                    let a1102 = 32u64;
                                                                                                                    let a1103 = 0u64;
                                                                                                                    nat_rem(a1101, a1102, a1103)
                                                                                                                };
                                                                                                                nat_add(a1104, a1105)?
                                                                                                            };
                                                                                                            nat_add(a1106, a1107)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1108 = v3;
                                                                                                        let a1109 = 5u64;
                                                                                                        if nat_eq(a1108, a1109) {
                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                let a1143 = {
                                                                                                                    let a1124 = {
                                                                                                                        let a1121 = v7;
                                                                                                                        let a1122 = {
                                                                                                                            let a1119 = {
                                                                                                                                let a1116 = {
                                                                                                                                    let a1113 = {
                                                                                                                                        let a1110 = v1;
                                                                                                                                        let a1111 = 128u64;
                                                                                                                                        let a1112 = 0u64;
                                                                                                                                        nat_rem(a1110, a1111, a1112)
                                                                                                                                    };
                                                                                                                                    let a1114 = 32u64;
                                                                                                                                    let a1115 = 0u64;
                                                                                                                                    nat_quot(a1113, a1114, a1115)
                                                                                                                                };
                                                                                                                                let a1117 = 2u64;
                                                                                                                                let a1118 = 0u64;
                                                                                                                                nat_quot(a1116, a1117, a1118)
                                                                                                                            };
                                                                                                                            let a1120 = 0u64;
                                                                                                                            if nat_eq(a1119, a1120) {
                                                                                                                                1u64
                                                                                                                            } else {
                                                                                                                                16u64
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a1123 = 0u64;
                                                                                                                        nat_quot(a1121, a1122, a1123)
                                                                                                                    };
                                                                                                                    let a1125 = 16u64;
                                                                                                                    let a1126 = 0u64;
                                                                                                                    nat_rem(a1124, a1125, a1126)
                                                                                                                };
                                                                                                                let a1144 = {
                                                                                                                    let a1141 = {
                                                                                                                        let a1138 = {
                                                                                                                            let a1135 = v8;
                                                                                                                            let a1136 = f10({
                                                                                                                                let a1133 = 2u64;
                                                                                                                                let a1134 = {
                                                                                                                                    let a1130 = {
                                                                                                                                        let a1127 = v1;
                                                                                                                                        let a1128 = 128u64;
                                                                                                                                        let a1129 = 0u64;
                                                                                                                                        nat_rem(a1127, a1128, a1129)
                                                                                                                                    };
                                                                                                                                    let a1131 = 32u64;
                                                                                                                                    let a1132 = 0u64;
                                                                                                                                    nat_quot(a1130, a1131, a1132)
                                                                                                                                };
                                                                                                                                nat_mul(a1133, a1134)?
                                                                                                                            });
                                                                                                                            let a1137 = 0u64;
                                                                                                                            nat_quot(a1135, a1136, a1137)
                                                                                                                        };
                                                                                                                        let a1139 = 4u64;
                                                                                                                        let a1140 = 0u64;
                                                                                                                        nat_rem(a1138, a1139, a1140)
                                                                                                                    };
                                                                                                                    let a1142 = 16u64;
                                                                                                                    nat_mul(a1141, a1142)?
                                                                                                                };
                                                                                                                nat_add(a1143, a1144)?
                                                                                                            }, 32u64, 3u64))
                                                                                                        } else {
                                                                                                            let a1145 = v3;
                                                                                                            let a1146 = 6u64;
                                                                                                            if nat_eq(a1145, a1146) {
                                                                                                                f4(2u64, v6, v7, 2u64)
                                                                                                            } else {
                                                                                                                let a1147 = v3;
                                                                                                                let a1148 = 7u64;
                                                                                                                if nat_eq(a1147, a1148) {
                                                                                                                    Ok(Adt1::C4({
                                                                                                                        let a1149 = v1;
                                                                                                                        let a1150 = v2;
                                                                                                                        nat_mul(a1149, a1150)?
                                                                                                                    }, v6))
                                                                                                                } else {
                                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    let a1151 = v0;
                                                                                    let a1152 = 34u64;
                                                                                    if nat_eq(a1151, a1152) {
                                                                                        let a1153 = v3;
                                                                                        let a1154 = 0u64;
                                                                                        if nat_eq(a1153, a1154) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a1155 = v1;
                                                                                                let a1156 = 160u64;
                                                                                                if nat_lt(a1155, a1156) {
                                                                                                    let a1157 = v1;
                                                                                                    let a1158 = 32u64;
                                                                                                    let a1159 = 0u64;
                                                                                                    nat_rem(a1157, a1158, a1159)
                                                                                                } else {
                                                                                                    let a1160 = v1;
                                                                                                    let a1161 = 240u64;
                                                                                                    if nat_lt(a1160, a1161) {
                                                                                                        let a1167 = 32u64;
                                                                                                        let a1168 = {
                                                                                                            let a1164 = {
                                                                                                                let a1162 = v1;
                                                                                                                let a1163 = 160u64;
                                                                                                                nat_sub(a1162, a1163)
                                                                                                            };
                                                                                                            let a1165 = 16u64;
                                                                                                            let a1166 = 0u64;
                                                                                                            nat_rem(a1164, a1165, a1166)
                                                                                                        };
                                                                                                        nat_add(a1167, a1168)?
                                                                                                    } else {
                                                                                                        let a1174 = 48u64;
                                                                                                        let a1175 = {
                                                                                                            let a1171 = {
                                                                                                                let a1169 = v1;
                                                                                                                let a1170 = 240u64;
                                                                                                                nat_sub(a1169, a1170)
                                                                                                            };
                                                                                                            let a1172 = 4u64;
                                                                                                            let a1173 = 0u64;
                                                                                                            nat_rem(a1171, a1172, a1173)
                                                                                                        };
                                                                                                        nat_add(a1174, a1175)?
                                                                                                    }
                                                                                                }
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a1176 = v3;
                                                                                            let a1177 = 1u64;
                                                                                            if nat_eq(a1176, a1177) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a1261 = {
                                                                                                        let a1259 = {
                                                                                                            let a1256 = {
                                                                                                                let a1254 = v7;
                                                                                                                let a1255 = {
                                                                                                                    let a1195 = {
                                                                                                                        let a1178 = v1;
                                                                                                                        let a1179 = 160u64;
                                                                                                                        if nat_lt(a1178, a1179) {
                                                                                                                            let a1180 = v1;
                                                                                                                            let a1181 = 32u64;
                                                                                                                            let a1182 = 0u64;
                                                                                                                            nat_quot(a1180, a1181, a1182)
                                                                                                                        } else {
                                                                                                                            let a1183 = v1;
                                                                                                                            let a1184 = 240u64;
                                                                                                                            if nat_lt(a1183, a1184) {
                                                                                                                                let a1187 = {
                                                                                                                                    let a1185 = v1;
                                                                                                                                    let a1186 = 160u64;
                                                                                                                                    nat_sub(a1185, a1186)
                                                                                                                                };
                                                                                                                                let a1188 = 16u64;
                                                                                                                                let a1189 = 0u64;
                                                                                                                                nat_quot(a1187, a1188, a1189)
                                                                                                                            } else {
                                                                                                                                let a1192 = {
                                                                                                                                    let a1190 = v1;
                                                                                                                                    let a1191 = 240u64;
                                                                                                                                    nat_sub(a1190, a1191)
                                                                                                                                };
                                                                                                                                let a1193 = 4u64;
                                                                                                                                let a1194 = 0u64;
                                                                                                                                nat_quot(a1192, a1193, a1194)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    };
                                                                                                                    let a1196 = 0u64;
                                                                                                                    if nat_eq(a1195, a1196) {
                                                                                                                        1u64
                                                                                                                    } else {
                                                                                                                        let a1214 = {
                                                                                                                            let a1197 = v1;
                                                                                                                            let a1198 = 160u64;
                                                                                                                            if nat_lt(a1197, a1198) {
                                                                                                                                let a1199 = v1;
                                                                                                                                let a1200 = 32u64;
                                                                                                                                let a1201 = 0u64;
                                                                                                                                nat_quot(a1199, a1200, a1201)
                                                                                                                            } else {
                                                                                                                                let a1202 = v1;
                                                                                                                                let a1203 = 240u64;
                                                                                                                                if nat_lt(a1202, a1203) {
                                                                                                                                    let a1206 = {
                                                                                                                                        let a1204 = v1;
                                                                                                                                        let a1205 = 160u64;
                                                                                                                                        nat_sub(a1204, a1205)
                                                                                                                                    };
                                                                                                                                    let a1207 = 16u64;
                                                                                                                                    let a1208 = 0u64;
                                                                                                                                    nat_quot(a1206, a1207, a1208)
                                                                                                                                } else {
                                                                                                                                    let a1211 = {
                                                                                                                                        let a1209 = v1;
                                                                                                                                        let a1210 = 240u64;
                                                                                                                                        nat_sub(a1209, a1210)
                                                                                                                                    };
                                                                                                                                    let a1212 = 4u64;
                                                                                                                                    let a1213 = 0u64;
                                                                                                                                    nat_quot(a1211, a1212, a1213)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a1215 = 1u64;
                                                                                                                        if nat_eq(a1214, a1215) {
                                                                                                                            3u64
                                                                                                                        } else {
                                                                                                                            let a1233 = {
                                                                                                                                let a1216 = v1;
                                                                                                                                let a1217 = 160u64;
                                                                                                                                if nat_lt(a1216, a1217) {
                                                                                                                                    let a1218 = v1;
                                                                                                                                    let a1219 = 32u64;
                                                                                                                                    let a1220 = 0u64;
                                                                                                                                    nat_quot(a1218, a1219, a1220)
                                                                                                                                } else {
                                                                                                                                    let a1221 = v1;
                                                                                                                                    let a1222 = 240u64;
                                                                                                                                    if nat_lt(a1221, a1222) {
                                                                                                                                        let a1225 = {
                                                                                                                                            let a1223 = v1;
                                                                                                                                            let a1224 = 160u64;
                                                                                                                                            nat_sub(a1223, a1224)
                                                                                                                                        };
                                                                                                                                        let a1226 = 16u64;
                                                                                                                                        let a1227 = 0u64;
                                                                                                                                        nat_quot(a1225, a1226, a1227)
                                                                                                                                    } else {
                                                                                                                                        let a1230 = {
                                                                                                                                            let a1228 = v1;
                                                                                                                                            let a1229 = 240u64;
                                                                                                                                            nat_sub(a1228, a1229)
                                                                                                                                        };
                                                                                                                                        let a1231 = 4u64;
                                                                                                                                        let a1232 = 0u64;
                                                                                                                                        nat_quot(a1230, a1231, a1232)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            let a1234 = 2u64;
                                                                                                                            if nat_eq(a1233, a1234) {
                                                                                                                                9u64
                                                                                                                            } else {
                                                                                                                                let a1252 = {
                                                                                                                                    let a1235 = v1;
                                                                                                                                    let a1236 = 160u64;
                                                                                                                                    if nat_lt(a1235, a1236) {
                                                                                                                                        let a1237 = v1;
                                                                                                                                        let a1238 = 32u64;
                                                                                                                                        let a1239 = 0u64;
                                                                                                                                        nat_quot(a1237, a1238, a1239)
                                                                                                                                    } else {
                                                                                                                                        let a1240 = v1;
                                                                                                                                        let a1241 = 240u64;
                                                                                                                                        if nat_lt(a1240, a1241) {
                                                                                                                                            let a1244 = {
                                                                                                                                                let a1242 = v1;
                                                                                                                                                let a1243 = 160u64;
                                                                                                                                                nat_sub(a1242, a1243)
                                                                                                                                            };
                                                                                                                                            let a1245 = 16u64;
                                                                                                                                            let a1246 = 0u64;
                                                                                                                                            nat_quot(a1244, a1245, a1246)
                                                                                                                                        } else {
                                                                                                                                            let a1249 = {
                                                                                                                                                let a1247 = v1;
                                                                                                                                                let a1248 = 240u64;
                                                                                                                                                nat_sub(a1247, a1248)
                                                                                                                                            };
                                                                                                                                            let a1250 = 4u64;
                                                                                                                                            let a1251 = 0u64;
                                                                                                                                            nat_quot(a1249, a1250, a1251)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let a1253 = 3u64;
                                                                                                                                if nat_eq(a1252, a1253) {
                                                                                                                                    27u64
                                                                                                                                } else {
                                                                                                                                    81u64
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                nat_mul(a1254, a1255)?
                                                                                                            };
                                                                                                            let a1257 = 256u64;
                                                                                                            let a1258 = 0u64;
                                                                                                            nat_rem(a1256, a1257, a1258)
                                                                                                        };
                                                                                                        let a1260 = 3u64;
                                                                                                        nat_mul(a1259, a1260)?
                                                                                                    };
                                                                                                    let a1262 = 256u64;
                                                                                                    let a1263 = 0u64;
                                                                                                    nat_quot(a1261, a1262, a1263)
                                                                                                }, 1u64, 2u64))
                                                                                            } else {
                                                                                                let a1264 = v3;
                                                                                                let a1265 = 2u64;
                                                                                                if nat_eq(a1264, a1265) {
                                                                                                    f4(2u64, v6, v4, 2u64)
                                                                                                } else {
                                                                                                    let a1266 = v3;
                                                                                                    let a1267 = 3u64;
                                                                                                    if nat_eq(a1266, a1267) {
                                                                                                        Ok(Adt1::C4({
                                                                                                            let a1268 = v1;
                                                                                                            let a1269 = v2;
                                                                                                            nat_mul(a1268, a1269)?
                                                                                                        }, v6))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C0(0u64))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let a1270 = v0;
                                                                                        let a1271 = 35u64;
                                                                                        if nat_eq(a1270, a1271) {
                                                                                            let a1272 = v3;
                                                                                            let a1273 = 0u64;
                                                                                            if nat_eq(a1272, a1273) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                    let a1282 = {
                                                                                                        let a1277 = 32u64;
                                                                                                        let a1278 = {
                                                                                                            let a1274 = v1;
                                                                                                            let a1275 = 128u64;
                                                                                                            let a1276 = 0u64;
                                                                                                            nat_quot(a1274, a1275, a1276)
                                                                                                        };
                                                                                                        nat_mul(a1277, a1278)?
                                                                                                    };
                                                                                                    let a1283 = {
                                                                                                        let a1279 = v1;
                                                                                                        let a1280 = 32u64;
                                                                                                        let a1281 = 0u64;
                                                                                                        nat_rem(a1279, a1280, a1281)
                                                                                                    };
                                                                                                    nat_add(a1282, a1283)?
                                                                                                }, 3u64))
                                                                                            } else {
                                                                                                let a1284 = v3;
                                                                                                let a1285 = 1u64;
                                                                                                if nat_eq(a1284, a1285) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a1297 = {
                                                                                                            let a1294 = v7;
                                                                                                            let a1295 = f10({
                                                                                                                let a1292 = 2u64;
                                                                                                                let a1293 = {
                                                                                                                    let a1289 = {
                                                                                                                        let a1286 = v1;
                                                                                                                        let a1287 = 128u64;
                                                                                                                        let a1288 = 0u64;
                                                                                                                        nat_rem(a1286, a1287, a1288)
                                                                                                                    };
                                                                                                                    let a1290 = 32u64;
                                                                                                                    let a1291 = 0u64;
                                                                                                                    nat_quot(a1289, a1290, a1291)
                                                                                                                };
                                                                                                                nat_mul(a1292, a1293)?
                                                                                                            });
                                                                                                            let a1296 = 0u64;
                                                                                                            nat_quot(a1294, a1295, a1296)
                                                                                                        };
                                                                                                        let a1298 = 4u64;
                                                                                                        let a1299 = 0u64;
                                                                                                        nat_rem(a1297, a1298, a1299)
                                                                                                    }, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1300 = v3;
                                                                                                    let a1301 = 2u64;
                                                                                                    if nat_eq(a1300, a1301) {
                                                                                                        f4(2u64, v6, v4, 2u64)
                                                                                                    } else {
                                                                                                        let a1302 = v3;
                                                                                                        let a1303 = 3u64;
                                                                                                        if nat_eq(a1302, a1303) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a1304 = v1;
                                                                                                                let a1305 = v2;
                                                                                                                nat_mul(a1304, a1305)?
                                                                                                            }, v6))
                                                                                                        } else {
                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            let a1306 = v0;
                                                                                            let a1307 = 16u64;
                                                                                            if nat_eq(a1306, a1307) {
                                                                                                let a1308 = v3;
                                                                                                let a1309 = 0u64;
                                                                                                if nat_eq(a1308, a1309) {
                                                                                                    Ok(Adt1::C2(0u64, 4u64, {
                                                                                                        let a1315 = 6u64;
                                                                                                        let a1316 = {
                                                                                                            let a1313 = {
                                                                                                                let a1310 = v1;
                                                                                                                let a1311 = 32u64;
                                                                                                                let a1312 = 0u64;
                                                                                                                nat_quot(a1310, a1311, a1312)
                                                                                                            };
                                                                                                            let a1314 = 8u64;
                                                                                                            nat_mul(a1313, a1314)?
                                                                                                        };
                                                                                                        nat_add(a1315, a1316)?
                                                                                                    }, 5u64))
                                                                                                } else {
                                                                                                    let a1317 = v3;
                                                                                                    let a1318 = 1u64;
                                                                                                    if nat_eq(a1317, a1318) {
                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                            let a1332 = 2u64;
                                                                                                            let a1333 = {
                                                                                                                let a1330 = {
                                                                                                                    let a1322 = {
                                                                                                                        let a1319 = v1;
                                                                                                                        let a1320 = 32u64;
                                                                                                                        let a1321 = 0u64;
                                                                                                                        nat_quot(a1319, a1320, a1321)
                                                                                                                    };
                                                                                                                    let a1323 = 8u64;
                                                                                                                    nat_mul(a1322, a1323)?
                                                                                                                };
                                                                                                                let a1331 = {
                                                                                                                    let a1327 = {
                                                                                                                        let a1324 = v1;
                                                                                                                        let a1325 = 32u64;
                                                                                                                        let a1326 = 0u64;
                                                                                                                        nat_rem(a1324, a1325, a1326)
                                                                                                                    };
                                                                                                                    let a1328 = 8u64;
                                                                                                                    let a1329 = 0u64;
                                                                                                                    nat_quot(a1327, a1328, a1329)
                                                                                                                };
                                                                                                                nat_add(a1330, a1331)?
                                                                                                            };
                                                                                                            nat_add(a1332, a1333)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1334 = v3;
                                                                                                        let a1335 = 2u64;
                                                                                                        if nat_eq(a1334, a1335) {
                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                let a1350 = 8u64;
                                                                                                                let a1351 = {
                                                                                                                    let a1347 = {
                                                                                                                        let a1344 = v9;
                                                                                                                        let a1345 = f10({
                                                                                                                            let a1342 = {
                                                                                                                                let a1339 = {
                                                                                                                                    let a1336 = v1;
                                                                                                                                    let a1337 = 32u64;
                                                                                                                                    let a1338 = 0u64;
                                                                                                                                    nat_rem(a1336, a1337, a1338)
                                                                                                                                };
                                                                                                                                let a1340 = 8u64;
                                                                                                                                let a1341 = 0u64;
                                                                                                                                nat_quot(a1339, a1340, a1341)
                                                                                                                            };
                                                                                                                            let a1343 = 7u64;
                                                                                                                            nat_mul(a1342, a1343)?
                                                                                                                        });
                                                                                                                        let a1346 = 0u64;
                                                                                                                        nat_quot(a1344, a1345, a1346)
                                                                                                                    };
                                                                                                                    let a1348 = 128u64;
                                                                                                                    let a1349 = 0u64;
                                                                                                                    nat_rem(a1347, a1348, a1349)
                                                                                                                };
                                                                                                                nat_add(a1350, a1351)?
                                                                                                            }, 7u64))
                                                                                                        } else {
                                                                                                            let a1352 = v3;
                                                                                                            let a1353 = 3u64;
                                                                                                            if nat_eq(a1352, a1353) {
                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                    let a1354 = v9;
                                                                                                                    let a1355 = 268435456u64;
                                                                                                                    let a1356 = 0u64;
                                                                                                                    nat_quot(a1354, a1355, a1356)
                                                                                                                }, 0u64, 2u64))
                                                                                                            } else {
                                                                                                                let a1357 = v3;
                                                                                                                let a1358 = 4u64;
                                                                                                                if nat_eq(a1357, a1358) {
                                                                                                                    f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                } else {
                                                                                                                    let a1359 = v3;
                                                                                                                    let a1360 = 5u64;
                                                                                                                    if nat_eq(a1359, a1360) {
                                                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                                                    } else {
                                                                                                                        let a1361 = v3;
                                                                                                                        let a1362 = 6u64;
                                                                                                                        if nat_eq(a1361, a1362) {
                                                                                                                            f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1363 = v3;
                                                                                                                            let a1364 = 7u64;
                                                                                                                            if nat_eq(a1363, a1364) {
                                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                    let a1372 = 1160u64;
                                                                                                                                    let a1373 = {
                                                                                                                                        let a1370 = {
                                                                                                                                            let a1365 = v8;
                                                                                                                                            let a1366 = 8u64;
                                                                                                                                            nat_mul(a1365, a1366)?
                                                                                                                                        };
                                                                                                                                        let a1371 = {
                                                                                                                                            let a1367 = v1;
                                                                                                                                            let a1368 = 8u64;
                                                                                                                                            let a1369 = 0u64;
                                                                                                                                            nat_rem(a1367, a1368, a1369)
                                                                                                                                        };
                                                                                                                                        nat_add(a1370, a1371)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1372, a1373)?
                                                                                                                                }, 3u64))
                                                                                                                            } else {
                                                                                                                                let a1374 = v3;
                                                                                                                                let a1375 = 8u64;
                                                                                                                                if nat_eq(a1374, a1375) {
                                                                                                                                    Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a1376 = v3;
                                                                                                                                    let a1377 = 9u64;
                                                                                                                                    if nat_eq(a1376, a1377) {
                                                                                                                                        f4(2u64, v6, v7, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1378 = v3;
                                                                                                                                        let a1379 = 10u64;
                                                                                                                                        if nat_eq(a1378, a1379) {
                                                                                                                                            Ok(f11(v6, {
                                                                                                                                                let a1389 = {
                                                                                                                                                    let a1386 = {
                                                                                                                                                        let a1383 = v11;
                                                                                                                                                        let a1384 = f10({
                                                                                                                                                            let a1380 = v1;
                                                                                                                                                            let a1381 = 8u64;
                                                                                                                                                            let a1382 = 0u64;
                                                                                                                                                            nat_rem(a1380, a1381, a1382)
                                                                                                                                                        });
                                                                                                                                                        let a1385 = 0u64;
                                                                                                                                                        nat_quot(a1383, a1384, a1385)
                                                                                                                                                    };
                                                                                                                                                    let a1387 = 2u64;
                                                                                                                                                    let a1388 = 0u64;
                                                                                                                                                    nat_rem(a1386, a1387, a1388)
                                                                                                                                                };
                                                                                                                                                let a1390 = 1u64;
                                                                                                                                                nat_eq(a1389, a1390)
                                                                                                                                            }, 2u64))
                                                                                                                                        } else {
                                                                                                                                            let a1391 = v3;
                                                                                                                                            let a1392 = 11u64;
                                                                                                                                            if nat_eq(a1391, a1392) {
                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                    let a1393 = v1;
                                                                                                                                                    let a1394 = v2;
                                                                                                                                                    nat_mul(a1393, a1394)?
                                                                                                                                                }, v6))
                                                                                                                                            } else {
                                                                                                                                                Ok(Adt1::C0(0u64))
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                let a1395 = v0;
                                                                                                let a1396 = 17u64;
                                                                                                if nat_eq(a1395, a1396) {
                                                                                                    let a1397 = v3;
                                                                                                    let a1398 = 0u64;
                                                                                                    if nat_eq(a1397, a1398) {
                                                                                                        Ok(Adt1::C2(0u64, 2u64, {
                                                                                                            let a1414 = 2u64;
                                                                                                            let a1415 = {
                                                                                                                let a1412 = {
                                                                                                                    let a1402 = {
                                                                                                                        let a1399 = v1;
                                                                                                                        let a1400 = 32u64;
                                                                                                                        let a1401 = 0u64;
                                                                                                                        nat_quot(a1399, a1400, a1401)
                                                                                                                    };
                                                                                                                    let a1403 = 8u64;
                                                                                                                    nat_mul(a1402, a1403)?
                                                                                                                };
                                                                                                                let a1413 = {
                                                                                                                    let a1410 = {
                                                                                                                        let a1407 = {
                                                                                                                            let a1404 = v1;
                                                                                                                            let a1405 = 32u64;
                                                                                                                            let a1406 = 0u64;
                                                                                                                            nat_rem(a1404, a1405, a1406)
                                                                                                                        };
                                                                                                                        let a1408 = 8u64;
                                                                                                                        let a1409 = 0u64;
                                                                                                                        nat_quot(a1407, a1408, a1409)
                                                                                                                    };
                                                                                                                    let a1411 = 2u64;
                                                                                                                    nat_mul(a1410, a1411)?
                                                                                                                };
                                                                                                                nat_add(a1412, a1413)?
                                                                                                            };
                                                                                                            nat_add(a1414, a1415)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1416 = v3;
                                                                                                        let a1417 = 1u64;
                                                                                                        if nat_eq(a1416, a1417) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a1421 = 66u64;
                                                                                                                let a1422 = {
                                                                                                                    let a1418 = v1;
                                                                                                                    let a1419 = 32u64;
                                                                                                                    let a1420 = 0u64;
                                                                                                                    nat_quot(a1418, a1419, a1420)
                                                                                                                };
                                                                                                                nat_add(a1421, a1422)?
                                                                                                            }, 5u64))
                                                                                                        } else {
                                                                                                            let a1423 = v3;
                                                                                                            let a1424 = 2u64;
                                                                                                            if nat_eq(a1423, a1424) {
                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                    let a1428 = 8u64;
                                                                                                                    let a1429 = {
                                                                                                                        let a1425 = v8;
                                                                                                                        let a1426 = 512u64;
                                                                                                                        let a1427 = 0u64;
                                                                                                                        nat_quot(a1425, a1426, a1427)
                                                                                                                    };
                                                                                                                    nat_add(a1428, a1429)?
                                                                                                                }, 7u64))
                                                                                                            } else {
                                                                                                                let a1430 = v3;
                                                                                                                let a1431 = 3u64;
                                                                                                                if nat_eq(a1430, a1431) {
                                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                                        let a1446 = {
                                                                                                                            let a1443 = v9;
                                                                                                                            let a1444 = {
                                                                                                                                let a1441 = {
                                                                                                                                    let a1438 = {
                                                                                                                                        let a1435 = {
                                                                                                                                            let a1432 = v1;
                                                                                                                                            let a1433 = 32u64;
                                                                                                                                            let a1434 = 0u64;
                                                                                                                                            nat_rem(a1432, a1433, a1434)
                                                                                                                                        };
                                                                                                                                        let a1436 = 8u64;
                                                                                                                                        let a1437 = 0u64;
                                                                                                                                        nat_quot(a1435, a1436, a1437)
                                                                                                                                    };
                                                                                                                                    let a1439 = 2u64;
                                                                                                                                    let a1440 = 0u64;
                                                                                                                                    nat_quot(a1438, a1439, a1440)
                                                                                                                                };
                                                                                                                                let a1442 = 0u64;
                                                                                                                                if nat_eq(a1441, a1442) {
                                                                                                                                    1u64
                                                                                                                                } else {
                                                                                                                                    16u64
                                                                                                                                }
                                                                                                                            };
                                                                                                                            let a1445 = 0u64;
                                                                                                                            nat_quot(a1443, a1444, a1445)
                                                                                                                        };
                                                                                                                        let a1447 = 16u64;
                                                                                                                        let a1448 = 0u64;
                                                                                                                        nat_rem(a1446, a1447, a1448)
                                                                                                                    }, 0u64, 2u64))
                                                                                                                } else {
                                                                                                                    let a1449 = v3;
                                                                                                                    let a1450 = 4u64;
                                                                                                                    if nat_eq(a1449, a1450) {
                                                                                                                        f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                    } else {
                                                                                                                        let a1451 = v3;
                                                                                                                        let a1452 = 5u64;
                                                                                                                        if nat_eq(a1451, a1452) {
                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1453 = v3;
                                                                                                                            let a1454 = 6u64;
                                                                                                                            if nat_eq(a1453, a1454) {
                                                                                                                                f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1455 = v3;
                                                                                                                                let a1456 = 7u64;
                                                                                                                                if nat_eq(a1455, a1456) {
                                                                                                                                    Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                        let a1467 = 3208u64;
                                                                                                                                        let a1468 = {
                                                                                                                                            let a1465 = {
                                                                                                                                                let a1460 = {
                                                                                                                                                    let a1457 = v8;
                                                                                                                                                    let a1458 = 512u64;
                                                                                                                                                    let a1459 = 0u64;
                                                                                                                                                    nat_rem(a1457, a1458, a1459)
                                                                                                                                                };
                                                                                                                                                let a1461 = 8u64;
                                                                                                                                                nat_mul(a1460, a1461)?
                                                                                                                                            };
                                                                                                                                            let a1466 = {
                                                                                                                                                let a1462 = v1;
                                                                                                                                                let a1463 = 8u64;
                                                                                                                                                let a1464 = 0u64;
                                                                                                                                                nat_rem(a1462, a1463, a1464)
                                                                                                                                            };
                                                                                                                                            nat_add(a1465, a1466)?
                                                                                                                                        };
                                                                                                                                        nat_add(a1467, a1468)?
                                                                                                                                    }, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a1469 = v3;
                                                                                                                                    let a1470 = 8u64;
                                                                                                                                    if nat_eq(a1469, a1470) {
                                                                                                                                        Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                    } else {
                                                                                                                                        let a1471 = v3;
                                                                                                                                        let a1472 = 9u64;
                                                                                                                                        if nat_eq(a1471, a1472) {
                                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a1473 = v3;
                                                                                                                                            let a1474 = 10u64;
                                                                                                                                            if nat_eq(a1473, a1474) {
                                                                                                                                                Ok(f11(v6, {
                                                                                                                                                    let a1484 = {
                                                                                                                                                        let a1481 = {
                                                                                                                                                            let a1478 = v11;
                                                                                                                                                            let a1479 = f10({
                                                                                                                                                                let a1475 = v1;
                                                                                                                                                                let a1476 = 8u64;
                                                                                                                                                                let a1477 = 0u64;
                                                                                                                                                                nat_rem(a1475, a1476, a1477)
                                                                                                                                                            });
                                                                                                                                                            let a1480 = 0u64;
                                                                                                                                                            nat_quot(a1478, a1479, a1480)
                                                                                                                                                        };
                                                                                                                                                        let a1482 = 2u64;
                                                                                                                                                        let a1483 = 0u64;
                                                                                                                                                        nat_rem(a1481, a1482, a1483)
                                                                                                                                                    };
                                                                                                                                                    let a1485 = 1u64;
                                                                                                                                                    nat_eq(a1484, a1485)
                                                                                                                                                }, 2u64))
                                                                                                                                            } else {
                                                                                                                                                let a1486 = v3;
                                                                                                                                                let a1487 = 11u64;
                                                                                                                                                if nat_eq(a1486, a1487) {
                                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                                        let a1488 = v1;
                                                                                                                                                        let a1489 = v2;
                                                                                                                                                        nat_mul(a1488, a1489)?
                                                                                                                                                    }, v6))
                                                                                                                                                } else {
                                                                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    let a1490 = v0;
                                                                                                    let a1491 = 22u64;
                                                                                                    if nat_eq(a1490, a1491) {
                                                                                                        let a1492 = v3;
                                                                                                        let a1493 = 0u64;
                                                                                                        if nat_eq(a1492, a1493) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a1507 = 2u64;
                                                                                                                let a1508 = {
                                                                                                                    let a1505 = {
                                                                                                                        let a1497 = {
                                                                                                                            let a1494 = v1;
                                                                                                                            let a1495 = 32u64;
                                                                                                                            let a1496 = 0u64;
                                                                                                                            nat_quot(a1494, a1495, a1496)
                                                                                                                        };
                                                                                                                        let a1498 = 4u64;
                                                                                                                        nat_mul(a1497, a1498)?
                                                                                                                    };
                                                                                                                    let a1506 = {
                                                                                                                        let a1502 = {
                                                                                                                            let a1499 = v1;
                                                                                                                            let a1500 = 32u64;
                                                                                                                            let a1501 = 0u64;
                                                                                                                            nat_rem(a1499, a1500, a1501)
                                                                                                                        };
                                                                                                                        let a1503 = 8u64;
                                                                                                                        let a1504 = 0u64;
                                                                                                                        nat_quot(a1502, a1503, a1504)
                                                                                                                    };
                                                                                                                    nat_add(a1505, a1506)?
                                                                                                                };
                                                                                                                nat_add(a1507, a1508)?
                                                                                                            }, 4u64))
                                                                                                        } else {
                                                                                                            let a1509 = v3;
                                                                                                            let a1510 = 1u64;
                                                                                                            if nat_eq(a1509, a1510) {
                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                    let a1514 = 66u64;
                                                                                                                    let a1515 = {
                                                                                                                        let a1511 = v1;
                                                                                                                        let a1512 = 32u64;
                                                                                                                        let a1513 = 0u64;
                                                                                                                        nat_quot(a1511, a1512, a1513)
                                                                                                                    };
                                                                                                                    nat_add(a1514, a1515)?
                                                                                                                }, 6u64))
                                                                                                            } else {
                                                                                                                let a1516 = v3;
                                                                                                                let a1517 = 2u64;
                                                                                                                if nat_eq(a1516, a1517) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1521 = 74u64;
                                                                                                                        let a1522 = {
                                                                                                                            let a1518 = v1;
                                                                                                                            let a1519 = 32u64;
                                                                                                                            let a1520 = 0u64;
                                                                                                                            nat_quot(a1518, a1519, a1520)
                                                                                                                        };
                                                                                                                        nat_add(a1521, a1522)?
                                                                                                                    }, 5u64))
                                                                                                                } else {
                                                                                                                    let a1523 = v3;
                                                                                                                    let a1524 = 3u64;
                                                                                                                    if nat_eq(a1523, a1524) {
                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                            let a1538 = 34u64;
                                                                                                                            let a1539 = {
                                                                                                                                let a1536 = {
                                                                                                                                    let a1528 = {
                                                                                                                                        let a1525 = v1;
                                                                                                                                        let a1526 = 32u64;
                                                                                                                                        let a1527 = 0u64;
                                                                                                                                        nat_quot(a1525, a1526, a1527)
                                                                                                                                    };
                                                                                                                                    let a1529 = 4u64;
                                                                                                                                    nat_mul(a1528, a1529)?
                                                                                                                                };
                                                                                                                                let a1537 = {
                                                                                                                                    let a1533 = {
                                                                                                                                        let a1530 = v1;
                                                                                                                                        let a1531 = 32u64;
                                                                                                                                        let a1532 = 0u64;
                                                                                                                                        nat_rem(a1530, a1531, a1532)
                                                                                                                                    };
                                                                                                                                    let a1534 = 8u64;
                                                                                                                                    let a1535 = 0u64;
                                                                                                                                    nat_quot(a1533, a1534, a1535)
                                                                                                                                };
                                                                                                                                nat_add(a1536, a1537)?
                                                                                                                            };
                                                                                                                            nat_add(a1538, a1539)?
                                                                                                                        }, 7u64))
                                                                                                                    } else {
                                                                                                                        let a1540 = v3;
                                                                                                                        let a1541 = 4u64;
                                                                                                                        if nat_eq(a1540, a1541) {
                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                let a1556 = {
                                                                                                                                    let a1553 = v9;
                                                                                                                                    let a1554 = {
                                                                                                                                        let a1551 = {
                                                                                                                                            let a1548 = {
                                                                                                                                                let a1545 = {
                                                                                                                                                    let a1542 = v1;
                                                                                                                                                    let a1543 = 32u64;
                                                                                                                                                    let a1544 = 0u64;
                                                                                                                                                    nat_rem(a1542, a1543, a1544)
                                                                                                                                                };
                                                                                                                                                let a1546 = 8u64;
                                                                                                                                                let a1547 = 0u64;
                                                                                                                                                nat_quot(a1545, a1546, a1547)
                                                                                                                                            };
                                                                                                                                            let a1549 = 2u64;
                                                                                                                                            let a1550 = 0u64;
                                                                                                                                            nat_quot(a1548, a1549, a1550)
                                                                                                                                        };
                                                                                                                                        let a1552 = 0u64;
                                                                                                                                        if nat_eq(a1551, a1552) {
                                                                                                                                            1u64
                                                                                                                                        } else {
                                                                                                                                            16u64
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    let a1555 = 0u64;
                                                                                                                                    nat_quot(a1553, a1554, a1555)
                                                                                                                                };
                                                                                                                                let a1557 = 16u64;
                                                                                                                                let a1558 = 0u64;
                                                                                                                                nat_rem(a1556, a1557, a1558)
                                                                                                                            }, 0u64, 2u64))
                                                                                                                        } else {
                                                                                                                            let a1559 = v3;
                                                                                                                            let a1560 = 5u64;
                                                                                                                            if nat_eq(a1559, a1560) {
                                                                                                                                f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1561 = v3;
                                                                                                                                let a1562 = 6u64;
                                                                                                                                if nat_eq(a1561, a1562) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1563 = v3;
                                                                                                                                    let a1564 = 7u64;
                                                                                                                                    if nat_eq(a1563, a1564) {
                                                                                                                                        f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1565 = v3;
                                                                                                                                        let a1566 = 8u64;
                                                                                                                                        if nat_eq(a1565, a1566) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1592 = 7304u64;
                                                                                                                                                let a1593 = {
                                                                                                                                                    let a1590 = {
                                                                                                                                                        let a1585 = {
                                                                                                                                                            let a1583 = v8;
                                                                                                                                                            let a1584 = {
                                                                                                                                                                let a1581 = {
                                                                                                                                                                    let a1578 = {
                                                                                                                                                                        let a1575 = v10;
                                                                                                                                                                        let a1576 = f10({
                                                                                                                                                                            let a1573 = 2u64;
                                                                                                                                                                            let a1574 = {
                                                                                                                                                                                let a1570 = {
                                                                                                                                                                                    let a1567 = v1;
                                                                                                                                                                                    let a1568 = 32u64;
                                                                                                                                                                                    let a1569 = 0u64;
                                                                                                                                                                                    nat_rem(a1567, a1568, a1569)
                                                                                                                                                                                };
                                                                                                                                                                                let a1571 = 8u64;
                                                                                                                                                                                let a1572 = 0u64;
                                                                                                                                                                                nat_quot(a1570, a1571, a1572)
                                                                                                                                                                            };
                                                                                                                                                                            nat_mul(a1573, a1574)?
                                                                                                                                                                        });
                                                                                                                                                                        let a1577 = 0u64;
                                                                                                                                                                        nat_quot(a1575, a1576, a1577)
                                                                                                                                                                    };
                                                                                                                                                                    let a1579 = 4u64;
                                                                                                                                                                    let a1580 = 0u64;
                                                                                                                                                                    nat_rem(a1578, a1579, a1580)
                                                                                                                                                                };
                                                                                                                                                                let a1582 = 256u64;
                                                                                                                                                                nat_mul(a1581, a1582)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a1583, a1584)?
                                                                                                                                                        };
                                                                                                                                                        let a1586 = 8u64;
                                                                                                                                                        nat_mul(a1585, a1586)?
                                                                                                                                                    };
                                                                                                                                                    let a1591 = {
                                                                                                                                                        let a1587 = v1;
                                                                                                                                                        let a1588 = 8u64;
                                                                                                                                                        let a1589 = 0u64;
                                                                                                                                                        nat_rem(a1587, a1588, a1589)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1590, a1591)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1592, a1593)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1594 = v3;
                                                                                                                                            let a1595 = 9u64;
                                                                                                                                            if nat_eq(a1594, a1595) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1596 = v3;
                                                                                                                                                let a1597 = 10u64;
                                                                                                                                                if nat_eq(a1596, a1597) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1598 = v3;
                                                                                                                                                    let a1599 = 11u64;
                                                                                                                                                    if nat_eq(a1598, a1599) {
                                                                                                                                                        Ok(f11(v6, {
                                                                                                                                                            let a1609 = {
                                                                                                                                                                let a1606 = {
                                                                                                                                                                    let a1603 = v11;
                                                                                                                                                                    let a1604 = f10({
                                                                                                                                                                        let a1600 = v1;
                                                                                                                                                                        let a1601 = 8u64;
                                                                                                                                                                        let a1602 = 0u64;
                                                                                                                                                                        nat_rem(a1600, a1601, a1602)
                                                                                                                                                                    });
                                                                                                                                                                    let a1605 = 0u64;
                                                                                                                                                                    nat_quot(a1603, a1604, a1605)
                                                                                                                                                                };
                                                                                                                                                                let a1607 = 2u64;
                                                                                                                                                                let a1608 = 0u64;
                                                                                                                                                                nat_rem(a1606, a1607, a1608)
                                                                                                                                                            };
                                                                                                                                                            let a1610 = 1u64;
                                                                                                                                                            nat_eq(a1609, a1610)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1611 = v3;
                                                                                                                                                        let a1612 = 12u64;
                                                                                                                                                        if nat_eq(a1611, a1612) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1613 = v1;
                                                                                                                                                                let a1614 = v2;
                                                                                                                                                                nat_mul(a1613, a1614)?
                                                                                                                                                            }, v6))
                                                                                                                                                        } else {
                                                                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1615 = v0;
                                                                                                        let a1616 = 18u64;
                                                                                                        if nat_eq(a1615, a1616) {
                                                                                                            let a1617 = v3;
                                                                                                            let a1618 = 0u64;
                                                                                                            if nat_eq(a1617, a1618) {
                                                                                                                Ok(Adt1::C2(0u64, 4u64, {
                                                                                                                    let a1624 = 66u64;
                                                                                                                    let a1625 = {
                                                                                                                        let a1622 = {
                                                                                                                            let a1619 = v1;
                                                                                                                            let a1620 = 32u64;
                                                                                                                            let a1621 = 0u64;
                                                                                                                            nat_quot(a1619, a1620, a1621)
                                                                                                                        };
                                                                                                                        let a1623 = 4u64;
                                                                                                                        nat_mul(a1622, a1623)?
                                                                                                                    };
                                                                                                                    nat_add(a1624, a1625)?
                                                                                                                }, 5u64))
                                                                                                            } else {
                                                                                                                let a1626 = v3;
                                                                                                                let a1627 = 1u64;
                                                                                                                if nat_eq(a1626, a1627) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1651 = 2u64;
                                                                                                                        let a1652 = {
                                                                                                                            let a1649 = {
                                                                                                                                let a1631 = {
                                                                                                                                    let a1628 = v1;
                                                                                                                                    let a1629 = 32u64;
                                                                                                                                    let a1630 = 0u64;
                                                                                                                                    nat_quot(a1628, a1629, a1630)
                                                                                                                                };
                                                                                                                                let a1632 = 8u64;
                                                                                                                                nat_mul(a1631, a1632)?
                                                                                                                            };
                                                                                                                            let a1650 = {
                                                                                                                                let a1647 = {
                                                                                                                                    let a1639 = {
                                                                                                                                        let a1636 = {
                                                                                                                                            let a1633 = v1;
                                                                                                                                            let a1634 = 32u64;
                                                                                                                                            let a1635 = 0u64;
                                                                                                                                            nat_rem(a1633, a1634, a1635)
                                                                                                                                        };
                                                                                                                                        let a1637 = 8u64;
                                                                                                                                        let a1638 = 0u64;
                                                                                                                                        nat_quot(a1636, a1637, a1638)
                                                                                                                                    };
                                                                                                                                    let a1640 = 2u64;
                                                                                                                                    nat_mul(a1639, a1640)?
                                                                                                                                };
                                                                                                                                let a1648 = {
                                                                                                                                    let a1644 = {
                                                                                                                                        let a1641 = v1;
                                                                                                                                        let a1642 = 8u64;
                                                                                                                                        let a1643 = 0u64;
                                                                                                                                        nat_rem(a1641, a1642, a1643)
                                                                                                                                    };
                                                                                                                                    let a1645 = 4u64;
                                                                                                                                    let a1646 = 0u64;
                                                                                                                                    nat_quot(a1644, a1645, a1646)
                                                                                                                                };
                                                                                                                                nat_add(a1647, a1648)?
                                                                                                                            };
                                                                                                                            nat_add(a1649, a1650)?
                                                                                                                        };
                                                                                                                        nat_add(a1651, a1652)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a1653 = v3;
                                                                                                                    let a1654 = 2u64;
                                                                                                                    if nat_eq(a1653, a1654) {
                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                            let a1669 = 8u64;
                                                                                                                            let a1670 = {
                                                                                                                                let a1666 = {
                                                                                                                                    let a1663 = v9;
                                                                                                                                    let a1664 = f10({
                                                                                                                                        let a1661 = {
                                                                                                                                            let a1658 = {
                                                                                                                                                let a1655 = v1;
                                                                                                                                                let a1656 = 32u64;
                                                                                                                                                let a1657 = 0u64;
                                                                                                                                                nat_rem(a1655, a1656, a1657)
                                                                                                                                            };
                                                                                                                                            let a1659 = 8u64;
                                                                                                                                            let a1660 = 0u64;
                                                                                                                                            nat_quot(a1658, a1659, a1660)
                                                                                                                                        };
                                                                                                                                        let a1662 = 7u64;
                                                                                                                                        nat_mul(a1661, a1662)?
                                                                                                                                    });
                                                                                                                                    let a1665 = 0u64;
                                                                                                                                    nat_quot(a1663, a1664, a1665)
                                                                                                                                };
                                                                                                                                let a1667 = 128u64;
                                                                                                                                let a1668 = 0u64;
                                                                                                                                nat_rem(a1666, a1667, a1668)
                                                                                                                            };
                                                                                                                            nat_add(a1669, a1670)?
                                                                                                                        }, 7u64))
                                                                                                                    } else {
                                                                                                                        let a1671 = v3;
                                                                                                                        let a1672 = 3u64;
                                                                                                                        if nat_eq(a1671, a1672) {
                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                let a1673 = v9;
                                                                                                                                let a1674 = 268435456u64;
                                                                                                                                let a1675 = 0u64;
                                                                                                                                nat_quot(a1673, a1674, a1675)
                                                                                                                            }, 0u64, 2u64))
                                                                                                                        } else {
                                                                                                                            let a1676 = v3;
                                                                                                                            let a1677 = 4u64;
                                                                                                                            if nat_eq(a1676, a1677) {
                                                                                                                                f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1678 = v3;
                                                                                                                                let a1679 = 5u64;
                                                                                                                                if nat_eq(a1678, a1679) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1680 = v3;
                                                                                                                                    let a1681 = 6u64;
                                                                                                                                    if nat_eq(a1680, a1681) {
                                                                                                                                        f4(2u64, v6, 1056964608u64, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1682 = v3;
                                                                                                                                        let a1683 = 7u64;
                                                                                                                                        if nat_eq(a1682, a1683) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1694 = 15496u64;
                                                                                                                                                let a1695 = {
                                                                                                                                                    let a1692 = {
                                                                                                                                                        let a1684 = v8;
                                                                                                                                                        let a1685 = 4u64;
                                                                                                                                                        nat_mul(a1684, a1685)?
                                                                                                                                                    };
                                                                                                                                                    let a1693 = {
                                                                                                                                                        let a1689 = {
                                                                                                                                                            let a1686 = v1;
                                                                                                                                                            let a1687 = 8u64;
                                                                                                                                                            let a1688 = 0u64;
                                                                                                                                                            nat_rem(a1686, a1687, a1688)
                                                                                                                                                        };
                                                                                                                                                        let a1690 = 4u64;
                                                                                                                                                        let a1691 = 0u64;
                                                                                                                                                        nat_rem(a1689, a1690, a1691)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1692, a1693)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1694, a1695)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1696 = v3;
                                                                                                                                            let a1697 = 8u64;
                                                                                                                                            if nat_eq(a1696, a1697) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1698 = v3;
                                                                                                                                                let a1699 = 9u64;
                                                                                                                                                if nat_eq(a1698, a1699) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1700 = v3;
                                                                                                                                                    let a1701 = 10u64;
                                                                                                                                                    if nat_eq(a1700, a1701) {
                                                                                                                                                        Ok(f11(v6, {
                                                                                                                                                            let a1711 = {
                                                                                                                                                                let a1708 = {
                                                                                                                                                                    let a1705 = v11;
                                                                                                                                                                    let a1706 = f10({
                                                                                                                                                                        let a1702 = v1;
                                                                                                                                                                        let a1703 = 8u64;
                                                                                                                                                                        let a1704 = 0u64;
                                                                                                                                                                        nat_rem(a1702, a1703, a1704)
                                                                                                                                                                    });
                                                                                                                                                                    let a1707 = 0u64;
                                                                                                                                                                    nat_quot(a1705, a1706, a1707)
                                                                                                                                                                };
                                                                                                                                                                let a1709 = 2u64;
                                                                                                                                                                let a1710 = 0u64;
                                                                                                                                                                nat_rem(a1708, a1709, a1710)
                                                                                                                                                            };
                                                                                                                                                            let a1712 = 1u64;
                                                                                                                                                            nat_eq(a1711, a1712)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1713 = v3;
                                                                                                                                                        let a1714 = 11u64;
                                                                                                                                                        if nat_eq(a1713, a1714) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1715 = v1;
                                                                                                                                                                let a1716 = v2;
                                                                                                                                                                nat_mul(a1715, a1716)?
                                                                                                                                                            }, v6))
                                                                                                                                                        } else {
                                                                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            let a1717 = v0;
                                                                                                            let a1718 = 21u64;
                                                                                                            if nat_eq(a1717, a1718) {
                                                                                                                let a1719 = v3;
                                                                                                                let a1720 = 0u64;
                                                                                                                if nat_eq(a1719, a1720) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1744 = 2u64;
                                                                                                                        let a1745 = {
                                                                                                                            let a1742 = {
                                                                                                                                let a1724 = {
                                                                                                                                    let a1721 = v1;
                                                                                                                                    let a1722 = 32u64;
                                                                                                                                    let a1723 = 0u64;
                                                                                                                                    nat_quot(a1721, a1722, a1723)
                                                                                                                                };
                                                                                                                                let a1725 = 8u64;
                                                                                                                                nat_mul(a1724, a1725)?
                                                                                                                            };
                                                                                                                            let a1743 = {
                                                                                                                                let a1740 = {
                                                                                                                                    let a1732 = {
                                                                                                                                        let a1729 = {
                                                                                                                                            let a1726 = v1;
                                                                                                                                            let a1727 = 32u64;
                                                                                                                                            let a1728 = 0u64;
                                                                                                                                            nat_rem(a1726, a1727, a1728)
                                                                                                                                        };
                                                                                                                                        let a1730 = 8u64;
                                                                                                                                        let a1731 = 0u64;
                                                                                                                                        nat_quot(a1729, a1730, a1731)
                                                                                                                                    };
                                                                                                                                    let a1733 = 2u64;
                                                                                                                                    nat_mul(a1732, a1733)?
                                                                                                                                };
                                                                                                                                let a1741 = {
                                                                                                                                    let a1737 = {
                                                                                                                                        let a1734 = v1;
                                                                                                                                        let a1735 = 8u64;
                                                                                                                                        let a1736 = 0u64;
                                                                                                                                        nat_rem(a1734, a1735, a1736)
                                                                                                                                    };
                                                                                                                                    let a1738 = 4u64;
                                                                                                                                    let a1739 = 0u64;
                                                                                                                                    nat_quot(a1737, a1738, a1739)
                                                                                                                                };
                                                                                                                                nat_add(a1740, a1741)?
                                                                                                                            };
                                                                                                                            nat_add(a1742, a1743)?
                                                                                                                        };
                                                                                                                        nat_add(a1744, a1745)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a1746 = v3;
                                                                                                                    let a1747 = 1u64;
                                                                                                                    if nat_eq(a1746, a1747) {
                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                            let a1751 = 66u64;
                                                                                                                            let a1752 = {
                                                                                                                                let a1748 = v1;
                                                                                                                                let a1749 = 32u64;
                                                                                                                                let a1750 = 0u64;
                                                                                                                                nat_quot(a1748, a1749, a1750)
                                                                                                                            };
                                                                                                                            nat_add(a1751, a1752)?
                                                                                                                        }, 6u64))
                                                                                                                    } else {
                                                                                                                        let a1753 = v3;
                                                                                                                        let a1754 = 2u64;
                                                                                                                        if nat_eq(a1753, a1754) {
                                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                let a1761 = 106u64;
                                                                                                                                let a1762 = {
                                                                                                                                    let a1758 = {
                                                                                                                                        let a1755 = v1;
                                                                                                                                        let a1756 = 32u64;
                                                                                                                                        let a1757 = 0u64;
                                                                                                                                        nat_quot(a1755, a1756, a1757)
                                                                                                                                    };
                                                                                                                                    let a1759 = 2u64;
                                                                                                                                    let a1760 = 0u64;
                                                                                                                                    nat_quot(a1758, a1759, a1760)
                                                                                                                                };
                                                                                                                                nat_add(a1761, a1762)?
                                                                                                                            }, 5u64))
                                                                                                                        } else {
                                                                                                                            let a1763 = v3;
                                                                                                                            let a1764 = 3u64;
                                                                                                                            if nat_eq(a1763, a1764) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a1778 = 74u64;
                                                                                                                                    let a1779 = {
                                                                                                                                        let a1776 = {
                                                                                                                                            let a1768 = {
                                                                                                                                                let a1765 = v1;
                                                                                                                                                let a1766 = 32u64;
                                                                                                                                                let a1767 = 0u64;
                                                                                                                                                nat_quot(a1765, a1766, a1767)
                                                                                                                                            };
                                                                                                                                            let a1769 = 4u64;
                                                                                                                                            nat_mul(a1768, a1769)?
                                                                                                                                        };
                                                                                                                                        let a1777 = {
                                                                                                                                            let a1773 = {
                                                                                                                                                let a1770 = v1;
                                                                                                                                                let a1771 = 32u64;
                                                                                                                                                let a1772 = 0u64;
                                                                                                                                                nat_rem(a1770, a1771, a1772)
                                                                                                                                            };
                                                                                                                                            let a1774 = 8u64;
                                                                                                                                            let a1775 = 0u64;
                                                                                                                                            nat_quot(a1773, a1774, a1775)
                                                                                                                                        };
                                                                                                                                        nat_add(a1776, a1777)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1778, a1779)?
                                                                                                                                }, 7u64))
                                                                                                                            } else {
                                                                                                                                let a1780 = v3;
                                                                                                                                let a1781 = 4u64;
                                                                                                                                if nat_eq(a1780, a1781) {
                                                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                                                        let a1798 = 1u64;
                                                                                                                                        let a1799 = {
                                                                                                                                            let a1796 = 2u64;
                                                                                                                                            let a1797 = {
                                                                                                                                                let a1793 = {
                                                                                                                                                    let a1790 = v9;
                                                                                                                                                    let a1791 = {
                                                                                                                                                        let a1788 = {
                                                                                                                                                            let a1785 = {
                                                                                                                                                                let a1782 = v1;
                                                                                                                                                                let a1783 = 32u64;
                                                                                                                                                                let a1784 = 0u64;
                                                                                                                                                                nat_quot(a1782, a1783, a1784)
                                                                                                                                                            };
                                                                                                                                                            let a1786 = 2u64;
                                                                                                                                                            let a1787 = 0u64;
                                                                                                                                                            nat_rem(a1785, a1786, a1787)
                                                                                                                                                        };
                                                                                                                                                        let a1789 = 0u64;
                                                                                                                                                        if nat_eq(a1788, a1789) {
                                                                                                                                                            1u64
                                                                                                                                                        } else {
                                                                                                                                                            16u64
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    let a1792 = 0u64;
                                                                                                                                                    nat_quot(a1790, a1791, a1792)
                                                                                                                                                };
                                                                                                                                                let a1794 = 16u64;
                                                                                                                                                let a1795 = 0u64;
                                                                                                                                                nat_rem(a1793, a1794, a1795)
                                                                                                                                            };
                                                                                                                                            nat_mul(a1796, a1797)?
                                                                                                                                        };
                                                                                                                                        nat_add(a1798, a1799)?
                                                                                                                                    }, 0u64, 2u64))
                                                                                                                                } else {
                                                                                                                                    let a1800 = v3;
                                                                                                                                    let a1801 = 5u64;
                                                                                                                                    if nat_eq(a1800, a1801) {
                                                                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1802 = v3;
                                                                                                                                        let a1803 = 6u64;
                                                                                                                                        if nat_eq(a1802, a1803) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1840 = 16520u64;
                                                                                                                                                let a1841 = {
                                                                                                                                                    let a1838 = {
                                                                                                                                                        let a1830 = {
                                                                                                                                                            let a1828 = v8;
                                                                                                                                                            let a1829 = {
                                                                                                                                                                let a1826 = {
                                                                                                                                                                    let a1823 = {
                                                                                                                                                                        let a1820 = v10;
                                                                                                                                                                        let a1821 = f10({
                                                                                                                                                                            let a1818 = {
                                                                                                                                                                                let a1810 = 2u64;
                                                                                                                                                                                let a1811 = {
                                                                                                                                                                                    let a1807 = {
                                                                                                                                                                                        let a1804 = v1;
                                                                                                                                                                                        let a1805 = 32u64;
                                                                                                                                                                                        let a1806 = 0u64;
                                                                                                                                                                                        nat_rem(a1804, a1805, a1806)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a1808 = 8u64;
                                                                                                                                                                                    let a1809 = 0u64;
                                                                                                                                                                                    nat_quot(a1807, a1808, a1809)
                                                                                                                                                                                };
                                                                                                                                                                                nat_mul(a1810, a1811)?
                                                                                                                                                                            };
                                                                                                                                                                            let a1819 = {
                                                                                                                                                                                let a1815 = {
                                                                                                                                                                                    let a1812 = v1;
                                                                                                                                                                                    let a1813 = 8u64;
                                                                                                                                                                                    let a1814 = 0u64;
                                                                                                                                                                                    nat_rem(a1812, a1813, a1814)
                                                                                                                                                                                };
                                                                                                                                                                                let a1816 = 4u64;
                                                                                                                                                                                let a1817 = 0u64;
                                                                                                                                                                                nat_quot(a1815, a1816, a1817)
                                                                                                                                                                            };
                                                                                                                                                                            nat_add(a1818, a1819)?
                                                                                                                                                                        });
                                                                                                                                                                        let a1822 = 0u64;
                                                                                                                                                                        nat_quot(a1820, a1821, a1822)
                                                                                                                                                                    };
                                                                                                                                                                    let a1824 = 2u64;
                                                                                                                                                                    let a1825 = 0u64;
                                                                                                                                                                    nat_rem(a1823, a1824, a1825)
                                                                                                                                                                };
                                                                                                                                                                let a1827 = 256u64;
                                                                                                                                                                nat_mul(a1826, a1827)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a1828, a1829)?
                                                                                                                                                        };
                                                                                                                                                        let a1831 = 4u64;
                                                                                                                                                        nat_mul(a1830, a1831)?
                                                                                                                                                    };
                                                                                                                                                    let a1839 = {
                                                                                                                                                        let a1835 = {
                                                                                                                                                            let a1832 = v1;
                                                                                                                                                            let a1833 = 8u64;
                                                                                                                                                            let a1834 = 0u64;
                                                                                                                                                            nat_rem(a1832, a1833, a1834)
                                                                                                                                                        };
                                                                                                                                                        let a1836 = 4u64;
                                                                                                                                                        let a1837 = 0u64;
                                                                                                                                                        nat_rem(a1835, a1836, a1837)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1838, a1839)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1840, a1841)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1842 = v3;
                                                                                                                                            let a1843 = 7u64;
                                                                                                                                            if nat_eq(a1842, a1843) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1844 = v3;
                                                                                                                                                let a1845 = 8u64;
                                                                                                                                                if nat_eq(a1844, a1845) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1846 = v3;
                                                                                                                                                    let a1847 = 9u64;
                                                                                                                                                    if nat_eq(a1846, a1847) {
                                                                                                                                                        Ok(f11(v6, {
                                                                                                                                                            let a1857 = {
                                                                                                                                                                let a1854 = {
                                                                                                                                                                    let a1851 = v11;
                                                                                                                                                                    let a1852 = f10({
                                                                                                                                                                        let a1848 = v1;
                                                                                                                                                                        let a1849 = 8u64;
                                                                                                                                                                        let a1850 = 0u64;
                                                                                                                                                                        nat_rem(a1848, a1849, a1850)
                                                                                                                                                                    });
                                                                                                                                                                    let a1853 = 0u64;
                                                                                                                                                                    nat_quot(a1851, a1852, a1853)
                                                                                                                                                                };
                                                                                                                                                                let a1855 = 2u64;
                                                                                                                                                                let a1856 = 0u64;
                                                                                                                                                                nat_rem(a1854, a1855, a1856)
                                                                                                                                                            };
                                                                                                                                                            let a1858 = 1u64;
                                                                                                                                                            nat_eq(a1857, a1858)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1859 = v3;
                                                                                                                                                        let a1860 = 10u64;
                                                                                                                                                        if nat_eq(a1859, a1860) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1861 = v1;
                                                                                                                                                                let a1862 = v2;
                                                                                                                                                                nat_mul(a1861, a1862)?
                                                                                                                                                            }, v6))
                                                                                                                                                        } else {
                                                                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                let a1863 = v0;
                                                                                                                let a1864 = 19u64;
                                                                                                                if nat_eq(a1863, a1864) {
                                                                                                                    let a1865 = v3;
                                                                                                                    let a1866 = 0u64;
                                                                                                                    if nat_eq(a1865, a1866) {
                                                                                                                        Ok(Adt1::C2(0u64, 2u64, {
                                                                                                                            let a1872 = 34u64;
                                                                                                                            let a1873 = {
                                                                                                                                let a1870 = {
                                                                                                                                    let a1867 = v1;
                                                                                                                                    let a1868 = 32u64;
                                                                                                                                    let a1869 = 0u64;
                                                                                                                                    nat_quot(a1867, a1868, a1869)
                                                                                                                                };
                                                                                                                                let a1871 = 2u64;
                                                                                                                                nat_mul(a1870, a1871)?
                                                                                                                            };
                                                                                                                            nat_add(a1872, a1873)?
                                                                                                                        }, 5u64))
                                                                                                                    } else {
                                                                                                                        let a1874 = v3;
                                                                                                                        let a1875 = 1u64;
                                                                                                                        if nat_eq(a1874, a1875) {
                                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                let a1889 = 2u64;
                                                                                                                                let a1890 = {
                                                                                                                                    let a1887 = {
                                                                                                                                        let a1879 = {
                                                                                                                                            let a1876 = v1;
                                                                                                                                            let a1877 = 32u64;
                                                                                                                                            let a1878 = 0u64;
                                                                                                                                            nat_quot(a1876, a1877, a1878)
                                                                                                                                        };
                                                                                                                                        let a1880 = 4u64;
                                                                                                                                        nat_mul(a1879, a1880)?
                                                                                                                                    };
                                                                                                                                    let a1888 = {
                                                                                                                                        let a1884 = {
                                                                                                                                            let a1881 = v1;
                                                                                                                                            let a1882 = 32u64;
                                                                                                                                            let a1883 = 0u64;
                                                                                                                                            nat_rem(a1881, a1882, a1883)
                                                                                                                                        };
                                                                                                                                        let a1885 = 8u64;
                                                                                                                                        let a1886 = 0u64;
                                                                                                                                        nat_quot(a1884, a1885, a1886)
                                                                                                                                    };
                                                                                                                                    nat_add(a1887, a1888)?
                                                                                                                                };
                                                                                                                                nat_add(a1889, a1890)?
                                                                                                                            }, 4u64))
                                                                                                                        } else {
                                                                                                                            let a1891 = v3;
                                                                                                                            let a1892 = 2u64;
                                                                                                                            if nat_eq(a1891, a1892) {
                                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                                    let a1901 = 1u64;
                                                                                                                                    let a1902 = {
                                                                                                                                        let a1899 = 2u64;
                                                                                                                                        let a1900 = {
                                                                                                                                            let a1896 = {
                                                                                                                                                let a1893 = v9;
                                                                                                                                                let a1894 = 4096u64;
                                                                                                                                                let a1895 = 0u64;
                                                                                                                                                nat_quot(a1893, a1894, a1895)
                                                                                                                                            };
                                                                                                                                            let a1897 = 8u64;
                                                                                                                                            let a1898 = 0u64;
                                                                                                                                            nat_rem(a1896, a1897, a1898)
                                                                                                                                        };
                                                                                                                                        nat_mul(a1899, a1900)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1901, a1902)?
                                                                                                                                }, 0u64, 2u64))
                                                                                                                            } else {
                                                                                                                                let a1903 = v3;
                                                                                                                                let a1904 = 3u64;
                                                                                                                                if nat_eq(a1903, a1904) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1905 = v3;
                                                                                                                                    let a1906 = 4u64;
                                                                                                                                    if nat_eq(a1905, a1906) {
                                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                            let a1932 = 18600u64;
                                                                                                                                            let a1933 = {
                                                                                                                                                let a1930 = {
                                                                                                                                                    let a1925 = {
                                                                                                                                                        let a1923 = v8;
                                                                                                                                                        let a1924 = {
                                                                                                                                                            let a1921 = {
                                                                                                                                                                let a1918 = {
                                                                                                                                                                    let a1915 = v9;
                                                                                                                                                                    let a1916 = f10({
                                                                                                                                                                        let a1913 = 3u64;
                                                                                                                                                                        let a1914 = {
                                                                                                                                                                            let a1910 = {
                                                                                                                                                                                let a1907 = v1;
                                                                                                                                                                                let a1908 = 32u64;
                                                                                                                                                                                let a1909 = 0u64;
                                                                                                                                                                                nat_rem(a1907, a1908, a1909)
                                                                                                                                                                            };
                                                                                                                                                                            let a1911 = 8u64;
                                                                                                                                                                            let a1912 = 0u64;
                                                                                                                                                                            nat_quot(a1910, a1911, a1912)
                                                                                                                                                                        };
                                                                                                                                                                        nat_mul(a1913, a1914)?
                                                                                                                                                                    });
                                                                                                                                                                    let a1917 = 0u64;
                                                                                                                                                                    nat_quot(a1915, a1916, a1917)
                                                                                                                                                                };
                                                                                                                                                                let a1919 = 8u64;
                                                                                                                                                                let a1920 = 0u64;
                                                                                                                                                                nat_rem(a1918, a1919, a1920)
                                                                                                                                                            };
                                                                                                                                                            let a1922 = 256u64;
                                                                                                                                                            nat_mul(a1921, a1922)?
                                                                                                                                                        };
                                                                                                                                                        nat_add(a1923, a1924)?
                                                                                                                                                    };
                                                                                                                                                    let a1926 = 8u64;
                                                                                                                                                    nat_mul(a1925, a1926)?
                                                                                                                                                };
                                                                                                                                                let a1931 = {
                                                                                                                                                    let a1927 = v1;
                                                                                                                                                    let a1928 = 8u64;
                                                                                                                                                    let a1929 = 0u64;
                                                                                                                                                    nat_rem(a1927, a1928, a1929)
                                                                                                                                                };
                                                                                                                                                nat_add(a1930, a1931)?
                                                                                                                                            };
                                                                                                                                            nat_add(a1932, a1933)?
                                                                                                                                        }, 3u64))
                                                                                                                                    } else {
                                                                                                                                        let a1934 = v3;
                                                                                                                                        let a1935 = 5u64;
                                                                                                                                        if nat_eq(a1934, a1935) {
                                                                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                let a1936 = v7;
                                                                                                                                                let a1937 = 128u64;
                                                                                                                                                if nat_lt(a1936, a1937) {
                                                                                                                                                    0u64
                                                                                                                                                } else {
                                                                                                                                                    256u64
                                                                                                                                                }
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1938 = v3;
                                                                                                                                            let a1939 = 6u64;
                                                                                                                                            if nat_eq(a1938, a1939) {
                                                                                                                                                f4(0u64, v7, {
                                                                                                                                                    let a1946 = {
                                                                                                                                                        let a1943 = {
                                                                                                                                                            let a1940 = v9;
                                                                                                                                                            let a1941 = 32768u64;
                                                                                                                                                            let a1942 = 0u64;
                                                                                                                                                            nat_quot(a1940, a1941, a1942)
                                                                                                                                                        };
                                                                                                                                                        let a1944 = 2u64;
                                                                                                                                                        let a1945 = 0u64;
                                                                                                                                                        nat_rem(a1943, a1944, a1945)
                                                                                                                                                    };
                                                                                                                                                    let a1947 = 1u64;
                                                                                                                                                    if nat_eq(a1946, a1947) {
                                                                                                                                                        3187671040u64
                                                                                                                                                    } else {
                                                                                                                                                        1040187392u64
                                                                                                                                                    }
                                                                                                                                                }, 3u64)
                                                                                                                                            } else {
                                                                                                                                                let a1948 = v3;
                                                                                                                                                let a1949 = 7u64;
                                                                                                                                                if nat_eq(a1948, a1949) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1950 = v3;
                                                                                                                                                    let a1951 = 8u64;
                                                                                                                                                    if nat_eq(a1950, a1951) {
                                                                                                                                                        Ok(Adt1::C4({
                                                                                                                                                            let a1952 = v1;
                                                                                                                                                            let a1953 = v2;
                                                                                                                                                            nat_mul(a1952, a1953)?
                                                                                                                                                        }, v6))
                                                                                                                                                    } else {
                                                                                                                                                        Ok(Adt1::C0(0u64))
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    let a1954 = v0;
                                                                                                                    let a1955 = 29u64;
                                                                                                                    if nat_eq(a1954, a1955) {
                                                                                                                        let a1956 = v3;
                                                                                                                        let a1957 = 0u64;
                                                                                                                        if nat_eq(a1956, a1957) {
                                                                                                                            Ok(Adt1::C2(0u64, 2u64, {
                                                                                                                                let a1963 = 48u64;
                                                                                                                                let a1964 = {
                                                                                                                                    let a1961 = {
                                                                                                                                        let a1958 = v1;
                                                                                                                                        let a1959 = 64u64;
                                                                                                                                        let a1960 = 0u64;
                                                                                                                                        nat_quot(a1958, a1959, a1960)
                                                                                                                                    };
                                                                                                                                    let a1962 = 2u64;
                                                                                                                                    nat_mul(a1961, a1962)?
                                                                                                                                };
                                                                                                                                nat_add(a1963, a1964)?
                                                                                                                            }, 6u64))
                                                                                                                        } else {
                                                                                                                            let a1965 = v3;
                                                                                                                            let a1966 = 1u64;
                                                                                                                            if nat_eq(a1965, a1966) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a1983 = 32u64;
                                                                                                                                    let a1984 = {
                                                                                                                                        let a1981 = {
                                                                                                                                            let a1970 = {
                                                                                                                                                let a1967 = v1;
                                                                                                                                                let a1968 = 32u64;
                                                                                                                                                let a1969 = 0u64;
                                                                                                                                                nat_quot(a1967, a1968, a1969)
                                                                                                                                            };
                                                                                                                                            let a1971 = 2u64;
                                                                                                                                            nat_mul(a1970, a1971)?
                                                                                                                                        };
                                                                                                                                        let a1982 = {
                                                                                                                                            let a1978 = {
                                                                                                                                                let a1975 = {
                                                                                                                                                    let a1972 = v1;
                                                                                                                                                    let a1973 = 32u64;
                                                                                                                                                    let a1974 = 0u64;
                                                                                                                                                    nat_rem(a1972, a1973, a1974)
                                                                                                                                                };
                                                                                                                                                let a1976 = 8u64;
                                                                                                                                                let a1977 = 0u64;
                                                                                                                                                nat_quot(a1975, a1976, a1977)
                                                                                                                                            };
                                                                                                                                            let a1979 = 2u64;
                                                                                                                                            let a1980 = 0u64;
                                                                                                                                            nat_quot(a1978, a1979, a1980)
                                                                                                                                        };
                                                                                                                                        nat_add(a1981, a1982)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1983, a1984)?
                                                                                                                                }, 5u64))
                                                                                                                            } else {
                                                                                                                                let a1985 = v3;
                                                                                                                                let a1986 = 2u64;
                                                                                                                                if nat_eq(a1985, a1986) {
                                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                        let a1987 = v1;
                                                                                                                                        let a1988 = 8u64;
                                                                                                                                        let a1989 = 0u64;
                                                                                                                                        nat_quot(a1987, a1988, a1989)
                                                                                                                                    }, 4u64))
                                                                                                                                } else {
                                                                                                                                    let a1990 = v3;
                                                                                                                                    let a1991 = 3u64;
                                                                                                                                    if nat_eq(a1990, a1991) {
                                                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                                                            let a2008 = 1u64;
                                                                                                                                            let a2009 = {
                                                                                                                                                let a2006 = 2u64;
                                                                                                                                                let a2007 = {
                                                                                                                                                    let a2003 = {
                                                                                                                                                        let a2000 = v10;
                                                                                                                                                        let a2001 = f10({
                                                                                                                                                            let a1998 = 3u64;
                                                                                                                                                            let a1999 = {
                                                                                                                                                                let a1995 = {
                                                                                                                                                                    let a1992 = v1;
                                                                                                                                                                    let a1993 = 16u64;
                                                                                                                                                                    let a1994 = 0u64;
                                                                                                                                                                    nat_quot(a1992, a1993, a1994)
                                                                                                                                                                };
                                                                                                                                                                let a1996 = 4u64;
                                                                                                                                                                let a1997 = 0u64;
                                                                                                                                                                nat_rem(a1995, a1996, a1997)
                                                                                                                                                            };
                                                                                                                                                            nat_mul(a1998, a1999)?
                                                                                                                                                        });
                                                                                                                                                        let a2002 = 0u64;
                                                                                                                                                        nat_quot(a2000, a2001, a2002)
                                                                                                                                                    };
                                                                                                                                                    let a2004 = 8u64;
                                                                                                                                                    let a2005 = 0u64;
                                                                                                                                                    nat_rem(a2003, a2004, a2005)
                                                                                                                                                };
                                                                                                                                                nat_mul(a2006, a2007)?
                                                                                                                                            };
                                                                                                                                            nat_add(a2008, a2009)?
                                                                                                                                        }, 0u64, 2u64))
                                                                                                                                    } else {
                                                                                                                                        let a2010 = v3;
                                                                                                                                        let a2011 = 4u64;
                                                                                                                                        if nat_eq(a2010, a2011) {
                                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a2012 = v3;
                                                                                                                                            let a2013 = 5u64;
                                                                                                                                            if nat_eq(a2012, a2013) {
                                                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                    let a2042 = 18600u64;
                                                                                                                                                    let a2043 = {
                                                                                                                                                        let a2040 = {
                                                                                                                                                            let a2035 = {
                                                                                                                                                                let a2033 = v8;
                                                                                                                                                                let a2034 = {
                                                                                                                                                                    let a2031 = {
                                                                                                                                                                        let a2028 = {
                                                                                                                                                                            let a2025 = v9;
                                                                                                                                                                            let a2026 = f10({
                                                                                                                                                                                let a2023 = 4u64;
                                                                                                                                                                                let a2024 = {
                                                                                                                                                                                    let a2020 = {
                                                                                                                                                                                        let a2017 = {
                                                                                                                                                                                            let a2014 = v1;
                                                                                                                                                                                            let a2015 = 32u64;
                                                                                                                                                                                            let a2016 = 0u64;
                                                                                                                                                                                            nat_rem(a2014, a2015, a2016)
                                                                                                                                                                                        };
                                                                                                                                                                                        let a2018 = 8u64;
                                                                                                                                                                                        let a2019 = 0u64;
                                                                                                                                                                                        nat_quot(a2017, a2018, a2019)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2021 = 2u64;
                                                                                                                                                                                    let a2022 = 0u64;
                                                                                                                                                                                    nat_rem(a2020, a2021, a2022)
                                                                                                                                                                                };
                                                                                                                                                                                nat_mul(a2023, a2024)?
                                                                                                                                                                            });
                                                                                                                                                                            let a2027 = 0u64;
                                                                                                                                                                            nat_quot(a2025, a2026, a2027)
                                                                                                                                                                        };
                                                                                                                                                                        let a2029 = 8u64;
                                                                                                                                                                        let a2030 = 0u64;
                                                                                                                                                                        nat_rem(a2028, a2029, a2030)
                                                                                                                                                                    };
                                                                                                                                                                    let a2032 = 256u64;
                                                                                                                                                                    nat_mul(a2031, a2032)?
                                                                                                                                                                };
                                                                                                                                                                nat_add(a2033, a2034)?
                                                                                                                                                            };
                                                                                                                                                            let a2036 = 8u64;
                                                                                                                                                            nat_mul(a2035, a2036)?
                                                                                                                                                        };
                                                                                                                                                        let a2041 = {
                                                                                                                                                            let a2037 = v1;
                                                                                                                                                            let a2038 = 8u64;
                                                                                                                                                            let a2039 = 0u64;
                                                                                                                                                            nat_rem(a2037, a2038, a2039)
                                                                                                                                                        };
                                                                                                                                                        nat_add(a2040, a2041)?
                                                                                                                                                    };
                                                                                                                                                    nat_add(a2042, a2043)?
                                                                                                                                                }, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a2044 = v3;
                                                                                                                                                let a2045 = 6u64;
                                                                                                                                                if nat_eq(a2044, a2045) {
                                                                                                                                                    Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                        let a2046 = v7;
                                                                                                                                                        let a2047 = 128u64;
                                                                                                                                                        if nat_lt(a2046, a2047) {
                                                                                                                                                            0u64
                                                                                                                                                        } else {
                                                                                                                                                            256u64
                                                                                                                                                        }
                                                                                                                                                    }, 3u64))
                                                                                                                                                } else {
                                                                                                                                                    let a2048 = v3;
                                                                                                                                                    let a2049 = 7u64;
                                                                                                                                                    if nat_eq(a2048, a2049) {
                                                                                                                                                        f4(0u64, v7, {
                                                                                                                                                            let a2069 = {
                                                                                                                                                                let a2066 = {
                                                                                                                                                                    let a2063 = v9;
                                                                                                                                                                    let a2064 = f10({
                                                                                                                                                                        let a2061 = {
                                                                                                                                                                            let a2059 = 4u64;
                                                                                                                                                                            let a2060 = {
                                                                                                                                                                                let a2056 = {
                                                                                                                                                                                    let a2053 = {
                                                                                                                                                                                        let a2050 = v1;
                                                                                                                                                                                        let a2051 = 32u64;
                                                                                                                                                                                        let a2052 = 0u64;
                                                                                                                                                                                        nat_rem(a2050, a2051, a2052)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2054 = 8u64;
                                                                                                                                                                                    let a2055 = 0u64;
                                                                                                                                                                                    nat_quot(a2053, a2054, a2055)
                                                                                                                                                                                };
                                                                                                                                                                                let a2057 = 2u64;
                                                                                                                                                                                let a2058 = 0u64;
                                                                                                                                                                                nat_rem(a2056, a2057, a2058)
                                                                                                                                                                            };
                                                                                                                                                                            nat_mul(a2059, a2060)?
                                                                                                                                                                        };
                                                                                                                                                                        let a2062 = 3u64;
                                                                                                                                                                        nat_add(a2061, a2062)?
                                                                                                                                                                    });
                                                                                                                                                                    let a2065 = 0u64;
                                                                                                                                                                    nat_quot(a2063, a2064, a2065)
                                                                                                                                                                };
                                                                                                                                                                let a2067 = 2u64;
                                                                                                                                                                let a2068 = 0u64;
                                                                                                                                                                nat_rem(a2066, a2067, a2068)
                                                                                                                                                            };
                                                                                                                                                            let a2070 = 1u64;
                                                                                                                                                            if nat_eq(a2069, a2070) {
                                                                                                                                                                3187671040u64
                                                                                                                                                            } else {
                                                                                                                                                                1040187392u64
                                                                                                                                                            }
                                                                                                                                                        }, 3u64)
                                                                                                                                                    } else {
                                                                                                                                                        let a2071 = v3;
                                                                                                                                                        let a2072 = 8u64;
                                                                                                                                                        if nat_eq(a2071, a2072) {
                                                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                                                        } else {
                                                                                                                                                            let a2073 = v3;
                                                                                                                                                            let a2074 = 9u64;
                                                                                                                                                            if nat_eq(a2073, a2074) {
                                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                                    let a2075 = v1;
                                                                                                                                                                    let a2076 = v2;
                                                                                                                                                                    nat_mul(a2075, a2076)?
                                                                                                                                                                }, v6))
                                                                                                                                                            } else {
                                                                                                                                                                Ok(Adt1::C0(0u64))
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        let a2077 = v0;
                                                                                                                        let a2078 = 20u64;
                                                                                                                        if nat_eq(a2077, a2078) {
                                                                                                                            let a2079 = v3;
                                                                                                                            let a2080 = 0u64;
                                                                                                                            if nat_eq(a2079, a2080) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a2084 = 2u64;
                                                                                                                                    let a2085 = {
                                                                                                                                        let a2081 = v1;
                                                                                                                                        let a2082 = 16u64;
                                                                                                                                        let a2083 = 0u64;
                                                                                                                                        nat_rem(a2081, a2082, a2083)
                                                                                                                                    };
                                                                                                                                    nat_add(a2084, a2085)?
                                                                                                                                }, 4u64))
                                                                                                                            } else {
                                                                                                                                let a2086 = v3;
                                                                                                                                let a2087 = 1u64;
                                                                                                                                if nat_eq(a2086, a2087) {
                                                                                                                                    Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                        let a2103 = 18568u64;
                                                                                                                                        let a2104 = {
                                                                                                                                            let a2101 = {
                                                                                                                                                let a2099 = {
                                                                                                                                                    let a2096 = {
                                                                                                                                                        let a2093 = v8;
                                                                                                                                                        let a2094 = {
                                                                                                                                                            let a2091 = {
                                                                                                                                                                let a2088 = v1;
                                                                                                                                                                let a2089 = 16u64;
                                                                                                                                                                let a2090 = 0u64;
                                                                                                                                                                nat_quot(a2088, a2089, a2090)
                                                                                                                                                            };
                                                                                                                                                            let a2092 = 0u64;
                                                                                                                                                            if nat_eq(a2091, a2092) {
                                                                                                                                                                1u64
                                                                                                                                                            } else {
                                                                                                                                                                16u64
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        let a2095 = 0u64;
                                                                                                                                                        nat_quot(a2093, a2094, a2095)
                                                                                                                                                    };
                                                                                                                                                    let a2097 = 16u64;
                                                                                                                                                    let a2098 = 0u64;
                                                                                                                                                    nat_rem(a2096, a2097, a2098)
                                                                                                                                                };
                                                                                                                                                let a2100 = 1u64;
                                                                                                                                                nat_mul(a2099, a2100)?
                                                                                                                                            };
                                                                                                                                            let a2102 = 0u64;
                                                                                                                                            nat_add(a2101, a2102)?
                                                                                                                                        };
                                                                                                                                        nat_add(a2103, a2104)?
                                                                                                                                    }, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a2105 = v3;
                                                                                                                                    let a2106 = 2u64;
                                                                                                                                    if nat_eq(a2105, a2106) {
                                                                                                                                        Ok(Adt1::C3(8u64, v7, {
                                                                                                                                            let a2107 = v7;
                                                                                                                                            let a2108 = 128u64;
                                                                                                                                            if nat_lt(a2107, a2108) {
                                                                                                                                                0u64
                                                                                                                                            } else {
                                                                                                                                                256u64
                                                                                                                                            }
                                                                                                                                        }, 2u64))
                                                                                                                                    } else {
                                                                                                                                        let a2109 = v3;
                                                                                                                                        let a2110 = 3u64;
                                                                                                                                        if nat_eq(a2109, a2110) {
                                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a2111 = v3;
                                                                                                                                            let a2112 = 4u64;
                                                                                                                                            if nat_eq(a2111, a2112) {
                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                    let a2113 = v1;
                                                                                                                                                    let a2114 = v2;
                                                                                                                                                    nat_mul(a2113, a2114)?
                                                                                                                                                }, v6))
                                                                                                                                            } else {
                                                                                                                                                Ok(Adt1::C0(0u64))
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            let a2115 = v0;
                                                                                                                            let a2116 = 23u64;
                                                                                                                            if nat_eq(a2115, a2116) {
                                                                                                                                let a2117 = v3;
                                                                                                                                let a2118 = 0u64;
                                                                                                                                if nat_eq(a2117, a2118) {
                                                                                                                                    Ok(Adt1::C2(0u64, 2u64, 2u64, 5u64))
                                                                                                                                } else {
                                                                                                                                    let a2119 = v3;
                                                                                                                                    let a2120 = 1u64;
                                                                                                                                    if nat_eq(a2119, a2120) {
                                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                            let a2127 = 4u64;
                                                                                                                                            let a2128 = {
                                                                                                                                                let a2124 = {
                                                                                                                                                    let a2121 = v1;
                                                                                                                                                    let a2122 = 32u64;
                                                                                                                                                    let a2123 = 0u64;
                                                                                                                                                    nat_quot(a2121, a2122, a2123)
                                                                                                                                                };
                                                                                                                                                let a2125 = 2u64;
                                                                                                                                                let a2126 = 0u64;
                                                                                                                                                nat_quot(a2124, a2125, a2126)
                                                                                                                                            };
                                                                                                                                            nat_add(a2127, a2128)?
                                                                                                                                        }, 4u64))
                                                                                                                                    } else {
                                                                                                                                        let a2129 = v3;
                                                                                                                                        let a2130 = 2u64;
                                                                                                                                        if nat_eq(a2129, a2130) {
                                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                                let a2158 = {
                                                                                                                                                    let a2142 = {
                                                                                                                                                        let a2139 = v8;
                                                                                                                                                        let a2140 = {
                                                                                                                                                            let a2137 = {
                                                                                                                                                                let a2134 = {
                                                                                                                                                                    let a2131 = v1;
                                                                                                                                                                    let a2132 = 32u64;
                                                                                                                                                                    let a2133 = 0u64;
                                                                                                                                                                    nat_quot(a2131, a2132, a2133)
                                                                                                                                                                };
                                                                                                                                                                let a2135 = 2u64;
                                                                                                                                                                let a2136 = 0u64;
                                                                                                                                                                nat_rem(a2134, a2135, a2136)
                                                                                                                                                            };
                                                                                                                                                            let a2138 = 0u64;
                                                                                                                                                            if nat_eq(a2137, a2138) {
                                                                                                                                                                1u64
                                                                                                                                                            } else {
                                                                                                                                                                16u64
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        let a2141 = 0u64;
                                                                                                                                                        nat_quot(a2139, a2140, a2141)
                                                                                                                                                    };
                                                                                                                                                    let a2143 = 16u64;
                                                                                                                                                    let a2144 = 0u64;
                                                                                                                                                    nat_rem(a2142, a2143, a2144)
                                                                                                                                                };
                                                                                                                                                let a2159 = {
                                                                                                                                                    let a2156 = {
                                                                                                                                                        let a2153 = {
                                                                                                                                                            let a2150 = v9;
                                                                                                                                                            let a2151 = f10({
                                                                                                                                                                let a2148 = 2u64;
                                                                                                                                                                let a2149 = {
                                                                                                                                                                    let a2145 = v1;
                                                                                                                                                                    let a2146 = 32u64;
                                                                                                                                                                    let a2147 = 0u64;
                                                                                                                                                                    nat_quot(a2145, a2146, a2147)
                                                                                                                                                                };
                                                                                                                                                                nat_mul(a2148, a2149)?
                                                                                                                                                            });
                                                                                                                                                            let a2152 = 0u64;
                                                                                                                                                            nat_quot(a2150, a2151, a2152)
                                                                                                                                                        };
                                                                                                                                                        let a2154 = 4u64;
                                                                                                                                                        let a2155 = 0u64;
                                                                                                                                                        nat_rem(a2153, a2154, a2155)
                                                                                                                                                    };
                                                                                                                                                    let a2157 = 16u64;
                                                                                                                                                    nat_mul(a2156, a2157)?
                                                                                                                                                };
                                                                                                                                                nat_add(a2158, a2159)?
                                                                                                                                            }, 32u64, 2u64))
                                                                                                                                        } else {
                                                                                                                                            let a2160 = v3;
                                                                                                                                            let a2161 = 3u64;
                                                                                                                                            if nat_eq(a2160, a2161) {
                                                                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                                                                            } else {
                                                                                                                                                let a2162 = v3;
                                                                                                                                                let a2163 = 4u64;
                                                                                                                                                if nat_eq(a2162, a2163) {
                                                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                                        let a2174 = 8u64;
                                                                                                                                                        let a2175 = {
                                                                                                                                                            let a2172 = {
                                                                                                                                                                let a2167 = {
                                                                                                                                                                    let a2164 = v1;
                                                                                                                                                                    let a2165 = 32u64;
                                                                                                                                                                    let a2166 = 0u64;
                                                                                                                                                                    nat_quot(a2164, a2165, a2166)
                                                                                                                                                                };
                                                                                                                                                                let a2168 = 16u64;
                                                                                                                                                                nat_mul(a2167, a2168)?
                                                                                                                                                            };
                                                                                                                                                            let a2173 = {
                                                                                                                                                                let a2169 = v1;
                                                                                                                                                                let a2170 = 16u64;
                                                                                                                                                                let a2171 = 0u64;
                                                                                                                                                                nat_rem(a2169, a2170, a2171)
                                                                                                                                                            };
                                                                                                                                                            nat_add(a2172, a2173)?
                                                                                                                                                        };
                                                                                                                                                        nat_add(a2174, a2175)?
                                                                                                                                                    }, 4u64))
                                                                                                                                                } else {
                                                                                                                                                    let a2176 = v3;
                                                                                                                                                    let a2177 = 5u64;
                                                                                                                                                    if nat_eq(a2176, a2177) {
                                                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                            let a2196 = 18568u64;
                                                                                                                                                            let a2197 = {
                                                                                                                                                                let a2194 = {
                                                                                                                                                                    let a2192 = {
                                                                                                                                                                        let a2189 = {
                                                                                                                                                                            let a2186 = v8;
                                                                                                                                                                            let a2187 = {
                                                                                                                                                                                let a2184 = {
                                                                                                                                                                                    let a2181 = {
                                                                                                                                                                                        let a2178 = v1;
                                                                                                                                                                                        let a2179 = 32u64;
                                                                                                                                                                                        let a2180 = 0u64;
                                                                                                                                                                                        nat_rem(a2178, a2179, a2180)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2182 = 16u64;
                                                                                                                                                                                    let a2183 = 0u64;
                                                                                                                                                                                    nat_quot(a2181, a2182, a2183)
                                                                                                                                                                                };
                                                                                                                                                                                let a2185 = 0u64;
                                                                                                                                                                                if nat_eq(a2184, a2185) {
                                                                                                                                                                                    1u64
                                                                                                                                                                                } else {
                                                                                                                                                                                    16u64
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            let a2188 = 0u64;
                                                                                                                                                                            nat_quot(a2186, a2187, a2188)
                                                                                                                                                                        };
                                                                                                                                                                        let a2190 = 16u64;
                                                                                                                                                                        let a2191 = 0u64;
                                                                                                                                                                        nat_rem(a2189, a2190, a2191)
                                                                                                                                                                    };
                                                                                                                                                                    let a2193 = 1u64;
                                                                                                                                                                    nat_mul(a2192, a2193)?
                                                                                                                                                                };
                                                                                                                                                                let a2195 = 0u64;
                                                                                                                                                                nat_add(a2194, a2195)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a2196, a2197)?
                                                                                                                                                        }, 3u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a2198 = v3;
                                                                                                                                                        let a2199 = 6u64;
                                                                                                                                                        if nat_eq(a2198, a2199) {
                                                                                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                                let a2200 = v7;
                                                                                                                                                                let a2201 = 128u64;
                                                                                                                                                                if nat_lt(a2200, a2201) {
                                                                                                                                                                    0u64
                                                                                                                                                                } else {
                                                                                                                                                                    256u64
                                                                                                                                                                }
                                                                                                                                                            }, 3u64))
                                                                                                                                                        } else {
                                                                                                                                                            let a2202 = v3;
                                                                                                                                                            let a2203 = 7u64;
                                                                                                                                                            if nat_eq(a2202, a2203) {
                                                                                                                                                                f4(2u64, v6, v7, 2u64)
                                                                                                                                                            } else {
                                                                                                                                                                let a2204 = v3;
                                                                                                                                                                let a2205 = 8u64;
                                                                                                                                                                if nat_eq(a2204, a2205) {
                                                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                                                        let a2206 = v1;
                                                                                                                                                                        let a2207 = v2;
                                                                                                                                                                        nat_mul(a2206, a2207)?
                                                                                                                                                                    }, v6))
                                                                                                                                                                } else {
                                                                                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                Ok(Adt1::C0(0u64))
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt1::C0(1u64))
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let a2208 = v0;
    let a2209 = 4u64;
    if if nat_lt(a2208, a2209) {
        let a2210 = v1;
        let a2211 = 4294967296u64;
        if nat_lt(a2210, a2211) {
            let a2212 = v2;
            let a2213 = 4294967296u64;
            if nat_lt(a2212, a2213) {
                let a2214 = v3;
                let a2215 = 8u64;
                nat_lt(a2214, a2215)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        if f12(v1) {
            Ok(Adt1::C3(11u64, f13(v1)?, 0u64, v3))
        } else {
            if f12(v2) {
                Ok(Adt1::C3(11u64, f13(v2)?, 0u64, v3))
            } else {
                let a2216 = v0;
                let a2217 = 0u64;
                if if nat_eq(a2216, a2217) {
                    let a2218 = f14(v1);
                    let a2219 = 2139095040u64;
                    if if nat_eq(a2218, a2219) {
                        let a2220 = f14(v2);
                        let a2221 = 2139095040u64;
                        nat_eq(a2220, a2221)
                    } else {
                        false
                    } {
                        let a2228 = {
                            let a2222 = v1;
                            let a2223 = 2147483648u64;
                            let a2224 = 0u64;
                            nat_quot(a2222, a2223, a2224)
                        };
                        let a2229 = {
                            let a2225 = v2;
                            let a2226 = 2147483648u64;
                            let a2227 = 0u64;
                            nat_quot(a2225, a2226, a2227)
                        };
                        !nat_eq(a2228, a2229)
                    } else {
                        false
                    }
                } else {
                    let a2230 = v0;
                    let a2231 = 1u64;
                    if nat_eq(a2230, a2231) {
                        let a2232 = f14(v1);
                        let a2233 = 2139095040u64;
                        if if nat_eq(a2232, a2233) {
                            let a2234 = f14(v2);
                            let a2235 = 2139095040u64;
                            nat_eq(a2234, a2235)
                        } else {
                            false
                        } {
                            let a2242 = {
                                let a2236 = v1;
                                let a2237 = 2147483648u64;
                                let a2238 = 0u64;
                                nat_quot(a2236, a2237, a2238)
                            };
                            let a2243 = {
                                let a2239 = v2;
                                let a2240 = 2147483648u64;
                                let a2241 = 0u64;
                                nat_quot(a2239, a2240, a2241)
                            };
                            nat_eq(a2242, a2243)
                        } else {
                            false
                        }
                    } else {
                        let a2244 = v0;
                        let a2245 = 2u64;
                        if nat_eq(a2244, a2245) {
                            let a2246 = f14(v1);
                            let a2247 = 2139095040u64;
                            if if nat_eq(a2246, a2247) {
                                let a2248 = f14(v2);
                                let a2249 = 0u64;
                                nat_eq(a2248, a2249)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a2250 = f14(v1);
                                let a2251 = 0u64;
                                if nat_eq(a2250, a2251) {
                                    let a2252 = f14(v2);
                                    let a2253 = 2139095040u64;
                                    nat_eq(a2252, a2253)
                                } else {
                                    false
                                }
                            }
                        } else {
                            let a2254 = v0;
                            let a2255 = 3u64;
                            if nat_eq(a2254, a2255) {
                                let a2256 = f14(v1);
                                let a2257 = 2139095040u64;
                                if if nat_eq(a2256, a2257) {
                                    let a2258 = f14(v2);
                                    let a2259 = 2139095040u64;
                                    nat_eq(a2258, a2259)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a2260 = f14(v1);
                                    let a2261 = 0u64;
                                    if nat_eq(a2260, a2261) {
                                        let a2262 = f14(v2);
                                        let a2263 = 0u64;
                                        nat_eq(a2262, a2263)
                                    } else {
                                        false
                                    }
                                }
                            } else {
                                false
                            }
                        }
                    }
                } {
                    Ok(Adt1::C3(11u64, 4290772992u64, 0u64, v3))
                } else {
                    Ok(Adt1::C3(v0, v1, v2, v3))
                }
            }
        }
    } else {
        Ok(Adt1::C0(0u64))
    }
}

pub fn f5(v0: u64) -> u64 {
    let a2264 = v0;
    let a2265 = 0u64;
    if nat_eq(a2264, a2265) {
        1u64
    } else {
        let a2266 = v0;
        let a2267 = 1u64;
        if nat_eq(a2266, a2267) {
            1u64
        } else {
            let a2268 = v0;
            let a2269 = 2u64;
            if nat_eq(a2268, a2269) {
                32u64
            } else {
                let a2270 = v0;
                let a2271 = 3u64;
                if nat_eq(a2270, a2271) {
                    32u64
                } else {
                    let a2272 = v0;
                    let a2273 = 6u64;
                    if nat_eq(a2272, a2273) {
                        32u64
                    } else {
                        let a2274 = v0;
                        let a2275 = 7u64;
                        if nat_eq(a2274, a2275) {
                            32u64
                        } else {
                            if f9(v0) {
                                let a2276 = v0;
                                let a2277 = 15u64;
                                if nat_eq(a2276, a2277) {
                                    256u64
                                } else {
                                    32u64
                                }
                            } else {
                                let a2278 = v0;
                                let a2279 = 10u64;
                                if nat_eq(a2278, a2279) {
                                    256u64
                                } else {
                                    let a2280 = v0;
                                    let a2281 = 11u64;
                                    if nat_eq(a2280, a2281) {
                                        256u64
                                    } else {
                                        let a2282 = v0;
                                        let a2283 = 12u64;
                                        if nat_eq(a2282, a2283) {
                                            256u64
                                        } else {
                                            let a2284 = v0;
                                            let a2285 = 13u64;
                                            if nat_eq(a2284, a2285) {
                                                256u64
                                            } else {
                                                let a2286 = v0;
                                                let a2287 = 14u64;
                                                if nat_eq(a2286, a2287) {
                                                    256u64
                                                } else {
                                                    let a2288 = v0;
                                                    let a2289 = 16u64;
                                                    if nat_eq(a2288, a2289) {
                                                        256u64
                                                    } else {
                                                        let a2290 = v0;
                                                        let a2291 = 17u64;
                                                        if nat_eq(a2290, a2291) {
                                                            256u64
                                                        } else {
                                                            let a2292 = v0;
                                                            let a2293 = 18u64;
                                                            if nat_eq(a2292, a2293) {
                                                                256u64
                                                            } else {
                                                                let a2294 = v0;
                                                                let a2295 = 19u64;
                                                                if nat_eq(a2294, a2295) {
                                                                    256u64
                                                                } else {
                                                                    let a2296 = v0;
                                                                    let a2297 = 20u64;
                                                                    if nat_eq(a2296, a2297) {
                                                                        32u64
                                                                    } else {
                                                                        let a2298 = v0;
                                                                        let a2299 = 21u64;
                                                                        if nat_eq(a2298, a2299) {
                                                                            256u64
                                                                        } else {
                                                                            let a2300 = v0;
                                                                            let a2301 = 22u64;
                                                                            if nat_eq(a2300, a2301) {
                                                                                256u64
                                                                            } else {
                                                                                let a2302 = v0;
                                                                                let a2303 = 23u64;
                                                                                if nat_eq(a2302, a2303) {
                                                                                    256u64
                                                                                } else {
                                                                                    let a2304 = v0;
                                                                                    let a2305 = 26u64;
                                                                                    if nat_eq(a2304, a2305) {
                                                                                        1u64
                                                                                    } else {
                                                                                        let a2306 = v0;
                                                                                        let a2307 = 29u64;
                                                                                        if nat_eq(a2306, a2307) {
                                                                                            256u64
                                                                                        } else {
                                                                                            let a2308 = v0;
                                                                                            let a2309 = 30u64;
                                                                                            if nat_eq(a2308, a2309) {
                                                                                                1u64
                                                                                            } else {
                                                                                                let a2310 = v0;
                                                                                                let a2311 = 34u64;
                                                                                                if nat_eq(a2310, a2311) {
                                                                                                    256u64
                                                                                                } else {
                                                                                                    let a2312 = v0;
                                                                                                    let a2313 = 35u64;
                                                                                                    if nat_eq(a2312, a2313) {
                                                                                                        256u64
                                                                                                    } else {
                                                                                                        let a2314 = v0;
                                                                                                        let a2315 = 39u64;
                                                                                                        if nat_eq(a2314, a2315) {
                                                                                                            32u64
                                                                                                        } else {
                                                                                                            let a2316 = v0;
                                                                                                            let a2317 = 40u64;
                                                                                                            if nat_eq(a2316, a2317) {
                                                                                                                64u64
                                                                                                            } else {
                                                                                                                let a2318 = v0;
                                                                                                                let a2319 = 41u64;
                                                                                                                if nat_eq(a2318, a2319) {
                                                                                                                    128u64
                                                                                                                } else {
                                                                                                                    let a2320 = v0;
                                                                                                                    let a2321 = 42u64;
                                                                                                                    if nat_eq(a2320, a2321) {
                                                                                                                        64u64
                                                                                                                    } else {
                                                                                                                        let a2322 = v0;
                                                                                                                        let a2323 = 43u64;
                                                                                                                        if nat_eq(a2322, a2323) {
                                                                                                                            1u64
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
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> u64 {
    let a2324 = v0;
    let a2325 = 0u64;
    if nat_eq(a2324, a2325) {
        4u64
    } else {
        let a2326 = v0;
        let a2327 = 1u64;
        if nat_eq(a2326, a2327) {
            2u64
        } else {
            let a2328 = v0;
            let a2329 = 2u64;
            if nat_eq(a2328, a2329) {
                18u64
            } else {
                let a2330 = v0;
                let a2331 = 3u64;
                if nat_eq(a2330, a2331) {
                    20u64
                } else {
                    let a2332 = v0;
                    let a2333 = 6u64;
                    if nat_eq(a2332, a2333) {
                        22u64
                    } else {
                        let a2334 = v0;
                        let a2335 = 7u64;
                        if nat_eq(a2334, a2335) {
                            24u64
                        } else {
                            if f9(v0) {
                                let a2336 = v0;
                                let a2337 = 8u64;
                                if nat_eq(a2336, a2337) {
                                    34u64
                                } else {
                                    let a2338 = v0;
                                    let a2339 = 9u64;
                                    if nat_eq(a2338, a2339) {
                                        36u64
                                    } else {
                                        292u64
                                    }
                                }
                            } else {
                                let a2340 = v0;
                                let a2341 = 10u64;
                                if nat_eq(a2340, a2341) {
                                    84u64
                                } else {
                                    let a2342 = v0;
                                    let a2343 = 11u64;
                                    if nat_eq(a2342, a2343) {
                                        110u64
                                    } else {
                                        let a2344 = v0;
                                        let a2345 = 12u64;
                                        if nat_eq(a2344, a2345) {
                                            144u64
                                        } else {
                                            let a2346 = v0;
                                            let a2347 = 13u64;
                                            if nat_eq(a2346, a2347) {
                                                176u64
                                            } else {
                                                let a2348 = v0;
                                                let a2349 = 14u64;
                                                if nat_eq(a2348, a2349) {
                                                    210u64
                                                } else {
                                                    let a2350 = v0;
                                                    let a2351 = 16u64;
                                                    if nat_eq(a2350, a2351) {
                                                        66u64
                                                    } else {
                                                        let a2352 = v0;
                                                        let a2353 = 17u64;
                                                        if nat_eq(a2352, a2353) {
                                                            74u64
                                                        } else {
                                                            let a2354 = v0;
                                                            let a2355 = 18u64;
                                                            if nat_eq(a2354, a2355) {
                                                                98u64
                                                            } else {
                                                                let a2356 = v0;
                                                                let a2357 = 19u64;
                                                                if nat_eq(a2356, a2357) {
                                                                    50u64
                                                                } else {
                                                                    let a2358 = v0;
                                                                    let a2359 = 20u64;
                                                                    if nat_eq(a2358, a2359) {
                                                                        18u64
                                                                    } else {
                                                                        let a2360 = v0;
                                                                        let a2361 = 21u64;
                                                                        if nat_eq(a2360, a2361) {
                                                                            110u64
                                                                        } else {
                                                                            let a2362 = v0;
                                                                            let a2363 = 22u64;
                                                                            if nat_eq(a2362, a2363) {
                                                                                82u64
                                                                            } else {
                                                                                let a2364 = v0;
                                                                                let a2365 = 23u64;
                                                                                if nat_eq(a2364, a2365) {
                                                                                    136u64
                                                                                } else {
                                                                                    let a2366 = v0;
                                                                                    let a2367 = 26u64;
                                                                                    if nat_eq(a2366, a2367) {
                                                                                        4u64
                                                                                    } else {
                                                                                        let a2368 = v0;
                                                                                        let a2369 = 29u64;
                                                                                        if nat_eq(a2368, a2369) {
                                                                                            56u64
                                                                                        } else {
                                                                                            let a2370 = v0;
                                                                                            let a2371 = 30u64;
                                                                                            if nat_eq(a2370, a2371) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a2372 = v0;
                                                                                                let a2373 = 34u64;
                                                                                                if nat_eq(a2372, a2373) {
                                                                                                    54u64
                                                                                                } else {
                                                                                                    let a2374 = v0;
                                                                                                    let a2375 = 35u64;
                                                                                                    if nat_eq(a2374, a2375) {
                                                                                                        66u64
                                                                                                    } else {
                                                                                                        let a2376 = v0;
                                                                                                        let a2377 = 39u64;
                                                                                                        if nat_eq(a2376, a2377) {
                                                                                                            17u64
                                                                                                        } else {
                                                                                                            let a2378 = v0;
                                                                                                            let a2379 = 40u64;
                                                                                                            if nat_eq(a2378, a2379) {
                                                                                                                36u64
                                                                                                            } else {
                                                                                                                let a2380 = v0;
                                                                                                                let a2381 = 41u64;
                                                                                                                if nat_eq(a2380, a2381) {
                                                                                                                    18u64
                                                                                                                } else {
                                                                                                                    let a2382 = v0;
                                                                                                                    let a2383 = 42u64;
                                                                                                                    if nat_eq(a2382, a2383) {
                                                                                                                        18u64
                                                                                                                    } else {
                                                                                                                        let a2384 = v0;
                                                                                                                        let a2385 = 43u64;
                                                                                                                        if nat_eq(a2384, a2385) {
                                                                                                                            1u64
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
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> u64 {
    let a2386 = v0;
    let a2387 = 0u64;
    if nat_eq(a2386, a2387) {
        0u64
    } else {
        let a2388 = v0;
        let a2389 = 1u64;
        if nat_eq(a2388, a2389) {
            0u64
        } else {
            let a2390 = v0;
            let a2391 = 2u64;
            if nat_eq(a2390, a2391) {
                2u64
            } else {
                let a2392 = v0;
                let a2393 = 3u64;
                if nat_eq(a2392, a2393) {
                    4u64
                } else {
                    let a2394 = v0;
                    let a2395 = 6u64;
                    if nat_eq(a2394, a2395) {
                        2u64
                    } else {
                        let a2396 = v0;
                        let a2397 = 7u64;
                        if nat_eq(a2396, a2397) {
                            4u64
                        } else {
                            if f9(v0) {
                                let a2398 = v0;
                                let a2399 = 15u64;
                                if nat_eq(a2398, a2399) {
                                    1u64
                                } else {
                                    2u64
                                }
                            } else {
                                let a2400 = v0;
                                let a2401 = 10u64;
                                if nat_eq(a2400, a2401) {
                                    4u64
                                } else {
                                    let a2402 = v0;
                                    let a2403 = 11u64;
                                    if nat_eq(a2402, a2403) {
                                        2u64
                                    } else {
                                        let a2404 = v0;
                                        let a2405 = 12u64;
                                        if nat_eq(a2404, a2405) {
                                            4u64
                                        } else {
                                            let a2406 = v0;
                                            let a2407 = 13u64;
                                            if nat_eq(a2406, a2407) {
                                                4u64
                                            } else {
                                                let a2408 = v0;
                                                let a2409 = 14u64;
                                                if nat_eq(a2408, a2409) {
                                                    2u64
                                                } else {
                                                    let a2410 = v0;
                                                    let a2411 = 16u64;
                                                    if nat_eq(a2410, a2411) {
                                                        2u64
                                                    } else {
                                                        let a2412 = v0;
                                                        let a2413 = 17u64;
                                                        if nat_eq(a2412, a2413) {
                                                            2u64
                                                        } else {
                                                            let a2414 = v0;
                                                            let a2415 = 18u64;
                                                            if nat_eq(a2414, a2415) {
                                                                2u64
                                                            } else {
                                                                let a2416 = v0;
                                                                let a2417 = 19u64;
                                                                if nat_eq(a2416, a2417) {
                                                                    2u64
                                                                } else {
                                                                    let a2418 = v0;
                                                                    let a2419 = 20u64;
                                                                    if nat_eq(a2418, a2419) {
                                                                        2u64
                                                                    } else {
                                                                        let a2420 = v0;
                                                                        let a2421 = 21u64;
                                                                        if nat_eq(a2420, a2421) {
                                                                            2u64
                                                                        } else {
                                                                            let a2422 = v0;
                                                                            let a2423 = 22u64;
                                                                            if nat_eq(a2422, a2423) {
                                                                                2u64
                                                                            } else {
                                                                                let a2424 = v0;
                                                                                let a2425 = 23u64;
                                                                                if nat_eq(a2424, a2425) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a2426 = v0;
                                                                                    let a2427 = 26u64;
                                                                                    if nat_eq(a2426, a2427) {
                                                                                        0u64
                                                                                    } else {
                                                                                        let a2428 = v0;
                                                                                        let a2429 = 29u64;
                                                                                        if nat_eq(a2428, a2429) {
                                                                                            3u64
                                                                                        } else {
                                                                                            let a2430 = v0;
                                                                                            let a2431 = 30u64;
                                                                                            if nat_eq(a2430, a2431) {
                                                                                                0u64
                                                                                            } else {
                                                                                                let a2432 = v0;
                                                                                                let a2433 = 34u64;
                                                                                                if nat_eq(a2432, a2433) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a2434 = v0;
                                                                                                    let a2435 = 35u64;
                                                                                                    if nat_eq(a2434, a2435) {
                                                                                                        2u64
                                                                                                    } else {
                                                                                                        let a2436 = v0;
                                                                                                        let a2437 = 39u64;
                                                                                                        if nat_eq(a2436, a2437) {
                                                                                                            2u64
                                                                                                        } else {
                                                                                                            let a2438 = v0;
                                                                                                            let a2439 = 40u64;
                                                                                                            if nat_eq(a2438, a2439) {
                                                                                                                0u64
                                                                                                            } else {
                                                                                                                let a2440 = v0;
                                                                                                                let a2441 = 41u64;
                                                                                                                if nat_eq(a2440, a2441) {
                                                                                                                    2u64
                                                                                                                } else {
                                                                                                                    let a2442 = v0;
                                                                                                                    let a2443 = 42u64;
                                                                                                                    if nat_eq(a2442, a2443) {
                                                                                                                        2u64
                                                                                                                    } else {
                                                                                                                        let a2444 = v0;
                                                                                                                        let a2445 = 43u64;
                                                                                                                        let _ = nat_eq(a2444, a2445);
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
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> u64 {
    let a2446 = v0;
    let a2447 = 0u64;
    if nat_eq(a2446, a2447) {
        2u64
    } else {
        let a2448 = v0;
        let a2449 = 26u64;
        if nat_eq(a2448, a2449) {
            2u64
        } else {
            let a2450 = v0;
            let a2451 = 1u64;
            if nat_eq(a2450, a2451) {
                3u64
            } else {
                let a2452 = v0;
                let a2453 = 30u64;
                if nat_eq(a2452, a2453) {
                    3u64
                } else {
                    let a2454 = v0;
                    let a2455 = 43u64;
                    if nat_eq(a2454, a2455) {
                        3u64
                    } else {
                        let a2456 = v0;
                        let a2457 = 2u64;
                        if nat_eq(a2456, a2457) {
                            4u64
                        } else {
                            let a2458 = v0;
                            let a2459 = 3u64;
                            if nat_eq(a2458, a2459) {
                                5u64
                            } else {
                                let a2460 = v0;
                                let a2461 = 6u64;
                                if nat_eq(a2460, a2461) {
                                    5u64
                                } else {
                                    let a2462 = v0;
                                    let a2463 = 7u64;
                                    if nat_eq(a2462, a2463) {
                                        6u64
                                    } else {
                                        if f9(v0) {
                                            4u64
                                        } else {
                                            let a2464 = v0;
                                            let a2465 = 41u64;
                                            if nat_eq(a2464, a2465) {
                                                3u64
                                            } else {
                                                let a2466 = v0;
                                                let a2467 = 42u64;
                                                if nat_eq(a2466, a2467) {
                                                    4u64
                                                } else {
                                                    let a2468 = v0;
                                                    let a2469 = 39u64;
                                                    if nat_eq(a2468, a2469) {
                                                        4u64
                                                    } else {
                                                        let a2470 = v0;
                                                        let a2471 = 40u64;
                                                        if nat_eq(a2470, a2471) {
                                                            6u64
                                                        } else {
                                                            let a2472 = v0;
                                                            let a2473 = 10u64;
                                                            if nat_eq(a2472, a2473) {
                                                                10u64
                                                            } else {
                                                                let a2474 = v0;
                                                                let a2475 = 11u64;
                                                                if nat_eq(a2474, a2475) {
                                                                    9u64
                                                                } else {
                                                                    let a2476 = v0;
                                                                    let a2477 = 12u64;
                                                                    if nat_eq(a2476, a2477) {
                                                                        12u64
                                                                    } else {
                                                                        let a2478 = v0;
                                                                        let a2479 = 13u64;
                                                                        if nat_eq(a2478, a2479) {
                                                                            13u64
                                                                        } else {
                                                                            let a2480 = v0;
                                                                            let a2481 = 14u64;
                                                                            if nat_eq(a2480, a2481) {
                                                                                8u64
                                                                            } else {
                                                                                let a2482 = v0;
                                                                                let a2483 = 34u64;
                                                                                if nat_eq(a2482, a2483) {
                                                                                    4u64
                                                                                } else {
                                                                                    let a2484 = v0;
                                                                                    let a2485 = 35u64;
                                                                                    if nat_eq(a2484, a2485) {
                                                                                        4u64
                                                                                    } else {
                                                                                        let a2486 = v0;
                                                                                        let a2487 = 16u64;
                                                                                        if nat_eq(a2486, a2487) {
                                                                                            12u64
                                                                                        } else {
                                                                                            let a2488 = v0;
                                                                                            let a2489 = 17u64;
                                                                                            if nat_eq(a2488, a2489) {
                                                                                                12u64
                                                                                            } else {
                                                                                                let a2490 = v0;
                                                                                                let a2491 = 22u64;
                                                                                                if nat_eq(a2490, a2491) {
                                                                                                    13u64
                                                                                                } else {
                                                                                                    let a2492 = v0;
                                                                                                    let a2493 = 18u64;
                                                                                                    if nat_eq(a2492, a2493) {
                                                                                                        12u64
                                                                                                    } else {
                                                                                                        let a2494 = v0;
                                                                                                        let a2495 = 21u64;
                                                                                                        if nat_eq(a2494, a2495) {
                                                                                                            11u64
                                                                                                        } else {
                                                                                                            let a2496 = v0;
                                                                                                            let a2497 = 19u64;
                                                                                                            if nat_eq(a2496, a2497) {
                                                                                                                9u64
                                                                                                            } else {
                                                                                                                let a2498 = v0;
                                                                                                                let a2499 = 29u64;
                                                                                                                if nat_eq(a2498, a2499) {
                                                                                                                    10u64
                                                                                                                } else {
                                                                                                                    let a2500 = v0;
                                                                                                                    let a2501 = 20u64;
                                                                                                                    if nat_eq(a2500, a2501) {
                                                                                                                        5u64
                                                                                                                    } else {
                                                                                                                        let a2502 = v0;
                                                                                                                        let a2503 = 23u64;
                                                                                                                        if nat_eq(a2502, a2503) {
                                                                                                                            9u64
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
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> bool {
    let a2504 = v0;
    let a2505 = 8u64;
    if if nat_eq(a2504, a2505) {
        true
    } else {
        let a2506 = v0;
        let a2507 = 9u64;
        nat_eq(a2506, a2507)
    } {
        true
    } else {
        let a2508 = v0;
        let a2509 = 15u64;
        nat_eq(a2508, a2509)
    }
}

pub fn f10(v0: u64) -> u64 {
    let a2510 = v0;
    let a2511 = 0u64;
    if nat_eq(a2510, a2511) {
        1u64
    } else {
        let a2512 = v0;
        let a2513 = 1u64;
        if nat_eq(a2512, a2513) {
            2u64
        } else {
            let a2514 = v0;
            let a2515 = 2u64;
            if nat_eq(a2514, a2515) {
                4u64
            } else {
                let a2516 = v0;
                let a2517 = 3u64;
                if nat_eq(a2516, a2517) {
                    8u64
                } else {
                    let a2518 = v0;
                    let a2519 = 4u64;
                    if nat_eq(a2518, a2519) {
                        16u64
                    } else {
                        let a2520 = v0;
                        let a2521 = 5u64;
                        if nat_eq(a2520, a2521) {
                            32u64
                        } else {
                            let a2522 = v0;
                            let a2523 = 6u64;
                            if nat_eq(a2522, a2523) {
                                64u64
                            } else {
                                let a2524 = v0;
                                let a2525 = 7u64;
                                if nat_eq(a2524, a2525) {
                                    128u64
                                } else {
                                    let a2526 = v0;
                                    let a2527 = 8u64;
                                    if nat_eq(a2526, a2527) {
                                        256u64
                                    } else {
                                        let a2528 = v0;
                                        let a2529 = 9u64;
                                        if nat_eq(a2528, a2529) {
                                            512u64
                                        } else {
                                            let a2530 = v0;
                                            let a2531 = 10u64;
                                            if nat_eq(a2530, a2531) {
                                                1024u64
                                            } else {
                                                let a2532 = v0;
                                                let a2533 = 11u64;
                                                if nat_eq(a2532, a2533) {
                                                    2048u64
                                                } else {
                                                    let a2534 = v0;
                                                    let a2535 = 12u64;
                                                    if nat_eq(a2534, a2535) {
                                                        4096u64
                                                    } else {
                                                        let a2536 = v0;
                                                        let a2537 = 13u64;
                                                        if nat_eq(a2536, a2537) {
                                                            8192u64
                                                        } else {
                                                            let a2538 = v0;
                                                            let a2539 = 14u64;
                                                            if nat_eq(a2538, a2539) {
                                                                16384u64
                                                            } else {
                                                                let a2540 = v0;
                                                                let a2541 = 15u64;
                                                                if nat_eq(a2540, a2541) {
                                                                    32768u64
                                                                } else {
                                                                    let a2542 = v0;
                                                                    let a2543 = 16u64;
                                                                    if nat_eq(a2542, a2543) {
                                                                        65536u64
                                                                    } else {
                                                                        let a2544 = v0;
                                                                        let a2545 = 17u64;
                                                                        if nat_eq(a2544, a2545) {
                                                                            131072u64
                                                                        } else {
                                                                            let a2546 = v0;
                                                                            let a2547 = 18u64;
                                                                            if nat_eq(a2546, a2547) {
                                                                                262144u64
                                                                            } else {
                                                                                let a2548 = v0;
                                                                                let a2549 = 19u64;
                                                                                if nat_eq(a2548, a2549) {
                                                                                    524288u64
                                                                                } else {
                                                                                    let a2550 = v0;
                                                                                    let a2551 = 20u64;
                                                                                    if nat_eq(a2550, a2551) {
                                                                                        1048576u64
                                                                                    } else {
                                                                                        let a2552 = v0;
                                                                                        let a2553 = 21u64;
                                                                                        if nat_eq(a2552, a2553) {
                                                                                            2097152u64
                                                                                        } else {
                                                                                            let a2554 = v0;
                                                                                            let a2555 = 22u64;
                                                                                            if nat_eq(a2554, a2555) {
                                                                                                4194304u64
                                                                                            } else {
                                                                                                let a2556 = v0;
                                                                                                let a2557 = 23u64;
                                                                                                if nat_eq(a2556, a2557) {
                                                                                                    8388608u64
                                                                                                } else {
                                                                                                    let a2558 = v0;
                                                                                                    let a2559 = 24u64;
                                                                                                    if nat_eq(a2558, a2559) {
                                                                                                        16777216u64
                                                                                                    } else {
                                                                                                        let a2560 = v0;
                                                                                                        let a2561 = 25u64;
                                                                                                        if nat_eq(a2560, a2561) {
                                                                                                            33554432u64
                                                                                                        } else {
                                                                                                            let a2562 = v0;
                                                                                                            let a2563 = 26u64;
                                                                                                            if nat_eq(a2562, a2563) {
                                                                                                                67108864u64
                                                                                                            } else {
                                                                                                                let a2564 = v0;
                                                                                                                let a2565 = 27u64;
                                                                                                                if nat_eq(a2564, a2565) {
                                                                                                                    134217728u64
                                                                                                                } else {
                                                                                                                    let a2566 = v0;
                                                                                                                    let a2567 = 28u64;
                                                                                                                    if nat_eq(a2566, a2567) {
                                                                                                                        268435456u64
                                                                                                                    } else {
                                                                                                                        let a2568 = v0;
                                                                                                                        let a2569 = 29u64;
                                                                                                                        if nat_eq(a2568, a2569) {
                                                                                                                            536870912u64
                                                                                                                        } else {
                                                                                                                            let a2570 = v0;
                                                                                                                            let a2571 = 30u64;
                                                                                                                            if nat_eq(a2570, a2571) {
                                                                                                                                1073741824u64
                                                                                                                            } else {
                                                                                                                                let a2572 = v0;
                                                                                                                                let a2573 = 31u64;
                                                                                                                                if nat_eq(a2572, a2573) {
                                                                                                                                    2147483648u64
                                                                                                                                } else {
                                                                                                                                    1u64
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: bool, v2: u64) -> Adt1 {
    let a2574 = v0;
    let a2575 = 4294967296u64;
    if if nat_lt(a2574, a2575) {
        let a2576 = v2;
        let a2577 = 8u64;
        nat_lt(a2576, a2577)
    } else {
        false
    } {
        Adt1::C3(if v1 {
            6u64
        } else {
            11u64
        }, v0, 0u64, v2)
    } else {
        Adt1::C0(0u64)
    }
}

pub fn f12(v0: u64) -> bool {
    let a2578 = 2139095040u64;
    let a2579 = f14(v0);
    nat_lt(a2578, a2579)
}

pub fn f13(v0: u64) -> R<u64> {
    let a2588 = v0;
    let a2589 = {
        let a2586 = {
            let a2583 = {
                let a2580 = v0;
                let a2581 = 4194304u64;
                let a2582 = 0u64;
                nat_quot(a2580, a2581, a2582)
            };
            let a2584 = 2u64;
            let a2585 = 0u64;
            nat_rem(a2583, a2584, a2585)
        };
        let a2587 = 0u64;
        if nat_eq(a2586, a2587) {
            4194304u64
        } else {
            0u64
        }
    };
    nat_add(a2588, a2589)
}

pub fn f14(v0: u64) -> u64 {
    let a2590 = v0;
    let a2591 = 2147483648u64;
    let a2592 = 0u64;
    nat_rem(a2590, a2591, a2592)
}
