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
        Adt0::C4(v34) => {
            Ok(Adt1::C4(2u64, if v34 {
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
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a37 = v1;
    let a38 = 8u64;
    if if if if nat_lt(a37, a38) {
        let a39 = v2;
        let a40 = 256u64;
        nat_lt(a39, a40)
    } else {
        false
    } {
        let a41 = v3;
        let a42 = 256u64;
        nat_lt(a41, a42)
    } else {
        false
    } {
        let a43 = v4;
        let a44 = 256u64;
        nat_lt(a43, a44)
    } else {
        false
    } {
        let v5: u64 = {
            let a45 = v1;
            let a46 = 4u64;
            if nat_lt(a45, a46) {
                let a47 = v2;
                let a48 = 64u64;
                let a49 = 0u64;
                nat_rem(a47, a48, a49)
            } else {
                let a58 = {
                    let a50 = v4;
                    let a51 = 16u64;
                    let a52 = 0u64;
                    nat_rem(a50, a51, a52)
                };
                let a59 = {
                    let a56 = 16u64;
                    let a57 = {
                        let a53 = v2;
                        let a54 = 64u64;
                        let a55 = 0u64;
                        nat_quot(a53, a54, a55)
                    };
                    nat_mul(a56, a57)?
                };
                nat_add(a58, a59)?
            }
        };
        let v6: u64 = {
            let a60 = v1;
            let a61 = 4u64;
            if nat_lt(a60, a61) {
                let a62 = v3;
                let a63 = 64u64;
                let a64 = 0u64;
                nat_rem(a62, a63, a64)
            } else {
                let a73 = {
                    let a65 = v4;
                    let a66 = 16u64;
                    let a67 = 0u64;
                    nat_quot(a65, a66, a67)
                };
                let a74 = {
                    let a71 = 16u64;
                    let a72 = {
                        let a68 = v3;
                        let a69 = 64u64;
                        let a70 = 0u64;
                        nat_quot(a68, a69, a70)
                    };
                    nat_mul(a71, a72)?
                };
                nat_add(a73, a74)?
            }
        };
        Ok(Adt1::C3({
            let a80 = if v0 {
                48u64
            } else {
                16u64
            };
            let a81 = {
                let a78 = 32u64;
                let a79 = {
                    let a75 = v1;
                    let a76 = 2u64;
                    let a77 = 0u64;
                    nat_quot(a75, a76, a77)
                };
                nat_mul(a78, a79)?
            };
            nat_add(a80, a81)?
        }, {
            let a85 = {
                let a82 = v1;
                let a83 = 2u64;
                let a84 = 0u64;
                nat_rem(a82, a83, a84)
            };
            let a86 = 0u64;
            if nat_eq(a85, a86) {
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
        let a87 = v0;
        let a88 = 0u64;
        if nat_eq(a87, a88) {
            66u64
        } else {
            let a89 = v0;
            let a90 = 1u64;
            if nat_eq(a89, a90) {
                74u64
            } else {
                82u64
            }
        }
    };
    let a91 = v0;
    let a92 = 3u64;
    if if nat_lt(a91, a92) {
        let a95 = v1;
        let a96 = {
            let a93 = v3;
            let a94 = v2;
            nat_mul(a93, a94)?
        };
        nat_eq(a95, a96)
    } else {
        false
    } {
        Ok(Adt1::C5(v3, 1040187392u64))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: u64, v1: u64) -> R<Adt1> {
    let a97 = v0;
    let a98 = 3u64;
    if if nat_lt(a97, a98) {
        let a99 = v1;
        let a100 = 32u64;
        nat_lt(a99, a100)
    } else {
        false
    } {
        let v2: u64 = {
            let a101 = v1;
            let a102 = 4u64;
            let a103 = 0u64;
            nat_quot(a101, a102, a103)
        };
        let v3: u64 = {
            let a104 = v1;
            let a105 = 4u64;
            let a106 = 0u64;
            nat_rem(a104, a105, a106)
        };
        let a107 = v0;
        let a108 = 0u64;
        if nat_eq(a107, a108) {
            let v4: u64 = {
                let a111 = 2u64;
                let a112 = {
                    let a109 = 8u64;
                    let a110 = v2;
                    nat_mul(a109, a110)?
                };
                nat_add(a111, a112)?
            };
            Ok(Adt1::C6({
                let a113 = v4;
                let a114 = v3;
                nat_add(a113, a114)?
            }, 1u64, {
                let a115 = v4;
                let a116 = 4u64;
                nat_add(a115, a116)?
            }, 4u64, v4, v4))
        } else {
            let a117 = v0;
            let a118 = 1u64;
            if nat_eq(a117, a118) {
                Ok(Adt1::C6({
                    let a121 = 2u64;
                    let a122 = {
                        let a119 = 2u64;
                        let a120 = v1;
                        nat_mul(a119, a120)?
                    };
                    nat_add(a121, a122)?
                }, 2u64, 0u64, 1u64, {
                    let a123 = 66u64;
                    let a124 = v2;
                    nat_add(a123, a124)?
                }, 0u64))
            } else {
                Ok(Adt1::C6({
                    let a125 = 2u64;
                    let a126 = v1;
                    nat_add(a125, a126)?
                }, 1u64, {
                    let a127 = 66u64;
                    let a128 = v2;
                    nat_add(a127, a128)?
                }, 1u64, {
                    let a129 = 74u64;
                    let a130 = v2;
                    nat_add(a129, a130)?
                }, {
                    let a131 = 34u64;
                    let a132 = v1;
                    nat_add(a131, a132)?
                }))
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a133 = v1;
        let a134 = 4u64;
        let a135 = 0u64;
        nat_rem(a133, a134, a135)
    };
    let a136 = v0;
    let a137 = 0u64;
    if nat_eq(a136, a137) {
        Ok(Adt1::C7({
            let a140 = 1160u64;
            let a141 = {
                let a138 = 8u64;
                let a139 = v2;
                nat_mul(a138, a139)?
            };
            nat_add(a140, a141)?
        }, true, {
            let a154 = 8u64;
            let a155 = {
                let a151 = {
                    let a148 = v3;
                    let a149 = {
                        let a142 = v6;
                        let a143 = 0u64;
                        if nat_eq(a142, a143) {
                            1u64
                        } else {
                            let a144 = v6;
                            let a145 = 1u64;
                            if nat_eq(a144, a145) {
                                128u64
                            } else {
                                let a146 = v6;
                                let a147 = 2u64;
                                if nat_eq(a146, a147) {
                                    16384u64
                                } else {
                                    2097152u64
                                }
                            }
                        }
                    };
                    let a150 = 0u64;
                    nat_quot(a148, a149, a150)
                };
                let a152 = 128u64;
                let a153 = 0u64;
                nat_rem(a151, a152, a153)
            };
            nat_add(a154, a155)?
        }, {
            let a161 = 1u64;
            let a162 = {
                let a159 = 2u64;
                let a160 = {
                    let a156 = v3;
                    let a157 = 268435456u64;
                    let a158 = 0u64;
                    nat_quot(a156, a157, a158)
                };
                nat_mul(a159, a160)?
            };
            nat_add(a161, a162)?
        }))
    } else {
        let a163 = v0;
        let a164 = 1u64;
        if nat_eq(a163, a164) {
            Ok(Adt1::C7({
                let a170 = 3208u64;
                let a171 = {
                    let a168 = 8u64;
                    let a169 = {
                        let a165 = v2;
                        let a166 = 512u64;
                        let a167 = 0u64;
                        nat_rem(a165, a166, a167)
                    };
                    nat_mul(a168, a169)?
                };
                nat_add(a170, a171)?
            }, true, {
                let a175 = 8u64;
                let a176 = {
                    let a172 = v2;
                    let a173 = 512u64;
                    let a174 = 0u64;
                    nat_quot(a172, a173, a174)
                };
                nat_add(a175, a176)?
            }, {
                let a187 = 1u64;
                let a188 = {
                    let a185 = 2u64;
                    let a186 = {
                        let a182 = {
                            let a179 = v4;
                            let a180 = {
                                let a177 = v6;
                                let a178 = 2u64;
                                if nat_lt(a177, a178) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a181 = 0u64;
                            nat_quot(a179, a180, a181)
                        };
                        let a183 = 16u64;
                        let a184 = 0u64;
                        nat_rem(a182, a183, a184)
                    };
                    nat_mul(a185, a186)?
                };
                nat_add(a187, a188)?
            }))
        } else {
            Ok(Adt1::C7({
                let a207 = 7304u64;
                let a208 = {
                    let a205 = 8u64;
                    let a206 = {
                        let a203 = v2;
                        let a204 = {
                            let a201 = 256u64;
                            let a202 = {
                                let a198 = {
                                    let a195 = v3;
                                    let a196 = {
                                        let a189 = v6;
                                        let a190 = 0u64;
                                        if nat_eq(a189, a190) {
                                            1u64
                                        } else {
                                            let a191 = v6;
                                            let a192 = 1u64;
                                            if nat_eq(a191, a192) {
                                                4u64
                                            } else {
                                                let a193 = v6;
                                                let a194 = 2u64;
                                                if nat_eq(a193, a194) {
                                                    16u64
                                                } else {
                                                    64u64
                                                }
                                            }
                                        }
                                    };
                                    let a197 = 0u64;
                                    nat_quot(a195, a196, a197)
                                };
                                let a199 = 4u64;
                                let a200 = 0u64;
                                nat_rem(a198, a199, a200)
                            };
                            nat_mul(a201, a202)?
                        };
                        nat_add(a203, a204)?
                    };
                    nat_mul(a205, a206)?
                };
                nat_add(a207, a208)?
            }, false, v5, {
                let a219 = 1u64;
                let a220 = {
                    let a217 = 2u64;
                    let a218 = {
                        let a214 = {
                            let a211 = v4;
                            let a212 = {
                                let a209 = v6;
                                let a210 = 2u64;
                                if nat_lt(a209, a210) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a213 = 0u64;
                            nat_quot(a211, a212, a213)
                        };
                        let a215 = 16u64;
                        let a216 = 0u64;
                        nat_rem(a214, a215, a216)
                    };
                    nat_mul(a217, a218)?
                };
                nat_add(a219, a220)?
            }))
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let v4: u64 = {
        let a221 = v2;
        let a222 = v3;
        nat_mul(a221, a222)?
    };
    Ok(Adt1::C1({
        let a229 = {
            let a226 = {
                let a223 = v1;
                let a224 = f6(v0);
                let a225 = 0u64;
                nat_quot(a223, a224, a225)
            };
            let a227 = 2u64;
            let a228 = 0u64;
            nat_rem(a226, a227, a228)
        };
        let a230 = 0u64;
        if nat_eq(a229, a230) {
            v4
        } else {
            let a231 = 65536u64;
            let a232 = v4;
            nat_sub(a231, a232)
        }
    }))
}

pub fn f6(v0: u64) -> u64 {
    let a233 = v0;
    let a234 = 0u64;
    if nat_eq(a233, a234) {
        1u64
    } else {
        let a235 = v0;
        let a236 = 1u64;
        if nat_eq(a235, a236) {
            2u64
        } else {
            let a237 = v0;
            let a238 = 2u64;
            if nat_eq(a237, a238) {
                4u64
            } else {
                let a239 = v0;
                let a240 = 3u64;
                if nat_eq(a239, a240) {
                    8u64
                } else {
                    let a241 = v0;
                    let a242 = 4u64;
                    if nat_eq(a241, a242) {
                        16u64
                    } else {
                        let a243 = v0;
                        let a244 = 5u64;
                        if nat_eq(a243, a244) {
                            32u64
                        } else {
                            let a245 = v0;
                            let a246 = 6u64;
                            if nat_eq(a245, a246) {
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

pub fn run(p0: Adt0) -> R<Adt1> {
    f0(p0)
}
