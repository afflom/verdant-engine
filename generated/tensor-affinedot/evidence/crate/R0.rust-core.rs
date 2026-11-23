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
        Adt0::C4(v46) => {
            Ok(Adt1::C4(2u64, if v46 {
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
        Adt0::C9(v34, v35, v36) => {
            let v37: u64 = if v34 {
                210u64
            } else {
                110u64
            };
            let a39 = v35;
            let a40 = {
                let a37 = v37;
                let a38 = v36;
                nat_mul(a37, a38)?
            };
            if nat_eq(a39, a40) {
                Ok(Adt1::C1(v37))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C10(v38, v39) => {
            f6(v38, v39)
        }
        Adt0::C11(v40, v41, v42, v43, v44, v45) => {
            f7(v40, v41, v42, v43, v44, v45)
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a41 = v1;
    let a42 = 8u64;
    if if if if nat_lt(a41, a42) {
        let a43 = v2;
        let a44 = 256u64;
        nat_lt(a43, a44)
    } else {
        false
    } {
        let a45 = v3;
        let a46 = 256u64;
        nat_lt(a45, a46)
    } else {
        false
    } {
        let a47 = v4;
        let a48 = 256u64;
        nat_lt(a47, a48)
    } else {
        false
    } {
        let v5: u64 = {
            let a49 = v1;
            let a50 = 4u64;
            if nat_lt(a49, a50) {
                let a51 = v2;
                let a52 = 64u64;
                let a53 = 0u64;
                nat_rem(a51, a52, a53)
            } else {
                let a62 = {
                    let a54 = v4;
                    let a55 = 16u64;
                    let a56 = 0u64;
                    nat_rem(a54, a55, a56)
                };
                let a63 = {
                    let a60 = 16u64;
                    let a61 = {
                        let a57 = v2;
                        let a58 = 64u64;
                        let a59 = 0u64;
                        nat_quot(a57, a58, a59)
                    };
                    nat_mul(a60, a61)?
                };
                nat_add(a62, a63)?
            }
        };
        let v6: u64 = {
            let a64 = v1;
            let a65 = 4u64;
            if nat_lt(a64, a65) {
                let a66 = v3;
                let a67 = 64u64;
                let a68 = 0u64;
                nat_rem(a66, a67, a68)
            } else {
                let a77 = {
                    let a69 = v4;
                    let a70 = 16u64;
                    let a71 = 0u64;
                    nat_quot(a69, a70, a71)
                };
                let a78 = {
                    let a75 = 16u64;
                    let a76 = {
                        let a72 = v3;
                        let a73 = 64u64;
                        let a74 = 0u64;
                        nat_quot(a72, a73, a74)
                    };
                    nat_mul(a75, a76)?
                };
                nat_add(a77, a78)?
            }
        };
        Ok(Adt1::C3({
            let a84 = if v0 {
                48u64
            } else {
                16u64
            };
            let a85 = {
                let a82 = 32u64;
                let a83 = {
                    let a79 = v1;
                    let a80 = 2u64;
                    let a81 = 0u64;
                    nat_quot(a79, a80, a81)
                };
                nat_mul(a82, a83)?
            };
            nat_add(a84, a85)?
        }, {
            let a89 = {
                let a86 = v1;
                let a87 = 2u64;
                let a88 = 0u64;
                nat_rem(a86, a87, a88)
            };
            let a90 = 0u64;
            if nat_eq(a89, a90) {
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
        let a91 = v0;
        let a92 = 0u64;
        if nat_eq(a91, a92) {
            66u64
        } else {
            let a93 = v0;
            let a94 = 1u64;
            if nat_eq(a93, a94) {
                74u64
            } else {
                82u64
            }
        }
    };
    let a95 = v0;
    let a96 = 3u64;
    if if nat_lt(a95, a96) {
        let a99 = v1;
        let a100 = {
            let a97 = v3;
            let a98 = v2;
            nat_mul(a97, a98)?
        };
        nat_eq(a99, a100)
    } else {
        false
    } {
        Ok(Adt1::C5(v3, 1040187392u64))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: u64, v1: u64) -> R<Adt1> {
    let a101 = v0;
    let a102 = 3u64;
    if if nat_lt(a101, a102) {
        let a103 = v1;
        let a104 = 32u64;
        nat_lt(a103, a104)
    } else {
        false
    } {
        let v2: u64 = {
            let a105 = v1;
            let a106 = 4u64;
            let a107 = 0u64;
            nat_quot(a105, a106, a107)
        };
        let v3: u64 = {
            let a108 = v1;
            let a109 = 4u64;
            let a110 = 0u64;
            nat_rem(a108, a109, a110)
        };
        let a111 = v0;
        let a112 = 0u64;
        if nat_eq(a111, a112) {
            let v4: u64 = {
                let a115 = 2u64;
                let a116 = {
                    let a113 = 8u64;
                    let a114 = v2;
                    nat_mul(a113, a114)?
                };
                nat_add(a115, a116)?
            };
            Ok(Adt1::C6({
                let a117 = v4;
                let a118 = v3;
                nat_add(a117, a118)?
            }, 1u64, {
                let a119 = v4;
                let a120 = 4u64;
                nat_add(a119, a120)?
            }, 4u64, v4, v4))
        } else {
            let a121 = v0;
            let a122 = 1u64;
            if nat_eq(a121, a122) {
                Ok(Adt1::C6({
                    let a125 = 2u64;
                    let a126 = {
                        let a123 = 2u64;
                        let a124 = v1;
                        nat_mul(a123, a124)?
                    };
                    nat_add(a125, a126)?
                }, 2u64, 0u64, 1u64, {
                    let a127 = 66u64;
                    let a128 = v2;
                    nat_add(a127, a128)?
                }, 0u64))
            } else {
                Ok(Adt1::C6({
                    let a129 = 2u64;
                    let a130 = v1;
                    nat_add(a129, a130)?
                }, 1u64, {
                    let a131 = 66u64;
                    let a132 = v2;
                    nat_add(a131, a132)?
                }, 1u64, {
                    let a133 = 74u64;
                    let a134 = v2;
                    nat_add(a133, a134)?
                }, {
                    let a135 = 34u64;
                    let a136 = v1;
                    nat_add(a135, a136)?
                }))
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a137 = v1;
        let a138 = 4u64;
        let a139 = 0u64;
        nat_rem(a137, a138, a139)
    };
    let a140 = v0;
    let a141 = 0u64;
    if nat_eq(a140, a141) {
        Ok(Adt1::C7({
            let a144 = 1160u64;
            let a145 = {
                let a142 = 8u64;
                let a143 = v2;
                nat_mul(a142, a143)?
            };
            nat_add(a144, a145)?
        }, true, {
            let a158 = 8u64;
            let a159 = {
                let a155 = {
                    let a152 = v3;
                    let a153 = {
                        let a146 = v6;
                        let a147 = 0u64;
                        if nat_eq(a146, a147) {
                            1u64
                        } else {
                            let a148 = v6;
                            let a149 = 1u64;
                            if nat_eq(a148, a149) {
                                128u64
                            } else {
                                let a150 = v6;
                                let a151 = 2u64;
                                if nat_eq(a150, a151) {
                                    16384u64
                                } else {
                                    2097152u64
                                }
                            }
                        }
                    };
                    let a154 = 0u64;
                    nat_quot(a152, a153, a154)
                };
                let a156 = 128u64;
                let a157 = 0u64;
                nat_rem(a155, a156, a157)
            };
            nat_add(a158, a159)?
        }, {
            let a165 = 1u64;
            let a166 = {
                let a163 = 2u64;
                let a164 = {
                    let a160 = v3;
                    let a161 = 268435456u64;
                    let a162 = 0u64;
                    nat_quot(a160, a161, a162)
                };
                nat_mul(a163, a164)?
            };
            nat_add(a165, a166)?
        }))
    } else {
        let a167 = v0;
        let a168 = 1u64;
        if nat_eq(a167, a168) {
            Ok(Adt1::C7({
                let a174 = 3208u64;
                let a175 = {
                    let a172 = 8u64;
                    let a173 = {
                        let a169 = v2;
                        let a170 = 512u64;
                        let a171 = 0u64;
                        nat_rem(a169, a170, a171)
                    };
                    nat_mul(a172, a173)?
                };
                nat_add(a174, a175)?
            }, true, {
                let a179 = 8u64;
                let a180 = {
                    let a176 = v2;
                    let a177 = 512u64;
                    let a178 = 0u64;
                    nat_quot(a176, a177, a178)
                };
                nat_add(a179, a180)?
            }, {
                let a191 = 1u64;
                let a192 = {
                    let a189 = 2u64;
                    let a190 = {
                        let a186 = {
                            let a183 = v4;
                            let a184 = {
                                let a181 = v6;
                                let a182 = 2u64;
                                if nat_lt(a181, a182) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a185 = 0u64;
                            nat_quot(a183, a184, a185)
                        };
                        let a187 = 16u64;
                        let a188 = 0u64;
                        nat_rem(a186, a187, a188)
                    };
                    nat_mul(a189, a190)?
                };
                nat_add(a191, a192)?
            }))
        } else {
            Ok(Adt1::C7({
                let a211 = 7304u64;
                let a212 = {
                    let a209 = 8u64;
                    let a210 = {
                        let a207 = v2;
                        let a208 = {
                            let a205 = 256u64;
                            let a206 = {
                                let a202 = {
                                    let a199 = v3;
                                    let a200 = {
                                        let a193 = v6;
                                        let a194 = 0u64;
                                        if nat_eq(a193, a194) {
                                            1u64
                                        } else {
                                            let a195 = v6;
                                            let a196 = 1u64;
                                            if nat_eq(a195, a196) {
                                                4u64
                                            } else {
                                                let a197 = v6;
                                                let a198 = 2u64;
                                                if nat_eq(a197, a198) {
                                                    16u64
                                                } else {
                                                    64u64
                                                }
                                            }
                                        }
                                    };
                                    let a201 = 0u64;
                                    nat_quot(a199, a200, a201)
                                };
                                let a203 = 4u64;
                                let a204 = 0u64;
                                nat_rem(a202, a203, a204)
                            };
                            nat_mul(a205, a206)?
                        };
                        nat_add(a207, a208)?
                    };
                    nat_mul(a209, a210)?
                };
                nat_add(a211, a212)?
            }, false, v5, {
                let a223 = 1u64;
                let a224 = {
                    let a221 = 2u64;
                    let a222 = {
                        let a218 = {
                            let a215 = v4;
                            let a216 = {
                                let a213 = v6;
                                let a214 = 2u64;
                                if nat_lt(a213, a214) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a217 = 0u64;
                            nat_quot(a215, a216, a217)
                        };
                        let a219 = 16u64;
                        let a220 = 0u64;
                        nat_rem(a218, a219, a220)
                    };
                    nat_mul(a221, a222)?
                };
                nat_add(a223, a224)?
            }))
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let v4: u64 = {
        let a225 = v2;
        let a226 = v3;
        nat_mul(a225, a226)?
    };
    Ok(Adt1::C1({
        let a233 = {
            let a230 = {
                let a227 = v1;
                let a228 = f8(v0);
                let a229 = 0u64;
                nat_quot(a227, a228, a229)
            };
            let a231 = 2u64;
            let a232 = 0u64;
            nat_rem(a230, a231, a232)
        };
        let a234 = 0u64;
        if nat_eq(a233, a234) {
            v4
        } else {
            let a235 = 65536u64;
            let a236 = v4;
            nat_sub(a235, a236)
        }
    }))
}

pub fn f6(v0: bool, v1: u64) -> R<Adt1> {
    let a237 = v1;
    let a238 = 256u64;
    if nat_lt(a237, a238) {
        let v2: u64 = {
            let a239 = v1;
            let a240 = 128u64;
            let a241 = 0u64;
            nat_quot(a239, a240, a241)
        };
        let v3: u64 = {
            let a245 = {
                let a242 = v1;
                let a243 = 32u64;
                let a244 = 0u64;
                nat_quot(a242, a243, a244)
            };
            let a246 = 4u64;
            let a247 = 0u64;
            nat_rem(a245, a246, a247)
        };
        let v4: u64 = {
            let a248 = v1;
            let a249 = 32u64;
            let a250 = 0u64;
            nat_rem(a248, a249, a250)
        };
        let v5: u64 = {
            let a251 = v1;
            let a252 = 16u64;
            let a253 = 0u64;
            nat_quot(a251, a252, a253)
        };
        let v6: u64 = if v0 {
            let a263 = {
                let a254 = v2;
                let a255 = 64u64;
                nat_mul(a254, a255)?
            };
            let a264 = {
                let a261 = {
                    let a259 = {
                        let a256 = v3;
                        let a257 = 2u64;
                        let a258 = 0u64;
                        nat_rem(a256, a257, a258)
                    };
                    let a260 = 32u64;
                    nat_mul(a259, a260)?
                };
                let a262 = v4;
                nat_add(a261, a262)?
            };
            nat_add(a263, a264)?
        } else {
            let a269 = 32u64;
            let a270 = {
                let a267 = {
                    let a265 = v2;
                    let a266 = 32u64;
                    nat_mul(a265, a266)?
                };
                let a268 = v4;
                nat_add(a267, a268)?
            };
            nat_add(a269, a270)?
        };
        let v7: u64 = if v0 {
            let a275 = 128u64;
            let a276 = {
                let a273 = {
                    let a271 = v2;
                    let a272 = 32u64;
                    nat_mul(a271, a272)?
                };
                let a274 = v4;
                nat_add(a273, a274)?
            };
            nat_add(a275, a276)?
        } else {
            v4
        };
        let v8: u64 = if v0 {
            let a277 = 192u64;
            let a278 = v5;
            nat_add(a277, a278)?
        } else {
            let a282 = 96u64;
            let a283 = {
                let a279 = v5;
                let a280 = 8u64;
                let a281 = 0u64;
                nat_rem(a279, a280, a281)
            };
            nat_add(a282, a283)?
        };
        let v9: u64 = if v0 {
            v8
        } else {
            let a287 = 104u64;
            let a288 = {
                let a284 = v5;
                let a285 = 4u64;
                let a286 = 0u64;
                nat_rem(a284, a285, a286)
            };
            nat_add(a287, a288)?
        };
        Ok(Adt1::C8(v6, v7, v8, v9))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let a289 = v1;
    let a290 = 256u64;
    if nat_lt(a289, a290) {
        let a291 = v2;
        let a292 = 256u64;
        if if if if nat_lt(a291, a292) {
            let a293 = v3;
            let a294 = 256u64;
            nat_lt(a293, a294)
        } else {
            false
        } {
            let a295 = v4;
            let a296 = 256u64;
            nat_lt(a295, a296)
        } else {
            false
        } {
            let a297 = v5;
            let a298 = 256u64;
            nat_lt(a297, a298)
        } else {
            false
        } {
            let v6: u64 = {
                let a302 = {
                    let a299 = v1;
                    let a300 = 32u64;
                    let a301 = 0u64;
                    nat_quot(a299, a300, a301)
                };
                let a303 = 4u64;
                let a304 = 0u64;
                nat_rem(a302, a303, a304)
            };
            let v7: u64 = {
                let a305 = v1;
                let a306 = 16u64;
                let a307 = 0u64;
                nat_quot(a305, a306, a307)
            };
            let v8: u64 = f8({
                let a308 = v6;
                let a309 = 2u64;
                nat_mul(a308, a309)?
            });
            let v9: u64 = if v0 {
                let a313 = {
                    let a310 = v3;
                    let a311 = v8;
                    let a312 = 0u64;
                    nat_quot(a310, a311, a312)
                };
                let a314 = 4u64;
                let a315 = 0u64;
                nat_rem(a313, a314, a315)
            } else {
                let a322 = {
                    let a319 = v3;
                    let a320 = f8({
                        let a316 = v1;
                        let a317 = 32u64;
                        let a318 = 0u64;
                        nat_quot(a316, a317, a318)
                    });
                    let a321 = 0u64;
                    nat_quot(a319, a320, a321)
                };
                let a323 = 2u64;
                let a324 = 0u64;
                nat_rem(a322, a323, a324)
            };
            let v10: u64 = if v0 {
                let a335 = {
                    let a330 = {
                        let a327 = v2;
                        let a328 = {
                            let a325 = v6;
                            let a326 = 2u64;
                            if nat_lt(a325, a326) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a329 = 0u64;
                        nat_quot(a327, a328, a329)
                    };
                    let a331 = 16u64;
                    let a332 = 0u64;
                    nat_rem(a330, a331, a332)
                };
                let a336 = {
                    let a333 = v9;
                    let a334 = 16u64;
                    nat_mul(a333, a334)?
                };
                nat_add(a335, a336)?
            } else {
                let a345 = {
                    let a340 = {
                        let a337 = v2;
                        let a338 = v8;
                        let a339 = 0u64;
                        nat_quot(a337, a338, a339)
                    };
                    let a341 = 4u64;
                    let a342 = 0u64;
                    nat_rem(a340, a341, a342)
                };
                let a346 = {
                    let a343 = v9;
                    let a344 = 4u64;
                    nat_mul(a343, a344)?
                };
                nat_add(a345, a346)?
            };
            let v11: u64 = if v0 {
                32u64
            } else {
                4u64
            };
            let v12: u64 = if v0 {
                let a349 = {
                    let a347 = v4;
                    let a348 = 128u64;
                    nat_add(a347, a348)?
                };
                let a350 = 256u64;
                let a351 = 0u64;
                nat_rem(a349, a350, a351)
            } else {
                let a373 = {
                    let a357 = {
                        let a354 = v4;
                        let a355 = {
                            let a352 = v7;
                            let a353 = 8u64;
                            if nat_lt(a352, a353) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a356 = 0u64;
                        nat_quot(a354, a355, a356)
                    };
                    let a358 = 16u64;
                    let a359 = 0u64;
                    nat_rem(a357, a358, a359)
                };
                let a374 = {
                    let a371 = 16u64;
                    let a372 = {
                        let a368 = {
                            let a365 = v5;
                            let a366 = f8({
                                let a363 = {
                                    let a360 = v7;
                                    let a361 = 4u64;
                                    let a362 = 0u64;
                                    nat_quot(a360, a361, a362)
                                };
                                let a364 = 2u64;
                                nat_mul(a363, a364)?
                            });
                            let a367 = 0u64;
                            nat_quot(a365, a366, a367)
                        };
                        let a369 = 4u64;
                        let a370 = 0u64;
                        nat_rem(a368, a369, a370)
                    };
                    nat_mul(a371, a372)?
                };
                nat_add(a373, a374)?
            };
            let v13: u64 = if v0 {
                128u64
            } else {
                32u64
            };
            let v14: u64 = {
                let a375 = v10;
                let a376 = v11;
                if nat_lt(a375, a376) {
                    let a377 = v11;
                    let a378 = v10;
                    nat_sub(a377, a378)
                } else {
                    let a379 = v10;
                    let a380 = v11;
                    nat_sub(a379, a380)
                }
            };
            let v15: u64 = {
                let a381 = v12;
                let a382 = v13;
                if nat_lt(a381, a382) {
                    let a383 = v13;
                    let a384 = v12;
                    nat_sub(a383, a384)
                } else {
                    let a385 = v12;
                    let a386 = v13;
                    nat_sub(a385, a386)
                }
            };
            let v16: u64 = {
                let a387 = v14;
                let a388 = v15;
                nat_mul(a387, a388)?
            };
            Ok(Adt1::C1({
                let a389 = v10;
                let a390 = v11;
                if if nat_lt(a389, a390) {
                    let a391 = v12;
                    let a392 = v13;
                    nat_lt(a391, a392)
                } else {
                    let a393 = v12;
                    let a394 = v13;
                    !nat_lt(a393, a394)
                } {
                    v16
                } else {
                    let a397 = {
                        let a395 = 65536u64;
                        let a396 = v16;
                        nat_sub(a395, a396)
                    };
                    let a398 = 65536u64;
                    let a399 = 0u64;
                    nat_rem(a397, a398, a399)
                }
            }))
        } else {
            Ok(Adt1::C0)
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f8(v0: u64) -> u64 {
    let a400 = v0;
    let a401 = 0u64;
    if nat_eq(a400, a401) {
        1u64
    } else {
        let a402 = v0;
        let a403 = 1u64;
        if nat_eq(a402, a403) {
            2u64
        } else {
            let a404 = v0;
            let a405 = 2u64;
            if nat_eq(a404, a405) {
                4u64
            } else {
                let a406 = v0;
                let a407 = 3u64;
                if nat_eq(a406, a407) {
                    8u64
                } else {
                    let a408 = v0;
                    let a409 = 4u64;
                    if nat_eq(a408, a409) {
                        16u64
                    } else {
                        let a410 = v0;
                        let a411 = 5u64;
                        if nat_eq(a410, a411) {
                            32u64
                        } else {
                            let a412 = v0;
                            let a413 = 6u64;
                            if nat_eq(a412, a413) {
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
