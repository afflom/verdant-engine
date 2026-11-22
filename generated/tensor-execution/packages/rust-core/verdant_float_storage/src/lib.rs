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
    F16,
}

impl Fn0 {
    pub fn apply(&self, p0: u64) -> R<u64> {
        match self {
            Fn0::F16 => f16(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F18,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt0) -> R<Option<Adt0>> {
        match self {
            Fn1::F18 => f18(p0),
        }
    }
}

pub fn f0(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    if if if f1(v0) {
        f1(v1)
    } else {
        false
    } {
        let a3 = v2;
        let a4 = {
            let a1 = v0;
            let a2 = 0u64;
            if nat_eq(a1, a2) {
                4294967296u64
            } else {
                65536u64
            }
        };
        nat_lt(a3, a4)
    } else {
        false
    } {
        Ok(Some({
            let a5 = v0;
            let a6 = v1;
            if nat_eq(a5, a6) {
                v2
            } else {
                let v3: u64 = f2(v0, v2)?;
                let a7 = v1;
                let a8 = 0u64;
                if nat_eq(a7, a8) {
                    v3
                } else {
                    let a9 = v1;
                    let a10 = 1u64;
                    if nat_eq(a9, a10) {
                        f3(v3)?
                    } else {
                        f4(v3)?
                    }
                }
            }
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64) -> bool {
    let a11 = v0;
    let a12 = 0u64;
    if if nat_eq(a11, a12) {
        true
    } else {
        let a13 = v0;
        let a14 = 1u64;
        nat_eq(a13, a14)
    } {
        true
    } else {
        let a15 = v0;
        let a16 = 30u64;
        nat_eq(a15, a16)
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a17 = v0;
    let a18 = 0u64;
    if nat_eq(a17, a18) {
        Ok(v1)
    } else {
        let a19 = v0;
        let a20 = 1u64;
        if nat_eq(a19, a20) {
            f5(v1)
        } else {
            let a21 = v1;
            let a22 = 65536u64;
            nat_mul(a21, a22)
        }
    }
}

pub fn f3(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a23 = f6(v0)?;
        let a24 = 32768u64;
        nat_mul(a23, a24)?
    };
    let v2: u64 = f7(v0)?;
    let a43 = v1;
    let a44 = {
        let a25 = v2;
        let a26 = 255u64;
        if nat_eq(a25, a26) {
            let a27 = f8(v0)?;
            let a28 = 0u64;
            if nat_eq(a27, a28) {
                31744u64
            } else {
                32256u64
            }
        } else {
            let a29 = 143u64;
            let a30 = v2;
            if nat_le(a29, a30) {
                31744u64
            } else {
                let a31 = v2;
                let a32 = 102u64;
                if nat_lt(a31, a32) {
                    0u64
                } else {
                    let a33 = 113u64;
                    let a34 = v2;
                    if nat_le(a33, a34) {
                        let a39 = {
                            let a37 = {
                                let a35 = v2;
                                let a36 = 113u64;
                                nat_sub(a35, a36)
                            };
                            let a38 = 1024u64;
                            nat_mul(a37, a38)?
                        };
                        let a40 = f9(f10(v0)?, 8192u64)?;
                        nat_add(a39, a40)?
                    } else {
                        f9(f10(v0)?, f11({
                            let a41 = 126u64;
                            let a42 = v2;
                            nat_sub(a41, a42)
                        })?)?
                    }
                }
            }
        }
    };
    nat_add(a43, a44)
}

pub fn f4(v0: u64) -> R<u64> {
    if f12(v0)? {
        let v1: u64 = {
            let a45 = v0;
            let a46 = 65536u64;
            let a47 = 0u64;
            nat_quot(a45, a46, a47)
        };
        let a56 = v1;
        let a57 = {
            let a54 = {
                let a51 = {
                    let a48 = v1;
                    let a49 = 64u64;
                    let a50 = 0u64;
                    nat_quot(a48, a49, a50)
                };
                let a52 = 2u64;
                let a53 = 0u64;
                nat_rem(a51, a52, a53)
            };
            let a55 = 0u64;
            if nat_eq(a54, a55) {
                64u64
            } else {
                0u64
            }
        };
        nat_add(a56, a57)
    } else {
        let a68 = {
            let a66 = {
                let a58 = v0;
                let a59 = 32767u64;
                nat_add(a58, a59)?
            };
            let a67 = {
                let a63 = {
                    let a60 = v0;
                    let a61 = 65536u64;
                    let a62 = 0u64;
                    nat_quot(a60, a61, a62)
                };
                let a64 = 2u64;
                let a65 = 0u64;
                nat_rem(a63, a64, a65)
            };
            nat_add(a66, a67)?
        };
        let a69 = 65536u64;
        let a70 = 0u64;
        Ok(nat_quot(a68, a69, a70))
    }
}

pub fn f5(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a74 = {
            let a71 = v0;
            let a72 = 32768u64;
            let a73 = 0u64;
            nat_quot(a71, a72, a73)
        };
        let a75 = 2147483648u64;
        nat_mul(a74, a75)?
    };
    let v2: u64 = {
        let a79 = {
            let a76 = v0;
            let a77 = 32768u64;
            let a78 = 0u64;
            nat_rem(a76, a77, a78)
        };
        let a80 = 1024u64;
        let a81 = 0u64;
        nat_quot(a79, a80, a81)
    };
    let v3: u64 = {
        let a82 = v0;
        let a83 = 1024u64;
        let a84 = 0u64;
        nat_rem(a82, a83, a84)
    };
    let a119 = v1;
    let a120 = {
        let a85 = v2;
        let a86 = 31u64;
        if nat_eq(a85, a86) {
            let a95 = 2139095040u64;
            let a96 = {
                let a93 = {
                    let a87 = 0u64;
                    let a88 = v3;
                    if if nat_lt(a87, a88) {
                        let a89 = v3;
                        let a90 = 512u64;
                        nat_lt(a89, a90)
                    } else {
                        false
                    } {
                        let a91 = v3;
                        let a92 = 512u64;
                        nat_add(a91, a92)?
                    } else {
                        v3
                    }
                };
                let a94 = 8192u64;
                nat_mul(a93, a94)?
            };
            nat_add(a95, a96)?
        } else {
            let a97 = v2;
            let a98 = 0u64;
            if nat_eq(a97, a98) {
                let a99 = v3;
                let a100 = 0u64;
                if nat_eq(a99, a100) {
                    0u64
                } else {
                    let v4: Adt0 = f13(113u64, v3, 0u64, 1024u64)?;
                    let a109 = {
                        let a102 = match v4.clone() {
                            Adt0::C0(h101, _, _, _) => {
                                h101
                            }
                        };
                        let a103 = 8388608u64;
                        nat_mul(a102, a103)?
                    };
                    let a110 = {
                        let a107 = {
                            let a105 = match v4 {
                                Adt0::C0(_, h104, _, _) => {
                                    h104
                                }
                            };
                            let a106 = 1024u64;
                            nat_sub(a105, a106)
                        };
                        let a108 = 8192u64;
                        nat_mul(a107, a108)?
                    };
                    nat_add(a109, a110)?
                }
            } else {
                let a117 = {
                    let a113 = {
                        let a111 = v2;
                        let a112 = 112u64;
                        nat_add(a111, a112)?
                    };
                    let a114 = 8388608u64;
                    nat_mul(a113, a114)?
                };
                let a118 = {
                    let a115 = v3;
                    let a116 = 8192u64;
                    nat_mul(a115, a116)?
                };
                nat_add(a117, a118)?
            }
        }
    };
    nat_add(a119, a120)
}

pub fn f6(v0: u64) -> R<u64> {
    let a145 = v0;
    let a146 = f11({
        let a143 = {
            let a141 = {
                let a121 = 1u64;
                let a122 = 1u64;
                nat_add(a121, a122)?
            };
            let a142 = {
                let a139 = {
                    let a137 = {
                        let a123 = 1u64;
                        let a124 = 1u64;
                        nat_add(a123, a124)?
                    };
                    let a138 = {
                        let a135 = {
                            let a133 = {
                                let a125 = 1u64;
                                let a126 = 1u64;
                                nat_add(a125, a126)?
                            };
                            let a134 = {
                                let a131 = {
                                    let a129 = {
                                        let a127 = 1u64;
                                        let a128 = 1u64;
                                        nat_add(a127, a128)?
                                    };
                                    let a130 = 1u64;
                                    nat_mul(a129, a130)?
                                };
                                let a132 = 1u64;
                                nat_add(a131, a132)?
                            };
                            nat_mul(a133, a134)?
                        };
                        let a136 = 1u64;
                        nat_add(a135, a136)?
                    };
                    nat_mul(a137, a138)?
                };
                let a140 = 1u64;
                nat_add(a139, a140)?
            };
            nat_mul(a141, a142)?
        };
        let a144 = 1u64;
        nat_add(a143, a144)?
    })?;
    let a147 = 0u64;
    Ok(nat_quot(a145, a146, a147))
}

pub fn f7(v0: u64) -> R<u64> {
    let a170 = f14(v0)?;
    let a171 = f11({
        let a168 = {
            let a166 = {
                let a148 = 1u64;
                let a149 = 1u64;
                nat_add(a148, a149)?
            };
            let a167 = {
                let a164 = {
                    let a162 = {
                        let a150 = 1u64;
                        let a151 = 1u64;
                        nat_add(a150, a151)?
                    };
                    let a163 = {
                        let a160 = {
                            let a158 = {
                                let a152 = 1u64;
                                let a153 = 1u64;
                                nat_add(a152, a153)?
                            };
                            let a159 = {
                                let a156 = {
                                    let a154 = 1u64;
                                    let a155 = 1u64;
                                    nat_add(a154, a155)?
                                };
                                let a157 = 1u64;
                                nat_mul(a156, a157)?
                            };
                            nat_mul(a158, a159)?
                        };
                        let a161 = 1u64;
                        nat_add(a160, a161)?
                    };
                    nat_mul(a162, a163)?
                };
                let a165 = 1u64;
                nat_add(a164, a165)?
            };
            nat_mul(a166, a167)?
        };
        let a169 = 1u64;
        nat_add(a168, a169)?
    })?;
    let a172 = 0u64;
    Ok(nat_quot(a170, a171, a172))
}

pub fn f8(v0: u64) -> R<u64> {
    let a195 = v0;
    let a196 = f11({
        let a193 = {
            let a191 = {
                let a173 = 1u64;
                let a174 = 1u64;
                nat_add(a173, a174)?
            };
            let a192 = {
                let a189 = {
                    let a187 = {
                        let a175 = 1u64;
                        let a176 = 1u64;
                        nat_add(a175, a176)?
                    };
                    let a188 = {
                        let a185 = {
                            let a183 = {
                                let a177 = 1u64;
                                let a178 = 1u64;
                                nat_add(a177, a178)?
                            };
                            let a184 = {
                                let a181 = {
                                    let a179 = 1u64;
                                    let a180 = 1u64;
                                    nat_add(a179, a180)?
                                };
                                let a182 = 1u64;
                                nat_mul(a181, a182)?
                            };
                            nat_mul(a183, a184)?
                        };
                        let a186 = 1u64;
                        nat_add(a185, a186)?
                    };
                    nat_mul(a187, a188)?
                };
                let a190 = 1u64;
                nat_add(a189, a190)?
            };
            nat_mul(a191, a192)?
        };
        let a194 = 1u64;
        nat_add(a193, a194)?
    })?;
    let a197 = 0u64;
    Ok(nat_rem(a195, a196, a197))
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a198 = v0;
        let a199 = v1;
        let a200 = 0u64;
        nat_quot(a198, a199, a200)
    };
    let v3: u64 = {
        let a204 = {
            let a201 = v0;
            let a202 = v1;
            let a203 = 0u64;
            nat_rem(a201, a202, a203)
        };
        let a205 = 2u64;
        nat_mul(a204, a205)?
    };
    let a215 = v2;
    let a216 = {
        let a206 = v1;
        let a207 = v3;
        if if nat_lt(a206, a207) {
            true
        } else {
            let a208 = v3;
            let a209 = v1;
            if nat_eq(a208, a209) {
                let a213 = {
                    let a210 = v2;
                    let a211 = 2u64;
                    let a212 = 0u64;
                    nat_rem(a210, a211, a212)
                };
                let a214 = 1u64;
                nat_eq(a213, a214)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a215, a216)
}

pub fn f10(v0: u64) -> R<u64> {
    let a241 = f8(v0)?;
    let a242 = {
        let a217 = f7(v0)?;
        let a218 = 0u64;
        if nat_eq(a217, a218) {
            0u64
        } else {
            f11({
                let a239 = {
                    let a237 = {
                        let a219 = 1u64;
                        let a220 = 1u64;
                        nat_add(a219, a220)?
                    };
                    let a238 = {
                        let a235 = {
                            let a233 = {
                                let a221 = 1u64;
                                let a222 = 1u64;
                                nat_add(a221, a222)?
                            };
                            let a234 = {
                                let a231 = {
                                    let a229 = {
                                        let a223 = 1u64;
                                        let a224 = 1u64;
                                        nat_add(a223, a224)?
                                    };
                                    let a230 = {
                                        let a227 = {
                                            let a225 = 1u64;
                                            let a226 = 1u64;
                                            nat_add(a225, a226)?
                                        };
                                        let a228 = 1u64;
                                        nat_mul(a227, a228)?
                                    };
                                    nat_mul(a229, a230)?
                                };
                                let a232 = 1u64;
                                nat_add(a231, a232)?
                            };
                            nat_mul(a233, a234)?
                        };
                        let a236 = 1u64;
                        nat_add(a235, a236)?
                    };
                    nat_mul(a237, a238)?
                };
                let a240 = 1u64;
                nat_add(a239, a240)?
            })?
        }
    };
    nat_add(a241, a242)
}

pub fn f11(v0: u64) -> R<u64> {
    f15(Fn0::F16, v0, 1u64)
}

pub fn f12(v0: u64) -> R<bool> {
    let a285 = f7(v0)?;
    let a286 = {
        let a283 = {
            let a281 = {
                let a243 = 1u64;
                let a244 = 1u64;
                nat_add(a243, a244)?
            };
            let a282 = {
                let a279 = {
                    let a277 = {
                        let a245 = 1u64;
                        let a246 = 1u64;
                        nat_add(a245, a246)?
                    };
                    let a278 = {
                        let a275 = {
                            let a273 = {
                                let a247 = 1u64;
                                let a248 = 1u64;
                                nat_add(a247, a248)?
                            };
                            let a274 = {
                                let a271 = {
                                    let a269 = {
                                        let a249 = 1u64;
                                        let a250 = 1u64;
                                        nat_add(a249, a250)?
                                    };
                                    let a270 = {
                                        let a267 = {
                                            let a265 = {
                                                let a251 = 1u64;
                                                let a252 = 1u64;
                                                nat_add(a251, a252)?
                                            };
                                            let a266 = {
                                                let a263 = {
                                                    let a261 = {
                                                        let a253 = 1u64;
                                                        let a254 = 1u64;
                                                        nat_add(a253, a254)?
                                                    };
                                                    let a262 = {
                                                        let a259 = {
                                                            let a257 = {
                                                                let a255 = 1u64;
                                                                let a256 = 1u64;
                                                                nat_add(a255, a256)?
                                                            };
                                                            let a258 = 1u64;
                                                            nat_mul(a257, a258)?
                                                        };
                                                        let a260 = 1u64;
                                                        nat_add(a259, a260)?
                                                    };
                                                    nat_mul(a261, a262)?
                                                };
                                                let a264 = 1u64;
                                                nat_add(a263, a264)?
                                            };
                                            nat_mul(a265, a266)?
                                        };
                                        let a268 = 1u64;
                                        nat_add(a267, a268)?
                                    };
                                    nat_mul(a269, a270)?
                                };
                                let a272 = 1u64;
                                nat_add(a271, a272)?
                            };
                            nat_mul(a273, a274)?
                        };
                        let a276 = 1u64;
                        nat_add(a275, a276)?
                    };
                    nat_mul(a277, a278)?
                };
                let a280 = 1u64;
                nat_add(a279, a280)?
            };
            nat_mul(a281, a282)?
        };
        let a284 = 1u64;
        nat_add(a283, a284)?
    };
    if nat_eq(a285, a286) {
        let a287 = 0u64;
        let a288 = f8(v0)?;
        Ok(nat_lt(a287, a288))
    } else {
        Ok(false)
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let (h289, _) = f17(Fn1::F18, {
        let a308 = {
            let a290 = 1u64;
            let a291 = 1u64;
            nat_add(a290, a291)?
        };
        let a309 = {
            let a306 = {
                let a292 = 1u64;
                let a293 = 1u64;
                nat_add(a292, a293)?
            };
            let a307 = {
                let a304 = {
                    let a294 = 1u64;
                    let a295 = 1u64;
                    nat_add(a294, a295)?
                };
                let a305 = {
                    let a302 = {
                        let a296 = 1u64;
                        let a297 = 1u64;
                        nat_add(a296, a297)?
                    };
                    let a303 = {
                        let a300 = {
                            let a298 = 1u64;
                            let a299 = 1u64;
                            nat_add(a298, a299)?
                        };
                        let a301 = 1u64;
                        nat_mul(a300, a301)?
                    };
                    nat_mul(a302, a303)?
                };
                nat_mul(a304, a305)?
            };
            nat_mul(a306, a307)?
        };
        nat_mul(a308, a309)?
    }, Adt0::C0(v0, v1, v2, v3))?;
    Ok(h289)
}

pub fn f14(v0: u64) -> R<u64> {
    let a334 = v0;
    let a335 = f11({
        let a332 = {
            let a330 = {
                let a310 = 1u64;
                let a311 = 1u64;
                nat_add(a310, a311)?
            };
            let a331 = {
                let a328 = {
                    let a326 = {
                        let a312 = 1u64;
                        let a313 = 1u64;
                        nat_add(a312, a313)?
                    };
                    let a327 = {
                        let a324 = {
                            let a322 = {
                                let a314 = 1u64;
                                let a315 = 1u64;
                                nat_add(a314, a315)?
                            };
                            let a323 = {
                                let a320 = {
                                    let a318 = {
                                        let a316 = 1u64;
                                        let a317 = 1u64;
                                        nat_add(a316, a317)?
                                    };
                                    let a319 = 1u64;
                                    nat_mul(a318, a319)?
                                };
                                let a321 = 1u64;
                                nat_add(a320, a321)?
                            };
                            nat_mul(a322, a323)?
                        };
                        let a325 = 1u64;
                        nat_add(a324, a325)?
                    };
                    nat_mul(a326, a327)?
                };
                let a329 = 1u64;
                nat_add(a328, a329)?
            };
            nat_mul(a330, a331)?
        };
        let a333 = 1u64;
        nat_add(a332, a333)?
    })?;
    let a336 = 0u64;
    Ok(nat_rem(a334, a335, a336))
}

pub fn f15(v0: Fn0, p340: u64, p341: u64) -> R<u64> {
    let m339 = (p340, p341);
    {
        let mut m339 = m339;
        loop {
            let step = {
                let (v1, v2) = m339;
                let m337 = v1;
                if m337 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v2))
                } else {
                    let v3 = m337 - 1;
                    Ok::<(u64, u64), R<u64>>((v3, {
                        let c338: Fn0 = v0.clone();
                        c338.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a342 = v0;
    let a343 = v0;
    nat_add(a342, a343)
}

pub fn f17(v0: Fn1, p348: u64, p349: Adt0) -> R<(Adt0, bool)> {
    let m347 = (p348, p349);
    {
        let mut m347 = m347;
        loop {
            let step = {
                let (v1, v2) = m347;
                let m344 = v1;
                if m344 == 0 {
                    Err::<(u64, Adt0), R<(Adt0, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m344 - 1;
                    let m346 = {
                        let c345: Fn1 = v0.clone();
                        c345.apply(v2.clone())?
                    };
                    match m346 {
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
                Ok(next) => { m347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f18(v0: Adt0) -> R<Option<Adt0>> {
    f19(v0)
}

pub fn f19(v0: Adt0) -> R<Option<Adt0>> {
    let a351 = 0u64;
    let a352 = match v0.clone() {
        Adt0::C0(_, h350, _, _) => {
            h350
        }
    };
    if if if nat_lt(a351, a352) {
        let a355 = match v0.clone() {
            Adt0::C0(_, h353, _, _) => {
                h353
            }
        };
        let a356 = match v0.clone() {
            Adt0::C0(_, _, _, h354) => {
                h354
            }
        };
        nat_lt(a355, a356)
    } else {
        false
    } {
        let a359 = match v0.clone() {
            Adt0::C0(_, _, h357, _) => {
                h357
            }
        };
        let a360 = match v0.clone() {
            Adt0::C0(h358, _, _, _) => {
                h358
            }
        };
        nat_lt(a359, a360)
    } else {
        false
    } {
        Ok(Some(Adt0::C0({
            let a362 = match v0.clone() {
                Adt0::C0(h361, _, _, _) => {
                    h361
                }
            };
            let a363 = 1u64;
            nat_sub(a362, a363)
        }, {
            let a366 = match v0.clone() {
                Adt0::C0(_, h364, _, _) => {
                    h364
                }
            };
            let a367 = match v0.clone() {
                Adt0::C0(_, h365, _, _) => {
                    h365
                }
            };
            nat_add(a366, a367)?
        }, match v0.clone() {
            Adt0::C0(_, _, h368, _) => {
                h368
            }
        }, match v0 {
            Adt0::C0(_, _, _, h369) => {
                h369
            }
        })))
    } else {
        Ok(None::<Adt0>)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64) -> R<Option<u64>> {
    f0(p0, p1, p2)
}
