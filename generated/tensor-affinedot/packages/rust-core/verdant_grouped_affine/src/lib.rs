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
    C0(bool, u64, u64),
    C1(u64),
    C2(bool, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64),
    C4(bool),
    C5(u64, u64, u64),
    C6(u64, u64),
    C7(u64, u64, u64, u64, u64, u64),
    C8(u64, u64, u64, u64),
    C9(bool, u64, u64),
    C10(bool, u64),
    C11(bool, u64, u64, u64, u64, u64),
    C12(u64, u64),
    C13(u64),
    C14(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64),
    C2(u64, u64, u64),
    C3(u64, u64, u64, u64, u64),
    C4(u64, u64),
    C5(u64, u64),
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, bool, u64, u64),
    C8(u64, u64, u64, u64),
    C9(u64, u64, u64),
    C10(u64, bool, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt1> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3) => {
            let v4: u64 = if v1 {
                176u64
            } else {
                144u64
            };
            let a4 = v2;
            let a5 = {
                let a2 = v4;
                let a3 = v3;
                nat_mul(a2, a3)?
            };
            if nat_eq(a4, a5) {
                Ok(Adt1::C1(v4))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C1(v5) => {
            let a6 = v5;
            let a7 = 8u64;
            if nat_lt(a6, a7) {
                let v6: u64 = {
                    let a8 = v5;
                    let a9 = 4u64;
                    let a10 = 0u64;
                    nat_rem(a8, a9, a10)
                };
                Ok(Adt1::C2({
                    let a11 = 4u64;
                    let a12 = v6;
                    nat_add(a11, a12)?
                }, {
                    let a13 = 8u64;
                    let a14 = v6;
                    nat_add(a13, a14)?
                }, {
                    let a15 = 12u64;
                    let a16 = v6;
                    nat_add(a15, a16)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C2(v7, v8, v9, v10, v11) => {
            f1(v7, v8, v9, v10, v11)
        }
        Adt0::C3(v12, v13, v14, v15, v16) => {
            let v17: u64 = {
                let a20 = {
                    let a17 = v12;
                    let a18 = v13;
                    let a19 = 0u64;
                    nat_quot(a17, a18, a19)
                };
                let a21 = 16u64;
                let a22 = 0u64;
                nat_rem(a20, a21, a22)
            };
            let v18: u64 = {
                let a23 = v15;
                let a24 = 0u64;
                if nat_eq(a23, a24) {
                    0u64
                } else {
                    let a31 = 16u64;
                    let a32 = {
                        let a28 = {
                            let a25 = v14;
                            let a26 = v15;
                            let a27 = 0u64;
                            nat_quot(a25, a26, a27)
                        };
                        let a29 = 2u64;
                        let a30 = 0u64;
                        nat_rem(a28, a29, a30)
                    };
                    nat_mul(a31, a32)?
                }
            };
            Ok(Adt1::C1({
                let a35 = v16;
                let a36 = {
                    let a33 = v17;
                    let a34 = v18;
                    nat_add(a33, a34)?
                };
                nat_mul(a35, a36)?
            }))
        }
        Adt0::C4(v57) => {
            Ok(Adt1::C4(2u64, if v57 {
                1u64
            } else {
                0u64
            }))
        }
        Adt0::C5(v19, v20, v21) => {
            let a37 = v19;
            let a38 = 3u64;
            if if nat_eq(a37, a38) {
                true
            } else {
                let a39 = v19;
                let a40 = 4u64;
                nat_eq(a39, a40)
            } {
                let v22: u64 = {
                    let a41 = v19;
                    let a42 = 3u64;
                    if nat_eq(a41, a42) {
                        98u64
                    } else {
                        110u64
                    }
                };
                let a46 = {
                    let a43 = v20;
                    let a44 = v22;
                    let a45 = 0u64;
                    nat_rem(a43, a44, a45)
                };
                let a47 = 0u64;
                if if nat_eq(a46, a47) {
                    let a51 = {
                        let a48 = v20;
                        let a49 = v22;
                        let a50 = 0u64;
                        nat_quot(a48, a49, a50)
                    };
                    let a52 = v21;
                    nat_eq(a51, a52)
                } else {
                    false
                } {
                    Ok(Adt1::C9(v22, {
                        let a53 = v19;
                        let a54 = 3u64;
                        if nat_eq(a53, a54) {
                            1048576000u64
                        } else {
                            1065353216u64
                        }
                    }, 4u64))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                f2(v19, v20, v21)
            }
        }
        Adt0::C6(v23, v24) => {
            let a55 = v23;
            let a56 = 3u64;
            if if nat_eq(a55, a56) {
                true
            } else {
                let a57 = v23;
                let a58 = 4u64;
                nat_eq(a57, a58)
            } {
                f3({
                    let a59 = v23;
                    let a60 = 3u64;
                    nat_eq(a59, a60)
                }, v24)
            } else {
                f4(v23, v24)
            }
        }
        Adt0::C7(v25, v26, v27, v28, v29, v30) => {
            let a61 = v25;
            let a62 = 3u64;
            if if nat_eq(a61, a62) {
                true
            } else {
                let a63 = v25;
                let a64 = 4u64;
                nat_eq(a63, a64)
            } {
                f5({
                    let a65 = v25;
                    let a66 = 3u64;
                    nat_eq(a65, a66)
                }, v26, v27, v28, v29, v30)
            } else {
                f6(v25, v26, v27, v28, v29, v30)
            }
        }
        Adt0::C8(v31, v32, v33, v34) => {
            f7(v31, v32, v33, v34)
        }
        Adt0::C9(v45, v46, v47) => {
            let v48: u64 = if v45 {
                210u64
            } else {
                110u64
            };
            let a126 = v46;
            let a127 = {
                let a124 = v48;
                let a125 = v47;
                nat_mul(a124, a125)?
            };
            if nat_eq(a126, a127) {
                Ok(Adt1::C1(v48))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C10(v49, v50) => {
            f9(v49, v50)
        }
        Adt0::C11(v51, v52, v53, v54, v55, v56) => {
            f10(v51, v52, v53, v54, v55, v56)
        }
        Adt0::C12(v35, v36) => {
            let a69 = v35;
            let a70 = {
                let a67 = 136u64;
                let a68 = v36;
                nat_mul(a67, a68)?
            };
            if nat_eq(a69, a70) {
                Ok(Adt1::C5(136u64, 1065353216u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C13(v37) => {
            let a71 = v37;
            let a72 = 8u64;
            if nat_lt(a71, a72) {
                Ok(Adt1::C2({
                    let a76 = 4u64;
                    let a77 = {
                        let a73 = v37;
                        let a74 = 2u64;
                        let a75 = 0u64;
                        nat_quot(a73, a74, a75)
                    };
                    nat_add(a76, a77)?
                }, {
                    let a81 = 2u64;
                    let a82 = {
                        let a78 = v37;
                        let a79 = 4u64;
                        let a80 = 0u64;
                        nat_quot(a78, a79, a80)
                    };
                    nat_add(a81, a82)?
                }, {
                    let a85 = 8u64;
                    let a86 = {
                        let a83 = v37;
                        let a84 = 16u64;
                        nat_mul(a83, a84)?
                    };
                    nat_add(a85, a86)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C14(v38, v39, v40) => {
            let a87 = v38;
            let a88 = 8u64;
            if if if nat_lt(a87, a88) {
                let a89 = v39;
                let a90 = 256u64;
                nat_lt(a89, a90)
            } else {
                false
            } {
                let a91 = v40;
                let a92 = 256u64;
                nat_lt(a91, a92)
            } else {
                false
            } {
                let v41: u64 = {
                    let a96 = {
                        let a93 = v38;
                        let a94 = 2u64;
                        let a95 = 0u64;
                        nat_rem(a93, a94, a95)
                    };
                    let a97 = 0u64;
                    if nat_eq(a96, a97) {
                        1u64
                    } else {
                        16u64
                    }
                };
                let v42: u64 = {
                    let a101 = 2u64;
                    let a102 = {
                        let a98 = v38;
                        let a99 = 4u64;
                        let a100 = 0u64;
                        nat_rem(a98, a99, a100)
                    };
                    nat_mul(a101, a102)?
                };
                let v43: u64 = {
                    let a106 = {
                        let a103 = v39;
                        let a104 = v41;
                        let a105 = 0u64;
                        nat_quot(a103, a104, a105)
                    };
                    let a107 = 16u64;
                    let a108 = 0u64;
                    nat_rem(a106, a107, a108)
                };
                let v44: u64 = {
                    let a112 = {
                        let a109 = v40;
                        let a110 = f8(v42);
                        let a111 = 0u64;
                        nat_quot(a109, a110, a111)
                    };
                    let a113 = 4u64;
                    let a114 = 0u64;
                    nat_rem(a112, a113, a114)
                };
                Ok(Adt1::C1({
                    let a121 = {
                        let a119 = 224u64;
                        let a120 = {
                            let a117 = v43;
                            let a118 = {
                                let a115 = 16u64;
                                let a116 = v44;
                                nat_mul(a115, a116)?
                            };
                            nat_add(a117, a118)?
                        };
                        nat_add(a119, a120)?
                    };
                    let a122 = 256u64;
                    let a123 = 0u64;
                    nat_rem(a121, a122, a123)
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a128 = v1;
    let a129 = 8u64;
    if if if if nat_lt(a128, a129) {
        let a130 = v2;
        let a131 = 256u64;
        nat_lt(a130, a131)
    } else {
        false
    } {
        let a132 = v3;
        let a133 = 256u64;
        nat_lt(a132, a133)
    } else {
        false
    } {
        let a134 = v4;
        let a135 = 256u64;
        nat_lt(a134, a135)
    } else {
        false
    } {
        let v5: u64 = {
            let a136 = v1;
            let a137 = 4u64;
            if nat_lt(a136, a137) {
                let a138 = v2;
                let a139 = 64u64;
                let a140 = 0u64;
                nat_rem(a138, a139, a140)
            } else {
                let a149 = {
                    let a141 = v4;
                    let a142 = 16u64;
                    let a143 = 0u64;
                    nat_rem(a141, a142, a143)
                };
                let a150 = {
                    let a147 = 16u64;
                    let a148 = {
                        let a144 = v2;
                        let a145 = 64u64;
                        let a146 = 0u64;
                        nat_quot(a144, a145, a146)
                    };
                    nat_mul(a147, a148)?
                };
                nat_add(a149, a150)?
            }
        };
        let v6: u64 = {
            let a151 = v1;
            let a152 = 4u64;
            if nat_lt(a151, a152) {
                let a153 = v3;
                let a154 = 64u64;
                let a155 = 0u64;
                nat_rem(a153, a154, a155)
            } else {
                let a164 = {
                    let a156 = v4;
                    let a157 = 16u64;
                    let a158 = 0u64;
                    nat_quot(a156, a157, a158)
                };
                let a165 = {
                    let a162 = 16u64;
                    let a163 = {
                        let a159 = v3;
                        let a160 = 64u64;
                        let a161 = 0u64;
                        nat_quot(a159, a160, a161)
                    };
                    nat_mul(a162, a163)?
                };
                nat_add(a164, a165)?
            }
        };
        Ok(Adt1::C3({
            let a171 = if v0 {
                48u64
            } else {
                16u64
            };
            let a172 = {
                let a169 = 32u64;
                let a170 = {
                    let a166 = v1;
                    let a167 = 2u64;
                    let a168 = 0u64;
                    nat_quot(a166, a167, a168)
                };
                nat_mul(a169, a170)?
            };
            nat_add(a171, a172)?
        }, {
            let a176 = {
                let a173 = v1;
                let a174 = 2u64;
                let a175 = 0u64;
                nat_rem(a173, a174, a175)
            };
            let a177 = 0u64;
            if nat_eq(a176, a177) {
                1u64
            } else {
                16u64
            }
        }, if v0 {
            f8(v1)
        } else {
            0u64
        }, v5, v6))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = {
        let a178 = v0;
        let a179 = 0u64;
        if nat_eq(a178, a179) {
            66u64
        } else {
            let a180 = v0;
            let a181 = 1u64;
            if nat_eq(a180, a181) {
                74u64
            } else {
                82u64
            }
        }
    };
    let a182 = v0;
    let a183 = 3u64;
    if if nat_lt(a182, a183) {
        let a186 = v1;
        let a187 = {
            let a184 = v3;
            let a185 = v2;
            nat_mul(a184, a185)?
        };
        nat_eq(a186, a187)
    } else {
        false
    } {
        Ok(Adt1::C5(v3, 1040187392u64))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: bool, v1: u64) -> R<Adt1> {
    let a188 = v1;
    let a189 = 64u64;
    if nat_lt(a188, a189) {
        let v2: u64 = {
            let a190 = v1;
            let a191 = 8u64;
            let a192 = 0u64;
            nat_quot(a190, a191, a192)
        };
        if v0 {
            Ok(Adt1::C6({
                let a193 = 2u64;
                let a194 = v1;
                nat_add(a193, a194)?
            }, 1u64, {
                let a197 = 66u64;
                let a198 = {
                    let a195 = 4u64;
                    let a196 = v2;
                    nat_mul(a195, a196)?
                };
                nat_add(a197, a198)?
            }, 4u64, 0u64, 0u64))
        } else {
            Ok(Adt1::C6({
                let a199 = 2u64;
                let a200 = v1;
                nat_add(a199, a200)?
            }, 1u64, {
                let a201 = 66u64;
                let a202 = v2;
                nat_add(a201, a202)?
            }, 1u64, {
                let a206 = 106u64;
                let a207 = {
                    let a203 = v2;
                    let a204 = 2u64;
                    let a205 = 0u64;
                    nat_quot(a203, a204, a205)
                };
                nat_add(a206, a207)?
            }, {
                let a211 = 74u64;
                let a212 = {
                    let a208 = v1;
                    let a209 = 2u64;
                    let a210 = 0u64;
                    nat_quot(a208, a209, a210)
                };
                nat_add(a211, a212)?
            }))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64) -> R<Adt1> {
    let a213 = v0;
    let a214 = 3u64;
    if if nat_lt(a213, a214) {
        let a215 = v1;
        let a216 = 32u64;
        nat_lt(a215, a216)
    } else {
        false
    } {
        let v2: u64 = {
            let a217 = v1;
            let a218 = 4u64;
            let a219 = 0u64;
            nat_quot(a217, a218, a219)
        };
        let v3: u64 = {
            let a220 = v1;
            let a221 = 4u64;
            let a222 = 0u64;
            nat_rem(a220, a221, a222)
        };
        let a223 = v0;
        let a224 = 0u64;
        if nat_eq(a223, a224) {
            let v4: u64 = {
                let a227 = 2u64;
                let a228 = {
                    let a225 = 8u64;
                    let a226 = v2;
                    nat_mul(a225, a226)?
                };
                nat_add(a227, a228)?
            };
            Ok(Adt1::C6({
                let a229 = v4;
                let a230 = v3;
                nat_add(a229, a230)?
            }, 1u64, {
                let a231 = v4;
                let a232 = 4u64;
                nat_add(a231, a232)?
            }, 4u64, v4, v4))
        } else {
            let a233 = v0;
            let a234 = 1u64;
            if nat_eq(a233, a234) {
                Ok(Adt1::C6({
                    let a237 = 2u64;
                    let a238 = {
                        let a235 = 2u64;
                        let a236 = v1;
                        nat_mul(a235, a236)?
                    };
                    nat_add(a237, a238)?
                }, 2u64, 0u64, 1u64, {
                    let a239 = 66u64;
                    let a240 = v2;
                    nat_add(a239, a240)?
                }, 0u64))
            } else {
                Ok(Adt1::C6({
                    let a241 = 2u64;
                    let a242 = v1;
                    nat_add(a241, a242)?
                }, 1u64, {
                    let a243 = 66u64;
                    let a244 = v2;
                    nat_add(a243, a244)?
                }, 1u64, {
                    let a245 = 74u64;
                    let a246 = v2;
                    nat_add(a245, a246)?
                }, {
                    let a247 = 34u64;
                    let a248 = v1;
                    nat_add(a247, a248)?
                }))
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a249 = v1;
        let a250 = 8u64;
        let a251 = 0u64;
        nat_rem(a249, a250, a251)
    };
    let v7: u64 = {
        let a255 = 4u64;
        let a256 = {
            let a252 = v6;
            let a253 = 2u64;
            let a254 = 0u64;
            nat_rem(a252, a253, a254)
        };
        nat_mul(a255, a256)?
    };
    if v0 {
        let v8: u64 = {
            let a257 = v6;
            let a258 = 2u64;
            if nat_lt(a257, a258) {
                1u64
            } else {
                let a259 = v6;
                let a260 = 4u64;
                if nat_lt(a259, a260) {
                    128u64
                } else {
                    let a261 = v6;
                    let a262 = 6u64;
                    if nat_lt(a261, a262) {
                        16384u64
                    } else {
                        2097152u64
                    }
                }
            }
        };
        Ok(Adt1::C10({
            let a265 = 15496u64;
            let a266 = {
                let a263 = 4u64;
                let a264 = v2;
                nat_mul(a263, a264)?
            };
            nat_add(a265, a266)?
        }, true, {
            let a273 = 8u64;
            let a274 = {
                let a270 = {
                    let a267 = v3;
                    let a268 = v8;
                    let a269 = 0u64;
                    nat_quot(a267, a268, a269)
                };
                let a271 = 128u64;
                let a272 = 0u64;
                nat_rem(a270, a271, a272)
            };
            nat_add(a273, a274)?
        }, {
            let a280 = 1u64;
            let a281 = {
                let a278 = 2u64;
                let a279 = {
                    let a275 = v3;
                    let a276 = 268435456u64;
                    let a277 = 0u64;
                    nat_quot(a275, a276, a277)
                };
                nat_mul(a278, a279)?
            };
            nat_add(a280, a281)?
        }, v7))
    } else {
        let v9: u64 = {
            let a285 = {
                let a282 = v3;
                let a283 = f8(v6);
                let a284 = 0u64;
                nat_quot(a282, a283, a284)
            };
            let a286 = 2u64;
            let a287 = 0u64;
            nat_rem(a285, a286, a287)
        };
        let v10: u64 = {
            let a296 = {
                let a293 = v4;
                let a294 = {
                    let a291 = {
                        let a288 = v1;
                        let a289 = 16u64;
                        let a290 = 0u64;
                        nat_rem(a288, a289, a290)
                    };
                    let a292 = 8u64;
                    if nat_lt(a291, a292) {
                        1u64
                    } else {
                        16u64
                    }
                };
                let a295 = 0u64;
                nat_quot(a293, a294, a295)
            };
            let a297 = 16u64;
            let a298 = 0u64;
            nat_rem(a296, a297, a298)
        };
        Ok(Adt1::C10({
            let a305 = 16520u64;
            let a306 = {
                let a303 = 4u64;
                let a304 = {
                    let a301 = v2;
                    let a302 = {
                        let a299 = 256u64;
                        let a300 = v9;
                        nat_mul(a299, a300)?
                    };
                    nat_add(a301, a302)?
                };
                nat_mul(a303, a304)?
            };
            nat_add(a305, a306)?
        }, false, v5, {
            let a309 = 1u64;
            let a310 = {
                let a307 = 2u64;
                let a308 = v10;
                nat_mul(a307, a308)?
            };
            nat_add(a309, a310)?
        }, v7))
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a311 = v1;
        let a312 = 4u64;
        let a313 = 0u64;
        nat_rem(a311, a312, a313)
    };
    let a314 = v0;
    let a315 = 0u64;
    if nat_eq(a314, a315) {
        Ok(Adt1::C7({
            let a318 = 1160u64;
            let a319 = {
                let a316 = 8u64;
                let a317 = v2;
                nat_mul(a316, a317)?
            };
            nat_add(a318, a319)?
        }, true, {
            let a332 = 8u64;
            let a333 = {
                let a329 = {
                    let a326 = v3;
                    let a327 = {
                        let a320 = v6;
                        let a321 = 0u64;
                        if nat_eq(a320, a321) {
                            1u64
                        } else {
                            let a322 = v6;
                            let a323 = 1u64;
                            if nat_eq(a322, a323) {
                                128u64
                            } else {
                                let a324 = v6;
                                let a325 = 2u64;
                                if nat_eq(a324, a325) {
                                    16384u64
                                } else {
                                    2097152u64
                                }
                            }
                        }
                    };
                    let a328 = 0u64;
                    nat_quot(a326, a327, a328)
                };
                let a330 = 128u64;
                let a331 = 0u64;
                nat_rem(a329, a330, a331)
            };
            nat_add(a332, a333)?
        }, {
            let a339 = 1u64;
            let a340 = {
                let a337 = 2u64;
                let a338 = {
                    let a334 = v3;
                    let a335 = 268435456u64;
                    let a336 = 0u64;
                    nat_quot(a334, a335, a336)
                };
                nat_mul(a337, a338)?
            };
            nat_add(a339, a340)?
        }))
    } else {
        let a341 = v0;
        let a342 = 1u64;
        if nat_eq(a341, a342) {
            Ok(Adt1::C7({
                let a348 = 3208u64;
                let a349 = {
                    let a346 = 8u64;
                    let a347 = {
                        let a343 = v2;
                        let a344 = 512u64;
                        let a345 = 0u64;
                        nat_rem(a343, a344, a345)
                    };
                    nat_mul(a346, a347)?
                };
                nat_add(a348, a349)?
            }, true, {
                let a353 = 8u64;
                let a354 = {
                    let a350 = v2;
                    let a351 = 512u64;
                    let a352 = 0u64;
                    nat_quot(a350, a351, a352)
                };
                nat_add(a353, a354)?
            }, {
                let a365 = 1u64;
                let a366 = {
                    let a363 = 2u64;
                    let a364 = {
                        let a360 = {
                            let a357 = v4;
                            let a358 = {
                                let a355 = v6;
                                let a356 = 2u64;
                                if nat_lt(a355, a356) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a359 = 0u64;
                            nat_quot(a357, a358, a359)
                        };
                        let a361 = 16u64;
                        let a362 = 0u64;
                        nat_rem(a360, a361, a362)
                    };
                    nat_mul(a363, a364)?
                };
                nat_add(a365, a366)?
            }))
        } else {
            Ok(Adt1::C7({
                let a385 = 7304u64;
                let a386 = {
                    let a383 = 8u64;
                    let a384 = {
                        let a381 = v2;
                        let a382 = {
                            let a379 = 256u64;
                            let a380 = {
                                let a376 = {
                                    let a373 = v3;
                                    let a374 = {
                                        let a367 = v6;
                                        let a368 = 0u64;
                                        if nat_eq(a367, a368) {
                                            1u64
                                        } else {
                                            let a369 = v6;
                                            let a370 = 1u64;
                                            if nat_eq(a369, a370) {
                                                4u64
                                            } else {
                                                let a371 = v6;
                                                let a372 = 2u64;
                                                if nat_eq(a371, a372) {
                                                    16u64
                                                } else {
                                                    64u64
                                                }
                                            }
                                        }
                                    };
                                    let a375 = 0u64;
                                    nat_quot(a373, a374, a375)
                                };
                                let a377 = 4u64;
                                let a378 = 0u64;
                                nat_rem(a376, a377, a378)
                            };
                            nat_mul(a379, a380)?
                        };
                        nat_add(a381, a382)?
                    };
                    nat_mul(a383, a384)?
                };
                nat_add(a385, a386)?
            }, false, v5, {
                let a397 = 1u64;
                let a398 = {
                    let a395 = 2u64;
                    let a396 = {
                        let a392 = {
                            let a389 = v4;
                            let a390 = {
                                let a387 = v6;
                                let a388 = 2u64;
                                if nat_lt(a387, a388) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a391 = 0u64;
                            nat_quot(a389, a390, a391)
                        };
                        let a393 = 16u64;
                        let a394 = 0u64;
                        nat_rem(a392, a393, a394)
                    };
                    nat_mul(a395, a396)?
                };
                nat_add(a397, a398)?
            }))
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let v4: u64 = {
        let a399 = v2;
        let a400 = v3;
        nat_mul(a399, a400)?
    };
    Ok(Adt1::C1({
        let a407 = {
            let a404 = {
                let a401 = v1;
                let a402 = f8(v0);
                let a403 = 0u64;
                nat_quot(a401, a402, a403)
            };
            let a405 = 2u64;
            let a406 = 0u64;
            nat_rem(a404, a405, a406)
        };
        let a408 = 0u64;
        if nat_eq(a407, a408) {
            v4
        } else {
            let a409 = 65536u64;
            let a410 = v4;
            nat_sub(a409, a410)
        }
    }))
}

pub fn f8(v0: u64) -> u64 {
    let a411 = v0;
    let a412 = 0u64;
    if nat_eq(a411, a412) {
        1u64
    } else {
        let a413 = v0;
        let a414 = 1u64;
        if nat_eq(a413, a414) {
            2u64
        } else {
            let a415 = v0;
            let a416 = 2u64;
            if nat_eq(a415, a416) {
                4u64
            } else {
                let a417 = v0;
                let a418 = 3u64;
                if nat_eq(a417, a418) {
                    8u64
                } else {
                    let a419 = v0;
                    let a420 = 4u64;
                    if nat_eq(a419, a420) {
                        16u64
                    } else {
                        let a421 = v0;
                        let a422 = 5u64;
                        if nat_eq(a421, a422) {
                            32u64
                        } else {
                            let a423 = v0;
                            let a424 = 6u64;
                            if nat_eq(a423, a424) {
                                64u64
                            } else {
                                128u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f9(v0: bool, v1: u64) -> R<Adt1> {
    let a425 = v1;
    let a426 = 256u64;
    if nat_lt(a425, a426) {
        let v2: u64 = {
            let a427 = v1;
            let a428 = 128u64;
            let a429 = 0u64;
            nat_quot(a427, a428, a429)
        };
        let v3: u64 = {
            let a433 = {
                let a430 = v1;
                let a431 = 32u64;
                let a432 = 0u64;
                nat_quot(a430, a431, a432)
            };
            let a434 = 4u64;
            let a435 = 0u64;
            nat_rem(a433, a434, a435)
        };
        let v4: u64 = {
            let a436 = v1;
            let a437 = 32u64;
            let a438 = 0u64;
            nat_rem(a436, a437, a438)
        };
        let v5: u64 = {
            let a439 = v1;
            let a440 = 16u64;
            let a441 = 0u64;
            nat_quot(a439, a440, a441)
        };
        let v6: u64 = if v0 {
            let a451 = {
                let a442 = v2;
                let a443 = 64u64;
                nat_mul(a442, a443)?
            };
            let a452 = {
                let a449 = {
                    let a447 = {
                        let a444 = v3;
                        let a445 = 2u64;
                        let a446 = 0u64;
                        nat_rem(a444, a445, a446)
                    };
                    let a448 = 32u64;
                    nat_mul(a447, a448)?
                };
                let a450 = v4;
                nat_add(a449, a450)?
            };
            nat_add(a451, a452)?
        } else {
            let a457 = 32u64;
            let a458 = {
                let a455 = {
                    let a453 = v2;
                    let a454 = 32u64;
                    nat_mul(a453, a454)?
                };
                let a456 = v4;
                nat_add(a455, a456)?
            };
            nat_add(a457, a458)?
        };
        let v7: u64 = if v0 {
            let a463 = 128u64;
            let a464 = {
                let a461 = {
                    let a459 = v2;
                    let a460 = 32u64;
                    nat_mul(a459, a460)?
                };
                let a462 = v4;
                nat_add(a461, a462)?
            };
            nat_add(a463, a464)?
        } else {
            v4
        };
        let v8: u64 = if v0 {
            let a465 = 192u64;
            let a466 = v5;
            nat_add(a465, a466)?
        } else {
            let a470 = 96u64;
            let a471 = {
                let a467 = v5;
                let a468 = 8u64;
                let a469 = 0u64;
                nat_rem(a467, a468, a469)
            };
            nat_add(a470, a471)?
        };
        let v9: u64 = if v0 {
            v8
        } else {
            let a475 = 104u64;
            let a476 = {
                let a472 = v5;
                let a473 = 4u64;
                let a474 = 0u64;
                nat_rem(a472, a473, a474)
            };
            nat_add(a475, a476)?
        };
        Ok(Adt1::C8(v6, v7, v8, v9))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f10(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let a477 = v1;
    let a478 = 256u64;
    if nat_lt(a477, a478) {
        let a479 = v2;
        let a480 = 256u64;
        if if if if nat_lt(a479, a480) {
            let a481 = v3;
            let a482 = 256u64;
            nat_lt(a481, a482)
        } else {
            false
        } {
            let a483 = v4;
            let a484 = 256u64;
            nat_lt(a483, a484)
        } else {
            false
        } {
            let a485 = v5;
            let a486 = 256u64;
            nat_lt(a485, a486)
        } else {
            false
        } {
            let v6: u64 = {
                let a490 = {
                    let a487 = v1;
                    let a488 = 32u64;
                    let a489 = 0u64;
                    nat_quot(a487, a488, a489)
                };
                let a491 = 4u64;
                let a492 = 0u64;
                nat_rem(a490, a491, a492)
            };
            let v7: u64 = {
                let a493 = v1;
                let a494 = 16u64;
                let a495 = 0u64;
                nat_quot(a493, a494, a495)
            };
            let v8: u64 = f8({
                let a496 = v6;
                let a497 = 2u64;
                nat_mul(a496, a497)?
            });
            let v9: u64 = if v0 {
                let a501 = {
                    let a498 = v3;
                    let a499 = v8;
                    let a500 = 0u64;
                    nat_quot(a498, a499, a500)
                };
                let a502 = 4u64;
                let a503 = 0u64;
                nat_rem(a501, a502, a503)
            } else {
                let a510 = {
                    let a507 = v3;
                    let a508 = f8({
                        let a504 = v1;
                        let a505 = 32u64;
                        let a506 = 0u64;
                        nat_quot(a504, a505, a506)
                    });
                    let a509 = 0u64;
                    nat_quot(a507, a508, a509)
                };
                let a511 = 2u64;
                let a512 = 0u64;
                nat_rem(a510, a511, a512)
            };
            let v10: u64 = if v0 {
                let a523 = {
                    let a518 = {
                        let a515 = v2;
                        let a516 = {
                            let a513 = v6;
                            let a514 = 2u64;
                            if nat_lt(a513, a514) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a517 = 0u64;
                        nat_quot(a515, a516, a517)
                    };
                    let a519 = 16u64;
                    let a520 = 0u64;
                    nat_rem(a518, a519, a520)
                };
                let a524 = {
                    let a521 = v9;
                    let a522 = 16u64;
                    nat_mul(a521, a522)?
                };
                nat_add(a523, a524)?
            } else {
                let a533 = {
                    let a528 = {
                        let a525 = v2;
                        let a526 = v8;
                        let a527 = 0u64;
                        nat_quot(a525, a526, a527)
                    };
                    let a529 = 4u64;
                    let a530 = 0u64;
                    nat_rem(a528, a529, a530)
                };
                let a534 = {
                    let a531 = v9;
                    let a532 = 4u64;
                    nat_mul(a531, a532)?
                };
                nat_add(a533, a534)?
            };
            let v11: u64 = if v0 {
                32u64
            } else {
                4u64
            };
            let v12: u64 = if v0 {
                let a537 = {
                    let a535 = v4;
                    let a536 = 128u64;
                    nat_add(a535, a536)?
                };
                let a538 = 256u64;
                let a539 = 0u64;
                nat_rem(a537, a538, a539)
            } else {
                let a561 = {
                    let a545 = {
                        let a542 = v4;
                        let a543 = {
                            let a540 = v7;
                            let a541 = 8u64;
                            if nat_lt(a540, a541) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a544 = 0u64;
                        nat_quot(a542, a543, a544)
                    };
                    let a546 = 16u64;
                    let a547 = 0u64;
                    nat_rem(a545, a546, a547)
                };
                let a562 = {
                    let a559 = 16u64;
                    let a560 = {
                        let a556 = {
                            let a553 = v5;
                            let a554 = f8({
                                let a551 = {
                                    let a548 = v7;
                                    let a549 = 4u64;
                                    let a550 = 0u64;
                                    nat_quot(a548, a549, a550)
                                };
                                let a552 = 2u64;
                                nat_mul(a551, a552)?
                            });
                            let a555 = 0u64;
                            nat_quot(a553, a554, a555)
                        };
                        let a557 = 4u64;
                        let a558 = 0u64;
                        nat_rem(a556, a557, a558)
                    };
                    nat_mul(a559, a560)?
                };
                nat_add(a561, a562)?
            };
            let v13: u64 = if v0 {
                128u64
            } else {
                32u64
            };
            let v14: u64 = {
                let a563 = v10;
                let a564 = v11;
                if nat_lt(a563, a564) {
                    let a565 = v11;
                    let a566 = v10;
                    nat_sub(a565, a566)
                } else {
                    let a567 = v10;
                    let a568 = v11;
                    nat_sub(a567, a568)
                }
            };
            let v15: u64 = {
                let a569 = v12;
                let a570 = v13;
                if nat_lt(a569, a570) {
                    let a571 = v13;
                    let a572 = v12;
                    nat_sub(a571, a572)
                } else {
                    let a573 = v12;
                    let a574 = v13;
                    nat_sub(a573, a574)
                }
            };
            let v16: u64 = {
                let a575 = v14;
                let a576 = v15;
                nat_mul(a575, a576)?
            };
            Ok(Adt1::C1({
                let a577 = v10;
                let a578 = v11;
                if if nat_lt(a577, a578) {
                    let a579 = v12;
                    let a580 = v13;
                    nat_lt(a579, a580)
                } else {
                    let a581 = v12;
                    let a582 = v13;
                    !nat_lt(a581, a582)
                } {
                    v16
                } else {
                    let a585 = {
                        let a583 = 65536u64;
                        let a584 = v16;
                        nat_sub(a583, a584)
                    };
                    let a586 = 65536u64;
                    let a587 = 0u64;
                    nat_rem(a585, a586, a587)
                }
            }))
        } else {
            Ok(Adt1::C0)
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt1> {
    f0(p0)
}
