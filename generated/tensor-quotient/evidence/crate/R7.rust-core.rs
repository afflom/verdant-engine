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
}

#[derive(Clone)]
pub enum Fn0 {
    F13,
    F14,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn0::F13 => f13(p0),
            Fn0::F14 => f14(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F22,
}

impl Fn1 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn1::F22 => f22(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 11u64;
    if if if nat_lt(a1, a2) {
        true
    } else {
        let a3 = 13u64;
        let a4 = v0;
        nat_lt(a3, a4)
    } {
        true
    } else {
        let a5 = 18446744073709551615u64;
        let a6 = v1;
        nat_lt(a5, a6)
    } {
        Ok(None::<u64>)
    } else {
        let a7 = v0;
        let a8 = 13u64;
        if nat_eq(a7, a8) {
            f1(v1)
        } else {
            Ok(Some({
                let a9 = v0;
                let a10 = 11u64;
                if nat_eq(a9, a10) {
                    f2(v1)?
                } else {
                    f3(v1)?
                }
            }))
        }
    }
}

pub fn f1(v0: u64) -> R<Option<u64>> {
    let m11 = f4(v0)?;
    match m11 {
        None => {
            Ok(None::<u64>)
        }
        Some(v1) => {
            let a12 = v1;
            let a13 = 2147483648u64;
            if if nat_lt(a12, a13) {
                true
            } else {
                let a14 = 18446744071562067968u64;
                let a15 = v1;
                nat_le(a14, a15)
            } {
                Ok(Some({
                    let a16 = v1;
                    let a17 = 4294967296u64;
                    let a18 = 0u64;
                    nat_rem(a16, a17, a18)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a19 = f5(v0);
        let a20 = 2147483648u64;
        nat_mul(a19, a20)?
    };
    let v2: u64 = f6(v0);
    let a33 = v1;
    let a34 = if f7(v0) {
        2143289344u64
    } else {
        if f8(v0) {
            2139095040u64
        } else {
            let a21 = 1150u64;
            let a22 = v2;
            if nat_lt(a21, a22) {
                2139095039u64
            } else {
                let a23 = 897u64;
                let a24 = v2;
                if nat_le(a23, a24) {
                    let a29 = {
                        let a27 = {
                            let a25 = v2;
                            let a26 = 897u64;
                            nat_sub(a25, a26)
                        };
                        let a28 = 8388608u64;
                        nat_mul(a27, a28)?
                    };
                    let a30 = f9(f10(v0)?, 29u64)?;
                    nat_add(a29, a30)?
                } else {
                    f9(f10(v0)?, {
                        let a31 = 926u64;
                        let a32 = v2;
                        nat_sub(a31, a32)
                    })?
                }
            }
        }
    };
    nat_add(a33, a34)
}

pub fn f3(v0: u64) -> R<u64> {
    let v1: bool = {
        let a35 = 9223372036854775808u64;
        let a36 = v0;
        nat_le(a35, a36)
    };
    let v2: u64 = if v1 {
        f11(v0)?
    } else {
        v0
    };
    let v3: Adt0 = {
        let (h37, _) = f12(Fn0::F13, 64u64, Adt0::C0(153u64, v2, 1u64, 67108864u64))?;
        h37
    };
    let v4: Adt0 = {
        let (h38, _) = f12(Fn0::F14, 32u64, v3)?;
        h38
    };
    let a41 = if v1 {
        2147483648u64
    } else {
        0u64
    };
    let a42 = f15(match v4.clone() {
        Adt0::C0(h39, _, _, _) => {
            h39
        }
    }, f9(match v4 {
        Adt0::C0(_, h40, _, _) => {
            h40
        }
    }, 3u64)?)?;
    nat_add(a41, a42)
}

pub fn f4(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f6(v0);
    let a43 = v1;
    let a44 = 1023u64;
    if nat_lt(a43, a44) {
        Ok(Some(0u64))
    } else {
        let a45 = 1086u64;
        let a46 = v1;
        if nat_le(a45, a46) {
            let a47 = v0;
            let a48 = 14114281232179134464u64;
            if nat_eq(a47, a48) {
                Ok(Some(9223372036854775808u64))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let v2: u64 = {
                let a49 = 4503599627370496u64;
                let a50 = f16(v0);
                nat_add(a49, a50)?
            };
            let v3: u64 = {
                let a51 = v1;
                let a52 = 1075u64;
                if nat_lt(a51, a52) {
                    let a55 = v2;
                    let a56 = f17({
                        let a53 = 1075u64;
                        let a54 = v1;
                        nat_sub(a53, a54)
                    })?;
                    let a57 = 0u64;
                    nat_quot(a55, a56, a57)
                } else {
                    let a60 = v2;
                    let a61 = f17({
                        let a58 = v1;
                        let a59 = 1075u64;
                        nat_sub(a58, a59)
                    })?;
                    nat_mul(a60, a61)?
                }
            };
            Ok(Some({
                let a62 = f5(v0);
                let a63 = 0u64;
                if nat_eq(a62, a63) {
                    v3
                } else {
                    f11(v3)?
                }
            }))
        }
    }
}

pub fn f5(v0: u64) -> u64 {
    let a64 = v0;
    let a65 = 9223372036854775808u64;
    let a66 = 0u64;
    nat_quot(a64, a65, a66)
}

pub fn f6(v0: u64) -> u64 {
    let a67 = f18(v0);
    let a68 = 4503599627370496u64;
    let a69 = 0u64;
    nat_quot(a67, a68, a69)
}

pub fn f7(v0: u64) -> bool {
    let a70 = f6(v0);
    let a71 = 2047u64;
    if nat_eq(a70, a71) {
        let a72 = 0u64;
        let a73 = f16(v0);
        nat_lt(a72, a73)
    } else {
        false
    }
}

pub fn f8(v0: u64) -> bool {
    let a74 = f6(v0);
    let a75 = 2047u64;
    if nat_eq(a74, a75) {
        let a76 = f16(v0);
        let a77 = 0u64;
        nat_eq(a76, a77)
    } else {
        false
    }
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let a78 = v1;
    let a79 = 0u64;
    if nat_eq(a78, a79) {
        Ok(v0)
    } else {
        let a80 = 64u64;
        let a81 = v1;
        if nat_le(a80, a81) {
            let a82 = v0;
            let a83 = 0u64;
            if nat_eq(a82, a83) {
                Ok(0u64)
            } else {
                Ok(1u64)
            }
        } else {
            let v2: u64 = f17(v1)?;
            let v3: u64 = {
                let a84 = v0;
                let a85 = v2;
                let a86 = 0u64;
                nat_quot(a84, a85, a86)
            };
            let a97 = v3;
            let a98 = {
                let a90 = 0u64;
                let a91 = {
                    let a87 = v0;
                    let a88 = v2;
                    let a89 = 0u64;
                    nat_rem(a87, a88, a89)
                };
                if if nat_lt(a90, a91) {
                    let a95 = {
                        let a92 = v3;
                        let a93 = 2u64;
                        let a94 = 0u64;
                        nat_rem(a92, a93, a94)
                    };
                    let a96 = 0u64;
                    nat_eq(a95, a96)
                } else {
                    false
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a97, a98)
        }
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let a101 = f16(v0);
    let a102 = {
        let a99 = f6(v0);
        let a100 = 0u64;
        if nat_eq(a99, a100) {
            0u64
        } else {
            4503599627370496u64
        }
    };
    nat_add(a101, a102)
}

pub fn f11(v0: u64) -> R<u64> {
    let a103 = v0;
    let a104 = 0u64;
    if nat_eq(a103, a104) {
        Ok(0u64)
    } else {
        let a107 = {
            let a105 = 18446744073709551615u64;
            let a106 = v0;
            nat_sub(a105, a106)
        };
        let a108 = 1u64;
        nat_add(a107, a108)
    }
}

pub fn f12(v0: Fn0, p113: u64, p114: Adt0) -> R<(Adt0, bool)> {
    let m112 = (p113, p114);
    {
        let mut m112 = m112;
        loop {
            let step = {
                let (v1, v2) = m112;
                let m109 = v1;
                if m109 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m109 - 1;
                    let m111 = {
                        let c110: Fn0 = v0.clone();
                        c110.apply(v2.clone())?
                    };
                    match m111 {
                        None => {
                            Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, Adt0), R<(Adt0, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f13(v0: Adt0) -> R<Option<Adt0>> {
    f19(v0)
}

pub fn f14(v0: Adt0) -> R<Option<Adt0>> {
    f20(v0)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let a115 = v1;
    let a116 = 0u64;
    if nat_eq(a115, a116) {
        Ok(0u64)
    } else {
        let a117 = 16777216u64;
        let a118 = v1;
        if nat_le(a117, a118) {
            let a124 = {
                let a119 = v0;
                let a120 = 8388608u64;
                nat_mul(a119, a120)?
            };
            let a125 = {
                let a121 = v1;
                let a122 = 2u64;
                let a123 = 0u64;
                nat_quot(a121, a122, a123)
            };
            nat_add(a124, a125)
        } else {
            let a130 = {
                let a128 = {
                    let a126 = v0;
                    let a127 = 1u64;
                    nat_sub(a126, a127)
                };
                let a129 = 8388608u64;
                nat_mul(a128, a129)?
            };
            let a131 = v1;
            nat_add(a130, a131)
        }
    }
}

pub fn f16(v0: u64) -> u64 {
    let a132 = v0;
    let a133 = 4503599627370496u64;
    let a134 = 0u64;
    nat_rem(a132, a133, a134)
}

pub fn f17(v0: u64) -> R<u64> {
    f21(Fn1::F22, v0, 1u64)
}

pub fn f18(v0: u64) -> u64 {
    let a135 = v0;
    let a136 = 9223372036854775808u64;
    let a137 = 0u64;
    nat_rem(a135, a136, a137)
}

pub fn f19(v0: Adt0) -> R<Option<Adt0>> {
    let a139 = 134217728u64;
    let a140 = match v0.clone() {
        Adt0::C0(_, h138, _, _) => {
            h138
        }
    };
    if nat_le(a139, a140) {
        Ok(Some(Adt0::C0({
            let a142 = match v0.clone() {
                Adt0::C0(h141, _, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            nat_add(a142, a143)?
        }, f9(match v0.clone() {
            Adt0::C0(_, h144, _, _) => {
                h144
            }
        }, 1u64)?, match v0.clone() {
            Adt0::C0(_, _, h145, _) => {
                h145
            }
        }, match v0 {
            Adt0::C0(_, _, _, h146) => {
                h146
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f20(v0: Adt0) -> R<Option<Adt0>> {
    let a148 = 0u64;
    let a149 = match v0.clone() {
        Adt0::C0(_, h147, _, _) => {
            h147
        }
    };
    if if if nat_lt(a148, a149) {
        let a152 = match v0.clone() {
            Adt0::C0(_, h150, _, _) => {
                h150
            }
        };
        let a153 = match v0.clone() {
            Adt0::C0(_, _, _, h151) => {
                h151
            }
        };
        nat_lt(a152, a153)
    } else {
        false
    } {
        let a156 = match v0.clone() {
            Adt0::C0(_, _, h154, _) => {
                h154
            }
        };
        let a157 = match v0.clone() {
            Adt0::C0(h155, _, _, _) => {
                h155
            }
        };
        nat_lt(a156, a157)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a159 = match v0.clone() {
                Adt0::C0(h158, _, _, _) => {
                    h158
                }
            };
            let a160 = 1u64;
            nat_sub(a159, a160)
        }, {
            let a163 = match v0.clone() {
                Adt0::C0(_, h161, _, _) => {
                    h161
                }
            };
            let a164 = match v0.clone() {
                Adt0::C0(_, h162, _, _) => {
                    h162
                }
            };
            nat_add(a163, a164)?
        }, match v0.clone() {
            Adt0::C0(_, _, h165, _) => {
                h165
            }
        }, match v0 {
            Adt0::C0(_, _, _, h166) => {
                h166
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn f21(v0: Fn1, p170: u64, p171: u64) -> R<u64> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v1, v2) = m169;
                let m167 = v1;
                if m167 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m167 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c168: Fn1 = v0.clone();
                        c168.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f22(v0: u64) -> R<u64> {
    let a172 = v0;
    let a173 = v0;
    nat_add(a172, a173)
}
