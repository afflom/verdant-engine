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
    C22(bool, u64, u64),
    C23(u64, Adt1),
    C24(u64, Adt1),
    C25(u64, Adt1),
    C26(u64, Adt1),
    C27(u64, Adt1),
    C28(bool, u64, u64),
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
        Adt0::C0(v17, v18) => {
            let a6 = v17;
            let a7 = 256u64;
            if if nat_eq(a6, a7) {
                let a8 = v18;
                let a9 = 210u64;
                nat_eq(a8, a9)
            } else {
                false
            } {
                Ok(Adt2::C1(16u64, 19u64, 32u64, 128u64))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C1(v19, v20, v21) => {
            let a10 = v21;
            let a11 = 4294967296u64;
            if if nat_lt(a10, a11) {
                if v19 {
                    let a15 = {
                        let a12 = v21;
                        let a13 = 2147483648u64;
                        let a14 = 0u64;
                        nat_rem(a12, a13, a14)
                    };
                    let a16 = 2139095040u64;
                    nat_lt(a15, a16)
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt2::C2({
                    let a20 = {
                        let a17 = v21;
                        let a18 = 2147483648u64;
                        let a19 = 0u64;
                        nat_rem(a17, a18, a19)
                    };
                    let a21 = 2139095040u64;
                    if if nat_le(a20, a21) {
                        let a28 = {
                            let a22 = v20;
                            let a23 = 2147483648u64;
                            let a24 = 0u64;
                            nat_rem(a22, a23, a24)
                        };
                        let a29 = {
                            let a25 = v21;
                            let a26 = 2147483648u64;
                            let a27 = 0u64;
                            nat_rem(a25, a26, a27)
                        };
                        nat_lt(a28, a29)
                    } else {
                        false
                    } {
                        v21
                    } else {
                        v20
                    }
                }))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C2(v22, v23) => {
            Ok(f7(v22, v23))
        }
        Adt0::C3(v24, v25) => {
            Ok(f8(v24, v25))
        }
        Adt0::C4(v26, v27) => {
            Ok(f9(v26, v27))
        }
        Adt0::C5(v28, v29) => {
            Ok(f10(v28, v29))
        }
        Adt0::C6(v30, v31) => {
            f11(v30, v31)
        }
        Adt0::C7(v32, v33) => {
            Ok(f12(v32, v33))
        }
        Adt0::C8(v34, v35, v36, v37) => {
            let a30 = v34;
            let a31 = 64u64;
            if if if nat_lt(a30, a31) {
                let a32 = v35;
                let a33 = 64u64;
                nat_lt(a32, a33)
            } else {
                false
            } {
                let a34 = v36;
                let a35 = 64u64;
                if nat_lt(a34, a35) {
                    let a36 = v37;
                    let a37 = 64u64;
                    nat_lt(a36, a37)
                } else {
                    false
                }
            } else {
                false
            } {
                f13(v34, v35, v36, v37)
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C9(v62, v63) => {
            let a42 = v63;
            let a43 = 4u64;
            if nat_eq(a42, a43) {
                Ok(Adt2::C2({
                    let a44 = v62;
                    let a45 = 1262485500u64;
                    if nat_le(a44, a45) {
                        0u64
                    } else {
                        let a46 = 1262485507u64;
                        let a47 = v62;
                        if nat_le(a46, a47) {
                            7u64
                        } else {
                            let a53 = {
                                let a51 = {
                                    let a48 = v62;
                                    let a49 = 8u64;
                                    let a50 = 0u64;
                                    nat_rem(a48, a49, a50)
                                };
                                let a52 = 4u64;
                                nat_add(a51, a52)?
                            };
                            let a54 = 8u64;
                            let a55 = 0u64;
                            nat_rem(a53, a54, a55)
                        }
                    }
                }))
            } else {
                let a56 = v63;
                let a57 = 32u64;
                if nat_eq(a56, a57) {
                    Ok(Adt2::C2({
                        let a58 = v62;
                        let a59 = 1262485472u64;
                        if nat_le(a58, a59) {
                            0u64
                        } else {
                            let a60 = 1262485535u64;
                            let a61 = v62;
                            if nat_le(a60, a61) {
                                63u64
                            } else {
                                let a67 = {
                                    let a65 = {
                                        let a62 = v62;
                                        let a63 = 64u64;
                                        let a64 = 0u64;
                                        nat_rem(a62, a63, a64)
                                    };
                                    let a66 = 32u64;
                                    nat_add(a65, a66)?
                                };
                                let a68 = 64u64;
                                let a69 = 0u64;
                                nat_rem(a67, a68, a69)
                            }
                        }
                    }))
                } else {
                    let a70 = v63;
                    let a71 = 128u64;
                    if nat_eq(a70, a71) {
                        Ok(Adt2::C2({
                            let a72 = v62;
                            let a73 = 1262485376u64;
                            if nat_le(a72, a73) {
                                0u64
                            } else {
                                let a74 = 1262485631u64;
                                let a75 = v62;
                                if nat_le(a74, a75) {
                                    255u64
                                } else {
                                    let a81 = {
                                        let a79 = {
                                            let a76 = v62;
                                            let a77 = 256u64;
                                            let a78 = 0u64;
                                            nat_rem(a76, a77, a78)
                                        };
                                        let a80 = 128u64;
                                        nat_add(a79, a80)?
                                    };
                                    let a82 = 256u64;
                                    let a83 = 0u64;
                                    nat_rem(a81, a82, a83)
                                }
                            }
                        }))
                    } else {
                        Ok(Adt2::C0)
                    }
                }
            }
        }
        Adt0::C10(v38, v39) => {
            let a38 = v38;
            let a39 = 256u64;
            if if nat_eq(a38, a39) {
                let a40 = v39;
                let a41 = 110u64;
                nat_eq(a40, a41)
            } else {
                false
            } {
                Ok(Adt2::C1(16u64, 5u64, 32u64, 64u64))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C11(v40, v41) => {
            Ok(f14(v40, v41))
        }
        Adt0::C12(v42, v43) => {
            Ok(f15(v42, v43))
        }
        Adt0::C13(v44, v45) => {
            Ok(f16(v44, v45))
        }
        Adt0::C14(v46, v47) => {
            Ok(f17(v46, v47))
        }
        Adt0::C15(v48, v49) => {
            Ok(f18(v48, v49))
        }
        Adt0::C16(v50, v51) => {
            Ok(f19(v50, v51))
        }
        Adt0::C17(v52, v53) => {
            Ok(f20(v52, v53))
        }
        Adt0::C18(v54, v55) => {
            Ok(f21(v54, v55))
        }
        Adt0::C19(v56, v57) => {
            Ok(f22(v56, v57))
        }
        Adt0::C20(v58, v59) => {
            Ok(f23(v58, v59))
        }
        Adt0::C21(v60, v61) => {
            f24(v60, v61)
        }
        Adt0::C22(v1, v2, v3) => {
            let a2 = v2;
            let a3 = if v1 {
                256u64
            } else {
                32u64
            };
            if if nat_eq(a2, a3) {
                let a4 = v3;
                let a5 = if v1 {
                    136u64
                } else {
                    18u64
                };
                nat_eq(a4, a5)
            } else {
                false
            } {
                Ok(Adt2::C1(32u64, if v1 {
                    16u64
                } else {
                    1u64
                }, if v1 {
                    8u64
                } else {
                    1u64
                }, 0u64))
            } else {
                Ok(Adt2::C0)
            }
        }
        Adt0::C23(v4, v5) => {
            Ok(f1(v4, v5))
        }
        Adt0::C24(v6, v7) => {
            f2(v6, v7)
        }
        Adt0::C25(v8, v9) => {
            f3(v8, v9)
        }
        Adt0::C26(v10, v11) => {
            Ok(f4(v10, v11))
        }
        Adt0::C27(v12, v13) => {
            Ok(f5(v12, v13))
        }
        Adt0::C28(v14, v15, v16) => {
            f6(v14, v15, v16)
        }
    }
}

pub fn f1(v0: u64, v1: Adt1) -> Adt2 {
    let a88 = {
        let a85 = match v1.clone() {
            Adt1::C0(h84, _, _, _, _, _, _, _) => {
                h84
            }
        };
        let a86 = 2147483648u64;
        let a87 = 0u64;
        nat_rem(a85, a86, a87)
    };
    let a89 = 646978941u64;
    if nat_lt(a88, a89) {
        Adt2::C2(0u64)
    } else {
        let a91 = match v1.clone() {
            Adt1::C0(_, h90, _, _, _, _, _, _) => {
                h90
            }
        };
        let a92 = 0u64;
        if nat_eq(a91, a92) {
            let a93 = v0;
            let a94 = 1u64;
            if nat_lt(a93, a94) {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(h95, _, _, _, _, _, _, _) => {
                        h95
                    }
                }, {
                    let a97 = match v1 {
                        Adt1::C0(_, _, h96, _, _, _, _, _) => {
                            h96
                        }
                    };
                    let a98 = 1u64;
                    if nat_eq(a97, a98) {
                        1123942400u64
                    } else {
                        3271426048u64
                    }
                }, 0u64)
            } else {
                let a99 = v0;
                let a100 = 2u64;
                if nat_lt(a99, a100) {
                    Adt2::C5(3u64, 1065353216u64, match v1 {
                        Adt1::C0(_, _, _, _, h101, _, _, _) => {
                            h101
                        }
                    }, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h102, _, _, _) => {
                            h102
                        }
                    })
                }
            }
        } else {
            let a103 = v0;
            let a104 = 1u64;
            if nat_lt(a103, a104) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, h105, _, _, _, _, _, _) => {
                        h105
                    }
                }, 135u64, 0u64)
            } else {
                let a106 = v0;
                let a107 = 2u64;
                if nat_lt(a106, a107) {
                    Adt2::C5(3u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, h108, _, _, _) => {
                            h108
                        }
                    }, match v1 {
                        Adt1::C0(h109, _, _, _, _, _, _, _) => {
                            h109
                        }
                    }, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h110, _, _, _) => {
                            h110
                        }
                    })
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1) -> R<Adt2> {
    let a111 = v0;
    let a112 = 4u64;
    if nat_lt(a111, a112) {
        let a113 = v0;
        let a114 = 2u64;
        if nat_lt(a113, a114) {
            let a115 = v0;
            let a116 = 1u64;
            if nat_lt(a115, a116) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h117, _, _, _, _, _, _, _) => {
                        h117
                    }
                }, match v1 {
                    Adt1::C0(_, h118, _, _, _, _, _, _) => {
                        h118
                    }
                }, 2u64))
            } else {
                Ok(Adt2::C6(f25(f26(match v1 {
                    Adt1::C0(_, _, _, _, _, _, h119, _) => {
                        h119
                    }
                })?), 128u64, 0u64))
            }
        } else {
            let a120 = v0;
            let a121 = 3u64;
            if nat_lt(a120, a121) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h122, _, _, _, _, _, _, _) => {
                        h122
                    }
                }, match v1 {
                    Adt1::C0(h123, _, _, _, _, _, _, _) => {
                        h123
                    }
                }, 1u64))
            } else {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h124, _, _) => {
                        h124
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h125, _, _, _) => {
                        h125
                    }
                }, 1u64))
            }
        }
    } else {
        let a126 = v0;
        let a127 = 6u64;
        if nat_lt(a126, a127) {
            let a128 = v0;
            let a129 = 5u64;
            if nat_lt(a128, a129) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h130, _, _) => {
                        h130
                    }
                }, match v1 {
                    Adt1::C0(h131, _, _, _, _, _, _, _) => {
                        h131
                    }
                }, 3u64))
            } else {
                Ok(Adt2::C5(0u64, match v1.clone() {
                    Adt1::C0(_, _, h132, _, _, _, _, _) => {
                        h132
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, h133) => {
                        h133
                    }
                }, 3u64))
            }
        } else {
            let a134 = v0;
            let a135 = 7u64;
            if nat_lt(a134, a135) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h136, _, _) => {
                        h136
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h137, _, _, _) => {
                        h137
                    }
                }, 1u64))
            } else {
                let a138 = v0;
                let a139 = 8u64;
                if nat_lt(a138, a139) {
                    Ok(Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, _, h140, _, _, _, _) => {
                            h140
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h141, _, _) => {
                            h141
                        }
                    }, 1u64))
                } else {
                    Ok(Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h142) => {
                            h142
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h143, _, _) => {
                            h143
                        }
                    }, f26(match v1 {
                        Adt1::C0(_, _, _, _, _, _, h144, _) => {
                            h144
                        }
                    })?, 0u64))
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1) -> R<Adt2> {
    let a145 = v0;
    let a146 = 1u64;
    if nat_lt(a145, a146) {
        Ok(Adt2::C5(2u64, match v1.clone() {
            Adt1::C0(h147, _, _, _, _, _, _, _) => {
                h147
            }
        }, match v1 {
            Adt1::C0(_, h148, _, _, _, _, _, _) => {
                h148
            }
        }, 0u64))
    } else {
        Ok(Adt2::C2(f26(match v1 {
            Adt1::C0(_, _, _, _, h149, _, _, _) => {
                h149
            }
        })?))
    }
}

pub fn f4(v0: u64, v1: Adt1) -> Adt2 {
    let a150 = v0;
    let a151 = 2u64;
    if nat_lt(a150, a151) {
        let a152 = v0;
        let a153 = 1u64;
        if nat_lt(a152, a153) {
            Adt2::C5(3u64, match v1 {
                Adt1::C0(h154, _, _, _, _, _, _, _) => {
                    h154
                }
            }, 3254779904u64, 0u64)
        } else {
            Adt2::C7(true, match v1 {
                Adt1::C0(_, _, _, _, h155, _, _, _) => {
                    h155
                }
            }, 1u64)
        }
    } else {
        let a156 = v0;
        let a157 = 3u64;
        if nat_lt(a156, a157) {
            let a162 = {
                let a159 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h158, _, _, _) => {
                        h158
                    }
                };
                let a160 = 2147483648u64;
                let a161 = 0u64;
                nat_rem(a159, a160, a161)
            };
            let a163 = 0u64;
            if nat_eq(a162, a163) {
                Adt2::C1(match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h164, _, _) => {
                        h164
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h165, _, _, _) => {
                        h165
                    }
                }, 0u64, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h166, _, _, _) => {
                        h166
                    }
                }, 2u64)
            }
        } else {
            Adt2::C1(match v1.clone() {
                Adt1::C0(_, _, _, _, _, h167, _, _) => {
                    h167
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h168, _, _, _) => {
                    h168
                }
            }, match v1 {
                Adt1::C0(_, _, _, _, _, _, h169, _) => {
                    h169
                }
            }, 0u64)
        }
    }
}

pub fn f5(v0: u64, v1: Adt1) -> Adt2 {
    let a170 = v0;
    let a171 = 3u64;
    if nat_lt(a170, a171) {
        let a172 = v0;
        let a173 = 1u64;
        if nat_lt(a172, a173) {
            Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(_, h174, _, _, _, _, _, _) => {
                    h174
                }
            }, match v1 {
                Adt1::C0(_, _, h175, _, _, _, _, _) => {
                    h175
                }
            }, 0u64)
        } else {
            let a176 = v0;
            let a177 = 2u64;
            if nat_lt(a176, a177) {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h178, _, _, _) => {
                        h178
                    }
                }, 0u64)
            } else {
                Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h179, _, _, _) => {
                        h179
                    }
                }, 32u64, 0u64)
            }
        }
    } else {
        let a180 = v0;
        let a181 = 5u64;
        if nat_lt(a180, a181) {
            let a182 = v0;
            let a183 = 4u64;
            if nat_lt(a182, a183) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h184, _, _, _) => {
                        h184
                    }
                }, 32u64, 1u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h185, _, _, _, _, _, _, _) => {
                        h185
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h186, _, _) => {
                        h186
                    }
                }, 1u64)
            }
        } else {
            let a187 = v0;
            let a188 = 6u64;
            if nat_lt(a187, a188) {
                let a193 = {
                    let a190 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h189, _, _) => {
                            h189
                        }
                    };
                    let a191 = 2147483648u64;
                    let a192 = 0u64;
                    nat_rem(a190, a191, a192)
                };
                let a194 = 0u64;
                if nat_eq(a193, a194) {
                    Adt2::C3(match v1 {
                        Adt1::C0(_, _, _, _, h195, _, _, _) => {
                            h195
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(3u64, 1065353216u64, match v1 {
                        Adt1::C0(_, _, _, _, _, h196, _, _) => {
                            h196
                        }
                    }, 1u64)
                }
            } else {
                Adt2::C3(match v1.clone() {
                    Adt1::C0(_, _, _, _, h197, _, _, _) => {
                        h197
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h198, _, _) => {
                        h198
                    }
                })
            }
        }
    }
}

pub fn f6(v0: bool, v1: u64, v2: u64) -> R<Adt2> {
    let a199 = v1;
    let a200 = 2u64;
    if nat_lt(a199, a200) {
        let a201 = v2;
        let a202 = 0u64;
        if nat_eq(a201, a202) {
            Ok(Adt2::C10(2u64, 0u64, {
                let a203 = v1;
                let a204 = 0u64;
                if nat_eq(a203, a204) {
                    1u64
                } else {
                    256u64
                }
            }, 256u64, 1u64))
        } else {
            Ok(Adt2::C2(0u64))
        }
    } else {
        if if v0 {
            let a205 = v1;
            let a206 = 4u64;
            nat_lt(a205, a206)
        } else {
            false
        } {
            let a207 = v2;
            let a208 = 4u64;
            if nat_lt(a207, a208) {
                Ok(Adt2::C10(1u64, {
                    let a213 = {
                        let a211 = 4u64;
                        let a212 = {
                            let a209 = v1;
                            let a210 = 2u64;
                            nat_sub(a209, a210)
                        };
                        nat_mul(a211, a212)?
                    };
                    let a214 = v2;
                    nat_add(a213, a214)?
                }, 16u64, 4u64, {
                    let a215 = v2;
                    let a216 = 2u64;
                    if nat_lt(a215, a216) {
                        let a217 = v2;
                        let a218 = 0u64;
                        if nat_eq(a217, a218) {
                            1u64
                        } else {
                            4u64
                        }
                    } else {
                        let a219 = v2;
                        let a220 = 2u64;
                        if nat_eq(a219, a220) {
                            16u64
                        } else {
                            64u64
                        }
                    }
                }))
            } else {
                Ok(Adt2::C2(0u64))
            }
        } else {
            if if v0 {
                let a221 = v1;
                let a222 = 8u64;
                nat_lt(a221, a222)
            } else {
                false
            } {
                let a223 = v2;
                let a224 = 2u64;
                if nat_lt(a223, a224) {
                    Ok(Adt2::C10(1u64, {
                        let a229 = {
                            let a227 = 2u64;
                            let a228 = {
                                let a225 = v1;
                                let a226 = 4u64;
                                nat_sub(a225, a226)
                            };
                            nat_mul(a227, a228)?
                        };
                        let a230 = v2;
                        nat_add(a229, a230)?
                    }, 1u64, 16u64, {
                        let a231 = v2;
                        let a232 = 0u64;
                        if nat_eq(a231, a232) {
                            1u64
                        } else {
                            16u64
                        }
                    }))
                } else {
                    Ok(Adt2::C2(0u64))
                }
            } else {
                let a233 = v1;
                let a234 = if v0 {
                    136u64
                } else {
                    18u64
                };
                if nat_lt(a233, a234) {
                    let a235 = v2;
                    let a236 = 2u64;
                    if nat_lt(a235, a236) {
                        Ok(Adt2::C10(0u64, {
                            let a253 = {
                                let a249 = {
                                    let a242 = 32u64;
                                    let a243 = {
                                        let a239 = {
                                            let a237 = v1;
                                            let a238 = if v0 {
                                                8u64
                                            } else {
                                                2u64
                                            };
                                            nat_sub(a237, a238)
                                        };
                                        let a240 = 16u64;
                                        let a241 = 0u64;
                                        nat_quot(a239, a240, a241)
                                    };
                                    nat_mul(a242, a243)?
                                };
                                let a250 = {
                                    let a246 = {
                                        let a244 = v1;
                                        let a245 = if v0 {
                                            8u64
                                        } else {
                                            2u64
                                        };
                                        nat_sub(a244, a245)
                                    };
                                    let a247 = 16u64;
                                    let a248 = 0u64;
                                    nat_rem(a246, a247, a248)
                                };
                                nat_add(a249, a250)?
                            };
                            let a254 = {
                                let a251 = 16u64;
                                let a252 = v2;
                                nat_mul(a251, a252)?
                            };
                            nat_add(a253, a254)?
                        }, 1u64, 16u64, {
                            let a255 = v2;
                            let a256 = 0u64;
                            if nat_eq(a255, a256) {
                                1u64
                            } else {
                                16u64
                            }
                        }))
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

pub fn f7(v0: u64, v1: Adt1) -> Adt2 {
    let a258 = match v1.clone() {
        Adt1::C0(_, h257, _, _, _, _, _, _) => {
            h257
        }
    };
    let a259 = 19u64;
    if nat_lt(a258, a259) {
        let a264 = {
            let a261 = match v1.clone() {
                Adt1::C0(h260, _, _, _, _, _, _, _) => {
                    h260
                }
            };
            let a262 = 2147483648u64;
            let a263 = 0u64;
            nat_rem(a261, a262, a263)
        };
        let a265 = 646978941u64;
        if nat_lt(a264, a265) {
            Adt2::C2(0u64)
        } else {
            let a267 = match v1.clone() {
                Adt1::C0(_, h266, _, _, _, _, _, _) => {
                    h266
                }
            };
            let a268 = 0u64;
            if nat_eq(a267, a268) {
                let a269 = v0;
                let a270 = 1u64;
                if nat_lt(a269, a270) {
                    Adt2::C5(3u64, 3254779904u64, match v1 {
                        Adt1::C0(h271, _, _, _, _, _, _, _) => {
                            h271
                        }
                    }, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h272, _, _, _) => {
                            h272
                        }
                    })
                }
            } else {
                let a273 = v0;
                let a274 = 2u64;
                if nat_lt(a273, a274) {
                    let a275 = v0;
                    let a276 = 1u64;
                    if nat_lt(a275, a276) {
                        Adt2::C6(match v1.clone() {
                            Adt1::C0(_, h277, _, _, _, _, _, _) => {
                                h277
                            }
                        }, {
                            let a279 = match v1 {
                                Adt1::C0(_, h278, _, _, _, _, _, _) => {
                                    h278
                                }
                            };
                            let a280 = 9u64;
                            if nat_le(a279, a280) {
                                10u64
                            } else {
                                9u64
                            }
                        }, 0u64)
                    } else {
                        Adt2::C5(2u64, 1036831949u64, match v1 {
                            Adt1::C0(_, _, _, _, h281, _, _, _) => {
                                h281
                            }
                        }, 0u64)
                    }
                } else {
                    let a282 = v0;
                    let a283 = 3u64;
                    if nat_lt(a282, a283) {
                        Adt2::C5(1u64, 3254779904u64, match v1 {
                            Adt1::C0(_, _, _, _, h284, _, _, _) => {
                                h284
                            }
                        }, 0u64)
                    } else {
                        let a285 = v0;
                        let a286 = 4u64;
                        if nat_lt(a285, a286) {
                            Adt2::C5(3u64, match v1.clone() {
                                Adt1::C0(_, _, _, _, h287, _, _, _) => {
                                    h287
                                }
                            }, match v1 {
                                Adt1::C0(h288, _, _, _, _, _, _, _) => {
                                    h288
                                }
                            }, 0u64)
                        } else {
                            Adt2::C2(match v1 {
                                Adt1::C0(_, _, _, _, h289, _, _, _) => {
                                    h289
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

pub fn f8(v0: u64, v1: Adt1) -> Adt2 {
    let a290 = v0;
    let a291 = 6u64;
    if nat_lt(a290, a291) {
        let a292 = v0;
        let a293 = 3u64;
        if nat_lt(a292, a293) {
            let a294 = v0;
            let a295 = 1u64;
            if nat_lt(a294, a295) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h296, _, _, _, _, _, _) => {
                        h296
                    }
                }, match v1 {
                    Adt1::C0(h297, _, _, _, _, _, _, _) => {
                        h297
                    }
                }, 0u64)
            } else {
                let a298 = v0;
                let a299 = 2u64;
                if nat_lt(a298, a299) {
                    Adt2::C5(0u64, 1262485504u64, match v1 {
                        Adt1::C0(_, _, _, _, h300, _, _, _) => {
                            h300
                        }
                    }, 0u64)
                } else {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h301, _, _, _) => {
                            h301
                        }
                    }, 32u64, 0u64)
                }
            }
        } else {
            let a302 = v0;
            let a303 = 4u64;
            if nat_lt(a302, a303) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h304, _, _, _) => {
                        h304
                    }
                }, 32u64, 0u64)
            } else {
                let a305 = v0;
                let a306 = 5u64;
                if nat_lt(a305, a306) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(h307, _, _, _, _, _, _, _) => {
                            h307
                        }
                    }, match v1 {
                        Adt1::C0(h308, _, _, _, _, _, _, _) => {
                            h308
                        }
                    }, 1u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h309, _, _) => {
                            h309
                        }
                    }, match v1 {
                        Adt1::C0(h310, _, _, _, _, _, _, _) => {
                            h310
                        }
                    }, 2u64)
                }
            }
        }
    } else {
        let a311 = v0;
        let a312 = 9u64;
        if nat_lt(a311, a312) {
            let a313 = v0;
            let a314 = 7u64;
            if nat_lt(a313, a314) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h315, _) => {
                        h315
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h316, _, _, _) => {
                        h316
                    }
                }, 2u64)
            } else {
                let a317 = v0;
                let a318 = 8u64;
                if nat_lt(a317, a318) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, h319, _, _, _, _, _) => {
                            h319
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h320, _) => {
                            h320
                        }
                    }, 2u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, h321, _, _) => {
                            h321
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, h322, _, _, _) => {
                            h322
                        }
                    }, 1u64)
                }
            }
        } else {
            let a323 = v0;
            let a324 = 10u64;
            if nat_lt(a323, a324) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h325, _, _) => {
                        h325
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h326, _, _, _) => {
                        h326
                    }
                }, 1u64)
            } else {
                let a327 = v0;
                let a328 = 11u64;
                if nat_lt(a327, a328) {
                    Adt2::C5(0u64, match v1.clone() {
                        Adt1::C0(_, _, _, h329, _, _, _, _) => {
                            h329
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h330, _, _) => {
                            h330
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h331, _) => {
                            h331
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h332, _, _) => {
                            h332
                        }
                    })
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: Adt1) -> Adt2 {
    let a334 = match v1.clone() {
        Adt1::C0(h333, _, _, _, _, _, _, _) => {
            h333
        }
    };
    let a335 = 1u64;
    if nat_eq(a334, a335) {
        let a340 = {
            let a337 = match v1.clone() {
                Adt1::C0(_, _, h336, _, _, _, _, _) => {
                    h336
                }
            };
            let a338 = 2147483648u64;
            let a339 = 0u64;
            nat_rem(a337, a338, a339)
        };
        let a341 = 0u64;
        if nat_eq(a340, a341) {
            Adt2::C4(0u64, 0u64)
        } else {
            let a342 = v0;
            let a343 = 1u64;
            if nat_lt(a342, a343) {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(_, h344, _, _, _, _, _, _) => {
                        h344
                    }
                }, match v1 {
                    Adt1::C0(_, _, h345, _, _, _, _, _) => {
                        h345
                    }
                }, 2u64)
            } else {
                let a346 = v0;
                let a347 = 2u64;
                if nat_lt(a346, a347) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h348, _) => {
                            h348
                        }
                    }, match v1 {
                        Adt1::C0(_, h349, _, _, _, _, _, _) => {
                            h349
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h350, _) => {
                            h350
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h351) => {
                            h351
                        }
                    })
                }
            }
        }
    } else {
        let a352 = v0;
        let a353 = 3u64;
        if nat_lt(a352, a353) {
            let a354 = v0;
            let a355 = 1u64;
            if nat_lt(a354, a355) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h356, _, _, _, _, _, _) => {
                        h356
                    }
                }, match v1 {
                    Adt1::C0(_, h357, _, _, _, _, _, _) => {
                        h357
                    }
                }, 0u64)
            } else {
                let a358 = v0;
                let a359 = 2u64;
                if nat_lt(a358, a359) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, h360, _, _, _, _) => {
                            h360
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h361, _, _, _, _, _) => {
                            h361
                        }
                    }, 1u64)
                } else {
                    Adt2::C8(match v1.clone() {
                        Adt1::C0(_, _, _, _, h362, _, _, _) => {
                            h362
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h363, _, _) => {
                            h363
                        }
                    }, 0u64)
                }
            }
        } else {
            let a364 = v0;
            let a365 = 5u64;
            if nat_lt(a364, a365) {
                let a366 = v0;
                let a367 = 4u64;
                if nat_lt(a366, a367) {
                    Adt2::C8(match v1 {
                        Adt1::C0(_, _, h368, _, _, _, _, _) => {
                            h368
                        }
                    }, 0u64, 1u64)
                } else {
                    let a370 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h369, _, _, _) => {
                            h369
                        }
                    };
                    let a371 = 2u64;
                    if if nat_eq(a370, a371) {
                        let a373 = match v1.clone() {
                            Adt1::C0(_, _, _, _, _, h372, _, _) => {
                                h372
                            }
                        };
                        let a374 = 2u64;
                        nat_eq(a373, a374)
                    } else {
                        false
                    } {
                        Adt2::C5(3u64, match v1.clone() {
                            Adt1::C0(_, h375, _, _, _, _, _, _) => {
                                h375
                            }
                        }, match v1 {
                            Adt1::C0(_, _, h376, _, _, _, _, _) => {
                                h376
                            }
                        }, 2u64)
                    } else {
                        Adt2::C2(0u64)
                    }
                }
            } else {
                let a377 = v0;
                let a378 = 6u64;
                if nat_lt(a377, a378) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h379, _) => {
                            h379
                        }
                    }, match v1 {
                        Adt1::C0(_, h380, _, _, _, _, _, _) => {
                            h380
                        }
                    }, 3u64)
                } else {
                    Adt2::C4(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h381, _) => {
                            h381
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, _, _, h382) => {
                            h382
                        }
                    })
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: Adt1) -> Adt2 {
    let a387 = {
        let a384 = match v1.clone() {
            Adt1::C0(h383, _, _, _, _, _, _, _) => {
                h383
            }
        };
        let a385 = 2147483648u64;
        let a386 = 0u64;
        nat_rem(a384, a385, a386)
    };
    let a388 = 646978941u64;
    if nat_lt(a387, a388) {
        Adt2::C2(0u64)
    } else {
        let a389 = v0;
        let a390 = 2u64;
        if nat_lt(a389, a390) {
            let a391 = v0;
            let a392 = 1u64;
            if nat_lt(a391, a392) {
                Adt2::C5(3u64, 3271557120u64, match v1 {
                    Adt1::C0(h393, _, _, _, _, _, _, _) => {
                        h393
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h394, _, _, _) => {
                        h394
                    }
                }, 1u64)
            }
        } else {
            let a395 = v0;
            let a396 = 3u64;
            if nat_lt(a395, a396) {
                Adt2::C7(true, match v1 {
                    Adt1::C0(_, _, _, _, _, h397, _, _) => {
                        h397
                    }
                }, 2u64)
            } else {
                let a398 = v0;
                let a399 = 4u64;
                if nat_lt(a398, a399) {
                    Adt2::C7(false, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h400, _) => {
                            h400
                        }
                    }, 1u64)
                } else {
                    Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, h401, _, _, _) => {
                            h401
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h402, _) => {
                            h402
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h403, _, _) => {
                            h403
                        }
                    }, 0u64)
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: Adt1) -> R<Adt2> {
    let a404 = v0;
    let a405 = 3u64;
    if nat_lt(a404, a405) {
        let a406 = v0;
        let a407 = 1u64;
        if nat_lt(a406, a407) {
            Ok(Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(h408, _, _, _, _, _, _, _) => {
                    h408
                }
            }, match v1 {
                Adt1::C0(_, _, h409, _, _, _, _, _) => {
                    h409
                }
            }, 0u64))
        } else {
            let a410 = v0;
            let a411 = 2u64;
            if nat_lt(a410, a411) {
                Ok(Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h412, _, _, _) => {
                        h412
                    }
                }, 0u64))
            } else {
                Ok(Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h413, _, _, _) => {
                        h413
                    }
                }, 128u64, 0u64))
            }
        }
    } else {
        let a414 = v0;
        let a415 = 4u64;
        if nat_lt(a414, a415) {
            Ok(Adt2::C6(match v1 {
                Adt1::C0(_, _, _, _, h416, _, _, _) => {
                    h416
                }
            }, 128u64, 1u64))
        } else {
            let a417 = v0;
            let a418 = 5u64;
            if nat_lt(a417, a418) {
                Ok(Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, h419, _, _, _, _, _, _) => {
                        h419
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h420, _, _) => {
                        h420
                    }
                }, 1u64))
            } else {
                Ok(Adt2::C3({
                    let a424 = {
                        let a422 = match v1.clone() {
                            Adt1::C0(_, _, _, _, h421, _, _, _) => {
                                h421
                            }
                        };
                        let a423 = 128u64;
                        nat_add(a422, a423)?
                    };
                    let a425 = 256u64;
                    let a426 = 0u64;
                    nat_rem(a424, a425, a426)
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, h427, _, _) => {
                        h427
                    }
                }))
            }
        }
    }
}

pub fn f12(v0: u64, v1: Adt1) -> Adt2 {
    let a429 = match v1.clone() {
        Adt1::C0(h428, _, _, _, _, _, _, _) => {
            h428
        }
    };
    let a430 = 1u64;
    if if nat_eq(a429, a430) {
        let a435 = {
            let a432 = match v1.clone() {
                Adt1::C0(_, _, h431, _, _, _, _, _) => {
                    h431
                }
            };
            let a433 = 2147483648u64;
            let a434 = 0u64;
            nat_rem(a432, a433, a434)
        };
        let a436 = 0u64;
        nat_eq(a435, a436)
    } else {
        false
    } {
        Adt2::C2(0u64)
    } else {
        let a437 = v0;
        let a438 = 2u64;
        if nat_lt(a437, a438) {
            let a439 = v0;
            let a440 = 1u64;
            if nat_lt(a439, a440) {
                let a442 = match v1.clone() {
                    Adt1::C0(h441, _, _, _, _, _, _, _) => {
                        h441
                    }
                };
                let a443 = 1u64;
                if nat_eq(a442, a443) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, h444, _, _, _, _, _) => {
                            h444
                        }
                    }, match v1 {
                        Adt1::C0(_, h445, _, _, _, _, _, _) => {
                            h445
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(3u64, match v1.clone() {
                        Adt1::C0(_, h446, _, _, _, _, _, _) => {
                            h446
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h447, _, _, _, _, _) => {
                            h447
                        }
                    }, 0u64)
                }
            } else {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h448, _, _, _) => {
                        h448
                    }
                }, 0u64)
            }
        } else {
            let a449 = v0;
            let a450 = 3u64;
            if nat_lt(a449, a450) {
                Adt2::C9(match v1 {
                    Adt1::C0(_, _, _, _, h451, _, _, _) => {
                        h451
                    }
                }, 32u64, 0u64)
            } else {
                Adt2::C2(match v1 {
                    Adt1::C0(_, _, _, _, h452, _, _, _) => {
                        h452
                    }
                })
            }
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt2> {
    Ok(Adt2::C1({
        let a461 = {
            let a453 = v0;
            let a454 = 16u64;
            let a455 = 0u64;
            nat_rem(a453, a454, a455)
        };
        let a462 = {
            let a459 = 16u64;
            let a460 = {
                let a456 = v2;
                let a457 = 16u64;
                let a458 = 0u64;
                nat_rem(a456, a457, a458)
            };
            nat_mul(a459, a460)?
        };
        nat_add(a461, a462)?
    }, {
        let a471 = {
            let a463 = v1;
            let a464 = 16u64;
            let a465 = 0u64;
            nat_rem(a463, a464, a465)
        };
        let a472 = {
            let a469 = 16u64;
            let a470 = {
                let a466 = v3;
                let a467 = 16u64;
                let a468 = 0u64;
                nat_rem(a466, a467, a468)
            };
            nat_mul(a469, a470)?
        };
        nat_add(a471, a472)?
    }, {
        let a495 = {
            let a481 = {
                let a473 = v0;
                let a474 = 16u64;
                let a475 = 0u64;
                nat_quot(a473, a474, a475)
            };
            let a482 = {
                let a479 = 4u64;
                let a480 = {
                    let a476 = v1;
                    let a477 = 16u64;
                    let a478 = 0u64;
                    nat_quot(a476, a477, a478)
                };
                nat_mul(a479, a480)?
            };
            nat_add(a481, a482)?
        };
        let a496 = {
            let a493 = {
                let a486 = 16u64;
                let a487 = {
                    let a483 = v2;
                    let a484 = 16u64;
                    let a485 = 0u64;
                    nat_quot(a483, a484, a485)
                };
                nat_mul(a486, a487)?
            };
            let a494 = {
                let a491 = 64u64;
                let a492 = {
                    let a488 = v3;
                    let a489 = 16u64;
                    let a490 = 0u64;
                    nat_quot(a488, a489, a490)
                };
                nat_mul(a491, a492)?
            };
            nat_add(a493, a494)?
        };
        nat_add(a495, a496)?
    }, 0u64))
}

pub fn f14(v0: u64, v1: Adt1) -> Adt2 {
    let a501 = {
        let a498 = match v1.clone() {
            Adt1::C0(h497, _, _, _, _, _, _, _) => {
                h497
            }
        };
        let a499 = 2147483648u64;
        let a500 = 0u64;
        nat_rem(a498, a499, a500)
    };
    let a502 = 646978941u64;
    if nat_lt(a501, a502) {
        Adt2::C2(0u64)
    } else {
        let a503 = v0;
        let a504 = 1u64;
        if nat_lt(a503, a504) {
            Adt2::C5(3u64, 3229614080u64, match v1 {
                Adt1::C0(h505, _, _, _, _, _, _, _) => {
                    h505
                }
            }, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h506, _, _, _) => {
                    h506
                }
            })
        }
    }
}

pub fn f15(v0: u64, v1: Adt1) -> Adt2 {
    let a507 = v0;
    let a508 = 2u64;
    if nat_lt(a507, a508) {
        let a509 = v0;
        let a510 = 1u64;
        if nat_lt(a509, a510) {
            Adt2::C5(2u64, match v1.clone() {
                Adt1::C0(h511, _, _, _, _, _, _, _) => {
                    h511
                }
            }, match v1 {
                Adt1::C0(_, h512, _, _, _, _, _, _) => {
                    h512
                }
            }, 0u64)
        } else {
            Adt2::C5(0u64, 1262485504u64, match v1 {
                Adt1::C0(_, _, _, _, h513, _, _, _) => {
                    h513
                }
            }, 0u64)
        }
    } else {
        let a514 = v0;
        let a515 = 3u64;
        if nat_lt(a514, a515) {
            Adt2::C9(match v1 {
                Adt1::C0(_, _, _, _, h516, _, _, _) => {
                    h516
                }
            }, 4u64, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h517, _, _, _) => {
                    h517
                }
            })
        }
    }
}

pub fn f16(v0: u64, v1: Adt1) -> Adt2 {
    let a518 = v0;
    let a519 = 2u64;
    if nat_lt(a518, a519) {
        let a520 = v0;
        let a521 = 1u64;
        if nat_lt(a520, a521) {
            Adt2::C5(3u64, match v1.clone() {
                Adt1::C0(h522, _, _, _, _, _, _, _) => {
                    h522
                }
            }, match v1 {
                Adt1::C0(_, h523, _, _, _, _, _, _) => {
                    h523
                }
            }, 0u64)
        } else {
            Adt2::C5(0u64, 1262485504u64, match v1 {
                Adt1::C0(_, _, _, _, h524, _, _, _) => {
                    h524
                }
            }, 0u64)
        }
    } else {
        let a525 = v0;
        let a526 = 3u64;
        if nat_lt(a525, a526) {
            Adt2::C9(match v1 {
                Adt1::C0(_, _, _, _, h527, _, _, _) => {
                    h527
                }
            }, 4u64, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h528, _, _, _) => {
                    h528
                }
            })
        }
    }
}

pub fn f17(v0: u64, v1: Adt1) -> Adt2 {
    f27(v0, v1, 0u64)
}

pub fn f18(v0: u64, v1: Adt1) -> Adt2 {
    f27(v0, v1, 1u64)
}

pub fn f19(v0: u64, v1: Adt1) -> Adt2 {
    if f28(match v1.clone() {
        Adt1::C0(_, h529, _, _, _, _, _, _) => {
            h529
        }
    }) {
        let a530 = v0;
        let a531 = 2u64;
        if nat_lt(a530, a531) {
            let a532 = v0;
            let a533 = 1u64;
            if nat_lt(a532, a533) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h534, _, _, _, _, _, _, _) => {
                        h534
                    }
                }, match v1 {
                    Adt1::C0(_, _, h535, _, _, _, _, _) => {
                        h535
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, h536, _, _, _) => {
                        h536
                    }
                }, match v1 {
                    Adt1::C0(_, h537, _, _, _, _, _, _) => {
                        h537
                    }
                }, 0u64)
            }
        } else {
            let a538 = v0;
            let a539 = 3u64;
            if nat_lt(a538, a539) {
                Adt2::C5(0u64, 1262485504u64, match v1 {
                    Adt1::C0(_, _, _, _, h540, _, _, _) => {
                        h540
                    }
                }, 0u64)
            } else {
                let a541 = v0;
                let a542 = 4u64;
                if nat_lt(a541, a542) {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h543, _, _, _) => {
                            h543
                        }
                    }, 4u64, 0u64)
                } else {
                    Adt2::C2(match v1 {
                        Adt1::C0(_, _, _, _, h544, _, _, _) => {
                            h544
                        }
                    })
                }
            }
        }
    } else {
        Adt2::C2(match v1 {
            Adt1::C0(_, _, _, h545, _, _, _, _) => {
                h545
            }
        })
    }
}

pub fn f20(v0: u64, v1: Adt1) -> Adt2 {
    if f28(match v1.clone() {
        Adt1::C0(_, h546, _, _, _, _, _, _) => {
            h546
        }
    }) {
        let a547 = v0;
        let a548 = 3u64;
        if nat_lt(a547, a548) {
            let a549 = v0;
            let a550 = 1u64;
            if nat_lt(a549, a550) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h551, _, _, _, _, _, _, _) => {
                        h551
                    }
                }, match v1 {
                    Adt1::C0(h552, _, _, _, _, _, _, _) => {
                        h552
                    }
                }, 0u64)
            } else {
                let a553 = v0;
                let a554 = 2u64;
                if nat_lt(a553, a554) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, h555, _, _, _) => {
                            h555
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, h556, _, _, _, _) => {
                            h556
                        }
                    }, 0u64)
                } else {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, _, h557, _, _, _, _, _) => {
                            h557
                        }
                    }, match v1 {
                        Adt1::C0(_, _, h558, _, _, _, _, _) => {
                            h558
                        }
                    }, 1u64)
                }
            }
        } else {
            let a559 = v0;
            let a560 = 4u64;
            if nat_lt(a559, a560) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h561, _, _) => {
                        h561
                    }
                }, match v1 {
                    Adt1::C0(_, h562, _, _, _, _, _, _) => {
                        h562
                    }
                }, 1u64)
            } else {
                let a563 = v0;
                let a564 = 5u64;
                if nat_lt(a563, a564) {
                    Adt2::C8(match v1.clone() {
                        Adt1::C0(_, _, _, _, h565, _, _, _) => {
                            h565
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h566, _, _) => {
                            h566
                        }
                    }, 2u64)
                } else {
                    Adt2::C2({
                        let a568 = match v1 {
                            Adt1::C0(_, _, _, _, _, _, h567, _) => {
                                h567
                            }
                        };
                        let a569 = 2u64;
                        if nat_eq(a568, a569) {
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

pub fn f21(v0: u64, v1: Adt1) -> Adt2 {
    if f28(match v1.clone() {
        Adt1::C0(_, h570, _, _, _, _, _, _) => {
            h570
        }
    }) {
        let a571 = v0;
        let a572 = 1u64;
        if nat_lt(a571, a572) {
            Adt2::C5(3u64, match v1.clone() {
                Adt1::C0(h573, _, _, _, _, _, _, _) => {
                    h573
                }
            }, match v1 {
                Adt1::C0(_, h574, _, _, _, _, _, _) => {
                    h574
                }
            }, 0u64)
        } else {
            Adt2::C2(match v1 {
                Adt1::C0(_, _, _, _, h575, _, _, _) => {
                    h575
                }
            })
        }
    } else {
        Adt2::C2(0u64)
    }
}

pub fn f22(v0: u64, v1: Adt1) -> Adt2 {
    let a580 = {
        let a577 = match v1.clone() {
            Adt1::C0(h576, _, _, _, _, _, _, _) => {
                h576
            }
        };
        let a578 = 2147483648u64;
        let a579 = 0u64;
        nat_rem(a577, a578, a579)
    };
    let a581 = 0u64;
    if nat_eq(a580, a581) {
        Adt2::C1(0u64, 0u64, 0u64, 0u64)
    } else {
        let a582 = v0;
        let a583 = 2u64;
        if nat_lt(a582, a583) {
            let a584 = v0;
            let a585 = 1u64;
            if nat_lt(a584, a585) {
                Adt2::C5(3u64, 3254779904u64, match v1 {
                    Adt1::C0(h586, _, _, _, _, _, _, _) => {
                        h586
                    }
                }, 0u64)
            } else {
                Adt2::C5(3u64, 1065353216u64, match v1 {
                    Adt1::C0(_, _, _, _, h587, _, _, _) => {
                        h587
                    }
                }, 1u64)
            }
        } else {
            let a588 = v0;
            let a589 = 3u64;
            if nat_lt(a588, a589) {
                Adt2::C7(true, match v1 {
                    Adt1::C0(_, _, _, _, _, h590, _, _) => {
                        h590
                    }
                }, 2u64)
            } else {
                let a591 = v0;
                let a592 = 4u64;
                if nat_lt(a591, a592) {
                    Adt2::C7(false, match v1 {
                        Adt1::C0(_, _, _, _, _, _, h593, _) => {
                            h593
                        }
                    }, 1u64)
                } else {
                    Adt2::C1(match v1.clone() {
                        Adt1::C0(_, _, _, _, h594, _, _, _) => {
                            h594
                        }
                    }, match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h595, _) => {
                            h595
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h596, _, _) => {
                            h596
                        }
                    }, 0u64)
                }
            }
        }
    }
}

pub fn f23(v0: u64, v1: Adt1) -> Adt2 {
    let a601 = {
        let a598 = match v1.clone() {
            Adt1::C0(h597, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a599 = 2147483648u64;
        let a600 = 0u64;
        nat_rem(a598, a599, a600)
    };
    let a602 = 0u64;
    if nat_eq(a601, a602) {
        Adt2::C3(0u64, 0u64)
    } else {
        let a603 = v0;
        let a604 = 3u64;
        if nat_lt(a603, a604) {
            let a605 = v0;
            let a606 = 1u64;
            if nat_lt(a605, a606) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h607, _, _, _, _, _, _, _) => {
                        h607
                    }
                }, match v1 {
                    Adt1::C0(_, _, h608, _, _, _, _, _) => {
                        h608
                    }
                }, 0u64)
            } else {
                let a609 = v0;
                let a610 = 2u64;
                if nat_lt(a609, a610) {
                    Adt2::C5(0u64, 1262485504u64, match v1 {
                        Adt1::C0(_, _, _, _, h611, _, _, _) => {
                            h611
                        }
                    }, 0u64)
                } else {
                    Adt2::C9(match v1 {
                        Adt1::C0(_, _, _, _, h612, _, _, _) => {
                            h612
                        }
                    }, 32u64, 0u64)
                }
            }
        } else {
            let a613 = v0;
            let a614 = 4u64;
            if nat_lt(a613, a614) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, _, _, _, h615, _, _, _) => {
                        h615
                    }
                }, 32u64, 1u64)
            } else {
                let a616 = v0;
                let a617 = 5u64;
                if nat_lt(a616, a617) {
                    Adt2::C5(2u64, match v1.clone() {
                        Adt1::C0(_, h618, _, _, _, _, _, _) => {
                            h618
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h619, _, _) => {
                            h619
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, h620, _, _, _) => {
                            h620
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h621, _, _) => {
                            h621
                        }
                    })
                }
            }
        }
    }
}

pub fn f24(v0: u64, v1: u64) -> R<Adt2> {
    let a622 = v0;
    let a623 = 32u64;
    if nat_lt(a622, a623) {
        let a624 = v1;
        let a625 = 8u64;
        if nat_lt(a624, a625) {
            Ok(Adt2::C10(0u64, {
                let a628 = v0;
                let a629 = {
                    let a626 = 32u64;
                    let a627 = v1;
                    nat_mul(a626, a627)?
                };
                nat_add(a628, a629)?
            }, 4u64, 2u64, {
                let a630 = v1;
                let a631 = 4u64;
                if nat_lt(a630, a631) {
                    let a632 = v1;
                    let a633 = 2u64;
                    if nat_lt(a632, a633) {
                        let a634 = v1;
                        let a635 = 0u64;
                        if nat_eq(a634, a635) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a636 = v1;
                        let a637 = 2u64;
                        if nat_eq(a636, a637) {
                            4u64
                        } else {
                            8u64
                        }
                    }
                } else {
                    let a638 = v1;
                    let a639 = 6u64;
                    if nat_lt(a638, a639) {
                        let a640 = v1;
                        let a641 = 4u64;
                        if nat_eq(a640, a641) {
                            16u64
                        } else {
                            32u64
                        }
                    } else {
                        let a642 = v1;
                        let a643 = 6u64;
                        if nat_eq(a642, a643) {
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
        let a644 = v0;
        let a645 = 96u64;
        if nat_lt(a644, a645) {
            let a646 = v1;
            let a647 = 4u64;
            if nat_lt(a646, a647) {
                Ok(Adt2::C10(0u64, {
                    let a664 = {
                        let a660 = {
                            let a653 = {
                                let a650 = {
                                    let a648 = v0;
                                    let a649 = 32u64;
                                    nat_sub(a648, a649)
                                };
                                let a651 = 32u64;
                                let a652 = 0u64;
                                nat_quot(a650, a651, a652)
                            };
                            let a654 = 128u64;
                            nat_mul(a653, a654)?
                        };
                        let a661 = {
                            let a657 = {
                                let a655 = v0;
                                let a656 = 32u64;
                                nat_sub(a655, a656)
                            };
                            let a658 = 32u64;
                            let a659 = 0u64;
                            nat_rem(a657, a658, a659)
                        };
                        nat_add(a660, a661)?
                    };
                    let a665 = {
                        let a662 = 32u64;
                        let a663 = v1;
                        nat_mul(a662, a663)?
                    };
                    nat_add(a664, a665)?
                }, 1u64, 4u64, {
                    let a668 = {
                        let a666 = 2u64;
                        let a667 = v1;
                        nat_mul(a666, a667)?
                    };
                    let a669 = 4u64;
                    if nat_lt(a668, a669) {
                        let a672 = {
                            let a670 = 2u64;
                            let a671 = v1;
                            nat_mul(a670, a671)?
                        };
                        let a673 = 2u64;
                        if nat_lt(a672, a673) {
                            let a676 = {
                                let a674 = 2u64;
                                let a675 = v1;
                                nat_mul(a674, a675)?
                            };
                            let a677 = 0u64;
                            if nat_eq(a676, a677) {
                                1u64
                            } else {
                                2u64
                            }
                        } else {
                            let a680 = {
                                let a678 = 2u64;
                                let a679 = v1;
                                nat_mul(a678, a679)?
                            };
                            let a681 = 2u64;
                            if nat_eq(a680, a681) {
                                4u64
                            } else {
                                8u64
                            }
                        }
                    } else {
                        let a684 = {
                            let a682 = 2u64;
                            let a683 = v1;
                            nat_mul(a682, a683)?
                        };
                        let a685 = 6u64;
                        if nat_lt(a684, a685) {
                            let a688 = {
                                let a686 = 2u64;
                                let a687 = v1;
                                nat_mul(a686, a687)?
                            };
                            let a689 = 4u64;
                            if nat_eq(a688, a689) {
                                16u64
                            } else {
                                32u64
                            }
                        } else {
                            let a692 = {
                                let a690 = 2u64;
                                let a691 = v1;
                                nat_mul(a690, a691)?
                            };
                            let a693 = 6u64;
                            if nat_eq(a692, a693) {
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
            let a694 = v0;
            let a695 = 104u64;
            if nat_lt(a694, a695) {
                let a696 = v1;
                let a697 = 2u64;
                if nat_lt(a696, a697) {
                    Ok(Adt2::C10(1u64, {
                        let a702 = {
                            let a698 = v0;
                            let a699 = 96u64;
                            nat_sub(a698, a699)
                        };
                        let a703 = {
                            let a700 = 8u64;
                            let a701 = v1;
                            nat_mul(a700, a701)?
                        };
                        nat_add(a702, a703)?
                    }, 1u64, 16u64, {
                        let a704 = v1;
                        let a705 = 0u64;
                        if nat_eq(a704, a705) {
                            1u64
                        } else {
                            16u64
                        }
                    }))
                } else {
                    Ok(Adt2::C2(0u64))
                }
            } else {
                let a706 = v0;
                let a707 = 108u64;
                if nat_lt(a706, a707) {
                    let a708 = v1;
                    let a709 = 4u64;
                    if nat_lt(a708, a709) {
                        Ok(Adt2::C10(1u64, {
                            let a714 = {
                                let a710 = v0;
                                let a711 = 104u64;
                                nat_sub(a710, a711)
                            };
                            let a715 = {
                                let a712 = 4u64;
                                let a713 = v1;
                                nat_mul(a712, a713)?
                            };
                            nat_add(a714, a715)?
                        }, 16u64, 4u64, {
                            let a718 = {
                                let a716 = 2u64;
                                let a717 = v1;
                                nat_mul(a716, a717)?
                            };
                            let a719 = 4u64;
                            if nat_lt(a718, a719) {
                                let a722 = {
                                    let a720 = 2u64;
                                    let a721 = v1;
                                    nat_mul(a720, a721)?
                                };
                                let a723 = 2u64;
                                if nat_lt(a722, a723) {
                                    let a726 = {
                                        let a724 = 2u64;
                                        let a725 = v1;
                                        nat_mul(a724, a725)?
                                    };
                                    let a727 = 0u64;
                                    if nat_eq(a726, a727) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                } else {
                                    let a730 = {
                                        let a728 = 2u64;
                                        let a729 = v1;
                                        nat_mul(a728, a729)?
                                    };
                                    let a731 = 2u64;
                                    if nat_eq(a730, a731) {
                                        4u64
                                    } else {
                                        8u64
                                    }
                                }
                            } else {
                                let a734 = {
                                    let a732 = 2u64;
                                    let a733 = v1;
                                    nat_mul(a732, a733)?
                                };
                                let a735 = 6u64;
                                if nat_lt(a734, a735) {
                                    let a738 = {
                                        let a736 = 2u64;
                                        let a737 = v1;
                                        nat_mul(a736, a737)?
                                    };
                                    let a739 = 4u64;
                                    if nat_eq(a738, a739) {
                                        16u64
                                    } else {
                                        32u64
                                    }
                                } else {
                                    let a742 = {
                                        let a740 = 2u64;
                                        let a741 = v1;
                                        nat_mul(a740, a741)?
                                    };
                                    let a743 = 6u64;
                                    if nat_eq(a742, a743) {
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
                    let a744 = v0;
                    let a745 = 110u64;
                    if nat_lt(a744, a745) {
                        let a746 = v1;
                        let a747 = 0u64;
                        if nat_eq(a746, a747) {
                            Ok(Adt2::C10(2u64, 0u64, {
                                let a748 = v0;
                                let a749 = 108u64;
                                if nat_eq(a748, a749) {
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

pub fn f25(v0: u64) -> u64 {
    let a750 = v0;
    let a751 = 8u64;
    if nat_lt(a750, a751) {
        let a752 = v0;
        let a753 = 4u64;
        if nat_lt(a752, a753) {
            let a754 = v0;
            let a755 = 2u64;
            if nat_lt(a754, a755) {
                let a756 = v0;
                let a757 = 1u64;
                if nat_lt(a756, a757) {
                    1u64
                } else {
                    24u64
                }
            } else {
                let a758 = v0;
                let a759 = 3u64;
                if nat_lt(a758, a759) {
                    45u64
                } else {
                    63u64
                }
            }
        } else {
            let a760 = v0;
            let a761 = 6u64;
            if nat_lt(a760, a761) {
                let a762 = v0;
                let a763 = 5u64;
                if nat_lt(a762, a763) {
                    79u64
                } else {
                    93u64
                }
            } else {
                let a764 = v0;
                let a765 = 7u64;
                if nat_lt(a764, a765) {
                    106u64
                } else {
                    118u64
                }
            }
        }
    } else {
        let a766 = v0;
        let a767 = 12u64;
        if nat_lt(a766, a767) {
            let a768 = v0;
            let a769 = 10u64;
            if nat_lt(a768, a769) {
                let a770 = v0;
                let a771 = 9u64;
                if nat_lt(a770, a771) {
                    129u64
                } else {
                    141u64
                }
            } else {
                let a772 = v0;
                let a773 = 11u64;
                if nat_lt(a772, a773) {
                    153u64
                } else {
                    166u64
                }
            }
        } else {
            let a774 = v0;
            let a775 = 14u64;
            if nat_lt(a774, a775) {
                let a776 = v0;
                let a777 = 13u64;
                if nat_lt(a776, a777) {
                    181u64
                } else {
                    197u64
                }
            } else {
                let a778 = v0;
                let a779 = 15u64;
                if nat_lt(a778, a779) {
                    217u64
                } else {
                    241u64
                }
            }
        }
    }
}

pub fn f26(v0: u64) -> R<u64> {
    let a783 = 2139095040u64;
    let a784 = {
        let a780 = v0;
        let a781 = 2147483648u64;
        let a782 = 0u64;
        nat_rem(a780, a781, a782)
    };
    if nat_lt(a783, a784) {
        Ok(15u64)
    } else {
        let v1: u64 = {
            let a785 = v0;
            let a786 = 2147483648u64;
            if nat_lt(a785, a786) {
                let a787 = v0;
                let a788 = 2147483648u64;
                nat_add(a787, a788)?
            } else {
                let a789 = 4294967295u64;
                let a790 = v0;
                nat_sub(a789, a790)
            }
        };
        let a791 = v1;
        let a792 = 1064304639u64;
        if nat_lt(a791, a792) {
            let a793 = v1;
            let a794 = 1033633791u64;
            if nat_lt(a793, a794) {
                let a795 = v1;
                let a796 = 1027932159u64;
                if nat_lt(a795, a796) {
                    let a797 = v1;
                    let a798 = 1025048575u64;
                    if nat_lt(a797, a798) {
                        Ok(0u64)
                    } else {
                        Ok(1u64)
                    }
                } else {
                    let a799 = v1;
                    let a800 = 1030488063u64;
                    if nat_lt(a799, a800) {
                        Ok(2u64)
                    } else {
                        Ok(3u64)
                    }
                }
            } else {
                let a801 = v1;
                let a802 = 1042022399u64;
                if nat_lt(a801, a802) {
                    let a803 = v1;
                    let a804 = 1037565951u64;
                    if nat_lt(a803, a804) {
                        Ok(4u64)
                    } else {
                        Ok(5u64)
                    }
                } else {
                    let a805 = v1;
                    let a806 = 1048575999u64;
                    if nat_lt(a805, a806) {
                        Ok(6u64)
                    } else {
                        Ok(7u64)
                    }
                }
            }
        } else {
            let a807 = v1;
            let a808 = 3258318848u64;
            if nat_lt(a807, a808) {
                let a809 = v1;
                let a810 = 3247964160u64;
                if nat_lt(a809, a810) {
                    let a811 = v1;
                    let a812 = 3235905536u64;
                    if nat_lt(a811, a812) {
                        Ok(8u64)
                    } else {
                        Ok(9u64)
                    }
                } else {
                    let a813 = v1;
                    let a814 = 3254517760u64;
                    if nat_lt(a813, a814) {
                        Ok(10u64)
                    } else {
                        Ok(11u64)
                    }
                }
            } else {
                let a815 = v1;
                let a816 = 3265134592u64;
                if nat_lt(a815, a816) {
                    let a817 = v1;
                    let a818 = 3262382080u64;
                    if nat_lt(a817, a818) {
                        Ok(12u64)
                    } else {
                        Ok(13u64)
                    }
                } else {
                    let a819 = v1;
                    let a820 = 3268018176u64;
                    if nat_lt(a819, a820) {
                        Ok(14u64)
                    } else {
                        Ok(15u64)
                    }
                }
            }
        }
    }
}

pub fn f27(v0: u64, v1: Adt1, v2: u64) -> Adt2 {
    let a821 = v0;
    let a822 = 4u64;
    if nat_lt(a821, a822) {
        let a823 = v0;
        let a824 = 2u64;
        if nat_lt(a823, a824) {
            let a825 = v0;
            let a826 = 1u64;
            if nat_lt(a825, a826) {
                Adt2::C6(match v1 {
                    Adt1::C0(_, h827, _, _, _, _, _, _) => {
                        h827
                    }
                }, 4u64, 0u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(h828, _, _, _, _, _, _, _) => {
                        h828
                    }
                }, match v1 {
                    Adt1::C0(h829, _, _, _, _, _, _, _) => {
                        h829
                    }
                }, 1u64)
            }
        } else {
            let a830 = v0;
            let a831 = 3u64;
            if nat_lt(a830, a831) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h832, _, _) => {
                        h832
                    }
                }, match v1 {
                    Adt1::C0(h833, _, _, _, _, _, _, _) => {
                        h833
                    }
                }, 2u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h834, _) => {
                        h834
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h835, _, _, _) => {
                        h835
                    }
                }, 2u64)
            }
        }
    } else {
        let a836 = v0;
        let a837 = 6u64;
        if nat_lt(a836, a837) {
            let a838 = v0;
            let a839 = 5u64;
            if nat_lt(a838, a839) {
                Adt2::C5(v2, match v1.clone() {
                    Adt1::C0(_, _, h840, _, _, _, _, _) => {
                        h840
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h841, _) => {
                        h841
                    }
                }, 2u64)
            } else {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h842, _, _) => {
                        h842
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h843, _, _, _) => {
                        h843
                    }
                }, 1u64)
            }
        } else {
            let a844 = v0;
            let a845 = 7u64;
            if nat_lt(a844, a845) {
                Adt2::C5(2u64, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h846, _, _) => {
                        h846
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, h847, _, _, _) => {
                        h847
                    }
                }, 1u64)
            } else {
                let a848 = v0;
                let a849 = 8u64;
                if nat_lt(a848, a849) {
                    Adt2::C5(v2, match v1.clone() {
                        Adt1::C0(_, _, _, h850, _, _, _, _) => {
                            h850
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h851, _, _) => {
                            h851
                        }
                    }, 1u64)
                } else {
                    Adt2::C3(match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, h852, _) => {
                            h852
                        }
                    }, match v1 {
                        Adt1::C0(_, _, _, _, _, h853, _, _) => {
                            h853
                        }
                    })
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> bool {
    let a854 = 0u64;
    let a855 = v0;
    if nat_lt(a854, a855) {
        let a856 = v0;
        let a857 = 2139095040u64;
        nat_le(a856, a857)
    } else {
        false
    }
}
