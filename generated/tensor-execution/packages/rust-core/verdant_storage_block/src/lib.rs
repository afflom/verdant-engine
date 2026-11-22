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
                    let a48 = v0;
                    let a49 = 8u64;
                    if nat_eq(a48, a49) {
                        let a50 = v1;
                        let a51 = 0u64;
                        if nat_eq(a50, a51) {
                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                        } else {
                            let a52 = v1;
                            let a53 = 1u64;
                            if nat_eq(a52, a53) {
                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                            } else {
                                Ok(Adt1::C0(0u64))
                            }
                        }
                    } else {
                        let a54 = v0;
                        let a55 = 10u64;
                        if nat_eq(a54, a55) {
                            let a56 = v1;
                            let a57 = 0u64;
                            if nat_eq(a56, a57) {
                                Ok(Adt1::C2(0u64, 2u64, 80u64, 0u64))
                            } else {
                                let a58 = v1;
                                let a59 = 1u64;
                                if nat_eq(a58, a59) {
                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                } else {
                                    let a60 = v1;
                                    let a61 = 2u64;
                                    if nat_eq(a60, a61) {
                                        Ok(Adt1::C2(0u64, 2u64, 82u64, 1u64))
                                    } else {
                                        let a62 = v1;
                                        let a63 = 3u64;
                                        if nat_eq(a62, a63) {
                                            Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                        } else {
                                            Ok(Adt1::C0(0u64))
                                        }
                                    }
                                }
                            }
                        } else {
                            let a64 = v0;
                            let a65 = 11u64;
                            if nat_eq(a64, a65) {
                                let a66 = v1;
                                let a67 = 0u64;
                                if nat_eq(a66, a67) {
                                    Ok(Adt1::C2(0u64, 2u64, 108u64, 0u64))
                                } else {
                                    let a68 = v1;
                                    let a69 = 1u64;
                                    if nat_eq(a68, a69) {
                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                    } else {
                                        Ok(Adt1::C0(0u64))
                                    }
                                }
                            } else {
                                let a70 = v0;
                                let a71 = 12u64;
                                if nat_eq(a70, a71) {
                                    let a72 = v1;
                                    let a73 = 0u64;
                                    if nat_eq(a72, a73) {
                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                    } else {
                                        let a74 = v1;
                                        let a75 = 1u64;
                                        if nat_eq(a74, a75) {
                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                        } else {
                                            let a76 = v1;
                                            let a77 = 2u64;
                                            if nat_eq(a76, a77) {
                                                Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                                            } else {
                                                let a78 = v1;
                                                let a79 = 3u64;
                                                if nat_eq(a78, a79) {
                                                    Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                                } else {
                                                    Ok(Adt1::C0(0u64))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let a80 = v0;
                                    let a81 = 13u64;
                                    if nat_eq(a80, a81) {
                                        let a82 = v1;
                                        let a83 = 0u64;
                                        if nat_eq(a82, a83) {
                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                        } else {
                                            let a84 = v1;
                                            let a85 = 1u64;
                                            if nat_eq(a84, a85) {
                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                            } else {
                                                let a86 = v1;
                                                let a87 = 2u64;
                                                if nat_eq(a86, a87) {
                                                    Ok(Adt1::C2(0u64, 2u64, 2u64, 1u64))
                                                } else {
                                                    let a88 = v1;
                                                    let a89 = 3u64;
                                                    if nat_eq(a88, a89) {
                                                        Ok(Adt1::C3(10u64, 1u64, v3, 1u64))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let a90 = v0;
                                        let a91 = 14u64;
                                        if nat_eq(a90, a91) {
                                            let a92 = v1;
                                            let a93 = 0u64;
                                            if nat_eq(a92, a93) {
                                                Ok(Adt1::C2(0u64, 2u64, 208u64, 0u64))
                                            } else {
                                                let a94 = v1;
                                                let a95 = 1u64;
                                                if nat_eq(a94, a95) {
                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                } else {
                                                    Ok(Adt1::C0(0u64))
                                                }
                                            }
                                        } else {
                                            let a96 = v0;
                                            let a97 = 16u64;
                                            if nat_eq(a96, a97) {
                                                let a98 = v1;
                                                let a99 = 0u64;
                                                if nat_eq(a98, a99) {
                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                } else {
                                                    let a100 = v1;
                                                    let a101 = 1u64;
                                                    if nat_eq(a100, a101) {
                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            } else {
                                                let a102 = v0;
                                                let a103 = 17u64;
                                                if nat_eq(a102, a103) {
                                                    let a104 = v1;
                                                    let a105 = 0u64;
                                                    if nat_eq(a104, a105) {
                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                    } else {
                                                        let a106 = v1;
                                                        let a107 = 1u64;
                                                        if nat_eq(a106, a107) {
                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                        } else {
                                                            Ok(Adt1::C0(0u64))
                                                        }
                                                    }
                                                } else {
                                                    let a108 = v0;
                                                    let a109 = 18u64;
                                                    if nat_eq(a108, a109) {
                                                        let a110 = v1;
                                                        let a111 = 0u64;
                                                        if nat_eq(a110, a111) {
                                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                        } else {
                                                            let a112 = v1;
                                                            let a113 = 1u64;
                                                            if nat_eq(a112, a113) {
                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                            } else {
                                                                Ok(Adt1::C0(0u64))
                                                            }
                                                        }
                                                    } else {
                                                        let a114 = v0;
                                                        let a115 = 19u64;
                                                        if nat_eq(a114, a115) {
                                                            let a116 = v1;
                                                            let a117 = 0u64;
                                                            if nat_eq(a116, a117) {
                                                                Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                            } else {
                                                                let a118 = v1;
                                                                let a119 = 1u64;
                                                                if nat_eq(a118, a119) {
                                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                } else {
                                                                    Ok(Adt1::C0(0u64))
                                                                }
                                                            }
                                                        } else {
                                                            let a120 = v0;
                                                            let a121 = 20u64;
                                                            if nat_eq(a120, a121) {
                                                                let a122 = v1;
                                                                let a123 = 0u64;
                                                                if nat_eq(a122, a123) {
                                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                } else {
                                                                    let a124 = v1;
                                                                    let a125 = 1u64;
                                                                    if nat_eq(a124, a125) {
                                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                    } else {
                                                                        Ok(Adt1::C0(0u64))
                                                                    }
                                                                }
                                                            } else {
                                                                let a126 = v0;
                                                                let a127 = 21u64;
                                                                if nat_eq(a126, a127) {
                                                                    let a128 = v1;
                                                                    let a129 = 0u64;
                                                                    if nat_eq(a128, a129) {
                                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                    } else {
                                                                        let a130 = v1;
                                                                        let a131 = 1u64;
                                                                        if nat_eq(a130, a131) {
                                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                        } else {
                                                                            Ok(Adt1::C0(0u64))
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a132 = v0;
                                                                    let a133 = 22u64;
                                                                    if nat_eq(a132, a133) {
                                                                        let a134 = v1;
                                                                        let a135 = 0u64;
                                                                        if nat_eq(a134, a135) {
                                                                            Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                        } else {
                                                                            let a136 = v1;
                                                                            let a137 = 1u64;
                                                                            if nat_eq(a136, a137) {
                                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                            } else {
                                                                                Ok(Adt1::C0(0u64))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let a138 = v0;
                                                                        let a139 = 23u64;
                                                                        if nat_eq(a138, a139) {
                                                                            let a140 = v1;
                                                                            let a141 = 0u64;
                                                                            if nat_eq(a140, a141) {
                                                                                Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                            } else {
                                                                                let a142 = v1;
                                                                                let a143 = 1u64;
                                                                                if nat_eq(a142, a143) {
                                                                                    Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                } else {
                                                                                    Ok(Adt1::C0(0u64))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            let a144 = v0;
                                                                            let a145 = 29u64;
                                                                            if nat_eq(a144, a145) {
                                                                                let a146 = v1;
                                                                                let a147 = 0u64;
                                                                                if nat_eq(a146, a147) {
                                                                                    Ok(Adt1::C2(0u64, 4u64, 48u64, 6u64))
                                                                                } else {
                                                                                    let a148 = v1;
                                                                                    let a149 = 1u64;
                                                                                    if nat_eq(a148, a149) {
                                                                                        Ok(Adt1::C2(0u64, 4u64, 52u64, 7u64))
                                                                                    } else {
                                                                                        let a150 = v1;
                                                                                        let a151 = 2u64;
                                                                                        if nat_eq(a150, a151) {
                                                                                            Ok(Adt1::C3(10u64, 1u64, {
                                                                                                let a186 = {
                                                                                                    let a166 = {
                                                                                                        let a155 = {
                                                                                                            let a152 = v8;
                                                                                                            let a153 = 4096u64;
                                                                                                            let a154 = 0u64;
                                                                                                            nat_quot(a152, a153, a154)
                                                                                                        };
                                                                                                        let a156 = 16u64;
                                                                                                        let a157 = 0u64;
                                                                                                        nat_rem(a155, a156, a157)
                                                                                                    };
                                                                                                    let a167 = {
                                                                                                        let a164 = {
                                                                                                            let a161 = {
                                                                                                                let a158 = v8;
                                                                                                                let a159 = 268435456u64;
                                                                                                                let a160 = 0u64;
                                                                                                                nat_quot(a158, a159, a160)
                                                                                                            };
                                                                                                            let a162 = 16u64;
                                                                                                            let a163 = 0u64;
                                                                                                            nat_rem(a161, a162, a163)
                                                                                                        };
                                                                                                        let a165 = 16u64;
                                                                                                        nat_mul(a164, a165)?
                                                                                                    };
                                                                                                    nat_add(a166, a167)?
                                                                                                };
                                                                                                let a187 = {
                                                                                                    let a184 = {
                                                                                                        let a174 = {
                                                                                                            let a171 = {
                                                                                                                let a168 = v9;
                                                                                                                let a169 = 4096u64;
                                                                                                                let a170 = 0u64;
                                                                                                                nat_quot(a168, a169, a170)
                                                                                                            };
                                                                                                            let a172 = 16u64;
                                                                                                            let a173 = 0u64;
                                                                                                            nat_rem(a171, a172, a173)
                                                                                                        };
                                                                                                        let a175 = 256u64;
                                                                                                        nat_mul(a174, a175)?
                                                                                                    };
                                                                                                    let a185 = {
                                                                                                        let a182 = {
                                                                                                            let a179 = {
                                                                                                                let a176 = v9;
                                                                                                                let a177 = 268435456u64;
                                                                                                                let a178 = 0u64;
                                                                                                                nat_quot(a176, a177, a178)
                                                                                                            };
                                                                                                            let a180 = 16u64;
                                                                                                            let a181 = 0u64;
                                                                                                            nat_rem(a179, a180, a181)
                                                                                                        };
                                                                                                        let a183 = 4096u64;
                                                                                                        nat_mul(a182, a183)?
                                                                                                    };
                                                                                                    nat_add(a184, a185)?
                                                                                                };
                                                                                                nat_add(a186, a187)?
                                                                                            }, 0u64))
                                                                                        } else {
                                                                                            Ok(Adt1::C0(0u64))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a188 = v0;
                                                                                let a189 = 34u64;
                                                                                if nat_eq(a188, a189) {
                                                                                    let a190 = v1;
                                                                                    let a191 = 0u64;
                                                                                    if nat_eq(a190, a191) {
                                                                                        Ok(Adt1::C2(0u64, 2u64, 52u64, 0u64))
                                                                                    } else {
                                                                                        let a192 = v1;
                                                                                        let a193 = 1u64;
                                                                                        if nat_eq(a192, a193) {
                                                                                            Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                        } else {
                                                                                            Ok(Adt1::C0(0u64))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    let a194 = v0;
                                                                                    let a195 = 35u64;
                                                                                    if nat_eq(a194, a195) {
                                                                                        let a196 = v1;
                                                                                        let a197 = 0u64;
                                                                                        if nat_eq(a196, a197) {
                                                                                            Ok(Adt1::C2(0u64, 2u64, 64u64, 0u64))
                                                                                        } else {
                                                                                            let a198 = v1;
                                                                                            let a199 = 1u64;
                                                                                            if nat_eq(a198, a199) {
                                                                                                Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                            } else {
                                                                                                Ok(Adt1::C0(0u64))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let a200 = v0;
                                                                                        let a201 = 39u64;
                                                                                        if nat_eq(a200, a201) {
                                                                                            let a202 = v1;
                                                                                            let a203 = 0u64;
                                                                                            if nat_eq(a202, a203) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, 0u64, 0u64))
                                                                                            } else {
                                                                                                let a204 = v1;
                                                                                                let a205 = 1u64;
                                                                                                if nat_eq(a204, a205) {
                                                                                                    Ok(Adt1::C3(9u64, 1u64, v2, 0u64))
                                                                                                } else {
                                                                                                    Ok(Adt1::C0(0u64))
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            let a206 = v0;
                                                                                            let a207 = 41u64;
                                                                                            if nat_eq(a206, a207) {
                                                                                                let a208 = v1;
                                                                                                let a209 = 0u64;
                                                                                                if nat_eq(a208, a209) {
                                                                                                    Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                                                } else {
                                                                                                    let a210 = v1;
                                                                                                    let a211 = 1u64;
                                                                                                    if nat_eq(a210, a211) {
                                                                                                        Ok(Adt1::C3(10u64, 1u64, v2, 0u64))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C0(0u64))
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                let a212 = v0;
                                                                                                let a213 = 42u64;
                                                                                                if nat_eq(a212, a213) {
                                                                                                    let a214 = v1;
                                                                                                    let a215 = 0u64;
                                                                                                    if nat_eq(a214, a215) {
                                                                                                        Ok(Adt1::C2(0u64, 2u64, 0u64, 0u64))
                                                                                                    } else {
                                                                                                        let a216 = v1;
                                                                                                        let a217 = 1u64;
                                                                                                        if nat_eq(a216, a217) {
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
    let a218 = v1;
    let a219 = f5(v0);
    if if nat_lt(a218, a219) {
        let a220 = 4u64;
        let a221 = v2;
        nat_le(a220, a221)
    } else {
        false
    } {
        let a222 = v0;
        let a223 = 0u64;
        if nat_eq(a222, a223) {
            let a224 = v3;
            let a225 = 0u64;
            if nat_eq(a224, a225) {
                Ok(Adt1::C2(0u64, 4u64, 0u64, 2u64))
            } else {
                let a226 = v3;
                let a227 = 1u64;
                if nat_eq(a226, a227) {
                    Ok(Adt1::C4({
                        let a228 = v1;
                        let a229 = v2;
                        nat_mul(a228, a229)?
                    }, v6))
                } else {
                    Ok(Adt1::C0(0u64))
                }
            }
        } else {
            let a230 = v0;
            let a231 = 26u64;
            if nat_eq(a230, a231) {
                let a232 = v3;
                let a233 = 0u64;
                if nat_eq(a232, a233) {
                    Ok(Adt1::C2(0u64, 4u64, 0u64, 2u64))
                } else {
                    let a234 = v3;
                    let a235 = 1u64;
                    if nat_eq(a234, a235) {
                        Ok(Adt1::C4({
                            let a236 = v1;
                            let a237 = v2;
                            nat_mul(a236, a237)?
                        }, v6))
                    } else {
                        Ok(Adt1::C0(0u64))
                    }
                }
            } else {
                let a238 = v0;
                let a239 = 1u64;
                if nat_eq(a238, a239) {
                    let a240 = v3;
                    let a241 = 0u64;
                    if nat_eq(a240, a241) {
                        Ok(Adt1::C2(0u64, 2u64, 0u64, 2u64))
                    } else {
                        let a242 = v3;
                        let a243 = 1u64;
                        if nat_eq(a242, a243) {
                            Ok(Adt1::C3(10u64, 1u64, v6, 2u64))
                        } else {
                            let a244 = v3;
                            let a245 = 2u64;
                            if nat_eq(a244, a245) {
                                Ok(Adt1::C4({
                                    let a246 = v1;
                                    let a247 = v2;
                                    nat_mul(a246, a247)?
                                }, v6))
                            } else {
                                Ok(Adt1::C0(0u64))
                            }
                        }
                    }
                } else {
                    let a248 = v0;
                    let a249 = 30u64;
                    if nat_eq(a248, a249) {
                        let a250 = v3;
                        let a251 = 0u64;
                        if nat_eq(a250, a251) {
                            Ok(Adt1::C2(0u64, 2u64, 0u64, 2u64))
                        } else {
                            let a252 = v3;
                            let a253 = 1u64;
                            if nat_eq(a252, a253) {
                                Ok(Adt1::C3(10u64, 30u64, v6, 2u64))
                            } else {
                                let a254 = v3;
                                let a255 = 2u64;
                                if nat_eq(a254, a255) {
                                    Ok(Adt1::C4({
                                        let a256 = v1;
                                        let a257 = v2;
                                        nat_mul(a256, a257)?
                                    }, v6))
                                } else {
                                    Ok(Adt1::C0(0u64))
                                }
                            }
                        }
                    } else {
                        let a258 = v0;
                        let a259 = 43u64;
                        if nat_eq(a258, a259) {
                            let a260 = v3;
                            let a261 = 0u64;
                            if nat_eq(a260, a261) {
                                Ok(Adt1::C2(0u64, 1u64, 0u64, 2u64))
                            } else {
                                let a262 = v3;
                                let a263 = 1u64;
                                if nat_eq(a262, a263) {
                                    Ok(Adt1::C3(9u64, 3u64, v6, 2u64))
                                } else {
                                    let a264 = v3;
                                    let a265 = 2u64;
                                    if nat_eq(a264, a265) {
                                        Ok(Adt1::C4({
                                            let a266 = v1;
                                            let a267 = v2;
                                            nat_mul(a266, a267)?
                                        }, v6))
                                    } else {
                                        Ok(Adt1::C0(0u64))
                                    }
                                }
                            }
                        } else {
                            let a268 = v0;
                            let a269 = 2u64;
                            if nat_eq(a268, a269) {
                                let a270 = v3;
                                let a271 = 0u64;
                                if nat_eq(a270, a271) {
                                    Ok(Adt1::C2(0u64, 1u64, {
                                        let a275 = 2u64;
                                        let a276 = {
                                            let a272 = v1;
                                            let a273 = 16u64;
                                            let a274 = 0u64;
                                            nat_rem(a272, a273, a274)
                                        };
                                        nat_add(a275, a276)?
                                    }, 3u64))
                                } else {
                                    let a277 = v3;
                                    let a278 = 1u64;
                                    if nat_eq(a277, a278) {
                                        Ok(Adt1::C3(8u64, {
                                            let a287 = {
                                                let a284 = v7;
                                                let a285 = {
                                                    let a282 = {
                                                        let a279 = v1;
                                                        let a280 = 16u64;
                                                        let a281 = 0u64;
                                                        nat_quot(a279, a280, a281)
                                                    };
                                                    let a283 = 0u64;
                                                    if nat_eq(a282, a283) {
                                                        1u64
                                                    } else {
                                                        16u64
                                                    }
                                                };
                                                let a286 = 0u64;
                                                nat_quot(a284, a285, a286)
                                            };
                                            let a288 = 16u64;
                                            let a289 = 0u64;
                                            nat_rem(a287, a288, a289)
                                        }, 8u64, 2u64))
                                    } else {
                                        let a290 = v3;
                                        let a291 = 2u64;
                                        if nat_eq(a290, a291) {
                                            f4(2u64, v6, v4, 2u64)
                                        } else {
                                            let a292 = v3;
                                            let a293 = 3u64;
                                            if nat_eq(a292, a293) {
                                                Ok(Adt1::C4({
                                                    let a294 = v1;
                                                    let a295 = v2;
                                                    nat_mul(a294, a295)?
                                                }, v6))
                                            } else {
                                                Ok(Adt1::C0(0u64))
                                            }
                                        }
                                    }
                                }
                            } else {
                                let a296 = v0;
                                let a297 = 3u64;
                                if nat_eq(a296, a297) {
                                    let a298 = v3;
                                    let a299 = 0u64;
                                    if nat_eq(a298, a299) {
                                        Ok(Adt1::C2(0u64, 1u64, {
                                            let a303 = 4u64;
                                            let a304 = {
                                                let a300 = v1;
                                                let a301 = 16u64;
                                                let a302 = 0u64;
                                                nat_rem(a300, a301, a302)
                                            };
                                            nat_add(a303, a304)?
                                        }, 3u64))
                                    } else {
                                        let a305 = v3;
                                        let a306 = 1u64;
                                        if nat_eq(a305, a306) {
                                            Ok(Adt1::C3(8u64, {
                                                let a315 = {
                                                    let a312 = v7;
                                                    let a313 = {
                                                        let a310 = {
                                                            let a307 = v1;
                                                            let a308 = 16u64;
                                                            let a309 = 0u64;
                                                            nat_quot(a307, a308, a309)
                                                        };
                                                        let a311 = 0u64;
                                                        if nat_eq(a310, a311) {
                                                            1u64
                                                        } else {
                                                            16u64
                                                        }
                                                    };
                                                    let a314 = 0u64;
                                                    nat_quot(a312, a313, a314)
                                                };
                                                let a316 = 16u64;
                                                let a317 = 0u64;
                                                nat_rem(a315, a316, a317)
                                            }, 0u64, 2u64))
                                        } else {
                                            let a318 = v3;
                                            let a319 = 2u64;
                                            if nat_eq(a318, a319) {
                                                f4(2u64, v6, v4, 2u64)
                                            } else {
                                                let a320 = v3;
                                                let a321 = 3u64;
                                                if nat_eq(a320, a321) {
                                                    f4(0u64, v6, v5, 2u64)
                                                } else {
                                                    let a322 = v3;
                                                    let a323 = 4u64;
                                                    if nat_eq(a322, a323) {
                                                        Ok(Adt1::C4({
                                                            let a324 = v1;
                                                            let a325 = v2;
                                                            nat_mul(a324, a325)?
                                                        }, v6))
                                                    } else {
                                                        Ok(Adt1::C0(0u64))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let a326 = v0;
                                    let a327 = 6u64;
                                    if nat_eq(a326, a327) {
                                        let a328 = v3;
                                        let a329 = 0u64;
                                        if nat_eq(a328, a329) {
                                            Ok(Adt1::C2(0u64, 1u64, {
                                                let a333 = 6u64;
                                                let a334 = {
                                                    let a330 = v1;
                                                    let a331 = 16u64;
                                                    let a332 = 0u64;
                                                    nat_rem(a330, a331, a332)
                                                };
                                                nat_add(a333, a334)?
                                            }, 3u64))
                                        } else {
                                            let a335 = v3;
                                            let a336 = 1u64;
                                            if nat_eq(a335, a336) {
                                                Ok(Adt1::C2(0u64, 1u64, {
                                                    let a340 = 2u64;
                                                    let a341 = {
                                                        let a337 = v1;
                                                        let a338 = 8u64;
                                                        let a339 = 0u64;
                                                        nat_quot(a337, a338, a339)
                                                    };
                                                    nat_add(a340, a341)?
                                                }, 4u64))
                                            } else {
                                                let a342 = v3;
                                                let a343 = 2u64;
                                                if nat_eq(a342, a343) {
                                                    Ok(Adt1::C3(8u64, {
                                                        let a366 = {
                                                            let a352 = {
                                                                let a349 = v7;
                                                                let a350 = {
                                                                    let a347 = {
                                                                        let a344 = v1;
                                                                        let a345 = 16u64;
                                                                        let a346 = 0u64;
                                                                        nat_quot(a344, a345, a346)
                                                                    };
                                                                    let a348 = 0u64;
                                                                    if nat_eq(a347, a348) {
                                                                        1u64
                                                                    } else {
                                                                        16u64
                                                                    }
                                                                };
                                                                let a351 = 0u64;
                                                                nat_quot(a349, a350, a351)
                                                            };
                                                            let a353 = 16u64;
                                                            let a354 = 0u64;
                                                            nat_rem(a352, a353, a354)
                                                        };
                                                        let a367 = {
                                                            let a364 = {
                                                                let a361 = {
                                                                    let a358 = v8;
                                                                    let a359 = f9({
                                                                        let a355 = v1;
                                                                        let a356 = 8u64;
                                                                        let a357 = 0u64;
                                                                        nat_rem(a355, a356, a357)
                                                                    });
                                                                    let a360 = 0u64;
                                                                    nat_quot(a358, a359, a360)
                                                                };
                                                                let a362 = 2u64;
                                                                let a363 = 0u64;
                                                                nat_rem(a361, a362, a363)
                                                            };
                                                            let a365 = 16u64;
                                                            nat_mul(a364, a365)?
                                                        };
                                                        nat_add(a366, a367)?
                                                    }, 16u64, 2u64))
                                                } else {
                                                    let a368 = v3;
                                                    let a369 = 3u64;
                                                    if nat_eq(a368, a369) {
                                                        f4(2u64, v6, v4, 2u64)
                                                    } else {
                                                        let a370 = v3;
                                                        let a371 = 4u64;
                                                        if nat_eq(a370, a371) {
                                                            Ok(Adt1::C4({
                                                                let a372 = v1;
                                                                let a373 = v2;
                                                                nat_mul(a372, a373)?
                                                            }, v6))
                                                        } else {
                                                            Ok(Adt1::C0(0u64))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let a374 = v0;
                                        let a375 = 7u64;
                                        if nat_eq(a374, a375) {
                                            let a376 = v3;
                                            let a377 = 0u64;
                                            if nat_eq(a376, a377) {
                                                Ok(Adt1::C2(0u64, 1u64, {
                                                    let a381 = 8u64;
                                                    let a382 = {
                                                        let a378 = v1;
                                                        let a379 = 16u64;
                                                        let a380 = 0u64;
                                                        nat_rem(a378, a379, a380)
                                                    };
                                                    nat_add(a381, a382)?
                                                }, 3u64))
                                            } else {
                                                let a383 = v3;
                                                let a384 = 1u64;
                                                if nat_eq(a383, a384) {
                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                        let a388 = 4u64;
                                                        let a389 = {
                                                            let a385 = v1;
                                                            let a386 = 8u64;
                                                            let a387 = 0u64;
                                                            nat_quot(a385, a386, a387)
                                                        };
                                                        nat_add(a388, a389)?
                                                    }, 4u64))
                                                } else {
                                                    let a390 = v3;
                                                    let a391 = 2u64;
                                                    if nat_eq(a390, a391) {
                                                        Ok(Adt1::C3(8u64, {
                                                            let a414 = {
                                                                let a400 = {
                                                                    let a397 = v7;
                                                                    let a398 = {
                                                                        let a395 = {
                                                                            let a392 = v1;
                                                                            let a393 = 16u64;
                                                                            let a394 = 0u64;
                                                                            nat_quot(a392, a393, a394)
                                                                        };
                                                                        let a396 = 0u64;
                                                                        if nat_eq(a395, a396) {
                                                                            1u64
                                                                        } else {
                                                                            16u64
                                                                        }
                                                                    };
                                                                    let a399 = 0u64;
                                                                    nat_quot(a397, a398, a399)
                                                                };
                                                                let a401 = 16u64;
                                                                let a402 = 0u64;
                                                                nat_rem(a400, a401, a402)
                                                            };
                                                            let a415 = {
                                                                let a412 = {
                                                                    let a409 = {
                                                                        let a406 = v8;
                                                                        let a407 = f9({
                                                                            let a403 = v1;
                                                                            let a404 = 8u64;
                                                                            let a405 = 0u64;
                                                                            nat_rem(a403, a404, a405)
                                                                        });
                                                                        let a408 = 0u64;
                                                                        nat_quot(a406, a407, a408)
                                                                    };
                                                                    let a410 = 2u64;
                                                                    let a411 = 0u64;
                                                                    nat_rem(a409, a410, a411)
                                                                };
                                                                let a413 = 16u64;
                                                                nat_mul(a412, a413)?
                                                            };
                                                            nat_add(a414, a415)?
                                                        }, 0u64, 2u64))
                                                    } else {
                                                        let a416 = v3;
                                                        let a417 = 3u64;
                                                        if nat_eq(a416, a417) {
                                                            f4(2u64, v6, v4, 2u64)
                                                        } else {
                                                            let a418 = v3;
                                                            let a419 = 4u64;
                                                            if nat_eq(a418, a419) {
                                                                f4(0u64, v6, v5, 2u64)
                                                            } else {
                                                                let a420 = v3;
                                                                let a421 = 5u64;
                                                                if nat_eq(a420, a421) {
                                                                    Ok(Adt1::C4({
                                                                        let a422 = v1;
                                                                        let a423 = v2;
                                                                        nat_mul(a422, a423)?
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
                                            let a424 = v0;
                                            let a425 = 8u64;
                                            if nat_eq(a424, a425) {
                                                let a426 = v3;
                                                let a427 = 0u64;
                                                if nat_eq(a426, a427) {
                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                        let a428 = 2u64;
                                                        let a429 = v1;
                                                        nat_add(a428, a429)?
                                                    }, 3u64))
                                                } else {
                                                    let a430 = v3;
                                                    let a431 = 1u64;
                                                    if nat_eq(a430, a431) {
                                                        Ok(Adt1::C3(8u64, v7, {
                                                            let a432 = v7;
                                                            let a433 = 128u64;
                                                            if nat_lt(a432, a433) {
                                                                0u64
                                                            } else {
                                                                256u64
                                                            }
                                                        }, 2u64))
                                                    } else {
                                                        let a434 = v3;
                                                        let a435 = 2u64;
                                                        if nat_eq(a434, a435) {
                                                            f4(2u64, v6, v4, 2u64)
                                                        } else {
                                                            let a436 = v3;
                                                            let a437 = 3u64;
                                                            if nat_eq(a436, a437) {
                                                                Ok(Adt1::C4({
                                                                    let a438 = v1;
                                                                    let a439 = v2;
                                                                    nat_mul(a438, a439)?
                                                                }, v6))
                                                            } else {
                                                                Ok(Adt1::C0(0u64))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                let a440 = v0;
                                                let a441 = 41u64;
                                                if nat_eq(a440, a441) {
                                                    let a442 = v3;
                                                    let a443 = 0u64;
                                                    if nat_eq(a442, a443) {
                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                            let a447 = 2u64;
                                                            let a448 = {
                                                                let a444 = v1;
                                                                let a445 = 8u64;
                                                                let a446 = 0u64;
                                                                nat_quot(a444, a445, a446)
                                                            };
                                                            nat_add(a447, a448)?
                                                        }, 3u64))
                                                    } else {
                                                        let a449 = v3;
                                                        let a450 = 1u64;
                                                        if nat_eq(a449, a450) {
                                                            Ok(Adt1::C3(6u64, v4, 0u64, 2u64))
                                                        } else {
                                                            let a451 = v3;
                                                            let a452 = 2u64;
                                                            if nat_eq(a451, a452) {
                                                                Ok(Adt1::C4({
                                                                    let a453 = v1;
                                                                    let a454 = v2;
                                                                    nat_mul(a453, a454)?
                                                                }, {
                                                                    let a464 = {
                                                                        let a461 = {
                                                                            let a458 = v7;
                                                                            let a459 = f9({
                                                                                let a455 = v1;
                                                                                let a456 = 8u64;
                                                                                let a457 = 0u64;
                                                                                nat_rem(a455, a456, a457)
                                                                            });
                                                                            let a460 = 0u64;
                                                                            nat_quot(a458, a459, a460)
                                                                        };
                                                                        let a462 = 2u64;
                                                                        let a463 = 0u64;
                                                                        nat_rem(a461, a462, a463)
                                                                    };
                                                                    let a465 = 1u64;
                                                                    if nat_eq(a464, a465) {
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
                                                    let a466 = v0;
                                                    let a467 = 42u64;
                                                    if nat_eq(a466, a467) {
                                                        let a468 = v3;
                                                        let a469 = 0u64;
                                                        if nat_eq(a468, a469) {
                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                let a473 = 2u64;
                                                                let a474 = {
                                                                    let a470 = v1;
                                                                    let a471 = 4u64;
                                                                    let a472 = 0u64;
                                                                    nat_quot(a470, a471, a472)
                                                                };
                                                                nat_add(a473, a474)?
                                                            }, 3u64))
                                                        } else {
                                                            let a475 = v3;
                                                            let a476 = 1u64;
                                                            if nat_eq(a475, a476) {
                                                                Ok(Adt1::C3(8u64, {
                                                                    let a485 = {
                                                                        let a482 = v7;
                                                                        let a483 = f9({
                                                                            let a480 = 2u64;
                                                                            let a481 = {
                                                                                let a477 = v1;
                                                                                let a478 = 4u64;
                                                                                let a479 = 0u64;
                                                                                nat_rem(a477, a478, a479)
                                                                            };
                                                                            nat_mul(a480, a481)?
                                                                        });
                                                                        let a484 = 0u64;
                                                                        nat_quot(a482, a483, a484)
                                                                    };
                                                                    let a486 = 4u64;
                                                                    let a487 = 0u64;
                                                                    nat_rem(a485, a486, a487)
                                                                }, 1u64, 2u64))
                                                            } else {
                                                                let a488 = v3;
                                                                let a489 = 2u64;
                                                                if nat_eq(a488, a489) {
                                                                    f4(2u64, v6, v4, 2u64)
                                                                } else {
                                                                    let a490 = v3;
                                                                    let a491 = 3u64;
                                                                    if nat_eq(a490, a491) {
                                                                        Ok(Adt1::C4({
                                                                            let a492 = v1;
                                                                            let a493 = v2;
                                                                            nat_mul(a492, a493)?
                                                                        }, v6))
                                                                    } else {
                                                                        Ok(Adt1::C0(0u64))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        let a494 = v0;
                                                        let a495 = 39u64;
                                                        if nat_eq(a494, a495) {
                                                            let a496 = v3;
                                                            let a497 = 0u64;
                                                            if nat_eq(a496, a497) {
                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                    let a501 = 1u64;
                                                                    let a502 = {
                                                                        let a498 = v1;
                                                                        let a499 = 16u64;
                                                                        let a500 = 0u64;
                                                                        nat_rem(a498, a499, a500)
                                                                    };
                                                                    nat_add(a501, a502)?
                                                                }, 3u64))
                                                            } else {
                                                                let a503 = v3;
                                                                let a504 = 1u64;
                                                                if nat_eq(a503, a504) {
                                                                    Ok(Adt1::C3(9u64, 6u64, {
                                                                        let a513 = {
                                                                            let a510 = v7;
                                                                            let a511 = {
                                                                                let a508 = {
                                                                                    let a505 = v1;
                                                                                    let a506 = 16u64;
                                                                                    let a507 = 0u64;
                                                                                    nat_quot(a505, a506, a507)
                                                                                };
                                                                                let a509 = 0u64;
                                                                                if nat_eq(a508, a509) {
                                                                                    1u64
                                                                                } else {
                                                                                    16u64
                                                                                }
                                                                            };
                                                                            let a512 = 0u64;
                                                                            nat_quot(a510, a511, a512)
                                                                        };
                                                                        let a514 = 16u64;
                                                                        let a515 = 0u64;
                                                                        nat_rem(a513, a514, a515)
                                                                    }, 2u64))
                                                                } else {
                                                                    let a516 = v3;
                                                                    let a517 = 2u64;
                                                                    if nat_eq(a516, a517) {
                                                                        f4(2u64, v6, v4, 2u64)
                                                                    } else {
                                                                        let a518 = v3;
                                                                        let a519 = 3u64;
                                                                        if nat_eq(a518, a519) {
                                                                            Ok(Adt1::C4({
                                                                                let a520 = v1;
                                                                                let a521 = v2;
                                                                                nat_mul(a520, a521)?
                                                                            }, v6))
                                                                        } else {
                                                                            Ok(Adt1::C0(0u64))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            let a522 = v0;
                                                            let a523 = 40u64;
                                                            if nat_eq(a522, a523) {
                                                                let a524 = v3;
                                                                let a525 = 0u64;
                                                                if nat_eq(a524, a525) {
                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                        let a526 = v1;
                                                                        let a527 = 16u64;
                                                                        let a528 = 0u64;
                                                                        nat_quot(a526, a527, a528)
                                                                    }, 3u64))
                                                                } else {
                                                                    let a529 = v3;
                                                                    let a530 = 1u64;
                                                                    if nat_eq(a529, a530) {
                                                                        Ok(Adt1::C3(9u64, 2u64, v7, 2u64))
                                                                    } else {
                                                                        let a531 = v3;
                                                                        let a532 = 2u64;
                                                                        if nat_eq(a531, a532) {
                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                let a543 = {
                                                                                    let a538 = 4u64;
                                                                                    let a539 = {
                                                                                        let a536 = {
                                                                                            let a533 = v1;
                                                                                            let a534 = 16u64;
                                                                                            let a535 = 0u64;
                                                                                            nat_quot(a533, a534, a535)
                                                                                        };
                                                                                        let a537 = 8u64;
                                                                                        nat_mul(a536, a537)?
                                                                                    };
                                                                                    nat_add(a538, a539)?
                                                                                };
                                                                                let a544 = {
                                                                                    let a540 = v1;
                                                                                    let a541 = 8u64;
                                                                                    let a542 = 0u64;
                                                                                    nat_rem(a540, a541, a542)
                                                                                };
                                                                                nat_add(a543, a544)?
                                                                            }, 3u64))
                                                                        } else {
                                                                            let a545 = v3;
                                                                            let a546 = 3u64;
                                                                            if nat_eq(a545, a546) {
                                                                                Ok(Adt1::C3(9u64, 6u64, {
                                                                                    let a558 = {
                                                                                        let a555 = v7;
                                                                                        let a556 = {
                                                                                            let a553 = {
                                                                                                let a550 = {
                                                                                                    let a547 = v1;
                                                                                                    let a548 = 16u64;
                                                                                                    let a549 = 0u64;
                                                                                                    nat_rem(a547, a548, a549)
                                                                                                };
                                                                                                let a551 = 8u64;
                                                                                                let a552 = 0u64;
                                                                                                nat_quot(a550, a551, a552)
                                                                                            };
                                                                                            let a554 = 0u64;
                                                                                            if nat_eq(a553, a554) {
                                                                                                1u64
                                                                                            } else {
                                                                                                16u64
                                                                                            }
                                                                                        };
                                                                                        let a557 = 0u64;
                                                                                        nat_quot(a555, a556, a557)
                                                                                    };
                                                                                    let a559 = 16u64;
                                                                                    let a560 = 0u64;
                                                                                    nat_rem(a558, a559, a560)
                                                                                }, 3u64))
                                                                            } else {
                                                                                let a561 = v3;
                                                                                let a562 = 4u64;
                                                                                if nat_eq(a561, a562) {
                                                                                    f4(2u64, v7, v6, 2u64)
                                                                                } else {
                                                                                    let a563 = v3;
                                                                                    let a564 = 5u64;
                                                                                    if nat_eq(a563, a564) {
                                                                                        Ok(Adt1::C4({
                                                                                            let a565 = v1;
                                                                                            let a566 = v2;
                                                                                            nat_mul(a565, a566)?
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
                                                                let a567 = v0;
                                                                let a568 = 10u64;
                                                                if nat_eq(a567, a568) {
                                                                    let a569 = v3;
                                                                    let a570 = 0u64;
                                                                    if nat_eq(a569, a570) {
                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                            let a571 = v1;
                                                                            let a572 = 16u64;
                                                                            let a573 = 0u64;
                                                                            nat_quot(a571, a572, a573)
                                                                        }, 3u64))
                                                                    } else {
                                                                        let a574 = v3;
                                                                        let a575 = 1u64;
                                                                        if nat_eq(a574, a575) {
                                                                            Ok(Adt1::C3(8u64, {
                                                                                let a576 = v7;
                                                                                let a577 = 16u64;
                                                                                let a578 = 0u64;
                                                                                nat_rem(a576, a577, a578)
                                                                            }, 0u64, 2u64))
                                                                        } else {
                                                                            let a579 = v3;
                                                                            let a580 = 2u64;
                                                                            if nat_eq(a579, a580) {
                                                                                f4(2u64, v4, v6, 2u64)
                                                                            } else {
                                                                                let a581 = v3;
                                                                                let a582 = 3u64;
                                                                                if nat_eq(a581, a582) {
                                                                                    Ok(Adt1::C3(8u64, {
                                                                                        let a583 = v7;
                                                                                        let a584 = 16u64;
                                                                                        let a585 = 0u64;
                                                                                        nat_quot(a583, a584, a585)
                                                                                    }, 0u64, 4u64))
                                                                                } else {
                                                                                    let a586 = v3;
                                                                                    let a587 = 4u64;
                                                                                    if nat_eq(a586, a587) {
                                                                                        f4(2u64, v5, v8, 4u64)
                                                                                    } else {
                                                                                        let a588 = v3;
                                                                                        let a589 = 5u64;
                                                                                        if nat_eq(a588, a589) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a600 = 16u64;
                                                                                                let a601 = {
                                                                                                    let a598 = {
                                                                                                        let a593 = 32u64;
                                                                                                        let a594 = {
                                                                                                            let a590 = v1;
                                                                                                            let a591 = 128u64;
                                                                                                            let a592 = 0u64;
                                                                                                            nat_quot(a590, a591, a592)
                                                                                                        };
                                                                                                        nat_mul(a593, a594)?
                                                                                                    };
                                                                                                    let a599 = {
                                                                                                        let a595 = v1;
                                                                                                        let a596 = 32u64;
                                                                                                        let a597 = 0u64;
                                                                                                        nat_rem(a595, a596, a597)
                                                                                                    };
                                                                                                    nat_add(a598, a599)?
                                                                                                };
                                                                                                nat_add(a600, a601)?
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a602 = v3;
                                                                                            let a603 = 6u64;
                                                                                            if nat_eq(a602, a603) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a615 = {
                                                                                                        let a612 = v7;
                                                                                                        let a613 = f9({
                                                                                                            let a610 = 2u64;
                                                                                                            let a611 = {
                                                                                                                let a607 = {
                                                                                                                    let a604 = v1;
                                                                                                                    let a605 = 128u64;
                                                                                                                    let a606 = 0u64;
                                                                                                                    nat_rem(a604, a605, a606)
                                                                                                                };
                                                                                                                let a608 = 32u64;
                                                                                                                let a609 = 0u64;
                                                                                                                nat_quot(a607, a608, a609)
                                                                                                            };
                                                                                                            nat_mul(a610, a611)?
                                                                                                        });
                                                                                                        let a614 = 0u64;
                                                                                                        nat_quot(a612, a613, a614)
                                                                                                    };
                                                                                                    let a616 = 4u64;
                                                                                                    let a617 = 0u64;
                                                                                                    nat_rem(a615, a616, a617)
                                                                                                }, 0u64, 3u64))
                                                                                            } else {
                                                                                                let a618 = v3;
                                                                                                let a619 = 7u64;
                                                                                                if nat_eq(a618, a619) {
                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                } else {
                                                                                                    let a620 = v3;
                                                                                                    let a621 = 8u64;
                                                                                                    if nat_eq(a620, a621) {
                                                                                                        f4(1u64, v6, v8, 2u64)
                                                                                                    } else {
                                                                                                        let a622 = v3;
                                                                                                        let a623 = 9u64;
                                                                                                        if nat_eq(a622, a623) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a624 = v1;
                                                                                                                let a625 = v2;
                                                                                                                nat_mul(a624, a625)?
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
                                                                    let a626 = v0;
                                                                    let a627 = 11u64;
                                                                    if nat_eq(a626, a627) {
                                                                        let a628 = v3;
                                                                        let a629 = 0u64;
                                                                        if nat_eq(a628, a629) {
                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                let a636 = 96u64;
                                                                                let a637 = {
                                                                                    let a633 = {
                                                                                        let a630 = v1;
                                                                                        let a631 = 16u64;
                                                                                        let a632 = 0u64;
                                                                                        nat_quot(a630, a631, a632)
                                                                                    };
                                                                                    let a634 = 8u64;
                                                                                    let a635 = 0u64;
                                                                                    nat_rem(a633, a634, a635)
                                                                                };
                                                                                nat_add(a636, a637)?
                                                                            }, 3u64))
                                                                        } else {
                                                                            let a638 = v3;
                                                                            let a639 = 1u64;
                                                                            if nat_eq(a638, a639) {
                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                    let a646 = 104u64;
                                                                                    let a647 = {
                                                                                        let a643 = {
                                                                                            let a640 = v1;
                                                                                            let a641 = 16u64;
                                                                                            let a642 = 0u64;
                                                                                            nat_quot(a640, a641, a642)
                                                                                        };
                                                                                        let a644 = 4u64;
                                                                                        let a645 = 0u64;
                                                                                        nat_rem(a643, a644, a645)
                                                                                    };
                                                                                    nat_add(a646, a647)?
                                                                                }, 4u64))
                                                                            } else {
                                                                                let a648 = v3;
                                                                                let a649 = 2u64;
                                                                                if nat_eq(a648, a649) {
                                                                                    Ok(Adt1::C3(8u64, {
                                                                                        let a680 = {
                                                                                            let a661 = {
                                                                                                let a658 = v7;
                                                                                                let a659 = {
                                                                                                    let a656 = {
                                                                                                        let a653 = {
                                                                                                            let a650 = v1;
                                                                                                            let a651 = 16u64;
                                                                                                            let a652 = 0u64;
                                                                                                            nat_quot(a650, a651, a652)
                                                                                                        };
                                                                                                        let a654 = 8u64;
                                                                                                        let a655 = 0u64;
                                                                                                        nat_quot(a653, a654, a655)
                                                                                                    };
                                                                                                    let a657 = 0u64;
                                                                                                    if nat_eq(a656, a657) {
                                                                                                        1u64
                                                                                                    } else {
                                                                                                        16u64
                                                                                                    }
                                                                                                };
                                                                                                let a660 = 0u64;
                                                                                                nat_quot(a658, a659, a660)
                                                                                            };
                                                                                            let a662 = 16u64;
                                                                                            let a663 = 0u64;
                                                                                            nat_rem(a661, a662, a663)
                                                                                        };
                                                                                        let a681 = {
                                                                                            let a678 = {
                                                                                                let a675 = {
                                                                                                    let a672 = v8;
                                                                                                    let a673 = f9({
                                                                                                        let a670 = 2u64;
                                                                                                        let a671 = {
                                                                                                            let a667 = {
                                                                                                                let a664 = v1;
                                                                                                                let a665 = 16u64;
                                                                                                                let a666 = 0u64;
                                                                                                                nat_quot(a664, a665, a666)
                                                                                                            };
                                                                                                            let a668 = 4u64;
                                                                                                            let a669 = 0u64;
                                                                                                            nat_quot(a667, a668, a669)
                                                                                                        };
                                                                                                        nat_mul(a670, a671)?
                                                                                                    });
                                                                                                    let a674 = 0u64;
                                                                                                    nat_quot(a672, a673, a674)
                                                                                                };
                                                                                                let a676 = 4u64;
                                                                                                let a677 = 0u64;
                                                                                                nat_rem(a675, a676, a677)
                                                                                            };
                                                                                            let a679 = 16u64;
                                                                                            nat_mul(a678, a679)?
                                                                                        };
                                                                                        nat_add(a680, a681)?
                                                                                    }, 32u64, 2u64))
                                                                                } else {
                                                                                    let a682 = v3;
                                                                                    let a683 = 3u64;
                                                                                    if nat_eq(a682, a683) {
                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                    } else {
                                                                                        let a684 = v3;
                                                                                        let a685 = 4u64;
                                                                                        if nat_eq(a684, a685) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a696 = 32u64;
                                                                                                let a697 = {
                                                                                                    let a694 = {
                                                                                                        let a689 = 32u64;
                                                                                                        let a690 = {
                                                                                                            let a686 = v1;
                                                                                                            let a687 = 128u64;
                                                                                                            let a688 = 0u64;
                                                                                                            nat_quot(a686, a687, a688)
                                                                                                        };
                                                                                                        nat_mul(a689, a690)?
                                                                                                    };
                                                                                                    let a695 = {
                                                                                                        let a691 = v1;
                                                                                                        let a692 = 32u64;
                                                                                                        let a693 = 0u64;
                                                                                                        nat_rem(a691, a692, a693)
                                                                                                    };
                                                                                                    nat_add(a694, a695)?
                                                                                                };
                                                                                                nat_add(a696, a697)?
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a698 = v3;
                                                                                            let a699 = 5u64;
                                                                                            if nat_eq(a698, a699) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                    let a700 = v1;
                                                                                                    let a701 = 32u64;
                                                                                                    let a702 = 0u64;
                                                                                                    nat_rem(a700, a701, a702)
                                                                                                }, 4u64))
                                                                                            } else {
                                                                                                let a703 = v3;
                                                                                                let a704 = 6u64;
                                                                                                if nat_eq(a703, a704) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a716 = {
                                                                                                            let a713 = v7;
                                                                                                            let a714 = f9({
                                                                                                                let a711 = 2u64;
                                                                                                                let a712 = {
                                                                                                                    let a708 = {
                                                                                                                        let a705 = v1;
                                                                                                                        let a706 = 128u64;
                                                                                                                        let a707 = 0u64;
                                                                                                                        nat_rem(a705, a706, a707)
                                                                                                                    };
                                                                                                                    let a709 = 32u64;
                                                                                                                    let a710 = 0u64;
                                                                                                                    nat_quot(a708, a709, a710)
                                                                                                                };
                                                                                                                nat_mul(a711, a712)?
                                                                                                            });
                                                                                                            let a715 = 0u64;
                                                                                                            nat_quot(a713, a714, a715)
                                                                                                        };
                                                                                                        let a717 = 4u64;
                                                                                                        let a718 = 0u64;
                                                                                                        nat_rem(a716, a717, a718)
                                                                                                    }, {
                                                                                                        let a728 = {
                                                                                                            let a725 = {
                                                                                                                let a722 = v8;
                                                                                                                let a723 = f9({
                                                                                                                    let a719 = v1;
                                                                                                                    let a720 = 32u64;
                                                                                                                    let a721 = 0u64;
                                                                                                                    nat_quot(a719, a720, a721)
                                                                                                                });
                                                                                                                let a724 = 0u64;
                                                                                                                nat_quot(a722, a723, a724)
                                                                                                            };
                                                                                                            let a726 = 2u64;
                                                                                                            let a727 = 0u64;
                                                                                                            nat_rem(a725, a726, a727)
                                                                                                        };
                                                                                                        let a729 = 1u64;
                                                                                                        if nat_eq(a728, a729) {
                                                                                                            0u64
                                                                                                        } else {
                                                                                                            4u64
                                                                                                        }
                                                                                                    }, 3u64))
                                                                                                } else {
                                                                                                    let a730 = v3;
                                                                                                    let a731 = 7u64;
                                                                                                    if nat_eq(a730, a731) {
                                                                                                        f4(2u64, v6, v7, 2u64)
                                                                                                    } else {
                                                                                                        let a732 = v3;
                                                                                                        let a733 = 8u64;
                                                                                                        if nat_eq(a732, a733) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a734 = v1;
                                                                                                                let a735 = v2;
                                                                                                                nat_mul(a734, a735)?
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
                                                                        let a736 = v0;
                                                                        let a737 = 12u64;
                                                                        if nat_eq(a736, a737) {
                                                                            let a738 = v3;
                                                                            let a739 = 0u64;
                                                                            if nat_eq(a738, a739) {
                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                    let a753 = 4u64;
                                                                                    let a754 = {
                                                                                        let a743 = {
                                                                                            let a740 = v1;
                                                                                            let a741 = 32u64;
                                                                                            let a742 = 0u64;
                                                                                            nat_quot(a740, a741, a742)
                                                                                        };
                                                                                        let a744 = 4u64;
                                                                                        if nat_lt(a743, a744) {
                                                                                            let a745 = v1;
                                                                                            let a746 = 32u64;
                                                                                            let a747 = 0u64;
                                                                                            nat_quot(a745, a746, a747)
                                                                                        } else {
                                                                                            let a751 = {
                                                                                                let a748 = v1;
                                                                                                let a749 = 32u64;
                                                                                                let a750 = 0u64;
                                                                                                nat_quot(a748, a749, a750)
                                                                                            };
                                                                                            let a752 = 4u64;
                                                                                            nat_sub(a751, a752)
                                                                                        }
                                                                                    };
                                                                                    nat_add(a753, a754)?
                                                                                }, 3u64))
                                                                            } else {
                                                                                let a755 = v3;
                                                                                let a756 = 1u64;
                                                                                if nat_eq(a755, a756) {
                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                        let a770 = 4u64;
                                                                                        let a771 = {
                                                                                            let a760 = {
                                                                                                let a757 = v1;
                                                                                                let a758 = 32u64;
                                                                                                let a759 = 0u64;
                                                                                                nat_quot(a757, a758, a759)
                                                                                            };
                                                                                            let a761 = 4u64;
                                                                                            if nat_lt(a760, a761) {
                                                                                                let a765 = {
                                                                                                    let a762 = v1;
                                                                                                    let a763 = 32u64;
                                                                                                    let a764 = 0u64;
                                                                                                    nat_quot(a762, a763, a764)
                                                                                                };
                                                                                                let a766 = 4u64;
                                                                                                nat_add(a765, a766)?
                                                                                            } else {
                                                                                                let a767 = v1;
                                                                                                let a768 = 32u64;
                                                                                                let a769 = 0u64;
                                                                                                nat_quot(a767, a768, a769)
                                                                                            }
                                                                                        };
                                                                                        nat_add(a770, a771)?
                                                                                    }, 4u64))
                                                                                } else {
                                                                                    let a772 = v3;
                                                                                    let a773 = 2u64;
                                                                                    if nat_eq(a772, a773) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a787 = 4u64;
                                                                                            let a788 = {
                                                                                                let a777 = {
                                                                                                    let a774 = v1;
                                                                                                    let a775 = 32u64;
                                                                                                    let a776 = 0u64;
                                                                                                    nat_quot(a774, a775, a776)
                                                                                                };
                                                                                                let a778 = 4u64;
                                                                                                if nat_lt(a777, a778) {
                                                                                                    let a779 = v1;
                                                                                                    let a780 = 32u64;
                                                                                                    let a781 = 0u64;
                                                                                                    nat_quot(a779, a780, a781)
                                                                                                } else {
                                                                                                    let a785 = {
                                                                                                        let a782 = v1;
                                                                                                        let a783 = 32u64;
                                                                                                        let a784 = 0u64;
                                                                                                        nat_quot(a782, a783, a784)
                                                                                                    };
                                                                                                    let a786 = 4u64;
                                                                                                    nat_add(a785, a786)?
                                                                                                }
                                                                                            };
                                                                                            nat_add(a787, a788)?
                                                                                        }, 5u64))
                                                                                    } else {
                                                                                        let a789 = v3;
                                                                                        let a790 = 3u64;
                                                                                        if nat_eq(a789, a790) {
                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                let a794 = {
                                                                                                    let a791 = v1;
                                                                                                    let a792 = 32u64;
                                                                                                    let a793 = 0u64;
                                                                                                    nat_quot(a791, a792, a793)
                                                                                                };
                                                                                                let a795 = 4u64;
                                                                                                if nat_lt(a794, a795) {
                                                                                                    let a796 = v7;
                                                                                                    let a797 = 64u64;
                                                                                                    let a798 = 0u64;
                                                                                                    nat_rem(a796, a797, a798)
                                                                                                } else {
                                                                                                    let a807 = {
                                                                                                        let a799 = v9;
                                                                                                        let a800 = 16u64;
                                                                                                        let a801 = 0u64;
                                                                                                        nat_rem(a799, a800, a801)
                                                                                                    };
                                                                                                    let a808 = {
                                                                                                        let a805 = {
                                                                                                            let a802 = v7;
                                                                                                            let a803 = 64u64;
                                                                                                            let a804 = 0u64;
                                                                                                            nat_quot(a802, a803, a804)
                                                                                                        };
                                                                                                        let a806 = 16u64;
                                                                                                        nat_mul(a805, a806)?
                                                                                                    };
                                                                                                    nat_add(a807, a808)?
                                                                                                }
                                                                                            }, 0u64, 2u64))
                                                                                        } else {
                                                                                            let a809 = v3;
                                                                                            let a810 = 4u64;
                                                                                            if nat_eq(a809, a810) {
                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                            } else {
                                                                                                let a811 = v3;
                                                                                                let a812 = 5u64;
                                                                                                if nat_eq(a811, a812) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a816 = {
                                                                                                            let a813 = v1;
                                                                                                            let a814 = 32u64;
                                                                                                            let a815 = 0u64;
                                                                                                            nat_quot(a813, a814, a815)
                                                                                                        };
                                                                                                        let a817 = 4u64;
                                                                                                        if nat_lt(a816, a817) {
                                                                                                            let a818 = v8;
                                                                                                            let a819 = 64u64;
                                                                                                            let a820 = 0u64;
                                                                                                            nat_rem(a818, a819, a820)
                                                                                                        } else {
                                                                                                            let a829 = {
                                                                                                                let a821 = v9;
                                                                                                                let a822 = 16u64;
                                                                                                                let a823 = 0u64;
                                                                                                                nat_quot(a821, a822, a823)
                                                                                                            };
                                                                                                            let a830 = {
                                                                                                                let a827 = {
                                                                                                                    let a824 = v8;
                                                                                                                    let a825 = 64u64;
                                                                                                                    let a826 = 0u64;
                                                                                                                    nat_quot(a824, a825, a826)
                                                                                                                };
                                                                                                                let a828 = 16u64;
                                                                                                                nat_mul(a827, a828)?
                                                                                                            };
                                                                                                            nat_add(a829, a830)?
                                                                                                        }
                                                                                                    }, 0u64, 6u64))
                                                                                                } else {
                                                                                                    let a831 = v3;
                                                                                                    let a832 = 6u64;
                                                                                                    if nat_eq(a831, a832) {
                                                                                                        f4(2u64, v5, v10, 6u64)
                                                                                                    } else {
                                                                                                        let a833 = v3;
                                                                                                        let a834 = 7u64;
                                                                                                        if nat_eq(a833, a834) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a848 = {
                                                                                                                    let a843 = 16u64;
                                                                                                                    let a844 = {
                                                                                                                        let a841 = {
                                                                                                                            let a838 = {
                                                                                                                                let a835 = v1;
                                                                                                                                let a836 = 32u64;
                                                                                                                                let a837 = 0u64;
                                                                                                                                nat_quot(a835, a836, a837)
                                                                                                                            };
                                                                                                                            let a839 = 2u64;
                                                                                                                            let a840 = 0u64;
                                                                                                                            nat_quot(a838, a839, a840)
                                                                                                                        };
                                                                                                                        let a842 = 32u64;
                                                                                                                        nat_mul(a841, a842)?
                                                                                                                    };
                                                                                                                    nat_add(a843, a844)?
                                                                                                                };
                                                                                                                let a849 = {
                                                                                                                    let a845 = v1;
                                                                                                                    let a846 = 32u64;
                                                                                                                    let a847 = 0u64;
                                                                                                                    nat_rem(a845, a846, a847)
                                                                                                                };
                                                                                                                nat_add(a848, a849)?
                                                                                                            }, 3u64))
                                                                                                        } else {
                                                                                                            let a850 = v3;
                                                                                                            let a851 = 8u64;
                                                                                                            if nat_eq(a850, a851) {
                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                    let a863 = {
                                                                                                                        let a860 = v7;
                                                                                                                        let a861 = {
                                                                                                                            let a858 = {
                                                                                                                                let a855 = {
                                                                                                                                    let a852 = v1;
                                                                                                                                    let a853 = 32u64;
                                                                                                                                    let a854 = 0u64;
                                                                                                                                    nat_quot(a852, a853, a854)
                                                                                                                                };
                                                                                                                                let a856 = 2u64;
                                                                                                                                let a857 = 0u64;
                                                                                                                                nat_rem(a855, a856, a857)
                                                                                                                            };
                                                                                                                            let a859 = 0u64;
                                                                                                                            if nat_eq(a858, a859) {
                                                                                                                                1u64
                                                                                                                            } else {
                                                                                                                                16u64
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a862 = 0u64;
                                                                                                                        nat_quot(a860, a861, a862)
                                                                                                                    };
                                                                                                                    let a864 = 16u64;
                                                                                                                    let a865 = 0u64;
                                                                                                                    nat_rem(a863, a864, a865)
                                                                                                                }, 0u64, 3u64))
                                                                                                            } else {
                                                                                                                let a866 = v3;
                                                                                                                let a867 = 9u64;
                                                                                                                if nat_eq(a866, a867) {
                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                } else {
                                                                                                                    let a868 = v3;
                                                                                                                    let a869 = 10u64;
                                                                                                                    if nat_eq(a868, a869) {
                                                                                                                        f4(1u64, v6, v10, 2u64)
                                                                                                                    } else {
                                                                                                                        let a870 = v3;
                                                                                                                        let a871 = 11u64;
                                                                                                                        if nat_eq(a870, a871) {
                                                                                                                            Ok(Adt1::C4({
                                                                                                                                let a872 = v1;
                                                                                                                                let a873 = v2;
                                                                                                                                nat_mul(a872, a873)?
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
                                                                            let a874 = v0;
                                                                            let a875 = 13u64;
                                                                            if nat_eq(a874, a875) {
                                                                                let a876 = v3;
                                                                                let a877 = 0u64;
                                                                                if nat_eq(a876, a877) {
                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                        let a891 = 4u64;
                                                                                        let a892 = {
                                                                                            let a881 = {
                                                                                                let a878 = v1;
                                                                                                let a879 = 32u64;
                                                                                                let a880 = 0u64;
                                                                                                nat_quot(a878, a879, a880)
                                                                                            };
                                                                                            let a882 = 4u64;
                                                                                            if nat_lt(a881, a882) {
                                                                                                let a883 = v1;
                                                                                                let a884 = 32u64;
                                                                                                let a885 = 0u64;
                                                                                                nat_quot(a883, a884, a885)
                                                                                            } else {
                                                                                                let a889 = {
                                                                                                    let a886 = v1;
                                                                                                    let a887 = 32u64;
                                                                                                    let a888 = 0u64;
                                                                                                    nat_quot(a886, a887, a888)
                                                                                                };
                                                                                                let a890 = 4u64;
                                                                                                nat_sub(a889, a890)
                                                                                            }
                                                                                        };
                                                                                        nat_add(a891, a892)?
                                                                                    }, 3u64))
                                                                                } else {
                                                                                    let a893 = v3;
                                                                                    let a894 = 1u64;
                                                                                    if nat_eq(a893, a894) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a908 = 4u64;
                                                                                            let a909 = {
                                                                                                let a898 = {
                                                                                                    let a895 = v1;
                                                                                                    let a896 = 32u64;
                                                                                                    let a897 = 0u64;
                                                                                                    nat_quot(a895, a896, a897)
                                                                                                };
                                                                                                let a899 = 4u64;
                                                                                                if nat_lt(a898, a899) {
                                                                                                    let a903 = {
                                                                                                        let a900 = v1;
                                                                                                        let a901 = 32u64;
                                                                                                        let a902 = 0u64;
                                                                                                        nat_quot(a900, a901, a902)
                                                                                                    };
                                                                                                    let a904 = 4u64;
                                                                                                    nat_add(a903, a904)?
                                                                                                } else {
                                                                                                    let a905 = v1;
                                                                                                    let a906 = 32u64;
                                                                                                    let a907 = 0u64;
                                                                                                    nat_quot(a905, a906, a907)
                                                                                                }
                                                                                            };
                                                                                            nat_add(a908, a909)?
                                                                                        }, 4u64))
                                                                                    } else {
                                                                                        let a910 = v3;
                                                                                        let a911 = 2u64;
                                                                                        if nat_eq(a910, a911) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a925 = 4u64;
                                                                                                let a926 = {
                                                                                                    let a915 = {
                                                                                                        let a912 = v1;
                                                                                                        let a913 = 32u64;
                                                                                                        let a914 = 0u64;
                                                                                                        nat_quot(a912, a913, a914)
                                                                                                    };
                                                                                                    let a916 = 4u64;
                                                                                                    if nat_lt(a915, a916) {
                                                                                                        let a917 = v1;
                                                                                                        let a918 = 32u64;
                                                                                                        let a919 = 0u64;
                                                                                                        nat_quot(a917, a918, a919)
                                                                                                    } else {
                                                                                                        let a923 = {
                                                                                                            let a920 = v1;
                                                                                                            let a921 = 32u64;
                                                                                                            let a922 = 0u64;
                                                                                                            nat_quot(a920, a921, a922)
                                                                                                        };
                                                                                                        let a924 = 4u64;
                                                                                                        nat_add(a923, a924)?
                                                                                                    }
                                                                                                };
                                                                                                nat_add(a925, a926)?
                                                                                            }, 5u64))
                                                                                        } else {
                                                                                            let a927 = v3;
                                                                                            let a928 = 3u64;
                                                                                            if nat_eq(a927, a928) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a932 = {
                                                                                                        let a929 = v1;
                                                                                                        let a930 = 32u64;
                                                                                                        let a931 = 0u64;
                                                                                                        nat_quot(a929, a930, a931)
                                                                                                    };
                                                                                                    let a933 = 4u64;
                                                                                                    if nat_lt(a932, a933) {
                                                                                                        let a934 = v7;
                                                                                                        let a935 = 64u64;
                                                                                                        let a936 = 0u64;
                                                                                                        nat_rem(a934, a935, a936)
                                                                                                    } else {
                                                                                                        let a945 = {
                                                                                                            let a937 = v9;
                                                                                                            let a938 = 16u64;
                                                                                                            let a939 = 0u64;
                                                                                                            nat_rem(a937, a938, a939)
                                                                                                        };
                                                                                                        let a946 = {
                                                                                                            let a943 = {
                                                                                                                let a940 = v7;
                                                                                                                let a941 = 64u64;
                                                                                                                let a942 = 0u64;
                                                                                                                nat_quot(a940, a941, a942)
                                                                                                            };
                                                                                                            let a944 = 16u64;
                                                                                                            nat_mul(a943, a944)?
                                                                                                        };
                                                                                                        nat_add(a945, a946)?
                                                                                                    }
                                                                                                }, 0u64, 2u64))
                                                                                            } else {
                                                                                                let a947 = v3;
                                                                                                let a948 = 4u64;
                                                                                                if nat_eq(a947, a948) {
                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                } else {
                                                                                                    let a949 = v3;
                                                                                                    let a950 = 5u64;
                                                                                                    if nat_eq(a949, a950) {
                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                            let a954 = {
                                                                                                                let a951 = v1;
                                                                                                                let a952 = 32u64;
                                                                                                                let a953 = 0u64;
                                                                                                                nat_quot(a951, a952, a953)
                                                                                                            };
                                                                                                            let a955 = 4u64;
                                                                                                            if nat_lt(a954, a955) {
                                                                                                                let a956 = v8;
                                                                                                                let a957 = 64u64;
                                                                                                                let a958 = 0u64;
                                                                                                                nat_rem(a956, a957, a958)
                                                                                                            } else {
                                                                                                                let a967 = {
                                                                                                                    let a959 = v9;
                                                                                                                    let a960 = 16u64;
                                                                                                                    let a961 = 0u64;
                                                                                                                    nat_quot(a959, a960, a961)
                                                                                                                };
                                                                                                                let a968 = {
                                                                                                                    let a965 = {
                                                                                                                        let a962 = v8;
                                                                                                                        let a963 = 64u64;
                                                                                                                        let a964 = 0u64;
                                                                                                                        nat_quot(a962, a963, a964)
                                                                                                                    };
                                                                                                                    let a966 = 16u64;
                                                                                                                    nat_mul(a965, a966)?
                                                                                                                };
                                                                                                                nat_add(a967, a968)?
                                                                                                            }
                                                                                                        }, 0u64, 6u64))
                                                                                                    } else {
                                                                                                        let a969 = v3;
                                                                                                        let a970 = 6u64;
                                                                                                        if nat_eq(a969, a970) {
                                                                                                            f4(2u64, v5, v10, 6u64)
                                                                                                        } else {
                                                                                                            let a971 = v3;
                                                                                                            let a972 = 7u64;
                                                                                                            if nat_eq(a971, a972) {
                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                    let a986 = {
                                                                                                                        let a981 = 48u64;
                                                                                                                        let a982 = {
                                                                                                                            let a979 = {
                                                                                                                                let a976 = {
                                                                                                                                    let a973 = v1;
                                                                                                                                    let a974 = 32u64;
                                                                                                                                    let a975 = 0u64;
                                                                                                                                    nat_quot(a973, a974, a975)
                                                                                                                                };
                                                                                                                                let a977 = 2u64;
                                                                                                                                let a978 = 0u64;
                                                                                                                                nat_quot(a976, a977, a978)
                                                                                                                            };
                                                                                                                            let a980 = 32u64;
                                                                                                                            nat_mul(a979, a980)?
                                                                                                                        };
                                                                                                                        nat_add(a981, a982)?
                                                                                                                    };
                                                                                                                    let a987 = {
                                                                                                                        let a983 = v1;
                                                                                                                        let a984 = 32u64;
                                                                                                                        let a985 = 0u64;
                                                                                                                        nat_rem(a983, a984, a985)
                                                                                                                    };
                                                                                                                    nat_add(a986, a987)?
                                                                                                                }, 3u64))
                                                                                                            } else {
                                                                                                                let a988 = v3;
                                                                                                                let a989 = 8u64;
                                                                                                                if nat_eq(a988, a989) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a993 = 16u64;
                                                                                                                        let a994 = {
                                                                                                                            let a990 = v1;
                                                                                                                            let a991 = 32u64;
                                                                                                                            let a992 = 0u64;
                                                                                                                            nat_rem(a990, a991, a992)
                                                                                                                        };
                                                                                                                        nat_add(a993, a994)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a995 = v3;
                                                                                                                    let a996 = 9u64;
                                                                                                                    if nat_eq(a995, a996) {
                                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                                            let a1022 = {
                                                                                                                                let a1008 = {
                                                                                                                                    let a1005 = v7;
                                                                                                                                    let a1006 = {
                                                                                                                                        let a1003 = {
                                                                                                                                            let a1000 = {
                                                                                                                                                let a997 = v1;
                                                                                                                                                let a998 = 32u64;
                                                                                                                                                let a999 = 0u64;
                                                                                                                                                nat_quot(a997, a998, a999)
                                                                                                                                            };
                                                                                                                                            let a1001 = 2u64;
                                                                                                                                            let a1002 = 0u64;
                                                                                                                                            nat_rem(a1000, a1001, a1002)
                                                                                                                                        };
                                                                                                                                        let a1004 = 0u64;
                                                                                                                                        if nat_eq(a1003, a1004) {
                                                                                                                                            1u64
                                                                                                                                        } else {
                                                                                                                                            16u64
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    let a1007 = 0u64;
                                                                                                                                    nat_quot(a1005, a1006, a1007)
                                                                                                                                };
                                                                                                                                let a1009 = 16u64;
                                                                                                                                let a1010 = 0u64;
                                                                                                                                nat_rem(a1008, a1009, a1010)
                                                                                                                            };
                                                                                                                            let a1023 = {
                                                                                                                                let a1020 = {
                                                                                                                                    let a1017 = {
                                                                                                                                        let a1014 = v8;
                                                                                                                                        let a1015 = f9({
                                                                                                                                            let a1011 = v1;
                                                                                                                                            let a1012 = 32u64;
                                                                                                                                            let a1013 = 0u64;
                                                                                                                                            nat_quot(a1011, a1012, a1013)
                                                                                                                                        });
                                                                                                                                        let a1016 = 0u64;
                                                                                                                                        nat_quot(a1014, a1015, a1016)
                                                                                                                                    };
                                                                                                                                    let a1018 = 2u64;
                                                                                                                                    let a1019 = 0u64;
                                                                                                                                    nat_rem(a1017, a1018, a1019)
                                                                                                                                };
                                                                                                                                let a1021 = 16u64;
                                                                                                                                nat_mul(a1020, a1021)?
                                                                                                                            };
                                                                                                                            nat_add(a1022, a1023)?
                                                                                                                        }, 0u64, 3u64))
                                                                                                                    } else {
                                                                                                                        let a1024 = v3;
                                                                                                                        let a1025 = 10u64;
                                                                                                                        if nat_eq(a1024, a1025) {
                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1026 = v3;
                                                                                                                            let a1027 = 11u64;
                                                                                                                            if nat_eq(a1026, a1027) {
                                                                                                                                f4(1u64, v6, v10, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1028 = v3;
                                                                                                                                let a1029 = 12u64;
                                                                                                                                if nat_eq(a1028, a1029) {
                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                        let a1030 = v1;
                                                                                                                                        let a1031 = v2;
                                                                                                                                        nat_mul(a1030, a1031)?
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
                                                                                let a1032 = v0;
                                                                                let a1033 = 14u64;
                                                                                if nat_eq(a1032, a1033) {
                                                                                    let a1034 = v3;
                                                                                    let a1035 = 0u64;
                                                                                    if nat_eq(a1034, a1035) {
                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                            let a1059 = {
                                                                                                let a1041 = 192u64;
                                                                                                let a1042 = {
                                                                                                    let a1039 = 8u64;
                                                                                                    let a1040 = {
                                                                                                        let a1036 = v1;
                                                                                                        let a1037 = 128u64;
                                                                                                        let a1038 = 0u64;
                                                                                                        nat_quot(a1036, a1037, a1038)
                                                                                                    };
                                                                                                    nat_mul(a1039, a1040)?
                                                                                                };
                                                                                                nat_add(a1041, a1042)?
                                                                                            };
                                                                                            let a1060 = {
                                                                                                let a1057 = {
                                                                                                    let a1049 = 2u64;
                                                                                                    let a1050 = {
                                                                                                        let a1046 = {
                                                                                                            let a1043 = v1;
                                                                                                            let a1044 = 128u64;
                                                                                                            let a1045 = 0u64;
                                                                                                            nat_rem(a1043, a1044, a1045)
                                                                                                        };
                                                                                                        let a1047 = 32u64;
                                                                                                        let a1048 = 0u64;
                                                                                                        nat_quot(a1046, a1047, a1048)
                                                                                                    };
                                                                                                    nat_mul(a1049, a1050)?
                                                                                                };
                                                                                                let a1058 = {
                                                                                                    let a1054 = {
                                                                                                        let a1051 = v1;
                                                                                                        let a1052 = 32u64;
                                                                                                        let a1053 = 0u64;
                                                                                                        nat_rem(a1051, a1052, a1053)
                                                                                                    };
                                                                                                    let a1055 = 16u64;
                                                                                                    let a1056 = 0u64;
                                                                                                    nat_quot(a1054, a1055, a1056)
                                                                                                };
                                                                                                nat_add(a1057, a1058)?
                                                                                            };
                                                                                            nat_add(a1059, a1060)?
                                                                                        }, 3u64))
                                                                                    } else {
                                                                                        let a1061 = v3;
                                                                                        let a1062 = 1u64;
                                                                                        if nat_eq(a1061, a1062) {
                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                let a1063 = v7;
                                                                                                let a1064 = 128u64;
                                                                                                if nat_lt(a1063, a1064) {
                                                                                                    0u64
                                                                                                } else {
                                                                                                    256u64
                                                                                                }
                                                                                            }, 2u64))
                                                                                        } else {
                                                                                            let a1065 = v3;
                                                                                            let a1066 = 2u64;
                                                                                            if nat_eq(a1065, a1066) {
                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                            } else {
                                                                                                let a1067 = v3;
                                                                                                let a1068 = 3u64;
                                                                                                if nat_eq(a1067, a1068) {
                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                        let a1090 = {
                                                                                                            let a1072 = 64u64;
                                                                                                            let a1073 = {
                                                                                                                let a1069 = v1;
                                                                                                                let a1070 = 128u64;
                                                                                                                let a1071 = 0u64;
                                                                                                                nat_quot(a1069, a1070, a1071)
                                                                                                            };
                                                                                                            nat_mul(a1072, a1073)?
                                                                                                        };
                                                                                                        let a1091 = {
                                                                                                            let a1088 = {
                                                                                                                let a1083 = 32u64;
                                                                                                                let a1084 = {
                                                                                                                    let a1080 = {
                                                                                                                        let a1077 = {
                                                                                                                            let a1074 = v1;
                                                                                                                            let a1075 = 128u64;
                                                                                                                            let a1076 = 0u64;
                                                                                                                            nat_rem(a1074, a1075, a1076)
                                                                                                                        };
                                                                                                                        let a1078 = 32u64;
                                                                                                                        let a1079 = 0u64;
                                                                                                                        nat_quot(a1077, a1078, a1079)
                                                                                                                    };
                                                                                                                    let a1081 = 2u64;
                                                                                                                    let a1082 = 0u64;
                                                                                                                    nat_rem(a1080, a1081, a1082)
                                                                                                                };
                                                                                                                nat_mul(a1083, a1084)?
                                                                                                            };
                                                                                                            let a1089 = {
                                                                                                                let a1085 = v1;
                                                                                                                let a1086 = 32u64;
                                                                                                                let a1087 = 0u64;
                                                                                                                nat_rem(a1085, a1086, a1087)
                                                                                                            };
                                                                                                            nat_add(a1088, a1089)?
                                                                                                        };
                                                                                                        nat_add(a1090, a1091)?
                                                                                                    }, 3u64))
                                                                                                } else {
                                                                                                    let a1092 = v3;
                                                                                                    let a1093 = 4u64;
                                                                                                    if nat_eq(a1092, a1093) {
                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                            let a1104 = 128u64;
                                                                                                            let a1105 = {
                                                                                                                let a1102 = {
                                                                                                                    let a1097 = 32u64;
                                                                                                                    let a1098 = {
                                                                                                                        let a1094 = v1;
                                                                                                                        let a1095 = 128u64;
                                                                                                                        let a1096 = 0u64;
                                                                                                                        nat_quot(a1094, a1095, a1096)
                                                                                                                    };
                                                                                                                    nat_mul(a1097, a1098)?
                                                                                                                };
                                                                                                                let a1103 = {
                                                                                                                    let a1099 = v1;
                                                                                                                    let a1100 = 32u64;
                                                                                                                    let a1101 = 0u64;
                                                                                                                    nat_rem(a1099, a1100, a1101)
                                                                                                                };
                                                                                                                nat_add(a1102, a1103)?
                                                                                                            };
                                                                                                            nat_add(a1104, a1105)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1106 = v3;
                                                                                                        let a1107 = 5u64;
                                                                                                        if nat_eq(a1106, a1107) {
                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                let a1141 = {
                                                                                                                    let a1122 = {
                                                                                                                        let a1119 = v7;
                                                                                                                        let a1120 = {
                                                                                                                            let a1117 = {
                                                                                                                                let a1114 = {
                                                                                                                                    let a1111 = {
                                                                                                                                        let a1108 = v1;
                                                                                                                                        let a1109 = 128u64;
                                                                                                                                        let a1110 = 0u64;
                                                                                                                                        nat_rem(a1108, a1109, a1110)
                                                                                                                                    };
                                                                                                                                    let a1112 = 32u64;
                                                                                                                                    let a1113 = 0u64;
                                                                                                                                    nat_quot(a1111, a1112, a1113)
                                                                                                                                };
                                                                                                                                let a1115 = 2u64;
                                                                                                                                let a1116 = 0u64;
                                                                                                                                nat_quot(a1114, a1115, a1116)
                                                                                                                            };
                                                                                                                            let a1118 = 0u64;
                                                                                                                            if nat_eq(a1117, a1118) {
                                                                                                                                1u64
                                                                                                                            } else {
                                                                                                                                16u64
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a1121 = 0u64;
                                                                                                                        nat_quot(a1119, a1120, a1121)
                                                                                                                    };
                                                                                                                    let a1123 = 16u64;
                                                                                                                    let a1124 = 0u64;
                                                                                                                    nat_rem(a1122, a1123, a1124)
                                                                                                                };
                                                                                                                let a1142 = {
                                                                                                                    let a1139 = {
                                                                                                                        let a1136 = {
                                                                                                                            let a1133 = v8;
                                                                                                                            let a1134 = f9({
                                                                                                                                let a1131 = 2u64;
                                                                                                                                let a1132 = {
                                                                                                                                    let a1128 = {
                                                                                                                                        let a1125 = v1;
                                                                                                                                        let a1126 = 128u64;
                                                                                                                                        let a1127 = 0u64;
                                                                                                                                        nat_rem(a1125, a1126, a1127)
                                                                                                                                    };
                                                                                                                                    let a1129 = 32u64;
                                                                                                                                    let a1130 = 0u64;
                                                                                                                                    nat_quot(a1128, a1129, a1130)
                                                                                                                                };
                                                                                                                                nat_mul(a1131, a1132)?
                                                                                                                            });
                                                                                                                            let a1135 = 0u64;
                                                                                                                            nat_quot(a1133, a1134, a1135)
                                                                                                                        };
                                                                                                                        let a1137 = 4u64;
                                                                                                                        let a1138 = 0u64;
                                                                                                                        nat_rem(a1136, a1137, a1138)
                                                                                                                    };
                                                                                                                    let a1140 = 16u64;
                                                                                                                    nat_mul(a1139, a1140)?
                                                                                                                };
                                                                                                                nat_add(a1141, a1142)?
                                                                                                            }, 32u64, 3u64))
                                                                                                        } else {
                                                                                                            let a1143 = v3;
                                                                                                            let a1144 = 6u64;
                                                                                                            if nat_eq(a1143, a1144) {
                                                                                                                f4(2u64, v6, v7, 2u64)
                                                                                                            } else {
                                                                                                                let a1145 = v3;
                                                                                                                let a1146 = 7u64;
                                                                                                                if nat_eq(a1145, a1146) {
                                                                                                                    Ok(Adt1::C4({
                                                                                                                        let a1147 = v1;
                                                                                                                        let a1148 = v2;
                                                                                                                        nat_mul(a1147, a1148)?
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
                                                                                    let a1149 = v0;
                                                                                    let a1150 = 34u64;
                                                                                    if nat_eq(a1149, a1150) {
                                                                                        let a1151 = v3;
                                                                                        let a1152 = 0u64;
                                                                                        if nat_eq(a1151, a1152) {
                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                let a1153 = v1;
                                                                                                let a1154 = 160u64;
                                                                                                if nat_lt(a1153, a1154) {
                                                                                                    let a1155 = v1;
                                                                                                    let a1156 = 32u64;
                                                                                                    let a1157 = 0u64;
                                                                                                    nat_rem(a1155, a1156, a1157)
                                                                                                } else {
                                                                                                    let a1158 = v1;
                                                                                                    let a1159 = 240u64;
                                                                                                    if nat_lt(a1158, a1159) {
                                                                                                        let a1165 = 32u64;
                                                                                                        let a1166 = {
                                                                                                            let a1162 = {
                                                                                                                let a1160 = v1;
                                                                                                                let a1161 = 160u64;
                                                                                                                nat_sub(a1160, a1161)
                                                                                                            };
                                                                                                            let a1163 = 16u64;
                                                                                                            let a1164 = 0u64;
                                                                                                            nat_rem(a1162, a1163, a1164)
                                                                                                        };
                                                                                                        nat_add(a1165, a1166)?
                                                                                                    } else {
                                                                                                        let a1172 = 48u64;
                                                                                                        let a1173 = {
                                                                                                            let a1169 = {
                                                                                                                let a1167 = v1;
                                                                                                                let a1168 = 240u64;
                                                                                                                nat_sub(a1167, a1168)
                                                                                                            };
                                                                                                            let a1170 = 4u64;
                                                                                                            let a1171 = 0u64;
                                                                                                            nat_rem(a1169, a1170, a1171)
                                                                                                        };
                                                                                                        nat_add(a1172, a1173)?
                                                                                                    }
                                                                                                }
                                                                                            }, 3u64))
                                                                                        } else {
                                                                                            let a1174 = v3;
                                                                                            let a1175 = 1u64;
                                                                                            if nat_eq(a1174, a1175) {
                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                    let a1259 = {
                                                                                                        let a1257 = {
                                                                                                            let a1254 = {
                                                                                                                let a1252 = v7;
                                                                                                                let a1253 = {
                                                                                                                    let a1193 = {
                                                                                                                        let a1176 = v1;
                                                                                                                        let a1177 = 160u64;
                                                                                                                        if nat_lt(a1176, a1177) {
                                                                                                                            let a1178 = v1;
                                                                                                                            let a1179 = 32u64;
                                                                                                                            let a1180 = 0u64;
                                                                                                                            nat_quot(a1178, a1179, a1180)
                                                                                                                        } else {
                                                                                                                            let a1181 = v1;
                                                                                                                            let a1182 = 240u64;
                                                                                                                            if nat_lt(a1181, a1182) {
                                                                                                                                let a1185 = {
                                                                                                                                    let a1183 = v1;
                                                                                                                                    let a1184 = 160u64;
                                                                                                                                    nat_sub(a1183, a1184)
                                                                                                                                };
                                                                                                                                let a1186 = 16u64;
                                                                                                                                let a1187 = 0u64;
                                                                                                                                nat_quot(a1185, a1186, a1187)
                                                                                                                            } else {
                                                                                                                                let a1190 = {
                                                                                                                                    let a1188 = v1;
                                                                                                                                    let a1189 = 240u64;
                                                                                                                                    nat_sub(a1188, a1189)
                                                                                                                                };
                                                                                                                                let a1191 = 4u64;
                                                                                                                                let a1192 = 0u64;
                                                                                                                                nat_quot(a1190, a1191, a1192)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    };
                                                                                                                    let a1194 = 0u64;
                                                                                                                    if nat_eq(a1193, a1194) {
                                                                                                                        1u64
                                                                                                                    } else {
                                                                                                                        let a1212 = {
                                                                                                                            let a1195 = v1;
                                                                                                                            let a1196 = 160u64;
                                                                                                                            if nat_lt(a1195, a1196) {
                                                                                                                                let a1197 = v1;
                                                                                                                                let a1198 = 32u64;
                                                                                                                                let a1199 = 0u64;
                                                                                                                                nat_quot(a1197, a1198, a1199)
                                                                                                                            } else {
                                                                                                                                let a1200 = v1;
                                                                                                                                let a1201 = 240u64;
                                                                                                                                if nat_lt(a1200, a1201) {
                                                                                                                                    let a1204 = {
                                                                                                                                        let a1202 = v1;
                                                                                                                                        let a1203 = 160u64;
                                                                                                                                        nat_sub(a1202, a1203)
                                                                                                                                    };
                                                                                                                                    let a1205 = 16u64;
                                                                                                                                    let a1206 = 0u64;
                                                                                                                                    nat_quot(a1204, a1205, a1206)
                                                                                                                                } else {
                                                                                                                                    let a1209 = {
                                                                                                                                        let a1207 = v1;
                                                                                                                                        let a1208 = 240u64;
                                                                                                                                        nat_sub(a1207, a1208)
                                                                                                                                    };
                                                                                                                                    let a1210 = 4u64;
                                                                                                                                    let a1211 = 0u64;
                                                                                                                                    nat_quot(a1209, a1210, a1211)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        };
                                                                                                                        let a1213 = 1u64;
                                                                                                                        if nat_eq(a1212, a1213) {
                                                                                                                            3u64
                                                                                                                        } else {
                                                                                                                            let a1231 = {
                                                                                                                                let a1214 = v1;
                                                                                                                                let a1215 = 160u64;
                                                                                                                                if nat_lt(a1214, a1215) {
                                                                                                                                    let a1216 = v1;
                                                                                                                                    let a1217 = 32u64;
                                                                                                                                    let a1218 = 0u64;
                                                                                                                                    nat_quot(a1216, a1217, a1218)
                                                                                                                                } else {
                                                                                                                                    let a1219 = v1;
                                                                                                                                    let a1220 = 240u64;
                                                                                                                                    if nat_lt(a1219, a1220) {
                                                                                                                                        let a1223 = {
                                                                                                                                            let a1221 = v1;
                                                                                                                                            let a1222 = 160u64;
                                                                                                                                            nat_sub(a1221, a1222)
                                                                                                                                        };
                                                                                                                                        let a1224 = 16u64;
                                                                                                                                        let a1225 = 0u64;
                                                                                                                                        nat_quot(a1223, a1224, a1225)
                                                                                                                                    } else {
                                                                                                                                        let a1228 = {
                                                                                                                                            let a1226 = v1;
                                                                                                                                            let a1227 = 240u64;
                                                                                                                                            nat_sub(a1226, a1227)
                                                                                                                                        };
                                                                                                                                        let a1229 = 4u64;
                                                                                                                                        let a1230 = 0u64;
                                                                                                                                        nat_quot(a1228, a1229, a1230)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            let a1232 = 2u64;
                                                                                                                            if nat_eq(a1231, a1232) {
                                                                                                                                9u64
                                                                                                                            } else {
                                                                                                                                let a1250 = {
                                                                                                                                    let a1233 = v1;
                                                                                                                                    let a1234 = 160u64;
                                                                                                                                    if nat_lt(a1233, a1234) {
                                                                                                                                        let a1235 = v1;
                                                                                                                                        let a1236 = 32u64;
                                                                                                                                        let a1237 = 0u64;
                                                                                                                                        nat_quot(a1235, a1236, a1237)
                                                                                                                                    } else {
                                                                                                                                        let a1238 = v1;
                                                                                                                                        let a1239 = 240u64;
                                                                                                                                        if nat_lt(a1238, a1239) {
                                                                                                                                            let a1242 = {
                                                                                                                                                let a1240 = v1;
                                                                                                                                                let a1241 = 160u64;
                                                                                                                                                nat_sub(a1240, a1241)
                                                                                                                                            };
                                                                                                                                            let a1243 = 16u64;
                                                                                                                                            let a1244 = 0u64;
                                                                                                                                            nat_quot(a1242, a1243, a1244)
                                                                                                                                        } else {
                                                                                                                                            let a1247 = {
                                                                                                                                                let a1245 = v1;
                                                                                                                                                let a1246 = 240u64;
                                                                                                                                                nat_sub(a1245, a1246)
                                                                                                                                            };
                                                                                                                                            let a1248 = 4u64;
                                                                                                                                            let a1249 = 0u64;
                                                                                                                                            nat_quot(a1247, a1248, a1249)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                let a1251 = 3u64;
                                                                                                                                if nat_eq(a1250, a1251) {
                                                                                                                                    27u64
                                                                                                                                } else {
                                                                                                                                    81u64
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                };
                                                                                                                nat_mul(a1252, a1253)?
                                                                                                            };
                                                                                                            let a1255 = 256u64;
                                                                                                            let a1256 = 0u64;
                                                                                                            nat_rem(a1254, a1255, a1256)
                                                                                                        };
                                                                                                        let a1258 = 3u64;
                                                                                                        nat_mul(a1257, a1258)?
                                                                                                    };
                                                                                                    let a1260 = 256u64;
                                                                                                    let a1261 = 0u64;
                                                                                                    nat_quot(a1259, a1260, a1261)
                                                                                                }, 1u64, 2u64))
                                                                                            } else {
                                                                                                let a1262 = v3;
                                                                                                let a1263 = 2u64;
                                                                                                if nat_eq(a1262, a1263) {
                                                                                                    f4(2u64, v6, v4, 2u64)
                                                                                                } else {
                                                                                                    let a1264 = v3;
                                                                                                    let a1265 = 3u64;
                                                                                                    if nat_eq(a1264, a1265) {
                                                                                                        Ok(Adt1::C4({
                                                                                                            let a1266 = v1;
                                                                                                            let a1267 = v2;
                                                                                                            nat_mul(a1266, a1267)?
                                                                                                        }, v6))
                                                                                                    } else {
                                                                                                        Ok(Adt1::C0(0u64))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let a1268 = v0;
                                                                                        let a1269 = 35u64;
                                                                                        if nat_eq(a1268, a1269) {
                                                                                            let a1270 = v3;
                                                                                            let a1271 = 0u64;
                                                                                            if nat_eq(a1270, a1271) {
                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                    let a1280 = {
                                                                                                        let a1275 = 32u64;
                                                                                                        let a1276 = {
                                                                                                            let a1272 = v1;
                                                                                                            let a1273 = 128u64;
                                                                                                            let a1274 = 0u64;
                                                                                                            nat_quot(a1272, a1273, a1274)
                                                                                                        };
                                                                                                        nat_mul(a1275, a1276)?
                                                                                                    };
                                                                                                    let a1281 = {
                                                                                                        let a1277 = v1;
                                                                                                        let a1278 = 32u64;
                                                                                                        let a1279 = 0u64;
                                                                                                        nat_rem(a1277, a1278, a1279)
                                                                                                    };
                                                                                                    nat_add(a1280, a1281)?
                                                                                                }, 3u64))
                                                                                            } else {
                                                                                                let a1282 = v3;
                                                                                                let a1283 = 1u64;
                                                                                                if nat_eq(a1282, a1283) {
                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                        let a1295 = {
                                                                                                            let a1292 = v7;
                                                                                                            let a1293 = f9({
                                                                                                                let a1290 = 2u64;
                                                                                                                let a1291 = {
                                                                                                                    let a1287 = {
                                                                                                                        let a1284 = v1;
                                                                                                                        let a1285 = 128u64;
                                                                                                                        let a1286 = 0u64;
                                                                                                                        nat_rem(a1284, a1285, a1286)
                                                                                                                    };
                                                                                                                    let a1288 = 32u64;
                                                                                                                    let a1289 = 0u64;
                                                                                                                    nat_quot(a1287, a1288, a1289)
                                                                                                                };
                                                                                                                nat_mul(a1290, a1291)?
                                                                                                            });
                                                                                                            let a1294 = 0u64;
                                                                                                            nat_quot(a1292, a1293, a1294)
                                                                                                        };
                                                                                                        let a1296 = 4u64;
                                                                                                        let a1297 = 0u64;
                                                                                                        nat_rem(a1295, a1296, a1297)
                                                                                                    }, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1298 = v3;
                                                                                                    let a1299 = 2u64;
                                                                                                    if nat_eq(a1298, a1299) {
                                                                                                        f4(2u64, v6, v4, 2u64)
                                                                                                    } else {
                                                                                                        let a1300 = v3;
                                                                                                        let a1301 = 3u64;
                                                                                                        if nat_eq(a1300, a1301) {
                                                                                                            Ok(Adt1::C4({
                                                                                                                let a1302 = v1;
                                                                                                                let a1303 = v2;
                                                                                                                nat_mul(a1302, a1303)?
                                                                                                            }, v6))
                                                                                                        } else {
                                                                                                            Ok(Adt1::C0(0u64))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            let a1304 = v0;
                                                                                            let a1305 = 16u64;
                                                                                            if nat_eq(a1304, a1305) {
                                                                                                let a1306 = v3;
                                                                                                let a1307 = 0u64;
                                                                                                if nat_eq(a1306, a1307) {
                                                                                                    Ok(Adt1::C2(0u64, 4u64, {
                                                                                                        let a1313 = 6u64;
                                                                                                        let a1314 = {
                                                                                                            let a1311 = {
                                                                                                                let a1308 = v1;
                                                                                                                let a1309 = 32u64;
                                                                                                                let a1310 = 0u64;
                                                                                                                nat_quot(a1308, a1309, a1310)
                                                                                                            };
                                                                                                            let a1312 = 8u64;
                                                                                                            nat_mul(a1311, a1312)?
                                                                                                        };
                                                                                                        nat_add(a1313, a1314)?
                                                                                                    }, 5u64))
                                                                                                } else {
                                                                                                    let a1315 = v3;
                                                                                                    let a1316 = 1u64;
                                                                                                    if nat_eq(a1315, a1316) {
                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                            let a1330 = 2u64;
                                                                                                            let a1331 = {
                                                                                                                let a1328 = {
                                                                                                                    let a1320 = {
                                                                                                                        let a1317 = v1;
                                                                                                                        let a1318 = 32u64;
                                                                                                                        let a1319 = 0u64;
                                                                                                                        nat_quot(a1317, a1318, a1319)
                                                                                                                    };
                                                                                                                    let a1321 = 8u64;
                                                                                                                    nat_mul(a1320, a1321)?
                                                                                                                };
                                                                                                                let a1329 = {
                                                                                                                    let a1325 = {
                                                                                                                        let a1322 = v1;
                                                                                                                        let a1323 = 32u64;
                                                                                                                        let a1324 = 0u64;
                                                                                                                        nat_rem(a1322, a1323, a1324)
                                                                                                                    };
                                                                                                                    let a1326 = 8u64;
                                                                                                                    let a1327 = 0u64;
                                                                                                                    nat_quot(a1325, a1326, a1327)
                                                                                                                };
                                                                                                                nat_add(a1328, a1329)?
                                                                                                            };
                                                                                                            nat_add(a1330, a1331)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1332 = v3;
                                                                                                        let a1333 = 2u64;
                                                                                                        if nat_eq(a1332, a1333) {
                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                let a1348 = 8u64;
                                                                                                                let a1349 = {
                                                                                                                    let a1345 = {
                                                                                                                        let a1342 = v9;
                                                                                                                        let a1343 = f9({
                                                                                                                            let a1340 = {
                                                                                                                                let a1337 = {
                                                                                                                                    let a1334 = v1;
                                                                                                                                    let a1335 = 32u64;
                                                                                                                                    let a1336 = 0u64;
                                                                                                                                    nat_rem(a1334, a1335, a1336)
                                                                                                                                };
                                                                                                                                let a1338 = 8u64;
                                                                                                                                let a1339 = 0u64;
                                                                                                                                nat_quot(a1337, a1338, a1339)
                                                                                                                            };
                                                                                                                            let a1341 = 7u64;
                                                                                                                            nat_mul(a1340, a1341)?
                                                                                                                        });
                                                                                                                        let a1344 = 0u64;
                                                                                                                        nat_quot(a1342, a1343, a1344)
                                                                                                                    };
                                                                                                                    let a1346 = 128u64;
                                                                                                                    let a1347 = 0u64;
                                                                                                                    nat_rem(a1345, a1346, a1347)
                                                                                                                };
                                                                                                                nat_add(a1348, a1349)?
                                                                                                            }, 7u64))
                                                                                                        } else {
                                                                                                            let a1350 = v3;
                                                                                                            let a1351 = 3u64;
                                                                                                            if nat_eq(a1350, a1351) {
                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                    let a1352 = v9;
                                                                                                                    let a1353 = 268435456u64;
                                                                                                                    let a1354 = 0u64;
                                                                                                                    nat_quot(a1352, a1353, a1354)
                                                                                                                }, 0u64, 2u64))
                                                                                                            } else {
                                                                                                                let a1355 = v3;
                                                                                                                let a1356 = 4u64;
                                                                                                                if nat_eq(a1355, a1356) {
                                                                                                                    f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                } else {
                                                                                                                    let a1357 = v3;
                                                                                                                    let a1358 = 5u64;
                                                                                                                    if nat_eq(a1357, a1358) {
                                                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                                                    } else {
                                                                                                                        let a1359 = v3;
                                                                                                                        let a1360 = 6u64;
                                                                                                                        if nat_eq(a1359, a1360) {
                                                                                                                            f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1361 = v3;
                                                                                                                            let a1362 = 7u64;
                                                                                                                            if nat_eq(a1361, a1362) {
                                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                    let a1370 = 1160u64;
                                                                                                                                    let a1371 = {
                                                                                                                                        let a1368 = {
                                                                                                                                            let a1363 = v8;
                                                                                                                                            let a1364 = 8u64;
                                                                                                                                            nat_mul(a1363, a1364)?
                                                                                                                                        };
                                                                                                                                        let a1369 = {
                                                                                                                                            let a1365 = v1;
                                                                                                                                            let a1366 = 8u64;
                                                                                                                                            let a1367 = 0u64;
                                                                                                                                            nat_rem(a1365, a1366, a1367)
                                                                                                                                        };
                                                                                                                                        nat_add(a1368, a1369)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1370, a1371)?
                                                                                                                                }, 3u64))
                                                                                                                            } else {
                                                                                                                                let a1372 = v3;
                                                                                                                                let a1373 = 8u64;
                                                                                                                                if nat_eq(a1372, a1373) {
                                                                                                                                    Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a1374 = v3;
                                                                                                                                    let a1375 = 9u64;
                                                                                                                                    if nat_eq(a1374, a1375) {
                                                                                                                                        f4(2u64, v6, v7, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1376 = v3;
                                                                                                                                        let a1377 = 10u64;
                                                                                                                                        if nat_eq(a1376, a1377) {
                                                                                                                                            Ok(f10(v6, {
                                                                                                                                                let a1387 = {
                                                                                                                                                    let a1384 = {
                                                                                                                                                        let a1381 = v11;
                                                                                                                                                        let a1382 = f9({
                                                                                                                                                            let a1378 = v1;
                                                                                                                                                            let a1379 = 8u64;
                                                                                                                                                            let a1380 = 0u64;
                                                                                                                                                            nat_rem(a1378, a1379, a1380)
                                                                                                                                                        });
                                                                                                                                                        let a1383 = 0u64;
                                                                                                                                                        nat_quot(a1381, a1382, a1383)
                                                                                                                                                    };
                                                                                                                                                    let a1385 = 2u64;
                                                                                                                                                    let a1386 = 0u64;
                                                                                                                                                    nat_rem(a1384, a1385, a1386)
                                                                                                                                                };
                                                                                                                                                let a1388 = 1u64;
                                                                                                                                                nat_eq(a1387, a1388)
                                                                                                                                            }, 2u64))
                                                                                                                                        } else {
                                                                                                                                            let a1389 = v3;
                                                                                                                                            let a1390 = 11u64;
                                                                                                                                            if nat_eq(a1389, a1390) {
                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                    let a1391 = v1;
                                                                                                                                                    let a1392 = v2;
                                                                                                                                                    nat_mul(a1391, a1392)?
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
                                                                                                let a1393 = v0;
                                                                                                let a1394 = 17u64;
                                                                                                if nat_eq(a1393, a1394) {
                                                                                                    let a1395 = v3;
                                                                                                    let a1396 = 0u64;
                                                                                                    if nat_eq(a1395, a1396) {
                                                                                                        Ok(Adt1::C2(0u64, 2u64, {
                                                                                                            let a1412 = 2u64;
                                                                                                            let a1413 = {
                                                                                                                let a1410 = {
                                                                                                                    let a1400 = {
                                                                                                                        let a1397 = v1;
                                                                                                                        let a1398 = 32u64;
                                                                                                                        let a1399 = 0u64;
                                                                                                                        nat_quot(a1397, a1398, a1399)
                                                                                                                    };
                                                                                                                    let a1401 = 8u64;
                                                                                                                    nat_mul(a1400, a1401)?
                                                                                                                };
                                                                                                                let a1411 = {
                                                                                                                    let a1408 = {
                                                                                                                        let a1405 = {
                                                                                                                            let a1402 = v1;
                                                                                                                            let a1403 = 32u64;
                                                                                                                            let a1404 = 0u64;
                                                                                                                            nat_rem(a1402, a1403, a1404)
                                                                                                                        };
                                                                                                                        let a1406 = 8u64;
                                                                                                                        let a1407 = 0u64;
                                                                                                                        nat_quot(a1405, a1406, a1407)
                                                                                                                    };
                                                                                                                    let a1409 = 2u64;
                                                                                                                    nat_mul(a1408, a1409)?
                                                                                                                };
                                                                                                                nat_add(a1410, a1411)?
                                                                                                            };
                                                                                                            nat_add(a1412, a1413)?
                                                                                                        }, 4u64))
                                                                                                    } else {
                                                                                                        let a1414 = v3;
                                                                                                        let a1415 = 1u64;
                                                                                                        if nat_eq(a1414, a1415) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a1419 = 66u64;
                                                                                                                let a1420 = {
                                                                                                                    let a1416 = v1;
                                                                                                                    let a1417 = 32u64;
                                                                                                                    let a1418 = 0u64;
                                                                                                                    nat_quot(a1416, a1417, a1418)
                                                                                                                };
                                                                                                                nat_add(a1419, a1420)?
                                                                                                            }, 5u64))
                                                                                                        } else {
                                                                                                            let a1421 = v3;
                                                                                                            let a1422 = 2u64;
                                                                                                            if nat_eq(a1421, a1422) {
                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                    let a1426 = 8u64;
                                                                                                                    let a1427 = {
                                                                                                                        let a1423 = v8;
                                                                                                                        let a1424 = 512u64;
                                                                                                                        let a1425 = 0u64;
                                                                                                                        nat_quot(a1423, a1424, a1425)
                                                                                                                    };
                                                                                                                    nat_add(a1426, a1427)?
                                                                                                                }, 7u64))
                                                                                                            } else {
                                                                                                                let a1428 = v3;
                                                                                                                let a1429 = 3u64;
                                                                                                                if nat_eq(a1428, a1429) {
                                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                                        let a1444 = {
                                                                                                                            let a1441 = v9;
                                                                                                                            let a1442 = {
                                                                                                                                let a1439 = {
                                                                                                                                    let a1436 = {
                                                                                                                                        let a1433 = {
                                                                                                                                            let a1430 = v1;
                                                                                                                                            let a1431 = 32u64;
                                                                                                                                            let a1432 = 0u64;
                                                                                                                                            nat_rem(a1430, a1431, a1432)
                                                                                                                                        };
                                                                                                                                        let a1434 = 8u64;
                                                                                                                                        let a1435 = 0u64;
                                                                                                                                        nat_quot(a1433, a1434, a1435)
                                                                                                                                    };
                                                                                                                                    let a1437 = 2u64;
                                                                                                                                    let a1438 = 0u64;
                                                                                                                                    nat_quot(a1436, a1437, a1438)
                                                                                                                                };
                                                                                                                                let a1440 = 0u64;
                                                                                                                                if nat_eq(a1439, a1440) {
                                                                                                                                    1u64
                                                                                                                                } else {
                                                                                                                                    16u64
                                                                                                                                }
                                                                                                                            };
                                                                                                                            let a1443 = 0u64;
                                                                                                                            nat_quot(a1441, a1442, a1443)
                                                                                                                        };
                                                                                                                        let a1445 = 16u64;
                                                                                                                        let a1446 = 0u64;
                                                                                                                        nat_rem(a1444, a1445, a1446)
                                                                                                                    }, 0u64, 2u64))
                                                                                                                } else {
                                                                                                                    let a1447 = v3;
                                                                                                                    let a1448 = 4u64;
                                                                                                                    if nat_eq(a1447, a1448) {
                                                                                                                        f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                    } else {
                                                                                                                        let a1449 = v3;
                                                                                                                        let a1450 = 5u64;
                                                                                                                        if nat_eq(a1449, a1450) {
                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                        } else {
                                                                                                                            let a1451 = v3;
                                                                                                                            let a1452 = 6u64;
                                                                                                                            if nat_eq(a1451, a1452) {
                                                                                                                                f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1453 = v3;
                                                                                                                                let a1454 = 7u64;
                                                                                                                                if nat_eq(a1453, a1454) {
                                                                                                                                    Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                        let a1465 = 3208u64;
                                                                                                                                        let a1466 = {
                                                                                                                                            let a1463 = {
                                                                                                                                                let a1458 = {
                                                                                                                                                    let a1455 = v8;
                                                                                                                                                    let a1456 = 512u64;
                                                                                                                                                    let a1457 = 0u64;
                                                                                                                                                    nat_rem(a1455, a1456, a1457)
                                                                                                                                                };
                                                                                                                                                let a1459 = 8u64;
                                                                                                                                                nat_mul(a1458, a1459)?
                                                                                                                                            };
                                                                                                                                            let a1464 = {
                                                                                                                                                let a1460 = v1;
                                                                                                                                                let a1461 = 8u64;
                                                                                                                                                let a1462 = 0u64;
                                                                                                                                                nat_rem(a1460, a1461, a1462)
                                                                                                                                            };
                                                                                                                                            nat_add(a1463, a1464)?
                                                                                                                                        };
                                                                                                                                        nat_add(a1465, a1466)?
                                                                                                                                    }, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a1467 = v3;
                                                                                                                                    let a1468 = 8u64;
                                                                                                                                    if nat_eq(a1467, a1468) {
                                                                                                                                        Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                    } else {
                                                                                                                                        let a1469 = v3;
                                                                                                                                        let a1470 = 9u64;
                                                                                                                                        if nat_eq(a1469, a1470) {
                                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a1471 = v3;
                                                                                                                                            let a1472 = 10u64;
                                                                                                                                            if nat_eq(a1471, a1472) {
                                                                                                                                                Ok(f10(v6, {
                                                                                                                                                    let a1482 = {
                                                                                                                                                        let a1479 = {
                                                                                                                                                            let a1476 = v11;
                                                                                                                                                            let a1477 = f9({
                                                                                                                                                                let a1473 = v1;
                                                                                                                                                                let a1474 = 8u64;
                                                                                                                                                                let a1475 = 0u64;
                                                                                                                                                                nat_rem(a1473, a1474, a1475)
                                                                                                                                                            });
                                                                                                                                                            let a1478 = 0u64;
                                                                                                                                                            nat_quot(a1476, a1477, a1478)
                                                                                                                                                        };
                                                                                                                                                        let a1480 = 2u64;
                                                                                                                                                        let a1481 = 0u64;
                                                                                                                                                        nat_rem(a1479, a1480, a1481)
                                                                                                                                                    };
                                                                                                                                                    let a1483 = 1u64;
                                                                                                                                                    nat_eq(a1482, a1483)
                                                                                                                                                }, 2u64))
                                                                                                                                            } else {
                                                                                                                                                let a1484 = v3;
                                                                                                                                                let a1485 = 11u64;
                                                                                                                                                if nat_eq(a1484, a1485) {
                                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                                        let a1486 = v1;
                                                                                                                                                        let a1487 = v2;
                                                                                                                                                        nat_mul(a1486, a1487)?
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
                                                                                                    let a1488 = v0;
                                                                                                    let a1489 = 22u64;
                                                                                                    if nat_eq(a1488, a1489) {
                                                                                                        let a1490 = v3;
                                                                                                        let a1491 = 0u64;
                                                                                                        if nat_eq(a1490, a1491) {
                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                let a1505 = 2u64;
                                                                                                                let a1506 = {
                                                                                                                    let a1503 = {
                                                                                                                        let a1495 = {
                                                                                                                            let a1492 = v1;
                                                                                                                            let a1493 = 32u64;
                                                                                                                            let a1494 = 0u64;
                                                                                                                            nat_quot(a1492, a1493, a1494)
                                                                                                                        };
                                                                                                                        let a1496 = 4u64;
                                                                                                                        nat_mul(a1495, a1496)?
                                                                                                                    };
                                                                                                                    let a1504 = {
                                                                                                                        let a1500 = {
                                                                                                                            let a1497 = v1;
                                                                                                                            let a1498 = 32u64;
                                                                                                                            let a1499 = 0u64;
                                                                                                                            nat_rem(a1497, a1498, a1499)
                                                                                                                        };
                                                                                                                        let a1501 = 8u64;
                                                                                                                        let a1502 = 0u64;
                                                                                                                        nat_quot(a1500, a1501, a1502)
                                                                                                                    };
                                                                                                                    nat_add(a1503, a1504)?
                                                                                                                };
                                                                                                                nat_add(a1505, a1506)?
                                                                                                            }, 4u64))
                                                                                                        } else {
                                                                                                            let a1507 = v3;
                                                                                                            let a1508 = 1u64;
                                                                                                            if nat_eq(a1507, a1508) {
                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                    let a1512 = 66u64;
                                                                                                                    let a1513 = {
                                                                                                                        let a1509 = v1;
                                                                                                                        let a1510 = 32u64;
                                                                                                                        let a1511 = 0u64;
                                                                                                                        nat_quot(a1509, a1510, a1511)
                                                                                                                    };
                                                                                                                    nat_add(a1512, a1513)?
                                                                                                                }, 6u64))
                                                                                                            } else {
                                                                                                                let a1514 = v3;
                                                                                                                let a1515 = 2u64;
                                                                                                                if nat_eq(a1514, a1515) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1519 = 74u64;
                                                                                                                        let a1520 = {
                                                                                                                            let a1516 = v1;
                                                                                                                            let a1517 = 32u64;
                                                                                                                            let a1518 = 0u64;
                                                                                                                            nat_quot(a1516, a1517, a1518)
                                                                                                                        };
                                                                                                                        nat_add(a1519, a1520)?
                                                                                                                    }, 5u64))
                                                                                                                } else {
                                                                                                                    let a1521 = v3;
                                                                                                                    let a1522 = 3u64;
                                                                                                                    if nat_eq(a1521, a1522) {
                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                            let a1536 = 34u64;
                                                                                                                            let a1537 = {
                                                                                                                                let a1534 = {
                                                                                                                                    let a1526 = {
                                                                                                                                        let a1523 = v1;
                                                                                                                                        let a1524 = 32u64;
                                                                                                                                        let a1525 = 0u64;
                                                                                                                                        nat_quot(a1523, a1524, a1525)
                                                                                                                                    };
                                                                                                                                    let a1527 = 4u64;
                                                                                                                                    nat_mul(a1526, a1527)?
                                                                                                                                };
                                                                                                                                let a1535 = {
                                                                                                                                    let a1531 = {
                                                                                                                                        let a1528 = v1;
                                                                                                                                        let a1529 = 32u64;
                                                                                                                                        let a1530 = 0u64;
                                                                                                                                        nat_rem(a1528, a1529, a1530)
                                                                                                                                    };
                                                                                                                                    let a1532 = 8u64;
                                                                                                                                    let a1533 = 0u64;
                                                                                                                                    nat_quot(a1531, a1532, a1533)
                                                                                                                                };
                                                                                                                                nat_add(a1534, a1535)?
                                                                                                                            };
                                                                                                                            nat_add(a1536, a1537)?
                                                                                                                        }, 7u64))
                                                                                                                    } else {
                                                                                                                        let a1538 = v3;
                                                                                                                        let a1539 = 4u64;
                                                                                                                        if nat_eq(a1538, a1539) {
                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                let a1554 = {
                                                                                                                                    let a1551 = v9;
                                                                                                                                    let a1552 = {
                                                                                                                                        let a1549 = {
                                                                                                                                            let a1546 = {
                                                                                                                                                let a1543 = {
                                                                                                                                                    let a1540 = v1;
                                                                                                                                                    let a1541 = 32u64;
                                                                                                                                                    let a1542 = 0u64;
                                                                                                                                                    nat_rem(a1540, a1541, a1542)
                                                                                                                                                };
                                                                                                                                                let a1544 = 8u64;
                                                                                                                                                let a1545 = 0u64;
                                                                                                                                                nat_quot(a1543, a1544, a1545)
                                                                                                                                            };
                                                                                                                                            let a1547 = 2u64;
                                                                                                                                            let a1548 = 0u64;
                                                                                                                                            nat_quot(a1546, a1547, a1548)
                                                                                                                                        };
                                                                                                                                        let a1550 = 0u64;
                                                                                                                                        if nat_eq(a1549, a1550) {
                                                                                                                                            1u64
                                                                                                                                        } else {
                                                                                                                                            16u64
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    let a1553 = 0u64;
                                                                                                                                    nat_quot(a1551, a1552, a1553)
                                                                                                                                };
                                                                                                                                let a1555 = 16u64;
                                                                                                                                let a1556 = 0u64;
                                                                                                                                nat_rem(a1554, a1555, a1556)
                                                                                                                            }, 0u64, 2u64))
                                                                                                                        } else {
                                                                                                                            let a1557 = v3;
                                                                                                                            let a1558 = 5u64;
                                                                                                                            if nat_eq(a1557, a1558) {
                                                                                                                                f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1559 = v3;
                                                                                                                                let a1560 = 6u64;
                                                                                                                                if nat_eq(a1559, a1560) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1561 = v3;
                                                                                                                                    let a1562 = 7u64;
                                                                                                                                    if nat_eq(a1561, a1562) {
                                                                                                                                        f4(2u64, v6, 1048576000u64, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1563 = v3;
                                                                                                                                        let a1564 = 8u64;
                                                                                                                                        if nat_eq(a1563, a1564) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1590 = 7304u64;
                                                                                                                                                let a1591 = {
                                                                                                                                                    let a1588 = {
                                                                                                                                                        let a1583 = {
                                                                                                                                                            let a1581 = v8;
                                                                                                                                                            let a1582 = {
                                                                                                                                                                let a1579 = {
                                                                                                                                                                    let a1576 = {
                                                                                                                                                                        let a1573 = v10;
                                                                                                                                                                        let a1574 = f9({
                                                                                                                                                                            let a1571 = 2u64;
                                                                                                                                                                            let a1572 = {
                                                                                                                                                                                let a1568 = {
                                                                                                                                                                                    let a1565 = v1;
                                                                                                                                                                                    let a1566 = 32u64;
                                                                                                                                                                                    let a1567 = 0u64;
                                                                                                                                                                                    nat_rem(a1565, a1566, a1567)
                                                                                                                                                                                };
                                                                                                                                                                                let a1569 = 8u64;
                                                                                                                                                                                let a1570 = 0u64;
                                                                                                                                                                                nat_quot(a1568, a1569, a1570)
                                                                                                                                                                            };
                                                                                                                                                                            nat_mul(a1571, a1572)?
                                                                                                                                                                        });
                                                                                                                                                                        let a1575 = 0u64;
                                                                                                                                                                        nat_quot(a1573, a1574, a1575)
                                                                                                                                                                    };
                                                                                                                                                                    let a1577 = 4u64;
                                                                                                                                                                    let a1578 = 0u64;
                                                                                                                                                                    nat_rem(a1576, a1577, a1578)
                                                                                                                                                                };
                                                                                                                                                                let a1580 = 256u64;
                                                                                                                                                                nat_mul(a1579, a1580)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a1581, a1582)?
                                                                                                                                                        };
                                                                                                                                                        let a1584 = 8u64;
                                                                                                                                                        nat_mul(a1583, a1584)?
                                                                                                                                                    };
                                                                                                                                                    let a1589 = {
                                                                                                                                                        let a1585 = v1;
                                                                                                                                                        let a1586 = 8u64;
                                                                                                                                                        let a1587 = 0u64;
                                                                                                                                                        nat_rem(a1585, a1586, a1587)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1588, a1589)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1590, a1591)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1592 = v3;
                                                                                                                                            let a1593 = 9u64;
                                                                                                                                            if nat_eq(a1592, a1593) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1594 = v3;
                                                                                                                                                let a1595 = 10u64;
                                                                                                                                                if nat_eq(a1594, a1595) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1596 = v3;
                                                                                                                                                    let a1597 = 11u64;
                                                                                                                                                    if nat_eq(a1596, a1597) {
                                                                                                                                                        Ok(f10(v6, {
                                                                                                                                                            let a1607 = {
                                                                                                                                                                let a1604 = {
                                                                                                                                                                    let a1601 = v11;
                                                                                                                                                                    let a1602 = f9({
                                                                                                                                                                        let a1598 = v1;
                                                                                                                                                                        let a1599 = 8u64;
                                                                                                                                                                        let a1600 = 0u64;
                                                                                                                                                                        nat_rem(a1598, a1599, a1600)
                                                                                                                                                                    });
                                                                                                                                                                    let a1603 = 0u64;
                                                                                                                                                                    nat_quot(a1601, a1602, a1603)
                                                                                                                                                                };
                                                                                                                                                                let a1605 = 2u64;
                                                                                                                                                                let a1606 = 0u64;
                                                                                                                                                                nat_rem(a1604, a1605, a1606)
                                                                                                                                                            };
                                                                                                                                                            let a1608 = 1u64;
                                                                                                                                                            nat_eq(a1607, a1608)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1609 = v3;
                                                                                                                                                        let a1610 = 12u64;
                                                                                                                                                        if nat_eq(a1609, a1610) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1611 = v1;
                                                                                                                                                                let a1612 = v2;
                                                                                                                                                                nat_mul(a1611, a1612)?
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
                                                                                                        let a1613 = v0;
                                                                                                        let a1614 = 18u64;
                                                                                                        if nat_eq(a1613, a1614) {
                                                                                                            let a1615 = v3;
                                                                                                            let a1616 = 0u64;
                                                                                                            if nat_eq(a1615, a1616) {
                                                                                                                Ok(Adt1::C2(0u64, 4u64, {
                                                                                                                    let a1622 = 66u64;
                                                                                                                    let a1623 = {
                                                                                                                        let a1620 = {
                                                                                                                            let a1617 = v1;
                                                                                                                            let a1618 = 32u64;
                                                                                                                            let a1619 = 0u64;
                                                                                                                            nat_quot(a1617, a1618, a1619)
                                                                                                                        };
                                                                                                                        let a1621 = 4u64;
                                                                                                                        nat_mul(a1620, a1621)?
                                                                                                                    };
                                                                                                                    nat_add(a1622, a1623)?
                                                                                                                }, 5u64))
                                                                                                            } else {
                                                                                                                let a1624 = v3;
                                                                                                                let a1625 = 1u64;
                                                                                                                if nat_eq(a1624, a1625) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1649 = 2u64;
                                                                                                                        let a1650 = {
                                                                                                                            let a1647 = {
                                                                                                                                let a1629 = {
                                                                                                                                    let a1626 = v1;
                                                                                                                                    let a1627 = 32u64;
                                                                                                                                    let a1628 = 0u64;
                                                                                                                                    nat_quot(a1626, a1627, a1628)
                                                                                                                                };
                                                                                                                                let a1630 = 8u64;
                                                                                                                                nat_mul(a1629, a1630)?
                                                                                                                            };
                                                                                                                            let a1648 = {
                                                                                                                                let a1645 = {
                                                                                                                                    let a1637 = {
                                                                                                                                        let a1634 = {
                                                                                                                                            let a1631 = v1;
                                                                                                                                            let a1632 = 32u64;
                                                                                                                                            let a1633 = 0u64;
                                                                                                                                            nat_rem(a1631, a1632, a1633)
                                                                                                                                        };
                                                                                                                                        let a1635 = 8u64;
                                                                                                                                        let a1636 = 0u64;
                                                                                                                                        nat_quot(a1634, a1635, a1636)
                                                                                                                                    };
                                                                                                                                    let a1638 = 2u64;
                                                                                                                                    nat_mul(a1637, a1638)?
                                                                                                                                };
                                                                                                                                let a1646 = {
                                                                                                                                    let a1642 = {
                                                                                                                                        let a1639 = v1;
                                                                                                                                        let a1640 = 8u64;
                                                                                                                                        let a1641 = 0u64;
                                                                                                                                        nat_rem(a1639, a1640, a1641)
                                                                                                                                    };
                                                                                                                                    let a1643 = 4u64;
                                                                                                                                    let a1644 = 0u64;
                                                                                                                                    nat_quot(a1642, a1643, a1644)
                                                                                                                                };
                                                                                                                                nat_add(a1645, a1646)?
                                                                                                                            };
                                                                                                                            nat_add(a1647, a1648)?
                                                                                                                        };
                                                                                                                        nat_add(a1649, a1650)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a1651 = v3;
                                                                                                                    let a1652 = 2u64;
                                                                                                                    if nat_eq(a1651, a1652) {
                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                            let a1667 = 8u64;
                                                                                                                            let a1668 = {
                                                                                                                                let a1664 = {
                                                                                                                                    let a1661 = v9;
                                                                                                                                    let a1662 = f9({
                                                                                                                                        let a1659 = {
                                                                                                                                            let a1656 = {
                                                                                                                                                let a1653 = v1;
                                                                                                                                                let a1654 = 32u64;
                                                                                                                                                let a1655 = 0u64;
                                                                                                                                                nat_rem(a1653, a1654, a1655)
                                                                                                                                            };
                                                                                                                                            let a1657 = 8u64;
                                                                                                                                            let a1658 = 0u64;
                                                                                                                                            nat_quot(a1656, a1657, a1658)
                                                                                                                                        };
                                                                                                                                        let a1660 = 7u64;
                                                                                                                                        nat_mul(a1659, a1660)?
                                                                                                                                    });
                                                                                                                                    let a1663 = 0u64;
                                                                                                                                    nat_quot(a1661, a1662, a1663)
                                                                                                                                };
                                                                                                                                let a1665 = 128u64;
                                                                                                                                let a1666 = 0u64;
                                                                                                                                nat_rem(a1664, a1665, a1666)
                                                                                                                            };
                                                                                                                            nat_add(a1667, a1668)?
                                                                                                                        }, 7u64))
                                                                                                                    } else {
                                                                                                                        let a1669 = v3;
                                                                                                                        let a1670 = 3u64;
                                                                                                                        if nat_eq(a1669, a1670) {
                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                let a1671 = v9;
                                                                                                                                let a1672 = 268435456u64;
                                                                                                                                let a1673 = 0u64;
                                                                                                                                nat_quot(a1671, a1672, a1673)
                                                                                                                            }, 0u64, 2u64))
                                                                                                                        } else {
                                                                                                                            let a1674 = v3;
                                                                                                                            let a1675 = 4u64;
                                                                                                                            if nat_eq(a1674, a1675) {
                                                                                                                                f4(0u64, 1056964608u64, v6, 2u64)
                                                                                                                            } else {
                                                                                                                                let a1676 = v3;
                                                                                                                                let a1677 = 5u64;
                                                                                                                                if nat_eq(a1676, a1677) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1678 = v3;
                                                                                                                                    let a1679 = 6u64;
                                                                                                                                    if nat_eq(a1678, a1679) {
                                                                                                                                        f4(2u64, v6, 1056964608u64, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1680 = v3;
                                                                                                                                        let a1681 = 7u64;
                                                                                                                                        if nat_eq(a1680, a1681) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1692 = 15496u64;
                                                                                                                                                let a1693 = {
                                                                                                                                                    let a1690 = {
                                                                                                                                                        let a1682 = v8;
                                                                                                                                                        let a1683 = 4u64;
                                                                                                                                                        nat_mul(a1682, a1683)?
                                                                                                                                                    };
                                                                                                                                                    let a1691 = {
                                                                                                                                                        let a1687 = {
                                                                                                                                                            let a1684 = v1;
                                                                                                                                                            let a1685 = 8u64;
                                                                                                                                                            let a1686 = 0u64;
                                                                                                                                                            nat_rem(a1684, a1685, a1686)
                                                                                                                                                        };
                                                                                                                                                        let a1688 = 4u64;
                                                                                                                                                        let a1689 = 0u64;
                                                                                                                                                        nat_rem(a1687, a1688, a1689)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1690, a1691)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1692, a1693)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1694 = v3;
                                                                                                                                            let a1695 = 8u64;
                                                                                                                                            if nat_eq(a1694, a1695) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1696 = v3;
                                                                                                                                                let a1697 = 9u64;
                                                                                                                                                if nat_eq(a1696, a1697) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1698 = v3;
                                                                                                                                                    let a1699 = 10u64;
                                                                                                                                                    if nat_eq(a1698, a1699) {
                                                                                                                                                        Ok(f10(v6, {
                                                                                                                                                            let a1709 = {
                                                                                                                                                                let a1706 = {
                                                                                                                                                                    let a1703 = v11;
                                                                                                                                                                    let a1704 = f9({
                                                                                                                                                                        let a1700 = v1;
                                                                                                                                                                        let a1701 = 8u64;
                                                                                                                                                                        let a1702 = 0u64;
                                                                                                                                                                        nat_rem(a1700, a1701, a1702)
                                                                                                                                                                    });
                                                                                                                                                                    let a1705 = 0u64;
                                                                                                                                                                    nat_quot(a1703, a1704, a1705)
                                                                                                                                                                };
                                                                                                                                                                let a1707 = 2u64;
                                                                                                                                                                let a1708 = 0u64;
                                                                                                                                                                nat_rem(a1706, a1707, a1708)
                                                                                                                                                            };
                                                                                                                                                            let a1710 = 1u64;
                                                                                                                                                            nat_eq(a1709, a1710)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1711 = v3;
                                                                                                                                                        let a1712 = 11u64;
                                                                                                                                                        if nat_eq(a1711, a1712) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1713 = v1;
                                                                                                                                                                let a1714 = v2;
                                                                                                                                                                nat_mul(a1713, a1714)?
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
                                                                                                            let a1715 = v0;
                                                                                                            let a1716 = 21u64;
                                                                                                            if nat_eq(a1715, a1716) {
                                                                                                                let a1717 = v3;
                                                                                                                let a1718 = 0u64;
                                                                                                                if nat_eq(a1717, a1718) {
                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                        let a1742 = 2u64;
                                                                                                                        let a1743 = {
                                                                                                                            let a1740 = {
                                                                                                                                let a1722 = {
                                                                                                                                    let a1719 = v1;
                                                                                                                                    let a1720 = 32u64;
                                                                                                                                    let a1721 = 0u64;
                                                                                                                                    nat_quot(a1719, a1720, a1721)
                                                                                                                                };
                                                                                                                                let a1723 = 8u64;
                                                                                                                                nat_mul(a1722, a1723)?
                                                                                                                            };
                                                                                                                            let a1741 = {
                                                                                                                                let a1738 = {
                                                                                                                                    let a1730 = {
                                                                                                                                        let a1727 = {
                                                                                                                                            let a1724 = v1;
                                                                                                                                            let a1725 = 32u64;
                                                                                                                                            let a1726 = 0u64;
                                                                                                                                            nat_rem(a1724, a1725, a1726)
                                                                                                                                        };
                                                                                                                                        let a1728 = 8u64;
                                                                                                                                        let a1729 = 0u64;
                                                                                                                                        nat_quot(a1727, a1728, a1729)
                                                                                                                                    };
                                                                                                                                    let a1731 = 2u64;
                                                                                                                                    nat_mul(a1730, a1731)?
                                                                                                                                };
                                                                                                                                let a1739 = {
                                                                                                                                    let a1735 = {
                                                                                                                                        let a1732 = v1;
                                                                                                                                        let a1733 = 8u64;
                                                                                                                                        let a1734 = 0u64;
                                                                                                                                        nat_rem(a1732, a1733, a1734)
                                                                                                                                    };
                                                                                                                                    let a1736 = 4u64;
                                                                                                                                    let a1737 = 0u64;
                                                                                                                                    nat_quot(a1735, a1736, a1737)
                                                                                                                                };
                                                                                                                                nat_add(a1738, a1739)?
                                                                                                                            };
                                                                                                                            nat_add(a1740, a1741)?
                                                                                                                        };
                                                                                                                        nat_add(a1742, a1743)?
                                                                                                                    }, 4u64))
                                                                                                                } else {
                                                                                                                    let a1744 = v3;
                                                                                                                    let a1745 = 1u64;
                                                                                                                    if nat_eq(a1744, a1745) {
                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                            let a1749 = 66u64;
                                                                                                                            let a1750 = {
                                                                                                                                let a1746 = v1;
                                                                                                                                let a1747 = 32u64;
                                                                                                                                let a1748 = 0u64;
                                                                                                                                nat_quot(a1746, a1747, a1748)
                                                                                                                            };
                                                                                                                            nat_add(a1749, a1750)?
                                                                                                                        }, 6u64))
                                                                                                                    } else {
                                                                                                                        let a1751 = v3;
                                                                                                                        let a1752 = 2u64;
                                                                                                                        if nat_eq(a1751, a1752) {
                                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                let a1759 = 106u64;
                                                                                                                                let a1760 = {
                                                                                                                                    let a1756 = {
                                                                                                                                        let a1753 = v1;
                                                                                                                                        let a1754 = 32u64;
                                                                                                                                        let a1755 = 0u64;
                                                                                                                                        nat_quot(a1753, a1754, a1755)
                                                                                                                                    };
                                                                                                                                    let a1757 = 2u64;
                                                                                                                                    let a1758 = 0u64;
                                                                                                                                    nat_quot(a1756, a1757, a1758)
                                                                                                                                };
                                                                                                                                nat_add(a1759, a1760)?
                                                                                                                            }, 5u64))
                                                                                                                        } else {
                                                                                                                            let a1761 = v3;
                                                                                                                            let a1762 = 3u64;
                                                                                                                            if nat_eq(a1761, a1762) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a1776 = 74u64;
                                                                                                                                    let a1777 = {
                                                                                                                                        let a1774 = {
                                                                                                                                            let a1766 = {
                                                                                                                                                let a1763 = v1;
                                                                                                                                                let a1764 = 32u64;
                                                                                                                                                let a1765 = 0u64;
                                                                                                                                                nat_quot(a1763, a1764, a1765)
                                                                                                                                            };
                                                                                                                                            let a1767 = 4u64;
                                                                                                                                            nat_mul(a1766, a1767)?
                                                                                                                                        };
                                                                                                                                        let a1775 = {
                                                                                                                                            let a1771 = {
                                                                                                                                                let a1768 = v1;
                                                                                                                                                let a1769 = 32u64;
                                                                                                                                                let a1770 = 0u64;
                                                                                                                                                nat_rem(a1768, a1769, a1770)
                                                                                                                                            };
                                                                                                                                            let a1772 = 8u64;
                                                                                                                                            let a1773 = 0u64;
                                                                                                                                            nat_quot(a1771, a1772, a1773)
                                                                                                                                        };
                                                                                                                                        nat_add(a1774, a1775)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1776, a1777)?
                                                                                                                                }, 7u64))
                                                                                                                            } else {
                                                                                                                                let a1778 = v3;
                                                                                                                                let a1779 = 4u64;
                                                                                                                                if nat_eq(a1778, a1779) {
                                                                                                                                    Ok(Adt1::C3(8u64, {
                                                                                                                                        let a1796 = 1u64;
                                                                                                                                        let a1797 = {
                                                                                                                                            let a1794 = 2u64;
                                                                                                                                            let a1795 = {
                                                                                                                                                let a1791 = {
                                                                                                                                                    let a1788 = v9;
                                                                                                                                                    let a1789 = {
                                                                                                                                                        let a1786 = {
                                                                                                                                                            let a1783 = {
                                                                                                                                                                let a1780 = v1;
                                                                                                                                                                let a1781 = 32u64;
                                                                                                                                                                let a1782 = 0u64;
                                                                                                                                                                nat_quot(a1780, a1781, a1782)
                                                                                                                                                            };
                                                                                                                                                            let a1784 = 2u64;
                                                                                                                                                            let a1785 = 0u64;
                                                                                                                                                            nat_rem(a1783, a1784, a1785)
                                                                                                                                                        };
                                                                                                                                                        let a1787 = 0u64;
                                                                                                                                                        if nat_eq(a1786, a1787) {
                                                                                                                                                            1u64
                                                                                                                                                        } else {
                                                                                                                                                            16u64
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    let a1790 = 0u64;
                                                                                                                                                    nat_quot(a1788, a1789, a1790)
                                                                                                                                                };
                                                                                                                                                let a1792 = 16u64;
                                                                                                                                                let a1793 = 0u64;
                                                                                                                                                nat_rem(a1791, a1792, a1793)
                                                                                                                                            };
                                                                                                                                            nat_mul(a1794, a1795)?
                                                                                                                                        };
                                                                                                                                        nat_add(a1796, a1797)?
                                                                                                                                    }, 0u64, 2u64))
                                                                                                                                } else {
                                                                                                                                    let a1798 = v3;
                                                                                                                                    let a1799 = 5u64;
                                                                                                                                    if nat_eq(a1798, a1799) {
                                                                                                                                        f4(2u64, v4, v6, 2u64)
                                                                                                                                    } else {
                                                                                                                                        let a1800 = v3;
                                                                                                                                        let a1801 = 6u64;
                                                                                                                                        if nat_eq(a1800, a1801) {
                                                                                                                                            Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                let a1838 = 16520u64;
                                                                                                                                                let a1839 = {
                                                                                                                                                    let a1836 = {
                                                                                                                                                        let a1828 = {
                                                                                                                                                            let a1826 = v8;
                                                                                                                                                            let a1827 = {
                                                                                                                                                                let a1824 = {
                                                                                                                                                                    let a1821 = {
                                                                                                                                                                        let a1818 = v10;
                                                                                                                                                                        let a1819 = f9({
                                                                                                                                                                            let a1816 = {
                                                                                                                                                                                let a1808 = 2u64;
                                                                                                                                                                                let a1809 = {
                                                                                                                                                                                    let a1805 = {
                                                                                                                                                                                        let a1802 = v1;
                                                                                                                                                                                        let a1803 = 32u64;
                                                                                                                                                                                        let a1804 = 0u64;
                                                                                                                                                                                        nat_rem(a1802, a1803, a1804)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a1806 = 8u64;
                                                                                                                                                                                    let a1807 = 0u64;
                                                                                                                                                                                    nat_quot(a1805, a1806, a1807)
                                                                                                                                                                                };
                                                                                                                                                                                nat_mul(a1808, a1809)?
                                                                                                                                                                            };
                                                                                                                                                                            let a1817 = {
                                                                                                                                                                                let a1813 = {
                                                                                                                                                                                    let a1810 = v1;
                                                                                                                                                                                    let a1811 = 8u64;
                                                                                                                                                                                    let a1812 = 0u64;
                                                                                                                                                                                    nat_rem(a1810, a1811, a1812)
                                                                                                                                                                                };
                                                                                                                                                                                let a1814 = 4u64;
                                                                                                                                                                                let a1815 = 0u64;
                                                                                                                                                                                nat_quot(a1813, a1814, a1815)
                                                                                                                                                                            };
                                                                                                                                                                            nat_add(a1816, a1817)?
                                                                                                                                                                        });
                                                                                                                                                                        let a1820 = 0u64;
                                                                                                                                                                        nat_quot(a1818, a1819, a1820)
                                                                                                                                                                    };
                                                                                                                                                                    let a1822 = 2u64;
                                                                                                                                                                    let a1823 = 0u64;
                                                                                                                                                                    nat_rem(a1821, a1822, a1823)
                                                                                                                                                                };
                                                                                                                                                                let a1825 = 256u64;
                                                                                                                                                                nat_mul(a1824, a1825)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a1826, a1827)?
                                                                                                                                                        };
                                                                                                                                                        let a1829 = 4u64;
                                                                                                                                                        nat_mul(a1828, a1829)?
                                                                                                                                                    };
                                                                                                                                                    let a1837 = {
                                                                                                                                                        let a1833 = {
                                                                                                                                                            let a1830 = v1;
                                                                                                                                                            let a1831 = 8u64;
                                                                                                                                                            let a1832 = 0u64;
                                                                                                                                                            nat_rem(a1830, a1831, a1832)
                                                                                                                                                        };
                                                                                                                                                        let a1834 = 4u64;
                                                                                                                                                        let a1835 = 0u64;
                                                                                                                                                        nat_rem(a1833, a1834, a1835)
                                                                                                                                                    };
                                                                                                                                                    nat_add(a1836, a1837)?
                                                                                                                                                };
                                                                                                                                                nat_add(a1838, a1839)?
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1840 = v3;
                                                                                                                                            let a1841 = 7u64;
                                                                                                                                            if nat_eq(a1840, a1841) {
                                                                                                                                                Ok(Adt1::C3(8u64, v7, 0u64, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a1842 = v3;
                                                                                                                                                let a1843 = 8u64;
                                                                                                                                                if nat_eq(a1842, a1843) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1844 = v3;
                                                                                                                                                    let a1845 = 9u64;
                                                                                                                                                    if nat_eq(a1844, a1845) {
                                                                                                                                                        Ok(f10(v6, {
                                                                                                                                                            let a1855 = {
                                                                                                                                                                let a1852 = {
                                                                                                                                                                    let a1849 = v11;
                                                                                                                                                                    let a1850 = f9({
                                                                                                                                                                        let a1846 = v1;
                                                                                                                                                                        let a1847 = 8u64;
                                                                                                                                                                        let a1848 = 0u64;
                                                                                                                                                                        nat_rem(a1846, a1847, a1848)
                                                                                                                                                                    });
                                                                                                                                                                    let a1851 = 0u64;
                                                                                                                                                                    nat_quot(a1849, a1850, a1851)
                                                                                                                                                                };
                                                                                                                                                                let a1853 = 2u64;
                                                                                                                                                                let a1854 = 0u64;
                                                                                                                                                                nat_rem(a1852, a1853, a1854)
                                                                                                                                                            };
                                                                                                                                                            let a1856 = 1u64;
                                                                                                                                                            nat_eq(a1855, a1856)
                                                                                                                                                        }, 2u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a1857 = v3;
                                                                                                                                                        let a1858 = 10u64;
                                                                                                                                                        if nat_eq(a1857, a1858) {
                                                                                                                                                            Ok(Adt1::C4({
                                                                                                                                                                let a1859 = v1;
                                                                                                                                                                let a1860 = v2;
                                                                                                                                                                nat_mul(a1859, a1860)?
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
                                                                                                                let a1861 = v0;
                                                                                                                let a1862 = 19u64;
                                                                                                                if nat_eq(a1861, a1862) {
                                                                                                                    let a1863 = v3;
                                                                                                                    let a1864 = 0u64;
                                                                                                                    if nat_eq(a1863, a1864) {
                                                                                                                        Ok(Adt1::C2(0u64, 2u64, {
                                                                                                                            let a1870 = 34u64;
                                                                                                                            let a1871 = {
                                                                                                                                let a1868 = {
                                                                                                                                    let a1865 = v1;
                                                                                                                                    let a1866 = 32u64;
                                                                                                                                    let a1867 = 0u64;
                                                                                                                                    nat_quot(a1865, a1866, a1867)
                                                                                                                                };
                                                                                                                                let a1869 = 2u64;
                                                                                                                                nat_mul(a1868, a1869)?
                                                                                                                            };
                                                                                                                            nat_add(a1870, a1871)?
                                                                                                                        }, 5u64))
                                                                                                                    } else {
                                                                                                                        let a1872 = v3;
                                                                                                                        let a1873 = 1u64;
                                                                                                                        if nat_eq(a1872, a1873) {
                                                                                                                            Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                let a1887 = 2u64;
                                                                                                                                let a1888 = {
                                                                                                                                    let a1885 = {
                                                                                                                                        let a1877 = {
                                                                                                                                            let a1874 = v1;
                                                                                                                                            let a1875 = 32u64;
                                                                                                                                            let a1876 = 0u64;
                                                                                                                                            nat_quot(a1874, a1875, a1876)
                                                                                                                                        };
                                                                                                                                        let a1878 = 4u64;
                                                                                                                                        nat_mul(a1877, a1878)?
                                                                                                                                    };
                                                                                                                                    let a1886 = {
                                                                                                                                        let a1882 = {
                                                                                                                                            let a1879 = v1;
                                                                                                                                            let a1880 = 32u64;
                                                                                                                                            let a1881 = 0u64;
                                                                                                                                            nat_rem(a1879, a1880, a1881)
                                                                                                                                        };
                                                                                                                                        let a1883 = 8u64;
                                                                                                                                        let a1884 = 0u64;
                                                                                                                                        nat_quot(a1882, a1883, a1884)
                                                                                                                                    };
                                                                                                                                    nat_add(a1885, a1886)?
                                                                                                                                };
                                                                                                                                nat_add(a1887, a1888)?
                                                                                                                            }, 4u64))
                                                                                                                        } else {
                                                                                                                            let a1889 = v3;
                                                                                                                            let a1890 = 2u64;
                                                                                                                            if nat_eq(a1889, a1890) {
                                                                                                                                Ok(Adt1::C3(8u64, {
                                                                                                                                    let a1899 = 1u64;
                                                                                                                                    let a1900 = {
                                                                                                                                        let a1897 = 2u64;
                                                                                                                                        let a1898 = {
                                                                                                                                            let a1894 = {
                                                                                                                                                let a1891 = v9;
                                                                                                                                                let a1892 = 4096u64;
                                                                                                                                                let a1893 = 0u64;
                                                                                                                                                nat_quot(a1891, a1892, a1893)
                                                                                                                                            };
                                                                                                                                            let a1895 = 8u64;
                                                                                                                                            let a1896 = 0u64;
                                                                                                                                            nat_rem(a1894, a1895, a1896)
                                                                                                                                        };
                                                                                                                                        nat_mul(a1897, a1898)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1899, a1900)?
                                                                                                                                }, 0u64, 2u64))
                                                                                                                            } else {
                                                                                                                                let a1901 = v3;
                                                                                                                                let a1902 = 3u64;
                                                                                                                                if nat_eq(a1901, a1902) {
                                                                                                                                    f4(2u64, v4, v6, 2u64)
                                                                                                                                } else {
                                                                                                                                    let a1903 = v3;
                                                                                                                                    let a1904 = 4u64;
                                                                                                                                    if nat_eq(a1903, a1904) {
                                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                            let a1930 = 18600u64;
                                                                                                                                            let a1931 = {
                                                                                                                                                let a1928 = {
                                                                                                                                                    let a1923 = {
                                                                                                                                                        let a1921 = v8;
                                                                                                                                                        let a1922 = {
                                                                                                                                                            let a1919 = {
                                                                                                                                                                let a1916 = {
                                                                                                                                                                    let a1913 = v9;
                                                                                                                                                                    let a1914 = f9({
                                                                                                                                                                        let a1911 = 3u64;
                                                                                                                                                                        let a1912 = {
                                                                                                                                                                            let a1908 = {
                                                                                                                                                                                let a1905 = v1;
                                                                                                                                                                                let a1906 = 32u64;
                                                                                                                                                                                let a1907 = 0u64;
                                                                                                                                                                                nat_rem(a1905, a1906, a1907)
                                                                                                                                                                            };
                                                                                                                                                                            let a1909 = 8u64;
                                                                                                                                                                            let a1910 = 0u64;
                                                                                                                                                                            nat_quot(a1908, a1909, a1910)
                                                                                                                                                                        };
                                                                                                                                                                        nat_mul(a1911, a1912)?
                                                                                                                                                                    });
                                                                                                                                                                    let a1915 = 0u64;
                                                                                                                                                                    nat_quot(a1913, a1914, a1915)
                                                                                                                                                                };
                                                                                                                                                                let a1917 = 8u64;
                                                                                                                                                                let a1918 = 0u64;
                                                                                                                                                                nat_rem(a1916, a1917, a1918)
                                                                                                                                                            };
                                                                                                                                                            let a1920 = 256u64;
                                                                                                                                                            nat_mul(a1919, a1920)?
                                                                                                                                                        };
                                                                                                                                                        nat_add(a1921, a1922)?
                                                                                                                                                    };
                                                                                                                                                    let a1924 = 8u64;
                                                                                                                                                    nat_mul(a1923, a1924)?
                                                                                                                                                };
                                                                                                                                                let a1929 = {
                                                                                                                                                    let a1925 = v1;
                                                                                                                                                    let a1926 = 8u64;
                                                                                                                                                    let a1927 = 0u64;
                                                                                                                                                    nat_rem(a1925, a1926, a1927)
                                                                                                                                                };
                                                                                                                                                nat_add(a1928, a1929)?
                                                                                                                                            };
                                                                                                                                            nat_add(a1930, a1931)?
                                                                                                                                        }, 3u64))
                                                                                                                                    } else {
                                                                                                                                        let a1932 = v3;
                                                                                                                                        let a1933 = 5u64;
                                                                                                                                        if nat_eq(a1932, a1933) {
                                                                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                let a1934 = v7;
                                                                                                                                                let a1935 = 128u64;
                                                                                                                                                if nat_lt(a1934, a1935) {
                                                                                                                                                    0u64
                                                                                                                                                } else {
                                                                                                                                                    256u64
                                                                                                                                                }
                                                                                                                                            }, 3u64))
                                                                                                                                        } else {
                                                                                                                                            let a1936 = v3;
                                                                                                                                            let a1937 = 6u64;
                                                                                                                                            if nat_eq(a1936, a1937) {
                                                                                                                                                f4(0u64, v7, {
                                                                                                                                                    let a1944 = {
                                                                                                                                                        let a1941 = {
                                                                                                                                                            let a1938 = v9;
                                                                                                                                                            let a1939 = 32768u64;
                                                                                                                                                            let a1940 = 0u64;
                                                                                                                                                            nat_quot(a1938, a1939, a1940)
                                                                                                                                                        };
                                                                                                                                                        let a1942 = 2u64;
                                                                                                                                                        let a1943 = 0u64;
                                                                                                                                                        nat_rem(a1941, a1942, a1943)
                                                                                                                                                    };
                                                                                                                                                    let a1945 = 1u64;
                                                                                                                                                    if nat_eq(a1944, a1945) {
                                                                                                                                                        3187671040u64
                                                                                                                                                    } else {
                                                                                                                                                        1040187392u64
                                                                                                                                                    }
                                                                                                                                                }, 3u64)
                                                                                                                                            } else {
                                                                                                                                                let a1946 = v3;
                                                                                                                                                let a1947 = 7u64;
                                                                                                                                                if nat_eq(a1946, a1947) {
                                                                                                                                                    f4(2u64, v6, v7, 2u64)
                                                                                                                                                } else {
                                                                                                                                                    let a1948 = v3;
                                                                                                                                                    let a1949 = 8u64;
                                                                                                                                                    if nat_eq(a1948, a1949) {
                                                                                                                                                        Ok(Adt1::C4({
                                                                                                                                                            let a1950 = v1;
                                                                                                                                                            let a1951 = v2;
                                                                                                                                                            nat_mul(a1950, a1951)?
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
                                                                                                                    let a1952 = v0;
                                                                                                                    let a1953 = 29u64;
                                                                                                                    if nat_eq(a1952, a1953) {
                                                                                                                        let a1954 = v3;
                                                                                                                        let a1955 = 0u64;
                                                                                                                        if nat_eq(a1954, a1955) {
                                                                                                                            Ok(Adt1::C2(0u64, 2u64, {
                                                                                                                                let a1961 = 48u64;
                                                                                                                                let a1962 = {
                                                                                                                                    let a1959 = {
                                                                                                                                        let a1956 = v1;
                                                                                                                                        let a1957 = 64u64;
                                                                                                                                        let a1958 = 0u64;
                                                                                                                                        nat_quot(a1956, a1957, a1958)
                                                                                                                                    };
                                                                                                                                    let a1960 = 2u64;
                                                                                                                                    nat_mul(a1959, a1960)?
                                                                                                                                };
                                                                                                                                nat_add(a1961, a1962)?
                                                                                                                            }, 6u64))
                                                                                                                        } else {
                                                                                                                            let a1963 = v3;
                                                                                                                            let a1964 = 1u64;
                                                                                                                            if nat_eq(a1963, a1964) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a1981 = 32u64;
                                                                                                                                    let a1982 = {
                                                                                                                                        let a1979 = {
                                                                                                                                            let a1968 = {
                                                                                                                                                let a1965 = v1;
                                                                                                                                                let a1966 = 32u64;
                                                                                                                                                let a1967 = 0u64;
                                                                                                                                                nat_quot(a1965, a1966, a1967)
                                                                                                                                            };
                                                                                                                                            let a1969 = 2u64;
                                                                                                                                            nat_mul(a1968, a1969)?
                                                                                                                                        };
                                                                                                                                        let a1980 = {
                                                                                                                                            let a1976 = {
                                                                                                                                                let a1973 = {
                                                                                                                                                    let a1970 = v1;
                                                                                                                                                    let a1971 = 32u64;
                                                                                                                                                    let a1972 = 0u64;
                                                                                                                                                    nat_rem(a1970, a1971, a1972)
                                                                                                                                                };
                                                                                                                                                let a1974 = 8u64;
                                                                                                                                                let a1975 = 0u64;
                                                                                                                                                nat_quot(a1973, a1974, a1975)
                                                                                                                                            };
                                                                                                                                            let a1977 = 2u64;
                                                                                                                                            let a1978 = 0u64;
                                                                                                                                            nat_quot(a1976, a1977, a1978)
                                                                                                                                        };
                                                                                                                                        nat_add(a1979, a1980)?
                                                                                                                                    };
                                                                                                                                    nat_add(a1981, a1982)?
                                                                                                                                }, 5u64))
                                                                                                                            } else {
                                                                                                                                let a1983 = v3;
                                                                                                                                let a1984 = 2u64;
                                                                                                                                if nat_eq(a1983, a1984) {
                                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                        let a1985 = v1;
                                                                                                                                        let a1986 = 8u64;
                                                                                                                                        let a1987 = 0u64;
                                                                                                                                        nat_quot(a1985, a1986, a1987)
                                                                                                                                    }, 4u64))
                                                                                                                                } else {
                                                                                                                                    let a1988 = v3;
                                                                                                                                    let a1989 = 3u64;
                                                                                                                                    if nat_eq(a1988, a1989) {
                                                                                                                                        Ok(Adt1::C3(8u64, {
                                                                                                                                            let a2006 = 1u64;
                                                                                                                                            let a2007 = {
                                                                                                                                                let a2004 = 2u64;
                                                                                                                                                let a2005 = {
                                                                                                                                                    let a2001 = {
                                                                                                                                                        let a1998 = v10;
                                                                                                                                                        let a1999 = f9({
                                                                                                                                                            let a1996 = 3u64;
                                                                                                                                                            let a1997 = {
                                                                                                                                                                let a1993 = {
                                                                                                                                                                    let a1990 = v1;
                                                                                                                                                                    let a1991 = 16u64;
                                                                                                                                                                    let a1992 = 0u64;
                                                                                                                                                                    nat_quot(a1990, a1991, a1992)
                                                                                                                                                                };
                                                                                                                                                                let a1994 = 4u64;
                                                                                                                                                                let a1995 = 0u64;
                                                                                                                                                                nat_rem(a1993, a1994, a1995)
                                                                                                                                                            };
                                                                                                                                                            nat_mul(a1996, a1997)?
                                                                                                                                                        });
                                                                                                                                                        let a2000 = 0u64;
                                                                                                                                                        nat_quot(a1998, a1999, a2000)
                                                                                                                                                    };
                                                                                                                                                    let a2002 = 8u64;
                                                                                                                                                    let a2003 = 0u64;
                                                                                                                                                    nat_rem(a2001, a2002, a2003)
                                                                                                                                                };
                                                                                                                                                nat_mul(a2004, a2005)?
                                                                                                                                            };
                                                                                                                                            nat_add(a2006, a2007)?
                                                                                                                                        }, 0u64, 2u64))
                                                                                                                                    } else {
                                                                                                                                        let a2008 = v3;
                                                                                                                                        let a2009 = 4u64;
                                                                                                                                        if nat_eq(a2008, a2009) {
                                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a2010 = v3;
                                                                                                                                            let a2011 = 5u64;
                                                                                                                                            if nat_eq(a2010, a2011) {
                                                                                                                                                Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                    let a2040 = 18600u64;
                                                                                                                                                    let a2041 = {
                                                                                                                                                        let a2038 = {
                                                                                                                                                            let a2033 = {
                                                                                                                                                                let a2031 = v8;
                                                                                                                                                                let a2032 = {
                                                                                                                                                                    let a2029 = {
                                                                                                                                                                        let a2026 = {
                                                                                                                                                                            let a2023 = v9;
                                                                                                                                                                            let a2024 = f9({
                                                                                                                                                                                let a2021 = 4u64;
                                                                                                                                                                                let a2022 = {
                                                                                                                                                                                    let a2018 = {
                                                                                                                                                                                        let a2015 = {
                                                                                                                                                                                            let a2012 = v1;
                                                                                                                                                                                            let a2013 = 32u64;
                                                                                                                                                                                            let a2014 = 0u64;
                                                                                                                                                                                            nat_rem(a2012, a2013, a2014)
                                                                                                                                                                                        };
                                                                                                                                                                                        let a2016 = 8u64;
                                                                                                                                                                                        let a2017 = 0u64;
                                                                                                                                                                                        nat_quot(a2015, a2016, a2017)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2019 = 2u64;
                                                                                                                                                                                    let a2020 = 0u64;
                                                                                                                                                                                    nat_rem(a2018, a2019, a2020)
                                                                                                                                                                                };
                                                                                                                                                                                nat_mul(a2021, a2022)?
                                                                                                                                                                            });
                                                                                                                                                                            let a2025 = 0u64;
                                                                                                                                                                            nat_quot(a2023, a2024, a2025)
                                                                                                                                                                        };
                                                                                                                                                                        let a2027 = 8u64;
                                                                                                                                                                        let a2028 = 0u64;
                                                                                                                                                                        nat_rem(a2026, a2027, a2028)
                                                                                                                                                                    };
                                                                                                                                                                    let a2030 = 256u64;
                                                                                                                                                                    nat_mul(a2029, a2030)?
                                                                                                                                                                };
                                                                                                                                                                nat_add(a2031, a2032)?
                                                                                                                                                            };
                                                                                                                                                            let a2034 = 8u64;
                                                                                                                                                            nat_mul(a2033, a2034)?
                                                                                                                                                        };
                                                                                                                                                        let a2039 = {
                                                                                                                                                            let a2035 = v1;
                                                                                                                                                            let a2036 = 8u64;
                                                                                                                                                            let a2037 = 0u64;
                                                                                                                                                            nat_rem(a2035, a2036, a2037)
                                                                                                                                                        };
                                                                                                                                                        nat_add(a2038, a2039)?
                                                                                                                                                    };
                                                                                                                                                    nat_add(a2040, a2041)?
                                                                                                                                                }, 3u64))
                                                                                                                                            } else {
                                                                                                                                                let a2042 = v3;
                                                                                                                                                let a2043 = 6u64;
                                                                                                                                                if nat_eq(a2042, a2043) {
                                                                                                                                                    Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                        let a2044 = v7;
                                                                                                                                                        let a2045 = 128u64;
                                                                                                                                                        if nat_lt(a2044, a2045) {
                                                                                                                                                            0u64
                                                                                                                                                        } else {
                                                                                                                                                            256u64
                                                                                                                                                        }
                                                                                                                                                    }, 3u64))
                                                                                                                                                } else {
                                                                                                                                                    let a2046 = v3;
                                                                                                                                                    let a2047 = 7u64;
                                                                                                                                                    if nat_eq(a2046, a2047) {
                                                                                                                                                        f4(0u64, v7, {
                                                                                                                                                            let a2067 = {
                                                                                                                                                                let a2064 = {
                                                                                                                                                                    let a2061 = v9;
                                                                                                                                                                    let a2062 = f9({
                                                                                                                                                                        let a2059 = {
                                                                                                                                                                            let a2057 = 4u64;
                                                                                                                                                                            let a2058 = {
                                                                                                                                                                                let a2054 = {
                                                                                                                                                                                    let a2051 = {
                                                                                                                                                                                        let a2048 = v1;
                                                                                                                                                                                        let a2049 = 32u64;
                                                                                                                                                                                        let a2050 = 0u64;
                                                                                                                                                                                        nat_rem(a2048, a2049, a2050)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2052 = 8u64;
                                                                                                                                                                                    let a2053 = 0u64;
                                                                                                                                                                                    nat_quot(a2051, a2052, a2053)
                                                                                                                                                                                };
                                                                                                                                                                                let a2055 = 2u64;
                                                                                                                                                                                let a2056 = 0u64;
                                                                                                                                                                                nat_rem(a2054, a2055, a2056)
                                                                                                                                                                            };
                                                                                                                                                                            nat_mul(a2057, a2058)?
                                                                                                                                                                        };
                                                                                                                                                                        let a2060 = 3u64;
                                                                                                                                                                        nat_add(a2059, a2060)?
                                                                                                                                                                    });
                                                                                                                                                                    let a2063 = 0u64;
                                                                                                                                                                    nat_quot(a2061, a2062, a2063)
                                                                                                                                                                };
                                                                                                                                                                let a2065 = 2u64;
                                                                                                                                                                let a2066 = 0u64;
                                                                                                                                                                nat_rem(a2064, a2065, a2066)
                                                                                                                                                            };
                                                                                                                                                            let a2068 = 1u64;
                                                                                                                                                            if nat_eq(a2067, a2068) {
                                                                                                                                                                3187671040u64
                                                                                                                                                            } else {
                                                                                                                                                                1040187392u64
                                                                                                                                                            }
                                                                                                                                                        }, 3u64)
                                                                                                                                                    } else {
                                                                                                                                                        let a2069 = v3;
                                                                                                                                                        let a2070 = 8u64;
                                                                                                                                                        if nat_eq(a2069, a2070) {
                                                                                                                                                            f4(2u64, v6, v7, 2u64)
                                                                                                                                                        } else {
                                                                                                                                                            let a2071 = v3;
                                                                                                                                                            let a2072 = 9u64;
                                                                                                                                                            if nat_eq(a2071, a2072) {
                                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                                    let a2073 = v1;
                                                                                                                                                                    let a2074 = v2;
                                                                                                                                                                    nat_mul(a2073, a2074)?
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
                                                                                                                        let a2075 = v0;
                                                                                                                        let a2076 = 20u64;
                                                                                                                        if nat_eq(a2075, a2076) {
                                                                                                                            let a2077 = v3;
                                                                                                                            let a2078 = 0u64;
                                                                                                                            if nat_eq(a2077, a2078) {
                                                                                                                                Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                    let a2082 = 2u64;
                                                                                                                                    let a2083 = {
                                                                                                                                        let a2079 = v1;
                                                                                                                                        let a2080 = 16u64;
                                                                                                                                        let a2081 = 0u64;
                                                                                                                                        nat_rem(a2079, a2080, a2081)
                                                                                                                                    };
                                                                                                                                    nat_add(a2082, a2083)?
                                                                                                                                }, 4u64))
                                                                                                                            } else {
                                                                                                                                let a2084 = v3;
                                                                                                                                let a2085 = 1u64;
                                                                                                                                if nat_eq(a2084, a2085) {
                                                                                                                                    Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                        let a2101 = 18568u64;
                                                                                                                                        let a2102 = {
                                                                                                                                            let a2099 = {
                                                                                                                                                let a2097 = {
                                                                                                                                                    let a2094 = {
                                                                                                                                                        let a2091 = v8;
                                                                                                                                                        let a2092 = {
                                                                                                                                                            let a2089 = {
                                                                                                                                                                let a2086 = v1;
                                                                                                                                                                let a2087 = 16u64;
                                                                                                                                                                let a2088 = 0u64;
                                                                                                                                                                nat_quot(a2086, a2087, a2088)
                                                                                                                                                            };
                                                                                                                                                            let a2090 = 0u64;
                                                                                                                                                            if nat_eq(a2089, a2090) {
                                                                                                                                                                1u64
                                                                                                                                                            } else {
                                                                                                                                                                16u64
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        let a2093 = 0u64;
                                                                                                                                                        nat_quot(a2091, a2092, a2093)
                                                                                                                                                    };
                                                                                                                                                    let a2095 = 16u64;
                                                                                                                                                    let a2096 = 0u64;
                                                                                                                                                    nat_rem(a2094, a2095, a2096)
                                                                                                                                                };
                                                                                                                                                let a2098 = 1u64;
                                                                                                                                                nat_mul(a2097, a2098)?
                                                                                                                                            };
                                                                                                                                            let a2100 = 0u64;
                                                                                                                                            nat_add(a2099, a2100)?
                                                                                                                                        };
                                                                                                                                        nat_add(a2101, a2102)?
                                                                                                                                    }, 3u64))
                                                                                                                                } else {
                                                                                                                                    let a2103 = v3;
                                                                                                                                    let a2104 = 2u64;
                                                                                                                                    if nat_eq(a2103, a2104) {
                                                                                                                                        Ok(Adt1::C3(8u64, v7, {
                                                                                                                                            let a2105 = v7;
                                                                                                                                            let a2106 = 128u64;
                                                                                                                                            if nat_lt(a2105, a2106) {
                                                                                                                                                0u64
                                                                                                                                            } else {
                                                                                                                                                256u64
                                                                                                                                            }
                                                                                                                                        }, 2u64))
                                                                                                                                    } else {
                                                                                                                                        let a2107 = v3;
                                                                                                                                        let a2108 = 3u64;
                                                                                                                                        if nat_eq(a2107, a2108) {
                                                                                                                                            f4(2u64, v4, v6, 2u64)
                                                                                                                                        } else {
                                                                                                                                            let a2109 = v3;
                                                                                                                                            let a2110 = 4u64;
                                                                                                                                            if nat_eq(a2109, a2110) {
                                                                                                                                                Ok(Adt1::C4({
                                                                                                                                                    let a2111 = v1;
                                                                                                                                                    let a2112 = v2;
                                                                                                                                                    nat_mul(a2111, a2112)?
                                                                                                                                                }, v6))
                                                                                                                                            } else {
                                                                                                                                                Ok(Adt1::C0(0u64))
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            let a2113 = v0;
                                                                                                                            let a2114 = 23u64;
                                                                                                                            if nat_eq(a2113, a2114) {
                                                                                                                                let a2115 = v3;
                                                                                                                                let a2116 = 0u64;
                                                                                                                                if nat_eq(a2115, a2116) {
                                                                                                                                    Ok(Adt1::C2(0u64, 2u64, 2u64, 5u64))
                                                                                                                                } else {
                                                                                                                                    let a2117 = v3;
                                                                                                                                    let a2118 = 1u64;
                                                                                                                                    if nat_eq(a2117, a2118) {
                                                                                                                                        Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                            let a2125 = 4u64;
                                                                                                                                            let a2126 = {
                                                                                                                                                let a2122 = {
                                                                                                                                                    let a2119 = v1;
                                                                                                                                                    let a2120 = 32u64;
                                                                                                                                                    let a2121 = 0u64;
                                                                                                                                                    nat_quot(a2119, a2120, a2121)
                                                                                                                                                };
                                                                                                                                                let a2123 = 2u64;
                                                                                                                                                let a2124 = 0u64;
                                                                                                                                                nat_quot(a2122, a2123, a2124)
                                                                                                                                            };
                                                                                                                                            nat_add(a2125, a2126)?
                                                                                                                                        }, 4u64))
                                                                                                                                    } else {
                                                                                                                                        let a2127 = v3;
                                                                                                                                        let a2128 = 2u64;
                                                                                                                                        if nat_eq(a2127, a2128) {
                                                                                                                                            Ok(Adt1::C3(8u64, {
                                                                                                                                                let a2156 = {
                                                                                                                                                    let a2140 = {
                                                                                                                                                        let a2137 = v8;
                                                                                                                                                        let a2138 = {
                                                                                                                                                            let a2135 = {
                                                                                                                                                                let a2132 = {
                                                                                                                                                                    let a2129 = v1;
                                                                                                                                                                    let a2130 = 32u64;
                                                                                                                                                                    let a2131 = 0u64;
                                                                                                                                                                    nat_quot(a2129, a2130, a2131)
                                                                                                                                                                };
                                                                                                                                                                let a2133 = 2u64;
                                                                                                                                                                let a2134 = 0u64;
                                                                                                                                                                nat_rem(a2132, a2133, a2134)
                                                                                                                                                            };
                                                                                                                                                            let a2136 = 0u64;
                                                                                                                                                            if nat_eq(a2135, a2136) {
                                                                                                                                                                1u64
                                                                                                                                                            } else {
                                                                                                                                                                16u64
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        let a2139 = 0u64;
                                                                                                                                                        nat_quot(a2137, a2138, a2139)
                                                                                                                                                    };
                                                                                                                                                    let a2141 = 16u64;
                                                                                                                                                    let a2142 = 0u64;
                                                                                                                                                    nat_rem(a2140, a2141, a2142)
                                                                                                                                                };
                                                                                                                                                let a2157 = {
                                                                                                                                                    let a2154 = {
                                                                                                                                                        let a2151 = {
                                                                                                                                                            let a2148 = v9;
                                                                                                                                                            let a2149 = f9({
                                                                                                                                                                let a2146 = 2u64;
                                                                                                                                                                let a2147 = {
                                                                                                                                                                    let a2143 = v1;
                                                                                                                                                                    let a2144 = 32u64;
                                                                                                                                                                    let a2145 = 0u64;
                                                                                                                                                                    nat_quot(a2143, a2144, a2145)
                                                                                                                                                                };
                                                                                                                                                                nat_mul(a2146, a2147)?
                                                                                                                                                            });
                                                                                                                                                            let a2150 = 0u64;
                                                                                                                                                            nat_quot(a2148, a2149, a2150)
                                                                                                                                                        };
                                                                                                                                                        let a2152 = 4u64;
                                                                                                                                                        let a2153 = 0u64;
                                                                                                                                                        nat_rem(a2151, a2152, a2153)
                                                                                                                                                    };
                                                                                                                                                    let a2155 = 16u64;
                                                                                                                                                    nat_mul(a2154, a2155)?
                                                                                                                                                };
                                                                                                                                                nat_add(a2156, a2157)?
                                                                                                                                            }, 32u64, 2u64))
                                                                                                                                        } else {
                                                                                                                                            let a2158 = v3;
                                                                                                                                            let a2159 = 3u64;
                                                                                                                                            if nat_eq(a2158, a2159) {
                                                                                                                                                f4(2u64, v4, v6, 2u64)
                                                                                                                                            } else {
                                                                                                                                                let a2160 = v3;
                                                                                                                                                let a2161 = 4u64;
                                                                                                                                                if nat_eq(a2160, a2161) {
                                                                                                                                                    Ok(Adt1::C2(0u64, 1u64, {
                                                                                                                                                        let a2172 = 8u64;
                                                                                                                                                        let a2173 = {
                                                                                                                                                            let a2170 = {
                                                                                                                                                                let a2165 = {
                                                                                                                                                                    let a2162 = v1;
                                                                                                                                                                    let a2163 = 32u64;
                                                                                                                                                                    let a2164 = 0u64;
                                                                                                                                                                    nat_quot(a2162, a2163, a2164)
                                                                                                                                                                };
                                                                                                                                                                let a2166 = 16u64;
                                                                                                                                                                nat_mul(a2165, a2166)?
                                                                                                                                                            };
                                                                                                                                                            let a2171 = {
                                                                                                                                                                let a2167 = v1;
                                                                                                                                                                let a2168 = 16u64;
                                                                                                                                                                let a2169 = 0u64;
                                                                                                                                                                nat_rem(a2167, a2168, a2169)
                                                                                                                                                            };
                                                                                                                                                            nat_add(a2170, a2171)?
                                                                                                                                                        };
                                                                                                                                                        nat_add(a2172, a2173)?
                                                                                                                                                    }, 4u64))
                                                                                                                                                } else {
                                                                                                                                                    let a2174 = v3;
                                                                                                                                                    let a2175 = 5u64;
                                                                                                                                                    if nat_eq(a2174, a2175) {
                                                                                                                                                        Ok(Adt1::C2(1u64, 1u64, {
                                                                                                                                                            let a2194 = 18568u64;
                                                                                                                                                            let a2195 = {
                                                                                                                                                                let a2192 = {
                                                                                                                                                                    let a2190 = {
                                                                                                                                                                        let a2187 = {
                                                                                                                                                                            let a2184 = v8;
                                                                                                                                                                            let a2185 = {
                                                                                                                                                                                let a2182 = {
                                                                                                                                                                                    let a2179 = {
                                                                                                                                                                                        let a2176 = v1;
                                                                                                                                                                                        let a2177 = 32u64;
                                                                                                                                                                                        let a2178 = 0u64;
                                                                                                                                                                                        nat_rem(a2176, a2177, a2178)
                                                                                                                                                                                    };
                                                                                                                                                                                    let a2180 = 16u64;
                                                                                                                                                                                    let a2181 = 0u64;
                                                                                                                                                                                    nat_quot(a2179, a2180, a2181)
                                                                                                                                                                                };
                                                                                                                                                                                let a2183 = 0u64;
                                                                                                                                                                                if nat_eq(a2182, a2183) {
                                                                                                                                                                                    1u64
                                                                                                                                                                                } else {
                                                                                                                                                                                    16u64
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            let a2186 = 0u64;
                                                                                                                                                                            nat_quot(a2184, a2185, a2186)
                                                                                                                                                                        };
                                                                                                                                                                        let a2188 = 16u64;
                                                                                                                                                                        let a2189 = 0u64;
                                                                                                                                                                        nat_rem(a2187, a2188, a2189)
                                                                                                                                                                    };
                                                                                                                                                                    let a2191 = 1u64;
                                                                                                                                                                    nat_mul(a2190, a2191)?
                                                                                                                                                                };
                                                                                                                                                                let a2193 = 0u64;
                                                                                                                                                                nat_add(a2192, a2193)?
                                                                                                                                                            };
                                                                                                                                                            nat_add(a2194, a2195)?
                                                                                                                                                        }, 3u64))
                                                                                                                                                    } else {
                                                                                                                                                        let a2196 = v3;
                                                                                                                                                        let a2197 = 6u64;
                                                                                                                                                        if nat_eq(a2196, a2197) {
                                                                                                                                                            Ok(Adt1::C3(8u64, v7, {
                                                                                                                                                                let a2198 = v7;
                                                                                                                                                                let a2199 = 128u64;
                                                                                                                                                                if nat_lt(a2198, a2199) {
                                                                                                                                                                    0u64
                                                                                                                                                                } else {
                                                                                                                                                                    256u64
                                                                                                                                                                }
                                                                                                                                                            }, 3u64))
                                                                                                                                                        } else {
                                                                                                                                                            let a2200 = v3;
                                                                                                                                                            let a2201 = 7u64;
                                                                                                                                                            if nat_eq(a2200, a2201) {
                                                                                                                                                                f4(2u64, v6, v7, 2u64)
                                                                                                                                                            } else {
                                                                                                                                                                let a2202 = v3;
                                                                                                                                                                let a2203 = 8u64;
                                                                                                                                                                if nat_eq(a2202, a2203) {
                                                                                                                                                                    Ok(Adt1::C4({
                                                                                                                                                                        let a2204 = v1;
                                                                                                                                                                        let a2205 = v2;
                                                                                                                                                                        nat_mul(a2204, a2205)?
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
    let a2206 = v0;
    let a2207 = 4u64;
    if if nat_lt(a2206, a2207) {
        let a2208 = v1;
        let a2209 = 4294967296u64;
        if nat_lt(a2208, a2209) {
            let a2210 = v2;
            let a2211 = 4294967296u64;
            if nat_lt(a2210, a2211) {
                let a2212 = v3;
                let a2213 = 8u64;
                nat_lt(a2212, a2213)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        if f11(v1) {
            Ok(Adt1::C3(11u64, f12(v1)?, 0u64, v3))
        } else {
            if f11(v2) {
                Ok(Adt1::C3(11u64, f12(v2)?, 0u64, v3))
            } else {
                let a2214 = v0;
                let a2215 = 0u64;
                if if nat_eq(a2214, a2215) {
                    let a2216 = f13(v1);
                    let a2217 = 2139095040u64;
                    if if nat_eq(a2216, a2217) {
                        let a2218 = f13(v2);
                        let a2219 = 2139095040u64;
                        nat_eq(a2218, a2219)
                    } else {
                        false
                    } {
                        let a2226 = {
                            let a2220 = v1;
                            let a2221 = 2147483648u64;
                            let a2222 = 0u64;
                            nat_quot(a2220, a2221, a2222)
                        };
                        let a2227 = {
                            let a2223 = v2;
                            let a2224 = 2147483648u64;
                            let a2225 = 0u64;
                            nat_quot(a2223, a2224, a2225)
                        };
                        !nat_eq(a2226, a2227)
                    } else {
                        false
                    }
                } else {
                    let a2228 = v0;
                    let a2229 = 1u64;
                    if nat_eq(a2228, a2229) {
                        let a2230 = f13(v1);
                        let a2231 = 2139095040u64;
                        if if nat_eq(a2230, a2231) {
                            let a2232 = f13(v2);
                            let a2233 = 2139095040u64;
                            nat_eq(a2232, a2233)
                        } else {
                            false
                        } {
                            let a2240 = {
                                let a2234 = v1;
                                let a2235 = 2147483648u64;
                                let a2236 = 0u64;
                                nat_quot(a2234, a2235, a2236)
                            };
                            let a2241 = {
                                let a2237 = v2;
                                let a2238 = 2147483648u64;
                                let a2239 = 0u64;
                                nat_quot(a2237, a2238, a2239)
                            };
                            nat_eq(a2240, a2241)
                        } else {
                            false
                        }
                    } else {
                        let a2242 = v0;
                        let a2243 = 2u64;
                        if nat_eq(a2242, a2243) {
                            let a2244 = f13(v1);
                            let a2245 = 2139095040u64;
                            if if nat_eq(a2244, a2245) {
                                let a2246 = f13(v2);
                                let a2247 = 0u64;
                                nat_eq(a2246, a2247)
                            } else {
                                false
                            } {
                                true
                            } else {
                                let a2248 = f13(v1);
                                let a2249 = 0u64;
                                if nat_eq(a2248, a2249) {
                                    let a2250 = f13(v2);
                                    let a2251 = 2139095040u64;
                                    nat_eq(a2250, a2251)
                                } else {
                                    false
                                }
                            }
                        } else {
                            let a2252 = v0;
                            let a2253 = 3u64;
                            if nat_eq(a2252, a2253) {
                                let a2254 = f13(v1);
                                let a2255 = 2139095040u64;
                                if if nat_eq(a2254, a2255) {
                                    let a2256 = f13(v2);
                                    let a2257 = 2139095040u64;
                                    nat_eq(a2256, a2257)
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    let a2258 = f13(v1);
                                    let a2259 = 0u64;
                                    if nat_eq(a2258, a2259) {
                                        let a2260 = f13(v2);
                                        let a2261 = 0u64;
                                        nat_eq(a2260, a2261)
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
    let a2262 = v0;
    let a2263 = 0u64;
    if nat_eq(a2262, a2263) {
        1u64
    } else {
        let a2264 = v0;
        let a2265 = 1u64;
        if nat_eq(a2264, a2265) {
            1u64
        } else {
            let a2266 = v0;
            let a2267 = 2u64;
            if nat_eq(a2266, a2267) {
                32u64
            } else {
                let a2268 = v0;
                let a2269 = 3u64;
                if nat_eq(a2268, a2269) {
                    32u64
                } else {
                    let a2270 = v0;
                    let a2271 = 6u64;
                    if nat_eq(a2270, a2271) {
                        32u64
                    } else {
                        let a2272 = v0;
                        let a2273 = 7u64;
                        if nat_eq(a2272, a2273) {
                            32u64
                        } else {
                            let a2274 = v0;
                            let a2275 = 8u64;
                            if nat_eq(a2274, a2275) {
                                32u64
                            } else {
                                let a2276 = v0;
                                let a2277 = 10u64;
                                if nat_eq(a2276, a2277) {
                                    256u64
                                } else {
                                    let a2278 = v0;
                                    let a2279 = 11u64;
                                    if nat_eq(a2278, a2279) {
                                        256u64
                                    } else {
                                        let a2280 = v0;
                                        let a2281 = 12u64;
                                        if nat_eq(a2280, a2281) {
                                            256u64
                                        } else {
                                            let a2282 = v0;
                                            let a2283 = 13u64;
                                            if nat_eq(a2282, a2283) {
                                                256u64
                                            } else {
                                                let a2284 = v0;
                                                let a2285 = 14u64;
                                                if nat_eq(a2284, a2285) {
                                                    256u64
                                                } else {
                                                    let a2286 = v0;
                                                    let a2287 = 16u64;
                                                    if nat_eq(a2286, a2287) {
                                                        256u64
                                                    } else {
                                                        let a2288 = v0;
                                                        let a2289 = 17u64;
                                                        if nat_eq(a2288, a2289) {
                                                            256u64
                                                        } else {
                                                            let a2290 = v0;
                                                            let a2291 = 18u64;
                                                            if nat_eq(a2290, a2291) {
                                                                256u64
                                                            } else {
                                                                let a2292 = v0;
                                                                let a2293 = 19u64;
                                                                if nat_eq(a2292, a2293) {
                                                                    256u64
                                                                } else {
                                                                    let a2294 = v0;
                                                                    let a2295 = 20u64;
                                                                    if nat_eq(a2294, a2295) {
                                                                        32u64
                                                                    } else {
                                                                        let a2296 = v0;
                                                                        let a2297 = 21u64;
                                                                        if nat_eq(a2296, a2297) {
                                                                            256u64
                                                                        } else {
                                                                            let a2298 = v0;
                                                                            let a2299 = 22u64;
                                                                            if nat_eq(a2298, a2299) {
                                                                                256u64
                                                                            } else {
                                                                                let a2300 = v0;
                                                                                let a2301 = 23u64;
                                                                                if nat_eq(a2300, a2301) {
                                                                                    256u64
                                                                                } else {
                                                                                    let a2302 = v0;
                                                                                    let a2303 = 26u64;
                                                                                    if nat_eq(a2302, a2303) {
                                                                                        1u64
                                                                                    } else {
                                                                                        let a2304 = v0;
                                                                                        let a2305 = 29u64;
                                                                                        if nat_eq(a2304, a2305) {
                                                                                            256u64
                                                                                        } else {
                                                                                            let a2306 = v0;
                                                                                            let a2307 = 30u64;
                                                                                            if nat_eq(a2306, a2307) {
                                                                                                1u64
                                                                                            } else {
                                                                                                let a2308 = v0;
                                                                                                let a2309 = 34u64;
                                                                                                if nat_eq(a2308, a2309) {
                                                                                                    256u64
                                                                                                } else {
                                                                                                    let a2310 = v0;
                                                                                                    let a2311 = 35u64;
                                                                                                    if nat_eq(a2310, a2311) {
                                                                                                        256u64
                                                                                                    } else {
                                                                                                        let a2312 = v0;
                                                                                                        let a2313 = 39u64;
                                                                                                        if nat_eq(a2312, a2313) {
                                                                                                            32u64
                                                                                                        } else {
                                                                                                            let a2314 = v0;
                                                                                                            let a2315 = 40u64;
                                                                                                            if nat_eq(a2314, a2315) {
                                                                                                                64u64
                                                                                                            } else {
                                                                                                                let a2316 = v0;
                                                                                                                let a2317 = 41u64;
                                                                                                                if nat_eq(a2316, a2317) {
                                                                                                                    128u64
                                                                                                                } else {
                                                                                                                    let a2318 = v0;
                                                                                                                    let a2319 = 42u64;
                                                                                                                    if nat_eq(a2318, a2319) {
                                                                                                                        64u64
                                                                                                                    } else {
                                                                                                                        let a2320 = v0;
                                                                                                                        let a2321 = 43u64;
                                                                                                                        if nat_eq(a2320, a2321) {
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
    let a2322 = v0;
    let a2323 = 0u64;
    if nat_eq(a2322, a2323) {
        4u64
    } else {
        let a2324 = v0;
        let a2325 = 1u64;
        if nat_eq(a2324, a2325) {
            2u64
        } else {
            let a2326 = v0;
            let a2327 = 2u64;
            if nat_eq(a2326, a2327) {
                18u64
            } else {
                let a2328 = v0;
                let a2329 = 3u64;
                if nat_eq(a2328, a2329) {
                    20u64
                } else {
                    let a2330 = v0;
                    let a2331 = 6u64;
                    if nat_eq(a2330, a2331) {
                        22u64
                    } else {
                        let a2332 = v0;
                        let a2333 = 7u64;
                        if nat_eq(a2332, a2333) {
                            24u64
                        } else {
                            let a2334 = v0;
                            let a2335 = 8u64;
                            if nat_eq(a2334, a2335) {
                                34u64
                            } else {
                                let a2336 = v0;
                                let a2337 = 10u64;
                                if nat_eq(a2336, a2337) {
                                    84u64
                                } else {
                                    let a2338 = v0;
                                    let a2339 = 11u64;
                                    if nat_eq(a2338, a2339) {
                                        110u64
                                    } else {
                                        let a2340 = v0;
                                        let a2341 = 12u64;
                                        if nat_eq(a2340, a2341) {
                                            144u64
                                        } else {
                                            let a2342 = v0;
                                            let a2343 = 13u64;
                                            if nat_eq(a2342, a2343) {
                                                176u64
                                            } else {
                                                let a2344 = v0;
                                                let a2345 = 14u64;
                                                if nat_eq(a2344, a2345) {
                                                    210u64
                                                } else {
                                                    let a2346 = v0;
                                                    let a2347 = 16u64;
                                                    if nat_eq(a2346, a2347) {
                                                        66u64
                                                    } else {
                                                        let a2348 = v0;
                                                        let a2349 = 17u64;
                                                        if nat_eq(a2348, a2349) {
                                                            74u64
                                                        } else {
                                                            let a2350 = v0;
                                                            let a2351 = 18u64;
                                                            if nat_eq(a2350, a2351) {
                                                                98u64
                                                            } else {
                                                                let a2352 = v0;
                                                                let a2353 = 19u64;
                                                                if nat_eq(a2352, a2353) {
                                                                    50u64
                                                                } else {
                                                                    let a2354 = v0;
                                                                    let a2355 = 20u64;
                                                                    if nat_eq(a2354, a2355) {
                                                                        18u64
                                                                    } else {
                                                                        let a2356 = v0;
                                                                        let a2357 = 21u64;
                                                                        if nat_eq(a2356, a2357) {
                                                                            110u64
                                                                        } else {
                                                                            let a2358 = v0;
                                                                            let a2359 = 22u64;
                                                                            if nat_eq(a2358, a2359) {
                                                                                82u64
                                                                            } else {
                                                                                let a2360 = v0;
                                                                                let a2361 = 23u64;
                                                                                if nat_eq(a2360, a2361) {
                                                                                    136u64
                                                                                } else {
                                                                                    let a2362 = v0;
                                                                                    let a2363 = 26u64;
                                                                                    if nat_eq(a2362, a2363) {
                                                                                        4u64
                                                                                    } else {
                                                                                        let a2364 = v0;
                                                                                        let a2365 = 29u64;
                                                                                        if nat_eq(a2364, a2365) {
                                                                                            56u64
                                                                                        } else {
                                                                                            let a2366 = v0;
                                                                                            let a2367 = 30u64;
                                                                                            if nat_eq(a2366, a2367) {
                                                                                                2u64
                                                                                            } else {
                                                                                                let a2368 = v0;
                                                                                                let a2369 = 34u64;
                                                                                                if nat_eq(a2368, a2369) {
                                                                                                    54u64
                                                                                                } else {
                                                                                                    let a2370 = v0;
                                                                                                    let a2371 = 35u64;
                                                                                                    if nat_eq(a2370, a2371) {
                                                                                                        66u64
                                                                                                    } else {
                                                                                                        let a2372 = v0;
                                                                                                        let a2373 = 39u64;
                                                                                                        if nat_eq(a2372, a2373) {
                                                                                                            17u64
                                                                                                        } else {
                                                                                                            let a2374 = v0;
                                                                                                            let a2375 = 40u64;
                                                                                                            if nat_eq(a2374, a2375) {
                                                                                                                36u64
                                                                                                            } else {
                                                                                                                let a2376 = v0;
                                                                                                                let a2377 = 41u64;
                                                                                                                if nat_eq(a2376, a2377) {
                                                                                                                    18u64
                                                                                                                } else {
                                                                                                                    let a2378 = v0;
                                                                                                                    let a2379 = 42u64;
                                                                                                                    if nat_eq(a2378, a2379) {
                                                                                                                        18u64
                                                                                                                    } else {
                                                                                                                        let a2380 = v0;
                                                                                                                        let a2381 = 43u64;
                                                                                                                        if nat_eq(a2380, a2381) {
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
    let a2382 = v0;
    let a2383 = 0u64;
    if nat_eq(a2382, a2383) {
        0u64
    } else {
        let a2384 = v0;
        let a2385 = 1u64;
        if nat_eq(a2384, a2385) {
            0u64
        } else {
            let a2386 = v0;
            let a2387 = 2u64;
            if nat_eq(a2386, a2387) {
                2u64
            } else {
                let a2388 = v0;
                let a2389 = 3u64;
                if nat_eq(a2388, a2389) {
                    4u64
                } else {
                    let a2390 = v0;
                    let a2391 = 6u64;
                    if nat_eq(a2390, a2391) {
                        2u64
                    } else {
                        let a2392 = v0;
                        let a2393 = 7u64;
                        if nat_eq(a2392, a2393) {
                            4u64
                        } else {
                            let a2394 = v0;
                            let a2395 = 8u64;
                            if nat_eq(a2394, a2395) {
                                2u64
                            } else {
                                let a2396 = v0;
                                let a2397 = 10u64;
                                if nat_eq(a2396, a2397) {
                                    4u64
                                } else {
                                    let a2398 = v0;
                                    let a2399 = 11u64;
                                    if nat_eq(a2398, a2399) {
                                        2u64
                                    } else {
                                        let a2400 = v0;
                                        let a2401 = 12u64;
                                        if nat_eq(a2400, a2401) {
                                            4u64
                                        } else {
                                            let a2402 = v0;
                                            let a2403 = 13u64;
                                            if nat_eq(a2402, a2403) {
                                                4u64
                                            } else {
                                                let a2404 = v0;
                                                let a2405 = 14u64;
                                                if nat_eq(a2404, a2405) {
                                                    2u64
                                                } else {
                                                    let a2406 = v0;
                                                    let a2407 = 16u64;
                                                    if nat_eq(a2406, a2407) {
                                                        2u64
                                                    } else {
                                                        let a2408 = v0;
                                                        let a2409 = 17u64;
                                                        if nat_eq(a2408, a2409) {
                                                            2u64
                                                        } else {
                                                            let a2410 = v0;
                                                            let a2411 = 18u64;
                                                            if nat_eq(a2410, a2411) {
                                                                2u64
                                                            } else {
                                                                let a2412 = v0;
                                                                let a2413 = 19u64;
                                                                if nat_eq(a2412, a2413) {
                                                                    2u64
                                                                } else {
                                                                    let a2414 = v0;
                                                                    let a2415 = 20u64;
                                                                    if nat_eq(a2414, a2415) {
                                                                        2u64
                                                                    } else {
                                                                        let a2416 = v0;
                                                                        let a2417 = 21u64;
                                                                        if nat_eq(a2416, a2417) {
                                                                            2u64
                                                                        } else {
                                                                            let a2418 = v0;
                                                                            let a2419 = 22u64;
                                                                            if nat_eq(a2418, a2419) {
                                                                                2u64
                                                                            } else {
                                                                                let a2420 = v0;
                                                                                let a2421 = 23u64;
                                                                                if nat_eq(a2420, a2421) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a2422 = v0;
                                                                                    let a2423 = 26u64;
                                                                                    if nat_eq(a2422, a2423) {
                                                                                        0u64
                                                                                    } else {
                                                                                        let a2424 = v0;
                                                                                        let a2425 = 29u64;
                                                                                        if nat_eq(a2424, a2425) {
                                                                                            3u64
                                                                                        } else {
                                                                                            let a2426 = v0;
                                                                                            let a2427 = 30u64;
                                                                                            if nat_eq(a2426, a2427) {
                                                                                                0u64
                                                                                            } else {
                                                                                                let a2428 = v0;
                                                                                                let a2429 = 34u64;
                                                                                                if nat_eq(a2428, a2429) {
                                                                                                    2u64
                                                                                                } else {
                                                                                                    let a2430 = v0;
                                                                                                    let a2431 = 35u64;
                                                                                                    if nat_eq(a2430, a2431) {
                                                                                                        2u64
                                                                                                    } else {
                                                                                                        let a2432 = v0;
                                                                                                        let a2433 = 39u64;
                                                                                                        if nat_eq(a2432, a2433) {
                                                                                                            2u64
                                                                                                        } else {
                                                                                                            let a2434 = v0;
                                                                                                            let a2435 = 40u64;
                                                                                                            if nat_eq(a2434, a2435) {
                                                                                                                0u64
                                                                                                            } else {
                                                                                                                let a2436 = v0;
                                                                                                                let a2437 = 41u64;
                                                                                                                if nat_eq(a2436, a2437) {
                                                                                                                    2u64
                                                                                                                } else {
                                                                                                                    let a2438 = v0;
                                                                                                                    let a2439 = 42u64;
                                                                                                                    if nat_eq(a2438, a2439) {
                                                                                                                        2u64
                                                                                                                    } else {
                                                                                                                        let a2440 = v0;
                                                                                                                        let a2441 = 43u64;
                                                                                                                        let _ = nat_eq(a2440, a2441);
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
    let a2442 = v0;
    let a2443 = 0u64;
    if nat_eq(a2442, a2443) {
        2u64
    } else {
        let a2444 = v0;
        let a2445 = 26u64;
        if nat_eq(a2444, a2445) {
            2u64
        } else {
            let a2446 = v0;
            let a2447 = 1u64;
            if nat_eq(a2446, a2447) {
                3u64
            } else {
                let a2448 = v0;
                let a2449 = 30u64;
                if nat_eq(a2448, a2449) {
                    3u64
                } else {
                    let a2450 = v0;
                    let a2451 = 43u64;
                    if nat_eq(a2450, a2451) {
                        3u64
                    } else {
                        let a2452 = v0;
                        let a2453 = 2u64;
                        if nat_eq(a2452, a2453) {
                            4u64
                        } else {
                            let a2454 = v0;
                            let a2455 = 3u64;
                            if nat_eq(a2454, a2455) {
                                5u64
                            } else {
                                let a2456 = v0;
                                let a2457 = 6u64;
                                if nat_eq(a2456, a2457) {
                                    5u64
                                } else {
                                    let a2458 = v0;
                                    let a2459 = 7u64;
                                    if nat_eq(a2458, a2459) {
                                        6u64
                                    } else {
                                        let a2460 = v0;
                                        let a2461 = 8u64;
                                        if nat_eq(a2460, a2461) {
                                            4u64
                                        } else {
                                            let a2462 = v0;
                                            let a2463 = 41u64;
                                            if nat_eq(a2462, a2463) {
                                                3u64
                                            } else {
                                                let a2464 = v0;
                                                let a2465 = 42u64;
                                                if nat_eq(a2464, a2465) {
                                                    4u64
                                                } else {
                                                    let a2466 = v0;
                                                    let a2467 = 39u64;
                                                    if nat_eq(a2466, a2467) {
                                                        4u64
                                                    } else {
                                                        let a2468 = v0;
                                                        let a2469 = 40u64;
                                                        if nat_eq(a2468, a2469) {
                                                            6u64
                                                        } else {
                                                            let a2470 = v0;
                                                            let a2471 = 10u64;
                                                            if nat_eq(a2470, a2471) {
                                                                10u64
                                                            } else {
                                                                let a2472 = v0;
                                                                let a2473 = 11u64;
                                                                if nat_eq(a2472, a2473) {
                                                                    9u64
                                                                } else {
                                                                    let a2474 = v0;
                                                                    let a2475 = 12u64;
                                                                    if nat_eq(a2474, a2475) {
                                                                        12u64
                                                                    } else {
                                                                        let a2476 = v0;
                                                                        let a2477 = 13u64;
                                                                        if nat_eq(a2476, a2477) {
                                                                            13u64
                                                                        } else {
                                                                            let a2478 = v0;
                                                                            let a2479 = 14u64;
                                                                            if nat_eq(a2478, a2479) {
                                                                                8u64
                                                                            } else {
                                                                                let a2480 = v0;
                                                                                let a2481 = 34u64;
                                                                                if nat_eq(a2480, a2481) {
                                                                                    4u64
                                                                                } else {
                                                                                    let a2482 = v0;
                                                                                    let a2483 = 35u64;
                                                                                    if nat_eq(a2482, a2483) {
                                                                                        4u64
                                                                                    } else {
                                                                                        let a2484 = v0;
                                                                                        let a2485 = 16u64;
                                                                                        if nat_eq(a2484, a2485) {
                                                                                            12u64
                                                                                        } else {
                                                                                            let a2486 = v0;
                                                                                            let a2487 = 17u64;
                                                                                            if nat_eq(a2486, a2487) {
                                                                                                12u64
                                                                                            } else {
                                                                                                let a2488 = v0;
                                                                                                let a2489 = 22u64;
                                                                                                if nat_eq(a2488, a2489) {
                                                                                                    13u64
                                                                                                } else {
                                                                                                    let a2490 = v0;
                                                                                                    let a2491 = 18u64;
                                                                                                    if nat_eq(a2490, a2491) {
                                                                                                        12u64
                                                                                                    } else {
                                                                                                        let a2492 = v0;
                                                                                                        let a2493 = 21u64;
                                                                                                        if nat_eq(a2492, a2493) {
                                                                                                            11u64
                                                                                                        } else {
                                                                                                            let a2494 = v0;
                                                                                                            let a2495 = 19u64;
                                                                                                            if nat_eq(a2494, a2495) {
                                                                                                                9u64
                                                                                                            } else {
                                                                                                                let a2496 = v0;
                                                                                                                let a2497 = 29u64;
                                                                                                                if nat_eq(a2496, a2497) {
                                                                                                                    10u64
                                                                                                                } else {
                                                                                                                    let a2498 = v0;
                                                                                                                    let a2499 = 20u64;
                                                                                                                    if nat_eq(a2498, a2499) {
                                                                                                                        5u64
                                                                                                                    } else {
                                                                                                                        let a2500 = v0;
                                                                                                                        let a2501 = 23u64;
                                                                                                                        if nat_eq(a2500, a2501) {
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

pub fn f9(v0: u64) -> u64 {
    let a2502 = v0;
    let a2503 = 0u64;
    if nat_eq(a2502, a2503) {
        1u64
    } else {
        let a2504 = v0;
        let a2505 = 1u64;
        if nat_eq(a2504, a2505) {
            2u64
        } else {
            let a2506 = v0;
            let a2507 = 2u64;
            if nat_eq(a2506, a2507) {
                4u64
            } else {
                let a2508 = v0;
                let a2509 = 3u64;
                if nat_eq(a2508, a2509) {
                    8u64
                } else {
                    let a2510 = v0;
                    let a2511 = 4u64;
                    if nat_eq(a2510, a2511) {
                        16u64
                    } else {
                        let a2512 = v0;
                        let a2513 = 5u64;
                        if nat_eq(a2512, a2513) {
                            32u64
                        } else {
                            let a2514 = v0;
                            let a2515 = 6u64;
                            if nat_eq(a2514, a2515) {
                                64u64
                            } else {
                                let a2516 = v0;
                                let a2517 = 7u64;
                                if nat_eq(a2516, a2517) {
                                    128u64
                                } else {
                                    let a2518 = v0;
                                    let a2519 = 8u64;
                                    if nat_eq(a2518, a2519) {
                                        256u64
                                    } else {
                                        let a2520 = v0;
                                        let a2521 = 9u64;
                                        if nat_eq(a2520, a2521) {
                                            512u64
                                        } else {
                                            let a2522 = v0;
                                            let a2523 = 10u64;
                                            if nat_eq(a2522, a2523) {
                                                1024u64
                                            } else {
                                                let a2524 = v0;
                                                let a2525 = 11u64;
                                                if nat_eq(a2524, a2525) {
                                                    2048u64
                                                } else {
                                                    let a2526 = v0;
                                                    let a2527 = 12u64;
                                                    if nat_eq(a2526, a2527) {
                                                        4096u64
                                                    } else {
                                                        let a2528 = v0;
                                                        let a2529 = 13u64;
                                                        if nat_eq(a2528, a2529) {
                                                            8192u64
                                                        } else {
                                                            let a2530 = v0;
                                                            let a2531 = 14u64;
                                                            if nat_eq(a2530, a2531) {
                                                                16384u64
                                                            } else {
                                                                let a2532 = v0;
                                                                let a2533 = 15u64;
                                                                if nat_eq(a2532, a2533) {
                                                                    32768u64
                                                                } else {
                                                                    let a2534 = v0;
                                                                    let a2535 = 16u64;
                                                                    if nat_eq(a2534, a2535) {
                                                                        65536u64
                                                                    } else {
                                                                        let a2536 = v0;
                                                                        let a2537 = 17u64;
                                                                        if nat_eq(a2536, a2537) {
                                                                            131072u64
                                                                        } else {
                                                                            let a2538 = v0;
                                                                            let a2539 = 18u64;
                                                                            if nat_eq(a2538, a2539) {
                                                                                262144u64
                                                                            } else {
                                                                                let a2540 = v0;
                                                                                let a2541 = 19u64;
                                                                                if nat_eq(a2540, a2541) {
                                                                                    524288u64
                                                                                } else {
                                                                                    let a2542 = v0;
                                                                                    let a2543 = 20u64;
                                                                                    if nat_eq(a2542, a2543) {
                                                                                        1048576u64
                                                                                    } else {
                                                                                        let a2544 = v0;
                                                                                        let a2545 = 21u64;
                                                                                        if nat_eq(a2544, a2545) {
                                                                                            2097152u64
                                                                                        } else {
                                                                                            let a2546 = v0;
                                                                                            let a2547 = 22u64;
                                                                                            if nat_eq(a2546, a2547) {
                                                                                                4194304u64
                                                                                            } else {
                                                                                                let a2548 = v0;
                                                                                                let a2549 = 23u64;
                                                                                                if nat_eq(a2548, a2549) {
                                                                                                    8388608u64
                                                                                                } else {
                                                                                                    let a2550 = v0;
                                                                                                    let a2551 = 24u64;
                                                                                                    if nat_eq(a2550, a2551) {
                                                                                                        16777216u64
                                                                                                    } else {
                                                                                                        let a2552 = v0;
                                                                                                        let a2553 = 25u64;
                                                                                                        if nat_eq(a2552, a2553) {
                                                                                                            33554432u64
                                                                                                        } else {
                                                                                                            let a2554 = v0;
                                                                                                            let a2555 = 26u64;
                                                                                                            if nat_eq(a2554, a2555) {
                                                                                                                67108864u64
                                                                                                            } else {
                                                                                                                let a2556 = v0;
                                                                                                                let a2557 = 27u64;
                                                                                                                if nat_eq(a2556, a2557) {
                                                                                                                    134217728u64
                                                                                                                } else {
                                                                                                                    let a2558 = v0;
                                                                                                                    let a2559 = 28u64;
                                                                                                                    if nat_eq(a2558, a2559) {
                                                                                                                        268435456u64
                                                                                                                    } else {
                                                                                                                        let a2560 = v0;
                                                                                                                        let a2561 = 29u64;
                                                                                                                        if nat_eq(a2560, a2561) {
                                                                                                                            536870912u64
                                                                                                                        } else {
                                                                                                                            let a2562 = v0;
                                                                                                                            let a2563 = 30u64;
                                                                                                                            if nat_eq(a2562, a2563) {
                                                                                                                                1073741824u64
                                                                                                                            } else {
                                                                                                                                let a2564 = v0;
                                                                                                                                let a2565 = 31u64;
                                                                                                                                if nat_eq(a2564, a2565) {
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

pub fn f10(v0: u64, v1: bool, v2: u64) -> Adt1 {
    let a2566 = v0;
    let a2567 = 4294967296u64;
    if if nat_lt(a2566, a2567) {
        let a2568 = v2;
        let a2569 = 8u64;
        nat_lt(a2568, a2569)
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

pub fn f11(v0: u64) -> bool {
    let a2570 = 2139095040u64;
    let a2571 = f13(v0);
    nat_lt(a2570, a2571)
}

pub fn f12(v0: u64) -> R<u64> {
    let a2580 = v0;
    let a2581 = {
        let a2578 = {
            let a2575 = {
                let a2572 = v0;
                let a2573 = 4194304u64;
                let a2574 = 0u64;
                nat_quot(a2572, a2573, a2574)
            };
            let a2576 = 2u64;
            let a2577 = 0u64;
            nat_rem(a2575, a2576, a2577)
        };
        let a2579 = 0u64;
        if nat_eq(a2578, a2579) {
            4194304u64
        } else {
            0u64
        }
    };
    nat_add(a2580, a2581)
}

pub fn f13(v0: u64) -> u64 {
    let a2582 = v0;
    let a2583 = 2147483648u64;
    let a2584 = 0u64;
    nat_rem(a2582, a2583, a2584)
}

pub fn run(p0: Adt0) -> R<Adt1> {
    f0(p0)
}
