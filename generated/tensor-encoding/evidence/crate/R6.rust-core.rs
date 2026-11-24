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
    C10(u64, u64),
    C11(u64, Adt1),
    C12(u64, Adt1),
    C13(u64, Adt1),
    C14(u64, Adt1),
    C15(u64, Adt1),
    C16(u64, Adt1),
    C17(u64, Adt1),
    C18(u64, Adt1),
    C19(u64, Adt1),
    C20(u64, Adt1),
    C21(u64, u64),
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
    C10(u64, u64, u64, u64, u64),
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
        Adt0::C9(v46, v47) => {
            let a38 = v47;
            let a39 = 4u64;
            if nat_eq(a38, a39) {
                Ok(Adt2::C2({
                    let a40 = v46;
                    let a41 = 1262485500u64;
                    if nat_le(a40, a41) {
                        0u64
                    } else {
                        let a42 = 1262485507u64;
                        let a43 = v46;
                        if nat_le(a42, a43) {
                            7u64
                        } else {
                            let a49 = {
                                let a47 = {
                                    let a44 = v46;
                                    let a45 = 8u64;
                                    let a46 = 0u64;
                                    nat_rem(a44, a45, a46)
                                };
                                let a48 = 4u64;
                                nat_add(a47, a48)?
                            };
                            let a50 = 8u64;
                            let a51 = 0u64;
                            nat_rem(a49, a50, a51)
                        }
                    }
                }))
            } else {
                let a52 = v47;
                let a53 = 32u64;
                if nat_eq(a52, a53) {
                    Ok(Adt2::C2({
                        let a54 = v46;
                        let a55 = 1262485472u64;
                        if nat_le(a54, a55) {
                            0u64
                        } else {
                            let a56 = 1262485535u64;
                            let a57 = v46;
                            if nat_le(a56, a57) {
                                63u64
                            } else {
                                let a63 = {
                                    let a61 = {
                                        let a58 = v46;
                                        let a59 = 64u64;
                                        let a60 = 0u64;
                                        nat_rem(a58, a59, a60)
                                    };
                                    let a62 = 32u64;
                                    nat_add(a61, a62)?
                                };
                                let a64 = 64u64;
                                let a65 = 0u64;
                                nat_rem(a63, a64, a65)
                            }
                        }
                    }))
                } else {
                    let a66 = v47;
                    let a67 = 128u64;
                    if nat_eq(a66, a67) {
                        Ok(Adt2::C2({
                            let a68 = v46;
                            let a69 = 1262485376u64;
                            if nat_le(a68, a69) {
                                0u64
                            } else {
                                let a70 = 1262485631u64;
                                let a71 = v46;
                                if nat_le(a70, a71) {
                                    255u64
                                } else {
                                    let a77 = {
                                        let a75 = {
                                            let a72 = v46;
                                            let a73 = 256u64;
                                            let a74 = 0u64;
                                            nat_rem(a72, a73, a74)
                                        };
                                        let a76 = 128u64;
                                        nat_add(a75, a76)?
                                    };
                                    let a78 = 256u64;
                                    let a79 = 0u64;
                                    nat_rem(a77, a78, a79)
                                }
                            }
                        }))
                    } else {
                        Ok(Adt2::C0)
                    }
                }
            }
        }
        Adt0::C10(v22, v23) => {
            let a34 = v22;
            let a35 = 256u64;
            if if nat_eq(a34, a35) {
                let a36 = v23;
                let a37 = 110u64;
                nat_eq(a36, a37)
            } else {
                false
            } {
                Ok(Adt2::C1(16u64, 5u64, 32u64, 64u64))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C11(v24, v25) => {
            Ok(f8(v24, v25))
        }
        Adt0::C12(v26, v27) => {
            Ok(f9(v26, v27))
        }
        Adt0::C13(v28, v29) => {
            Ok(f10(v28, v29))
        }
        Adt0::C14(v30, v31) => {
            Ok(f11(v30, v31))
        }
        Adt0::C15(v32, v33) => {
            Ok(f12(v32, v33))
        }
        Adt0::C16(v34, v35) => {
            Ok(f13(v34, v35))
        }
        Adt0::C17(v36, v37) => {
            Ok(f14(v36, v37))
        }
        Adt0::C18(v38, v39) => {
            Ok(f15(v38, v39))
        }
        Adt0::C19(v40, v41) => {
            Ok(f16(v40, v41))
        }
        Adt0::C20(v42, v43) => {
            Ok(f17(v42, v43))
        }
        Adt0::C21(v44, v45) => {
            f18(v44, v45)
        }
    }
}

pub fn f1(v0: u64, v1: Adt1) -> Adt2 {
    let a81 = match v1.clone() {
        Adt1::C0(_, h80, _, _, _, _, _, _) => {
            h80
        }
    };
    let a82 = 19u64;
    if nat_lt(a81, a82) {
        let a87 = {
            let a84 = match v1.clone() {
                Adt1::C0(h83, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a85 = 2147483648u64;
            let a86 = 0u64;
            nat_rem(a84, a85, a86)
        };
        let a88 = 646978941u64;
        if nat_lt(a87, a88) {
            Adt2::C2(0u64)
        } else {
            let a90 = match v1.clone() {
                Adt1::C0(_, h89, _, _, _, _, _, _) => {
                    h89
                }
            };
            let a91 = 0u64;
            if nat_eq(a90, a91) {
                let a92 = v0;
                let a93 = 1u64;
                if nat_lt(a92, a93) {
                    Adt2::C5(3u64, 3254779904u64, match v1 {
                        Adt1::C0(h94, _, _, _, _, _, _, _) => {
                            h94
                        }
                    }, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h95, _, _, _) => {
                            h95
                        }
                    })
                }
            } else {
                let a96 = v0;
                let a97 = 2u64;
                if nat_lt(a96, a97) {
                    let a98 = v0;
                    let a99 = 1u64;
                    if nat_lt(a98, a99) {
                        Adt2::C6(match v1.clone() {
                            Adt1::C0(_, h100, _, _, _, _, _, _) => {
                                h100
                            }
                        }, {
                            let a102 = match v1 {
                                Adt1::C0(_, h101, _, _, _, _, _, _) => {
                                    h101
                                }
                            };
                            let a103 = 9u64;
                            if nat_le(a102, a103) {
                                10u64
                            } else {
                                9u64
                            }
                        }, 0u64)
                    } else {
                        Adt2::C5(2u64, 1036831949u64, match v1 {
                            Adt1::C0(_, _, _, _, h104, _, _, _) => {
                                h104
                            }
                        }, 0u64)
                    }
                } else {
                    let a105 = v0;
                    let a106 = 3u64;
                    if nat_lt(a105, a106) {
                        Adt2::C5(1u64, 3254779904u64, match v1 {
                            Adt1::C0(_, _, _, _, h107, _, _, _) => {
                                h107
                            }
                        }, 0u64)
                    } else {
                        let a108 = v0;
                        let a109 = 4u64;
                        if nat_lt(a108, a109) {
                            Adt2::C5(3u64, match v1.clone() {
                                Adt1::C0(_, _, _, _, h110, _, _, _) => {
                                    h110
                                }
                            }, match v1 {
                                Adt1::C0(h111, _, _, _, _, _, _, _) => {
                                    h111
                                }
                            }, 0u64)
                        } else {
                            Adt2::C2(match v1 {
                                Adt1::C0(_, _, _, _, h112, _, _, _) => {
                                    h112
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
    let a113 = v0;
    let a114 = 6u64;
    if nat_lt(a113, a114) {
        let a115 = v0;
        let a116 = 3u64;
        if nat_lt(a115, a116) {
            let a117 = v0;
            let a118 = 1u64;
            if nat_lt(a117, a118) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h119, _, _, _, _, _, _) => {
                        h119
                    }
                }, match v1 {
                    Adt1::C0(h120, _, _, _, _, _, _, _) => {
                        h120
                    }
                }, 0u64)
            } else {
                let a121 = v0;
                let a122 = 2u64;
                if nat_lt(a121, a122) {
                    Adt2::C5(0u64, 1262485504u64, match v1 {
                        Adt1::C0(_, _, _, _, h123, _, _, _) => {
                            h123
                        }
                    }, 0u64)
                } else {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h124, _, _, _) => {
                            h124
                        }
                    }, 32u64, 0u64)
                }
            }
        } else {
            let a125 = v0;
            let a126 = 4u64;
            if nat_lt(a125, a126) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h127, _, _, _) => {
                        h127
                    }
                }, 32u64, 0u64)
            } else {
                let a128 = v0;
                let a129 = 5u64;
                if nat_lt(a128, a129) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(h130, _, _, _, _, _, _, _) => {
                            h130
                        }
                    }, match v1 {
                        Adt1::C0(h131, _, _, _, _, _, _, _) => {
                            h131
                        }
                    }, 1u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h132, _, _) => {
                            h132
                        }
                    }, match v1 {
                        Adt1::C0(h133, _, _, _, _, _, _, _) => {
                            h133
                        }
                    }, 2u64)
                }
            }
        }
    } else {
        let a134 = v0;
        let a135 = 9u64;
        if nat_lt(a134, a135) {
            let a136 = v0;
            let a137 = 7u64;
            if nat_lt(a136, a137) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h138, _) => {
                        h138
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h139, _, _, _) => {
                        h139
                    }
                }, 2u64)
            } else {
                let a140 = v0;
                let a141 = 8u64;
                if nat_lt(a140, a141) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, h142, _, _, _, _, _) => {
                            h142
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h143, _) => {
                            h143
                        }
                    }, 2u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h144, _, _) => {
                            h144
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, h145, _, _, _) => {
                            h145
                        }
                    }, 1u64)
                }
            }
        } else {
            let a146 = v0;
            let a147 = 10u64;
            if nat_lt(a146, a147) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h148, _, _) => {
                        h148
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h149, _, _, _) => {
                        h149
                    }
                }, 1u64)
            } else {
                let a150 = v0;
                let a151 = 11u64;
                if nat_lt(a150, a151) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, _, h152, _, _, _, _) => {
                            h152
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h153, _, _) => {
                            h153
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h154, _) => {
                            h154
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h155, _, _) => {
                            h155
                        }
                    })
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1) -> Adt2 {
    let a157 = match v1.clone() {
        Adt1::C0(h156, _, _, _, _, _, _, _) => {
            h156
        }
    };
    let a158 = 1u64;
    if nat_eq(a157, a158) {
        let a163 = {
            let a160 = match v1.clone() {
                Adt1::C0(_, _, h159, _, _, _, _, _) => {
                    h159
                }
            };
            let a161 = 2147483648u64;
            let a162 = 0u64;
            nat_rem(a160, a161, a162)
        };
        let a164 = 0u64;
        if nat_eq(a163, a164) {
            Adt2::C4(0u64, 0u64)
        } else {
            let a165 = v0;
            let a166 = 1u64;
            if nat_lt(a165, a166) {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(_, h167, _, _, _, _, _, _) => {
                        h167
                    }
                }, match v1 {
                    Adt1::C0(_, _, h168, _, _, _, _, _) => {
                        h168
                    }
                }, 2u64)
            } else {
                let a169 = v0;
                let a170 = 2u64;
                if nat_lt(a169, a170) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h171, _) => {
                            h171
                        }
                    }, match v1 {
                        Adt1::C0(_, h172, _, _, _, _, _, _) => {
                            h172
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h173, _) => {
                            h173
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h174) => {
                            h174
                        }
                    })
                }
            }
        }
    } else {
        let a175 = v0;
        let a176 = 3u64;
        if nat_lt(a175, a176) {
            let a177 = v0;
            let a178 = 1u64;
            if nat_lt(a177, a178) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h179, _, _, _, _, _, _) => {
                        h179
                    }
                }, match v1 {
                    Adt1::C0(_, h180, _, _, _, _, _, _) => {
                        h180
                    }
                }, 0u64)
            } else {
                let a181 = v0;
                let a182 = 2u64;
                if nat_lt(a181, a182) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, h183, _, _, _, _) => {
                            h183
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h184, _, _, _, _, _) => {
                            h184
                        }
                    }, 1u64)
                } else {
                    Adt2::C8(match v1.clone() {
                        Adt1::C0(_, _, _, _, h185, _, _, _) => {
                            h185
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h186, _, _) => {
                            h186
                        }
                    }, 0u64)
                }
            }
        } else {
            let a187 = v0;
            let a188 = 5u64;
            if nat_lt(a187, a188) {
                let a189 = v0;
                let a190 = 4u64;
                if nat_lt(a189, a190) {
                    Adt2::C8(match v1 {
                        Adt1::C0(_, _, h191, _, _, _, _, _) => {
                            h191
                        }
                    }, 0u64, 1u64)
                } else {
                    let a193 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h192, _, _, _) => {
                            h192
                        }
                    };
                    let a194 = 2u64;
                    if if nat_eq(a193, a194) {
                        let a196 = match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h195, _, _) => {
                                h195
                            }
                        };
                        let a197 = 2u64;
                        nat_eq(a196, a197)
                    } else {
                        false
                    } {
                        Adt2::C5(3u64, match v1.clone() {
                            Adt1::C0(_, h198, _, _, _, _, _, _) => {
                                h198
                            }
                        }, match v1 {
                            Adt1::C0(_, _, h199, _, _, _, _, _) => {
                                h199
                            }
                        }, 2u64)
                    } else {
                        Adt2::C2(0u64)
                    }
                }
            } else {
                let a200 = v0;
                let a201 = 6u64;
                if nat_lt(a200, a201) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h202, _) => {
                            h202
                        }
                    }, match v1 {
                        Adt1::C0(_, h203, _, _, _, _, _, _) => {
                            h203
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h204, _) => {
                            h204
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h205) => {
                            h205
                        }
                    })
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: Adt1) -> Adt2 {
    let a210 = {
        let a207 = match v1.clone() {
            Adt1::C0(h206, _, _, _, _, _, _, _) => {
                h206
            }
        };
        let a208 = 2147483648u64;
        let a209 = 0u64;
        nat_rem(a207, a208, a209)
    };
    let a211 = 646978941u64;
    if nat_lt(a210, a211) {
        Adt2::C2(0u64)
    } else {
        let a212 = v0;
        let a213 = 2u64;
        if nat_lt(a212, a213) {
            let a214 = v0;
            let a215 = 1u64;
            if nat_lt(a214, a215) {
                Adt2::C5(3u64, 3271557120u64, match v1 {
                    Adt1::C0(h216, _, _, _, _, _, _, _) => {
                        h216
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h217, _, _, _) => {
                        h217
                    }
                }, 1u64)
            }
        } else {
            let a218 = v0;
            let a219 = 3u64;
            if nat_lt(a218, a219) {
                Adt2::C7(true, match v1 {
                    Adt1::C0(_, _, _, _, _, h220, _, _) => {
                        h220
                    }
                }, 2u64)
            } else {
                let a221 = v0;
                let a222 = 4u64;
                if nat_lt(a221, a222) {
                    Adt2::C7(false, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h223, _) => {
                            h223
                        }
                    }, 1u64)
                } else {
                    Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, h224, _, _, _) => {
                            h224
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h225, _) => {
                            h225
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h226, _, _) => {
                            h226
                        }
                    }, 0u64)
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1) -> R<Adt2> {
    let a227 = v0;
    let a228 = 3u64;
    if nat_lt(a227, a228) {
        let a229 = v0;
        let a230 = 1u64;
        if nat_lt(a229, a230) {
            Ok(Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(h231, _, _, _, _, _, _, _) => {
                    h231
                }
            }, match v1 {
                Adt1::C0(_, _, h232, _, _, _, _, _) => {
                    h232
                }
            }, 0u64))
        } else {
            let a233 = v0;
            let a234 = 2u64;
            if nat_lt(a233, a234) {
                Ok(Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h235, _, _, _) => {
                        h235
                    }
                }, 0u64))
            } else {
                Ok(Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h236, _, _, _) => {
                        h236
                    }
                }, 128u64, 0u64))
            }
        }
    } else {
        let a237 = v0;
        let a238 = 4u64;
        if nat_lt(a237, a238) {
            Ok(Adt2::C6(match v1 {
                Adt1::C0(_, _, _, _, h239, _, _, _) => {
                    h239
                }
            }, 128u64, 1u64))
        } else {
            let a240 = v0;
            let a241 = 5u64;
            if nat_lt(a240, a241) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h242, _, _, _, _, _, _) => {
                        h242
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h243, _, _) => {
                        h243
                    }
                }, 1u64))
            } else {
                Ok(Adt2::C3({
                    let a247 = {
                        let a245 = match v1.clone() {
                            Adt1::C0(_, _, _, _, h244, _, _, _) => {
                                h244
                            }
                        };
                        let a246 = 128u64;
                        nat_add(a245, a246)?
                    };
                    let a248 = 256u64;
                    let a249 = 0u64;
                    nat_rem(a247, a248, a249)
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h250, _, _) => {
                        h250
                    }
                }))
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1) -> Adt2 {
    let a252 = match v1.clone() {
        Adt1::C0(h251, _, _, _, _, _, _, _) => {
            h251
        }
    };
    let a253 = 1u64;
    if if nat_eq(a252, a253) {
        let a258 = {
            let a255 = match v1.clone() {
                Adt1::C0(_, _, h254, _, _, _, _, _) => {
                    h254
                }
            };
            let a256 = 2147483648u64;
            let a257 = 0u64;
            nat_rem(a255, a256, a257)
        };
        let a259 = 0u64;
        nat_eq(a258, a259)
    } else {
        false
    } {
        Adt2::C2(0u64)
    } else {
        let a260 = v0;
        let a261 = 2u64;
        if nat_lt(a260, a261) {
            let a262 = v0;
            let a263 = 1u64;
            if nat_lt(a262, a263) {
                let a265 = match v1.clone() {
                    Adt1::C0(h264, _, _, _, _, _, _, _) => {
                        h264
                    }
                };
                let a266 = 1u64;
                if nat_eq(a265, a266) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, h267, _, _, _, _, _) => {
                            h267
                        }
                    }, match v1 {
                        Adt1::C0(_, h268, _, _, _, _, _, _) => {
                            h268
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(3u64, match v1.clone() {
                        Adt1::C0(_, h269, _, _, _, _, _, _) => {
                            h269
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h270, _, _, _, _, _) => {
                            h270
                        }
                    }, 0u64)
                }
            } else {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h271, _, _, _) => {
                        h271
                    }
                }, 0u64)
            }
        } else {
            let a272 = v0;
            let a273 = 3u64;
            if nat_lt(a272, a273) {
                Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h274, _, _, _) => {
                        h274
                    }
                }, 32u64, 0u64)
            } else {
                Adt2::C2(match v1 {
                    Adt1::C0(_, _, _, _, h275, _, _, _) => {
                        h275
                    }
                })
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    Ok(Adt2::C1({
        let a284 = {
            let a276 = v0;
            let a277 = 16u64;
            let a278 = 0u64;
            nat_rem(a276, a277, a278)
        };
        let a285 = {
            let a282 = 16u64;
            let a283 = {
                let a279 = v2;
                let a280 = 16u64;
                let a281 = 0u64;
                nat_rem(a279, a280, a281)
            };
            nat_mul(a282, a283)?
        };
        nat_add(a284, a285)?
    }, {
        let a294 = {
            let a286 = v1;
            let a287 = 16u64;
            let a288 = 0u64;
            nat_rem(a286, a287, a288)
        };
        let a295 = {
            let a292 = 16u64;
            let a293 = {
                let a289 = v3;
                let a290 = 16u64;
                let a291 = 0u64;
                nat_rem(a289, a290, a291)
            };
            nat_mul(a292, a293)?
        };
        nat_add(a294, a295)?
    }, {
        let a318 = {
            let a304 = {
                let a296 = v0;
                let a297 = 16u64;
                let a298 = 0u64;
                nat_quot(a296, a297, a298)
            };
            let a305 = {
                let a302 = 4u64;
                let a303 = {
                    let a299 = v1;
                    let a300 = 16u64;
                    let a301 = 0u64;
                    nat_quot(a299, a300, a301)
                };
                nat_mul(a302, a303)?
            };
            nat_add(a304, a305)?
        };
        let a319 = {
            let a316 = {
                let a309 = 16u64;
                let a310 = {
                    let a306 = v2;
                    let a307 = 16u64;
                    let a308 = 0u64;
                    nat_quot(a306, a307, a308)
                };
                nat_mul(a309, a310)?
            };
            let a317 = {
                let a314 = 64u64;
                let a315 = {
                    let a311 = v3;
                    let a312 = 16u64;
                    let a313 = 0u64;
                    nat_quot(a311, a312, a313)
                };
                nat_mul(a314, a315)?
            };
            nat_add(a316, a317)?
        };
        nat_add(a318, a319)?
    }, 0u64))
}

pub fn f8(v0: u64, v1: Adt1) -> Adt2 {
    let a324 = {
        let a321 = match v1.clone() {
            Adt1::C0(h320, _, _, _, _, _, _, _) => {
                h320
            }
        };
        let a322 = 2147483648u64;
        let a323 = 0u64;
        nat_rem(a321, a322, a323)
    };
    let a325 = 646978941u64;
    if nat_lt(a324, a325) {
        Adt2::C2(0u64)
    } else {
        let a326 = v0;
        let a327 = 1u64;
        if nat_lt(a326, a327) {
            Adt2::C5(3u64, 3229614080u64, match v1 {
                Adt1::C0(h328, _, _, _, _, _, _, _) => {
                    h328
                }
            }, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h329, _, _, _) => {
                    h329
                }
            })
        }
    }
}

pub fn f9(v0: u64, v1: Adt1) -> Adt2 {
    let a330 = v0;
    let a331 = 2u64;
    if nat_lt(a330, a331) {
        let a332 = v0;
        let a333 = 1u64;
        if nat_lt(a332, a333) {
            Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(h334, _, _, _, _, _, _, _) => {
                    h334
                }
            }, match v1 {
                Adt1::C0(_, h335, _, _, _, _, _, _) => {
                    h335
                }
            }, 0u64)
        } else {
            Adt2::C5(0u64, 1262485504u64, match v1 {
                Adt1::C0(_, _, _, _, h336, _, _, _) => {
                    h336
                }
            }, 0u64)
        }
    } else {
        let a337 = v0;
        let a338 = 3u64;
        if nat_lt(a337, a338) {
            Adt2::C9(match v1 {
                Adt1::C0(_, _, _, _, h339, _, _, _) => {
                    h339
                }
            }, 4u64, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h340, _, _, _) => {
                    h340
                }
            })
        }
    }
}

pub fn f10(v0: u64, v1: Adt1) -> Adt2 {
    let a341 = v0;
    let a342 = 2u64;
    if nat_lt(a341, a342) {
        let a343 = v0;
        let a344 = 1u64;
        if nat_lt(a343, a344) {
            Adt2::C5(3u64, match v1.clone() {
                Adt1::C0(h345, _, _, _, _, _, _, _) => {
                    h345
                }
            }, match v1 {
                Adt1::C0(_, h346, _, _, _, _, _, _) => {
                    h346
                }
            }, 0u64)
        } else {
            Adt2::C5(0u64, 1262485504u64, match v1 {
                Adt1::C0(_, _, _, _, h347, _, _, _) => {
                    h347
                }
            }, 0u64)
        }
    } else {
        let a348 = v0;
        let a349 = 3u64;
        if nat_lt(a348, a349) {
            Adt2::C9(match v1 {
                Adt1::C0(_, _, _, _, h350, _, _, _) => {
                    h350
                }
            }, 4u64, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h351, _, _, _) => {
                    h351
                }
            })
        }
    }
}

pub fn f11(v0: u64, v1: Adt1) -> Adt2 {
    f19(v0, v1, 0u64)
}

pub fn f12(v0: u64, v1: Adt1) -> Adt2 {
    f19(v0, v1, 1u64)
}

pub fn f13(v0: u64, v1: Adt1) -> Adt2 {
    if f20(match v1.clone() {
        Adt1::C0(_, h352, _, _, _, _, _, _) => {
            h352
        }
    }) {
        let a353 = v0;
        let a354 = 2u64;
        if nat_lt(a353, a354) {
            let a355 = v0;
            let a356 = 1u64;
            if nat_lt(a355, a356) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h357, _, _, _, _, _, _, _) => {
                        h357
                    }
                }, match v1 {
                    Adt1::C0(_, _, h358, _, _, _, _, _) => {
                        h358
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, h359, _, _, _) => {
                        h359
                    }
                }, match v1 {
                    Adt1::C0(_, h360, _, _, _, _, _, _) => {
                        h360
                    }
                }, 0u64)
            }
        } else {
            let a361 = v0;
            let a362 = 3u64;
            if nat_lt(a361, a362) {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h363, _, _, _) => {
                        h363
                    }
                }, 0u64)
            } else {
                let a364 = v0;
                let a365 = 4u64;
                if nat_lt(a364, a365) {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h366, _, _, _) => {
                            h366
                        }
                    }, 4u64, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h367, _, _, _) => {
                            h367
                        }
                    })
                }
            }
        }
    } else {
        Adt2::C2(match v1 {
            Adt1::C0(_, _, _, h368, _, _, _, _) => {
                h368
            }
        })
    }
}

pub fn f14(v0: u64, v1: Adt1) -> Adt2 {
    if f20(match v1.clone() {
        Adt1::C0(_, h369, _, _, _, _, _, _) => {
            h369
        }
    }) {
        let a370 = v0;
        let a371 = 3u64;
        if nat_lt(a370, a371) {
            let a372 = v0;
            let a373 = 1u64;
            if nat_lt(a372, a373) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h374, _, _, _, _, _, _, _) => {
                        h374
                    }
                }, match v1 {
                    Adt1::C0(h375, _, _, _, _, _, _, _) => {
                        h375
                    }
                }, 0u64)
            } else {
                let a376 = v0;
                let a377 = 2u64;
                if nat_lt(a376, a377) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, h378, _, _, _) => {
                            h378
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, h379, _, _, _, _) => {
                            h379
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, h380, _, _, _, _, _) => {
                            h380
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h381, _, _, _, _, _) => {
                            h381
                        }
                    }, 1u64)
                }
            }
        } else {
            let a382 = v0;
            let a383 = 4u64;
            if nat_lt(a382, a383) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h384, _, _) => {
                        h384
                    }
                }, match v1 {
                    Adt1::C0(_, h385, _, _, _, _, _, _) => {
                        h385
                    }
                }, 1u64)
            } else {
                let a386 = v0;
                let a387 = 5u64;
                if nat_lt(a386, a387) {
                    Adt2::C8(match v1.clone() {
                        Adt1::C0(_, _, _, _, h388, _, _, _) => {
                            h388
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h389, _, _) => {
                            h389
                        }
                    }, 2u64)
                } else {
                    Adt2::C2({
                        let a391 = match v1 {
                            Adt1::C0(_, _, _, _, _, _, h390, _) => {
                                h390
                            }
                        };
                        let a392 = 2u64;
                        if nat_eq(a391, a392) {
                            1u64
                        } else {
                            0u64
                        }
                    })
                }
            }
        }
    } else {
        Adt2::C2(0u64)
    }
}

pub fn f15(v0: u64, v1: Adt1) -> Adt2 {
    if f20(match v1.clone() {
        Adt1::C0(_, h393, _, _, _, _, _, _) => {
            h393
        }
    }) {
        let a394 = v0;
        let a395 = 1u64;
        if nat_lt(a394, a395) {
            Adt2::C5(3u64, match v1.clone() {
                Adt1::C0(h396, _, _, _, _, _, _, _) => {
                    h396
                }
            }, match v1 {
                Adt1::C0(_, h397, _, _, _, _, _, _) => {
                    h397
                }
            }, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h398, _, _, _) => {
                    h398
                }
            })
        }
    } else {
        Adt2::C2(0u64)
    }
}

pub fn f16(v0: u64, v1: Adt1) -> Adt2 {
    let a403 = {
        let a400 = match v1.clone() {
            Adt1::C0(h399, _, _, _, _, _, _, _) => {
                h399
            }
        };
        let a401 = 2147483648u64;
        let a402 = 0u64;
        nat_rem(a400, a401, a402)
    };
    let a404 = 0u64;
    if nat_eq(a403, a404) {
        Adt2::C1(0u64, 0u64, 0u64, 0u64)
    } else {
        let a405 = v0;
        let a406 = 2u64;
        if nat_lt(a405, a406) {
            let a407 = v0;
            let a408 = 1u64;
            if nat_lt(a407, a408) {
                Adt2::C5(3u64, 3254779904u64, match v1 {
                    Adt1::C0(h409, _, _, _, _, _, _, _) => {
                        h409
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h410, _, _, _) => {
                        h410
                    }
                }, 1u64)
            }
        } else {
            let a411 = v0;
            let a412 = 3u64;
            if nat_lt(a411, a412) {
                Adt2::C7(true, match v1 {
                    Adt1::C0(_, _, _, _, _, h413, _, _) => {
                        h413
                    }
                }, 2u64)
            } else {
                let a414 = v0;
                let a415 = 4u64;
                if nat_lt(a414, a415) {
                    Adt2::C7(false, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h416, _) => {
                            h416
                        }
                    }, 1u64)
                } else {
                    Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, h417, _, _, _) => {
                            h417
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h418, _) => {
                            h418
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h419, _, _) => {
                            h419
                        }
                    }, 0u64)
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: Adt1) -> Adt2 {
    let a424 = {
        let a421 = match v1.clone() {
            Adt1::C0(h420, _, _, _, _, _, _, _) => {
                h420
            }
        };
        let a422 = 2147483648u64;
        let a423 = 0u64;
        nat_rem(a421, a422, a423)
    };
    let a425 = 0u64;
    if nat_eq(a424, a425) {
        Adt2::C3(0u64, 0u64)
    } else {
        let a426 = v0;
        let a427 = 3u64;
        if nat_lt(a426, a427) {
            let a428 = v0;
            let a429 = 1u64;
            if nat_lt(a428, a429) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h430, _, _, _, _, _, _, _) => {
                        h430
                    }
                }, match v1 {
                    Adt1::C0(_, _, h431, _, _, _, _, _) => {
                        h431
                    }
                }, 0u64)
            } else {
                let a432 = v0;
                let a433 = 2u64;
                if nat_lt(a432, a433) {
                    Adt2::C5(0u64, 1262485504u64, match v1 {
                        Adt1::C0(_, _, _, _, h434, _, _, _) => {
                            h434
                        }
                    }, 0u64)
                } else {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h435, _, _, _) => {
                            h435
                        }
                    }, 32u64, 0u64)
                }
            }
        } else {
            let a436 = v0;
            let a437 = 4u64;
            if nat_lt(a436, a437) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h438, _, _, _) => {
                        h438
                    }
                }, 32u64, 1u64)
            } else {
                let a439 = v0;
                let a440 = 5u64;
                if nat_lt(a439, a440) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, h441, _, _, _, _, _, _) => {
                            h441
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h442, _, _) => {
                            h442
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, h443, _, _, _) => {
                            h443
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h444, _, _) => {
                            h444
                        }
                    })
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64) -> R<Adt2> {
    let a445 = v0;
    let a446 = 32u64;
    if nat_lt(a445, a446) {
        let a447 = v1;
        let a448 = 8u64;
        if nat_lt(a447, a448) {
            Ok(Adt2::C10(0u64, {
                let a451 = v0;
                let a452 = {
                    let a449 = 32u64;
                    let a450 = v1;
                    nat_mul(a449, a450)?
                };
                nat_add(a451, a452)?
            }, 4u64, 2u64, {
                let a453 = v1;
                let a454 = 4u64;
                if nat_lt(a453, a454) {
                    let a455 = v1;
                    let a456 = 2u64;
                    if nat_lt(a455, a456) {
                        let a457 = v1;
                        let a458 = 0u64;
                        if nat_eq(a457, a458) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a459 = v1;
                        let a460 = 2u64;
                        if nat_eq(a459, a460) {
                            4u64
                        } else {
                            8u64
                        }
                    }
                } else {
                    let a461 = v1;
                    let a462 = 6u64;
                    if nat_lt(a461, a462) {
                        let a463 = v1;
                        let a464 = 4u64;
                        if nat_eq(a463, a464) {
                            16u64
                        } else {
                            32u64
                        }
                    } else {
                        let a465 = v1;
                        let a466 = 6u64;
                        if nat_eq(a465, a466) {
                            64u64
                        } else {
                            128u64
                        }
                    }
                }
            }))
        } else {
            Ok(Adt2::C2(0u64))
        }
    } else {
        let a467 = v0;
        let a468 = 96u64;
        if nat_lt(a467, a468) {
            let a469 = v1;
            let a470 = 4u64;
            if nat_lt(a469, a470) {
                Ok(Adt2::C10(0u64, {
                    let a487 = {
                        let a483 = {
                            let a476 = {
                                let a473 = {
                                    let a471 = v0;
                                    let a472 = 32u64;
                                    nat_sub(a471, a472)
                                };
                                let a474 = 32u64;
                                let a475 = 0u64;
                                nat_quot(a473, a474, a475)
                            };
                            let a477 = 128u64;
                            nat_mul(a476, a477)?
                        };
                        let a484 = {
                            let a480 = {
                                let a478 = v0;
                                let a479 = 32u64;
                                nat_sub(a478, a479)
                            };
                            let a481 = 32u64;
                            let a482 = 0u64;
                            nat_rem(a480, a481, a482)
                        };
                        nat_add(a483, a484)?
                    };
                    let a488 = {
                        let a485 = 32u64;
                        let a486 = v1;
                        nat_mul(a485, a486)?
                    };
                    nat_add(a487, a488)?
                }, 1u64, 4u64, {
                    let a491 = {
                        let a489 = 2u64;
                        let a490 = v1;
                        nat_mul(a489, a490)?
                    };
                    let a492 = 4u64;
                    if nat_lt(a491, a492) {
                        let a495 = {
                            let a493 = 2u64;
                            let a494 = v1;
                            nat_mul(a493, a494)?
                        };
                        let a496 = 2u64;
                        if nat_lt(a495, a496) {
                            let a499 = {
                                let a497 = 2u64;
                                let a498 = v1;
                                nat_mul(a497, a498)?
                            };
                            let a500 = 0u64;
                            if nat_eq(a499, a500) {
                                1u64
                            } else {
                                2u64
                            }
                        } else {
                            let a503 = {
                                let a501 = 2u64;
                                let a502 = v1;
                                nat_mul(a501, a502)?
                            };
                            let a504 = 2u64;
                            if nat_eq(a503, a504) {
                                4u64
                            } else {
                                8u64
                            }
                        }
                    } else {
                        let a507 = {
                            let a505 = 2u64;
                            let a506 = v1;
                            nat_mul(a505, a506)?
                        };
                        let a508 = 6u64;
                        if nat_lt(a507, a508) {
                            let a511 = {
                                let a509 = 2u64;
                                let a510 = v1;
                                nat_mul(a509, a510)?
                            };
                            let a512 = 4u64;
                            if nat_eq(a511, a512) {
                                16u64
                            } else {
                                32u64
                            }
                        } else {
                            let a515 = {
                                let a513 = 2u64;
                                let a514 = v1;
                                nat_mul(a513, a514)?
                            };
                            let a516 = 6u64;
                            if nat_eq(a515, a516) {
                                64u64
                            } else {
                                128u64
                            }
                        }
                    }
                }))
            } else {
                Ok(Adt2::C2(0u64))
            }
        } else {
            let a517 = v0;
            let a518 = 104u64;
            if nat_lt(a517, a518) {
                let a519 = v1;
                let a520 = 2u64;
                if nat_lt(a519, a520) {
                    Ok(Adt2::C10(1u64, {
                        let a525 = {
                            let a521 = v0;
                            let a522 = 96u64;
                            nat_sub(a521, a522)
                        };
                        let a526 = {
                            let a523 = 8u64;
                            let a524 = v1;
                            nat_mul(a523, a524)?
                        };
                        nat_add(a525, a526)?
                    }, 1u64, 16u64, {
                        let a527 = v1;
                        let a528 = 0u64;
                        if nat_eq(a527, a528) {
                            1u64
                        } else {
                            16u64
                        }
                    }))
                } else {
                    Ok(Adt2::C2(0u64))
                }
            } else {
                let a529 = v0;
                let a530 = 108u64;
                if nat_lt(a529, a530) {
                    let a531 = v1;
                    let a532 = 4u64;
                    if nat_lt(a531, a532) {
                        Ok(Adt2::C10(1u64, {
                            let a537 = {
                                let a533 = v0;
                                let a534 = 104u64;
                                nat_sub(a533, a534)
                            };
                            let a538 = {
                                let a535 = 4u64;
                                let a536 = v1;
                                nat_mul(a535, a536)?
                            };
                            nat_add(a537, a538)?
                        }, 16u64, 4u64, {
                            let a541 = {
                                let a539 = 2u64;
                                let a540 = v1;
                                nat_mul(a539, a540)?
                            };
                            let a542 = 4u64;
                            if nat_lt(a541, a542) {
                                let a545 = {
                                    let a543 = 2u64;
                                    let a544 = v1;
                                    nat_mul(a543, a544)?
                                };
                                let a546 = 2u64;
                                if nat_lt(a545, a546) {
                                    let a549 = {
                                        let a547 = 2u64;
                                        let a548 = v1;
                                        nat_mul(a547, a548)?
                                    };
                                    let a550 = 0u64;
                                    if nat_eq(a549, a550) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                } else {
                                    let a553 = {
                                        let a551 = 2u64;
                                        let a552 = v1;
                                        nat_mul(a551, a552)?
                                    };
                                    let a554 = 2u64;
                                    if nat_eq(a553, a554) {
                                        4u64
                                    } else {
                                        8u64
                                    }
                                }
                            } else {
                                let a557 = {
                                    let a555 = 2u64;
                                    let a556 = v1;
                                    nat_mul(a555, a556)?
                                };
                                let a558 = 6u64;
                                if nat_lt(a557, a558) {
                                    let a561 = {
                                        let a559 = 2u64;
                                        let a560 = v1;
                                        nat_mul(a559, a560)?
                                    };
                                    let a562 = 4u64;
                                    if nat_eq(a561, a562) {
                                        16u64
                                    } else {
                                        32u64
                                    }
                                } else {
                                    let a565 = {
                                        let a563 = 2u64;
                                        let a564 = v1;
                                        nat_mul(a563, a564)?
                                    };
                                    let a566 = 6u64;
                                    if nat_eq(a565, a566) {
                                        64u64
                                    } else {
                                        128u64
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(Adt2::C2(0u64))
                    }
                } else {
                    let a567 = v0;
                    let a568 = 110u64;
                    if nat_lt(a567, a568) {
                        let a569 = v1;
                        let a570 = 0u64;
                        if nat_eq(a569, a570) {
                            Ok(Adt2::C10(2u64, 0u64, {
                                let a571 = v0;
                                let a572 = 108u64;
                                if nat_eq(a571, a572) {
                                    1u64
                                } else {
                                    256u64
                                }
                            }, 256u64, 1u64))
                        } else {
                            Ok(Adt2::C2(0u64))
                        }
                    } else {
                        Ok(Adt2::C0)
                    }
                }
            }
        }
    }
}

pub fn f19(v0: u64, v1: Adt1, v2: u64) -> Adt2 {
    let a573 = v0;
    let a574 = 4u64;
    if nat_lt(a573, a574) {
        let a575 = v0;
        let a576 = 2u64;
        if nat_lt(a575, a576) {
            let a577 = v0;
            let a578 = 1u64;
            if nat_lt(a577, a578) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, h579, _, _, _, _, _, _) => {
                        h579
                    }
                }, 4u64, 0u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h580, _, _, _, _, _, _, _) => {
                        h580
                    }
                }, match v1 {
                    Adt1::C0(h581, _, _, _, _, _, _, _) => {
                        h581
                    }
                }, 1u64)
            }
        } else {
            let a582 = v0;
            let a583 = 3u64;
            if nat_lt(a582, a583) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h584, _, _) => {
                        h584
                    }
                }, match v1 {
                    Adt1::C0(h585, _, _, _, _, _, _, _) => {
                        h585
                    }
                }, 2u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h586, _) => {
                        h586
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h587, _, _, _) => {
                        h587
                    }
                }, 2u64)
            }
        }
    } else {
        let a588 = v0;
        let a589 = 6u64;
        if nat_lt(a588, a589) {
            let a590 = v0;
            let a591 = 5u64;
            if nat_lt(a590, a591) {
                Adt2::C5(v2, match v1.clone() {
                    Adt1::C0(_, _, h592, _, _, _, _, _) => {
                        h592
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h593, _) => {
                        h593
                    }
                }, 2u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h594, _, _) => {
                        h594
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h595, _, _, _) => {
                        h595
                    }
                }, 1u64)
            }
        } else {
            let a596 = v0;
            let a597 = 7u64;
            if nat_lt(a596, a597) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h598, _, _) => {
                        h598
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h599, _, _, _) => {
                        h599
                    }
                }, 1u64)
            } else {
                let a600 = v0;
                let a601 = 8u64;
                if nat_lt(a600, a601) {
                    Adt2::C5(v2, match v1.clone() {
                        Adt1::C0(_, _, _, h602, _, _, _, _) => {
                            h602
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h603, _, _) => {
                            h603
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h604, _) => {
                            h604
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h605, _, _) => {
                            h605
                        }
                    })
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> bool {
    let a606 = 0u64;
    let a607 = v0;
    if nat_lt(a606, a607) {
        let a608 = v0;
        let a609 = 2139095040u64;
        nat_le(a608, a609)
    } else {
        false
    }
}
