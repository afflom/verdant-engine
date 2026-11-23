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
        Adt0::C4(v56) => {
            Ok(Adt1::C4(2u64, if v56 {
                1u64
            } else {
                0u64
            }))
        }
        Adt0::C5(v19, v20, v21) => {
            f2(v19, v20, v21)
        }
        Adt0::C6(v22, v23) => {
            f3(v22, v23)
        }
        Adt0::C7(v24, v25, v26, v27, v28, v29) => {
            f4(v24, v25, v26, v27, v28, v29)
        }
        Adt0::C8(v30, v31, v32, v33) => {
            f5(v30, v31, v32, v33)
        }
        Adt0::C9(v44, v45, v46) => {
            let v47: u64 = if v44 {
                210u64
            } else {
                110u64
            };
            let a96 = v45;
            let a97 = {
                let a94 = v47;
                let a95 = v46;
                nat_mul(a94, a95)?
            };
            if nat_eq(a96, a97) {
                Ok(Adt1::C1(v47))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C10(v48, v49) => {
            f7(v48, v49)
        }
        Adt0::C11(v50, v51, v52, v53, v54, v55) => {
            f8(v50, v51, v52, v53, v54, v55)
        }
        Adt0::C12(v34, v35) => {
            let a39 = v34;
            let a40 = {
                let a37 = 136u64;
                let a38 = v35;
                nat_mul(a37, a38)?
            };
            if nat_eq(a39, a40) {
                Ok(Adt1::C5(136u64, 1065353216u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C13(v36) => {
            let a41 = v36;
            let a42 = 8u64;
            if nat_lt(a41, a42) {
                Ok(Adt1::C2({
                    let a46 = 4u64;
                    let a47 = {
                        let a43 = v36;
                        let a44 = 2u64;
                        let a45 = 0u64;
                        nat_quot(a43, a44, a45)
                    };
                    nat_add(a46, a47)?
                }, {
                    let a51 = 2u64;
                    let a52 = {
                        let a48 = v36;
                        let a49 = 4u64;
                        let a50 = 0u64;
                        nat_quot(a48, a49, a50)
                    };
                    nat_add(a51, a52)?
                }, {
                    let a55 = 8u64;
                    let a56 = {
                        let a53 = v36;
                        let a54 = 16u64;
                        nat_mul(a53, a54)?
                    };
                    nat_add(a55, a56)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C14(v37, v38, v39) => {
            let a57 = v37;
            let a58 = 8u64;
            if if if nat_lt(a57, a58) {
                let a59 = v38;
                let a60 = 256u64;
                nat_lt(a59, a60)
            } else {
                false
            } {
                let a61 = v39;
                let a62 = 256u64;
                nat_lt(a61, a62)
            } else {
                false
            } {
                let v40: u64 = {
                    let a66 = {
                        let a63 = v37;
                        let a64 = 2u64;
                        let a65 = 0u64;
                        nat_rem(a63, a64, a65)
                    };
                    let a67 = 0u64;
                    if nat_eq(a66, a67) {
                        1u64
                    } else {
                        16u64
                    }
                };
                let v41: u64 = {
                    let a71 = 2u64;
                    let a72 = {
                        let a68 = v37;
                        let a69 = 4u64;
                        let a70 = 0u64;
                        nat_rem(a68, a69, a70)
                    };
                    nat_mul(a71, a72)?
                };
                let v42: u64 = {
                    let a76 = {
                        let a73 = v38;
                        let a74 = v40;
                        let a75 = 0u64;
                        nat_quot(a73, a74, a75)
                    };
                    let a77 = 16u64;
                    let a78 = 0u64;
                    nat_rem(a76, a77, a78)
                };
                let v43: u64 = {
                    let a82 = {
                        let a79 = v39;
                        let a80 = f6(v41);
                        let a81 = 0u64;
                        nat_quot(a79, a80, a81)
                    };
                    let a83 = 4u64;
                    let a84 = 0u64;
                    nat_rem(a82, a83, a84)
                };
                Ok(Adt1::C1({
                    let a91 = {
                        let a89 = 224u64;
                        let a90 = {
                            let a87 = v42;
                            let a88 = {
                                let a85 = 16u64;
                                let a86 = v43;
                                nat_mul(a85, a86)?
                            };
                            nat_add(a87, a88)?
                        };
                        nat_add(a89, a90)?
                    };
                    let a92 = 256u64;
                    let a93 = 0u64;
                    nat_rem(a91, a92, a93)
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a98 = v1;
    let a99 = 8u64;
    if if if if nat_lt(a98, a99) {
        let a100 = v2;
        let a101 = 256u64;
        nat_lt(a100, a101)
    } else {
        false
    } {
        let a102 = v3;
        let a103 = 256u64;
        nat_lt(a102, a103)
    } else {
        false
    } {
        let a104 = v4;
        let a105 = 256u64;
        nat_lt(a104, a105)
    } else {
        false
    } {
        let v5: u64 = {
            let a106 = v1;
            let a107 = 4u64;
            if nat_lt(a106, a107) {
                let a108 = v2;
                let a109 = 64u64;
                let a110 = 0u64;
                nat_rem(a108, a109, a110)
            } else {
                let a119 = {
                    let a111 = v4;
                    let a112 = 16u64;
                    let a113 = 0u64;
                    nat_rem(a111, a112, a113)
                };
                let a120 = {
                    let a117 = 16u64;
                    let a118 = {
                        let a114 = v2;
                        let a115 = 64u64;
                        let a116 = 0u64;
                        nat_quot(a114, a115, a116)
                    };
                    nat_mul(a117, a118)?
                };
                nat_add(a119, a120)?
            }
        };
        let v6: u64 = {
            let a121 = v1;
            let a122 = 4u64;
            if nat_lt(a121, a122) {
                let a123 = v3;
                let a124 = 64u64;
                let a125 = 0u64;
                nat_rem(a123, a124, a125)
            } else {
                let a134 = {
                    let a126 = v4;
                    let a127 = 16u64;
                    let a128 = 0u64;
                    nat_quot(a126, a127, a128)
                };
                let a135 = {
                    let a132 = 16u64;
                    let a133 = {
                        let a129 = v3;
                        let a130 = 64u64;
                        let a131 = 0u64;
                        nat_quot(a129, a130, a131)
                    };
                    nat_mul(a132, a133)?
                };
                nat_add(a134, a135)?
            }
        };
        Ok(Adt1::C3({
            let a141 = if v0 {
                48u64
            } else {
                16u64
            };
            let a142 = {
                let a139 = 32u64;
                let a140 = {
                    let a136 = v1;
                    let a137 = 2u64;
                    let a138 = 0u64;
                    nat_quot(a136, a137, a138)
                };
                nat_mul(a139, a140)?
            };
            nat_add(a141, a142)?
        }, {
            let a146 = {
                let a143 = v1;
                let a144 = 2u64;
                let a145 = 0u64;
                nat_rem(a143, a144, a145)
            };
            let a147 = 0u64;
            if nat_eq(a146, a147) {
                1u64
            } else {
                16u64
            }
        }, if v0 {
            f6(v1)
        } else {
            0u64
        }, v5, v6))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = {
        let a148 = v0;
        let a149 = 0u64;
        if nat_eq(a148, a149) {
            66u64
        } else {
            let a150 = v0;
            let a151 = 1u64;
            if nat_eq(a150, a151) {
                74u64
            } else {
                82u64
            }
        }
    };
    let a152 = v0;
    let a153 = 3u64;
    if if nat_lt(a152, a153) {
        let a156 = v1;
        let a157 = {
            let a154 = v3;
            let a155 = v2;
            nat_mul(a154, a155)?
        };
        nat_eq(a156, a157)
    } else {
        false
    } {
        Ok(Adt1::C5(v3, 1040187392u64))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: u64, v1: u64) -> R<Adt1> {
    let a158 = v0;
    let a159 = 3u64;
    if if nat_lt(a158, a159) {
        let a160 = v1;
        let a161 = 32u64;
        nat_lt(a160, a161)
    } else {
        false
    } {
        let v2: u64 = {
            let a162 = v1;
            let a163 = 4u64;
            let a164 = 0u64;
            nat_quot(a162, a163, a164)
        };
        let v3: u64 = {
            let a165 = v1;
            let a166 = 4u64;
            let a167 = 0u64;
            nat_rem(a165, a166, a167)
        };
        let a168 = v0;
        let a169 = 0u64;
        if nat_eq(a168, a169) {
            let v4: u64 = {
                let a172 = 2u64;
                let a173 = {
                    let a170 = 8u64;
                    let a171 = v2;
                    nat_mul(a170, a171)?
                };
                nat_add(a172, a173)?
            };
            Ok(Adt1::C6({
                let a174 = v4;
                let a175 = v3;
                nat_add(a174, a175)?
            }, 1u64, {
                let a176 = v4;
                let a177 = 4u64;
                nat_add(a176, a177)?
            }, 4u64, v4, v4))
        } else {
            let a178 = v0;
            let a179 = 1u64;
            if nat_eq(a178, a179) {
                Ok(Adt1::C6({
                    let a182 = 2u64;
                    let a183 = {
                        let a180 = 2u64;
                        let a181 = v1;
                        nat_mul(a180, a181)?
                    };
                    nat_add(a182, a183)?
                }, 2u64, 0u64, 1u64, {
                    let a184 = 66u64;
                    let a185 = v2;
                    nat_add(a184, a185)?
                }, 0u64))
            } else {
                Ok(Adt1::C6({
                    let a186 = 2u64;
                    let a187 = v1;
                    nat_add(a186, a187)?
                }, 1u64, {
                    let a188 = 66u64;
                    let a189 = v2;
                    nat_add(a188, a189)?
                }, 1u64, {
                    let a190 = 74u64;
                    let a191 = v2;
                    nat_add(a190, a191)?
                }, {
                    let a192 = 34u64;
                    let a193 = v1;
                    nat_add(a192, a193)?
                }))
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a194 = v1;
        let a195 = 4u64;
        let a196 = 0u64;
        nat_rem(a194, a195, a196)
    };
    let a197 = v0;
    let a198 = 0u64;
    if nat_eq(a197, a198) {
        Ok(Adt1::C7({
            let a201 = 1160u64;
            let a202 = {
                let a199 = 8u64;
                let a200 = v2;
                nat_mul(a199, a200)?
            };
            nat_add(a201, a202)?
        }, true, {
            let a215 = 8u64;
            let a216 = {
                let a212 = {
                    let a209 = v3;
                    let a210 = {
                        let a203 = v6;
                        let a204 = 0u64;
                        if nat_eq(a203, a204) {
                            1u64
                        } else {
                            let a205 = v6;
                            let a206 = 1u64;
                            if nat_eq(a205, a206) {
                                128u64
                            } else {
                                let a207 = v6;
                                let a208 = 2u64;
                                if nat_eq(a207, a208) {
                                    16384u64
                                } else {
                                    2097152u64
                                }
                            }
                        }
                    };
                    let a211 = 0u64;
                    nat_quot(a209, a210, a211)
                };
                let a213 = 128u64;
                let a214 = 0u64;
                nat_rem(a212, a213, a214)
            };
            nat_add(a215, a216)?
        }, {
            let a222 = 1u64;
            let a223 = {
                let a220 = 2u64;
                let a221 = {
                    let a217 = v3;
                    let a218 = 268435456u64;
                    let a219 = 0u64;
                    nat_quot(a217, a218, a219)
                };
                nat_mul(a220, a221)?
            };
            nat_add(a222, a223)?
        }))
    } else {
        let a224 = v0;
        let a225 = 1u64;
        if nat_eq(a224, a225) {
            Ok(Adt1::C7({
                let a231 = 3208u64;
                let a232 = {
                    let a229 = 8u64;
                    let a230 = {
                        let a226 = v2;
                        let a227 = 512u64;
                        let a228 = 0u64;
                        nat_rem(a226, a227, a228)
                    };
                    nat_mul(a229, a230)?
                };
                nat_add(a231, a232)?
            }, true, {
                let a236 = 8u64;
                let a237 = {
                    let a233 = v2;
                    let a234 = 512u64;
                    let a235 = 0u64;
                    nat_quot(a233, a234, a235)
                };
                nat_add(a236, a237)?
            }, {
                let a248 = 1u64;
                let a249 = {
                    let a246 = 2u64;
                    let a247 = {
                        let a243 = {
                            let a240 = v4;
                            let a241 = {
                                let a238 = v6;
                                let a239 = 2u64;
                                if nat_lt(a238, a239) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a242 = 0u64;
                            nat_quot(a240, a241, a242)
                        };
                        let a244 = 16u64;
                        let a245 = 0u64;
                        nat_rem(a243, a244, a245)
                    };
                    nat_mul(a246, a247)?
                };
                nat_add(a248, a249)?
            }))
        } else {
            Ok(Adt1::C7({
                let a268 = 7304u64;
                let a269 = {
                    let a266 = 8u64;
                    let a267 = {
                        let a264 = v2;
                        let a265 = {
                            let a262 = 256u64;
                            let a263 = {
                                let a259 = {
                                    let a256 = v3;
                                    let a257 = {
                                        let a250 = v6;
                                        let a251 = 0u64;
                                        if nat_eq(a250, a251) {
                                            1u64
                                        } else {
                                            let a252 = v6;
                                            let a253 = 1u64;
                                            if nat_eq(a252, a253) {
                                                4u64
                                            } else {
                                                let a254 = v6;
                                                let a255 = 2u64;
                                                if nat_eq(a254, a255) {
                                                    16u64
                                                } else {
                                                    64u64
                                                }
                                            }
                                        }
                                    };
                                    let a258 = 0u64;
                                    nat_quot(a256, a257, a258)
                                };
                                let a260 = 4u64;
                                let a261 = 0u64;
                                nat_rem(a259, a260, a261)
                            };
                            nat_mul(a262, a263)?
                        };
                        nat_add(a264, a265)?
                    };
                    nat_mul(a266, a267)?
                };
                nat_add(a268, a269)?
            }, false, v5, {
                let a280 = 1u64;
                let a281 = {
                    let a278 = 2u64;
                    let a279 = {
                        let a275 = {
                            let a272 = v4;
                            let a273 = {
                                let a270 = v6;
                                let a271 = 2u64;
                                if nat_lt(a270, a271) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a274 = 0u64;
                            nat_quot(a272, a273, a274)
                        };
                        let a276 = 16u64;
                        let a277 = 0u64;
                        nat_rem(a275, a276, a277)
                    };
                    nat_mul(a278, a279)?
                };
                nat_add(a280, a281)?
            }))
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let v4: u64 = {
        let a282 = v2;
        let a283 = v3;
        nat_mul(a282, a283)?
    };
    Ok(Adt1::C1({
        let a290 = {
            let a287 = {
                let a284 = v1;
                let a285 = f6(v0);
                let a286 = 0u64;
                nat_quot(a284, a285, a286)
            };
            let a288 = 2u64;
            let a289 = 0u64;
            nat_rem(a287, a288, a289)
        };
        let a291 = 0u64;
        if nat_eq(a290, a291) {
            v4
        } else {
            let a292 = 65536u64;
            let a293 = v4;
            nat_sub(a292, a293)
        }
    }))
}

pub fn f6(v0: u64) -> u64 {
    let a294 = v0;
    let a295 = 0u64;
    if nat_eq(a294, a295) {
        1u64
    } else {
        let a296 = v0;
        let a297 = 1u64;
        if nat_eq(a296, a297) {
            2u64
        } else {
            let a298 = v0;
            let a299 = 2u64;
            if nat_eq(a298, a299) {
                4u64
            } else {
                let a300 = v0;
                let a301 = 3u64;
                if nat_eq(a300, a301) {
                    8u64
                } else {
                    let a302 = v0;
                    let a303 = 4u64;
                    if nat_eq(a302, a303) {
                        16u64
                    } else {
                        let a304 = v0;
                        let a305 = 5u64;
                        if nat_eq(a304, a305) {
                            32u64
                        } else {
                            let a306 = v0;
                            let a307 = 6u64;
                            if nat_eq(a306, a307) {
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

pub fn f7(v0: bool, v1: u64) -> R<Adt1> {
    let a308 = v1;
    let a309 = 256u64;
    if nat_lt(a308, a309) {
        let v2: u64 = {
            let a310 = v1;
            let a311 = 128u64;
            let a312 = 0u64;
            nat_quot(a310, a311, a312)
        };
        let v3: u64 = {
            let a316 = {
                let a313 = v1;
                let a314 = 32u64;
                let a315 = 0u64;
                nat_quot(a313, a314, a315)
            };
            let a317 = 4u64;
            let a318 = 0u64;
            nat_rem(a316, a317, a318)
        };
        let v4: u64 = {
            let a319 = v1;
            let a320 = 32u64;
            let a321 = 0u64;
            nat_rem(a319, a320, a321)
        };
        let v5: u64 = {
            let a322 = v1;
            let a323 = 16u64;
            let a324 = 0u64;
            nat_quot(a322, a323, a324)
        };
        let v6: u64 = if v0 {
            let a334 = {
                let a325 = v2;
                let a326 = 64u64;
                nat_mul(a325, a326)?
            };
            let a335 = {
                let a332 = {
                    let a330 = {
                        let a327 = v3;
                        let a328 = 2u64;
                        let a329 = 0u64;
                        nat_rem(a327, a328, a329)
                    };
                    let a331 = 32u64;
                    nat_mul(a330, a331)?
                };
                let a333 = v4;
                nat_add(a332, a333)?
            };
            nat_add(a334, a335)?
        } else {
            let a340 = 32u64;
            let a341 = {
                let a338 = {
                    let a336 = v2;
                    let a337 = 32u64;
                    nat_mul(a336, a337)?
                };
                let a339 = v4;
                nat_add(a338, a339)?
            };
            nat_add(a340, a341)?
        };
        let v7: u64 = if v0 {
            let a346 = 128u64;
            let a347 = {
                let a344 = {
                    let a342 = v2;
                    let a343 = 32u64;
                    nat_mul(a342, a343)?
                };
                let a345 = v4;
                nat_add(a344, a345)?
            };
            nat_add(a346, a347)?
        } else {
            v4
        };
        let v8: u64 = if v0 {
            let a348 = 192u64;
            let a349 = v5;
            nat_add(a348, a349)?
        } else {
            let a353 = 96u64;
            let a354 = {
                let a350 = v5;
                let a351 = 8u64;
                let a352 = 0u64;
                nat_rem(a350, a351, a352)
            };
            nat_add(a353, a354)?
        };
        let v9: u64 = if v0 {
            v8
        } else {
            let a358 = 104u64;
            let a359 = {
                let a355 = v5;
                let a356 = 4u64;
                let a357 = 0u64;
                nat_rem(a355, a356, a357)
            };
            nat_add(a358, a359)?
        };
        Ok(Adt1::C8(v6, v7, v8, v9))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f8(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let a360 = v1;
    let a361 = 256u64;
    if nat_lt(a360, a361) {
        let a362 = v2;
        let a363 = 256u64;
        if if if if nat_lt(a362, a363) {
            let a364 = v3;
            let a365 = 256u64;
            nat_lt(a364, a365)
        } else {
            false
        } {
            let a366 = v4;
            let a367 = 256u64;
            nat_lt(a366, a367)
        } else {
            false
        } {
            let a368 = v5;
            let a369 = 256u64;
            nat_lt(a368, a369)
        } else {
            false
        } {
            let v6: u64 = {
                let a373 = {
                    let a370 = v1;
                    let a371 = 32u64;
                    let a372 = 0u64;
                    nat_quot(a370, a371, a372)
                };
                let a374 = 4u64;
                let a375 = 0u64;
                nat_rem(a373, a374, a375)
            };
            let v7: u64 = {
                let a376 = v1;
                let a377 = 16u64;
                let a378 = 0u64;
                nat_quot(a376, a377, a378)
            };
            let v8: u64 = f6({
                let a379 = v6;
                let a380 = 2u64;
                nat_mul(a379, a380)?
            });
            let v9: u64 = if v0 {
                let a384 = {
                    let a381 = v3;
                    let a382 = v8;
                    let a383 = 0u64;
                    nat_quot(a381, a382, a383)
                };
                let a385 = 4u64;
                let a386 = 0u64;
                nat_rem(a384, a385, a386)
            } else {
                let a393 = {
                    let a390 = v3;
                    let a391 = f6({
                        let a387 = v1;
                        let a388 = 32u64;
                        let a389 = 0u64;
                        nat_quot(a387, a388, a389)
                    });
                    let a392 = 0u64;
                    nat_quot(a390, a391, a392)
                };
                let a394 = 2u64;
                let a395 = 0u64;
                nat_rem(a393, a394, a395)
            };
            let v10: u64 = if v0 {
                let a406 = {
                    let a401 = {
                        let a398 = v2;
                        let a399 = {
                            let a396 = v6;
                            let a397 = 2u64;
                            if nat_lt(a396, a397) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a400 = 0u64;
                        nat_quot(a398, a399, a400)
                    };
                    let a402 = 16u64;
                    let a403 = 0u64;
                    nat_rem(a401, a402, a403)
                };
                let a407 = {
                    let a404 = v9;
                    let a405 = 16u64;
                    nat_mul(a404, a405)?
                };
                nat_add(a406, a407)?
            } else {
                let a416 = {
                    let a411 = {
                        let a408 = v2;
                        let a409 = v8;
                        let a410 = 0u64;
                        nat_quot(a408, a409, a410)
                    };
                    let a412 = 4u64;
                    let a413 = 0u64;
                    nat_rem(a411, a412, a413)
                };
                let a417 = {
                    let a414 = v9;
                    let a415 = 4u64;
                    nat_mul(a414, a415)?
                };
                nat_add(a416, a417)?
            };
            let v11: u64 = if v0 {
                32u64
            } else {
                4u64
            };
            let v12: u64 = if v0 {
                let a420 = {
                    let a418 = v4;
                    let a419 = 128u64;
                    nat_add(a418, a419)?
                };
                let a421 = 256u64;
                let a422 = 0u64;
                nat_rem(a420, a421, a422)
            } else {
                let a444 = {
                    let a428 = {
                        let a425 = v4;
                        let a426 = {
                            let a423 = v7;
                            let a424 = 8u64;
                            if nat_lt(a423, a424) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a427 = 0u64;
                        nat_quot(a425, a426, a427)
                    };
                    let a429 = 16u64;
                    let a430 = 0u64;
                    nat_rem(a428, a429, a430)
                };
                let a445 = {
                    let a442 = 16u64;
                    let a443 = {
                        let a439 = {
                            let a436 = v5;
                            let a437 = f6({
                                let a434 = {
                                    let a431 = v7;
                                    let a432 = 4u64;
                                    let a433 = 0u64;
                                    nat_quot(a431, a432, a433)
                                };
                                let a435 = 2u64;
                                nat_mul(a434, a435)?
                            });
                            let a438 = 0u64;
                            nat_quot(a436, a437, a438)
                        };
                        let a440 = 4u64;
                        let a441 = 0u64;
                        nat_rem(a439, a440, a441)
                    };
                    nat_mul(a442, a443)?
                };
                nat_add(a444, a445)?
            };
            let v13: u64 = if v0 {
                128u64
            } else {
                32u64
            };
            let v14: u64 = {
                let a446 = v10;
                let a447 = v11;
                if nat_lt(a446, a447) {
                    let a448 = v11;
                    let a449 = v10;
                    nat_sub(a448, a449)
                } else {
                    let a450 = v10;
                    let a451 = v11;
                    nat_sub(a450, a451)
                }
            };
            let v15: u64 = {
                let a452 = v12;
                let a453 = v13;
                if nat_lt(a452, a453) {
                    let a454 = v13;
                    let a455 = v12;
                    nat_sub(a454, a455)
                } else {
                    let a456 = v12;
                    let a457 = v13;
                    nat_sub(a456, a457)
                }
            };
            let v16: u64 = {
                let a458 = v14;
                let a459 = v15;
                nat_mul(a458, a459)?
            };
            Ok(Adt1::C1({
                let a460 = v10;
                let a461 = v11;
                if if nat_lt(a460, a461) {
                    let a462 = v12;
                    let a463 = v13;
                    nat_lt(a462, a463)
                } else {
                    let a464 = v12;
                    let a465 = v13;
                    !nat_lt(a464, a465)
                } {
                    v16
                } else {
                    let a468 = {
                        let a466 = 65536u64;
                        let a467 = v16;
                        nat_sub(a466, a467)
                    };
                    let a469 = 65536u64;
                    let a470 = 0u64;
                    nat_rem(a468, a469, a470)
                }
            }))
        } else {
            Ok(Adt1::C0)
        }
    } else {
        Ok(Adt1::C0)
    }
}
