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
    C15,
    C16(u64, u64, u64, u64),
    C17(u64, u64, bool),
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
    C11(u64, u64, bool),
    C12(u64, u64, u64, u64),
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
        Adt0::C4(v65) => {
            Ok(Adt1::C4(2u64, if v65 {
                1u64
            } else {
                0u64
            }))
        }
        Adt0::C5(v26, v27, v28) => {
            let a69 = v26;
            let a70 = 5u64;
            if if nat_eq(a69, a70) {
                true
            } else {
                let a71 = v26;
                let a72 = 6u64;
                nat_eq(a71, a72)
            } {
                let v29: u64 = {
                    let a73 = v26;
                    let a74 = 6u64;
                    if nat_eq(a73, a74) {
                        56u64
                    } else {
                        50u64
                    }
                };
                let a78 = {
                    let a75 = v27;
                    let a76 = v29;
                    let a77 = 0u64;
                    nat_rem(a75, a76, a77)
                };
                let a79 = 0u64;
                if if nat_eq(a78, a79) {
                    let a83 = {
                        let a80 = v27;
                        let a81 = v29;
                        let a82 = 0u64;
                        nat_quot(a80, a81, a82)
                    };
                    let a84 = v28;
                    nat_eq(a83, a84)
                } else {
                    false
                } {
                    Ok(Adt1::C12(v29, 1065353216u64, 8u64, 1040187392u64))
                } else {
                    Ok(Adt1::C0)
                }
            } else {
                let a85 = v26;
                let a86 = 3u64;
                if if nat_eq(a85, a86) {
                    true
                } else {
                    let a87 = v26;
                    let a88 = 4u64;
                    nat_eq(a87, a88)
                } {
                    let v30: u64 = {
                        let a89 = v26;
                        let a90 = 3u64;
                        if nat_eq(a89, a90) {
                            98u64
                        } else {
                            110u64
                        }
                    };
                    let a94 = {
                        let a91 = v27;
                        let a92 = v30;
                        let a93 = 0u64;
                        nat_rem(a91, a92, a93)
                    };
                    let a95 = 0u64;
                    if if nat_eq(a94, a95) {
                        let a99 = {
                            let a96 = v27;
                            let a97 = v30;
                            let a98 = 0u64;
                            nat_quot(a96, a97, a98)
                        };
                        let a100 = v28;
                        nat_eq(a99, a100)
                    } else {
                        false
                    } {
                        Ok(Adt1::C9(v30, {
                            let a101 = v26;
                            let a102 = 3u64;
                            if nat_eq(a101, a102) {
                                1048576000u64
                            } else {
                                1065353216u64
                            }
                        }, 4u64))
                    } else {
                        Ok(Adt1::C0)
                    }
                } else {
                    f3(v26, v27, v28)
                }
            }
        }
        Adt0::C6(v31, v32) => {
            let a103 = v31;
            let a104 = 5u64;
            if if nat_eq(a103, a104) {
                true
            } else {
                let a105 = v31;
                let a106 = 6u64;
                nat_eq(a105, a106)
            } {
                f4({
                    let a107 = v31;
                    let a108 = 6u64;
                    nat_eq(a107, a108)
                }, v32)
            } else {
                let a109 = v31;
                let a110 = 3u64;
                if if nat_eq(a109, a110) {
                    true
                } else {
                    let a111 = v31;
                    let a112 = 4u64;
                    nat_eq(a111, a112)
                } {
                    f5({
                        let a113 = v31;
                        let a114 = 3u64;
                        nat_eq(a113, a114)
                    }, v32)
                } else {
                    f6(v31, v32)
                }
            }
        }
        Adt0::C7(v33, v34, v35, v36, v37, v38) => {
            let a115 = v33;
            let a116 = 5u64;
            if if nat_eq(a115, a116) {
                true
            } else {
                let a117 = v33;
                let a118 = 6u64;
                nat_eq(a117, a118)
            } {
                f7({
                    let a119 = v33;
                    let a120 = 6u64;
                    nat_eq(a119, a120)
                }, v34, v35, v36, v37)
            } else {
                let a121 = v33;
                let a122 = 3u64;
                if if nat_eq(a121, a122) {
                    true
                } else {
                    let a123 = v33;
                    let a124 = 4u64;
                    nat_eq(a123, a124)
                } {
                    f8({
                        let a125 = v33;
                        let a126 = 3u64;
                        nat_eq(a125, a126)
                    }, v34, v35, v36, v37, v38)
                } else {
                    f9(v33, v34, v35, v36, v37, v38)
                }
            }
        }
        Adt0::C8(v39, v40, v41, v42) => {
            f10(v39, v40, v41, v42)
        }
        Adt0::C9(v53, v54, v55) => {
            let v56: u64 = if v53 {
                210u64
            } else {
                110u64
            };
            let a186 = v54;
            let a187 = {
                let a184 = v56;
                let a185 = v55;
                nat_mul(a184, a185)?
            };
            if nat_eq(a186, a187) {
                Ok(Adt1::C1(v56))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C10(v57, v58) => {
            f12(v57, v58)
        }
        Adt0::C11(v59, v60, v61, v62, v63, v64) => {
            f13(v59, v60, v61, v62, v63, v64)
        }
        Adt0::C12(v43, v44) => {
            let a129 = v43;
            let a130 = {
                let a127 = 136u64;
                let a128 = v44;
                nat_mul(a127, a128)?
            };
            if nat_eq(a129, a130) {
                Ok(Adt1::C5(136u64, 1065353216u64))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C13(v45) => {
            let a131 = v45;
            let a132 = 8u64;
            if nat_lt(a131, a132) {
                Ok(Adt1::C2({
                    let a136 = 4u64;
                    let a137 = {
                        let a133 = v45;
                        let a134 = 2u64;
                        let a135 = 0u64;
                        nat_quot(a133, a134, a135)
                    };
                    nat_add(a136, a137)?
                }, {
                    let a141 = 2u64;
                    let a142 = {
                        let a138 = v45;
                        let a139 = 4u64;
                        let a140 = 0u64;
                        nat_quot(a138, a139, a140)
                    };
                    nat_add(a141, a142)?
                }, {
                    let a145 = 8u64;
                    let a146 = {
                        let a143 = v45;
                        let a144 = 16u64;
                        nat_mul(a143, a144)?
                    };
                    nat_add(a145, a146)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C14(v46, v47, v48) => {
            let a147 = v46;
            let a148 = 8u64;
            if if if nat_lt(a147, a148) {
                let a149 = v47;
                let a150 = 256u64;
                nat_lt(a149, a150)
            } else {
                false
            } {
                let a151 = v48;
                let a152 = 256u64;
                nat_lt(a151, a152)
            } else {
                false
            } {
                let v49: u64 = {
                    let a156 = {
                        let a153 = v46;
                        let a154 = 2u64;
                        let a155 = 0u64;
                        nat_rem(a153, a154, a155)
                    };
                    let a157 = 0u64;
                    if nat_eq(a156, a157) {
                        1u64
                    } else {
                        16u64
                    }
                };
                let v50: u64 = {
                    let a161 = 2u64;
                    let a162 = {
                        let a158 = v46;
                        let a159 = 4u64;
                        let a160 = 0u64;
                        nat_rem(a158, a159, a160)
                    };
                    nat_mul(a161, a162)?
                };
                let v51: u64 = {
                    let a166 = {
                        let a163 = v47;
                        let a164 = v49;
                        let a165 = 0u64;
                        nat_quot(a163, a164, a165)
                    };
                    let a167 = 16u64;
                    let a168 = 0u64;
                    nat_rem(a166, a167, a168)
                };
                let v52: u64 = {
                    let a172 = {
                        let a169 = v48;
                        let a170 = f11(v50);
                        let a171 = 0u64;
                        nat_quot(a169, a170, a171)
                    };
                    let a173 = 4u64;
                    let a174 = 0u64;
                    nat_rem(a172, a173, a174)
                };
                Ok(Adt1::C1({
                    let a181 = {
                        let a179 = 224u64;
                        let a180 = {
                            let a177 = v51;
                            let a178 = {
                                let a175 = 16u64;
                                let a176 = v52;
                                nat_mul(a175, a176)?
                            };
                            nat_add(a177, a178)?
                        };
                        nat_add(a179, a180)?
                    };
                    let a182 = 256u64;
                    let a183 = 0u64;
                    nat_rem(a181, a182, a183)
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C15 => {
            Ok(Adt1::C8(48u64, 50u64, 52u64, 54u64))
        }
        Adt0::C16(v19, v20, v21, v22) => {
            let a37 = v19;
            let a38 = 65536u64;
            if if if if nat_lt(a37, a38) {
                let a39 = v20;
                let a40 = 65536u64;
                nat_lt(a39, a40)
            } else {
                false
            } {
                let a41 = v21;
                let a42 = 65536u64;
                nat_lt(a41, a42)
            } else {
                false
            } {
                let a43 = v22;
                let a44 = 65536u64;
                nat_lt(a43, a44)
            } else {
                false
            } {
                Ok(Adt1::C1({
                    let a67 = {
                        let a45 = v19;
                        let a46 = 4096u64;
                        let a47 = 0u64;
                        nat_quot(a45, a46, a47)
                    };
                    let a68 = {
                        let a65 = {
                            let a51 = 16u64;
                            let a52 = {
                                let a48 = v20;
                                let a49 = 4096u64;
                                let a50 = 0u64;
                                nat_quot(a48, a49, a50)
                            };
                            nat_mul(a51, a52)?
                        };
                        let a66 = {
                            let a63 = {
                                let a56 = 256u64;
                                let a57 = {
                                    let a53 = v21;
                                    let a54 = 4096u64;
                                    let a55 = 0u64;
                                    nat_quot(a53, a54, a55)
                                };
                                nat_mul(a56, a57)?
                            };
                            let a64 = {
                                let a61 = 4096u64;
                                let a62 = {
                                    let a58 = v22;
                                    let a59 = 4096u64;
                                    let a60 = 0u64;
                                    nat_quot(a58, a59, a60)
                                };
                                nat_mul(a61, a62)?
                            };
                            nat_add(a63, a64)?
                        };
                        nat_add(a65, a66)?
                    };
                    nat_add(a67, a68)?
                }))
            } else {
                Ok(Adt1::C0)
            }
        }
        Adt0::C17(v23, v24, v25) => {
            f2(v23, v24, v25)
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let a188 = v1;
    let a189 = 8u64;
    if if if if nat_lt(a188, a189) {
        let a190 = v2;
        let a191 = 256u64;
        nat_lt(a190, a191)
    } else {
        false
    } {
        let a192 = v3;
        let a193 = 256u64;
        nat_lt(a192, a193)
    } else {
        false
    } {
        let a194 = v4;
        let a195 = 256u64;
        nat_lt(a194, a195)
    } else {
        false
    } {
        let v5: u64 = {
            let a196 = v1;
            let a197 = 4u64;
            if nat_lt(a196, a197) {
                let a198 = v2;
                let a199 = 64u64;
                let a200 = 0u64;
                nat_rem(a198, a199, a200)
            } else {
                let a209 = {
                    let a201 = v4;
                    let a202 = 16u64;
                    let a203 = 0u64;
                    nat_rem(a201, a202, a203)
                };
                let a210 = {
                    let a207 = 16u64;
                    let a208 = {
                        let a204 = v2;
                        let a205 = 64u64;
                        let a206 = 0u64;
                        nat_quot(a204, a205, a206)
                    };
                    nat_mul(a207, a208)?
                };
                nat_add(a209, a210)?
            }
        };
        let v6: u64 = {
            let a211 = v1;
            let a212 = 4u64;
            if nat_lt(a211, a212) {
                let a213 = v3;
                let a214 = 64u64;
                let a215 = 0u64;
                nat_rem(a213, a214, a215)
            } else {
                let a224 = {
                    let a216 = v4;
                    let a217 = 16u64;
                    let a218 = 0u64;
                    nat_quot(a216, a217, a218)
                };
                let a225 = {
                    let a222 = 16u64;
                    let a223 = {
                        let a219 = v3;
                        let a220 = 64u64;
                        let a221 = 0u64;
                        nat_quot(a219, a220, a221)
                    };
                    nat_mul(a222, a223)?
                };
                nat_add(a224, a225)?
            }
        };
        Ok(Adt1::C3({
            let a231 = if v0 {
                48u64
            } else {
                16u64
            };
            let a232 = {
                let a229 = 32u64;
                let a230 = {
                    let a226 = v1;
                    let a227 = 2u64;
                    let a228 = 0u64;
                    nat_quot(a226, a227, a228)
                };
                nat_mul(a229, a230)?
            };
            nat_add(a231, a232)?
        }, {
            let a236 = {
                let a233 = v1;
                let a234 = 2u64;
                let a235 = 0u64;
                nat_rem(a233, a234, a235)
            };
            let a237 = 0u64;
            if nat_eq(a236, a237) {
                1u64
            } else {
                16u64
            }
        }, if v0 {
            f11(v1)
        } else {
            0u64
        }, v5, v6))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f2(v0: u64, v1: u64, v2: bool) -> R<Adt1> {
    let a238 = v0;
    let a239 = 255u64;
    if if if nat_eq(a238, a239) {
        true
    } else {
        let a240 = v0;
        let a241 = 2u64;
        nat_lt(a240, a241)
    } {
        let a242 = v1;
        let a243 = 16u64;
        nat_lt(a242, a243)
    } else {
        false
    } {
        let v3: u64 = {
            let a244 = v0;
            let a245 = 0u64;
            if nat_eq(a244, a245) {
                1u64
            } else {
                let a246 = v0;
                let a247 = 255u64;
                if if nat_eq(a246, a247) {
                    v2
                } else {
                    !v2
                } {
                    9u64
                } else {
                    7u64
                }
            }
        };
        let v4: u64 = {
            let a248 = v3;
            let a249 = v1;
            nat_mul(a248, a249)?
        };
        Ok(Adt1::C1({
            let a250 = v0;
            let a251 = 255u64;
            if if nat_eq(a250, a251) {
                true
            } else {
                let a252 = v0;
                let a253 = 0u64;
                if nat_eq(a252, a253) {
                    v2
                } else {
                    false
                }
            } {
                let a254 = 65536u64;
                let a255 = v4;
                nat_sub(a254, a255)
            } else {
                v4
            }
        }))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<Adt1> {
    let v3: u64 = {
        let a256 = v0;
        let a257 = 0u64;
        if nat_eq(a256, a257) {
            66u64
        } else {
            let a258 = v0;
            let a259 = 1u64;
            if nat_eq(a258, a259) {
                74u64
            } else {
                82u64
            }
        }
    };
    let a260 = v0;
    let a261 = 3u64;
    if if nat_lt(a260, a261) {
        let a264 = v1;
        let a265 = {
            let a262 = v3;
            let a263 = v2;
            nat_mul(a262, a263)?
        };
        nat_eq(a264, a265)
    } else {
        false
    } {
        Ok(Adt1::C5(v3, 1040187392u64))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f4(v0: bool, v1: u64) -> R<Adt1> {
    let a266 = v1;
    let a267 = 32u64;
    if nat_lt(a266, a267) {
        if v0 {
            Ok(Adt1::C6(v1, 1u64, {
                let a273 = 48u64;
                let a274 = {
                    let a271 = 2u64;
                    let a272 = {
                        let a268 = v1;
                        let a269 = 8u64;
                        let a270 = 0u64;
                        nat_quot(a268, a269, a270)
                    };
                    nat_mul(a271, a272)?
                };
                nat_add(a273, a274)?
            }, 2u64, {
                let a278 = 32u64;
                let a279 = {
                    let a275 = v1;
                    let a276 = 2u64;
                    let a277 = 0u64;
                    nat_quot(a275, a276, a277)
                };
                nat_add(a278, a279)?
            }, 0u64))
        } else {
            Ok(Adt1::C6({
                let a280 = 2u64;
                let a281 = v1;
                nat_add(a280, a281)?
            }, 1u64, {
                let a287 = 34u64;
                let a288 = {
                    let a285 = 2u64;
                    let a286 = {
                        let a282 = v1;
                        let a283 = 4u64;
                        let a284 = 0u64;
                        nat_quot(a282, a283, a284)
                    };
                    nat_mul(a285, a286)?
                };
                nat_add(a287, a288)?
            }, 2u64, 0u64, 0u64))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f5(v0: bool, v1: u64) -> R<Adt1> {
    let a289 = v1;
    let a290 = 64u64;
    if nat_lt(a289, a290) {
        let v2: u64 = {
            let a291 = v1;
            let a292 = 8u64;
            let a293 = 0u64;
            nat_quot(a291, a292, a293)
        };
        if v0 {
            Ok(Adt1::C6({
                let a294 = 2u64;
                let a295 = v1;
                nat_add(a294, a295)?
            }, 1u64, {
                let a298 = 66u64;
                let a299 = {
                    let a296 = 4u64;
                    let a297 = v2;
                    nat_mul(a296, a297)?
                };
                nat_add(a298, a299)?
            }, 4u64, 0u64, 0u64))
        } else {
            Ok(Adt1::C6({
                let a300 = 2u64;
                let a301 = v1;
                nat_add(a300, a301)?
            }, 1u64, {
                let a302 = 66u64;
                let a303 = v2;
                nat_add(a302, a303)?
            }, 1u64, {
                let a307 = 106u64;
                let a308 = {
                    let a304 = v2;
                    let a305 = 2u64;
                    let a306 = 0u64;
                    nat_quot(a304, a305, a306)
                };
                nat_add(a307, a308)?
            }, {
                let a312 = 74u64;
                let a313 = {
                    let a309 = v1;
                    let a310 = 2u64;
                    let a311 = 0u64;
                    nat_quot(a309, a310, a311)
                };
                nat_add(a312, a313)?
            }))
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f6(v0: u64, v1: u64) -> R<Adt1> {
    let a314 = v0;
    let a315 = 3u64;
    if if nat_lt(a314, a315) {
        let a316 = v1;
        let a317 = 32u64;
        nat_lt(a316, a317)
    } else {
        false
    } {
        let v2: u64 = {
            let a318 = v1;
            let a319 = 4u64;
            let a320 = 0u64;
            nat_quot(a318, a319, a320)
        };
        let v3: u64 = {
            let a321 = v1;
            let a322 = 4u64;
            let a323 = 0u64;
            nat_rem(a321, a322, a323)
        };
        let a324 = v0;
        let a325 = 0u64;
        if nat_eq(a324, a325) {
            let v4: u64 = {
                let a328 = 2u64;
                let a329 = {
                    let a326 = 8u64;
                    let a327 = v2;
                    nat_mul(a326, a327)?
                };
                nat_add(a328, a329)?
            };
            Ok(Adt1::C6({
                let a330 = v4;
                let a331 = v3;
                nat_add(a330, a331)?
            }, 1u64, {
                let a332 = v4;
                let a333 = 4u64;
                nat_add(a332, a333)?
            }, 4u64, v4, v4))
        } else {
            let a334 = v0;
            let a335 = 1u64;
            if nat_eq(a334, a335) {
                Ok(Adt1::C6({
                    let a338 = 2u64;
                    let a339 = {
                        let a336 = 2u64;
                        let a337 = v1;
                        nat_mul(a336, a337)?
                    };
                    nat_add(a338, a339)?
                }, 2u64, 0u64, 1u64, {
                    let a340 = 66u64;
                    let a341 = v2;
                    nat_add(a340, a341)?
                }, 0u64))
            } else {
                Ok(Adt1::C6({
                    let a342 = 2u64;
                    let a343 = v1;
                    nat_add(a342, a343)?
                }, 1u64, {
                    let a344 = 66u64;
                    let a345 = v2;
                    nat_add(a344, a345)?
                }, 1u64, {
                    let a346 = 74u64;
                    let a347 = v2;
                    nat_add(a346, a347)?
                }, {
                    let a348 = 34u64;
                    let a349 = v1;
                    nat_add(a348, a349)?
                }))
            }
        }
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f7(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt1> {
    let v5: u64 = {
        let a350 = v1;
        let a351 = 4u64;
        let a352 = 0u64;
        nat_rem(a350, a351, a352)
    };
    let v6: u64 = if v0 {
        let a358 = v4;
        let a359 = {
            let a356 = {
                let a353 = v5;
                let a354 = 2u64;
                let a355 = 0u64;
                nat_rem(a353, a354, a355)
            };
            let a357 = 0u64;
            if nat_eq(a356, a357) {
                1u64
            } else {
                16u64
            }
        };
        let a360 = 0u64;
        nat_quot(a358, a359, a360)
    } else {
        let a367 = v3;
        let a368 = {
            let a361 = v5;
            let a362 = 0u64;
            if nat_eq(a361, a362) {
                1u64
            } else {
                let a363 = v5;
                let a364 = 1u64;
                if nat_eq(a363, a364) {
                    8u64
                } else {
                    let a365 = v5;
                    let a366 = 2u64;
                    if nat_eq(a365, a366) {
                        64u64
                    } else {
                        512u64
                    }
                }
            }
        };
        let a369 = 0u64;
        nat_quot(a367, a368, a369)
    };
    let v7: u64 = {
        let a373 = {
            let a370 = v1;
            let a371 = 8u64;
            let a372 = 0u64;
            nat_rem(a370, a371, a372)
        };
        let a374 = 2u64;
        let a375 = 0u64;
        nat_quot(a373, a374, a375)
    };
    let v8: u64 = if v0 {
        let a376 = v7;
        let a377 = 0u64;
        if nat_eq(a376, a377) {
            1u64
        } else {
            let a378 = v7;
            let a379 = 1u64;
            if nat_eq(a378, a379) {
                8u64
            } else {
                let a380 = v7;
                let a381 = 2u64;
                if nat_eq(a380, a381) {
                    64u64
                } else {
                    512u64
                }
            }
        }
    } else {
        4096u64
    };
    Ok(Adt1::C11({
        let a391 = 18600u64;
        let a392 = {
            let a389 = 8u64;
            let a390 = {
                let a387 = v2;
                let a388 = {
                    let a385 = 256u64;
                    let a386 = {
                        let a382 = v6;
                        let a383 = 8u64;
                        let a384 = 0u64;
                        nat_rem(a382, a383, a384)
                    };
                    nat_mul(a385, a386)?
                };
                nat_add(a387, a388)?
            };
            nat_mul(a389, a390)?
        };
        nat_add(a391, a392)?
    }, {
        let a401 = 1u64;
        let a402 = {
            let a399 = 2u64;
            let a400 = {
                let a396 = {
                    let a393 = v3;
                    let a394 = v8;
                    let a395 = 0u64;
                    nat_quot(a393, a394, a395)
                };
                let a397 = 8u64;
                let a398 = 0u64;
                nat_rem(a396, a397, a398)
            };
            nat_mul(a399, a400)?
        };
        nat_add(a401, a402)?
    }, if v0 {
        let a409 = {
            let a406 = {
                let a403 = v6;
                let a404 = 8u64;
                let a405 = 0u64;
                nat_quot(a403, a404, a405)
            };
            let a407 = 2u64;
            let a408 = 0u64;
            nat_rem(a406, a407, a408)
        };
        let a410 = 1u64;
        nat_eq(a409, a410)
    } else {
        let a411 = 32768u64;
        let a412 = v3;
        nat_le(a411, a412)
    }))
}

pub fn f8(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a413 = v1;
        let a414 = 8u64;
        let a415 = 0u64;
        nat_rem(a413, a414, a415)
    };
    let v7: u64 = {
        let a419 = 4u64;
        let a420 = {
            let a416 = v6;
            let a417 = 2u64;
            let a418 = 0u64;
            nat_rem(a416, a417, a418)
        };
        nat_mul(a419, a420)?
    };
    if v0 {
        let v8: u64 = {
            let a421 = v6;
            let a422 = 2u64;
            if nat_lt(a421, a422) {
                1u64
            } else {
                let a423 = v6;
                let a424 = 4u64;
                if nat_lt(a423, a424) {
                    128u64
                } else {
                    let a425 = v6;
                    let a426 = 6u64;
                    if nat_lt(a425, a426) {
                        16384u64
                    } else {
                        2097152u64
                    }
                }
            }
        };
        Ok(Adt1::C10({
            let a429 = 15496u64;
            let a430 = {
                let a427 = 4u64;
                let a428 = v2;
                nat_mul(a427, a428)?
            };
            nat_add(a429, a430)?
        }, true, {
            let a437 = 8u64;
            let a438 = {
                let a434 = {
                    let a431 = v3;
                    let a432 = v8;
                    let a433 = 0u64;
                    nat_quot(a431, a432, a433)
                };
                let a435 = 128u64;
                let a436 = 0u64;
                nat_rem(a434, a435, a436)
            };
            nat_add(a437, a438)?
        }, {
            let a444 = 1u64;
            let a445 = {
                let a442 = 2u64;
                let a443 = {
                    let a439 = v3;
                    let a440 = 268435456u64;
                    let a441 = 0u64;
                    nat_quot(a439, a440, a441)
                };
                nat_mul(a442, a443)?
            };
            nat_add(a444, a445)?
        }, v7))
    } else {
        let v9: u64 = {
            let a449 = {
                let a446 = v3;
                let a447 = f11(v6);
                let a448 = 0u64;
                nat_quot(a446, a447, a448)
            };
            let a450 = 2u64;
            let a451 = 0u64;
            nat_rem(a449, a450, a451)
        };
        let v10: u64 = {
            let a460 = {
                let a457 = v4;
                let a458 = {
                    let a455 = {
                        let a452 = v1;
                        let a453 = 16u64;
                        let a454 = 0u64;
                        nat_rem(a452, a453, a454)
                    };
                    let a456 = 8u64;
                    if nat_lt(a455, a456) {
                        1u64
                    } else {
                        16u64
                    }
                };
                let a459 = 0u64;
                nat_quot(a457, a458, a459)
            };
            let a461 = 16u64;
            let a462 = 0u64;
            nat_rem(a460, a461, a462)
        };
        Ok(Adt1::C10({
            let a469 = 16520u64;
            let a470 = {
                let a467 = 4u64;
                let a468 = {
                    let a465 = v2;
                    let a466 = {
                        let a463 = 256u64;
                        let a464 = v9;
                        nat_mul(a463, a464)?
                    };
                    nat_add(a465, a466)?
                };
                nat_mul(a467, a468)?
            };
            nat_add(a469, a470)?
        }, false, v5, {
            let a473 = 1u64;
            let a474 = {
                let a471 = 2u64;
                let a472 = v10;
                nat_mul(a471, a472)?
            };
            nat_add(a473, a474)?
        }, v7))
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let v6: u64 = {
        let a475 = v1;
        let a476 = 4u64;
        let a477 = 0u64;
        nat_rem(a475, a476, a477)
    };
    let a478 = v0;
    let a479 = 0u64;
    if nat_eq(a478, a479) {
        Ok(Adt1::C7({
            let a482 = 1160u64;
            let a483 = {
                let a480 = 8u64;
                let a481 = v2;
                nat_mul(a480, a481)?
            };
            nat_add(a482, a483)?
        }, true, {
            let a496 = 8u64;
            let a497 = {
                let a493 = {
                    let a490 = v3;
                    let a491 = {
                        let a484 = v6;
                        let a485 = 0u64;
                        if nat_eq(a484, a485) {
                            1u64
                        } else {
                            let a486 = v6;
                            let a487 = 1u64;
                            if nat_eq(a486, a487) {
                                128u64
                            } else {
                                let a488 = v6;
                                let a489 = 2u64;
                                if nat_eq(a488, a489) {
                                    16384u64
                                } else {
                                    2097152u64
                                }
                            }
                        }
                    };
                    let a492 = 0u64;
                    nat_quot(a490, a491, a492)
                };
                let a494 = 128u64;
                let a495 = 0u64;
                nat_rem(a493, a494, a495)
            };
            nat_add(a496, a497)?
        }, {
            let a503 = 1u64;
            let a504 = {
                let a501 = 2u64;
                let a502 = {
                    let a498 = v3;
                    let a499 = 268435456u64;
                    let a500 = 0u64;
                    nat_quot(a498, a499, a500)
                };
                nat_mul(a501, a502)?
            };
            nat_add(a503, a504)?
        }))
    } else {
        let a505 = v0;
        let a506 = 1u64;
        if nat_eq(a505, a506) {
            Ok(Adt1::C7({
                let a512 = 3208u64;
                let a513 = {
                    let a510 = 8u64;
                    let a511 = {
                        let a507 = v2;
                        let a508 = 512u64;
                        let a509 = 0u64;
                        nat_rem(a507, a508, a509)
                    };
                    nat_mul(a510, a511)?
                };
                nat_add(a512, a513)?
            }, true, {
                let a517 = 8u64;
                let a518 = {
                    let a514 = v2;
                    let a515 = 512u64;
                    let a516 = 0u64;
                    nat_quot(a514, a515, a516)
                };
                nat_add(a517, a518)?
            }, {
                let a529 = 1u64;
                let a530 = {
                    let a527 = 2u64;
                    let a528 = {
                        let a524 = {
                            let a521 = v4;
                            let a522 = {
                                let a519 = v6;
                                let a520 = 2u64;
                                if nat_lt(a519, a520) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a523 = 0u64;
                            nat_quot(a521, a522, a523)
                        };
                        let a525 = 16u64;
                        let a526 = 0u64;
                        nat_rem(a524, a525, a526)
                    };
                    nat_mul(a527, a528)?
                };
                nat_add(a529, a530)?
            }))
        } else {
            Ok(Adt1::C7({
                let a549 = 7304u64;
                let a550 = {
                    let a547 = 8u64;
                    let a548 = {
                        let a545 = v2;
                        let a546 = {
                            let a543 = 256u64;
                            let a544 = {
                                let a540 = {
                                    let a537 = v3;
                                    let a538 = {
                                        let a531 = v6;
                                        let a532 = 0u64;
                                        if nat_eq(a531, a532) {
                                            1u64
                                        } else {
                                            let a533 = v6;
                                            let a534 = 1u64;
                                            if nat_eq(a533, a534) {
                                                4u64
                                            } else {
                                                let a535 = v6;
                                                let a536 = 2u64;
                                                if nat_eq(a535, a536) {
                                                    16u64
                                                } else {
                                                    64u64
                                                }
                                            }
                                        }
                                    };
                                    let a539 = 0u64;
                                    nat_quot(a537, a538, a539)
                                };
                                let a541 = 4u64;
                                let a542 = 0u64;
                                nat_rem(a540, a541, a542)
                            };
                            nat_mul(a543, a544)?
                        };
                        nat_add(a545, a546)?
                    };
                    nat_mul(a547, a548)?
                };
                nat_add(a549, a550)?
            }, false, v5, {
                let a561 = 1u64;
                let a562 = {
                    let a559 = 2u64;
                    let a560 = {
                        let a556 = {
                            let a553 = v4;
                            let a554 = {
                                let a551 = v6;
                                let a552 = 2u64;
                                if nat_lt(a551, a552) {
                                    1u64
                                } else {
                                    16u64
                                }
                            };
                            let a555 = 0u64;
                            nat_quot(a553, a554, a555)
                        };
                        let a557 = 16u64;
                        let a558 = 0u64;
                        nat_rem(a556, a557, a558)
                    };
                    nat_mul(a559, a560)?
                };
                nat_add(a561, a562)?
            }))
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt1> {
    let v4: u64 = {
        let a563 = v2;
        let a564 = v3;
        nat_mul(a563, a564)?
    };
    Ok(Adt1::C1({
        let a571 = {
            let a568 = {
                let a565 = v1;
                let a566 = f11(v0);
                let a567 = 0u64;
                nat_quot(a565, a566, a567)
            };
            let a569 = 2u64;
            let a570 = 0u64;
            nat_rem(a568, a569, a570)
        };
        let a572 = 0u64;
        if nat_eq(a571, a572) {
            v4
        } else {
            let a573 = 65536u64;
            let a574 = v4;
            nat_sub(a573, a574)
        }
    }))
}

pub fn f11(v0: u64) -> u64 {
    let a575 = v0;
    let a576 = 0u64;
    if nat_eq(a575, a576) {
        1u64
    } else {
        let a577 = v0;
        let a578 = 1u64;
        if nat_eq(a577, a578) {
            2u64
        } else {
            let a579 = v0;
            let a580 = 2u64;
            if nat_eq(a579, a580) {
                4u64
            } else {
                let a581 = v0;
                let a582 = 3u64;
                if nat_eq(a581, a582) {
                    8u64
                } else {
                    let a583 = v0;
                    let a584 = 4u64;
                    if nat_eq(a583, a584) {
                        16u64
                    } else {
                        let a585 = v0;
                        let a586 = 5u64;
                        if nat_eq(a585, a586) {
                            32u64
                        } else {
                            let a587 = v0;
                            let a588 = 6u64;
                            if nat_eq(a587, a588) {
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

pub fn f12(v0: bool, v1: u64) -> R<Adt1> {
    let a589 = v1;
    let a590 = 256u64;
    if nat_lt(a589, a590) {
        let v2: u64 = {
            let a591 = v1;
            let a592 = 128u64;
            let a593 = 0u64;
            nat_quot(a591, a592, a593)
        };
        let v3: u64 = {
            let a597 = {
                let a594 = v1;
                let a595 = 32u64;
                let a596 = 0u64;
                nat_quot(a594, a595, a596)
            };
            let a598 = 4u64;
            let a599 = 0u64;
            nat_rem(a597, a598, a599)
        };
        let v4: u64 = {
            let a600 = v1;
            let a601 = 32u64;
            let a602 = 0u64;
            nat_rem(a600, a601, a602)
        };
        let v5: u64 = {
            let a603 = v1;
            let a604 = 16u64;
            let a605 = 0u64;
            nat_quot(a603, a604, a605)
        };
        let v6: u64 = if v0 {
            let a615 = {
                let a606 = v2;
                let a607 = 64u64;
                nat_mul(a606, a607)?
            };
            let a616 = {
                let a613 = {
                    let a611 = {
                        let a608 = v3;
                        let a609 = 2u64;
                        let a610 = 0u64;
                        nat_rem(a608, a609, a610)
                    };
                    let a612 = 32u64;
                    nat_mul(a611, a612)?
                };
                let a614 = v4;
                nat_add(a613, a614)?
            };
            nat_add(a615, a616)?
        } else {
            let a621 = 32u64;
            let a622 = {
                let a619 = {
                    let a617 = v2;
                    let a618 = 32u64;
                    nat_mul(a617, a618)?
                };
                let a620 = v4;
                nat_add(a619, a620)?
            };
            nat_add(a621, a622)?
        };
        let v7: u64 = if v0 {
            let a627 = 128u64;
            let a628 = {
                let a625 = {
                    let a623 = v2;
                    let a624 = 32u64;
                    nat_mul(a623, a624)?
                };
                let a626 = v4;
                nat_add(a625, a626)?
            };
            nat_add(a627, a628)?
        } else {
            v4
        };
        let v8: u64 = if v0 {
            let a629 = 192u64;
            let a630 = v5;
            nat_add(a629, a630)?
        } else {
            let a634 = 96u64;
            let a635 = {
                let a631 = v5;
                let a632 = 8u64;
                let a633 = 0u64;
                nat_rem(a631, a632, a633)
            };
            nat_add(a634, a635)?
        };
        let v9: u64 = if v0 {
            v8
        } else {
            let a639 = 104u64;
            let a640 = {
                let a636 = v5;
                let a637 = 4u64;
                let a638 = 0u64;
                nat_rem(a636, a637, a638)
            };
            nat_add(a639, a640)?
        };
        Ok(Adt1::C8(v6, v7, v8, v9))
    } else {
        Ok(Adt1::C0)
    }
}

pub fn f13(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt1> {
    let a641 = v1;
    let a642 = 256u64;
    if nat_lt(a641, a642) {
        let a643 = v2;
        let a644 = 256u64;
        if if if if nat_lt(a643, a644) {
            let a645 = v3;
            let a646 = 256u64;
            nat_lt(a645, a646)
        } else {
            false
        } {
            let a647 = v4;
            let a648 = 256u64;
            nat_lt(a647, a648)
        } else {
            false
        } {
            let a649 = v5;
            let a650 = 256u64;
            nat_lt(a649, a650)
        } else {
            false
        } {
            let v6: u64 = {
                let a654 = {
                    let a651 = v1;
                    let a652 = 32u64;
                    let a653 = 0u64;
                    nat_quot(a651, a652, a653)
                };
                let a655 = 4u64;
                let a656 = 0u64;
                nat_rem(a654, a655, a656)
            };
            let v7: u64 = {
                let a657 = v1;
                let a658 = 16u64;
                let a659 = 0u64;
                nat_quot(a657, a658, a659)
            };
            let v8: u64 = f11({
                let a660 = v6;
                let a661 = 2u64;
                nat_mul(a660, a661)?
            });
            let v9: u64 = if v0 {
                let a665 = {
                    let a662 = v3;
                    let a663 = v8;
                    let a664 = 0u64;
                    nat_quot(a662, a663, a664)
                };
                let a666 = 4u64;
                let a667 = 0u64;
                nat_rem(a665, a666, a667)
            } else {
                let a674 = {
                    let a671 = v3;
                    let a672 = f11({
                        let a668 = v1;
                        let a669 = 32u64;
                        let a670 = 0u64;
                        nat_quot(a668, a669, a670)
                    });
                    let a673 = 0u64;
                    nat_quot(a671, a672, a673)
                };
                let a675 = 2u64;
                let a676 = 0u64;
                nat_rem(a674, a675, a676)
            };
            let v10: u64 = if v0 {
                let a687 = {
                    let a682 = {
                        let a679 = v2;
                        let a680 = {
                            let a677 = v6;
                            let a678 = 2u64;
                            if nat_lt(a677, a678) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a681 = 0u64;
                        nat_quot(a679, a680, a681)
                    };
                    let a683 = 16u64;
                    let a684 = 0u64;
                    nat_rem(a682, a683, a684)
                };
                let a688 = {
                    let a685 = v9;
                    let a686 = 16u64;
                    nat_mul(a685, a686)?
                };
                nat_add(a687, a688)?
            } else {
                let a697 = {
                    let a692 = {
                        let a689 = v2;
                        let a690 = v8;
                        let a691 = 0u64;
                        nat_quot(a689, a690, a691)
                    };
                    let a693 = 4u64;
                    let a694 = 0u64;
                    nat_rem(a692, a693, a694)
                };
                let a698 = {
                    let a695 = v9;
                    let a696 = 4u64;
                    nat_mul(a695, a696)?
                };
                nat_add(a697, a698)?
            };
            let v11: u64 = if v0 {
                32u64
            } else {
                4u64
            };
            let v12: u64 = if v0 {
                let a701 = {
                    let a699 = v4;
                    let a700 = 128u64;
                    nat_add(a699, a700)?
                };
                let a702 = 256u64;
                let a703 = 0u64;
                nat_rem(a701, a702, a703)
            } else {
                let a725 = {
                    let a709 = {
                        let a706 = v4;
                        let a707 = {
                            let a704 = v7;
                            let a705 = 8u64;
                            if nat_lt(a704, a705) {
                                1u64
                            } else {
                                16u64
                            }
                        };
                        let a708 = 0u64;
                        nat_quot(a706, a707, a708)
                    };
                    let a710 = 16u64;
                    let a711 = 0u64;
                    nat_rem(a709, a710, a711)
                };
                let a726 = {
                    let a723 = 16u64;
                    let a724 = {
                        let a720 = {
                            let a717 = v5;
                            let a718 = f11({
                                let a715 = {
                                    let a712 = v7;
                                    let a713 = 4u64;
                                    let a714 = 0u64;
                                    nat_quot(a712, a713, a714)
                                };
                                let a716 = 2u64;
                                nat_mul(a715, a716)?
                            });
                            let a719 = 0u64;
                            nat_quot(a717, a718, a719)
                        };
                        let a721 = 4u64;
                        let a722 = 0u64;
                        nat_rem(a720, a721, a722)
                    };
                    nat_mul(a723, a724)?
                };
                nat_add(a725, a726)?
            };
            let v13: u64 = if v0 {
                128u64
            } else {
                32u64
            };
            let v14: u64 = {
                let a727 = v10;
                let a728 = v11;
                if nat_lt(a727, a728) {
                    let a729 = v11;
                    let a730 = v10;
                    nat_sub(a729, a730)
                } else {
                    let a731 = v10;
                    let a732 = v11;
                    nat_sub(a731, a732)
                }
            };
            let v15: u64 = {
                let a733 = v12;
                let a734 = v13;
                if nat_lt(a733, a734) {
                    let a735 = v13;
                    let a736 = v12;
                    nat_sub(a735, a736)
                } else {
                    let a737 = v12;
                    let a738 = v13;
                    nat_sub(a737, a738)
                }
            };
            let v16: u64 = {
                let a739 = v14;
                let a740 = v15;
                nat_mul(a739, a740)?
            };
            Ok(Adt1::C1({
                let a741 = v10;
                let a742 = v11;
                if if nat_lt(a741, a742) {
                    let a743 = v12;
                    let a744 = v13;
                    nat_lt(a743, a744)
                } else {
                    let a745 = v12;
                    let a746 = v13;
                    !nat_lt(a745, a746)
                } {
                    v16
                } else {
                    let a749 = {
                        let a747 = 65536u64;
                        let a748 = v16;
                        nat_sub(a747, a748)
                    };
                    let a750 = 65536u64;
                    let a751 = 0u64;
                    nat_rem(a749, a750, a751)
                }
            }))
        } else {
            Ok(Adt1::C0)
        }
    } else {
        Ok(Adt1::C0)
    }
}
