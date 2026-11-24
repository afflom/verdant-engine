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
    C0(u64, u64),
    C1(bool, u64, u64),
    C2(u64, Adt1),
    C3(u64, Adt1),
    C4(u64, Adt1),
    C5(u64, Adt1),
    C6(u64, Adt1),
    C7(u64, Adt1),
    C8(u64, u64, u64, u64),
    C9(u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(u64, u64, u64, u64),
    C2(u64),
    C3(u64, u64),
    C4(u64, u64),
    C5(u64, u64, u64, u64),
    C6(u64, u64, u64),
    C7(bool, u64, u64),
    C8(u64, u64, u64),
    C9(u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt2> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2) => {
            let a2 = v1;
            let a3 = 256u64;
            if if nat_eq(a2, a3) {
                let a4 = v2;
                let a5 = 210u64;
                nat_eq(a4, a5)
            } else {
                false
            } {
                Ok(Adt2::C1(16u64, 19u64, 32u64, 128u64))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C1(v3, v4, v5) => {
            let a6 = v5;
            let a7 = 4294967296u64;
            if if nat_lt(a6, a7) {
                if v3 {
                    let a11 = {
                        let a8 = v5;
                        let a9 = 2147483648u64;
                        let a10 = 0u64;
                        nat_rem(a8, a9, a10)
                    };
                    let a12 = 2139095040u64;
                    nat_lt(a11, a12)
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt2::C2({
                    let a16 = {
                        let a13 = v5;
                        let a14 = 2147483648u64;
                        let a15 = 0u64;
                        nat_rem(a13, a14, a15)
                    };
                    let a17 = 2139095040u64;
                    if if nat_le(a16, a17) {
                        let a24 = {
                            let a18 = v4;
                            let a19 = 2147483648u64;
                            let a20 = 0u64;
                            nat_rem(a18, a19, a20)
                        };
                        let a25 = {
                            let a21 = v5;
                            let a22 = 2147483648u64;
                            let a23 = 0u64;
                            nat_rem(a21, a22, a23)
                        };
                        nat_lt(a24, a25)
                    } else {
                        false
                    } {
                        v5
                    } else {
                        v4
                    }
                }))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C2(v6, v7) => {
            Ok(f1(v6, v7))
        }
        Adt0::C3(v8, v9) => {
            Ok(f2(v8, v9))
        }
        Adt0::C4(v10, v11) => {
            Ok(f3(v10, v11))
        }
        Adt0::C5(v12, v13) => {
            Ok(f4(v12, v13))
        }
        Adt0::C6(v14, v15) => {
            f5(v14, v15)
        }
        Adt0::C7(v16, v17) => {
            Ok(f6(v16, v17))
        }
        Adt0::C8(v18, v19, v20, v21) => {
            let a26 = v18;
            let a27 = 64u64;
            if if if nat_lt(a26, a27) {
                let a28 = v19;
                let a29 = 64u64;
                nat_lt(a28, a29)
            } else {
                false
            } {
                let a30 = v20;
                let a31 = 64u64;
                if nat_lt(a30, a31) {
                    let a32 = v21;
                    let a33 = 64u64;
                    nat_lt(a32, a33)
                } else {
                    false
                }
            } else {
                false
            } {
                f7(v18, v19, v20, v21)
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C9(v22, v23) => {
            let a34 = v23;
            let a35 = 32u64;
            if nat_eq(a34, a35) {
                Ok(Adt2::C2({
                    let a36 = v22;
                    let a37 = 1262485472u64;
                    if nat_le(a36, a37) {
                        0u64
                    } else {
                        let a38 = 1262485535u64;
                        let a39 = v22;
                        if nat_le(a38, a39) {
                            63u64
                        } else {
                            let a45 = {
                                let a43 = {
                                    let a40 = v22;
                                    let a41 = 64u64;
                                    let a42 = 0u64;
                                    nat_rem(a40, a41, a42)
                                };
                                let a44 = 32u64;
                                nat_add(a43, a44)?
                            };
                            let a46 = 64u64;
                            let a47 = 0u64;
                            nat_rem(a45, a46, a47)
                        }
                    }
                }))
            } else {
                let a48 = v23;
                let a49 = 128u64;
                if nat_eq(a48, a49) {
                    Ok(Adt2::C2({
                        let a50 = v22;
                        let a51 = 1262485376u64;
                        if nat_le(a50, a51) {
                            0u64
                        } else {
                            let a52 = 1262485631u64;
                            let a53 = v22;
                            if nat_le(a52, a53) {
                                255u64
                            } else {
                                let a59 = {
                                    let a57 = {
                                        let a54 = v22;
                                        let a55 = 256u64;
                                        let a56 = 0u64;
                                        nat_rem(a54, a55, a56)
                                    };
                                    let a58 = 128u64;
                                    nat_add(a57, a58)?
                                };
                                let a60 = 256u64;
                                let a61 = 0u64;
                                nat_rem(a59, a60, a61)
                            }
                        }
                    }))
                } else {
                    Ok(Adt2::C0)
                }
            }
        }
    }
}

pub fn f1(v0: u64, v1: Adt1) -> Adt2 {
    let a63 = match v1.clone() {
        Adt1::C0(_, h62, _, _, _, _, _, _) => {
            h62
        }
    };
    let a64 = 19u64;
    if nat_lt(a63, a64) {
        let a69 = {
            let a66 = match v1.clone() {
                Adt1::C0(h65, _, _, _, _, _, _, _) => {
                    h65
                }
            };
            let a67 = 2147483648u64;
            let a68 = 0u64;
            nat_rem(a66, a67, a68)
        };
        let a70 = 646978941u64;
        if nat_lt(a69, a70) {
            Adt2::C2(0u64)
        } else {
            let a72 = match v1.clone() {
                Adt1::C0(_, h71, _, _, _, _, _, _) => {
                    h71
                }
            };
            let a73 = 0u64;
            if nat_eq(a72, a73) {
                let a74 = v0;
                let a75 = 1u64;
                if nat_lt(a74, a75) {
                    Adt2::C5(3u64, 3254779904u64, match v1 {
                        Adt1::C0(h76, _, _, _, _, _, _, _) => {
                            h76
                        }
                    }, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h77, _, _, _) => {
                            h77
                        }
                    })
                }
            } else {
                let a78 = v0;
                let a79 = 2u64;
                if nat_lt(a78, a79) {
                    let a80 = v0;
                    let a81 = 1u64;
                    if nat_lt(a80, a81) {
                        Adt2::C6(match v1.clone() {
                            Adt1::C0(_, h82, _, _, _, _, _, _) => {
                                h82
                            }
                        }, {
                            let a84 = match v1 {
                                Adt1::C0(_, h83, _, _, _, _, _, _) => {
                                    h83
                                }
                            };
                            let a85 = 9u64;
                            if nat_le(a84, a85) {
                                10u64
                            } else {
                                9u64
                            }
                        }, 0u64)
                    } else {
                        Adt2::C5(2u64, 1036831949u64, match v1 {
                            Adt1::C0(_, _, _, _, h86, _, _, _) => {
                                h86
                            }
                        }, 0u64)
                    }
                } else {
                    let a87 = v0;
                    let a88 = 3u64;
                    if nat_lt(a87, a88) {
                        Adt2::C5(1u64, 3254779904u64, match v1 {
                            Adt1::C0(_, _, _, _, h89, _, _, _) => {
                                h89
                            }
                        }, 0u64)
                    } else {
                        let a90 = v0;
                        let a91 = 4u64;
                        if nat_lt(a90, a91) {
                            Adt2::C5(3u64, match v1.clone() {
                                Adt1::C0(_, _, _, _, h92, _, _, _) => {
                                    h92
                                }
                            }, match v1 {
                                Adt1::C0(h93, _, _, _, _, _, _, _) => {
                                    h93
                                }
                            }, 0u64)
                        } else {
                            Adt2::C2(match v1 {
                                Adt1::C0(_, _, _, _, h94, _, _, _) => {
                                    h94
                                }
                            })
                        }
                    }
                }
            }
        }
    } else {
        Adt2::C0
    }
}

pub fn f2(v0: u64, v1: Adt1) -> Adt2 {
    let a95 = v0;
    let a96 = 6u64;
    if nat_lt(a95, a96) {
        let a97 = v0;
        let a98 = 3u64;
        if nat_lt(a97, a98) {
            let a99 = v0;
            let a100 = 1u64;
            if nat_lt(a99, a100) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h101, _, _, _, _, _, _) => {
                        h101
                    }
                }, match v1 {
                    Adt1::C0(h102, _, _, _, _, _, _, _) => {
                        h102
                    }
                }, 0u64)
            } else {
                let a103 = v0;
                let a104 = 2u64;
                if nat_lt(a103, a104) {
                    Adt2::C5(0u64, 1262485504u64, match v1 {
                        Adt1::C0(_, _, _, _, h105, _, _, _) => {
                            h105
                        }
                    }, 0u64)
                } else {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h106, _, _, _) => {
                            h106
                        }
                    }, 32u64, 0u64)
                }
            }
        } else {
            let a107 = v0;
            let a108 = 4u64;
            if nat_lt(a107, a108) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h109, _, _, _) => {
                        h109
                    }
                }, 32u64, 0u64)
            } else {
                let a110 = v0;
                let a111 = 5u64;
                if nat_lt(a110, a111) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(h112, _, _, _, _, _, _, _) => {
                            h112
                        }
                    }, match v1 {
                        Adt1::C0(h113, _, _, _, _, _, _, _) => {
                            h113
                        }
                    }, 1u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h114, _, _) => {
                            h114
                        }
                    }, match v1 {
                        Adt1::C0(h115, _, _, _, _, _, _, _) => {
                            h115
                        }
                    }, 2u64)
                }
            }
        }
    } else {
        let a116 = v0;
        let a117 = 9u64;
        if nat_lt(a116, a117) {
            let a118 = v0;
            let a119 = 7u64;
            if nat_lt(a118, a119) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h120, _) => {
                        h120
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h121, _, _, _) => {
                        h121
                    }
                }, 2u64)
            } else {
                let a122 = v0;
                let a123 = 8u64;
                if nat_lt(a122, a123) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, h124, _, _, _, _, _) => {
                            h124
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h125, _) => {
                            h125
                        }
                    }, 2u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h126, _, _) => {
                            h126
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, h127, _, _, _) => {
                            h127
                        }
                    }, 1u64)
                }
            }
        } else {
            let a128 = v0;
            let a129 = 10u64;
            if nat_lt(a128, a129) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h130, _, _) => {
                        h130
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h131, _, _, _) => {
                        h131
                    }
                }, 1u64)
            } else {
                let a132 = v0;
                let a133 = 11u64;
                if nat_lt(a132, a133) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, _, h134, _, _, _, _) => {
                            h134
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h135, _, _) => {
                            h135
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h136, _) => {
                            h136
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h137, _, _) => {
                            h137
                        }
                    })
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1) -> Adt2 {
    let a139 = match v1.clone() {
        Adt1::C0(h138, _, _, _, _, _, _, _) => {
            h138
        }
    };
    let a140 = 1u64;
    if nat_eq(a139, a140) {
        let a145 = {
            let a142 = match v1.clone() {
                Adt1::C0(_, _, h141, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 2147483648u64;
            let a144 = 0u64;
            nat_rem(a142, a143, a144)
        };
        let a146 = 0u64;
        if nat_eq(a145, a146) {
            Adt2::C4(0u64, 0u64)
        } else {
            let a147 = v0;
            let a148 = 1u64;
            if nat_lt(a147, a148) {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(_, h149, _, _, _, _, _, _) => {
                        h149
                    }
                }, match v1 {
                    Adt1::C0(_, _, h150, _, _, _, _, _) => {
                        h150
                    }
                }, 2u64)
            } else {
                let a151 = v0;
                let a152 = 2u64;
                if nat_lt(a151, a152) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h153, _) => {
                            h153
                        }
                    }, match v1 {
                        Adt1::C0(_, h154, _, _, _, _, _, _) => {
                            h154
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h155, _) => {
                            h155
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h156) => {
                            h156
                        }
                    })
                }
            }
        }
    } else {
        let a157 = v0;
        let a158 = 3u64;
        if nat_lt(a157, a158) {
            let a159 = v0;
            let a160 = 1u64;
            if nat_lt(a159, a160) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h161, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1 {
                    Adt1::C0(_, h162, _, _, _, _, _, _) => {
                        h162
                    }
                }, 0u64)
            } else {
                let a163 = v0;
                let a164 = 2u64;
                if nat_lt(a163, a164) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, h165, _, _, _, _) => {
                            h165
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h166, _, _, _, _, _) => {
                            h166
                        }
                    }, 1u64)
                } else {
                    Adt2::C8(match v1.clone() {
                        Adt1::C0(_, _, _, _, h167, _, _, _) => {
                            h167
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h168, _, _) => {
                            h168
                        }
                    }, 0u64)
                }
            }
        } else {
            let a169 = v0;
            let a170 = 5u64;
            if nat_lt(a169, a170) {
                let a171 = v0;
                let a172 = 4u64;
                if nat_lt(a171, a172) {
                    Adt2::C8(match v1 {
                        Adt1::C0(_, _, h173, _, _, _, _, _) => {
                            h173
                        }
                    }, 0u64, 1u64)
                } else {
                    let a175 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h174, _, _, _) => {
                            h174
                        }
                    };
                    let a176 = 2u64;
                    if if nat_eq(a175, a176) {
                        let a178 = match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h177, _, _) => {
                                h177
                            }
                        };
                        let a179 = 2u64;
                        nat_eq(a178, a179)
                    } else {
                        false
                    } {
                        Adt2::C5(3u64, match v1.clone() {
                            Adt1::C0(_, h180, _, _, _, _, _, _) => {
                                h180
                            }
                        }, match v1 {
                            Adt1::C0(_, _, h181, _, _, _, _, _) => {
                                h181
                            }
                        }, 2u64)
                    } else {
                        Adt2::C2(0u64)
                    }
                }
            } else {
                let a182 = v0;
                let a183 = 6u64;
                if nat_lt(a182, a183) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h184, _) => {
                            h184
                        }
                    }, match v1 {
                        Adt1::C0(_, h185, _, _, _, _, _, _) => {
                            h185
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h186, _) => {
                            h186
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h187) => {
                            h187
                        }
                    })
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: Adt1) -> Adt2 {
    let a192 = {
        let a189 = match v1.clone() {
            Adt1::C0(h188, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 2147483648u64;
        let a191 = 0u64;
        nat_rem(a189, a190, a191)
    };
    let a193 = 646978941u64;
    if nat_lt(a192, a193) {
        Adt2::C2(0u64)
    } else {
        let a194 = v0;
        let a195 = 2u64;
        if nat_lt(a194, a195) {
            let a196 = v0;
            let a197 = 1u64;
            if nat_lt(a196, a197) {
                Adt2::C5(3u64, 3271557120u64, match v1 {
                    Adt1::C0(h198, _, _, _, _, _, _, _) => {
                        h198
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h199, _, _, _) => {
                        h199
                    }
                }, 1u64)
            }
        } else {
            let a200 = v0;
            let a201 = 3u64;
            if nat_lt(a200, a201) {
                Adt2::C7(true, match v1 {
                    Adt1::C0(_, _, _, _, _, h202, _, _) => {
                        h202
                    }
                }, 2u64)
            } else {
                let a203 = v0;
                let a204 = 4u64;
                if nat_lt(a203, a204) {
                    Adt2::C7(false, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h205, _) => {
                            h205
                        }
                    }, 1u64)
                } else {
                    Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, h206, _, _, _) => {
                            h206
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h207, _) => {
                            h207
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h208, _, _) => {
                            h208
                        }
                    }, 0u64)
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1) -> R<Adt2> {
    let a209 = v0;
    let a210 = 3u64;
    if nat_lt(a209, a210) {
        let a211 = v0;
        let a212 = 1u64;
        if nat_lt(a211, a212) {
            Ok(Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(h213, _, _, _, _, _, _, _) => {
                    h213
                }
            }, match v1 {
                Adt1::C0(_, _, h214, _, _, _, _, _) => {
                    h214
                }
            }, 0u64))
        } else {
            let a215 = v0;
            let a216 = 2u64;
            if nat_lt(a215, a216) {
                Ok(Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h217, _, _, _) => {
                        h217
                    }
                }, 0u64))
            } else {
                Ok(Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h218, _, _, _) => {
                        h218
                    }
                }, 128u64, 0u64))
            }
        }
    } else {
        let a219 = v0;
        let a220 = 4u64;
        if nat_lt(a219, a220) {
            Ok(Adt2::C6(match v1 {
                Adt1::C0(_, _, _, _, h221, _, _, _) => {
                    h221
                }
            }, 128u64, 1u64))
        } else {
            let a222 = v0;
            let a223 = 5u64;
            if nat_lt(a222, a223) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h224, _, _, _, _, _, _) => {
                        h224
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h225, _, _) => {
                        h225
                    }
                }, 1u64))
            } else {
                Ok(Adt2::C3({
                    let a229 = {
                        let a227 = match v1.clone() {
                            Adt1::C0(_, _, _, _, h226, _, _, _) => {
                                h226
                            }
                        };
                        let a228 = 128u64;
                        nat_add(a227, a228)?
                    };
                    let a230 = 256u64;
                    let a231 = 0u64;
                    nat_rem(a229, a230, a231)
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h232, _, _) => {
                        h232
                    }
                }))
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1) -> Adt2 {
    let a234 = match v1.clone() {
        Adt1::C0(h233, _, _, _, _, _, _, _) => {
            h233
        }
    };
    let a235 = 1u64;
    if if nat_eq(a234, a235) {
        let a240 = {
            let a237 = match v1.clone() {
                Adt1::C0(_, _, h236, _, _, _, _, _) => {
                    h236
                }
            };
            let a238 = 2147483648u64;
            let a239 = 0u64;
            nat_rem(a237, a238, a239)
        };
        let a241 = 0u64;
        nat_eq(a240, a241)
    } else {
        false
    } {
        Adt2::C2(0u64)
    } else {
        let a242 = v0;
        let a243 = 2u64;
        if nat_lt(a242, a243) {
            let a244 = v0;
            let a245 = 1u64;
            if nat_lt(a244, a245) {
                let a247 = match v1.clone() {
                    Adt1::C0(h246, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                let a248 = 1u64;
                if nat_eq(a247, a248) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, h249, _, _, _, _, _) => {
                            h249
                        }
                    }, match v1 {
                        Adt1::C0(_, h250, _, _, _, _, _, _) => {
                            h250
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(3u64, match v1.clone() {
                        Adt1::C0(_, h251, _, _, _, _, _, _) => {
                            h251
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h252, _, _, _, _, _) => {
                            h252
                        }
                    }, 0u64)
                }
            } else {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h253, _, _, _) => {
                        h253
                    }
                }, 0u64)
            }
        } else {
            let a254 = v0;
            let a255 = 3u64;
            if nat_lt(a254, a255) {
                Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h256, _, _, _) => {
                        h256
                    }
                }, 32u64, 0u64)
            } else {
                Adt2::C2(match v1 {
                    Adt1::C0(_, _, _, _, h257, _, _, _) => {
                        h257
                    }
                })
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    Ok(Adt2::C1({
        let a266 = {
            let a258 = v0;
            let a259 = 16u64;
            let a260 = 0u64;
            nat_rem(a258, a259, a260)
        };
        let a267 = {
            let a264 = 16u64;
            let a265 = {
                let a261 = v2;
                let a262 = 16u64;
                let a263 = 0u64;
                nat_rem(a261, a262, a263)
            };
            nat_mul(a264, a265)?
        };
        nat_add(a266, a267)?
    }, {
        let a276 = {
            let a268 = v1;
            let a269 = 16u64;
            let a270 = 0u64;
            nat_rem(a268, a269, a270)
        };
        let a277 = {
            let a274 = 16u64;
            let a275 = {
                let a271 = v3;
                let a272 = 16u64;
                let a273 = 0u64;
                nat_rem(a271, a272, a273)
            };
            nat_mul(a274, a275)?
        };
        nat_add(a276, a277)?
    }, {
        let a300 = {
            let a286 = {
                let a278 = v0;
                let a279 = 16u64;
                let a280 = 0u64;
                nat_quot(a278, a279, a280)
            };
            let a287 = {
                let a284 = 4u64;
                let a285 = {
                    let a281 = v1;
                    let a282 = 16u64;
                    let a283 = 0u64;
                    nat_quot(a281, a282, a283)
                };
                nat_mul(a284, a285)?
            };
            nat_add(a286, a287)?
        };
        let a301 = {
            let a298 = {
                let a291 = 16u64;
                let a292 = {
                    let a288 = v2;
                    let a289 = 16u64;
                    let a290 = 0u64;
                    nat_quot(a288, a289, a290)
                };
                nat_mul(a291, a292)?
            };
            let a299 = {
                let a296 = 64u64;
                let a297 = {
                    let a293 = v3;
                    let a294 = 16u64;
                    let a295 = 0u64;
                    nat_quot(a293, a294, a295)
                };
                nat_mul(a296, a297)?
            };
            nat_add(a298, a299)?
        };
        nat_add(a300, a301)?
    }, 0u64))
}
